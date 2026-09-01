# Graph Report - claude-code-setup-rustified  (2026-09-01)

## Corpus Check
- 34 files · ~22,612 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 486 nodes · 657 edges · 33 communities (30 shown, 3 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 8 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `21c8d0e1`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_Bellek ve Gomu Motoru|Bellek ve Gomu Motoru]]
- [[_COMMUNITY_Eklenti Manifesti|Eklenti Manifesti]]
- [[_COMMUNITY_README Genel Tanitim|README Genel Tanitim]]
- [[_COMMUNITY_Kurulum ve Tanilama|Kurulum ve Tanilama]]
- [[_COMMUNITY_MCP Yapilandirma Yonetimi|MCP Yapilandirma Yonetimi]]
- [[_COMMUNITY_MCP Sunucu Protokolu|MCP Sunucu Protokolu]]
- [[_COMMUNITY_Kurulum Kilavuzu|Kurulum Kilavuzu]]
- [[_COMMUNITY_Git Dal Yonetimi|Git Dal Yonetimi]]
- [[_COMMUNITY_Guvenlik Denetimi|Guvenlik Denetimi]]
- [[_COMMUNITY_Gorev Direktifi|Gorev Direktifi]]
- [[_COMMUNITY_Dagitim Kilavuzu|Dagitim Kilavuzu]]
- [[_COMMUNITY_Sorun Giderme Kilavuzu|Sorun Giderme Kilavuzu]]
- [[_COMMUNITY_Elle Kurulum Kilavuzu|Elle Kurulum Kilavuzu]]
- [[_COMMUNITY_CLI Komut Tanimlari|CLI Komut Tanimlari]]
- [[_COMMUNITY_Ajan Is Akisi|Ajan Is Akisi]]
- [[_COMMUNITY_Eklenti Paketleme|Eklenti Paketleme]]
- [[_COMMUNITY_Giris Noktasi|Giris Noktasi]]
- [[_COMMUNITY_macOS Kurucu|macOS Kurucu]]
- [[_COMMUNITY_Community 19|Community 19]]
- [[_COMMUNITY_Community 20|Community 20]]
- [[_COMMUNITY_Community 21|Community 21]]
- [[_COMMUNITY_Community 22|Community 22]]
- [[_COMMUNITY_Community 23|Community 23]]
- [[_COMMUNITY_Community 24|Community 24]]
- [[_COMMUNITY_Community 25|Community 25]]
- [[_COMMUNITY_Community 26|Community 26]]
- [[_COMMUNITY_Community 27|Community 27]]
- [[_COMMUNITY_Community 28|Community 28]]
- [[_COMMUNITY_Community 29|Community 29]]
- [[_COMMUNITY_Community 30|Community 30]]
- [[_COMMUNITY_Community 31|Community 31]]
- [[_COMMUNITY_Community 32|Community 32]]

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
- `add_memory_note()` --calls--> `sanitize_description()`  [INFERRED]
  src/memory_engine.rs → src/branch_manager.rs
- `resolve_config_path()` --calls--> `get_home_dir()`  [INFERRED]
  src/mcp.rs → src/installer.rs
- `default_knowledge_dir()` --calls--> `get_home_dir()`  [INFERRED]
  src/memory_engine.rs → src/installer.rs
- `embedding_cache_dir()` --calls--> `get_home_dir()`  [INFERRED]
  src/memory_engine.rs → src/installer.rs
- `get_db_path()` --calls--> `get_home_dir()`  [INFERRED]
  src/memory_engine.rs → src/installer.rs

## Import Cycles
- 1-file cycle: `src/agent.rs -> src/agent.rs`
- 1-file cycle: `src/installer.rs -> src/installer.rs`
- 1-file cycle: `src/main.rs -> src/main.rs`
- 1-file cycle: `src/mcp.rs -> src/mcp.rs`
- 1-file cycle: `src/memory_engine.rs -> src/memory_engine.rs`
- 1-file cycle: `src/tester.rs -> src/tester.rs`
- 1-file cycle: `src/security.rs -> src/security.rs`

## Communities (33 total, 3 thin omitted)

### Community 0 - "Bellek ve Gomu Motoru"
Cohesion: 0.13
Nodes (40): Connection, add_memory_note(), bytes_to_f32_vec(), chunk_content(), cosine_similarity(), default_knowledge_dir(), embedding_cache_dir(), escape_fts5_query() (+32 more)

