# Footprints (FOOTPRINTS.md) - MyTree

**English** | [日本語版](../ja/FOOTPRINTS.md)

This document records the binary size, resource consumption (memory footprint), and compile performance of `MyTree` on release builds.

---

## 1. Measurement Environment

- **OS**: Windows 11 64-bit
- **Toolchain**: Rust stable (2021 edition, `x86_64-pc-windows-msvc`)
- **Target Binary**: `target/release/MyTree.exe`

---

## 2. Footprint Records

### 2.1 Binary Size

| Build Type                        |      Size (KB)      | Remarks                |
| :-------------------------------- | :-----------------: | :--------------------- |
| Debug (`cargo build`)             | Approx. 250 - 450 KB | Includes debug symbols |
| Release (`cargo build --release`) | Approx. 140 - 220 KB | Optimized              |

> **Features**: Since the utility does not import any external dependencies (e.g. `clap`), the release binary remains extremely small at around 200KB.

### 2.2 Memory Footprint (RAM)

- **Runtime Peak Memory**: Approx. 2.5 MB - 4.0 MB
- **Stack Usage**: Traversal depth-dependent. To prevent stack overflows, only lightweight references (`&str`, `&Path`) are passed downstream even during unlimited deep traverses.

### 2.3 Compilation Performance

- **Clean Build Time (`cargo build --release`)**: Approx. 0.8s - 1.5s
- **Incremental Build Time (`cargo build`)**: Approx. 0.2s - 0.5s

---

## 3. Future Optimization Policy (LTO / Strip)

If you wish to shrink the binary size even further, you can append the following profile configurations to `Cargo.toml`:

```toml
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-Time Optimization
codegen-units = 1   # Single code generation unit
panic = "abort"     # Skip panic stack unwinding
strip = true        # Strip symbols
```
