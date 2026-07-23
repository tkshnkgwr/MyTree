# Architecture (ARCHITECTURE.md) - MyTree

**English** | [日本語版](../ja/ARCHITECTURE.md)

This document describes the system architecture, module design, internal algorithms, and technical rationale for `MyTree` (`MyTree.exe`).

---

## 1. Component Structure

The application is structured into modular components, cleanly separating different concerns.

```text
src/
├── main.rs                     # Entry point & Windows ANSI control
├── config.rs                   # CLI arguments parsing & TreeConfig/OutputFormat definitions
├── tree.rs                     # Directory traversal & text tree rendering core engine
├── serializer.rs               # JSON / XML structured data serializer engine
├── util.rs                     # General helper functions (wildcard match, byte format)
└── icon.rs                     # Icon emoji detection module (filename/extension mapping)
```

### 1.1 Module Roles
- **`src/main.rs`**:
  - `main()`: Entry point and top-level execution flow matching.
  - `enable_ansi_support()`: Enables ANSI escape colors on Windows consoles (Win32 API FFI).
- **`src/config.rs`**:
  - Defines configuration structures (`TreeConfig`, `OutputFormat`).
  - Contains parser and help screen utilities: `parse_arguments()`, `print_version()`, `print_help()`.
- **`src/tree.rs`**:
  - `print_tree()`: Core recursive engine to read directories and draw text trees.
  - `format_root_line()`, `calculate_dir_summary()`: Generates the root visual style and aggregates child stats.
- **`src/serializer.rs`**:
  - `TreeNode`: In-memory hierarchical tree representation.
  - `build_tree_node()`: Core function to recursively build tree data structures.
  - Hand-crafted serialization logic: `serialize_json()`, `serialize_xml()`.
  - Escaping utilities: `escape_json_str()`, `escape_xml_str()`.
- **`src/util.rs`**:
  - `match_wildcard()`: Evaluates wildcard string matching (`*`, `?`) with an allocation-free ASCII fast-path.
  - `format_byte_size()`: Formats raw bytes to human-readable units (`B`, `KB`, `MB`).
  - `sort_dir_entries()`: Directory-first multi-mode sorting with key-caching to minimize system calls.
  - `count_file_lines()`: Streaming line counter (with 10MB limit and binary detection).
  - `parse_gitignore_file()`: Parses `.gitignore` files for pattern matching.
- **`src/icon.rs`**:
  - `get_icon_emoji()`: Resolves appropriate emoji icons based on filenames, extensions, and directory status.

---

## 2. Core Design Philosophy

### 2.1 Zero External Dependencies
- We strictly avoid third-party crates such as `clap`, `structopt`, `colored`, or `crossterm`. The utility is implemented 100% using Rust's standard library (`std::env`, `std::fs`, `std::path`).
- **Benefits**:
  - Extremely fast compile time (sub-second incremental builds).
  - Eliminates dependency vulnerabilities and supply-chain attack vectors.
  - Keeps binary size exceptionally small (usually under 200KB).

### 2.2 Direct Win32 API FFI (`enable_ansi_support`)
Older versions of Windows Command Prompt (`cmd.exe`) and PowerShell disable ANSI escape colors by default.
To avoid importing heavy cross-platform terminal crates, `MyTree` directly binds to functions inside `kernel32.dll` using `extern "system"`. This enables native virtual terminal processing (`ENABLE_VIRTUAL_TERMINAL_PROCESSING = 0x0004`) directly, requiring zero external crates.

---

## 3. Directory Traversal & Sorting Algorithm

### 3.1 Directory-First & Key-Cached Sorting
To achieve high readability and filesystem traversal performance, sorting in `sort_dir_entries` follows these rules and optimizations:

1. **Key Caching**: Directory status and metadata (size, modified time) are fetched once per entry upfront, eliminating repetitive `fs::metadata` system calls during sort comparisons ($O(N)$).
2. **Directory-First**: Directories are always sorted above files.
3. **Multi-mode Sorting**: Entries of the same type are sorted by the specified mode (`Name`, `Size`, `Time`), with optional order reversal (`reverse_sort`).

```rust
// Pre-fetch keys to minimize system calls followed by index-based sorting
let keys: Vec<EntryKey> = entries.iter().map(|entry| ... ).collect();
let mut indices: Vec<usize> = (0..entries.len()).collect();
indices.sort_by(|&i, &j| { ... });
// Reorder slice in-place safely
for i in 0..entries.len() { ... }
```

### 3.2 Branch Connectors & Indentation Control
During recursive calls, a stack list of indentation prefixes (`prefixes: &mut Vec<&str>`) is passed downward. This prevents unnecessary string allocations and heap allocations, while keeping accurate branch boundaries (`│   ` or `    `) drawn efficiently.

---

## 4. Machine-Readable Data Serialization Design (`--json` / `--xml`)

Instead of pulling in `serde` or `quick-xml`, we support structured JSON/XML output via standard library structures and string escaping.

### 4.1 In-Memory Tree Model (`TreeNode`)
`build_tree_node()` recursively traverses the directories and builds an in-memory node representation after sorting and applying filters.

```rust
struct TreeNode {
    name: String,
    path: String,
    is_dir: bool,
    size: u64,
    children: Vec<TreeNode>,
}
```

### 4.2 Character Escaping Functions (`escape_json_str` / `escape_xml_str`)
- **JSON**: Safely escapes double quotes (`\"`), backslashes (`\\`), and control characters (`\n`, `\r`, `\t`, `\uXXXX`).
- **XML**: Safely escapes XML entities (`&amp;`, `&lt;`, `&gt;`, `&quot;`, `&apos;`).

---

## 5. CI/CD Pipeline Configurations (GitHub Actions)

We utilize GitHub Actions workflows to automate code quality checks and release builds.

### 5.1 CI Workflow (`.github/workflows/ci.yml`)
Runs automatically on Push / Pull Requests targeting the `main` branch. (Automatically skipped when only markdown/document files are updated).
- **Checks performed**: `cargo test`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`, `cargo doc`, and `cargo build --release`.

### 5.2 Release Workflow (`.github/workflows/release.yml`)
Triggered only on `v*` (e.g. `v0.1.1`, `v1.0.0`) tag pushes to automate distribution.
- Builds the release Windows binary (`MyTree.exe`), compresses it to `MyTree-windows-x64.zip`, and attaches it as an asset on GitHub Releases.
