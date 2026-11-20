# MyTree 🌲

[![License: GPL-3.0-or-later](https://img.shields.io/badge/License-GPL--3.0--or--later-blue.svg)](LICENSE)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)
[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](https://www.rust-lang.org/)
![Version](https://img.shields.io/badge/version-0.1.7-green.svg)
[![CI](https://github.com/tkshnkgwr/MyTree/actions/workflows/ci.yml/badge.svg)](https://github.com/tkshnkgwr/MyTree/actions/workflows/ci.yml)

**English** | [日本語版](README_JA.md)

A lightweight, high-performance CLI utility written in Rust to display directory structures in a clean tree format for Windows Command Prompt, PowerShell, and Unix terminals.
Built exclusively using Rust's standard library (`std`) with **zero external dependencies** (`clap`, `crossterm`, etc.), ensuring lightning-fast compilation and minimal binary size.

---

## ✨ Features

- 🚀 **Zero External Dependencies**: Built 100% with Rust `std` library for fast compilation and lightweight footprint.
- 🎨 **Colored & Emoji Formatting**: Rich terminal output with customizable emoji icons and ANSI terminal colors (can be disabled).
- 📜 **Classic Mode (`-c`)**: Plain text ASCII output (`+-- `, `\-- `, `|   `) without ANSI colors, icons, or file sizes for classic DOS/PowerShell terminals.
- 💻 **Windows Console Optimization**: Enables ANSI virtual terminal processing on Windows 10/11 via native Win32 API calls.
- 🔍 **Flexible Options**:
  - Classic display mode (`-c` / `--classic`)
  - Auto-apply `.gitignore` rules (`-g` / `--gitignore`)
  - Total files & size calculation for directories (`-D` / `--dir-sizes` / `--du`)
  - Streaming file line count display (`-l` / `--lines`)
  - Multiple sorting keys & reverse order (`--sort <MODE>`, `-r` / `--reverse`)
  - Structured data output (`--json`, `--xml`)
  - Wildcard pattern matching (`-e "*.log,*.tmp"`)
  - Icon mapping help (`--help-icons`)
  - Version info (`-v` / `--version`)
  - Hide icons (`-I` / `--no-icons`)
  - Hide file sizes (`-S` / `--no-sizes`)
  - Max recursion depth control (`-d`)
  - Show hidden files (`-a` / `--all`)

---

## 🚀 Installation & Build

### Prerequisites
- [Rust Toolchain](https://rustup.rs/) (2021 edition or newer)

### Build from Source
```bash
# Clone or navigate to the project root
cd MyTree

# Build optimized release binary
cargo build --release
```

The compiled binary will be located at `target/release/MyTree.exe` (or `target/release/MyTree` on Linux/macOS).

---

## 💻 Usage & Options

```bash
# Display tree of current directory
MyTree

# Display in Classic mode (no color, no icons, no file sizes, ASCII connectors)
MyTree -c

# Display total files and total size for directories
MyTree -D

# Display file line counts (text files only)
MyTree -l

# Sort by file size in ascending order (reverse of default descending size sort)
MyTree --sort size -r

# Output tree as JSON structured data
MyTree --json

# Output tree as XML structured data
MyTree --xml

# Display version information
MyTree -v

# Display icon emoji mapping help
MyTree --help-icons

# Limit recursion depth to 2 levels
MyTree -d 2

# Auto-apply exclusion rules from .gitignore
MyTree -g

# Exclude specific patterns (wildcards supported)
MyTree -e "*.log,*.tmp,target"
```

### 📸 Sample Output

#### Default Output (`MyTree`)
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

#### Directory Summary Mode (`MyTree -D`)
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

### CLI Options

| Flag       | Long Flag             |       Default       | Description                                                                        |
| :--------- | :-------------------- | :-----------------: | :--------------------------------------------------------------------------------- |
| `-h`       | `--help`              |          -          | Show help menu (categorized layout)                                                |
| `-v`       | `--version`           |          -          | Display version information                                                        |
|            | `--help-icons`        |          -          | Display icon emoji mapping reference table                                         |
| `-c`       | `--classic`           |      Disabled       | Classic mode (no colors, no icons, no file sizes, ASCII `+--`/`\--` connectors)    |
| `-i`       | `--icons`             |     **Enabled**     | Enable emoji icons for files and folders                                           |
| `-I`       | `--no-icons`          |          -          | Disable emoji icons                                                                |
| `-s`       | `--sizes`             |     **Enabled**     | Display file size (`B`, `KB`, `MB`, `GB`)                                          |
| `-S`       | `--no-sizes`          |          -          | Disable file size display                                                          |
| `-D`       | `--dir-sizes`, `--du` |      Disabled       | Display total file count and size for directories                                  |
| `-l`       | `--lines`             |      Disabled       | Display file line counts (text files only, >10MB skipped)                          |
|            | `--json`              |      Disabled       | Output tree as JSON structured data                                                |
|            | `--xml`               |      Disabled       | Output tree as XML structured data                                                 |
| `-a`       | `--all`               |      Disabled       | Show hidden files and directories starting with `.`                                |
| `-g`       | `--gitignore`         |      Disabled       | Automatically load and apply exclusion rules from `.gitignore`                     |
|            | `--sort <MODE>`       |       `name`        | Select sort mode (`name`: A-Z, `size`: largest first, `time`: newest first)        |
| `-r`       | `--reverse`           |      Disabled       | Reverse the sorting order                                                          |
| `-d <D>`   | `--depth <DEPTH>`     |      Unlimited      | Set maximum recursion depth (e.g., `-d 2`)                                         |
| `-e <PAT>` | `--exclude <PAT>`     | `"target,...,.git"` | Exclude directory names or wildcard patterns                                       |

---

## 📚 Documentation

Detailed documentation is available in the `docs/en/` directory:
- [CLI Specification](docs/en/SPEC.md)
- [System Architecture](docs/en/ARCHITECTURE.md)
- [System Diagrams](docs/en/DIAGRAM.md)
- [Performance & Footprints](docs/en/FOOTPRINTS.md)
- [AI Development Instructions](docs/en/INSTRUCTIONS.md)
- [Testing Policy & Guide](docs/en/TESTING.md)
- [Release Procedures](docs/en/RELEASE.md)
- [Contribution Guidelines](docs/en/CONTRIBUTING.md)
- [Security Policy](docs/en/SECURITY.md)
- [TODO & Roadmap](docs/en/TODO.md)
- [Changelog](docs/en/CHANGELOG.md)

---

## 📄 License

This project is licensed under the GNU General Public License v3.0 or later (GPL-3.0-or-later) - Copyright (c) 2026 tkshnkgwr. See the [LICENSE](LICENSE) file for details.
