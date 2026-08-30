# Deployment Guide

This guide deploys the Rust CLI onto a fresh machine (Windows, Linux, or macOS).

## Prerequisites

| Tool | Windows | Linux / macOS |
| :--- | :--- | :--- |
| Rust toolchain (1.80+) | [rustup](https://rustup.rs) | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| Git | [git-scm.com](https://git-scm.com) | distro package manager |
| Claude Code CLI | `npm install -g @anthropic-ai/claude-code` (needs Node.js) | same |

`install` checks all of these and reports anything missing.

## Deploy

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
