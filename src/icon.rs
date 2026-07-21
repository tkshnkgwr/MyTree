//! アイコン・絵文字判定モジュール
//!
//! ファイル名や拡張子、ディレクトリ名に応じた絵文字アイコンの取得ロジックを提供します。

use std::path::Path;

/// 拡張子やフォルダ名に応じた絵文字の取得
pub fn get_icon_emoji(name: &str, is_directory: bool) -> &'static str {
    let lower_name = name.to_lowercase();
    if is_directory {
        match lower_name.as_str() {
            "src" => "📂",
            "tests" | "test" => "🧪",
            "target" | "build" | "dist" | "out" => "🔨",
            "node_modules" | "vendor" => "📦",
            ".git" | ".github" => "🐙",
            _ => "📁",
        }
    } else {
        // 特別なファイル名判定
        if lower_name.starts_with(".env") {
            return "🔑";
        }
        if lower_name == ".gitignore" || lower_name == ".dockerignore" {
            return "🙈";
        }
        if lower_name == "dockerfile" || lower_name.starts_with("docker-compose") {
            return "🐳";
        }

        let path = Path::new(name);
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        match ext.as_str() {
            // プログラミング言語 & スクリプト
            "rs" => "🦀",
            "py" | "pyw" => "🐍",
            "js" | "jsx" | "ts" | "tsx" | "mjs" | "cjs" => "🟨",
            "go" => "🐹",
            "rb" => "💎",
            "java" | "jar" | "class" => "☕",
            "c" | "cpp" | "h" | "hpp" | "cs" => "⚙",
            "php" => "🐘",
            "exe" | "bat" | "cmd" | "sh" | "ps1" => "🚀",

            // Web & スタイル
            "html" | "htm" => "🌐",
            "css" | "scss" | "sass" | "less" => "🎨",

            // データベース & SQL
            "sql" | "db" | "sqlite" | "sqlite3" => "💾",

            // Office ドキュメント
            "xlsx" | "xls" | "csv" | "tsv" => "📊",
            "docx" | "doc" => "📘",
            "pptx" | "ppt" => "📙",
            "pdf" => "📕",

            // テキスト & 構造化データ
            "txt" | "text" | "log" => "📜",
            "md" | "markdown" => "📝",
            "json" | "jsonc" => "📄",
            "toml" | "yaml" | "yml" | "ini" | "conf" | "config" => "🔧",

            // アーカイブ
            "zip" | "tar" | "gz" | "7z" | "rar" | "bz2" | "xz" => "📦",

            // メディア
            "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" | "ico" | "bmp" => "🖼",
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" => "🎵",
            "mp4" | "avi" | "mkv" | "mov" | "webm" | "wmv" => "🎬",

            _ => "📄",
        }
    }
}

/// アイコン絵文字対応一覧ヘルプの出力
pub fn print_icons_help() {
    println!(
        "\x1b[1mMyTree アイコン絵文字対応表 (絵文字マップ)\x1b[0m\n\n\
\x1b[36m【フォルダ】\x1b[0m\n  \
  📂  src                        (ソースフォルダ)\n  \
  🧪  tests, test                (テストフォルダ)\n  \
  🔨  target, build, dist, out   (ビルド・出力フォルダ)\n  \
  📦  node_modules, vendor       (依存ライブラリフォルダ)\n  \
  🐙  .git, .github              (Git管理フォルダ)\n  \
  📁  その他一般フォルダ\n\n\
\x1b[36m【プログラミング言語 & スクリプト】\x1b[0m\n  \
  🦀  .rs                        (Rust)\n  \
  🐍  .py, .pyw                  (Python)\n  \
  🟨  .js, .jsx, .ts, .tsx       (JavaScript / TypeScript)\n  \
  🐹  .go                        (Go)\n  \
  💎  .rb                        (Ruby)\n  \
  ☕  .java, .jar, .class        (Java)\n  \
  ⚙  .c, .cpp, .h, .hpp, .cs    (C / C++ / C#)\n  \
  🐘  .php                       (PHP)\n  \
  🚀  .exe, .bat, .cmd, .sh, .ps1(実行ファイル・スクリプト)\n\n\
\x1b[36m【Web & デザイン & データベース】\x1b[0m\n  \
  🌐  .html, .htm                (HTML)\n  \
  🎨  .css, .scss, .sass, .less  (スタイルシート)\n  \
  💾  .sql, .db, .sqlite         (データベース / SQL)\n\n\
\x1b[36m【Office & ドキュメント】\x1b[0m\n  \
  📊  .xlsx, .xls, .csv, .tsv    (Excel / 表計算)\n  \
  📘  .docx, .doc                (Word / 文書)\n  \
  📙  .pptx, .ppt                (PowerPoint / プレゼン)\n  \
  📕  .pdf                       (PDF)\n  \
  📜  .txt, .text, .log          (テキスト / ログ)\n  \
  📝  .md, .markdown             (Markdown)\n  \
  📄  .json, .jsonc              (JSON)\n  \
  🔧  .toml, .yaml, .yml, .ini (設定ファイル)\n  \
  🔑  .env*                      (環境変数)\n  \
  🙈  .gitignore, .dockerignore  (無視リスト)\n  \
  🐳  Dockerfile, docker-compose (Docker)\n\n\
\x1b[36m【メディア & アーカイブ】\x1b[0m\n  \
  🖼  .png, .jpg, .gif, .svg     (画像)\n  \
  🎵  .mp3, .wav, .flac, .m4a     (音声)\n  \
  🎬  .mp4, .avi, .mkv, .webm    (動画)\n  \
  📦  .zip, .tar, .gz, .7z, .rar  (圧縮アーカイブ)\n  \
  📄  その他一般ファイル"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_icon_emoji() {
        assert_eq!(get_icon_emoji("src", true), "📂");
        assert_eq!(get_icon_emoji("unknown", true), "📁");
        assert_eq!(get_icon_emoji("main.rs", false), "🦀");
        assert_eq!(get_icon_emoji("Cargo.toml", false), "🔧");
        assert_eq!(get_icon_emoji("README.md", false), "📝");
        assert_eq!(get_icon_emoji(".gitignore", false), "🙈");
        assert_eq!(get_icon_emoji("schema.sql", false), "💾");
        assert_eq!(get_icon_emoji("notes.txt", false), "📜");
        assert_eq!(get_icon_emoji("data.xlsx", false), "📊");
        assert_eq!(get_icon_emoji("report.docx", false), "📘");
        assert_eq!(get_icon_emoji("slides.pptx", false), "📙");
        assert_eq!(get_icon_emoji("script.py", false), "🐍");
        assert_eq!(get_icon_emoji("index.html", false), "🌐");
    }

    #[test]
    fn test_get_icon_emoji_case_insensitive() {
        assert_eq!(get_icon_emoji("SRC", true), "📂");
        assert_eq!(get_icon_emoji("MAIN.RS", false), "🦀");
        assert_eq!(get_icon_emoji("CARGO.TOML", false), "🔧");
        assert_eq!(get_icon_emoji(".ENV", false), "🔑");
        assert_eq!(get_icon_emoji("DOCKERFILE", false), "🐳");
        assert_eq!(get_icon_emoji("SCHEMA.SQL", false), "💾");
    }
}
