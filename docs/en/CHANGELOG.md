# Changelog

**English** | [日本語版](../ja/CHANGELOG.md)

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased] / [0.1.1] - 2026-07-21

### Optimized
- Refactored `src/main.rs` into multiple modules:
  - Split the 1,000+ line `main.rs` into 5 files based on concerns (`config.rs`, `tree.rs`, `serializer.rs`, `util.rs`, `main.rs`).
  - Distributed unit tests appropriately across these modules.

### Added
- Set up GitHub Actions CI / Release workflow (`.github/workflows/`):
  - `ci.yml`: Runs `cargo test`, `clippy`, `fmt`, `doc`, and `build` on Push/PR to `main` branch.
  - `release.yml`: Triggered on `v*` (e.g., `v0.1.1`) tag push to build and upload Windows release binary (`MyTree-windows-x64.zip`) to GitHub Releases.
- Auto-read `.gitignore` pattern exclusion option (`-g` / `--gitignore`):
  - Implemented `.gitignore` parser `parse_gitignore_file` with zero external dependencies (standard `std::fs` only).
  - Integrated dynamic patterns exclusion during tree traversal and structured serialization (`--json` / `--xml`).
  - Added unit tests for `.gitignore` parsing logic.
- Expanded file name and extension emoji icon mapping in `src/icon.rs`:
  - Added folder icons (`docs`, `.vscode`, `.idea`, `public`, `assets`, `static`).
  - Added lock files (`.lock`, `Cargo.lock`, `package-lock.json`, etc. => 🔒).
  - Added build tools (`Makefile`, `justfile`, `CMakeLists.txt` => 🛠).
  - Added modern programming languages & frameworks (`.swift`, `.kt`, `.dart`, `.lua`, `.vue`, `.svelte`, `.astro`, etc.).
  - Added schema & IaC (`.graphql`, `.prisma`, `.tf`, `.proto`, `.wasm`).
  - Added fonts (`.ttf`, `.woff`, etc. => 🔤) and extra image formats (`.avif`, `.psd`, etc.).
  - Synchronized help page (`--help-icons`) and unit tests.
- Support for machine-readable structured output (`--json` / `--xml`):
  - `--json`: Formats the tree as a JSON object, ready for integration with other tools.
  - `--xml`: Formats the tree as an XML structure including summaries.
  - Custom implemented serializers and character escaping (`escape_json_str`, `escape_xml_str`) using only `std`.
  - Added in-memory node building model `TreeNode` and traversal function `build_tree_node`.
  - Added unit tests for argument parsing, JSON/XML escaping, and serialization (total tests increased to 16).

## [0.1.0] - 2026-07-21

### Added
- Initial release of `MyTree`.
- A zero-dependency, lightweight directory tree CLI utility written in Rust, supporting Windows, Linux, and macOS.
- Classic mode (`-c` / `--classic`):
  - Plain text ASCII output (`+-- `, `\-- `, `|   `) without ANSI colors, emoji icons, or file sizes.
- Wildcard (`*`, `?`) pattern exclusion logic (`match_wildcard`):
  - Supports patterns like `-e "*.tmp,*.log,build*"` natively.
- Comprehensive unit test suite covering all domains (total 12 tests):
  - Argument parsing (success/failure scenarios, unknown options, duplicates).
  - Byte size formatting (`format_byte_size`) boundary values (0B, 1023B, 1024B, 1MB, etc.).
  - Case sensitivity for icon mapping.
  - Wildcard match test cases.
  - Directory summaries calculation and classic mode parsing.
- Core CLI flags:
  - `-h`, `--help`: Prints group-organized help menu and default behaviors.
  - `-v`, `--version`: Prints version info.
  - `--help-icons`: Displays file names, extensions, folders, and their corresponding emoji icon mapping.
  - `-c`, `--classic`: Enables classic output format.
  - `-i`, `--icons`: Enables emoji icons (default).
  - `-I`, `--no-icons`: Hides emoji icons.
  - `-s`, `--sizes`: Enables file sizes (default).
  - `-S`, `--no-sizes`: Hides file sizes.
  - `-D`, `--dir-sizes`, `--du`: Displays directory total files and size summaries.
  - `-a`, `--all`: Shows hidden files and directories (dot files).
  - `-d`, `--depth <DEPTH>`: Limits directory search depth.
  - `-e`, `--exclude <PATTERNS>`: Comma-separated exclude patterns or wildcards.
- Added emoji icon mappings (text, SQL, Office, Python, Web, media, archive, etc.).
- Refactored icon mapping into a separate module (`src/icon.rs`).
- Native Win32 API FFI calls (`enable_ansi_support`) to enable ANSI virtual terminal processing natively on Windows 10/11 consoles.
- Directory-first sorting algorithm to improve tree readability.
- Initial document structure (`SPEC.md`, `ARCHITECTURE.md`, `DIAGRAM.md`, `FOOTPRINTS.md`, `TODO.md`, `INSTRUCTIONS.md`, `README.md`, `README_JA.md`).
- MIT License.
