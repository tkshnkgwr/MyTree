//! 設定およびCLI引数パースモジュール
//!
//! コマンドライン引数の解析および実行設定構造体の定義を提供します。

use crate::icon;
use std::env;
use std::path::PathBuf;

/// 出力フォーマットの定義
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
    Xml,
}

/// ツリーソートモードの定義
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortMode {
    /// 名前順 (アルファベット昇順: a -> z)
    Name,
    /// ファイルサイズ順 (降順: 大きい順)
    Size,
    /// 最終更新日時順 (降順: 新しい順)
    Time,
}

/// ツールでサポートする設定オプション
#[derive(Debug, Clone)]
pub struct TreeConfig {
    pub target_path: PathBuf,
    pub show_icons: bool,
    pub show_sizes: bool,
    pub show_dir_sizes: bool,
    pub show_lines: bool,
    pub show_hidden: bool,
    pub max_depth: usize,
    pub exclude_patterns: Vec<String>,
    pub classic_mode: bool,
    pub output_format: OutputFormat,
    pub use_gitignore: bool,
    pub sort_mode: SortMode,
    pub reverse_sort: bool,
}

impl Default for TreeConfig {
    fn default() -> Self {
        Self {
            target_path: PathBuf::from("."),
            show_icons: true,
            show_sizes: true,
            show_dir_sizes: false,
            show_lines: false,
            show_hidden: false,
            max_depth: 0,
            exclude_patterns: vec!["target", "node_modules", ".git"]
                .into_iter()
                .map(String::from)
                .collect(),
            classic_mode: false,
            output_format: OutputFormat::Text,
            use_gitignore: false,
            sort_mode: SortMode::Name,
            reverse_sort: false,
        }
    }
}

/// 引数を手動で解析 (軽量化のため標準ライブラリのみを使用)
pub fn parse_arguments(args: &[String], config: &mut TreeConfig) -> Result<(), String> {
    let mut i = 1;
    let mut path_set = false;

    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "-v" | "--version" => {
                print_version();
                std::process::exit(0);
            }
            "--help-icons" | "--icons-help" => {
                icon::print_icons_help();
                std::process::exit(0);
            }
            "-i" | "--icons" => {
                config.show_icons = true;
            }
            "-I" | "--no-icons" => {
                config.show_icons = false;
            }
            "-s" | "--sizes" => {
                config.show_sizes = true;
            }
            "-S" | "--no-sizes" => {
                config.show_sizes = false;
            }
            "-c" | "--classic" => {
                config.classic_mode = true;
                config.show_icons = false;
                config.show_sizes = false;
            }
            "-D" | "--dir-sizes" | "--du" => {
                config.show_dir_sizes = true;
            }
            "-l" | "--lines" => {
                config.show_lines = true;
            }
            "-a" | "--all" => {
                config.show_hidden = true;
            }
            "-g" | "--gitignore" => {
                config.use_gitignore = true;
            }
            "-r" | "--reverse" => {
                config.reverse_sort = true;
            }
            "--sort" => {
                if i + 1 < args.len() {
                    let mode_str = &args[i + 1];
                    match mode_str.to_lowercase().as_str() {
                        "name" => config.sort_mode = SortMode::Name,
                        "size" => config.sort_mode = SortMode::Size,
                        "time" => config.sort_mode = SortMode::Time,
                        _ => {
                            return Err(format!(
                                "無効なソートモードが指定されました: {} (使用可能: name, size, time)",
                                mode_str
                            ));
                        }
                    }
                    i += 1;
                } else {
                    return Err("ソートモードが指定されていません (--sort <MODE>)".to_string());
                }
            }
            "--json" => {
                config.output_format = OutputFormat::Json;
            }
            "--xml" => {
                config.output_format = OutputFormat::Xml;
            }
            "-d" | "--depth" => {
                if i + 1 < args.len() {
                    let depth_str = &args[i + 1];
                    config.max_depth = depth_str
                        .parse::<usize>()
                        .map_err(|_| format!("無効な深度が指定されました: {}", depth_str))?;
                    i += 1;
                } else {
                    return Err("深度の値が指定されていません (-d <DEPTH>)".to_string());
                }
            }
            "-e" | "--exclude" => {
                if i + 1 < args.len() {
                    let excludes_str = &args[i + 1];
                    config.exclude_patterns = excludes_str
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    i += 1;
                } else {
                    return Err("除外パターンが指定されていません (-e <PATTERNS>)".to_string());
                }
            }
            _ => {
                if arg.starts_with('-') {
                    return Err(format!("不明なオプションです: {}", arg));
                } else if !path_set {
                    config.target_path = PathBuf::from(arg);
                    path_set = true;
                } else {
                    return Err(format!("重複したターゲットパス指定です: {}", arg));
                }
            }
        }
        i += 1;
    }

    Ok(())
}

