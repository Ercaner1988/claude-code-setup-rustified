**🌍 [Türkçe](README.md) | [English](README.en.md) | [العربية](README.ar.md) | [日本語](README.ja.md) | [中文](README.zh.md) | [Русский](README.ru.md) | [Español](README.es.md)**

# Claude Code 独立安装配置 (Rust 内核单一二进制)

[![Rust](https://img.shields.io/badge/Rust%20core-%2591-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/Tests-30%20Passed-green.svg)]()

用于管理 **Claude Code** 环境的本地部署、安全审计与记忆引擎 (`claude-code-setup`)。运行时是单一的 Rust 二进制文件；运行它无需在机器上安装 Rust、Python 或 Node。

### 诚实说明：本仓库并非 100% Rust

GitHub 语言统计与实测代码库构成 (2026-09-02)：**Rust 2891 行 / 非 Rust 279 行 = 行数占比 91.2% Rust** (GitHub Linguist：**Rust 90.5%、Shell 3.5%、Python 3.2%、PowerShell 2.8%**)。

| 语言 / 文件 | 行数 | GitHub 占比 | 何时运行 |
| :--- | ---: | ---: | :--- |
| `Rust` (`src/*.rs`，10 个文件) | 2891 | 90.5% | 运行时执行 (CLI + MCP 服务端) |
| `install-macos.sh` (Shell / Bash) | 121 | 3.5% | Linux/macOS 安装时 |
| `package-extension.py` (Python) | 87 | 3.2% | 仅在版本发布 / `.mcpb` 打包时 (CI) |
| `install-windows.ps1` (PowerShell) | 71 | 2.8% | Windows 安装时 |

其他依赖：
- `src/security.rs` 中的 pre-commit 钩子以**内嵌 bash 脚本**形式写出 (`#!/usr/bin/env bash`)，其运行需要 Git 自带的 bash。
- `.github/workflows/release.yml` 发布流程使用 `actions/setup-python` 与 `npx @anthropic-ai/mcpb validate`，因此 **CI 链条依赖 Python 与 Node**。
- 记忆引擎的嵌入层通过 `fastembed` **下载 ONNX Runtime 预编译的 C++ 二进制文件** (`ort-download-binaries`)。

准确的概括：**运行时二进制是纯 Rust；但安装、打包与发布流程使用 Bash + PowerShell + Python + Node。**

---

## 🎯 1. 目标与已完成事项

- **单一二进制运行时：** 遗留的 Bash 与 Python *运行时*脚本已迁移至 Rust。安装与打包脚本 (`install-*.{sh,ps1}`、`package-extension.py`) 被有意保留，因为安装器本身必须在二进制文件下载之前运行。
- **动态路径规整：** 硬编码的路径定义 (如 `/home/jb_remus`) 会动态适配目标操作系统与本地用户主目录。
- **多目标 MCP 管理 (`--target`)：**
  - 通过同一个 CLI 管理 **Claude Code** (`~/.claude.json`)、**项目** (`./.mcp.json`) 与 **Claude Desktop** (`claude_desktop_config.json`) 三种配置。
  - 借助 `serde_json::Value` 结构保留未定型 JSON 字段的原子写入引擎 (附带 `.bak` 自动备份)。
- **MCP 服务器模式 (`--mcp-mode`)：** 同一个二进制可化为通过 stdin/stdout 讲 JSON-RPC 的 MCP 服务器，并将 `manifest.json` 中声明的 8 个工具提供给 Claude Desktop。
- **记忆引擎 (SQLite + 向量 + 图)：**
  - **快速添加笔记 (`memory-note`)：** 以 kebab-case 文件名安全创建笔记。
  - **FTS5 关键词检索：** 带引号转义机制的 SQLite 关键词索引。
  - **本地嵌入：** 通过 `fastembed` (Multilingual-E5-Small) 计算余弦相似度。模型在首次使用时自 Hugging Face 下载并写入 `$HOME/.claude/fastembed_cache`；**在首次下载之后**，检索完全离线运行。
  - **图边与 Wikilink：** 经由 `[[笔记名]]` 链接与超过阈值的语义边进行邻域检索 (`memory-related`)。
  - **RRF 混合排序：** 以 Reciprocal Rank Fusion (`k=60`) 融合 FTS5 与向量检索结果。
- **自动修复式安全审计 (`security-audit --fix`)：**
  - 扫描配置文件中的明文密钥 (`ghp_`、`github_pat_`、`sk-`、`xox[baprs]-`、`AKIA`)。
  - 将文件权限收紧至 600 — **仅限 Unix**；在 Windows 上只打印关于 ACL 权限的提示信息，不做修改。
  - 安装 Git pre-commit 分支保护与密钥扫描钩子。
- **自主 Git 工作流 (`agent-workflow`)：**
  - 自远端默认分支自动派生特性分支。
  - 阻止向受保护主分支的直接推送。

---

## 🏗️ 2. 架构与模块

```
claude-code-setup-rustified/
├── Cargo.toml                  # Rust 依赖与包定义 (v0.1.6)
├── manifest.json               # Claude Desktop 扩展清单 (8 个 MCP 工具)
├── icon.png                    # 扩展图标
├── .env.example                # 环境变量示例
├── src/
│   ├── main.rs                 # CLI 入口与命令分派 (123 行)
│   ├── cli.rs                  # 基于 Clap 的命令、目标与旗标定义 (222)
│   ├── mcp.rs                  # 多目标、保留 JSON Value 的 MCP 管理器 (488)
│   ├── mcp_server.rs           # MCP stdio JSON-RPC 服务器；将 8 个工具映射至 CLI (436)
│   ├── memory_engine.rs        # FTS5 + 向量 + 图 + RRF + memory-note 引擎 (821)
│   ├── installer.rs            # 骨架目录、初始 README 与 .env 安装器 (191)
│   ├── security.rs             # 自动修复式安全审计器与钩子管理器 (296)
│   ├── branch_manager.rs       # 带受保护分支防护的自主 Git 工作流执行器 (161)
│   ├── tester.rs               # 系统与环境诊断测试执行器 (123)
│   └── agent.rs                # 代理整合接口 (30)
├── install-windows.ps1         # PowerShell 安装脚本 (非 Rust)
├── install-macos.sh            # Bash 安装脚本 (非 Rust)
├── package-extension.py        # .mcpb 打包器，在 CI 中调用 (非 Rust)
├── .github/workflows/
│   ├── rust.yml                # fmt + clippy + test + build (ubuntu/windows/macos)
│   └── release.yml             # 三平台二进制与 .mcpb 发布流程
└── docs/                       # 安装与疑难排解指南
```

### 模块职责
- `src/main.rs`：解析命令行参数；若给出 `--mcp-mode` 则将控制权交予 MCP 服务器，否则交予相应模块函数。
- `src/cli.rs`：通过 Clap 的 `Parser` 结构管理 15 个子命令、旗标 (`--target`、`--fix`、`--hooks`、`--mode`、`--min-score`) 以及全局旗标 `--mcp-mode`。
- `src/mcp.rs`：依 `--target` 参数 (`claude-code`、`project`、`claude-desktop`) 读取并更新 MCP 设置；原子写入且不删除未知字段。
- `src/mcp_server.rs`：建立 stdin/stdout 的 JSON-RPC 循环；将 `manifest.json` 中的 8 个工具 (`mcp_list`、`mcp_add`、`security_audit`、`memory_note`、`memory_index`、`memory_search`、`status`、`test`) 映射到真实的 CLI 命令。该映射由 `her_arac_gercek_bir_cli_komutuna_esleniyor` 测试锁定。
- `src/memory_engine.rs`：将笔记切分为约 1500 字符的窗口后嵌入并取均值 (mean-pooling)；管理 SQLite 的 `knowledge_notes` 与 `note_edges` 两张表。嵌入缓存位于 `$HOME/.claude/fastembed_cache`。
- `src/installer.rs`：创建 `$HOME/claude_global_memory/knowledge` 目录与初始 `README.md`，绝不覆盖已有内容；若 `.env` 不存在则复制。
- `src/security.rs`：扫描明文密钥、检查权限、以 `--fix` 修复，并安装 Git 钩子 (该钩子是内嵌的 bash 脚本)。
- `src/branch_manager.rs`：管理自主分支创建、受保护分支拦截以及安全的 commit/push 流程。
- `src/tester.rs`：执行系统诊断 (`status`) 与测试校验。

---

## 🚀 3. 安装与配置

### 快速上手

存在两种不同的安装方式，请先确定所需。

**Claude Desktop 扩展 (推荐)** — 自[最新发布](https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest)下载与操作系统相符的包，拖入 Claude Desktop → Settings → Extensions 界面：

| 操作系统 | 文件 | 约计大小 |
|---|---|---|
| Windows | `claude-code-setup-windows.mcpb` | 9 MB |
| macOS | `claude-code-setup-macos.mcpb` | 10 MB |
| Linux | `claude-code-setup-linux.mcpb` | 12 MB |

**命令行工具** — 若希望自终端使用：

```powershell
irm https://raw.githubusercontent.com/Ercaner1988/claude-code-setup-rustified/main/install-windows.ps1 | iex
```

```bash
curl -fsSL https://raw.githubusercontent.com/Ercaner1988/claude-code-setup-rustified/main/install-macos.sh | bash
```

这两个安装脚本分别是 PowerShell 与 Bash 脚本 (并非 Rust)；它们将下载的二进制文件安装至用户目录并加入 PATH (无需管理员权限)。它们**不会注册**扩展 — 扩展请走上文的 `.mcpb` 路径。验证时可在新终端中运行 `claude-code-setup status`。

详细安装步骤参见 [INSTALLATION.zh.md](INSTALLATION.zh.md)

---

### 手动安装：自源码构建

#### 前置条件
- **Rust 工具链：** `rustc` 与 `cargo` (1.80 及以上)
- 首次构建时 `fastembed` 会下载 ONNX Runtime 二进制文件，故需网络连接。

#### 构建
```bash
cargo build --release

# 生成的二进制文件：
# Windows: ./target/release/claude-code-setup.exe
# Linux/macOS: ./target/release/claude-code-setup
```

### 自动安装与环境诊断
```bash
# 检查前置条件并建立记忆骨架
./target/release/claude-code-setup install --hooks

# 系统与环境诊断状态
./target/release/claude-code-setup status
```

---

## 📖 4. 使用方法与示例

### 命令概览表

| 命令 | 说明 |
| :--- | :--- |
| `--mcp-mode` (全局旗标) | 将二进制作为通过 stdin/stdout 讲 JSON-RPC 的 MCP 服务器运行 |
| `install [--hooks] [--skip-prereqs]` | 环境安装、记忆骨架与 `.env` 复制 |
| `test` / `status` | 对 Claude CLI、`.claude.json`、记忆数据库与钩子进行诊断 |
| `mcp-list [--target T]` | 按目标列出已配置的 MCP 服务器 |
| `mcp-set <srv> [--command C] [--arg A]… [--env K=V]… [--target T]` | 添加或更新 MCP 服务器 (`--target`：`claude-code`、`project`、`claude-desktop`) |
| `mcp-unset <srv> [--env K]… [--clear-args] [--remove] [--target T]` | 删除变量，或彻底移除服务器 (`--remove` 为必需) |
| `mcp-enable <srv>` / `mcp-disable <srv>` | 在不破坏配置的前提下启用/停用服务器 |
| `memory-note <标题> [--body ...] [--dir D]` | 向知识库添加新的 Markdown 笔记 |
| `memory-index [--source 目录]… [--edge-threshold 0.70]` | 将笔记索引进 SQLite + 向量 + 图引擎 |
| `memory-search <查询> [--mode keyword\|semantic\|hybrid] [--limit 5] [--min-score 0.30]` | 以 FTS5 关键词、向量或 RRF 混合模式检索记忆 |
| `memory-related <note.md>` | 经由图边与 Wikilink 列出相关笔记 |
| `install-hooks [--repo-dir 路径]` | 为仓库安装 pre-commit 安全钩子 |
| `security-audit [--fix]` | 执行安全审计；以 `--fix` 应用自动修复 |
| `agent-workflow [--branch-type 类型] --description 说明 [--files F]…` | 运行带受保护分支防护的自主 Git 分支与 commit 工作流 |

所有命令均接受 `--home-dir` 覆写以隔离测试 (`install-hooks` 与 `agent-workflow` 除外)。

### 使用示例

#### 按目标管理 MCP 服务器
```bash
# 在项目层级 (.mcp.json) 定义 MCP 服务器
./target/release/claude-code-setup mcp-set github \
  --command "npx" --arg "-y" --arg "@modelcontextprotocol/server-github" \
  --env "GITHUB_TOKEN=$GITHUB_TOKEN" --target project

# 停用 Claude Desktop 配置中的服务器
./target/release/claude-code-setup mcp-disable github --target claude-desktop

# 彻底移除服务器 (--remove 旗标为强制)
./target/release/claude-code-setup mcp-unset github --remove --target claude-code
```

#### 添加记忆笔记与 RRF 混合检索
```bash
./target/release/claude-code-setup memory-note "架构决策" --body "运行时向 Rust 原生二进制的迁移已完成。"
./target/release/claude-code-setup memory-index --edge-threshold 0.70
./target/release/claude-code-setup memory-search "Rust 架构" --mode hybrid --limit 5 --min-score 0.30
./target/release/claude-code-setup memory-related mimari-kararlar.md
```

---

## 🛡️ 5. 测试与质量关卡

```bash
cargo test
# running 30 tests
# test result: ok. 30 passed; 0 failed; 0 ignored
```

源码中定义了 **31 个测试**；其中一个 (`test_enforce_file_permissions_fixes_mode`) 标注了 `#[cfg(unix)]`，故在 Windows 上不参与编译。实测：**Windows 上 30/30，Unix 上 31/31 全绿** (2026-09-02)。

按文件分布：`memory_engine.rs` 14、`mcp.rs` 5、`mcp_server.rs` 5、`security.rs` 3、`branch_manager.rs` 2、`installer.rs` 2。

### 质量标准
- **覆盖范围：** MCP 多目标管理、JSON `Value` 保留、FTS5 字符转义、RRF 混合排序、mean-pooling、Wikilink 解析、嵌入缓存路径回归、密钥扫描、MCP 工具与 CLI 的映射，以及受保护分支拦截。
- **格式化：** `cargo fmt --all -- --check` → 洁净 (2026-09-02)。
- **静态检查：** `cargo clippy --all-targets -- -D warnings` → 无警告 (2026-09-02)。
- **持续集成：** `.github/workflows/rust.yml` 在三种操作系统 (ubuntu、windows、macos) 上运行 fmt + clippy + test + release 构建。`.github/workflows/release.yml` 产出三平台二进制与 `.mcpb` 包；该流程使用 Python 与 Node。

---

## 👥 6. 贡献者

以下数字由 `git shortlog -sne --all` 及对提交正文中 `Co-authored-by` 标记的计数实测所得 (2026-09-02，共 45 次提交)。

| 贡献者 | 角色 / 职责 | 实测贡献 |
| :--- | :--- | :--- |
| **Ercan ER** | 项目架构、Rust 迁移、主要开发者 | 41 次提交 (作者) |
| **Claude Opus 5** | 自主 AI 代理、模块开发 | 14 次提交 (共同作者) |
| **Copilot App** | AI 编码助手 | 11 次提交 (共同作者) |
| **Claude Opus 4.8** | 自主 AI 代理 | 3 次提交 (共同作者) |
| **Claude** (未标版本) | 自主 AI 代理 | 2 次提交 (共同作者) |
| **jb_remus** | 上游 (upstream) 原始作者 | 2 次提交 (作者) |
| **Mihenk** | 代码审查者与质量裁判 | 1 次提交 (作者) |
| **arturo-ebuck** | 开源贡献者 | 1 次提交 (作者) |

**Kassam** 是记录于 `Cargo.toml` `authors` 字段中的代理身份，并无独立的 Git 作者记录。

---

## 📄 7. 许可与资源

本项目依 [MIT 许可证](LICENSE) 授权 (版权所有 © 2026 Ercan Er)。

### 相关文档
- [部署指南](DEPLOYMENT_GUIDE.md)
- [手动安装指南](docs/MANUAL_SETUP.md)
- [疑难排解指南](docs/TROUBLESHOOTING.md)
- [开发者指令](docs/dev/TASK-KASSAM-1-2.md)
