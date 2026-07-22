# Specification (SPEC.md) - MyTree

**English** | [日本語版](../ja/SPEC.md)

This document defines the functional specification, CLI arguments, output formatting, error handling, and target operating systems for `MyTree` (`MyTree.exe`).

---

## 1. Overview & Goals

- **Objective**: Displays directory structures in a fast, highly readable tree format inside the terminal.
- **Design Constraints**: Zero external dependencies (uses standard `std` library only).
- **Supported OS**: Windows 10/11 (with ANSI enabled via native Win32 API), Linux, macOS.

---

## 2. CLI Command Line Specification

### 2.1 Syntax
```text
MyTree [OPTIONS] [TARGET_PATH]
```

### 2.2 Options List

| Option | Long Flag             | Argument             | Default        | Description                                                             |
| :----- | :-------------------- | :------------------- | :------------: | :---------------------------------------------------------------------- |
| `-h`   | `--help`              | None                 | -              | Shows categorized help menu and exits with code `0`.                    |
| `-v`   | `--version`           | None                 | -              | Prints version info and exits with code `0`.                            |
|        | `--help-icons`        | None                 | -              | Prints icon mapping table and exits with code `0`.                      |
| `-c`   | `--classic`           | None                 | `false`        | Classic mode (no colors, icons, sizes; ASCII connectors).               |
| `-i`   | `--icons`             | None                 | `true` (std)   | Shows emoji icons for files and directories.                            |
| `-I`   | `--no-icons`          | None                 | `false`        | Hides emoji icons.                                                      |
| `-s`   | `--sizes`             | None                 | `true` (std)   | Displays file sizes next to file names.                                 |
| `-S`   | `--no-sizes`          | None                 | `false`        | Hides file sizes.                                                       |
| `-D`   | `--dir-sizes`, `--du` | None                 | `false`        | Displays total file count and size for directories.                     |
|        | `--json`              | None                 | `false`        | Outputs tree structure in JSON format.                                  |
|        | `--xml`               | None                 | `false`        | Outputs tree structure in XML format.                                   |
| `-a`   | `--all`               | None                 | `false`        | Includes hidden files starting with a dot (`.`).                        |
| `-g`   | `--gitignore`         | None                 | `false`        | Auto-applies ignore patterns from `.gitignore` files.                   |
| `-d`   | `--depth`             | `<DEPTH>` (usize)    | `0` (unlimited)| Restricts maximum tree traversal depth.                                 |
| `-e`   | `--exclude`           | `<PATTERNS>` (String)| `"target,..."` | Exclude directory names or wildcard patterns (`*`, `?`).                |

### 2.3 Default Configuration Structure (`TreeConfig`)

```rust
enum OutputFormat {
    Text,
    Json,
    Xml,
}

struct TreeConfig {
    target_path: PathBuf,        // Default: "."
    show_icons: bool,            // Default: true
    show_sizes: bool,            // Default: true
    show_dir_sizes: bool,        // Default: false (enabled via option)
    show_hidden: bool,           // Default: false
    max_depth: usize,            // Default: 0 (unlimited)
    exclude_patterns: Vec<String>, // Default: ["target", "node_modules", ".git"]
    classic_mode: bool,          // Default: false (enabled via option)
    output_format: OutputFormat, // Default: OutputFormat::Text
    use_gitignore: bool,         // Default: false (enabled via option)
}
```

---

## 3. Output Format Specification

### 3.1 Branch Connectors
- **Standard Mode (Default)**:
  - Middle entry: `├── `
  - Last entry: `└── `
  - Directory Indentation: `│   ` (when continuing) / `    ` (when trailing parent was last)
- **Classic Mode (`-c` active)**:
  - Middle entry: `+-- `
  - Last entry: `\-- `
  - Directory Indentation: `|   ` (when continuing) / `    ` (when trailing parent was last)

### 3.2 Directory Summary Format (`-D` active)
- Standard Mode: ` \x1b[90m[{total_files} files, {total_size}]\x1b[0m`
- Classic Mode: ` [{total_files} files, {total_size}]`

### 3.3 Icon & Emoji Mapping (`get_icon_emoji`)

#### Directories
- `src` => `📂`
- `tests` / `test` => `🧪`
- `target` / `build` / `dist` / `out` => `🔨`
- `node_modules` / `vendor` => `📦`
- `.git` / `.github` => `🐙`
- `docs` / `doc` => `📚`
- `.vscode` / `.idea` => `💡`
- `public` / `assets` / `static` => `🖼`
- Other directories => `📁`

