# Specification (SPEC.md) - MyTree

本文書は、`MyTree` (実行ファイル名: `MyTree.exe`) の機能仕様、CLI引数、表示制御、エラーハンドリング、動作要件に関する完全な技術仕様を定義します。

---

## 1. 概要と目標

- **目的**: ディレクトリ構造を高速かつ視認性の高いツリー形式でターミナルに出力する。
- **設計制約**: 外部クレートの依存関係ゼロ (`std` ライブラリのみを使用)。
- **対応OS**: Windows 10/11 (ネイティブWin32 APIでANSI有効化), Linux, macOS。

---

## 2. CLI コマンドライン仕様

### 2.1 構文
```text
MyTree [OPTIONS] [TARGET_PATH]
```

### 2.2 オプション引数一覧

| オプション | ロングフラグ | 引数 | デフォルト値 | 説明 |
| :--- | :--- | :--- | :--- | :--- |
| `-h` | `--help` | なし | - | ヘルプ表示（カテゴリ別グループ化表記）後、終了コード `0` で終了。 |
| `-v` | `--version` | なし | - | バージョン情報を標準出力に表示後、終了コード `0` で終了。 |
| | `--help-icons` | なし | - | ファイル名・拡張子・フォルダと絵文字アイコンの対応一覧表 (`icon::print_icons_help`) を標準出力に表示後、終了コード `0` で終了。 |
| `-c` | `--classic` | なし | `false` | クラシック表示モード。ANSIカラー無効、アイコン非表示、ファイルサイズ非表示、枝記号をASCII記号 (`+-- `, `\-- `, `|   `) で出力する。 |
| `-i` | `--icons` | なし | `true` (標準) | ディレクトリおよびファイル種別ごとの絵文字アイコンを表示する。 |
| `-I` | `--no-icons` | なし | `false` | 絵文字アイコンを非表示にする。 |
| `-s` | `--sizes` | なし | `true` (標準) | ファイル名の横にファイルサイズ (`[1.2 KB]` 等) を表示する。 |
| `-S` | `--no-sizes` | なし | `false` | ファイルサイズを非表示にする。 |
| `-D` | `--dir-sizes`, `--du` | なし | `false` | フォルダ名の横に、そのフォルダ配下の総ファイル数および総容量 (`[15 files, 4.2 MB]` 等) を集計・表示する。 |
| | `--json` | なし | `false` | 構造化データとして整形された JSON 形式でツリーを出力する。 |
| | `--xml` | なし | `false` | 構造化データとして XML 形式でツリーを出力する。 |
| `-a` | `--all` | なし | `false` | ドット (`.`) から始まる隠しファイル・ディレクトリを表示に含める。 |
| `-g` | `--gitignore` | なし | `false` | `.gitignore` ファイルから除外パターンを自動読み込みして適用する。 |
| `-d` | `--depth` | `<DEPTH>` (usize) | `0` (無制限) | ツリー探索の最大階層深さを制限する。 |
| `-e` | `--exclude` | `<PATTERNS>` (String) | `"target,node_modules,.git"` | 除外するディレクトリ名またはワイルドカードパターン (`*`, `?`) をカンマ区切りで指定する。 |

### 2.3 デフォルト設定構造体 (`TreeConfig`)

```rust
enum OutputFormat {
    Text,
    Json,
    Xml,
}

struct TreeConfig {
    target_path: PathBuf,        // デフォルト: "."
    show_icons: bool,            // デフォルト: true
    show_sizes: bool,            // デフォルト: true
    show_dir_sizes: bool,        // デフォルト: false (集計オプション指定時のみ有効化)
    show_hidden: bool,           // デフォルト: false
    max_depth: usize,            // デフォルト: 0 (無制限)
    exclude_patterns: Vec<String>, // デフォルト: ["target", "node_modules", ".git"]
    classic_mode: bool,          // デフォルト: false (クラシックモードオプション指定時有効化)
    output_format: OutputFormat, // デフォルト: OutputFormat::Text
    use_gitignore: bool,         // デフォルト: false (.gitignore読み込み指定時有効化)
}
```

---

## 3. 出力フォーマット仕様

### 3.1 枝記号 (Branch Connectors)
- **通常モード (デフォルト)**:
  - 中間要素: `├── `
  - 最終要素: `└── `
  - ディレクトリインデント: `│   ` (継続時) / `    ` (最終要素配下)
- **クラシックモード (`-c` 指定時)**:
  - 中間要素: `+-- `
  - 最終要素: `\-- `
  - ディレクトリインデント: `|   ` (継続時) / `    ` (最終要素配下)

### 3.2 ディレクトリ下位集計フォーマット (`-D` 指定時)
- 通常モード: ` \x1b[90m[{total_files} files, {total_size}]\x1b[0m`
- クラシックモード: ` [{total_files} files, {total_size}]`

### 3.3 アイコン・絵文字マップ (`get_icon_emoji`)

#### ディレクトリ
- `src` => `📂`
- `tests` / `test` => `🧪`
- `target` / `build` / `dist` / `out` => `🔨`
- `node_modules` / `vendor` => `📦`
- `.git` / `.github` => `🐙`
- `docs` / `doc` => `📚`
- `.vscode` / `.idea` => `💡`
- `public` / `assets` / `static` => `🖼`
- その他ディレクトリ => `📁`

