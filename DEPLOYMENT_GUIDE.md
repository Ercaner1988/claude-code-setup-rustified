# Deployment Guide

This guide deploys the Rust CLI onto a fresh machine (Windows, Linux, or macOS).

## Quick Start: Claude Code Extension

### Windows (Automatic)
```powershell
powershell -ExecutionPolicy Bypass -File install-windows.ps1
```

### macOS (Automatic)
```bash
bash install-macos.sh
```

These scripts handle:
- ✅ Downloading latest release
- ✅ Installing to system PATH
- ✅ Registering as Claude Code Extension
- ✅ Configuring MCP servers

Then:
1. Open Claude Code Desktop → Settings → Extensions
2. Find "claude-code-setup" in the list
3. Click "Configure" to see available tools

---

## Manual Extension Installation

1. **Clone or download the repository**
2. **Build release binary:**
   ```bash
   cargo build --release
   ```
3. **Create extension folder:**
   ```
   $APPDATA\Claude\Claude Extensions\ercaner1988.claude-code-setup
   ```
4. **Copy files:**
   - `manifest.json` ← Deploy this
   - `target/release/claude-code-setup.exe` (or the binary for your OS)
   - `README.md`
   - `LICENSE`
5. **Register:**
   - Claude Code Desktop → Settings → Extensions
   - "Select extension" → "Browse extensions folder"
   - Choose the `ercaner1988.claude-code-setup` folder
   - Click "Install unpacked extension"
6. **Restart** Claude Code Desktop

---

## Prerequisites (For Manual Build)

| Tool | Windows | Linux / macOS |
| :--- | :--- | :--- |
| Rust toolchain (1.80+) | [rustup](https://rustup.rs) | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Git | [git-scm.com](https://git-scm.com) | distro package manager |
| Claude Code CLI | `npm install -g @anthropic-ai/claude-code` (needs Node.js) | same |

`install` checks all of these and reports anything missing.

## Deploy from Source

```bash
git clone https://github.com/Ercaner1988/claude-code-setup-rustified.git
cd claude-code-setup-rustified
cargo build --release

# Verify prerequisites, create the memory knowledge base skeleton and .env
./target/release/claude-code-setup install --hooks

# Verify the environment
./target/release/claude-code-setup status
```

The binary is self-contained; copy `target/release/claude-code-setup` (`.exe` on Windows) anywhere on your PATH for convenient access.

The embedding model (~100 MB) is downloaded once on the first `memory-index` or `memory-search` run and cached locally afterwards.

## What Gets Configured

- `~/claude_global_memory/knowledge/` — knowledge base skeleton with a seed README (existing content is never touched)
- `.env` in the repo — created from `.env.example` only if missing; fill in your API keys
- Pre-commit security hook in the current repository (with `--hooks`)

Nothing under `~/.claude/` is ever overwritten by `install`.

## Day-to-day Operations

```bash
# Manage MCP servers (Claude Code user config by default)
claude-code-setup mcp-list
claude-code-setup mcp-set my-server --command npx --arg "-y" --arg "some-mcp" --env KEY=VALUE
claude-code-setup mcp-disable my-server
claude-code-setup mcp-unset my-server --remove          # deletion requires --remove

# Project-scoped servers (./.mcp.json) or Claude Desktop
claude-code-setup mcp-list --target project
claude-code-setup mcp-list --target claude-desktop

# Memory
claude-code-setup memory-note "My finding" --body "Details..."
claude-code-setup memory-index
claude-code-setup memory-search "how do hooks work"
claude-code-setup memory-related my-finding.md

# Security
claude-code-setup security-audit --fix
```

## Updating

```bash
git pull
cargo build --release
```

## Verification Checklist

- `status` reports Claude CLI installed and `~/.claude.json` parsed with your MCP servers
- `mcp-list` shows your servers
- `memory-search` returns results after a note has been added and indexed
- `security-audit` reports no findings

See [docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) if anything reports red.