### Community 1 - "Eklenti Manifesti"
Cohesion: 0.05
Nodes (36): author, name, url, compatibility, claude_desktop, platforms, description, display_name (+28 more)

### Community 2 - "README Genel Tanitim"
Cohesion: 0.09
Nodes (22): 🎯 1. Varış Noktası ve Tamamlananlar, 🏗️ 2. Mimari ve Modüller, 🚀 3. Kurulum ve Yapılandırma, 📖 4. Kullanım ve Örnekler, 🛡️ 5. Test ve Kalite Kapıları, 👥 6. Katkıda Bulunanlar, 📄 7. Lisans ve Kaynaklar, Claude Code Bağımsız Kurulum (%100 Rust Motoru) (+14 more)

### Community 3 - "Kurulum ve Tanilama"
Cohesion: 0.17
Nodes (17): check_cmd(), ensure_env_file(), ensure_knowledge_skeleton(), get_home_dir(), log_info(), log_success(), log_warning(), Option (+9 more)

### Community 4 - "MCP Yapilandirma Yonetimi"
Cohesion: 0.29
Nodes (20): list_mcp_servers(), mcp_set(), mcp_toggle(), mcp_unset(), McpTarget, read_json_value(), resolve_config_path(), Option (+12 more)

### Community 5 - "MCP Sunucu Protokolu"
Cohesion: 0.21
Nodes (19): call_tool(), handle_request(), her_arac_gercek_bir_cli_komutuna_esleniyor(), initialize(), json_rpc_error(), JsonRpcRequest, list_resources(), list_tools() (+11 more)

### Community 6 - "Kurulum Kilavuzu"
Cohesion: 0.11
Nodes (17): 🛠️ 1. Ön Gereksinimler (İsteğe Bağlı & Otomatik Kurulumlar), 📥 2. Adım Adım Hızlı Kurulum (Hazır İkili Dosya - En Kolay Yol), ⚙️ 3. Kurulum Sonrası Doğrulama ve Tanı, 🛡️ 4. Güvenlik Denetimi ve Kanca (Hook) Kurulumu, 📚 5. İlgili Dokümanlar, A. Claude Code CLI (Ortamınızda Yoksa), B. Rust Toolchain (Yalnızca Kaynak Koddan Derleyecekler İçin), Claude Code Bağımsız Kurulum Kılavuzu (%100 Rust Motoru) (+9 more)

### Community 7 - "Git Dal Yonetimi"
Cohesion: 0.36
Nodes (12): create_feature_branch(), default_remote_branch(), ensure_safe_branch(), get_current_branch(), git_stdout(), is_protected_branch(), Result, String (+4 more)

### Community 8 - "Guvenlik Denetimi"
Cohesion: 0.29
Nodes (12): enforce_file_permissions(), find_secrets(), install_git_hooks(), Option, Path, Result, String, Vec (+4 more)

### Community 9 - "Gorev Direktifi"
Cohesion: 0.15
Nodes (12): A. Gerçek kusurlar, B. Kalite, Kassam Görev Direktifi — Özellik 1 (Dinamik MCP) + Özellik 2 (Semantik + Graph Memory), Korkuluklar (değişmedi), Kurallar (ponytail — inceleme bunları zorlayacak), Teslim, TUR 2 — Denetim sonrası düzeltme + kalite, Tur 2 kabul kriterleri (+4 more)

### Community 10 - "Dagitim Kilavuzu"
Cohesion: 0.17
Nodes (11): Day-to-day Operations, Deploy from Source, Deployment Guide, macOS (Automatic), Manual Extension Installation, Prerequisites (For Manual Build), Quick Start: Claude Code Extension, Updating (+3 more)

### Community 11 - "Sorun Giderme Kilavuzu"
Cohesion: 0.20
Nodes (9): `agent-workflow` fails with "git fetch origin failed", Build errors on Windows, `claude` command not found, First `memory-index` / `memory-search` is slow or downloads something, `mcp-set` wrote to a config I didn't expect, `memory-search` says "Memory database not found", Pre-commit hook does not run on Windows, Search returns no results for queries with `-`, `:`, quotes or `*` (+1 more)

### Community 12 - "Elle Kurulum Kilavuzu"
Cohesion: 0.25
Nodes (7): 1. Prerequisites, 2. Knowledge Base, 3. MCP Servers, 4. Environment Variables, 5. Git Hooks, 6. Verify, Manual Setup Guide

### Community 13 - "CLI Komut Tanimlari"
Cohesion: 0.40
Nodes (4): Commands, Cli, Commands, Option

