# Footprints (FOOTPRINTS.md) - MyTree

[English](../en/FOOTPRINTS.md) | **日本語版**

本文書は、`MyTree` のリリースビルド時におけるバイナリサイズ、リソース消費量（メモリフットプリント）、およびビルドパフォーマンスの記録です。

---

## 1. 計測環境

- **OS**: Windows 11 64-bit
- **Toolchain**: Rust stable (2021 edition, `x86_64-pc-windows-msvc`)
- **ターゲットバイナリ**: `target/release/MyTree.exe`

---

## 2. フットプリント記録

### 2.1 バイナリサイズ

| ビルドタイプ                      | サイズ (KB)     | 備考                 |
| :-------------------------------- | :-------------: | :------------------- |
| Debug (`cargo build`)             | 約 250 - 450 KB | デバッグシンボル含む |
| Release (`cargo build --release`) | 約 140 - 220 KB | 最適化済             |

> **特徴**: 外部依存ライブラリ（`clap` など）を一切含まないため、リリースバイナリは 200KB 前後の極小サイズを実現しています。

### 2.2 メモリフットプリント (RAM)

- **実行時ピークメモリ**: 約 2.5 MB ~ 4.0 MB
- **スタック使用量**: ツリー再帰探索による深さ依存。スタックオーバーフローを防ぐため、デフォルト制限なしでも軽量なデータ参照 (`&str`, `&Path`) のみを受け渡します。

### 2.3 コンパイル性能

- **クリーンビルド時間 (`cargo build --release`)**: 約 0.8秒 ~ 1.5秒
- **増分ビルド時間 (`cargo build`)**: 約 0.2秒 ~ 0.5秒

---

## 3. 将来的最適化方針 (LTO / Strip)

バイナリサイズをさらに縮小したい場合は、`Cargo.toml` に以下のプロファイルを追記可能です：

```toml
[profile.release]
opt-level = "z"     # サイズ最適化
lto = true          # Link-Time Optimization
codegen-units = 1   # 単一コード生成ユニット
panic = "abort"     # パニック時のスタック展開を省略
strip = true        # シンボルのストリップ
```
