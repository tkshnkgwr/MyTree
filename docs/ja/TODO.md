# TODO & Roadmap (TODO.md) - MyTree

[English](../en/TODO.md) | **日本語版**

本文書は、`MyTree` の完了済み機能、現在対応中/計画中のタスク、および将来の拡張アイデアを管理します。

---

## 1. 完了した機能 (Done)

- [x] **標準ライブラリ（`std`）のみによる初期実装** (依存ゼロ軽量構成)
- [x] **主要CLIオプションの実装**:
  - `-h`, `--help`: ヘルプメニュー出力 (カテゴリ別グループ化 & デフォルト値表記)
  - `-v`, `--version`: バージョン情報表示
  - `--help-icons`: アイコン絵文字の対応一覧表を表示
  - `-c`, `--classic`: クラシック表示モード (無色・アイコンなし・サイズなし・ASCII枝記号)
  - `-i`, `--icons`: ディレクトリ/ファイル別絵文字アイコン表示
  - `-I`, `--no-icons`: アイコン非表示
  - `-s`, `--sizes`: ファイルサイズフォーマット表示 (`B`, `KB`, `MB`)
  - `-S`, `--no-sizes`: ファイルサイズ非表示
  - `-D`, `--dir-sizes`, `--du`: フォルダ配下の総ファイル数および総容量を集計表示
  - `-a`, `--all`: ドット (`.`) 隠しファイル表示
  - `-g`, `--gitignore`: `.gitignore` ファイルから除外パターンを自動読み込み・適用する機能
  - `-d`, `--depth`: ツリー探索の階層制限
  - `-e`, `--exclude`: 除外ディレクトリ名 / ワイルドカードパターンの指定
- [x] **GitHub Actions CI/CD パイプラインの構築**:
  - `.github/workflows/ci.yml`: `main` ブランチへの Push / PR 時に `cargo test`, `clippy`, `fmt`, `doc`, `build` を自動実行
  - `.github/workflows/release.yml`: `v*` (例: `v0.1.1`) タグの Push 時のみ起動し、Windows Release バイナリ (`MyTree-windows-x64.zip`) を GitHub Release へ自動デプロイ
- [x] **ワイルドカード / Glob パターン除外機能 (`match_wildcard`)**:
  - 外部ライブラリなしで `*.tmp`, `*.log`, `build*`, `test?.rs` 等のパターン除外に対応
- [x] **JSON / XML 構造化出力フォーマット (`--json` / `--xml`)**:
  - 他ツールやプログラムから利用可能な整形済み JSON / XML 形式でツリーおよびサマリーを出力
  - 特殊文字エスケープ処理 (`escape_json_str`, `escape_xml_str`) を `std` のみで完全手動実装
- [x] **拡張絵文字アイコンマッピングの追加 (`src/icon.rs`)**:
  - フォルダアイコン (`docs`, `.vscode`, `public` 等) やプログラミング言語・フレームワーク (`.swift`, `.kt`, `.vue`, `.svelte` 等)・ロックファイル・フォント・メディア等の網羅的追加
- [x] **モジュール分割リファクタリング (1,000行ルール準拠)**:
  - 単一ファイルであった `src/main.rs` を関心事ごとに 5 つのモジュール (`main.rs`, `config.rs`, `tree.rs`, `serializer.rs`, `util.rs`, `icon.rs`) に分割し高保守性を実現
- [x] **ユニットテストの全領域への拡充 (17テスト)**:
  - 引数解析、サイズフォーマット、アイコン判定、ワイルドカード、.gitignoreパース、ディレクトリ集計、JSON/XMLシリアライザ等すべてのモジュールを網羅
- [x] **Windows 10/11 ANSIカラー出力を自動有効化** (`kernel32.dll` ダイレクト Win32 API FFI)
- [x] **ディレクトリ優先ソート機能** (視認性の向上)
- [x] **完全ドキュメンテーション体系の構築**:
  - `README.md` / `README_JA.md`
  - `SPEC.md`, `ARCHITECTURE.md`, `DIAGRAM.md`, `FOOTPRINTS.md`, `TODO.md`, `INSTRUCTIONS.md`, `CHANGELOG.md`
  - `LICENSE` (MIT)

---

## 2. 近期の改善・拡張予定 (Todo / In Progress)

- [ ] **表示ソート機能の強化 (`--sort`)**:
  - 名前順 (`name`)、ファイルサイズ順 (`size`)、更新日時順 (`time`) でのソートオプション追加
- [ ] **カラー出力制御フラグ (`--color <MODE>`) のサポート**:
  - `auto`, `always`, `never` によるターミナルリダイレクト時のカラー制御最適化

---

## 3. 将来の拡張アイデア (Backlog)

- [ ] **カスタムカラー・絵文字マップ設定 (`~/.mytreerc` / `icons.json`)**:
  - ユーザー独自の表示テーマ変更および外部設定ファイルのオプショナル読み込み
- [ ] **ファイル数/ディレクトリ数に加えた総容量の表示強化**:
  - ルートまたは末尾のサマリー行に、探索した全ファイルの合計サイズを分かりやすく表示する機能