### Community 14 - "Ajan Is Akisi"
Cohesion: 0.67
Nodes (3): Result, String, run_agent_workflow()

### Community 19 - "Community 19"
Cohesion: 0.11
Nodes (18): 🎯 1. الغرض والميزات, 🏗️ 2. البنية والموديولات, 🚀 3. التثبيت والإعداد, 📖 4. الاستخدام والأمثلة, 🛡️ 5. بوابات الجودة والاختبارات, 👥 6. المساهمون, 📄 7. الترخيص والمراجع, Claude Code الإعداد المستقل (محرك Rust بنسبة 100%) (+10 more)

### Community 20 - "Community 20"
Cohesion: 0.11
Nodes (18): 🎯 1. Purpose & Features, 🏗️ 2. Architecture & Modules, 🚀 3. Installation & Setup, 📖 4. Usage & Examples, 🛡️ 5. Quality Gates & Testing, 👥 6. Contributors, 📄 7. License & Resources, Adding Memory Notes & Hybrid Search (+10 more)

### Community 21 - "Community 21"
Cohesion: 0.11
Nodes (18): 🎯 1. Propósito y Características, 🏗️ 2. Arquitectura y Módulos, 🚀 3. Instalación y Configuración, 📖 4. Uso y Ejemplos, 🛡️ 5. Puertas de Calidad y Pruebas, 👥 6. Colaboradores, 📄 7. Licencia y Recursos, Añadir Notas de Memoria y Búsqueda Híbrida (+10 more)

### Community 22 - "Community 22"
Cohesion: 0.11
Nodes (18): 🎯 1. 目的と特徴, 🏗️ 2. アーキテクチャとモジュール, 🚀 3. インストールと設定, 📖 4. 使用方法と例, 🛡️ 5. 品質ゲートとテスト, 👥 6. 貢献者, 📄 7. ライセンスとリソース, Claude Code スタンドアロンセットアップ (100% Rust エンジン) (+10 more)

### Community 23 - "Community 23"
Cohesion: 0.11
Nodes (18): 🎯 1. Назначение и Возможности, 🏗️ 2. Архитектура и Модули, 🚀 3. Установка и Настройка, 📖 4. Использование и Примеры, 🛡️ 5. Ворота Качества и Тестирование, 👥 6. Участники, 📄 7. Лицензия и Ресурсы, Claude Code Автономный Установщик (100% Rust Движок) (+10 more)

### Community 24 - "Community 24"
Cohesion: 0.11
Nodes (18): 🎯 1. Varış Noktası ve Tamamlananlar, 🏗️ 2. Mimari ve Modüller, 🚀 3. Kurulum ve Yapılandırma, 📖 4. Kullanım ve Örnekler, 🛡️ 5. Test ve Kalite Kapıları, 👥 6. Katkıda Bulunanlar, 📄 7. Lisans ve Kaynaklar, Claude Code Bağımsız Kurulum (%100 Rust Motoru) (+10 more)

### Community 25 - "Community 25"
Cohesion: 0.11
Nodes (18): 🎯 1. 目标与特性, 🏗️ 2. 架构与模块, 🚀 3. 安装与配置, 📖 4. 使用说明与示例, 🛡️ 5. 质量门禁与测试, 👥 6. 贡献者, 📄 7. 许可证与相关资源, Claude Code 独立安装程序 (100% Rust 引擎) (+10 more)

### Community 26 - "Community 26"
Cohesion: 0.15
Nodes (12): 🛠️ 1. Ön Gereksinimler (İsteğe Bağlı & Otomatik Kurulumlar), 📥 2. Adım Adım Hızlı Kurulum (Hazır İkili Dosya - En Kolay Yol), ⚙️ 3. Kurulum Sonrası Doğrulama ve Tanı, 🛡️ 4. Güvenlik Denetimi ve Kanca (Hook) Kurulumu, 📚 5. İlgili Dokümanlar, A. Claude Code CLI (Ortamınızda Yoksa), B. Rust Toolchain (Yalnızca Kaynak Koddan Derleyecekler İçin), Claude Code Bağımsız Kurulum Kılavuzu (%100 Rust Motoru) (+4 more)