#### Files & Extensions
- Programming & Web:
  - `.rs` => `🦀` (Rust)
  - `.py`, `.pyw` => `🐍` (Python)
  - `.js`, `.jsx`, `.ts`, `.tsx`, `.mjs`, `.cjs` => `🟨` (JavaScript / TypeScript)
  - `.vue` => `💚` (Vue)
  - `.svelte` => `🟧` (Svelte)
  - `.astro` => `🚀` (Astro)
  - `.go` => `🐹` (Go)
  - `.rb` => `💎` (Ruby)
  - `.java`, `.jar`, `.class` => `☕` (Java)
  - `.swift` => `🦅` (Swift)
  - `.kt`, `.kts` => `🟪` (Kotlin)
  - `.dart` => `🎯` (Dart)
  - `.lua` => `🌙` (Lua)
  - `.c`, `.cpp`, `.h`, `.hpp`, `.cs` => `⚙` (C / C++ / C#)
  - `.php` => `🐘` (PHP)
- Databases, Schema & IaC:
  - `.sql`, `.db`, `.sqlite`, `.sqlite3` => `💾` (Database)
  - `.graphql`, `.gql`, `.prisma` => `🔷` (GraphQL / Prisma)
  - `.tf`, `.tfvars` => `🏗` (Terraform)
  - `.proto`, `.wasm` => `⚡` (Protobuf / WebAssembly)
- Office Documents & Sheets:
  - `.xlsx`, `.xls`, `.csv`, `.tsv` => `📊` (Excel / Sheets)
  - `.docx`, `.doc` => `📘` (Word)
  - `.pptx`, `.ppt` => `📙` (PowerPoint)
  - `.pdf` => `📕` (PDF)
- Text, Configuration & Locks:
  - `.txt`, `.text`, `.log` => `📜` (Text / Log)
  - `.md`, `.markdown` => `📝` (Markdown)
  - `.json`, `.jsonc` => `📄` (JSON)
  - `.toml`, `.yaml`, `.yml`, `.ini`, `.conf`, `.config` => `🔧` (Config files)
  - `.lock`, `Cargo.lock`, `package-lock.json`, `yarn.lock`, `pnpm-lock.yaml` => `🔒` (Lock files)
  - `Makefile`, `justfile`, `CMakeLists.txt` => `🛠` (Build tools)
  - `.env*` => `🔑` (Environment Variables)
  - `.gitignore`, `.dockerignore` => `🙈` (Ignore list)
  - `Dockerfile`, `docker-compose*` => `🐳` (Docker)
- Executables & Scripts:
  - `.exe`, `.bat`, `.cmd`, `.sh`, `.ps1`, `.zsh`, `.bash`, `.fish` => `🚀`
- Archives (Compressed):
  - `.zip`, `.tar`, `.gz`, `.7z`, `.rar`, `.bz2`, `.xz` => `📦`
- Fonts & Media:
  - Fonts (`.ttf`, `.otf`, `.woff`, `.woff2`, `.eot`) => `🔤`
  - Images (`.png`, `.jpg`, `.jpeg`, `.gif`, `.svg`, `.webp`, `.ico`, `.bmp`, `.avif`, `.heic`, `.psd`, `.ai`) => `🖼`
  - Audio (`.mp3`, `.wav`, `.flac`, `.aac`, `.ogg`, `.m4a`) => `🎵`
  - Video (`.mp4`, `.avi`, `.mkv`, `.mov`, `.webm`, `.wmv`) => `🎬`
- Other Undefined Files => `📄`

### 3.4 Color Code Specification (ANSI Escape Codes)
- Root line / Directory Name: `\x1b[1;34m` (Bold Blue) *Disabled in classic mode
- File Size / Directory Summaries: `\x1b[90m` (Dark Gray) *Disabled in classic mode
- Final Statistics (`X directories, Y files`): `\x1b[36m` (Cyan) *Disabled in classic mode
- Reset: `\x1b[0m`

---

## 4. Error Handling & Exit Codes

| Scenario                         | Error Output                                     | Exit Code |
| :------------------------------- | :----------------------------------------------- | :-------: |
| Success                          | None                                             |    `0`    |
| `--help` invoked                 | Displays group-organized help menu               |    `0`    |
| `--version` invoked              | Displays version information                     |    `0`    |
| `--help-icons` invoked           | Displays file types and icon mappings            |    `0`    |
| Unknown options                  | `[ERROR]: Unknown option: <ARG>`                 |    `1`    |
| Target path does not exist       | `[ERROR]: Specified path does not exist: <PATH>` |    `1`    |
| Missing arguments for `-d` / `-e`| `[ERROR]: Depth value not specified...`          |    `1`    |
| Access Denied (during traversal) | Skips entry without printing error message       | Continues |
