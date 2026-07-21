# Architecture (ARCHITECTURE.md) - MyTree

本文書は、`MyTree` (実行ファイル名: `MyTree.exe`) のシステムアーキテクチャ、モジュール設計、内部アルゴリズム、および技術選定の背景について記述します。

---

## 1. コンポーネント構造

本アプリケーションは関心事に統合・分割されたモジュール構造で構成されています。

```text
src/
├── main.rs                     # エントリポイント & Windows ANSI制御
├── config.rs                   # CLI引数パース & 設定構造体 (TreeConfig, OutputFormat)
├── tree.rs                     # ディレクトリ探索 & テキストツリー描画コアエンジン
├── serializer.rs               # JSON / XML 構造化データシリアライザエンジン
├── util.rs                     # 汎用ヘルパー関数 (ワイルドカード検索, バイト表示)
└── icon.rs                     # アイコン絵文字判定モジュール (ファイル名/拡張子マッピング)
```

### 1.1 モジュール役割
- **`src/main.rs`**:
  - `main()`: エントリポイントおよび全体フローマッチング
  - `enable_ansi_support()`: Windows ANSIエスケープ有効化 (Win32 API FFI)
- **`src/config.rs`**:
  - `TreeConfig`, `OutputFormat`: オプション設定構造体・フォーマット定義
  - `parse_arguments()` / `print_version()` / `print_help()`: CLIオプションパースおよびヘルプ出力
- **`src/tree.rs`**:
  - `print_tree()`: 再帰的ディレクトリ探索および標準構造ツリー描画
  - `format_root_line()` / `calculate_dir_summary()`: ルート行生成およびフォルダ容量・ファイル数集計
- **`src/serializer.rs`**:
  - `TreeNode`: メモリ内階層データ構造モデル
  - `build_tree_node()`: 階層ツリー構築関数
  - `serialize_json()` / `serialize_xml()`: 手動構造化シリアライザエンジン
  - `escape_json_str()` / `escape_xml_str()`: 特殊文字エスケープ
- **`src/util.rs`**:
  - `match_wildcard()`: ワイルドカード (*, ?) パターンマッチ処理
  - `format_byte_size()`: ファイルサイズ単位換算表記
- **`src/icon.rs`**:
  - `get_icon_emoji()`: 拡張子・ファイル名・ディレクトリ名に応じた絵文字マッピングエンジン

---

## 2. コア設計思想

### 2.1 外部依存性ゼロ (Zero-Dependency)
- `clap`, `structopt`, `colored`, `crossterm` などのサードパーティ製クレートを使わず、すべて Rust 標準ライブラリ (`std::env`, `std::fs`, `std::path`) のみで実装しています。
- **メリット**:
  - コンパイル時間がほぼ一瞬。
  - 依存ライブラリの脆弱性リスクや供給網攻撃の排除。
  - 生成バイナリサイズの大幅な削減 (通常 1MB 以下)。

### 2.2 Win32 API ダイレクト FFI (`enable_ansi_support`)
Windows 10/11 の標準コンソール (`cmd.exe`) や旧PowerShellではデフォルトで ANSI エスケープカラーが無効化されています。
本ツールでは `winapi` クレートを使わず、`extern "system"` を用いて `kernel32.dll` の関数を直接呼び出すことで、追加の依存関係なしに仮想ターミナル処理 (`ENABLE_VIRTUAL_TERMINAL_PROCESSING = 0x0004`) を有効化しています。

---

## 3. ディレクトリ探索 & ソートアルゴリズム

### 3.1 ディレクトリ優先ソート (Directory-First Sorting)
ツリー構造の視認性を向上させるため、各階層の読込結果 (`ReadDir`) は以下のルールに従って並べ替えられます：

1. ディレクトリとファイルを比較する場合、**ディレクトリを優先**。
2. ディレクトリ同士、ファイル同士の場合は、**ファイル名（辞書順）**で並べ替え。

```rust
valid_entries.sort_by(|a, b| {
    let is_dir_a = a.path().is_dir();
    let is_dir_b = b.path().is_dir();

    if is_dir_a != is_dir_b {
        if is_dir_a { std::cmp::Ordering::Less } else { std::cmp::Ordering::Greater }
    } else {
        a.file_name().cmp(&b.file_name())
    }
});
```

### 3.2 枝記号インデント制御
再帰呼び出し時に `prefixes: &mut Vec<&str>` をスタック渡しすることにより、不必要な文字列結合やヒープ確保を回避しながら正しい階層境界線 (`│   ` または `    `) を描画します。

---

## 4. 機械可読データシリアライズ設計 (`--json` / `--xml`)

`serde` や `quick-xml` などの外部シリアライズクレートを導入せず、`std` のみで JSON / XML 構造を出力するための階層モデルと文字列エスケープ機構を備えています。

### 4.1 メモリ内ツリーモデル (`TreeNode`)
`build_tree_node()` 関数が再帰的にディレクトリを巡回し、フィルタ・ソート適用済みのインメモリツリー構造体を生成します。

```rust
struct TreeNode {
    name: String,
    path: String,
    is_dir: bool,
    size: u64,
    children: Vec<TreeNode>,
}
```

### 4.2 文字エスケープ関数 (`escape_json_str` / `escape_xml_str`)
- **JSON**: ダブルクォート (`\"`)、バックスラッシュ (`\\`)、制御文字 (`\n`, `\r`, `\t`, `\uXXXX`) を安全にエスケープ。
- **XML**: 特殊エンティティ文字 (`&amp;`, `&lt;`, `&gt;`, `&quot;`, `&apos;`) をエスケープ。

---

## 5. CI/CD パイプライン構成 (GitHub Actions)

本プロジェクトでは、コード品質の自動検証およびリリース配布の自動化のために GitHub Actions ワークフローを導入しています。

### 5.1 CI ワークフロー (`.github/workflows/ci.yml`)
`main` ブランチへの Push / PR 時に自動実行されます（ドキュメント類のみの変更時は自動スキップ）。
- **検証項目**: `cargo test`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`, `cargo doc`, `cargo build --release`

### 5.2 Release ワークフロー (`.github/workflows/release.yml`)
`v*` (例: `v0.1.1`, `v1.0.0`) タグの Push 時のみトリガーされる明示的なリリース自動化パイプラインです。
- リリース用 Windows バイナリ (`MyTree.exe`) を生成・Zip 圧縮 (`MyTree-windows-x64.zip`) し、GitHub Release のアセットとして自動添付します。

