**🌍 [Türkçe](INSTALLATION.md) | [English](INSTALLATION.en.md) | [العربية](INSTALLATION.ar.md) | [日本語](INSTALLATION.ja.md) | [中文](INSTALLATION.zh.md) | [Русский](INSTALLATION.ru.md) | [Español](INSTALLATION.es.md)**

# Claude Code 独立安装指南 (Rust 内核单一二进制)

本指南提供在不同平台上安装和配置 Rust 内核 CLI 管理工具 **Claude Code Setup** (`claude-code-setup`) 的详细步骤。

> **诚实说明：** 您运行的二进制文件是纯 Rust 编写且自成一体的。但本指南中的**安装脚本并非 Rust**：`install-windows.ps1` 是 PowerShell 脚本，`install-macos.sh` 是 Bash 脚本。发布包由 `package-extension.py` (Python) 生成。GitHub 语言统计：**Rust 90.5%、Shell 3.5%、Python 3.2%、PowerShell 2.8%**（实测行数构成：Rust 91.2%，PowerShell + Bash + Python 8.8%）。

---

## 🎯 1. 概述

- **单文件二进制 (Single Binary):** 彻底消除对 Shell (`.sh`) 及 Python (`.py`) 脚本的依赖。
- **跨平台支持:** 在 Windows (x64)、Linux (x64) 和 macOS (x64 / ARM64) 上提供原生运行性能。
- **零外部依赖:** 直接下载预编译二进制文件或在数秒内通过 `cargo` 构建即可完成安装。

---

## 📥 2. 方法 1: 下载预编译二进制文件 (推荐)

可直接从 GitHub Release 页面下载适用于您操作系统的可执行文件。

### Windows (x64)
通过 PowerShell 下载并运行：
```powershell
# 下载 release 二进制资源
Invoke-WebRequest -Uri "https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-windows-x86_64.exe" -OutFile "claude-code-setup.exe"

# 启动自动安装与记忆库初始化
.\claude-code-setup.exe install --hooks
```

### Linux (x64)
在终端中下载并赋予执行权限：
```bash
# 下载二进制资源
curl -LO https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-linux-x86_64

# 赋予执行权限
chmod +x claude-code-setup-linux-x86_64

# 运行安装
./claude-code-setup-linux-x86_64 install --hooks
```

### macOS (x64)
```bash
# 下载二进制资源
curl -LO https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-macos-x86_64

# 赋予执行权限
chmod +x claude-code-setup-macos-x86_64

# 运行安装
./claude-code-setup-macos-x86_64 install --hooks
```

---

## 🛠️ 3. 方法 2: 从源码构建 (Cargo)

如果您的电脑已安装 Rust 工具链 (`cargo` 1.80+)：

```bash
# 克隆代码库
git clone https://github.com/Ercaner1988/claude-code-setup-rustified.git
cd claude-code-setup-rustified

# 构建 release 二进制
cargo build --release

# 运行安装
./target/release/claude-code-setup install --hooks
```

如需全局安装至系统 PATH：
```bash
cargo install --path .
claude-code-setup install --hooks
```

---

## ⚙️ 4. 安装后验证与诊断

安装完成后运行诊断：

```bash
# 运行环境状态诊断
claude-code-setup status

# 运行诊断测试套件
claude-code-setup test
```

---

## 🛡️ 5. 安全审计与 Git 钩子设置

```bash
# 自动修复安全审计
claude-code-setup security-audit --fix

# 在目标仓库安装提交前保护钩子
claude-code-setup install-hooks --repo-dir .
```

---

## 📚 6. 相关文档

- [完整文档 (README.md)](README.md)
- [部署指南 (DEPLOYMENT_GUIDE.md)](DEPLOYMENT_GUIDE.md)
- [故障排查指南 (TROUBLESHOOTING.md)](docs/TROUBLESHOOTING.md)
