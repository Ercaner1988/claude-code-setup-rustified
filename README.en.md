**🌍 [Türkçe](README.md) | [English](README.en.md) | [العربية](README.ar.md) | [日本語](README.ja.md) | [中文](README.zh.md) | [Русский](README.ru.md) | [Español](README.es.md)**

# Claude Code Standalone Setup (Rust-Core Single Binary)

[![Rust](https://img.shields.io/badge/Rust%20core-%2591-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/Tests-30%20Passed-green.svg)]()

A local deployment, security auditing, and memory engine (`claude-code-setup`) for managing the **Claude Code** environment. The runtime is a single Rust binary; running it requires no Rust, Python, or Node installation on the host machine.

### Honesty note: this repository is not 100% Rust

GitHub language statistics and measured codebase distribution (2026-09-02): **2891 lines of Rust / 279 lines of non-Rust = 91.2% line ratio** (GitHub Linguist: **90.5% Rust, 3.5% Shell, 3.2% Python, 2.8% PowerShell**).

| Language / File | Lines | GitHub Share | When it runs |
| :--- | ---: | ---: | :--- |
| `Rust` (`src/*.rs`, 10 files) | 2891 | 90.5% | Runtime execution (CLI + MCP server) |
| `install-macos.sh` (Shell / Bash) | 121 | 3.5% | During Linux/macOS installation |
| `package-extension.py` (Python) | 87 | 3.2% | Release publishing / `.mcpb` packaging only (CI) |
| `install-windows.ps1` (PowerShell) | 71 | 2.8% | During Windows installation |

Additional dependencies:
- The pre-commit hook in `src/security.rs` is written out as an **embedded bash script** (`#!/usr/bin/env bash`) — the hook needs Git's bash to run.
- The `.github/workflows/release.yml` pipeline uses `actions/setup-python` and `npx @anthropic-ai/mcpb validate` → the **CI chain depends on Python and Node**.
- The memory engine's embedding layer downloads **ONNX Runtime's pre-compiled C++ binary** through `fastembed` (`ort-download-binaries`).

Accurate summary: **the runtime binary is pure Rust; installation, packaging, and the release pipeline use Bash + PowerShell + Python + Node.**

---

## 🎯 1. Destination and Completed Work

- **Single-Binary Runtime:** Legacy Bash and Python *runtime* scripts were migrated to Rust. Installation and packaging scripts (`install-*.{sh,ps1}`, `package-extension.py`) were deliberately kept, because the installer itself must run before the binary is downloaded.
- **Dynamic Path Normalization:** Hardcoded path definitions (e.g. `/home/jb_remus`) adapt dynamically to the target operating system and the local user home directory.
- **Multi-Target MCP Management (`--target`):**
  - Manage **Claude Code** (`~/.claude.json`), **Project** (`./.mcp.json`), and **Claude Desktop** (`claude_desktop_config.json`) configurations from a single CLI.
  - An atomic writer that preserves untyped JSON fields via `serde_json::Value` (with automatic `.bak` backup).
- **MCP Server Mode (`--mcp-mode`):** The same binary turns into an MCP server speaking JSON-RPC over stdin/stdout, exposing the 8 tools declared in `manifest.json` to Claude Desktop.
- **Memory Engine (SQLite + Vector + Graph):**
  - **Quick Note Creation (`memory-note`):** Safe note creation with kebab-case file names.
  - **FTS5 Keyword Search:** SQLite keyword indexing with quote-escaping.
  - **Local Embedding:** Cosine similarity via `fastembed` (Multilingual-E5-Small). The model is downloaded from Hugging Face on first use and cached under `$HOME/.claude/fastembed_cache`; **after that first download**, search works fully offline.
  - **Graph Edges and Wikilinks:** Neighbourhood search (`memory-related`) over `[[Note-Name]]` links and above-threshold semantic edges.
  - **RRF Hybrid Ranking:** Reciprocal Rank Fusion (`k=60`) fusing FTS5 and vector results.
- **Auto-Fixing Security Audit (`security-audit --fix`):**
  - Plaintext secret scanning in configuration files (`ghp_`, `github_pat_`, `sk-`, `xox[baprs]-`, `AKIA`).
  - Tightening file permissions to 600 — **on Unix only**; on Windows an informational note about ACL-based permissions is printed and no fix is applied.
  - Installation of the Git pre-commit branch-protection and secret-scanning hook.
- **Autonomous Git Workflow (`agent-workflow`):**
  - Automatic feature-branch creation from the remote default branch.
  - Blocking direct pushes to protected main branches.

---

## 🏗️ 2. Architecture and Modules

```
claude-code-setup-rustified/
├── Cargo.toml                  # Rust dependencies and package definitions (v0.1.6)
├── manifest.json               # Claude Desktop extension manifest (8 MCP tools)
├── icon.png                    # Extension icon
├── .env.example                # Sample environment variables
├── src/
│   ├── main.rs                 # CLI entry point and command dispatcher (123 lines)
│   ├── cli.rs                  # Clap-based command, target, and flag definitions (222)
│   ├── mcp.rs                  # Multi-target, JSON Value-preserving MCP manager (488)
│   ├── mcp_server.rs           # MCP stdio JSON-RPC server; maps 8 tools to the CLI (436)
│   ├── memory_engine.rs        # FTS5 + Vector + Graph + RRF + memory-note engine (821)
│   ├── installer.rs            # Skeleton directory, seed README, and .env installer (191)
│   ├── security.rs             # Auto-fixing security auditor and hook manager (296)
│   ├── branch_manager.rs       # Autonomous Git workflow with protected-branch guards (161)
│   ├── tester.rs               # System and environment diagnostic test runner (123)
│   └── agent.rs                # Agent integration interface (30)
├── install-windows.ps1         # PowerShell installer (NOT Rust)
├── install-macos.sh            # Bash installer (NOT Rust)
├── package-extension.py        # .mcpb packager, invoked in CI (NOT Rust)
├── .github/workflows/
│   ├── rust.yml                # fmt + clippy + test + build (ubuntu/windows/macos)
│   └── release.yml             # 3-platform binary + .mcpb release pipeline
└── docs/                       # Installation and troubleshooting guides
```

### Module Responsibilities
- `src/main.rs`: Parses command-line arguments; hands control to the MCP server when `--mcp-mode` is given, otherwise to the relevant module function.
- `src/cli.rs`: Manages 15 subcommands, flags (`--target`, `--fix`, `--hooks`, `--mode`, `--min-score`), and the global `--mcp-mode` flag through a Clap `Parser` struct.
- `src/mcp.rs`: Reads and updates MCP settings according to `--target` (`claude-code`, `project`, `claude-desktop`); writes atomically without dropping unknown fields.
- `src/mcp_server.rs`: Runs the stdin/stdout JSON-RPC loop; maps the 8 tools in `manifest.json` (`mcp_list`, `mcp_add`, `security_audit`, `memory_note`, `memory_index`, `memory_search`, `status`, `test`) onto real CLI commands. This mapping is locked by the `her_arac_gercek_bir_cli_komutuna_esleniyor` test.
- `src/memory_engine.rs`: Embeds notes in ~1500-character windows and mean-pools them; manages the `knowledge_notes` and `note_edges` SQLite tables. Embedding cache: `$HOME/.claude/fastembed_cache`.
- `src/installer.rs`: Creates the `$HOME/claude_global_memory/knowledge` directory and the seed `README.md` without ever overwriting them; copies `.env` if missing.
- `src/security.rs`: Scans for plaintext secrets, checks permissions, fixes them with `--fix`, and installs the Git hook (the hook is an embedded bash script).
- `src/branch_manager.rs`: Handles autonomous branch creation, the protected-branch guard, and safe commit/push flows.
- `src/tester.rs`: Performs system diagnostics (`status`) and test verification.

---

## 🚀 3. Installation and Configuration

### Quick Start

There are two distinct installations; decide which one you want.

**Claude Desktop extension (recommended)** — download the package matching your operating system from the [latest release](https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest) and drag it into Claude Desktop → Settings → Extensions:

| Operating system | File | Approx. size |
|---|---|---|
| Windows | `claude-code-setup-windows.mcpb` | 9 MB |
| macOS | `claude-code-setup-macos.mcpb` | 10 MB |
| Linux | `claude-code-setup-linux.mcpb` | 12 MB |

**Command-line tool** — if you want to use it from a terminal:

```powershell
irm https://raw.githubusercontent.com/Ercaner1988/claude-code-setup-rustified/main/install-windows.ps1 | iex
```

```bash
curl -fsSL https://raw.githubusercontent.com/Ercaner1988/claude-code-setup-rustified/main/install-macos.sh | bash
```

These installers are PowerShell and Bash scripts (not Rust); they install the downloaded binary into your user directory and add it to PATH (no administrator rights needed). They do **not** register the extension — use the `.mcpb` route above for that. To verify, run `claude-code-setup status` in a new terminal.

For detailed installation, see [INSTALLATION.en.md](INSTALLATION.en.md)

---

### Manual Installation: Building from Source

#### Requirements
- **Rust Toolchain:** `rustc` and `cargo` (1.80+)
- On the first build, `fastembed` downloads the ONNX Runtime binary → network access is required.

#### Build
```bash
cargo build --release

# Resulting binary:
# Windows: ./target/release/claude-code-setup.exe
# Linux/macOS: ./target/release/claude-code-setup
```

### Automatic Setup and Environment Diagnostics
```bash
# Checks prerequisites and installs the memory skeleton
./target/release/claude-code-setup install --hooks

# System and environment diagnostic status
./target/release/claude-code-setup status
```

---

## 📖 4. Usage and Examples

### Command Summary Table

| Command | Description |
| :--- | :--- |
| `--mcp-mode` (global flag) | Runs the binary as an MCP server speaking JSON-RPC over stdin/stdout |
| `install [--hooks] [--skip-prereqs]` | Environment setup, memory skeleton, and `.env` copying |
| `test` / `status` | Diagnostics for Claude CLI, `.claude.json`, memory DB, and hooks |
| `mcp-list [--target T]` | Lists configured MCP servers for the given target |
| `mcp-set <srv> [--command C] [--arg A]… [--env K=V]… [--target T]` | Adds or updates an MCP server (`--target`: `claude-code`, `project`, `claude-desktop`) |
| `mcp-unset <srv> [--env K]… [--clear-args] [--remove] [--target T]` | Removes variables or the server entirely (`--remove` is required) |
| `mcp-enable <srv>` / `mcp-disable <srv>` | Enables/disables a server without breaking the configuration |
| `memory-note <title> [--body ...] [--dir D]` | Adds a new Markdown note to the knowledge base |
| `memory-index [--source DIR]… [--edge-threshold 0.70]` | Indexes notes into the SQLite + Vector + Graph engine |
| `memory-search <query> [--mode keyword\|semantic\|hybrid] [--limit 5] [--min-score 0.30]` | Searches memory in FTS5 keyword, vector, or RRF hybrid mode |
| `memory-related <note.md>` | Lists related notes via graph edges and wikilinks |
| `install-hooks [--repo-dir PATH]` | Installs the pre-commit security hook into a repository |
| `security-audit [--fix]` | Runs a security audit; applies auto-fixes with `--fix` |
| `agent-workflow [--branch-type TYPE] --description DESC [--files F]…` | Runs the autonomous Git branch and commit workflow with protected-branch guards |

All commands accept a `--home-dir` override for test isolation (except `install-hooks` and `agent-workflow`).

### Example Scenarios

#### Managing MCP Servers per Target
```bash
# Define an MCP server at project level (.mcp.json)
./target/release/claude-code-setup mcp-set github \
  --command "npx" --arg "-y" --arg "@modelcontextprotocol/server-github" \
  --env "GITHUB_TOKEN=$GITHUB_TOKEN" --target project

# Disable the server in the Claude Desktop configuration
./target/release/claude-code-setup mcp-disable github --target claude-desktop

# Remove the server entirely (the --remove flag is mandatory)
./target/release/claude-code-setup mcp-unset github --remove --target claude-code
```

#### Adding Notes and RRF Hybrid Search
```bash
./target/release/claude-code-setup memory-note "Architecture Decisions" --body "Runtime migration to a native Rust binary is complete."
./target/release/claude-code-setup memory-index --edge-threshold 0.70
./target/release/claude-code-setup memory-search "Rust architecture" --mode hybrid --limit 5 --min-score 0.30
./target/release/claude-code-setup memory-related architecture-decisions.md
```

---

## 🛡️ 5. Tests and Quality Gates

```bash
cargo test
# running 30 tests
# test result: ok. 30 passed; 0 failed; 0 ignored
```

**31 tests** are defined in the source; one of them (`test_enforce_file_permissions_fixes_mode`) is marked `#[cfg(unix)]` and therefore does not compile on Windows. Measured: **30/30 green on Windows, 31/31 on Unix** (2026-09-02).

Per-file breakdown: `memory_engine.rs` 14, `mcp.rs` 5, `mcp_server.rs` 5, `security.rs` 3, `branch_manager.rs` 2, `installer.rs` 2.

### Quality Standards
- **Coverage:** Multi-target MCP management, JSON `Value` preservation, FTS5 character escaping, RRF hybrid ranking, mean-pooling, wikilink parsing, embedding cache path regression, secret scanning, MCP tool-to-CLI mapping, and protected-branch guards.
- **Formatting:** `cargo fmt --all -- --check` → clean (2026-09-02).
- **Lint:** `cargo clippy --all-targets -- -D warnings` → no warnings (2026-09-02).
- **Continuous Integration:** `.github/workflows/rust.yml` runs fmt + clippy + test + release build on three operating systems (ubuntu, windows, macos). `.github/workflows/release.yml` produces the three platform binaries and the `.mcpb` packages; that pipeline uses Python and Node.

---

## 👥 6. Contributors

The numbers below were measured with `git shortlog -sne --all` and by counting `Co-authored-by` trailers in commit bodies (2026-09-02, 45 commits total).

| Contributor | Role / Responsibility | Measured contribution |
| :--- | :--- | :--- |
| **Ercan ER** | Project architecture, Rust migration, lead developer | 41 commits (author) |
| **Claude Opus 5** | Autonomous AI agent, module development | 14 commits (co-author) |
| **Copilot App** | AI coding assistant | 11 commits (co-author) |
| **Claude Opus 4.8** | Autonomous AI agent | 3 commits (co-author) |
| **Claude** (version unspecified) | Autonomous AI agent | 2 commits (co-author) |
| **jb_remus** | Original upstream author | 2 commits (author) |
| **Mihenk** | Code reviewer and quality referee | 1 commit (author) |
| **arturo-ebuck** | Open-source contributor | 1 commit (author) |

**Kassam** is the agent identity recorded in the `authors` field of `Cargo.toml`; it has no separate Git author record.

---

## 📄 7. License and Resources

This project is licensed under the [MIT License](LICENSE) (Copyright © 2026 Ercan Er).

### Related Documents
- [Deployment Guide](DEPLOYMENT_GUIDE.md)
- [Manual Setup Guide](docs/MANUAL_SETUP.md)
- [Troubleshooting Guide](docs/TROUBLESHOOTING.md)
- [Developer Directives](docs/dev/TASK-KASSAM-1-2.md)
