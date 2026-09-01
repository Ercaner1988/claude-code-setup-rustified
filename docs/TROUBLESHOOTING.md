# Troubleshooting Guide

## `claude` command not found

Reinstall the CLI and make sure the npm global bin directory is on your PATH:

```bash
npm install -g @anthropic-ai/claude-code
```

## Claude Desktop, kurduğum eklenti için "bu paket bu platform için değil" diyor

Her `.mcpb` paketi **tek bir işletim sistemi** içindir; içinde yalnızca o
sistemin ikili dosyası vardır. Yanlış paketi kurarsan Claude Desktop uyarı
verir ve eklenti çalışmaz.

Son sürümden işletim sistemine uyanı indir:

| İşletim sistemi | Dosya |
|---|---|
| Windows (32 ve 64 bit fark etmez) | `claude-code-setup-windows.mcpb` |
| macOS | `claude-code-setup-macos.mcpb` |
| Linux | `claude-code-setup-linux.mcpb` |

Eski sürümlerde Windows paketinin adında `win32` geçiyordu. Bu "32 bit"
demek değil — `win32`, Windows'un platform kimliğidir ve 64 bit makineler
için de doğru dosyadır. Kafa karışıklığını önlemek için adlandırma
`windows` olarak değiştirildi.

Yanlış paketi kurduysan: Claude Desktop → Settings → Extensions'tan kaldır,
doğru dosyayı sürükle.

## `memory-search` says "Memory database not found"

Run `memory-index` first. The database lives at `~/.claude/memory_index.db`. If your notes are not in the default `~/claude_global_memory/knowledge` directory, pass `--source`:

```bash
claude-code-setup memory-index --source path/to/notes
```

## First `memory-index` / `memory-search` is slow or downloads something

The Multilingual-E5-Small embedding model (~100 MB) and the ONNX runtime are downloaded once and cached under `~/.cache`. Subsequent runs are offline. A working internet connection is required for the first run.

## `mcp-set` wrote to a config I didn't expect

The default target is Claude Code's user config (`~/.claude.json`). Use `--target project` for `./.mcp.json` or `--target claude-desktop` for the Claude Desktop config. `mcp-list` prints the exact path it operates on. Every write leaves a `.bak` backup next to the config file — restore from it if needed.

## Search returns no results for queries with `-`, `:`, quotes or `*`

These characters are FTS5 syntax. The CLI escapes each word automatically, but if results look wrong, try `--mode semantic` or simpler wording.

## Pre-commit hook does not run on Windows

The hook is a Bash script and requires Git Bash (bundled with Git for Windows). If hooks don't fire, verify `git config core.hooksPath` is unset and `.git/hooks/pre-commit` exists.

## `agent-workflow` fails with "git fetch origin failed"

The command requires a remote named `origin`. Add one (`git remote add origin <url>`) or create the branch manually.

## Build errors on Windows

Install the "Desktop development with C++" workload from Visual Studio Build Tools — the bundled SQLite and ONNX runtime need a C linker.
