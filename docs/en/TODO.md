# TODO & Roadmap (TODO.md) - MyTree

**English** | [日本語版](../ja/TODO.md)

This document tracks completed features, current tasks, and future ideas for `MyTree`.

---

## 1. Completed Features (Done)

- [x] **Initial Implementation Using Only `std` Library** (zero-dependency lightweight layout)
- [x] **Core CLI Options Implementation**:
  - `-h`, `--help`: Prints group-organized help menu (with default configurations).
  - `-v`, `--version`: Prints version info.
  - `--help-icons`: Prints mapping table of file types to emoji icons.
  - `-c`, `--classic`: Classic mode (ASCII output: `+-- `, `\-- `, `|   `; no color, icons, or sizes).
  - `-i`, `--icons`: Enables directory/file type emoji icons.
  - `-I`, `--no-icons`: Hides emoji icons.
  - `-s`, `--sizes`: Enables file size formatting (`B`, `KB`, `MB`).
  - `-S`, `--no-sizes`: Hides file sizes.
  - `-D`, `--dir-sizes`, `--du`: Calculates and prints total file counts and sizes for folders.
  - `-a`, `--all`: Shows hidden files/directories starting with a dot (`.`).
  - `-g`, `--gitignore`: Reads and respects `.gitignore` rules dynamically.
  - `-d`, `--depth`: Restricts search depth.
  - `-e`, `--exclude`: Excludes directory names or glob wildcard patterns.
- [x] **GitHub Actions CI/CD Pipeline**:
  - `.github/workflows/ci.yml`: Performs `cargo test`, `clippy`, `fmt`, `doc`, and `build` on Push/PR to `main`.
  - `.github/workflows/release.yml`: Triggers on `v*` tag pushes to bundle and publish Windows binaries (`MyTree-windows-x64.zip`) to GitHub Releases.
- [x] **Glob Wildcard Exclusion Patterns (`match_wildcard`)**:
  - Excludes files/folders natively matching pattern structures like `*.tmp`, `*.log`, `build*`, `test?.rs`.
- [x] **Structured Machine-Readable Output (`--json` / `--xml`)**:
  - Outputs formatted JSON/XML trees for other utilities/scripts.
  - Hand-crafted escaping functions (`escape_json_str`, `escape_xml_str`) using standard library only.
- [x] **Expanded Emoji Icon Map (`src/icon.rs`)**:
  - Added mappings for directory types (`docs`, `.vscode`, `public`, etc.), lock files, build tools, languages (`.swift`, `.kt`, `.vue`, `.svelte`, `.astro`, etc.), and databases/schemas.
- [x] **Refactored Module Separation (1,000-Line Rule Met)**:
  - Split single file `src/main.rs` into 5 distinct modules (`main.rs`, `config.rs`, `tree.rs`, `serializer.rs`, `util.rs`, `icon.rs`) to maintain codebase cleanliness.
- [x] **Expanded Unit Tests (21 Total Tests)**:
  - High test coverage spanning CLI options, version info, byte conversions, icon resolution, wildcard rules, gitignore parsing, JSON/XML serialization, and traversal sums.
- [x] **Automatic Windows ANSI Virtual Terminal Initialization** (direct `kernel32.dll` Win32 API FFI)
- [x] **Directory-First Traversal Sorting** (folders listed before files)
- [x] **Added File Line Count Option (`-l` / `--lines`)**:
  - Implemented memory-efficient streaming buffered sequential `\n` line counting.
  - Integrated curated text extension filtering, binary detection (null byte check), and >10MB limit skip (`[>10MB skipped]`).
  - Added unit tests for `count_file_lines` and `-l` / `--lines` (expanded to 18 total tests).
- [x] **Enhanced Tree Display Sorting (`--sort <MODE>`, `-r` / `--reverse`)**:
  - Implemented sort modes by name (`name`), file size (`size`), and modified time (`time`).
  - Added order reversal flag (`-r` / `--reverse`) and centralized logic in `sort_dir_entries` (expanded to 20 total tests).
- [x] **Complete Documentation System**:
  - `README.md` / `README_JA.md`
  - `SPEC.md`, `ARCHITECTURE.md`, `DIAGRAM.md`, `FOOTPRINTS.md`, `TODO.md`, `INSTRUCTIONS.md`, `CHANGELOG.md`
  - `LICENSE` (MIT)

---

## 2. Upcoming Tasks (Todo / In Progress)

- [ ] **ANSI Color Mode Flags (`--color <MODE>`)**:
  - Support `auto`, `always`, and `never` to control color behavior when redirecting stdout.

---

## 3. Future Ideas (Backlog)

- [ ] **Custom Color & Emoji Configurations (`~/.mytreerc` / `icons.json`)**:
  - Read user-specific custom themes or mapping overrides dynamically.
- [ ] **Aggregated Traversal Metrics**:
  - Print overall total file count and size at the footer of standard tree outputs.
