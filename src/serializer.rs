//! 機械可読データ構造シリアライザモジュール
//!
//! ツリー構造を JSON または XML フォーマットの文字列へ手動シリアライズする機能を提供します。

use crate::config::TreeConfig;
use crate::tree::calculate_dir_summary;
use crate::util::{match_wildcard, parse_gitignore_file, sort_dir_entries};
use std::fs;
use std::path::Path;

/// 機械可読フォーマット出力用のメモリ内ツリーノード構造体
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub children: Vec<TreeNode>,
}

/// 再帰的にインメモリ TreeNode 階層構造を構築する
pub fn build_tree_node(
    dir: &Path,
    current_depth: usize,
    config: &TreeConfig,
    dir_count: &mut usize,
    file_count: &mut usize,
) -> TreeNode {
    let name = dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(".")
        .to_string();

    let path_str = dir.to_string_lossy().replace('\\', "/");
    let is_dir = dir.is_dir();

    if !is_dir {
        *file_count += 1;
        let size = fs::metadata(dir).map(|m| m.len()).unwrap_or(0);
        return TreeNode {
            name,
            path: path_str,
            is_dir: false,
            size,
            children: Vec::new(),
        };
    }

    *dir_count += 1;

    let mut children = Vec::new();

    // 深度制限のチェック
    if config.max_depth > 0 && current_depth >= config.max_depth {
        let (total_size, _) = calculate_dir_summary(dir, config);
        return TreeNode {
            name,
            path: path_str,
            is_dir: true,
            size: total_size,
            children,
        };
    }

    let gitignore_patterns = if config.use_gitignore {
        parse_gitignore_file(dir)
    } else {
        Vec::new()
    };

    if let Ok(entries) = fs::read_dir(dir) {
        let mut valid_entries = Vec::new();
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().into_owned();

            if file_name.starts_with('.') && !config.show_hidden {
                continue;
            }

            let is_excluded = config
                .exclude_patterns
                .iter()
                .chain(gitignore_patterns.iter())
                .any(|pattern| match_wildcard(pattern, &file_name));
            if is_excluded {
                continue;
            }

            valid_entries.push(entry);
        }

        // ソート処理 (ディレクトリ優先 & 指定モード)
        sort_dir_entries(&mut valid_entries, config);

        for entry in valid_entries {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                children.push(build_tree_node(
                    &entry_path,
                    current_depth + 1,
                    config,
                    dir_count,
                    file_count,
                ));
            } else {
                *file_count += 1;
                let file_name = entry.file_name().to_string_lossy().into_owned();
                let file_path_str = entry_path.to_string_lossy().replace('\\', "/");
                let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
                children.push(TreeNode {
                    name: file_name,
                    path: file_path_str,
                    is_dir: false,
                    size,
                    children: Vec::new(),
                });
            }
        }
    }

    let total_size = children.iter().map(|c| c.size).sum();

    TreeNode {
        name,
        path: path_str,
        is_dir: true,
        size: total_size,
        children,
    }
}

/// JSON用文字列エスケープ
pub fn escape_json_str(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            c if c.is_control() => {
                escaped.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => escaped.push(c),
        }
    }
    escaped
}

/// XML用エンティティエスケープ
pub fn escape_xml_str(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            c => escaped.push(c),
        }
    }
    escaped
}

/// TreeNode 階層を JSON フォーマット文字列へ手動シリアライズ
pub fn serialize_json(root: &TreeNode, dir_count: usize, file_count: usize) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    serialize_json_node(root, 1, &mut out);
    out.push_str(",\n  \"summary\": {\n");
    out.push_str(&format!("    \"directories\": {},\n", dir_count));
    out.push_str(&format!("    \"files\": {}\n", file_count));
    out.push_str("  }\n}");
    out
}

