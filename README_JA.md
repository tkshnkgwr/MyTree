# MyTree 🌲

[![License: GPL-3.0-or-later](https://img.shields.io/badge/License-GPL--3.0--or--later-blue.svg)](LICENSE)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)
[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](https://www.rust-lang.org/)
![Version](https://img.shields.io/badge/version-0.1.7-green.svg)
[![CI](https://github.com/tkshnkgwr/MyTree/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/MyTree/actions/workflows/ci.yml)

[English README](README.md) | **日本語版**

WindowsのコマンドプロンプトやPowerShell（およびLinux/macOS環境）でディレクトリ構造を美しいツリー形式で表示するためのRust製CLIツールです。
外部依存クレート（`clap`等）を一切使用せず、Rust標準ライブラリ（`std`）のみで構築されているため、超軽量・高速に動作します。

---

## ✨ 主な特徴

- 🚀 **超軽量・高速**: 外部依存ゼロ（`std` のみ）。コンパイル速度と実行性能を極限まで高めています。
- 🎨 **カラー＆絵文字表示**: ANSIエスケープシーケンスによるカラー表示と、フォルダ・ファイル種別ごとの絵文字表示に対応（非表示切り替えも可能）。
- 📜 **クラシック表示モード (`-c`)**: カラー無効・アイコン無効・ファイルサイズ無効・ASCII記号 (`+-- `, `\-- `, `|   `) によるDOS風シンプルテキスト表示に対応。
- 💻 **Windows標準コンソール最適化**: Win32 APIを直接使用して、Windows 10/11のコマンドプロンプトやPowerShellでANSIカラー出力を自動有効化。
- 🔍 **柔軟なカスタマイズ**:
  - クラシック表示モード (`-c` / `--classic`)
  - `.gitignore` 自動読み込み・適用 (`-g` / `--gitignore`)
  - フォルダ配下の総容量・総ファイル数集計表示 (`-D` / `--dir-sizes` / `--du`)
  - ストリーミング逐次ファイル行数カウント表示 (`-l` / `--lines`)
  - 多彩なソート順・反転表示 (`--sort <MODE>`, `-r` / `--reverse`)
  - 構造化データ形式での出力 (`--json`, `--xml`)
  - ワイルドカード (`*`, `?`) パターン除外 (`-e "*.log,*.tmp"`)
  - アイコン非表示 (`-I` / `--no-icons`)
  - ファイルサイズ非表示 (`-S` / `--no-sizes`)
  - 探索階層（深さ）の制限 (`-d`)
  - 隠しファイル・ディレクトリの表示切り替え (`-a` / `--all`)
  - バージョン表示 (`-v` / `--version`)
  - アイコン絵文字対応一覧ヘルプ (`--help-icons`)

---

## 🚀 インストール & ビルド手順

### 前提条件
- [Rust](https://rustup.rs/) (2021 edition 以降)

### リリースビルド
```bash
# クローンまたはダウンロードしたディレクトリに移動
cd MyTree

# 最適化ビルド
cargo build --release
```

ビルドが完了すると、`target/release/MyTree.exe` (Linux/macOSの場合は `target/release/MyTree`) が生成されます。

---

## 💻 使い方・実行例

```bash
# カレントディレクトリをツリー表示
MyTree

# クラシックモード (カラーなし・アイコンなし・サイズなし・+--記号) で表示
MyTree -c

# フォルダごとの配下ファイル数・合計容量を集計して表示
MyTree -D

# テキストファイルの行数を表示
MyTree -l

# ファイルサイズ順で昇順（小さい順・逆順）にソートして表示（※デフォルトの --sort size は降順）
MyTree --sort size -r

# JSON フォーマットで構造化データを出力
MyTree --json

# XML フォーマットで構造化データを出力
MyTree --xml

# バージョンを表示
MyTree -v

# アイコン絵文字の対応一覧を表示
MyTree --help-icons

# 特定の拡張子やパターンを除外して表示
MyTree -e "*.log,*.tmp,target"

# ディレクトリ深さを2に制限
MyTree -d 2

# .gitignore の除外ルールを自動的に読み込んで適用
MyTree -g

# 特定のディレクトリを指定して実行
MyTree "%USERPROFILE%\Documents"
```

### 📸 実行結果（出力例）

#### デフォルト表示 (`MyTree`)
```text
📁 .
├── 📚 docs
│   ├── 📁 en
│   │   ├── 📝 ARCHITECTURE.md [6.1 KB]
│   │   ├── 📝 CHANGELOG.md [7.7 KB]
│   │   ├── 📝 CONTRIBUTING.md [1.9 KB]
│   │   ├── 📝 DIAGRAM.md [4.3 KB]
│   │   ├── 📝 FOOTPRINTS.md [1.8 KB]
│   │   ├── 📝 INSTRUCTIONS.md [5.2 KB]
│   │   ├── 📝 RELEASE.md [1.5 KB]
│   │   ├── 📝 SECURITY.md [1.7 KB]
│   │   ├── 📝 SPEC.md [9.1 KB]
│   │   ├── 📝 TESTING.md [1.9 KB]
│   │   └── 📝 TODO.md [4.1 KB]
│   └── 📁 ja
│       ├── 📝 ARCHITECTURE.md [7.2 KB]
│       ├── 📝 CHANGELOG.md [9.9 KB]
│       ├── 📝 CONTRIBUTING.md [2.4 KB]
│       ├── 📝 DIAGRAM.md [4.6 KB]
│       ├── 📝 FOOTPRINTS.md [2.0 KB]
│       ├── 📝 INSTRUCTIONS.md [6.1 KB]
│       ├── 📝 RELEASE.md [1.9 KB]
│       ├── 📝 SECURITY.md [2.1 KB]
│       ├── 📝 SPEC.md [10.5 KB]
│       ├── 📝 TESTING.md [2.4 KB]
│       └── 📝 TODO.md [5.1 KB]
├── 📂 src
│   ├── 🦀 config.rs [14.2 KB]
│   ├── 🦀 icon.rs [9.0 KB]
│   ├── 🦀 main.rs [4.5 KB]
│   ├── 🦀 serializer.rs [10.4 KB]
│   ├── 🦀 tree.rs [10.4 KB]
│   └── 🦀 util.rs [16.8 KB]
├── 📝 CHANGELOG.md [394 B]
├── 📝 CLAUDE.md [641 B]
├── 🔒 Cargo.lock [150 B]
├── 🔧 Cargo.toml [452 B]
├── 📝 GEMINI.md [1.2 KB]
├── 📄 LICENSE [34.2 KB]
├── 📝 README.md [10.4 KB]
├── 📝 README_JA.md [12.2 KB]
└── 📝 RULES.md [5.7 KB]

4 directories, 37 files
```

#### ディレクトリ集計モード (`MyTree -D`)
```text
📁 . [37 files, 230.0 KB]
├── 📚 docs [22 files, 99.4 KB]
│   ├── 📁 en [11 files, 45.3 KB]
│   │   ├── 📝 ARCHITECTURE.md [6.1 KB]
│   │   ├── 📝 CHANGELOG.md [7.7 KB]
│   │   ├── 📝 CONTRIBUTING.md [1.9 KB]
│   │   ├── 📝 DIAGRAM.md [4.3 KB]
│   │   ├── 📝 FOOTPRINTS.md [1.8 KB]
│   │   ├── 📝 INSTRUCTIONS.md [5.2 KB]
│   │   ├── 📝 RELEASE.md [1.5 KB]
│   │   ├── 📝 SECURITY.md [1.7 KB]
│   │   ├── 📝 SPEC.md [9.1 KB]
│   │   ├── 📝 TESTING.md [1.9 KB]
│   │   └── 📝 TODO.md [4.1 KB]
│   └── 📁 ja [11 files, 54.2 KB]
│       ├── 📝 ARCHITECTURE.md [7.2 KB]
│       ├── 📝 CHANGELOG.md [9.9 KB]
│       ├── 📝 CONTRIBUTING.md [2.4 KB]
│       ├── 📝 DIAGRAM.md [4.6 KB]
│       ├── 📝 FOOTPRINTS.md [2.0 KB]
│       ├── 📝 INSTRUCTIONS.md [6.1 KB]
│       ├── 📝 RELEASE.md [1.9 KB]
│       ├── 📝 SECURITY.md [2.1 KB]
│       ├── 📝 SPEC.md [10.5 KB]
│       ├── 📝 TESTING.md [2.4 KB]
│       └── 📝 TODO.md [5.1 KB]
├── 📂 src [6 files, 65.3 KB]
│   ├── 🦀 config.rs [14.2 KB]
│   ├── 🦀 icon.rs [9.0 KB]
│   ├── 🦀 main.rs [4.5 KB]
│   ├── 🦀 serializer.rs [10.4 KB]
│   ├── 🦀 tree.rs [10.4 KB]
│   └── 🦀 util.rs [16.8 KB]
├── 📝 CHANGELOG.md [394 B]
├── 📝 CLAUDE.md [641 B]
├── 🔒 Cargo.lock [150 B]
├── 🔧 Cargo.toml [452 B]
├── 📝 GEMINI.md [1.2 KB]
├── 📄 LICENSE [34.2 KB]
├── 📝 README.md [10.4 KB]
├── 📝 README_JA.md [12.2 KB]
└── 📝 RULES.md [5.7 KB]

4 directories, 37 files
```

### コマンドラインオプション

| オプション | ロングフラグ          |      デフォルト     | 説明                                                                           |
| :--------- | :-------------------- | :-----------------: | :----------------------------------------------------------------------------- |
| `-h`       | `--help`              |          -          | ヘルプ表示 (カテゴリ別 & デフォルト動作含む)                                   |
| `-v`       | `--version`           |          -          | バージョン情報を表示                                                           |
|            | `--help-icons`        |          -          | アイコン絵文字とファイル拡張子の対応一覧を表示                                 |
| `-c`       | `--classic`           |        無効         | クラシックモード (カラー無効・アイコン無効・サイズ無効・`+--`/`\--`記号)       |
| `-i`       | `--icons`             |      **有効**       | ディレクトリやファイルに絵文字アイコンを付与                                   |
| `-I`       | `--no-icons`          |          -          | アイコン絵文字を表示しない (非表示)                                            |
| `-s`       | `--sizes`             |      **有効**       | ファイルサイズ (`B`, `KB`, `MB`, `GB`) を表示                                  |
| `-S`       | `--no-sizes`          |          -          | ファイルサイズを表示しない (非表示)                                            |
| `-D`       | `--dir-sizes`, `--du` |        無効         | フォルダ配下の総ファイル数および総容量を集計・表示                             |
| `-l`       | `--lines`             |        無効         | ファイル行数を表示 (テキストファイル限定、10MB超はスキップ)                    |
|            | `--json`              |        無効         | 構造化データとして JSON 形式でツリーを出力                                     |
|            | `--xml`               |        無効         | 構造化データとして XML 形式でツリーを出力                                      |
| `-a`       | `--all`               |        無効         | ドット (`.`) から始まる隠しファイル・フォルダも表示                            |
| `-g`       | `--gitignore`         |        無効         | `.gitignore` ファイルから除外パターンを自動読み込みして適用                    |
|            | `--sort <MODE>`       |       `name`        | 表示ソート順を指定 (`name`: 名前昇順, `size`: サイズ降順, `time`: 更新日時降順)|
| `-r`       | `--reverse`           |        無効         | ソート順を反転 (逆順) して表示                                                 |
| `-d <D>`   | `--depth <DEPTH>`     |       無制限        | 探索する深さの上限を指定 (例: `-d 2`)                                          |
| `-e <PAT>` | `--exclude <PAT>`     | `"target,...,.git"` | 除外する名前やワイルドカードパターンをカンマ区切りで指定                       |

---

## 🛠️ Windows 環境での PATH 登録

いつでもどこからでも `MyTree` コマンドを実行できるように PATH を登録します。

1. `target/release/MyTree.exe` を任意の固定フォルダ（例: `C:\tools\bin` や `%USERPROFILE%\bin`）に配置します。
2. Windowsキーを押し、「環境変数を編集」を検索して開きます。
3. `Path` 環境変数に配置場所のフォルダパスを追加します。
4. コマンドプロンプトやPowerShellを再起動すると、`MyTree` がご利用いただけます。

---

## 📚 関連ドキュメント

詳細なドキュメントは `docs/ja/` ディレクトリに用意されています：
- [機能仕様書](docs/ja/SPEC.md)
- [システムアーキテクチャ設計書](docs/ja/ARCHITECTURE.md)
- [システム構成図・実行フロー](docs/ja/DIAGRAM.md)
- [リソース消費・フットプリント記録](docs/ja/FOOTPRINTS.md)
- [AIエージェント開発指示書](docs/ja/INSTRUCTIONS.md)
- [テスト方針・実行ガイド](docs/ja/TESTING.md)
- [リリース手順書](docs/ja/RELEASE.md)
- [貢献ガイドライン](docs/ja/CONTRIBUTING.md)
- [セキュリティポリシー](docs/ja/SECURITY.md)
- [TODO・ロードマップ](docs/ja/TODO.md)
- [変更履歴](docs/ja/CHANGELOG.md)

---

## 📄 ライセンス

本プロジェクトは GNU General Public License v3.0 or later (GPL-3.0-or-later) のもとで公開されています - Copyright (c) 2026 tkshnkgwr。詳細は [LICENSE](LICENSE) をご覧ください。
