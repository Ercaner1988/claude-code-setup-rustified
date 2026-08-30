**🌍 [Türkçe](README.md) | [English](README.en.md) | [العربية](README.ar.md) | [日本語](README.ja.md) | [中文](README.zh.md) | [Русский](README.ru.md) | [Español](README.es.md)**

# Claude Code 独立安装程序 (100% Rust 引擎)

[![Rust](https://img.shields.io/badge/Rust-100%25-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/Tests-24%20Passed-green.svg)]()

专为 **Claude Code** 环境打造的高性能、单文件、**100% Rust 原生** 部署、安全审计与记忆引擎 (`claude-code-setup.exe`)。

已完全移除旧版 Bash (`.sh`) 及 Python (`.py`) 脚本，重构为统一的 Rust CLI 工具。

---

## 🎯 1. 目标与特性

- **100% 纯 Rust 架构:** 零 Shell 脚本及 Python 运行时依赖。
- **动态路径标准化:** 硬编码路径模式 (如 `/home/jb_remus`) 自动解析为目标操作系统及本地用户主目录。
- **多目标 MCP 管理 (`--target`):**
  - 支持在统一 CLI 中动态管理 **Claude Code** (`~/.claude.json`)、**项目** (`./.mcp.json`) 及 **Claude Desktop** (`claude_desktop_config.json`) 配置。
  - 采用 JSON Value 模式更新配置，保留未知字段，并提供自动 `.bak` 备份。
- **基于 SQLite 的高速记忆引擎 (向量 + 图谱):**
  - **快速创建笔记 (`memory-note`):** 使用 kebab-case 文件名安全添加 Markdown 笔记，绝不覆盖已有文件。
  - **FTS5 关键词检索:** 全文检索并对特殊查询语法进行自动转义。
  - **本地嵌入 (Embeddings):** 通过 `fastembed` (Multilingual-E5-Small) 完全离线生成余弦相似度。
  - **图谱边与 Wikilink 关联:** 基于 `[[Note-Name]]` 引用及语义关联进行广度优先 (BFS) 邻域搜索 (`memory-related`)。
  - **RRF 混合排序:** 采用倒数排名融合 (Reciprocal Rank Fusion, `k=60`) 算法融合检索结果。
- **自动修复安全审计 (`security-audit --fix`):**
  - 扫描配置文件中的明文密钥 (tokens)。
  - 在 Unix 系统上自动修复文件权限。
  - 自动安装 Git 提交前分支保护与密钥扫描钩子 (hooks)。
- **安全自主 Git 工作流 (`agent-workflow`):**
  - 自动从远程默认分支派生功能分支。
  - 强制执行受保护分支防护，禁止直接 Push 到主分支。

---

## 🏗️ 2. 架构与模块

```
claude-code-complete-setup/
├── Cargo.toml                  # 项目清单与 Rust 依赖定义
├── src/
│   ├── main.rs                 # CLI 入口点与命令路由
│   ├── cli.rs                  # 基于 Clap 的命令、目标与标志定义
│   ├── mcp.rs                  # 支持多目标的 JSON Value 保留 MCP 管理器
│   ├── memory_engine.rs        # FTS5 + 向量 + 图谱 + RRF + memory-note 引擎
│   ├── installer.rs            # 骨架目录、初始 README 与 .env 构建器
│   ├── security.rs             # 包含自动修复的安全审计员与钩子管理器
│   ├── branch_manager.rs       # 受保护分支安全的 Git 工作流运行器
│   ├── tester.rs               # 诊断测试套件运行器
│   └── agent.rs                # 智能体集成接口
└── docs/                       # 安装与故障排查指南
```

### 模块职责
- `src/main.rs`: 解析命令行参数并分发至对应模块。
- `src/cli.rs`: 通过 Clap `Parser` 结构体管理所有子命令、目标标志 (`--target`)、修整标志 (`--fix`) 与帮助信息。
- `src/mcp.rs`: 根据 `--target` 参数 (`claude-code`, `project`, `claude-desktop`) 读取和更新 MCP 配置，保留未知字段。
- `src/memory_engine.rs`: 管理 `knowledge_notes` 与 `note_edges` SQLite 数据表。通过 `memory-note` 安全添加笔记。
- `src/installer.rs`: 创建 `~/claude_global_memory/knowledge` 目录及初始 `README.md`，绝不覆盖已有文件。
- `src/security.rs`: 审计权限与密钥，使用 `--fix` 自动修复并安装 Git 安全钩子。
- `src/branch_manager.rs`: 自动完成分支创建与受保护分支防护检查。
- `src/tester.rs`: 执行系统诊断 (`status`) 与测试验证 (`test`)。

---

## 🚀 3. 安装与配置

### 前置条件
- **Rust 工具链:** `rustc` 与 `cargo` (1.80+)

### 编译
```bash
# 编译 Release 二进制文件
cargo build --release

# 生成的二进制文件路径:
# Windows: ./target/release/claude-code-setup.exe
# Linux/macOS: ./target/release/claude-code-setup
```

### 自动安装与诊断
```bash
# 运行自动安装程序并安装安全钩子
./target/release/claude-code-setup install --hooks

# 运行诊断状态检查
./target/release/claude-code-setup status
```

---

## 📖 4. 使用说明与示例

### 命令汇总表

| 命令 | 说明 |
| :--- | :--- |
| `install [--hooks]` | 执行自动化环境配置、骨架初始化与 `.env` 生成 |
| `test` / `status` | 运行 Claude CLI、`.claude.json`、记忆 DB 与钩子诊断 |
| `mcp-list [--target T]` | 列出指定目标的已配置 MCP 服务器 |
| `mcp-set <srv> [...] [--target T]` | 添加或更新 MCP 服务器 (`--target`: `claude-code`, `project`, `claude-desktop`) |
| `mcp-unset <srv> [...] [--remove] [--target T]` | 移除配置项；完全删除服务器需要指定 `--remove` |
| `mcp-enable <srv>` / `mcp-disable <srv>` | 启用/禁用 MCP 服务器且保留其配置 |
| `memory-note <标题> [--body ...]` | 安全添加新的 Markdown 笔记至知识库 |
| `memory-index [--source 目录]...` | 将笔记索引至 SQLite + 向量 + 图谱引擎 |
| `memory-search <查询> [--mode ...]` | 使用 FTS5 关键词、向量或 RRF 混合模式搜索笔记 |
| `memory-related <笔记.md>` | 通过图谱边与 Wikilink 检索关联笔记 |
| `install-hooks [--repo-dir 路径]` | 安装 Git pre-commit 分支保护钩子 |
| `security-audit [--fix]` | 审计权限安全与密钥；`--fix` 执行自动修复 |
| `agent-workflow [-t 类型] -d 描述` | 执行带防护机制的自主 Git 分支与提交工作流 |

### 典型使用场景

#### 按目标管理 MCP 服务器
```bash
# 在项目层级 (.mcp.json) 配置 MCP 服务器
./target/release/claude-code-setup mcp-set github --command "npx" --arg "-y" --arg "@modelcontextprotocol/server-github" --env "GITHUB_TOKEN=ghp_example" --target project

# 禁用 Claude Desktop 配置中的服务器
./target/release/claude-code-setup mcp-disable github --target claude-desktop

# 完全删除服务器 (必须指定 --remove 标志)
./target/release/claude-code-setup mcp-unset github --remove --target claude-code
```

#### 添加记忆笔记与混合检索
```bash
# 添加新笔记
./target/release/claude-code-setup memory-note "架构决策" --body "完成 100% Rust 二进制重构。"

# 索引知识笔记
./target/release/claude-code-setup memory-index --edge-threshold 0.70

# 执行 RRF 混合检索
./target/release/claude-code-setup memory-search "Rust 架构" --mode hybrid --limit 5

# 查询关联笔记
./target/release/claude-code-setup memory-related architecture-decisions.md
```

---

## 🛡️ 5. 质量门禁与测试

本项目包含 24 个单元测试，当前已全部通过：

```bash
cargo test
```

### 质量标准
- **单元测试 (24/24 通过):** 覆盖多目标 MCP 管理、JSON Value 结构保留、FTS5 转义、RRF 混合排序、均值池化、Wikilink 解析、密钥审计及受保护分支防护。
- **代码格式化:** 通过 `cargo fmt --check` 强制校验。
- **持续集成 (CI):** 已在 Ubuntu、macOS 与 Windows 上通过 `.github/workflows/rust.yml` 和 `.github/workflows/release.yml` 校验。

---

## 👥 6. 贡献者

| 贡献者 | 角色 / 职责 | 贡献指标 |
| :--- | :--- | :--- |
| **Ercan ER** | 首席架构师、Rust 迁移与主要开发者 | 26 commits |
| **Kassam** | 自主 AI 智能体、Rust 引擎与模块开发者 | 联合作者 / 贡献者 |
| **Copilot** | AI 编码助手 | 4 commits |
| **jb_remus** | 上游原始开发者 (Upstream) | 2 commits |
| **Mihenk** | 代码审计员与质量评审员 | 1 commit |
| **arturo-ebuck** | 开源贡献者 | 1 commit |

---

## 📄 7. 许可证与相关资源

本程序基于 [MIT 许可证](LICENSE) 发布。

### 相关文档
- [部署指南](DEPLOYMENT_GUIDE.md)
- [手动安装指南](docs/MANUAL_SETUP.md)
- [故障排查指南](docs/TROUBLESHOOTING.md)
- [开发者指令](docs/dev/TASK-KASSAM-1-2.md)
