# MyTree 🌲

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)]()
[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/version-0.1.2-green.svg)]()
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
  - フォルダ配下の総容量・総ファイル数集計表示 (`-D` / `--dir-sizes`)
  - ワイルドカード (`*`, `?`) パターン除外 (`-e "*.log,*.tmp"`)
  - アイコン非表示 (`-I` / `--no-icons`)
  - ファイルサイズ非表示 (`-S` / `--no-sizes`)
  - 探索階層（深さ）の制限 (`-d`)
  - 隠しファイル・ディレクトリの表示切り替え (`-a`)
  - バージョン表示 (`-v` / `--version`)
  - アイコン絵文字対応一覧ヘルプ (`--help-icons`)

---

## 🚀 インストール & ビルド手順

### 前提条件
- [Rust](https://rustup.rs/) (2021 edition 以降)

### リリリースビルド
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
MyTree C:\Users\Public
```

### 📸 実行結果（出力例）

#### デフォルト表示 (`MyTree`)
```text
📁 .
├── 📁 docs
│   ├── 📝 ARCHITECTURE.md [5.3 KB]
│   ├── 📝 DIAGRAM.md [3.6 KB]
│   ├── 📝 FOOTPRINTS.md [1.8 KB]
│   ├── 📝 INSTRUCTIONS.md [2.4 KB]
│   ├── 📝 SPEC.md [7.4 KB]
│   └── 📝 TODO.md [3.2 KB]
├── 📂 src
│   ├── 🦀 config.rs [10.0 KB]
│   ├── 🦀 icon.rs [6.4 KB]
│   ├── 🦀 main.rs [4.2 KB]
│   ├── 🦀 serializer.rs [9.5 KB]
│   ├── 🦀 tree.rs [7.7 KB]
│   └── 🦀 util.rs [2.6 KB]
├── 📝 CHANGELOG.md [4.5 KB]
├── 📄 Cargo.lock [150 B]
├── 🔧 Cargo.toml [426 B]
├── 📄 LICENSE [1.1 KB]
├── 📝 README.md [3.8 KB]
└── 📝 README_JA.md [5.4 KB]

2 directories, 18 files
```

#### クラシックモード (`MyTree -c`)
```text
.
+-- docs
|   +-- ARCHITECTURE.md
|   +-- DIAGRAM.md
|   +-- FOOTPRINTS.md
|   +-- INSTRUCTIONS.md
|   +-- SPEC.md
|   \-- TODO.md
+-- src
|   +-- config.rs
|   +-- icon.rs
|   +-- main.rs
|   +-- serializer.rs
|   +-- tree.rs
|   \-- util.rs
+-- CHANGELOG.md
+-- Cargo.lock
+-- Cargo.toml
+-- LICENSE
+-- README.md
\-- README_JA.md

2 directories, 18 files
```

#### フォルダ集計表示 (`MyTree -D`)
```text
📁 . [18 files, 79.4 KB]
├── 📁 docs [6 files, 23.8 KB]
│   ├── 📝 ARCHITECTURE.md [5.3 KB]
│   ├── 📝 DIAGRAM.md [3.6 KB]
│   ├── 📝 FOOTPRINTS.md [1.8 KB]
│   ├── 📝 INSTRUCTIONS.md [2.4 KB]
│   ├── 📝 SPEC.md [7.4 KB]
│   └── 📝 TODO.md [3.2 KB]
├── 📂 src [6 files, 40.3 KB]
│   ├── 🦀 config.rs [10.0 KB]
│   ├── 🦀 icon.rs [6.4 KB]
│   ├── 🦀 main.rs [4.2 KB]
│   ├── 🦀 serializer.rs [9.5 KB]
│   ├── 🦀 tree.rs [7.7 KB]
│   └── 🦀 util.rs [2.6 KB]
├── 📝 CHANGELOG.md [4.5 KB]
├── 📄 Cargo.lock [150 B]
├── 🔧 Cargo.toml [426 B]
├── 📄 LICENSE [1.1 KB]
├── 📝 README.md [3.8 KB]
└── 📝 README_JA.md [5.4 KB]

2 directories, 18 files
```

### コマンドラインオプション

| オプション | ロングフラグ | デフォルト | 説明 |
| :--- | :--- | :---: | :--- |
| `-h` | `--help` | - | ヘルプメニューを表示 (カテゴリ別 & デフォルト動作含む) |
| `-v` | `--version` | - | バージョン情報を表示 |
| | `--help-icons` | - | アイコン絵文字とファイル拡張子の対応一覧を表示 |
| `-c` | `--classic` | 無効 | クラシックモード (カラー無効・アイコン無効・サイズ無効・`+--`/`\--`記号) |
| `-i` | `--icons` | **有効** | ディレクトリやファイルに絵文字アイコンを付与 |
| `-I` | `--no-icons` | - | アイコン絵文字を表示しない (非表示) |
| `-s` | `--sizes` | **有効** | ファイルサイズ (`B`, `KB`, `MB`) を表示 |
| `-S` | `--no-sizes` | - | ファイルサイズを表示しない (非表示) |
| `-D` | `--dir-sizes`, `--du` | 無効 | フォルダ配下の総ファイル数および総容量を集計・表示 |
| | `--json` | 無効 | 構造化データとして JSON 形式でツリーを出力 |
| | `--xml` | 無効 | 構造化データとして XML 形式でツリーを出力 |
| `-a` | `--all` | 無効 | ドット (`.`) から始まる隠しファイル・フォルダも表示 |
| `-g` | `--gitignore` | 無効 | `.gitignore` ファイルから除外パターンを自動読み込みして適用 |
| `-d <DEPTH>` | `--depth <DEPTH>` | 無制限 | 探索する深さの上限を指定 (例: `-d 2`) |
| `-e <PAT>` | `--exclude <PAT>` | `"target,node_modules,.git"` | 除外する名前やワイルドカードパターンをカンマ区切りで指定 |

---

## 🛠️ Windows 環境での PATH 登録

いつでもどこからでも `MyTree` コマンドを実行できるように PATH を登録します。

1. `target/release/MyTree.exe` を任意の固定フォルダ（例: `C:\tools\bin` や `C:\Program Files\MyTree`）に配置します。
2. Windowsキーを押し、「環境変数を編集」を検索して開きます。
3. `Path` 環境変数に配置場所のフォルダパスを追加します。
4. コマンドプロンプトやPowerShellを再起動すると、`MyTree` がご利用いただけます。

---

## 📄 ライセンス

本プロジェクトは [MIT License](LICENSE) のもとで公開されています。
