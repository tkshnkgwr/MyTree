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

## 3. Testing & Verification Policies

- When adding features, include corresponding unit tests in the respective `tests` module.
- Always run the following commands before completing your changes to confirm there are no warnings or errors:
  - `cargo test`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo fmt --check`
  - `cargo doc --no-deps --document-private-items`

---

## 4. Refactoring & Module Splitting Policies

- **1,000-Line Limit Rule**:
  - Once any single source file (e.g. `src/main.rs`) grows to around **1,000 lines**, propose and implement refactoring by separating concerns into appropriate modules (e.g., `config.rs`, `tree.rs`, `serializer.rs`, `util.rs`, etc.) to maintain readability and maintainability.
