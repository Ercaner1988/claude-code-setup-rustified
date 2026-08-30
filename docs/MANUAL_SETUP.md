# Manual Setup Guide

The Rust CLI automates everything below, but you can replicate each step by hand.

## 1. Prerequisites

- **Rust toolchain** via [rustup](https://rustup.rs)
- **Git**
- **Node.js** (for the Claude Code CLI and npm-based MCP servers)
- **Claude Code CLI**: `npm install -g @anthropic-ai/claude-code`

## 2. Knowledge Base

```bash
# Windows (PowerShell)
mkdir "$HOME\claude_global_memory\knowledge"

# Linux / macOS
mkdir -p ~/claude_global_memory/knowledge
```

Add Markdown notes here (or via `claude-code-setup memory-note`). `[[Wikilinks]]` between notes become graph edges when indexed.

## 3. MCP Servers

Edit one of these files directly, or use `mcp-set`/`mcp-unset`:

| Target | File |
| :--- | :--- |
| Claude Code (user) | `~/.claude.json` → top-level `mcpServers` object |
| Claude Code (project) | `<repo>/.mcp.json` → top-level `mcpServers` object |
| Claude Desktop | `%APPDATA%\Claude\claude_desktop_config.json` (Windows), `~/.config/claude-code/claude_desktop_config.json` (Linux) |

Entry shape:

```json
{
  "mcpServers": {
    "my-server": {
      "command": "npx",
      "args": ["-y", "some-mcp-package"],
      "env": { "API_KEY": "..." }
    }
  }
}
```

Prefer environment variables over plaintext keys — `security-audit` flags known token patterns.

## 4. Environment Variables

Copy `.env.example` to `.env` and fill in the keys you use. Load it from your shell profile, e.g. on Linux/macOS:

```bash
echo '[[ -f ~/.env.claude ]] && source ~/.env.claude' >> ~/.bashrc
```

## 5. Git Hooks

`claude-code-setup install-hooks` writes a pre-commit hook that blocks direct commits to `main`/`master` and scans staged diffs for hardcoded secrets. Equivalent manual step: copy the hook content into `.git/hooks/pre-commit` and make it executable.

## 6. Verify

```bash
claude --version
claude-code-setup status
```