### Community 27 - "Community 27"
Cohesion: 0.18
Nodes (10): 🎯 1. نظرة عامة, 📥 2. الطريقة 1: تنزيل البرنامج التنفيذي الجاهز (موصى به), 🛠️ 3. الطريقة 2: البناء من المصدر (Cargo), ⚙️ 4. التحقق والتشخيص بعد التثبيت, 🛡️ 5. تدقيق الأمان وإعداد خطافات Git, 📚 6. الوثائق ذات الصلة, Linux (x64), macOS (x64) (+2 more)

### Community 28 - "Community 28"
Cohesion: 0.18
Nodes (10): 🎯 1. Overview, 📥 2. Method 1: Pre-Compiled Binary Download (Recommended), 🛠️ 3. Method 2: Building from Source (Cargo), ⚙️ 4. Post-Installation Verification & Diagnostics, 🛡️ 5. Security Auditing & Git Hook Setup, 📚 6. Related Documentation, Claude Code Standalone Installation Guide (100% Rust Engine), Linux (x64) (+2 more)

### Community 29 - "Community 29"
Cohesion: 0.18
Nodes (10): 🎯 1. Descripción General, 📥 2. Método 1: Descarga de Binarios Precompilados (Recomendado), 🛠️ 3. Método 2: Compilación desde el Código Fuente (Cargo), ⚙️ 4. Verificación y Diagnóstico Posterior a la Instalación, 🛡️ 5. Auditoría de Seguridad y Configuración de Ganchos Git, 📚 6. Documentación Relacionada, Guía de Instalación Independiente de Claude Code (Motor 100% Rust), Linux (x64) (+2 more)

### Community 30 - "Community 30"
Cohesion: 0.18
Nodes (10): 🎯 1. 概要, 📥 2. 方法 1: ビルド済みバイナリのダウンロード (推奨), 🛠️ 3. 方法 2: ソースコードからのビルド (Cargo), ⚙️ 4. インストール後の検証と診断, 🛡️ 5. セキュリティ監査と Git フックのセットアップ, 📚 6. 関連ドキュメント, Claude Code スタンドアロンインストールガイド (100% Rust エンジン), Linux (x64) (+2 more)

### Community 31 - "Community 31"
Cohesion: 0.18
Nodes (10): 🎯 1. Обзор, 📥 2. Способ 1: Скачивание Готового Файла (Рекомендуется), 🛠️ 3. Способ 2: Сборка из Исходного Кода (Cargo), ⚙️ 4. Проверка и Диагностика после Установки, 🛡️ 5. Аудит Безопасности и Настройка Хуков Git, 📚 6. Связанная Документация, Linux (x64), macOS (x64) (+2 more)

### Community 32 - "Community 32"
Cohesion: 0.18
Nodes (10): 🎯 1. 概述, 📥 2. 方法 1: 下载预编译二进制文件 (推荐), 🛠️ 3. 方法 2: 从源码构建 (Cargo), ⚙️ 4. 安装后验证与诊断, 🛡️ 5. 安全审计与 Git 钩子设置, 📚 6. 相关文档, Claude Code 独立安装指南 (100% Rust 引擎), Linux (x64) (+2 more)

## Knowledge Gaps
- **235 isolated node(s):** `install-macos.sh script`, `manifest_version`, `name`, `display_name`, `version` (+230 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **3 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `get_home_dir()` connect `Kurulum ve Tanilama` to `Bellek ve Gomu Motoru`, `Guvenlik Denetimi`, `MCP Yapilandirma Yonetimi`?**
  _High betweenness centrality (0.034) - this node is a cross-community bridge._
- **Why does `resolve_config_path()` connect `MCP Yapilandirma Yonetimi` to `Kurulum ve Tanilama`?**
  _High betweenness centrality (0.016) - this node is a cross-community bridge._
- **Why does `add_memory_note()` connect `Bellek ve Gomu Motoru` to `Git Dal Yonetimi`?**
  _High betweenness centrality (0.013) - this node is a cross-community bridge._
- **Are the 6 inferred relationships involving `get_home_dir()` (e.g. with `resolve_config_path()` and `default_knowledge_dir()`) actually correct?**
  _`get_home_dir()` has 6 INFERRED edges - model-reasoned connections that need verification._
- **What connects `install-macos.sh script`, `manifest_version`, `name` to the rest of the system?**
  _235 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Bellek ve Gomu Motoru` be split into smaller, more focused modules?**
  _Cohesion score 0.132890365448505 - nodes in this community are weakly interconnected._
- **Should `Eklenti Manifesti` be split into smaller, more focused modules?**
  _Cohesion score 0.05405405405405406 - nodes in this community are weakly interconnected._