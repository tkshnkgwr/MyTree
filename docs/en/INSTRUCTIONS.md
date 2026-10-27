# AI Development Instructions (INSTRUCTIONS.md) - MyTree

**English** | [日本語版](../ja/INSTRUCTIONS.md)

This document provides guidelines and conventions for AI agents (like the Grand Sage) to follow when modifying, maintaining, or adding features to `MyTree`.

---

## 1. Development Principles

1. **Strict Adherence to Zero External Dependencies**:
   - Never add third-party crates under `[dependencies]` in `Cargo.toml`. Always utilize Rust standard library (`std`) APIs.
2. **Cross-Platform Compatibility**:
   - When adding Windows-specific processes (`Win32 API` bindings or `#[cfg(windows)]`), configure conditional compilation carefully to avoid breaking builds on Linux/macOS.
3. **Maintain & Sync Rustdoc Comments**:
   - When adding new functions, structs, modules, or fields, always document them using `///` or `//!` doc comments.
4. **Explanations Tailored for Developers Experienced in Other Languages**:
   - The user has software development experience in other languages, but is unfamiliar with specific Rust/React/TypeScript concepts and jargon (e.g., ownership, traits, hooks). When explaining code changes, avoid overusing language-specific terms; prefer general programming concepts or provide clear, intuitive explanations.

---

## 2. Code Style Guidelines

- **Formatting**: Adhere to standard `cargo fmt` formatting.
- **Naming Conventions**:
  - Structs / Enums: `PascalCase` (e.g. `TreeConfig`)
  - Functions / Variables / Modules: `snake_case` (e.g. `parse_arguments`)
  - Constants: `SCREAMING_SNAKE_CASE` (e.g. `STD_OUTPUT_HANDLE`)
- **Error Handling**:
  - Avoid panic points (`panic!`, `unwrap()`). Prefer returning `Result<T, String>` or managing errors explicitly.

---

## 3. Documentation Synchronization Rules

When modifying code, refactoring, adding features, fixing bugs, or updating dependencies, keep all related documentation **strictly synchronized between English (`docs/en/`) and Japanese (`docs/ja/`)**.
*(Note: If only modifying Markdown (`.md`) files, the full automated verification process may be skipped.)*

| Document                         | Role                      | Update Trigger & Rules                                                                                      |
| :------------------------------- | :------------------------ | :---------------------------------------------------------------------------------------------------------- |
| **CHANGELOG.md**                 | Release history & changes | Append on feature completion. Group by date (`## [YYYY-MM-DD]`) and category (`Added`, `Fixed`, etc.).     |
| **SPEC.md**                      | Functional specification  | Update on CLI flags, arguments, display rules, or OS support changes.                                       |
| **DIAGRAM.md**                   | System flow & diagrams    | Update Mermaid diagrams when flow or module structures change.                                              |
| **README.md** / **README_JA.md** | Overview & quick start    | Update root readmes on options, build steps, or prerequisites changes.                                      |
| **FOOTPRINTS.md**                | Performance metrics       | Append updated binary sizes, RAM usage, and build benchmarks.                                               |
| **ARCHITECTURE.md**              | Architecture & internals  | Update on module refactoring, zero-dependency changes, or internal algorithms.                              |
| **INSTRUCTIONS.md**              | AI guidelines & rules     | Update coding conventions and AI development instructions.                                                  |
| **TODO.md**                      | Task tracking & roadmap   | Update task statuses (Done, In Progress, Backlog).                                                          |
| **TESTING.md**                   | Testing strategy & guide  | Update testing instructions, test suites, or verification checklists.                                       |
| **RELEASE.md**                   | Release procedures        | Update versioning steps, build processes, or tagging instructions.                                          |
| **CONTRIBUTING.md**              | Contributor guide         | Update contribution workflow, PR checklists, or commit conventions.                                         |
| **SECURITY.md**                  | Security policy           | Update security architecture, supported versions, or reporting guidelines.                                  |
| **Rustdoc** (Source comments)    | In-code specification     | Keep doc comments (`///` and `//!`) synchronized on API or struct changes.                                  |

---

## 4. Quality Control & Pre-Verification Rules

- **1,000-Line Limit Rule**:
  - Once any single source file (e.g. `src/main.rs`) grows to around **1,000 lines**, propose and implement refactoring by separating concerns into appropriate modules (e.g., `config.rs`, `tree.rs`, `serializer.rs`, `util.rs`, etc.).
- **Local Pre-Verification**:
  *(Note: If only modifying Markdown (`.md`) files, the following local pre-verification may be skipped for faster turnaround.)*
  1. `cargo test`
  2. `cargo clippy --all-targets -- -D warnings`
  3. `cargo fmt --check`
  4. `cargo doc --no-deps --document-private-items`

