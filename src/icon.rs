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
            "docs" | "doc" => "📚",
            ".vscode" | ".idea" => "💡",
            "public" | "assets" | "static" => "🖼",
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
        if lower_name == "makefile" || lower_name == "justfile" || lower_name == "cmakelists.txt" {
            return "🛠";
        }
        if lower_name.ends_with(".lock")
            || lower_name == "cargo.lock"
            || lower_name == "package-lock.json"
            || lower_name == "yarn.lock"
            || lower_name == "pnpm-lock.yaml"
        {
            return "🔒";
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
            "vue" => "💚",
            "svelte" => "🟧",
            "astro" => "🚀",
            "go" => "🐹",
            "rb" => "💎",
            "java" | "jar" | "class" => "☕",
            "swift" => "🦅",
            "kt" | "kts" => "🟪",
            "dart" => "🎯",
            "lua" => "🌙",
            "c" | "cpp" | "h" | "hpp" | "cs" => "⚙",
            "php" => "🐘",
            "exe" | "bat" | "cmd" | "sh" | "ps1" | "zsh" | "bash" | "fish" => "🚀",

            // Web & スタイル & DB & API
            "html" | "htm" => "🌐",
            "css" | "scss" | "sass" | "less" => "🎨",
            "sql" | "db" | "sqlite" | "sqlite3" => "💾",
            "graphql" | "gql" | "prisma" => "🔷",

            // インフラ & 通信 & バイナリ
            "tf" | "tfvars" => "🏗",
            "proto" | "wasm" => "⚡",

            // Office ドキュメント
            "xlsx" | "xls" | "csv" | "tsv" => "📊",
            "docx" | "doc" => "📘",
            "pptx" | "ppt" => "📙",
            "pdf" => "📕",

            // テキスト & 構造化データ & ロック
            "txt" | "text" | "log" => "📜",
            "md" | "markdown" => "📝",
            "json" | "jsonc" => "📄",
            "lock" => "🔒",
            "toml" | "yaml" | "yml" | "ini" | "conf" | "config" => "🔧",

            // アーカイブ
            "zip" | "tar" | "gz" | "7z" | "rar" | "bz2" | "xz" => "📦",

            // フォント
            "ttf" | "otf" | "woff" | "woff2" | "eot" => "🔤",

            // メディア
            "png" | "jpg" | "jpeg" | "gif" | "svg" | "webp" | "ico" | "bmp" | "avif" | "heic"
            | "psd" | "ai" => "🖼",
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
  📚  docs, doc                  (ドキュメントフォルダ)\n  \
  💡  .vscode, .idea             (IDE設定フォルダ)\n  \
  🖼  public, assets, static     (静的アセットフォルダ)\n  \
  📁  その他一般フォルダ\n\n\
\x1b[36m【プログラミング言語 & スクリプト】\x1b[0m\n  \
  🦀  .rs                        (Rust)\n  \
  🐍  .py, .pyw                  (Python)\n  \
  🟨  .js, .jsx, .ts, .tsx       (JavaScript / TypeScript)\n  \
  💚  .vue                       (Vue)\n  \
  🟧  .svelte                    (Svelte)\n  \
  🐹  .go                        (Go)\n  \
  💎  .rb                        (Ruby)\n  \
  ☕  .java, .jar, .class        (Java)\n  \
  🦅  .swift                     (Swift)\n  \
  🟪  .kt, .kts                  (Kotlin)\n  \
  🎯  .dart                      (Dart)\n  \
  🌙  .lua                       (Lua)\n  \
  ⚙  .c, .cpp, .h, .hpp, .cs    (C / C++ / C#)\n  \
  🐘  .php                       (PHP)\n  \
  🚀  .exe, .bat, .cmd, .sh, .ps1, .zsh, .bash, .fish, .astro (実行ファイル・スクリプト)\n\n\
\x1b[36m【Web & デザイン & データベース & インフラ】\x1b[0m\n  \
  🌐  .html, .htm                (HTML)\n  \
  🎨  .css, .scss, .sass, .less  (スタイルシート)\n  \
  💾  .sql, .db, .sqlite         (データベース / SQL)\n  \
  🔷  .graphql, .gql, .prisma    (GraphQL / Prisma)\n  \
  🏗  .tf, .tfvars               (Terraform)\n  \
  ⚡  .proto, .wasm              (Protobuf / WebAssembly)\n\n\
\x1b[36m【Office & ドキュメント & 設定】\x1b[0m\n  \
  📊  .xlsx, .xls, .csv, .tsv    (Excel / 表計算)\n  \
  📘  .docx, .doc                (Word / 文書)\n  \
  📙  .pptx, .ppt                (PowerPoint / プレゼン)\n  \
  📕  .pdf                       (PDF)\n  \
  📜  .txt, .text, .log          (テキスト / ログ)\n  \
  📝  .md, .markdown             (Markdown)\n  \
  📄  .json, .jsonc              (JSON)\n  \
  🔧  .toml, .yaml, .yml, .ini (設定ファイル)\n  \
  🔒  .lock, Cargo.lock, yarn.lock (ロックファイル)\n  \
  🛠  Makefile, justfile         (ビルドツール)\n  \
  🔑  .env*                      (環境変数)\n  \
  🙈  .gitignore, .dockerignore  (無視リスト)\n  \
  🐳  Dockerfile, docker-compose (Docker)\n\n\
\x1b[36m【フォント & メディア & アーカイブ】\x1b[0m\n  \
  🔤  .ttf, .otf, .woff, .woff2  (フォント)\n  \
  🖼  .png, .jpg, .gif, .svg, .avif, .psd (画像)\n  \
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
        assert_eq!(get_icon_emoji("docs", true), "📚");
        assert_eq!(get_icon_emoji(".vscode", true), "💡");
        assert_eq!(get_icon_emoji("public", true), "🖼");
        assert_eq!(get_icon_emoji("unknown", true), "📁");

        assert_eq!(get_icon_emoji("main.rs", false), "🦀");
        assert_eq!(get_icon_emoji("Cargo.toml", false), "🔧");
        assert_eq!(get_icon_emoji("Cargo.lock", false), "🔒");
        assert_eq!(get_icon_emoji("Makefile", false), "🛠");
        assert_eq!(get_icon_emoji("App.vue", false), "💚");
        assert_eq!(get_icon_emoji("App.svelte", false), "🟧");
        assert_eq!(get_icon_emoji("main.swift", false), "🦅");
        assert_eq!(get_icon_emoji("Main.kt", false), "🟪");
        assert_eq!(get_icon_emoji("main.dart", false), "🎯");
        assert_eq!(get_icon_emoji("script.lua", false), "🌙");
        assert_eq!(get_icon_emoji("schema.graphql", false), "🔷");
        assert_eq!(get_icon_emoji("main.tf", false), "🏗");
        assert_eq!(get_icon_emoji("service.proto", false), "⚡");
        assert_eq!(get_icon_emoji("font.woff2", false), "🔤");
        assert_eq!(get_icon_emoji("banner.avif", false), "🖼");
        assert_eq!(get_icon_emoji("README.md", false), "📝");
        assert_eq!(get_icon_emoji(".gitignore", false), "🙈");
    }

    #[test]
    fn test_get_icon_emoji_case_insensitive() {
        assert_eq!(get_icon_emoji("SRC", true), "📂");
        assert_eq!(get_icon_emoji("DOCS", true), "📚");
        assert_eq!(get_icon_emoji("MAIN.RS", false), "🦀");
        assert_eq!(get_icon_emoji("CARGO.LOCK", false), "🔒");
        assert_eq!(get_icon_emoji("MAKEFILE", false), "🛠");
        assert_eq!(get_icon_emoji("APP.VUE", false), "💚");
    }
}
