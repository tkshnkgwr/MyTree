# Testing Policy & Execution Guide (TESTING.md) - MyTree

**English** | [日本語版](../ja/TESTING.md)

This document outlines the testing strategy, execution steps, and quality verification guidelines for the `MyTree` project.

---

## 1. Overview

`MyTree` is a lightweight CLI utility built exclusively with Rust's standard library (`std`). Comprehensive testing ensures reliability, correctness, and cross-platform consistency.

Key test areas:
- **Argument Parsing**: Option parsing, default configurations, and flag conflicts.
- **Path Filtering**: Wildcard matching (`*`, `?`) and `.gitignore` pattern parsing.
- **Tree Traversal**: Directory walking algorithm and recursion depth control.
- **Serialization Output**: Plain text, Classic mode, JSON, and XML format output generation.
- **Cross-Platform Compatibility**: Path separators and character encoding handling on Windows, Linux, and macOS.

---

## 2. Running Tests

Execute local tests using standard `cargo` commands:

### Run All Unit Tests
```bash
cargo test
```

### Run Specific Test Case
```bash
cargo test test_parse_arguments
```

### Pre-Commit Verification Pipeline
Before submitting changes, ensure the following commands complete without any errors or warnings:

```bash
# 1. Run unit tests
cargo test

# 2. Static analysis (Clippy)
cargo clippy --all-targets -- -D warnings

# 3. Check code formatting
cargo fmt --check

# 4. Build Rustdoc documentation
cargo doc --no-deps --document-private-items
```

---

## 3. Test Guidelines

1. **Inline Test Modules**:
   - Place `#[cfg(test)] mod tests` at the bottom of each source module to test internal functions.
2. **Robustness & Error Handling**:
   - Test invalid inputs (non-existent paths, malformed arguments) to ensure explicit error handling without panics.
3. **Platform-Specific Attributes**:
   - Use `#[cfg(windows)]` or `#[cfg(not(windows))]` for platform-specific test logic.