fn serialize_json_node(node: &TreeNode, indent_level: usize, out: &mut String) {
    let indent = "  ".repeat(indent_level);
    let node_type = if node.is_dir { "directory" } else { "file" };

    out.push_str(&format!("{}\"type\": \"{}\",\n", indent, node_type));
    out.push_str(&format!(
        "{}\"name\": \"{}\",\n",
        indent,
        escape_json_str(&node.name)
    ));
    out.push_str(&format!(
        "{}\"path\": \"{}\",\n",
        indent,
        escape_json_str(&node.path)
    ));
    out.push_str(&format!("{}\"size\": {}", indent, node.size));

    if node.is_dir {
        out.push_str(",\n");
        out.push_str(&format!("{}\"children\": [\n", indent));
        for (idx, child) in node.children.iter().enumerate() {
            let child_indent = "  ".repeat(indent_level + 1);
            out.push_str(&format!("{}{{\n", child_indent));
            serialize_json_node(child, indent_level + 2, out);
            out.push('\n');
            if idx < node.children.len() - 1 {
                out.push_str(&format!("{}}},\n", child_indent));
            } else {
                out.push_str(&format!("{}}}\n", child_indent));
            }
        }
        out.push_str(&format!("{}]", indent));
    }
}

/// TreeNode 階層を XML フォーマット文字列へ手動シリアライズ
pub fn serialize_xml(root: &TreeNode, dir_count: usize, file_count: usize) -> String {
    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str("<tree>\n");
    serialize_xml_node(root, 1, &mut out);
    out.push_str(&format!(
        "  <summary directories=\"{}\" files=\"{}\" />\n",
        dir_count, file_count
    ));
    out.push_str("</tree>");
    out
}

fn serialize_xml_node(node: &TreeNode, indent_level: usize, out: &mut String) {
    let indent = "  ".repeat(indent_level);
    let node_type = if node.is_dir { "directory" } else { "file" };

    out.push_str(&format!(
        "{}<node type=\"{}\" name=\"{}\" path=\"{}\" size=\"{}\"",
        indent,
        node_type,
        escape_xml_str(&node.name),
        escape_xml_str(&node.path),
        node.size
    ));

    if node.is_dir && !node.children.is_empty() {
        out.push_str(">\n");
        out.push_str(&format!("{}  <children>\n", indent));
        for child in &node.children {
            serialize_xml_node(child, indent_level + 2, out);
        }
        out.push_str(&format!("{}  </children>\n", indent));
        out.push_str(&format!("{}</node>\n", indent));
    } else {
        out.push_str(" />\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_json_str() {
        assert_eq!(escape_json_str("hello \"world\""), "hello \\\"world\\\"");
        assert_eq!(escape_json_str("path\\to\\file"), "path\\\\to\\\\file");
        assert_eq!(escape_json_str("line1\nline2"), "line1\\nline2");
    }

    #[test]
    fn test_escape_xml_str() {
        assert_eq!(
            escape_xml_str("a & b < c > d \"e\" 'f'"),
            "a &amp; b &lt; c &gt; d &quot;e&quot; &apos;f&apos;"
        );
    }

    #[test]
    fn test_serialize_json_and_xml() {
        let node = TreeNode {
            name: "root".to_string(),
            path: ".".to_string(),
            is_dir: true,
            size: 100,
            children: vec![TreeNode {
                name: "file.txt".to_string(),
                path: "./file.txt".to_string(),
                is_dir: false,
                size: 100,
                children: Vec::new(),
            }],
        };

        let json = serialize_json(&node, 1, 1);
        assert!(json.contains("\"type\": \"directory\""));
        assert!(json.contains("\"name\": \"root\""));
        assert!(json.contains("\"name\": \"file.txt\""));
        assert!(json.contains("\"directories\": 1"));

        let xml = serialize_xml(&node, 1, 1);
        assert!(xml.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(xml.contains("<node type=\"directory\" name=\"root\""));
        assert!(xml.contains("<node type=\"file\" name=\"file.txt\""));
        assert!(xml.contains("<summary directories=\"1\" files=\"1\" />"));
    }
}
