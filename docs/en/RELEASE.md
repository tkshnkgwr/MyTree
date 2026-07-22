# Release Procedure Guide (RELEASE.md) - MyTree

**English** | [日本語版](../ja/RELEASE.md)

This document provides a step-by-step procedure for updating versions and publishing new releases for `MyTree`.

---

## 1. Pre-Release Verification

Ensure all code and documentation meet quality standards prior to release:

1. **Run Verification Commands**:
   ```bash
   cargo test
   cargo clippy --all-targets -- -D warnings
   cargo fmt --check
   cargo doc --no-deps --document-private-items
   ```
2. **Verify Documentation**:
   - Update `docs/en/CHANGELOG.md` and `docs/ja/CHANGELOG.md` with release notes.
   - Record updated binary sizes in `docs/en/FOOTPRINTS.md` and `docs/ja/FOOTPRINTS.md`.

---

## 2. Version Bumping

1. **Update `Cargo.toml`**:
   ```toml
   [package]
   name = "MyTree"
   version = "X.Y.Z" # Specify new version number
   ```
2. **Synchronize `Cargo.lock`**:
   ```bash
   cargo check
   ```
3. **Update Badges in Readmes**:
   - Update version badges in `README.md` and `README_JA.md` (`version-X.Y.Z-green.svg`).

---

## 3. Building & Tagging

1. **Build Optimized Release Binary**:
   ```bash
   cargo build --release
   ```
2. **Create Release Commit & Git Tag**:
   ```bash
   git add .
   git commit -m "chore: release vX.Y.Z"
   git tag -a vX.Y.Z -m "Release version X.Y.Z"
   ```
3. **Push to Remote Repository**:
   ```bash
   git push origin main --tags
   ```

---

## 4. Post-Release Checks

- Confirm GitHub Actions CI/CD pipeline completes successfully.
- Verify GitHub Release artifacts and changelog entries.
