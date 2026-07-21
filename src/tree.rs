//! ディレクトリ探索およびテキストツリー描画コアモジュール
//!
//! ディレクトリの再帰的探索、アイコン・枝記号付与、テキスト表示描画を提供します。

use crate::config::TreeConfig;
use crate::icon;
use crate::util::{format_byte_size, match_wildcard};
use std::fs;
use std::path::Path;

/// ルート行のフォーマット
pub fn format_root_line(config: &TreeConfig) -> String {
    let folder_name = config
        .target_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(".");

    let base = if config.classic_mode {
        folder_name.to_string()
    } else if config.show_icons {
        format!("\x1b[1;34m📁 {}\x1b[0m", folder_name)
    } else {
        format!("\x1b[1;34m{}\x1b[0m", folder_name)
    };

    if config.show_dir_sizes {
        let (total_size, total_files) = calculate_dir_summary(&config.target_path, config);
        if config.classic_mode {
            format!(
                "{} [{} files, {}]",
                base,
                total_files,
                format_byte_size(total_size)
            )
        } else {
            format!(
                "{} \x1b[90m[{} files, {}]\x1b[0m",
                base,
                total_files,
                format_byte_size(total_size)
            )
        }
    } else {
        base
    }
}

/// 指定ディレクトリ配下の全ファイルの合計サイズおよびファイル数を再帰的に集計する
pub fn calculate_dir_summary(dir: &Path, config: &TreeConfig) -> (u64, usize) {
    let mut total_size = 0u64;
    let mut total_files = 0usize;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().into_owned();

            // 隠しファイルのフィルタ
            if file_name.starts_with('.') && !config.show_hidden {
                continue;
            }

            // 除外パターンのフィルタ (ワイルドカード対応)
            let is_excluded = config
                .exclude_patterns
                .iter()
                .any(|pattern| match_wildcard(pattern, &file_name));
            if is_excluded {
                continue;
            }

            let path = entry.path();
            if path.is_dir() {
                let (sub_size, sub_files) = calculate_dir_summary(&path, config);
                total_size += sub_size;
                total_files += sub_files;
            } else {
                total_files += 1;
                if let Ok(metadata) = entry.metadata() {
                    total_size += metadata.len();
                }
            }
        }
    }

    (total_size, total_files)
}

/// 再帰的なツリー構築コア処理 (標準テキスト表示)
pub fn print_tree(
    dir: &Path,
    current_depth: usize,
    prefixes: &mut Vec<&str>,
    config: &TreeConfig,
    dir_count: &mut usize,
    file_count: &mut usize,
) {
    // 深度制限のチェック
    if config.max_depth > 0 && current_depth >= config.max_depth {
        return;
    }

    // ディレクトリのエントリを読み込む
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return, // 権限エラーなどはスキップ
    };

    // エントリを一度ベクターに詰める
    let mut valid_entries = Vec::new();
    for entry in entries.flatten() {
        let file_name = entry.file_name().to_string_lossy().into_owned();

        // 隠しファイルのフィルタ
        if file_name.starts_with('.') && !config.show_hidden {
            continue;
        }

        // 除外パターンのフィルタ (ワイルドカード対応)
        let is_excluded = config
            .exclude_patterns
            .iter()
            .any(|pattern| match_wildcard(pattern, &file_name));
        if is_excluded {
            continue;
        }

        valid_entries.push(entry);
    }

    // ソート処理 (ディレクトリを優先ソート)
    valid_entries.sort_by(|a, b| {
        let name_a = a.file_name();
        let name_b = b.file_name();

        let is_dir_a = a.path().is_dir();
        let is_dir_b = b.path().is_dir();

        if is_dir_a != is_dir_b {
            if is_dir_a {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        } else {
            name_a.cmp(&name_b)
        }
    });

    let count = valid_entries.len();
    for (idx, entry) in valid_entries.iter().enumerate() {
        let is_last = idx == count - 1;
        let path = entry.path();
        let is_directory = path.is_dir();
        let file_name = entry.file_name().to_string_lossy().into_owned();

        // 1. インデント接頭辞の出力
        for pref in prefixes.iter() {
            print!("{}", pref);
        }

        // 2. ブランチ記号の決定
        let branch = if config.classic_mode {
            if is_last {
                "\\-- "
            } else {
                "+-- "
            }
        } else if is_last {
            "└── "
        } else {
            "├── "
        };
        print!("{}", branch);

        // 3. アイコン絵文字の出力 (有効な場合)
        if config.show_icons {
            print!("{} ", icon::get_icon_emoji(&file_name, is_directory));
        }

        // 4. 名前と各種メタ情報の出力
        if is_directory {
            *dir_count += 1;
            if config.classic_mode {
                print!("{}", file_name);
            } else {
                // ディレクトリは青字・太字
                print!("\x1b[1;34m{}\x1b[0m", file_name);
            }

            // ディレクトリ配下の集計情報表示 (-D / --dir-sizes 有効時)
            if config.show_dir_sizes {
                let (total_size, total_files) = calculate_dir_summary(&path, config);
                if config.classic_mode {
                    print!(" [{} files, {}]", total_files, format_byte_size(total_size));
                } else {
                    print!(
                        " \x1b[90m[{} files, {}]\x1b[0m",
                        total_files,
                        format_byte_size(total_size)
                    );
                }
            }
        } else {
            *file_count += 1;
            // 通常のファイル
            print!("{}", file_name);

            // ファイルサイズ表示
            if config.show_sizes {
                if let Ok(metadata) = entry.metadata() {
                    let size = metadata.len();
                    if config.classic_mode {
                        print!(" [{}]", format_byte_size(size));
                    } else {
                        print!(" \x1b[90m[{}]\x1b[0m", format_byte_size(size));
                    }
                }
            }
        }
        println!();

        // ディレクトリの場合は再帰呼び出し
        if is_directory {
            let next_prefix = if config.classic_mode {
                if is_last {
                    "    "
                } else {
                    "|   "
                }
            } else if is_last {
                "    "
            } else {
                "│   "
            };
            prefixes.push(next_prefix);
            print_tree(
                &path,
                current_depth + 1,
                prefixes,
                config,
                dir_count,
                file_count,
            );
            prefixes.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_dir_summary() {
        let config = TreeConfig::default();
        let (size, files) = calculate_dir_summary(Path::new("."), &config);
        assert!(files > 0);
        assert!(size > 0);
    }
}
