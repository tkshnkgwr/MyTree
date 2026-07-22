# 貢献ガイドライン (CONTRIBUTING.md) - MyTree

[English](../en/CONTRIBUTING.md) | **日本語版**

`MyTree` プロジェクトへの貢献に興味を持っていただきありがとうございます！
本ドキュメントでは、バグ報告、機能提案、プルリクエスト提出時のガイドラインを説明します。

---

## 1. 開発方針と重要原則

開発を行う際は、以下の基本方針を遵守してください。

1. **外部依存ゼロ (Zero External Dependencies)**:
   - `Cargo.toml` にサードパーティクレートを追加しないでください。常に Rust 標準ライブラリ (`std`) のみで機能を実装します。
2. **クロスプラットフォーム配慮**:
   - Windows 固有処理（Win32 API等）には条件付きコンパイル `#[cfg(windows)]` を使用し、Linux/macOS でのビルドエラーを防いでください。
3. **多言語ドキュメントの同期**:
   - 仕様変更や機能追加を行う際は、`docs/ja/` および `docs/en/` の両方のドキュメントを更新してください。

---

## 2. 開発環境のセットアップ

1. **リポジトリのクローン**:
   ```bash
   git clone https://github.com/tkshnkgwr/MyTree.git
   cd MyTree
   ```
2. **動作確認**:
   ```bash
   cargo run
   ```

---

## 3. コミットおよびプルリクエスト手順

### コミットメッセージの規約
コミットメッセージには Conventional Commits 形式を使用してください：

- `feat:` 新機能の追加
- `fix:` バグ修正
- `docs:` ドキュメントの変更
- `refactor:` リファクタリング
- `perf:` パフォーマンス改善
- `test:` テストの追加・修正
- `chore:` ビルドスクリプトや設定の変更

### プルリクエスト作成前のチェックリスト
プルリクエストを送信する前に、以下のコマンドを実行し全て合格することを確認してください：

- [ ] `cargo test` （ユニットテスト合格）
- [ ] `cargo clippy --all-targets -- -D warnings` （静的解析の警告ゼロ）
- [ ] `cargo fmt --check` （コードフォーマット準拠）
- [ ] `cargo doc --no-deps --document-private-items` （ドキュメントビルドエラーゼロ）
