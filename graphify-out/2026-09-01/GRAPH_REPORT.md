# Graph Report - .  (2026-09-01)

## Corpus Check
- Corpus is ~13,276 words - fits in a single context window. You may not need a graph.

## Summary
- 273 nodes · 458 edges · 19 communities (16 shown, 3 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 8 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- [[_COMMUNITY_macOS Kurucu|macOS Kurucu]]
- [[_COMMUNITY_Eklenti Manifesti|Eklenti Manifesti]]
- [[_COMMUNITY_Eklenti Paketleme|Eklenti Paketleme]]
- [[_COMMUNITY_Ajan Is Akisi|Ajan Is Akisi]]
- [[_COMMUNITY_Git Dal Yonetimi|Git Dal Yonetimi]]
- [[_COMMUNITY_CLI Komut Tanimlari|CLI Komut Tanimlari]]
- [[_COMMUNITY_Kurulum ve Tanilama|Kurulum ve Tanilama]]
- [[_COMMUNITY_Giris Noktasi|Giris Noktasi]]
- [[_COMMUNITY_MCP Yapilandirma Yonetimi|MCP Yapilandirma Yonetimi]]
- [[_COMMUNITY_MCP Sunucu Protokolu|MCP Sunucu Protokolu]]
- [[_COMMUNITY_Bellek ve Gomu Motoru|Bellek ve Gomu Motoru]]
- [[_COMMUNITY_Guvenlik Denetimi|Guvenlik Denetimi]]
- [[_COMMUNITY_Dagitim Kilavuzu|Dagitim Kilavuzu]]
- [[_COMMUNITY_Kurulum Kilavuzu|Kurulum Kilavuzu]]
- [[_COMMUNITY_README Genel Tanitim|README Genel Tanitim]]
- [[_COMMUNITY_Elle Kurulum Kilavuzu|Elle Kurulum Kilavuzu]]
- [[_COMMUNITY_Sorun Giderme Kilavuzu|Sorun Giderme Kilavuzu]]
- [[_COMMUNITY_Gorev Direktifi|Gorev Direktifi]]

## God Nodes (most connected - your core abstractions)
1. `index_memory()` - 13 edges
2. `get_home_dir()` - 12 edges
3. `resolve_config_path()` - 12 edges
4. `Result` - 12 edges
5. `run_install()` - 11 edges
6. `mcp_set()` - 11 edges
7. `mcp_unset()` - 11 edges
8. `get_db_path()` - 10 edges
9. `String` - 10 edges
10. `add_memory_note()` - 10 edges

## Surprising Connections (you probably didn't know these)
- `sanitize_description()` --calls--> `add_memory_note()`  [INFERRED]
  src/branch_manager.rs → src/memory_engine.rs
- `get_home_dir()` --calls--> `resolve_config_path()`  [INFERRED]
  src/installer.rs → src/mcp.rs
- `get_home_dir()` --calls--> `default_knowledge_dir()`  [INFERRED]
  src/installer.rs → src/memory_engine.rs
- `get_home_dir()` --calls--> `embedding_cache_dir()`  [INFERRED]
  src/installer.rs → src/memory_engine.rs
- `get_home_dir()` --calls--> `get_db_path()`  [INFERRED]
  src/installer.rs → src/memory_engine.rs

## Import Cycles
- 1-file cycle: `src/agent.rs -> src/agent.rs`
- 1-file cycle: `src/installer.rs -> src/installer.rs`
- 1-file cycle: `src/main.rs -> src/main.rs`
- 1-file cycle: `src/mcp.rs -> src/mcp.rs`
- 1-file cycle: `src/memory_engine.rs -> src/memory_engine.rs`
- 1-file cycle: `src/tester.rs -> src/tester.rs`
- 1-file cycle: `src/security.rs -> src/security.rs`

## Communities (19 total, 3 thin omitted)

### Community 1 - "Eklenti Manifesti"
Cohesion: 0.05
Nodes (36): manifest_version, name, display_name, version, description, author, name, url (+28 more)

### Community 14 - "Ajan Is Akisi"
Cohesion: 0.67
Nodes (3): run_agent_workflow(), String, Result

### Community 7 - "Git Dal Yonetimi"
Cohesion: 0.36
Nodes (12): is_protected_branch(), sanitize_description(), String, run_git(), Result, git_stdout(), get_current_branch(), default_remote_branch() (+4 more)

### Community 13 - "CLI Komut Tanimlari"
Cohesion: 0.40
Nodes (4): Cli, Option, Commands, Commands

### Community 3 - "Kurulum ve Tanilama"
Cohesion: 0.17
Nodes (17): get_home_dir(), Option, String, Result, PathBuf, log_info(), log_success(), log_warning() (+9 more)

### Community 4 - "MCP Yapilandirma Yonetimi"
Cohesion: 0.29
Nodes (20): McpTarget, resolve_config_path(), Option, String, Result, PathBuf, read_json_value(), Path (+12 more)

### Community 5 - "MCP Sunucu Protokolu"
Cohesion: 0.23
Nodes (18): JsonRpcRequest, Value, String, run_mcp_mode(), Result, handle_request(), initialize(), list_tools() (+10 more)

### Community 0 - "Bellek ve Gomu Motoru"
Cohesion: 0.13
Nodes (40): embedding_cache_dir(), Result, PathBuf, new_embedding_model(), TextEmbedding, get_db_path(), Option, String (+32 more)

### Community 8 - "Guvenlik Denetimi"
Cohesion: 0.29
Nodes (12): find_secrets(), Vec, String, install_git_hooks(), Option, Result, enforce_file_permissions(), Path (+4 more)

### Community 10 - "Dagitim Kilavuzu"
Cohesion: 0.17
Nodes (11): Deployment Guide, Quick Start: Claude Code Extension, Windows (Automatic), macOS (Automatic), Manual Extension Installation, Prerequisites (For Manual Build), Deploy from Source, What Gets Configured (+3 more)

### Community 6 - "Kurulum Kilavuzu"
Cohesion: 0.11
Nodes (17): Claude Code Bağımsız Kurulum Kılavuzu (%100 Rust Motoru), 🚀 HIZLI BAŞLANGIÇ: Claude Code Extension, En Kolay Yol - Extension Installer, Windows (PowerShell), macOS (Terminal), 📥 Manual Extension Kurulumu (İsteğe Bağlı), 💡 ÖNEMLİ NOT: Rust Kurulu Olması Gerekir mi?, 🛠️ 1. Ön Gereksinimler (İsteğe Bağlı & Otomatik Kurulumlar) (+9 more)

### Community 2 - "README Genel Tanitim"
Cohesion: 0.09
Nodes (22): Claude Code Bağımsız Kurulum (%100 Rust Motoru), 🎯 1. Varış Noktası ve Tamamlananlar, 🏗️ 2. Mimari ve Modüller, Modül Sorumlulukları, 🚀 3. Kurulum ve Yapılandırma, Hızlı Başlangıç: Claude Code Extension, Windows x64, macOS x64 (+14 more)

### Community 12 - "Elle Kurulum Kilavuzu"
Cohesion: 0.25
Nodes (7): Manual Setup Guide, 1. Prerequisites, 2. Knowledge Base, 3. MCP Servers, 4. Environment Variables, 5. Git Hooks, 6. Verify

### Community 11 - "Sorun Giderme Kilavuzu"
Cohesion: 0.20
Nodes (9): Troubleshooting Guide, `claude` command not found, `memory-search` says "Memory database not found", First `memory-index` / `memory-search` is slow or downloads something, `mcp-set` wrote to a config I didn't expect, Search returns no results for queries with `-`, `:`, quotes or `*`, Pre-commit hook does not run on Windows, `agent-workflow` fails with "git fetch origin failed" (+1 more)

### Community 9 - "Gorev Direktifi"
Cohesion: 0.15
Nodes (12): Kassam Görev Direktifi — Özellik 1 (Dinamik MCP) + Özellik 2 (Semantik + Graph Memory), ÖNCE ÇÖZ (kod yazmadan yanıtla), Kurallar (ponytail — inceleme bunları zorlayacak), Özellik 2 — Semantik + Graph Memory, Özellik 1 — Dinamik MCP Parametre Yönetimi, Teslim, TUR 2 — Denetim sonrası düzeltme + kalite, ÖNCE YANITLA (kod yazmadan) (+4 more)

## Knowledge Gaps
- **100 isolated node(s):** `install-macos.sh script`, `manifest_version`, `name`, `display_name`, `version` (+95 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **3 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `get_home_dir()` connect `Kurulum ve Tanilama` to `Bellek ve Gomu Motoru`, `Guvenlik Denetimi`, `MCP Yapilandirma Yonetimi`?**
  _High betweenness centrality (0.109) - this node is a cross-community bridge._
- **Why does `resolve_config_path()` connect `MCP Yapilandirma Yonetimi` to `Kurulum ve Tanilama`?**
  _High betweenness centrality (0.051) - this node is a cross-community bridge._
- **Why does `add_memory_note()` connect `Bellek ve Gomu Motoru` to `Git Dal Yonetimi`?**
  _High betweenness centrality (0.041) - this node is a cross-community bridge._
- **Are the 6 inferred relationships involving `get_home_dir()` (e.g. with `resolve_config_path()` and `default_knowledge_dir()`) actually correct?**
  _`get_home_dir()` has 6 INFERRED edges - model-reasoned connections that need verification._
- **What connects `install-macos.sh script`, `manifest_version`, `name` to the rest of the system?**
  _100 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Eklenti Manifesti` be split into smaller, more focused modules?**
  _Cohesion score 0.05405405405405406 - nodes in this community are weakly interconnected._
- **Should `Bellek ve Gomu Motoru` be split into smaller, more focused modules?**
  _Cohesion score 0.132890365448505 - nodes in this community are weakly interconnected._