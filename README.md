# MyTree 🌲

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)]()
[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg)](https://www.rust-lang.org/)

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
  - Wildcard pattern matching (`-e "*.log,*.tmp"`)
  - Icon mapping help (`--help-icons`)
  - Directory total files & size calculation (`-D` / `--dir-sizes` / `--du`)
  - Version info (`-v` / `--version`)
  - Hide icons (`-I` / `--no-icons`)
  - Hide file sizes (`-S` / `--no-sizes`)
  - Max recursion depth control (`-d`)

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

# Exclude specific patterns (wildcards supported)
MyTree -e "*.log,*.tmp,target"
```

### CLI Options

| Flag | Long Flag | Default | Description |
| :--- | :--- | :---: | :--- |
| `-h` | `--help` | - | Show help menu (categorized layout) |
| `-v` | `--version` | - | Display version information |
| | `--help-icons` | - | Display icon emoji mapping reference table |
| `-c` | `--classic` | Disabled | Classic mode (no colors, no icons, no file sizes, ASCII `+--`/`\--` connectors) |
| `-i` | `--icons` | **Enabled** | Enable emoji icons for files and folders |
| `-I` | `--no-icons` | - | Disable emoji icons |
| `-s` | `--sizes` | **Enabled** | Display file size (`B`, `KB`, `MB`) |
| `-S` | `--no-sizes` | - | Disable file size display |
| `-D` | `--dir-sizes`, `--du` | Disabled | Display total file count and size for directories |
| | `--json` | Disabled | Output tree as JSON structured data |
| | `--xml` | Disabled | Output tree as XML structured data |
| `-a` | `--all` | Disabled | Show hidden files and directories starting with `.` |
| `-d <DEPTH>` | `--depth <DEPTH>` | Unlimited | Set maximum recursion depth (e.g., `-d 2`) |
| `-e <PAT>` | `--exclude <PAT>` | `"target,node_modules,.git"` | Exclude directory names or wildcard patterns |

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