/// バージョン情報の出力
pub fn print_version() {
    println!(
        "MyTree v{}\nCopyright (c) 2026 tkshnkgwr\nLicense: MIT",
        env!("CARGO_PKG_VERSION")
    );
}

/// ヘルプメニューの出力
pub fn print_help() {
    println!(
        "\x1b[1mMyTree v{} (Windows/クロスプラットフォーム対応)\x1b[0m\n\
\x1b[33m使用法:\x1b[0m\n  MyTree [オプション] [ターゲットディレクトリ]\n\n\
\x1b[33mオプション一覧:\x1b[0m\n  \
\x1b[36m【基本情報】\x1b[0m\n  \
  -h, --help                 ヘルプメニューを表示\n  \
  -v, --version              バージョン情報を表示\n  \
  --help-icons               アイコン絵文字の対応一覧を表示\n\n  \
\x1b[36m【表示・スタイル設定】\x1b[0m\n  \
  -c, --classic              クラシックモード (カラー無効・アイコン無効・サイズ無効・+--/\\--記号) [デフォルト: 無効]\n  \
  -i, --icons                ファイル・フォルダに絵文字アイコンを表示 [デフォルト: 有効]\n  \
  -I, --no-icons             絵文字アイコンを非表示\n  \
  -s, --sizes                ファイルサイズを表示 ([1.2 KB] 等) [デフォルト: 有効]\n  \
  -S, --no-sizes             ファイルサイズを非表示\n  \
  -D, --dir-sizes, --du      フォルダ配下の総ファイル数および総容量を集計表示 [デフォルト: 無効]\n  \
  -l, --lines                ファイル行数を表示 (テキストファイル限定、10MB超はスキップ) [デフォルト: 無効]\n\n  \
\x1b[36m【出力フォーマット】\x1b[0m\n  \
  --json                     構造化データとしてJSON形式で出力\n  \
  --xml                      構造化データとしてXML形式で出力\n\n  \
\x1b[36m【フィルタリング・探索制御】\x1b[0m\n  \
  -a, --all                  隠しファイル・ディレクトリ (ドット開始) も表示 [デフォルト: 無効]\n  \
  -g, --gitignore            .gitignore ファイルを読み込んで除外パターンを自動適用 [デフォルト: 無効]\n  \
  --sort <MODE>              表示ソート順を指定 (name: 名前順, size: サイズ順, time: 更新日時順) [デフォルト: name]\n  \
  -r, --reverse              ソート順を逆順 (反転) に表示 [デフォルト: 無効]\n  \
  -d, --depth <DEPTH>        探索する最大の階層深さを制限 (例: -d 2) [デフォルト: 無制限]\n  \
  -e, --exclude <PATTERNS>   除外する名前・ワイルドカード(*,?)をカンマ区切り指定 [デフォルト: \"target,node_modules,.git\"]",
        env!("CARGO_PKG_VERSION")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_arguments_default() {
        let mut config = TreeConfig::default();
        let args = vec!["MyTree".to_string()];
        assert!(parse_arguments(&args, &mut config).is_ok());
        assert_eq!(config.target_path, PathBuf::from("."));
        assert!(config.show_icons);
        assert!(config.show_sizes);
        assert!(!config.show_dir_sizes);
        assert!(!config.show_hidden);
        assert!(!config.use_gitignore);
        assert_eq!(config.max_depth, 0);
        assert_eq!(config.output_format, OutputFormat::Text);
    }

    #[test]
    fn test_parse_arguments_custom() {
        let mut config = TreeConfig::default();
        let args = vec![
            "MyTree".to_string(),
            "-a".to_string(),
            "-g".to_string(),
            "-d".to_string(),
            "3".to_string(),
            "-e".to_string(),
            "*.tmp,build".to_string(),
            "./src".to_string(),
        ];
        assert!(parse_arguments(&args, &mut config).is_ok());
        assert_eq!(config.target_path, PathBuf::from("./src"));
        assert!(config.show_hidden);
        assert!(config.use_gitignore);
        assert_eq!(config.max_depth, 3);
        assert_eq!(config.exclude_patterns, vec!["*.tmp", "build"]);
    }

    #[test]
    fn test_parse_arguments_classic() {
        let mut config = TreeConfig::default();
        let args = vec!["MyTree".to_string(), "-c".to_string()];
        assert!(parse_arguments(&args, &mut config).is_ok());
        assert!(config.classic_mode);
        assert!(!config.show_icons);
        assert!(!config.show_sizes);
    }

    #[test]
    fn test_parse_arguments_no_icons_sizes() {
        let mut config = TreeConfig::default();
        let args = vec!["MyTree".to_string(), "-I".to_string(), "-S".to_string()];
        assert!(parse_arguments(&args, &mut config).is_ok());
        assert!(!config.show_icons);
        assert!(!config.show_sizes);
    }

    #[test]
    fn test_parse_arguments_dir_sizes() {
        let mut config = TreeConfig::default();
        let args = vec!["MyTree".to_string(), "-D".to_string()];
        assert!(parse_arguments(&args, &mut config).is_ok());
        assert!(config.show_dir_sizes);
    }

    #[test]
    fn test_parse_arguments_json_xml() {
        let mut config = TreeConfig::default();
        let args = vec!["MyTree".to_string(), "--json".to_string()];
        assert!(parse_arguments(&args, &mut config).is_ok());
        assert_eq!(config.output_format, OutputFormat::Json);

        let mut config = TreeConfig::default();
        let args = vec!["MyTree".to_string(), "--xml".to_string()];
        assert!(parse_arguments(&args, &mut config).is_ok());
        assert_eq!(config.output_format, OutputFormat::Xml);
    }

    #[test]
    fn test_parse_arguments_errors() {
        let mut config = TreeConfig::default();

        // 未知のオプション
        let args = vec!["MyTree".to_string(), "-x".to_string()];
        assert!(parse_arguments(&args, &mut config).is_err());

        // -d の値欠落
        let args = vec!["MyTree".to_string(), "-d".to_string()];
        assert!(parse_arguments(&args, &mut config).is_err());

        // -d の無効な数値
        let args = vec!["MyTree".to_string(), "-d".to_string(), "abc".to_string()];
        assert!(parse_arguments(&args, &mut config).is_err());

        // -e の値欠落
        let args = vec!["MyTree".to_string(), "-e".to_string()];
        assert!(parse_arguments(&args, &mut config).is_err());

        // 重複ターゲットパス指定
        let args = vec![
            "MyTree".to_string(),
            "./dir1".to_string(),
            "./dir2".to_string(),
        ];
        assert!(parse_arguments(&args, &mut config).is_err());

        // 無効なソートモード
        let args = vec![
            "MyTree".to_string(),
            "--sort".to_string(),
            "invalid".to_string(),
        ];
        assert!(parse_arguments(&args, &mut config).is_err());

        // --sort の値欠落
        let args = vec!["MyTree".to_string(), "--sort".to_string()];
        assert!(parse_arguments(&args, &mut config).is_err());
    }

    #[test]
    fn test_parse_arguments_sort() {
        let mut config = TreeConfig::default();
        let args = vec![
            "MyTree".to_string(),
            "--sort".to_string(),
            "size".to_string(),
            "-r".to_string(),
        ];
        assert!(parse_arguments(&args, &mut config).is_ok());
        assert_eq!(config.sort_mode, SortMode::Size);
        assert!(config.reverse_sort);

        let mut config = TreeConfig::default();
        let args = vec![
            "MyTree".to_string(),
            "--sort".to_string(),
            "TIME".to_string(),
            "--reverse".to_string(),
        ];
        assert!(parse_arguments(&args, &mut config).is_ok());
        assert_eq!(config.sort_mode, SortMode::Time);
        assert!(config.reverse_sort);
    }

    #[test]
    fn test_version_env() {
        assert_eq!(env!("CARGO_PKG_VERSION"), "0.1.5");
    }
}
