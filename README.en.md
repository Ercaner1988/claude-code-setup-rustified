**🌍 [Türkçe](README.md) | [English](README.en.md) | [العربية](README.ar.md) | [日本語](README.ja.md) | [中文](README.zh.md) | [Русский](README.ru.md) | [Español](README.es.md)**

# Claude Code Standalone Setup (100% Rust Engine)

[![Rust](https://img.shields.io/badge/Rust-100%25-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/Tests-24%20Passed-green.svg)]()

A high-performance, single-binary **100% Rust-native** deployment, security auditing, and memory engine (`claude-code-setup.exe`) for **Claude Code**.

All legacy Bash (`.sh`) and Python (`.py`) scripts have been completely removed and refactored into a unified Rust CLI tool.

---

## 🎯 1. Purpose & Features

- **100% Pure Rust Architecture:** Zero dependencies on Shell scripts or Python runtimes.
- **Dynamic Path Normalization:** Hardcoded path patterns (e.g. `/home/jb_remus`) automatically resolve to the target environment and local home directory.
- **Multi-Target MCP Management (`--target`):**
  - Dynamically manage MCP servers across **Claude Code** (`~/.claude.json`), **Project** (`./.mcp.json`), and **Claude Desktop** (`claude_desktop_config.json`).
  - Value-preserving JSON configuration manager preserving unmodeled fields with automated `.bak` backups.
- **SQLite-Backed Fast Memory Engine (Vector + Graph):**
  - **Fast Note Creation (`memory-note`):** Safely add Markdown notes with kebab-case filenames without overwriting existing files.
  - **FTS5 Keyword Search:** Full-text search with automatic string escaping for special query syntax.
  - **Local Embeddings:** Offline local cosine similarity generated via `fastembed` (Multilingual-E5-Small).
  - **Graph Edges & Wikilinks:** BFS neighborhood traversal across `[[Note-Name]]` references and semantic ties (`memory-related`).
  - **RRF Hybrid Ranking:** Reciprocal Rank Fusion (`k=60`) merging keyword and vector search results seamlessly.
- **Auto-Fixing Security Audit (`security-audit --fix`):**
  - Scans configuration files for plaintext secret tokens.
  - Enforces and automatically repairs file permissions on Unix systems.
  - Installs Git pre-commit branch protection and secret scanning hooks.
- **Safe Autonomous Git Workflow (`agent-workflow`):**
  - Automates feature branch creation from default remote branches.
  - Enforces protected branch guards to prevent direct pushes to main/master.

---

## 🏗️ 2. Architecture & Modules

```
claude-code-complete-setup/
├── Cargo.toml                  # Project manifest & Rust dependencies
├── src/
│   ├── main.rs                 # CLI entry point & command router
│   ├── cli.rs                  # Clap command definitions, targets & flags
│   ├── mcp.rs                  # Multi-target JSON Value-preserving MCP manager
│   ├── memory_engine.rs        # FTS5 + Vector + Graph + RRF + memory-note engine
│   ├── installer.rs            # Skeleton directory, seed README & .env builder
│   ├── security.rs             # Auto-fixing security auditor & hook manager
│   ├── branch_manager.rs       # Protected-branch safe Git workflow runner
│   ├── tester.rs               # Environment diagnostic & test suite runner
│   └── agent.rs                # Agent integration interface
└── docs/                       # Setup & troubleshooting guides
```

### Module Responsibilities
- `src/main.rs`: Parses CLI flags and dispatches execution to dedicated modules.
- `src/cli.rs`: Manages commands, options (`--target`, `--fix`, `--hooks`, `--mode`), and help messages via Clap `Parser`.
- `src/mcp.rs`: Reads and updates MCP configuration based on `--target` (`claude-code`, `project`, `claude-desktop`), retaining custom fields via `serde_json::Value`.
- `src/memory_engine.rs`: Windows text into ~1500-character chunks, computes mean-pooled embeddings, and manages `knowledge_notes` and `note_edges` SQLite tables. Includes safe note creation (`memory-note`).
- `src/installer.rs`: Bootstraps `~/claude_global_memory/knowledge` skeleton and seed `README.md` without overwriting existing files; copies `.env` if missing.
- `src/security.rs`: Audits permissions, scans plaintext secrets, applies `--fix` repairs, and installs pre-commit security hooks.
- `src/branch_manager.rs`: Automates feature branch creation, file staging, commits, and protected-branch guard checks.
- `src/tester.rs`: Executes diagnostic verification checks (`status` and `test`).

---

## 🚀 3. Installation & Setup

### Prerequisites
- **Rust Toolchain:** `rustc` and `cargo` (1.80+)

### Compilation
```bash
# Build release binary
cargo build --release

# Resulting binary:
# Windows: ./target/release/claude-code-setup.exe
# Linux/macOS: ./target/release/claude-code-setup
```

### Automated Setup & Diagnostics
```bash
# Run automated setup and install pre-commit security hooks
./target/release/claude-code-setup install --hooks

# Run environment diagnostics
./target/release/claude-code-setup status
```

---

## 📖 4. Usage & Examples

### Command Summary

| Command | Description |
| :--- | :--- |
| `install [--hooks]` | Full setup, knowledge skeleton creation & `.env` initialization |
| `test` / `status` | Environment diagnostics: Claude CLI, `.claude.json`, memory DB, hooks |
| `mcp-list [--target T]` | List configured MCP servers for specified target |
| `mcp-set <srv> [...] [--target T]` | Add or update an MCP server (`--target`: `claude-code`, `project`, `claude-desktop`) |
| `mcp-unset <srv> [...] [--remove] [--target T]` | Remove fields; `--remove` required to delete server |
| `mcp-enable <srv>` / `mcp-disable <srv>` | Toggle a server without deleting its config |
| `memory-note <title> [--body ...]` | Add a new Markdown note to knowledge base safely |
| `memory-index [--source DIR]...` | Index notes into SQLite + Vector + Graph engine |
| `memory-search <query> [--mode ...]` | Search notes using FTS5 Keyword, Vector, or RRF Hybrid mode |
| `memory-related <note.md>` | Show related notes via graph edges (BFS) |
| `install-hooks [--repo-dir PATH]` | Install pre-commit security hook into a repository |
| `security-audit [--fix]` | Audit permission security & secrets; `--fix` auto-repairs |
| `agent-workflow [-t TYPE] -d DESC` | Execute autonomous Git branch & commit workflow with protection guards |

### Scenario Examples

#### Managing MCP Servers by Target
```bash
# Configure a project-level MCP server (.mcp.json)
./target/release/claude-code-setup mcp-set github --command "npx" --arg "-y" --arg "@modelcontextprotocol/server-github" --env "GITHUB_TOKEN=ghp_example" --target project

# Disable a server in Claude Desktop config
./target/release/claude-code-setup mcp-disable github --target claude-desktop

# Completely remove a server (--remove flag required)
./target/release/claude-code-setup mcp-unset github --remove --target claude-code
```

#### Adding Memory Notes & Hybrid Search
```bash
# Add a new note
./target/release/claude-code-setup memory-note "Architecture Decisions" --body "Completed 100% Rust binary refactoring."

# Index knowledge notes
./target/release/claude-code-setup memory-index --edge-threshold 0.70

# Execute RRF Hybrid Search
./target/release/claude-code-setup memory-search "Rust architecture" --mode hybrid --limit 5

# Query related notes via graph traversal
./target/release/claude-code-setup memory-related architecture-decisions.md
```

---

## 🛡️ 5. Quality Gates & Testing

The project includes 24 unit tests, all currently passing:

```bash
cargo test
```

### Quality Standards
- **Unit Tests (24/24 Passed):** Multi-target MCP management, Value preservation, FTS5 query escaping, RRF hybrid ranking, mean-pooling, wikilink extraction, secret auditing, and protected branch guardrails.
- **Formatting:** Enforced via `cargo fmt --check`
- **Continuous Integration (CI):** Verified on Ubuntu, macOS, and Windows via `.github/workflows/rust.yml` and `.github/workflows/release.yml`.

---

## 👥 6. Contributors

| Contributor | Role / Responsibility | Metrics |
| :--- | :--- | :--- |
| **Ercan ER** | Lead Architect, Rust Migration & Primary Developer | 26 commits |
| **Kassam** | Autonomous AI Agent, Rust Engine & Module Developer | Co-author / Contributor |
| **Copilot** | AI Coding Assistant | 4 commits |
| **jb_remus** | Original Upstream Author | 2 commits |
| **Mihenk** | Code Auditor & Reviewer | 1 commit |
| **arturo-ebuck** | Open Source Contributor | 1 commit |

---

## 📄 7. License & Resources

Distributed under the [MIT License](LICENSE).

### Related Documentation
- [Deployment Guide](DEPLOYMENT_GUIDE.md)
- [Manual Setup Guide](docs/MANUAL_SETUP.md)
- [Troubleshooting Guide](docs/TROUBLESHOOTING.md)
- [Developer Directives](docs/dev/TASK-KASSAM-1-2.md)