#### ファイル・拡張子
- プログラミング言語 & Web:
  - `.rs` => `🦀` (Rust)
  - `.py`, `.pyw` => `🐍` (Python)
  - `.js`, `.jsx`, `.ts`, `.tsx`, `.mjs`, `.cjs` => `🟨` (JavaScript / TypeScript)
  - `.vue` => `💚` (Vue)
  - `.svelte` => `🟧` (Svelte)
  - `.astro` => `🚀` (Astro)
  - `.go` => `🐹` (Go)
  - `.rb` => `💎` (Ruby)
  - `.java`, `.jar`, `.class` => `☕` (Java)
  - `.swift` => `🦅` (Swift)
  - `.kt`, `.kts` => `🟪` (Kotlin)
  - `.dart` => `🎯` (Dart)
  - `.lua` => `🌙` (Lua)
  - `.c`, `.cpp`, `.h`, `.hpp`, `.cs` => `⚙` (C / C++ / C#)
  - `.php` => `🐘` (PHP)
- データベース & スキーマ & IaC:
  - `.sql`, `.db`, `.sqlite`, `.sqlite3` => `💾` (Database)
  - `.graphql`, `.gql`, `.prisma` => `🔷` (GraphQL / Prisma)
  - `.tf`, `.tfvars` => `🏗` (Terraform)
  - `.proto`, `.wasm` => `⚡` (Protobuf / WebAssembly)
- Office ドキュメント & 表計算:
  - `.xlsx`, `.xls`, `.csv`, `.tsv` => `📊` (Excel / 表計算)
  - `.docx`, `.doc` => `📘` (Word)
  - `.pptx`, `.ppt` => `📙` (PowerPoint)
  - `.pdf` => `📕` (PDF)
- テキスト & ドキュメント & 設定 & ロック:
  - `.txt`, `.text`, `.log` => `📜` (テキスト / ログ)
  - `.md`, `.markdown` => `📝` (Markdown)
  - `.json`, `.jsonc` => `📄` (JSON)
  - `.toml`, `.yaml`, `.yml`, `.ini`, `.conf`, `.config` => `🔧` (設定ファイル)
  - `.lock`, `Cargo.lock`, `package-lock.json`, `yarn.lock`, `pnpm-lock.yaml` => `🔒` (ロックファイル)
  - `Makefile`, `justfile`, `CMakeLists.txt` => `🛠` (ビルドツール)
  - `.env*` => `🔑` (環境変数)
  - `.gitignore`, `.dockerignore` => `🙈` (無視リスト)
  - `Dockerfile`, `docker-compose*` => `🐳` (Docker)
- 実行ファイル & スクリプト:
  - `.exe`, `.bat`, `.cmd`, `.sh`, `.ps1`, `.zsh`, `.bash`, `.fish` => `🚀`
- アーカイブ (圧縮):
  - `.zip`, `.tar`, `.gz`, `.7z`, `.rar`, `.bz2`, `.xz` => `📦`
- フォント & メディア:
  - フォント (`.ttf`, `.otf`, `.woff`, `.woff2`, `.eot`) => `🔤`
  - 画像 (`.png`, `.jpg`, `.jpeg`, `.gif`, `.svg`, `.webp`, `.ico`, `.bmp`, `.avif`, `.heic`, `.psd`, `.ai`) => `🖼`
  - 音声 (`.mp3`, `.wav`, `.flac`, `.aac`, `.ogg`, `.m4a`) => `🎵`
  - 動画 (`.mp4`, `.avi`, `.mkv`, `.mov`, `.webm`, `.wmv`) => `🎬`
- その他未定義ファイル => `📄`

### 3.4 カラーコード仕様 (ANSI Escape Codes)
- ルート行・ディレクトリ名: `\x1b[1;34m` (太字青) ※クラシックモード時無効
- ファイルサイズ / ディレクトリ集計情報: `\x1b[90m` (ダークグレー) ※クラシックモード時無効
- 最終統計情報 (`X directories, Y files`): `\x1b[36m` (シアン) ※クラシックモード時無効
- リセット: `\x1b[0m`

---

## 4. エラーハンドリング & 終了コード

| ケース | エラー出力 | 終了コード |
| :--- | :--- | :--- |
| 正常終了 | なし | `0` |
| `--help` 呼び出し | ヘルプ表示 (カテゴリ別・バージョン情報含む) | `0` |
| `--version` 呼び出し | バージョン表示 | `0` |
| `--help-icons` 呼び出し | アイコン対応一覧表示 (`icon::print_icons_help`) | `0` |
| 未知のオプション指定 | `【エラー】: 不明なオプションです: <ARG>` | `1` |
| パスが存在しない | `【エラー】: 指定されたパスが存在しません: <PATH>` | `1` |
| `-d` や `-e` の引数欠落 | `【エラー】: 深度の値が指定されていません...` | `1` |
| アクセス権限エラー (探索時) | エラーメッセージを出力せずスキップ | 継続 |
