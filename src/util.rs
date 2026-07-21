//! 汎用ヘルパー関数モジュール
//!
//! ワイルドカード検索、.gitignore パース、バイトサイズ表記フォーマットなどの純粋ヘルパー関数を提供します。

use std::fs;
use std::path::Path;

/// ワイルドカード (*, ?) パターンマッチ関数 (依存ライブラリなし)
pub fn match_wildcard(pattern: &str, text: &str) -> bool {
    let p_chars: Vec<char> = pattern.to_lowercase().chars().collect();
    let t_chars: Vec<char> = text.to_lowercase().chars().collect();

    let mut p_idx = 0;
    let mut t_idx = 0;
    let mut star_idx = None;
    let mut match_t_idx = 0;

    while t_idx < t_chars.len() {
        if p_idx < p_chars.len() && (p_chars[p_idx] == '?' || p_chars[p_idx] == t_chars[t_idx]) {
            p_idx += 1;
            t_idx += 1;
        } else if p_idx < p_chars.len() && p_chars[p_idx] == '*' {
            star_idx = Some(p_idx);
            match_t_idx = t_idx;
            p_idx += 1;
        } else if let Some(s_idx) = star_idx {
            p_idx = s_idx + 1;
            match_t_idx += 1;
            t_idx = match_t_idx;
        } else {
            return false;
        }
    }

    while p_idx < p_chars.len() && p_chars[p_idx] == '*' {
        p_idx += 1;
    }

    p_idx == p_chars.len()
}

/// バイト単位のファイルサイズを読みやすい形式に変換 (`B`, `KB`, `MB`)
pub fn format_byte_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

/// 指定したディレクトリ内の `.gitignore` を読み込み、有効な除外パターン文字列をリストで返す
pub fn parse_gitignore_file(dir_path: &Path) -> Vec<String> {
    let gitignore_path = dir_path.join(".gitignore");
    if !gitignore_path.exists() || !gitignore_path.is_file() {
        return Vec::new();
    }

    let content = match fs::read_to_string(&gitignore_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let mut pattern = line;
            if pattern.starts_with('/') {
                pattern = &pattern[1..];
            }
            if pattern.ends_with('/') {
                pattern = &pattern[..pattern.len() - 1];
            }
            pattern.to_string()
        })
        .filter(|pattern| !pattern.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_format_byte_size() {
        assert_eq!(format_byte_size(0), "0 B");
        assert_eq!(format_byte_size(500), "500 B");
        assert_eq!(format_byte_size(1023), "1023 B");
        assert_eq!(format_byte_size(1024), "1.0 KB");
        assert_eq!(format_byte_size(1536), "1.5 KB");
        assert_eq!(format_byte_size(1048575), "1024.0 KB");
        assert_eq!(format_byte_size(1048576), "1.0 MB");
        assert_eq!(format_byte_size(10485760), "10.0 MB");
    }

    #[test]
    fn test_match_wildcard() {
        assert!(match_wildcard("*.tmp", "temp.tmp"));
        assert!(match_wildcard("*.TMP", "temp.tmp"));
        assert!(match_wildcard("target", "target"));
        assert!(match_wildcard("target", "TARGET"));
        assert!(match_wildcard("build*", "build_out"));
        assert!(match_wildcard("test?.rs", "test1.rs"));
        assert!(!match_wildcard("*.log", "main.rs"));
        assert!(!match_wildcard("test?.rs", "test12.rs"));
    }

    #[test]
    fn test_parse_gitignore_file() {
        let temp_dir = std::env::temp_dir().join("mytree_test_gitignore");
        let _ = fs::create_dir_all(&temp_dir);

        let gitignore_path = temp_dir.join(".gitignore");
        let mut file = File::create(&gitignore_path).unwrap();
        writeln!(file, "# Comment line").unwrap();
        writeln!(file).unwrap();
        writeln!(file, "target/").unwrap();
        writeln!(file, "*.log").unwrap();
        writeln!(file, "/build/").unwrap();
        writeln!(file, "  .env  ").unwrap();

        let patterns = parse_gitignore_file(&temp_dir);
        assert_eq!(patterns, vec!["target", "*.log", "build", ".env"]);

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
