# Contribution Guidelines (CONTRIBUTING.md) - MyTree

**English** | [日本語版](../ja/CONTRIBUTING.md)

Thank you for your interest in contributing to `MyTree`!
This document provides guidelines for reporting issues, proposing features, and submitting pull requests.

---

## 1. Core Principles

Please adhere to the following principles when contributing to `MyTree`:

1. **Zero External Dependencies**:
   - Do not add third-party crates to `Cargo.toml`. Always utilize Rust standard library (`std`) features.
2. **Cross-Platform Compatibility**:
   - Wrap platform-specific code (e.g., Win32 API calls) with conditional compilation flags (`#[cfg(windows)]`) to prevent build failures on Linux and macOS.
3. **Bilingual Documentation**:
   - Ensure all documentation changes are made synchronously in both `docs/en/` and `docs/ja/`.

---

## 2. Setting Up Development Environment

1. **Clone Repository**:
   ```bash
   git clone https://github.com/tkshnkgwr/MyTree.git
   cd MyTree
   ```
2. **Verify Execution**:
   ```bash
   cargo run
   ```

---

## 3. Commits & Pull Requests

### Commit Message Format
Use the Conventional Commits format for commit messages. Prefixes are written in English (`feat:`, `fix:`, `chore:`, etc.), while the following commit description body should be written in **Japanese** (e.g., `feat: 新機能の追加`, `chore: ビルド設定の更新`):

- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation changes
- `refactor:` Code refactoring
- `perf:` Performance optimizations
- `test:` Adding or updating tests
- `chore:` Build scripts or maintenance tasks

### Pre-PR Checklist
Before creating a pull request, ensure all local checks pass:

- [ ] `cargo test` (Unit tests pass)
- [ ] `cargo clippy --all-targets -- -D warnings` (Zero Clippy warnings)
- [ ] `cargo fmt --check` (Code formatting compliant)
- [ ] `cargo doc --no-deps --document-private-items` (Zero Rustdoc warnings)
