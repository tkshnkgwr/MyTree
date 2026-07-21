# 変更履歴 (CHANGELOG)

このプロジェクトのすべての重要な変更は、このファイルに記録されます。

フォーマットは [Keep a Changelog](https://keepachangelog.com/ja/1.0.0/) に基づいており、
このプロジェクトは [セマンティック バージョニング](https://semver.org/lang/ja/) に準拠しています。

## [Unreleased] / [0.1.1] - 2026-07-21

### Optimized
- `src/main.rs` のモジュール分割リファクタリング:
  - 1,000行を超えた `main.rs` を関心事ごとに 5 つのモジュール (`config.rs`, `tree.rs`, `serializer.rs`, `util.rs`, `main.rs`) に分割。
  - テストコードも各モジュールに適切に整理・分散配置。

### Added
- GitHub Actions CI / Release ワークフローの構築 (`.github/workflows/`):
  - `ci.yml`: `main` ブランチへの Push / PR 時に `cargo test`, `clippy`, `fmt`, `doc`, `build` の自動検証を実行。
  - `release.yml`: `v*` (例: `v0.1.1`) タグ Push 時のみ動作し、Windows リリースバイナリ Zip (`MyTree-windows-x64.zip`) を生成して GitHub Release に自動デプロイ。
- `.gitignore` ファイルの自動読み込み・除外パターン適用オプション (`-g` / `--gitignore`):
  - 外部依存ゼロ（`std::fs` のみ）による `.gitignore` パース関数 `parse_gitignore_file` の実装。
  - ディレクトリ探索および構造化データ (`--json` / `--xml`) シリアライズ時の動的パターン除外統合。
  - `.gitignore` パース処理に対するユニットテストの追加。
- ディレクトリおよびファイル絵文字アイコンの対応拡張子・特殊ファイル名の拡張 (`src/icon.rs`):
  - フォルダアイコンの拡充 (`docs`, `.vscode`, `.idea`, `public`, `assets`, `static`)
  - ロックファイル (`.lock`, `Cargo.lock`, `package-lock.json` 等 => 🔒)
  - ビルドツール (`Makefile`, `justfile`, `CMakeLists.txt` => 🛠)
  - モダンプログラミング言語・フレームワーク (`.swift`, `.kt`, `.dart`, `.lua`, `.vue`, `.svelte`, `.astro` 等)
  - スキーマ & IaC (`.graphql`, `.prisma`, `.tf`, `.proto`, `.wasm`)
  - フォント (`.ttf`, `.woff` 等 => 🔤) および追加画像フォーマット (`.avif`, `.psd` 等)
  - ヘルプ表示 (`--help-icons`) およびユニットテストの同期更新。
- 機械可読構造化データ形式への出力サポート (`--json` / `--xml`):
  - `--json`: 他ツールやプログラムから利用可能な整形済み JSON オブジェクト形式で出力。
  - `--xml`: 階層ノードとサマリー情報を含む構造化 XML 形式で出力。
  - `std` のみによる完全手動シリアライズおよび文字エスケープ処理 (`escape_json_str`, `escape_xml_str`) の実装。
  - メモリ内階層ノード構築モデル `TreeNode` および再帰構築関数 `build_tree_node` の追加。
  - 引数解析、JSON/XML エスケープ、シリアライズに関するユニットテストの追加・拡充 (合計 16 テストへ増加)。

## [0.1.0] - 2026-07-21

### Added
- `MyTree` の初版リリース。
- Windows およびクロスプラットフォームのターミナル環境で動作する、外部依存ゼロの軽量 Tree CLI ユーティリティ。
- クラシック表示モード (`-c` / `--classic`) の追加:
  - ANSIカラー無効・アイコン無効・ファイルサイズ無効・ASCII記号 (`+-- `, `\-- `, `|   `) による純粋なプレーンテキスト表示。
- ワイルドカード (`*`, `?`) パターン除外機能 (`match_wildcard`) の自前軽量実装:
  - `-e "*.tmp,*.log,build*"` のようなフレキシブルなワイルドカード除外に対応。
- 全機能領域を網羅するユニットテスト群の構築 (合計 12 テスト):
  - 引数解析の正常系および異常系 (未知オプション、引数欠落、重複引数)
  - ファイルサイズ整形 (`format_byte_size`) の境界値テスト (0B, 1023B, 1024B, 1MB 等)
  - アイコン判定の大文字小文字テスト
  - ワイルドカードマッチの正常・異常系テスト
  - ディレクトリ集計計算およびクラシックモードパーステスト
- 主要な CLI フラグの実装:
  - `-h`, `--help`: ヘルプメニューを表示（カテゴリ別グループ化 & デフォルト動作表示）。
  - `-v`, `--version`: バージョン情報を表示して終了。
  - `--help-icons`: ファイル名・拡張子・フォルダと絵文字アイコンの対応一覧を表示して終了。
  - `-c`, `--classic`: クラシック表示モード (無色・アイコンなし・サイズなし・ASCII枝記号)。
  - `-i`, `--icons`: ファイルやフォルダに絵文字アイコンを付与。
  - `-I`, `--no-icons`: アイコン絵文字を表示しない (アイコン非表示)。
  - `-s`, `--sizes`: フォーマットされたファイルサイズ (`B`, `KB`, `MB`) を表示。
  - `-S`, `--no-sizes`: ファイルサイズを表示しない (サイズ非表示)。
  - `-D`, `--dir-sizes`, `--du`: フォルダ配下の総ファイル数および総容量を集計・表示 (`[X files, Y KB]`)。
  - `-a`, `--all`: ドット (`.`) から始まる隠しファイル・ディレクトリを含めて表示。
  - `-d`, `--depth <DEPTH>`: ディレクトリ探索の最大階層深さを制限。
  - `-e`, `--exclude <PATTERNS>`: カンマ区切りの除外ディレクトリ名リスト / ワイルドカードパターンを指定。
- 拡張絵文字アイコンマッピングの追加 (テキスト, SQL, Office, Python, Web, メディア, 圧縮等)。
- 専用アイコンモジュール (`src/icon.rs`) への分離・リファクタリング。
- Windows 10/11 の標準コンソールにおける ANSI 仮想ターミナルカラー処理の自動有効化機能 (`enable_ansi_support`)。
- ツリー表示の見やすさを向上させるディレクトリ優先ソートアルゴリズム。
- 総合的なドキュメント体系の構築 (`SPEC.md`, `ARCHITECTURE.md`, `DIAGRAM.md`, `FOOTPRINTS.md`, `TODO.md`, `INSTRUCTIONS.md`, `README.md`, `README_JA.md`)。
- MIT ライセンスの適用。
