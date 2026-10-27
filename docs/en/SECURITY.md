# Security Policy (SECURITY.md) - MyTree

**English** | [日本語版](../ja/SECURITY.md)

This document outlines the security policies, design principles, and vulnerability reporting procedures for `MyTree`.

---

## 1. Security Architecture

`MyTree` ensures high security standards through the following architectural design principles:

1. **Zero External Dependencies**:
   - By eliminating all third-party crate dependencies, `MyTree` is completely immune to supply-chain attacks and vulnerabilities originating from external packages.
2. **Memory Safety**:
   - Leverages Rust's memory safety guarantees (ownership, borrowing, and strict type safety) to prevent buffer overflows and memory corruption bugs.
3. **Local-Only Execution**:
   - `MyTree` performs zero network communications; it operates strictly as a local file system tree display utility.

---

## 2. Supported Versions

Security updates are provided for the following versions:

| Version                   | Support Status |
| :------------------------ | :------------: |
| Latest Release (`v0.1.x`) |  ✅ Supported  |
| Older Releases            | ❌ Unsupported |

---

## 3. Reporting a Vulnerability

If you discover a security vulnerability in `MyTree`, please report it responsibly rather than opening a public issue:

1. **Contact Method**:
   - Contact the repository maintainer directly or submit a report via private email.
2. **Information to Include**:
   - Affected `MyTree` version and OS environment.
   - Detailed description of the vulnerability and steps/POC to reproduce.
3. **Response Timeline**:
   - We will acknowledge receipt of your report within 3 days and work on releasing a fix as quickly as possible.
