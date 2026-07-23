//! 汎用ヘルパー関数モジュール
//!
//! ワイルドカード検索、.gitignore パース、バイトサイズ表記フォーマットなどの純粋ヘルパー関数を提供します。

use crate::config::{SortMode, TreeConfig};
use std::fs::{self, DirEntry};
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::time::SystemTime;

/// ファイル行数カウント結果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineCountResult {
    /// 正常にカウントされた行数
    Count(usize),
    /// ファイルサイズが制限（10MB等）を超過しているためスキップ
    SkippedTooLarge,
    /// 対象外の拡張子またはバイナリファイルのためスキップ
    SkippedBinaryOrUnsupported,
    /// 読み込みエラー
    Error,
}

/// テキスト拡張子か判定
pub fn is_supported_text_extension(ext: &str) -> bool {
    matches!(
        ext.to_lowercase().as_str(),
        "rs" | "py"
            | "js"
            | "ts"
            | "jsx"
            | "tsx"
            | "html"
            | "css"
            | "json"
            | "toml"
            | "yaml"
            | "yml"
            | "md"
            | "txt"
            | "c"
            | "cpp"
            | "h"
            | "hpp"
            | "java"
            | "go"
            | "sh"
            | "bat"
            | "ps1"
            | "xml"
            | "sql"
            | "rb"
            | "php"
            | "ini"
            | "conf"
            | "cfg"
            | "csv"
            | "tsv"
            | "log"
            | "env"
            | "gitignore"
            | ".gitignore"
    )
}

/// ファイルの行数をストリーミングで安全に計算
pub fn count_file_lines(path: &Path, file_size: u64) -> LineCountResult {
    const MAX_LINE_COUNT_SIZE: u64 = 10 * 1024 * 1024; // 10 MB

    if file_size > MAX_LINE_COUNT_SIZE {
        return LineCountResult::SkippedTooLarge;
    }

    // 拡張子のチェック
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_else(|| path.file_name().and_then(|n| n.to_str()).unwrap_or(""));

    if !is_supported_text_extension(ext) {
        return LineCountResult::SkippedBinaryOrUnsupported;
    }

    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return LineCountResult::Error,
    };

    let mut reader = BufReader::with_capacity(8192, file);

    // バイナリチェック（先頭1024バイトにヌルバイトが含まれるか）
    if let Ok(buf) = reader.fill_buf() {
        let check_len = buf.len().min(1024);
        if buf[..check_len].contains(&0) {
            return LineCountResult::SkippedBinaryOrUnsupported;
        }
    } else {
        return LineCountResult::Error;
    }

    // 行数カウント (バイナリ中の \n を高速カウント)
    let mut line_count = 0;
    let mut buffer = [0u8; 8192];
    let mut has_bytes = false;
    let mut last_byte_was_newline = true;

    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                has_bytes = true;
                for &b in &buffer[..n] {
                    if b == b'\n' {
                        line_count += 1;
                    }
                }
                last_byte_was_newline = buffer[n - 1] == b'\n';
            }
            Err(_) => return LineCountResult::Error,
        }
    }

    if !has_bytes {
        return LineCountResult::Count(0);
    }

    // 最終行が \n で終わっていない場合でも1行としてカウント
    if !last_byte_was_newline {
        line_count += 1;
    }

    LineCountResult::Count(line_count)
}

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

