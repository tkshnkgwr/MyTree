# AI Development Instructions (INSTRUCTIONS.md) - MyTree

[English](../en/INSTRUCTIONS.md) | **日本語版**

本文書は、AIエージェント（大賢者）が `MyTree` のソースコードを変更・保守・機能追加する際に遵守すべきガイドラインおよび開発規約です。

---

## 1. 開発原則

1. **外部依存ゼロ (Zero External Dependencies) の厳守**:
   - `Cargo.toml` の `[dependencies]` にサードパーティクレートを追加してはなりません。常に Rust 標準ライブラリ (`std`) の機能を優先して活用してください。
2. **クロスプラットフォーム互換性**:
   - Windows 固有の処理 (`Win32 API` や `#[cfg(windows)]`) を追加する場合は、Linux/macOS 環境でビルドエラーにならないよう条件付きコンパイルを徹底してください。
3. **Rustdoc コメントの維持・同期**:
   - 新しい関数、構造体、モジュール、フィールドを追加した場合は、必ず `///` による Rustdoc ドキュメントコメントを記述してください。
4. **他言語開発経験者に配慮した解説方針**:
   - ユーザーは他言語での開発経験をお持ちですが、Rust / React / TypeScript 等の固有の専門用語（所有権、トレイト、フック等）には不慣れです。修正内容の説明時は、固有表現をむやみに並べず、一般的なプログラミング概念に置き換えるか直感的な補足解説を添えて報告してください。

---

## 2. コードスタイルのガイドライン

- **フォーマット規則**: `cargo fmt` の標準スタイルに従ってください。
- **型と変数名**:
  - 構造体 / 列挙型: `PascalCase` (例: `TreeConfig`)
  - 関数 / 変数 / モジュール: `snake_case` (例: `parse_arguments`)
  - 定数: `SCREAMING_SNAKE_CASE` (例: `STD_OUTPUT_HANDLE`)
- **エラー処理**:
  - パニック (`panic!`, `unwrap()`) の使用は回避し、可能な限り `Result<T, String>` や明示的なエラーハンドリングを行ってください。

---

## 3. テストと検証方針

- 新機能追加時には関連モジュールの `tests` モジュールにユニットテストを併記してください。
- 変更完了時には必ず以下の標準コマンドを実行し、警告およびエラーがゼロであることを確認してください：
  - `cargo test`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo fmt --check`
  - `cargo doc --no-deps --document-private-items`

---

## 4. モジュール分割とリファクタリング方針

- **1,000 行制限ルール**:
  - 単一のプログラムソースファイル（例: `src/main.rs`）が **1,000 行** を超えたあたりで、関心事・役割に応じたモジュール分割（例: `config.rs`, `tree.rs`, `serializer.rs`, `util.rs` 等）を提案・実施し、コードの可読性と保守性を維持してください。
