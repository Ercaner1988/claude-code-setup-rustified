# Claude Code Setup — Rustified

A single-binary, **100% Rust** CLI for managing your **Claude Code** environment: dynamic MCP server management, a local semantic + graph memory engine, security auditing, and safe Git workflows.

Türkçe: [README.tr.md](README.tr.md) · العربية: [README.ar.md](README.ar.md)

## Features

1. **Dynamic MCP management** — manage MCP servers across **Claude Code** (`~/.claude.json`), **project** (`.mcp.json`) and **Claude Desktop** configs with `--target`; edits preserve unknown fields, write atomically, and keep a `.bak` backup.
2. **Local memory engine** — indexes Markdown notes into SQLite with FTS5 keyword search, local embeddings (Multilingual-E5-Small via fastembed, fully offline), RRF hybrid ranking, and a wikilink + semantic similarity graph (`memory-related`).
3. **Security audit with auto-fix** — scans configs for plaintext tokens, enforces file permissions on Unix, and installs pre-commit branch-protection/secret-scanning hooks (`security-audit --fix`).
4. **Safe autonomous Git workflow** — `agent-workflow` creates feature branches from the remote default branch and refuses to push to protected branches; all git failures surface as errors.

## Building & Usage

Requires a Rust toolchain (1.80+). Works on Windows, Linux and macOS.

```bash
# Build binary
cargo build --release

# Verify prerequisites and set up memory skeleton + .env
./target/release/claude-code-setup install

# Run environment diagnostics
./target/release/claude-code-setup status
```

## Commands

| Command | Description |
| :--- | :--- |
| `install [--hooks]` | Verify prerequisites, create `~/claude_global_memory/knowledge` (seed README, never overwrites), create `.env` from `.env.example` if missing |
| `test` / `status` | Environment diagnostics: Claude CLI, `~/.claude.json`, memory DB, model cache, hooks, env vars |
| `mcp-list [--target T]` | List configured MCP servers |
| `mcp-set <srv> [--command X] [--arg A]... [--env K=V]... [--target T]` | Create/update an MCP server entry |
| `mcp-unset <srv> [--env K]... [--clear-args] [--remove] [--target T]` | Remove fields; `--remove` required to delete a server |
| `mcp-enable <srv>` / `mcp-disable <srv> [--target T]` | Toggle a server without deleting its config |
| `memory-note <title> [--body ...]` | Add a note to the knowledge base (kebab-case filename, never overwrites) |
| `memory-index [--source DIR]... [--edge-threshold 0.70]` | Index notes into SQLite (embeddings + graph edges) |
| `memory-search <query> [--mode keyword\|semantic\|hybrid] [--limit 5] [--min-score 0.30]` | Search indexed notes (default: hybrid RRF) |
| `memory-related <note.md>` | Show related notes via graph edges (BFS, 2 hops) |
| `install-hooks [--repo-dir PATH]` | Install pre-commit security hook into a repo |
| `security-audit [--fix]` | Secret scan, permission check (Unix), hook check, branch check |
| `agent-workflow [-t TYPE] -d DESC [-f FILE]...` | Create feature branch, commit files, push — with protected-branch guardrails |

`--target` values: `claude-code` (default, `~/.claude.json`), `project` (`./.mcp.json`), `claude-desktop` (`claude_desktop_config.json`).

## Memory Engine Notes

- Default knowledge directory: `~/claude_global_memory/knowledge` (created by `install`; add notes with `memory-note`).
- The embedding model (~100 MB) downloads on first `memory-index`/`memory-search` and is cached locally; everything runs offline afterwards.
- Linear cosine search is intentional at this scale; the code marks where an ANN index would go if the note count grows into the thousands.

## Security

- No secrets in the repository; `.env` is git-ignored and never overwritten.
- Every config write is atomic (temp + rename) and leaves a `.bak` backup.
- `mcp-unset <srv>` without flags refuses to act — destructive deletion requires `--remove`.
