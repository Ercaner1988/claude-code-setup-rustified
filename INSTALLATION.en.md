**🌍 [Türkçe](INSTALLATION.md) | [English](INSTALLATION.en.md) | [العربية](INSTALLATION.ar.md) | [日本語](INSTALLATION.ja.md) | [中文](INSTALLATION.zh.md) | [Русский](INSTALLATION.ru.md) | [Español](INSTALLATION.es.md)**

# Claude Code Standalone Installation Guide (Rust-Core Single Binary)

This guide provides step-by-step instructions for installing and configuring **Claude Code Setup** (`claude-code-setup`), a Rust-core CLI management, security, and memory tool across different platforms.

> **Honesty note:** The binary you run is pure Rust and self-contained. The **installers in this guide are not Rust**: `install-windows.ps1` is a PowerShell script and `install-macos.sh` is a Bash script. Release packages are produced by `package-extension.py` (Python). GitHub language statistics: **90.5% Rust, 3.5% Shell, 3.2% Python, 2.8% PowerShell** (measured line ratio: 91.2% Rust, 8.8% PowerShell + Bash + Python).

---

## 🎯 1. Overview

- **Single Binary Runtime:** Running the tool requires no Rust, Python, or Node installation. The installers themselves are PowerShell/Bash scripts, and the `.mcpb` packager is Python — those run around the tool, not inside it.
- **Cross-Platform:** Native performance on Windows (x64), Linux (x64), and macOS (x64 / ARM64).
- **Offline After First Run:** Semantic search downloads the embedding model and the ONNX Runtime binary from Hugging Face on first use, then works fully offline from the `$HOME/.claude/fastembed_cache` directory.

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
