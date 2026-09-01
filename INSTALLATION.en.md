**🌍 [Türkçe](INSTALLATION.md) | [English](INSTALLATION.en.md) | [العربية](INSTALLATION.ar.md) | [日本語](INSTALLATION.ja.md) | [中文](INSTALLATION.zh.md) | [Русский](INSTALLATION.ru.md) | [Español](INSTALLATION.es.md)**

# Claude Code Standalone Installation Guide (100% Rust Engine)

This guide provides step-by-step instructions for installing and configuring **Claude Code Setup** (`claude-code-setup`), a **100% Rust-native** CLI management, security, and memory tool across different platforms.

---

## 🎯 1. Overview

- **Single Binary:** Zero dependencies on Shell (`.sh`) or Python (`.py`) scripts.
- **Cross-Platform:** Native performance on Windows (x64), Linux (x64), and macOS (x64 / ARM64).
- **Zero External Dependencies:** Install by downloading pre-compiled release binaries or building via `cargo` in seconds.

---

## 📥 2. Method 1: Pre-Compiled Binary Download (Recommended)

Download the pre-compiled binary for your operating system directly from GitHub Releases.

### Windows (x64)
Download and execute via PowerShell:
```powershell
# Download binary release asset
Invoke-WebRequest -Uri "https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-windows-x86_64.exe" -OutFile "claude-code-setup.exe"

# Execute setup & memory initialization
.\claude-code-setup.exe install --hooks
```

### Linux (x64)
Download via terminal and grant execution permission:
```bash
# Download binary release asset
curl -LO https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-linux-x86_64

# Grant execution permission
chmod +x claude-code-setup-linux-x86_64

# Execute setup
./claude-code-setup-linux-x86_64 install --hooks
```

### macOS (x64)
```bash
# Download binary release asset
curl -LO https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-macos-x86_64

# Grant execution permission
chmod +x claude-code-setup-macos-x86_64

# Execute setup
./claude-code-setup-macos-x86_64 install --hooks
```

---

## 🛠️ 3. Method 2: Building from Source (Cargo)

If you have a Rust toolchain (`cargo` 1.80+) installed:

```bash
# Clone repository
git clone https://github.com/Ercaner1988/claude-code-setup-rustified.git
cd claude-code-setup-rustified

# Build release binary
cargo build --release

# Execute setup
./target/release/claude-code-setup install --hooks
```

To install system-wide via cargo:
```bash
cargo install --path .
claude-code-setup install --hooks
```

---

## ⚙️ 4. Post-Installation Verification & Diagnostics

Verify system diagnostics after setup:

```bash
# Run environment diagnostics
claude-code-setup status

# Run diagnostic verification test suite
claude-code-setup test
```

---

## 🛡️ 5. Security Auditing & Git Hook Setup

Audit configuration security and install pre-commit branch protection hooks:

```bash
# Security audit with auto-fix
claude-code-setup security-audit --fix

# Install pre-commit hook into a repository
claude-code-setup install-hooks --repo-dir .
```

---

## 📚 6. Related Documentation

- [Full Documentation (README.md)](README.md)
- [Deployment Guide (DEPLOYMENT_GUIDE.md)](DEPLOYMENT_GUIDE.md)
- [Troubleshooting Guide (TROUBLESHOOTING.md)](docs/TROUBLESHOOTING.md)