/// ディレクトリ・ファイルエントリのソート処理
///
/// 基本ルール: ディレクトリ優先（フォルダが上、ファイルが下）。
/// 同種内（フォルダ同士・ファイル同士）で `config.sort_mode` に従ってソートします。
/// `config.reverse_sort` が有効な場合は、同種内の並び順を反転させます。
pub fn sort_dir_entries(entries: &mut [DirEntry], config: &TreeConfig) {
    entries.sort_by(|a, b| {
        let is_dir_a = a.path().is_dir();
        let is_dir_b = b.path().is_dir();

        // 1. ディレクトリ優先
        if is_dir_a != is_dir_b {
            if is_dir_a {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        }

        // 2. 同種内での比較
        let name_a = a.file_name();
        let name_b = b.file_name();

        let ord = match config.sort_mode {
            SortMode::Name => name_a.cmp(&name_b),
            SortMode::Size => {
                let size_a = fs::metadata(a.path()).map(|m| m.len()).unwrap_or(0);
                let size_b = fs::metadata(b.path()).map(|m| m.len()).unwrap_or(0);
                // サイズが大きい順 (降順)
                size_b.cmp(&size_a).then_with(|| name_a.cmp(&name_b))
            }
            SortMode::Time => {
                let time_a = fs::metadata(a.path())
                    .and_then(|m| m.modified())
                    .unwrap_or(SystemTime::UNIX_EPOCH);
                let time_b = fs::metadata(b.path())
                    .and_then(|m| m.modified())
                    .unwrap_or(SystemTime::UNIX_EPOCH);
                // 新しい日時順 (降順)
                time_b.cmp(&time_a).then_with(|| name_a.cmp(&name_b))
            }
        };

        if config.reverse_sort {
            ord.reverse()
        } else {
            ord
        }
    });
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

    #[test]
    fn test_sort_dir_entries() {
        let temp_dir = std::env::temp_dir().join("mytree_test_sort_entries");
        let _ = fs::create_dir_all(&temp_dir);

        let file_small = temp_dir.join("a_small.txt");
        let file_large = temp_dir.join("b_large.txt");
        let dir_sub = temp_dir.join("c_dir");

        let _ = fs::create_dir_all(&dir_sub);

        let mut f1 = File::create(&file_small).unwrap();
        write!(f1, "small").unwrap(); // 5 bytes
        drop(f1);

        let mut f2 = File::create(&file_large).unwrap();
        write!(f2, "large content text").unwrap(); // 18 bytes
        drop(f2);

        let mut entries: Vec<DirEntry> = fs::read_dir(&temp_dir)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        // 1. デフォルト (名前順, ディレクトリ優先)
        let mut config = TreeConfig::default();
        sort_dir_entries(&mut entries, &config);
        assert_eq!(entries[0].file_name().to_str().unwrap(), "c_dir");
        assert_eq!(entries[1].file_name().to_str().unwrap(), "a_small.txt");
        assert_eq!(entries[2].file_name().to_str().unwrap(), "b_large.txt");

        // 2. サイズ順 (降順: 大きいファイルが上)
        config.sort_mode = SortMode::Size;
        sort_dir_entries(&mut entries, &config);
        assert_eq!(entries[0].file_name().to_str().unwrap(), "c_dir");
        assert_eq!(entries[1].file_name().to_str().unwrap(), "b_large.txt");
        assert_eq!(entries[2].file_name().to_str().unwrap(), "a_small.txt");

        // 3. サイズ順 + 逆順
        config.reverse_sort = true;
        sort_dir_entries(&mut entries, &config);
        assert_eq!(entries[0].file_name().to_str().unwrap(), "c_dir");
        assert_eq!(entries[1].file_name().to_str().unwrap(), "a_small.txt");
        assert_eq!(entries[2].file_name().to_str().unwrap(), "b_large.txt");

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_count_file_lines() {
        let temp_dir = std::env::temp_dir().join("mytree_test_count_lines");
        let _ = fs::create_dir_all(&temp_dir);

        // テキストファイル (3行)
        let txt_path = temp_dir.join("test.txt");
        let mut file = File::create(&txt_path).unwrap();
        writeln!(file, "Line 1").unwrap();
        writeln!(file, "Line 2").unwrap();
        write!(file, "Line 3").unwrap(); // 最後の改行なし

        let res = count_file_lines(&txt_path, 100);
        assert_eq!(res, LineCountResult::Count(3));

        // 空ファイル
        let empty_path = temp_dir.join("empty.rs");
        let _ = File::create(&empty_path).unwrap();
        let res = count_file_lines(&empty_path, 0);
        assert_eq!(res, LineCountResult::Count(0));

        // 10MB超過
        let res = count_file_lines(&txt_path, 11 * 1024 * 1024);
        assert_eq!(res, LineCountResult::SkippedTooLarge);

        // 対象外拡張子 (png)
        let png_path = temp_dir.join("image.png");
        let _ = File::create(&png_path).unwrap();
        let res = count_file_lines(&png_path, 100);
        assert_eq!(res, LineCountResult::SkippedBinaryOrUnsupported);

        // バイナリ (ヌルバイト含む)
        let bin_path = temp_dir.join("binary.txt");
        let mut file = File::create(&bin_path).unwrap();
        file.write_all(&[
            0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x00, 0x57, 0x6f, 0x72, 0x6c, 0x64,
        ])
        .unwrap();
        let res = count_file_lines(&bin_path, 11);
        assert_eq!(res, LineCountResult::SkippedBinaryOrUnsupported);

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
