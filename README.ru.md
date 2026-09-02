**🌍 [Türkçe](README.md) | [English](README.en.md) | [العربية](README.ar.md) | [日本語](README.ja.md) | [中文](README.zh.md) | [Русский](README.ru.md) | [Español](README.es.md)**

# Автономная Установка Claude Code (Ядро на Rust, единый бинарный файл)

[![Rust](https://img.shields.io/badge/Rust%20core-%2591-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/Tests-30%20Passed-green.svg)]()

Локальный инструмент развёртывания, аудита безопасности и движок памяти (`claude-code-setup`), разработанный для управления средой **Claude Code**. Во время работы это единый бинарный файл на Rust; для его запуска не требуется установленный Rust, Python или Node.

### Замечание о честности: этот репозиторий не на 100% написан на Rust

Статистика языков GitHub и измеренная структура кодовой базы (2026-09-02): **2891 строка на Rust / 279 строк не на Rust = 91,2% строк на Rust** (GitHub Linguist: **90,5% Rust, 3,5% Shell, 3,2% Python, 2,8% PowerShell**).

| Язык / Файл | Строк | Доля GitHub | Когда выполняется |
| :--- | ---: | ---: | :--- |
| `Rust` (`src/*.rs`, 10 файлов) | 2891 | 90,5% | Во время работы (CLI + сервер MCP) |
| `install-macos.sh` (Shell / Bash) | 121 | 3,5% | При установке на Linux/macOS |
| `package-extension.py` (Python) | 87 | 3,2% | Только при выпуске версии / упаковке `.mcpb` (CI) |
| `install-windows.ps1` (PowerShell) | 71 | 2,8% | При установке на Windows |

Дополнительные зависимости:
- Хук pre-commit в `src/security.rs` записывается как **встроенный сценарий bash** (`#!/usr/bin/env bash`) — для его работы нужен bash, поставляемый с Git.
- Конвейер выпуска `.github/workflows/release.yml` использует `actions/setup-python` и `npx @anthropic-ai/mcpb validate` → **цепочка CI зависит от Python и Node**.
- Слой векторных представлений движка памяти через `fastembed` **загружает предварительно скомпилированный бинарный файл ONNX Runtime на C++** (`ort-download-binaries`).

Точное резюме: **исполняемый файл во время работы написан на чистом Rust; однако установка, упаковка и конвейер выпуска используют Bash + PowerShell + Python + Node.**

---

## 🎯 1. Цель и Выполненные Работы

- **Единый бинарный файл во время работы:** унаследованные *исполняемые* сценарии на Bash и Python перенесены на Rust. Сценарии установки и упаковки (`install-*.{sh,ps1}`, `package-extension.py`) сохранены осознанно, поскольку сам установщик обязан работать до загрузки бинарного файла.
- **Динамическая нормализация путей:** жёстко заданные пути (например, `/home/jb_remus`) динамически подстраиваются под целевую операционную систему и домашний каталог локального пользователя.
- **Управление MCP для нескольких целей (`--target`):**
  - Управление конфигурациями **Claude Code** (`~/.claude.json`), **проекта** (`./.mcp.json`) и **Claude Desktop** (`claude_desktop_config.json`) из единого CLI.
  - Атомарный механизм записи, сохраняющий нетипизированные поля JSON благодаря структуре `serde_json::Value` (с автоматической резервной копией `.bak`).
- **Режим сервера MCP (`--mcp-mode`):** тот же бинарный файл превращается в сервер MCP, общающийся по JSON-RPC через stdin/stdout, и предоставляет Claude Desktop 8 инструментов, объявленных в `manifest.json`.
- **Движок памяти (SQLite + векторы + граф):**
  - **Быстрое добавление заметки (`memory-note`):** безопасное создание заметок с именами файлов в стиле kebab-case.
  - **Поиск по словам через FTS5:** словарное индексирование SQLite с механизмом экранирования кавычек.
  - **Локальные векторные представления:** косинусное сходство через `fastembed` (Multilingual-E5-Small). Модель загружается с Hugging Face при первом использовании и записывается в `$HOME/.claude/fastembed_cache`; **после этой первой загрузки** поиск работает полностью автономно.
  - **Рёбра графа и Wikilink:** поиск по соседству (`memory-related`) через ссылки `[[Имя-Заметки]]` и семантические связи выше порога.
  - **Гибридное упорядочивание RRF:** объединение результатов FTS5 и векторного поиска методом Reciprocal Rank Fusion (`k=60`).
- **Аудит безопасности с автоисправлением (`security-audit --fix`):**
  - Сканирование секретов, записанных открытым текстом в конфигурациях (`ghp_`, `github_pat_`, `sk-`, `xox[baprs]-`, `AKIA`).
  - Ужесточение прав доступа к файлам до 600 — **только в Unix**; в Windows выводится справочное замечание о правах на основе ACL, исправление не выполняется.
  - Установка хука Git pre-commit для защиты ветвей и сканирования секретов.
- **Автономный рабочий поток Git (`agent-workflow`):**
  - Автоматическое создание ветви функциональности из удалённой ветви по умолчанию.
  - Блокировка прямой отправки (push) в защищённые основные ветви.

---

## 🏗️ 2. Архитектура и Модули

```
claude-code-setup-rustified/
├── Cargo.toml                  # Зависимости Rust и определения пакета (v0.1.6)
├── manifest.json               # Манифест расширения Claude Desktop (8 инструментов MCP)
├── icon.png                    # Значок расширения
├── .env.example                # Образец переменных среды
├── src/
│   ├── main.rs                 # Точка входа CLI и маршрутизатор команд (123 строки)
│   ├── cli.rs                  # Определения команд, целей и флагов на основе Clap (222)
│   ├── mcp.rs                  # Многоцелевой менеджер MCP, сохраняющий значения JSON (488)
│   ├── mcp_server.rs           # Сервер MCP stdio JSON-RPC; сопоставляет 8 инструментов с CLI (436)
│   ├── memory_engine.rs        # Движок FTS5 + векторы + граф + RRF + memory-note (821)
│   ├── installer.rs            # Установщик каркаса каталогов, начального README и .env (191)
│   ├── security.rs             # Аудитор безопасности с автоисправлением и менеджер хуков (296)
│   ├── branch_manager.rs       # Исполнитель автономного потока Git с защитой ветвей (161)
│   ├── tester.rs               # Исполнитель диагностических тестов системы и среды (123)
│   └── agent.rs                # Интерфейс интеграции агента (30)
├── install-windows.ps1         # Установщик на PowerShell (НЕ Rust)
├── install-macos.sh            # Установщик на Bash (НЕ Rust)
├── package-extension.py        # Упаковщик .mcpb, вызывается в CI (НЕ Rust)
├── .github/workflows/
│   ├── rust.yml                # fmt + clippy + test + build (ubuntu/windows/macos)
│   └── release.yml             # Конвейер выпуска бинарных файлов для 3 платформ и .mcpb
└── docs/                       # Руководства по установке и устранению неполадок
```

### Обязанности Модулей
- `src/main.rs`: разбирает аргументы командной строки; при указании `--mcp-mode` передаёт управление серверу MCP, иначе — функции соответствующего модуля.
- `src/cli.rs`: через структуру `Parser` из Clap управляет 15 подкомандами, флагами (`--target`, `--fix`, `--hooks`, `--mode`, `--min-score`) и общим флагом `--mcp-mode`.
- `src/mcp.rs`: считывает и обновляет настройки MCP согласно параметру `--target` (`claude-code`, `project`, `claude-desktop`); обеспечивает атомарную запись без удаления неизвестных полей.
- `src/mcp_server.rs`: устанавливает цикл JSON-RPC на stdin/stdout; сопоставляет 8 инструментов из `manifest.json` (`mcp_list`, `mcp_add`, `security_audit`, `memory_note`, `memory_index`, `memory_search`, `status`, `test`) с реальными командами CLI. Это сопоставление закреплено тестом `her_arac_gercek_bir_cli_komutuna_esleniyor`.
- `src/memory_engine.rs`: разбивает заметки на окна примерно по 1500 символов, вычисляет их векторные представления и усредняет (mean-pooling); управляет таблицами SQLite `knowledge_notes` и `note_edges`. Кэш представлений — `$HOME/.claude/fastembed_cache`.
- `src/installer.rs`: создаёт каталог `$HOME/claude_global_memory/knowledge` и начальный файл `README.md`, никогда их не перезаписывая; копирует `.env`, если он отсутствует.
- `src/security.rs`: сканирует секреты в открытом виде, проверяет права доступа, исправляет их с флагом `--fix` и устанавливает хук Git (хук является встроенным сценарием bash).
- `src/branch_manager.rs`: управляет автономным созданием ветвей, барьером защищённых ветвей и безопасными процессами commit/push.
- `src/tester.rs`: выполняет диагностику системы (`status`) и проверку тестов.

---

## 🚀 3. Установка и Настройка

### Быстрый Старт

Существует две разные установки; решите, какая вам нужна.

**Расширение Claude Desktop (рекомендуется)** — загрузите пакет, соответствующий вашей операционной системе, из [последнего выпуска](https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest) и перетащите его в Claude Desktop → Settings → Extensions:

| Операционная система | Файл | Примерный размер |
|---|---|---|
| Windows | `claude-code-setup-windows.mcpb` | 9 МБ |
| macOS | `claude-code-setup-macos.mcpb` | 10 МБ |
| Linux | `claude-code-setup-linux.mcpb` | 12 МБ |

**Инструмент командной строки** — если вы хотите пользоваться им из терминала:

```powershell
irm https://raw.githubusercontent.com/Ercaner1988/claude-code-setup-rustified/main/install-windows.ps1 | iex
```

```bash
curl -fsSL https://raw.githubusercontent.com/Ercaner1988/claude-code-setup-rustified/main/install-macos.sh | bash
```

Эти установщики являются сценариями PowerShell и Bash (не Rust); они помещают загруженный бинарный файл в каталог пользователя и добавляют его в PATH (права администратора не нужны). Расширение они **не регистрируют** — для расширения используйте путь `.mcpb` выше. Для проверки выполните `claude-code-setup status` в новом терминале.

Подробную установку см. в [INSTALLATION.ru.md](INSTALLATION.ru.md)

---

### Ручная Установка: Сборка из Исходного Кода

#### Требования
- **Инструментарий Rust:** `rustc` и `cargo` (1.80+)
- При первой сборке `fastembed` загружает бинарный файл ONNX Runtime → требуется доступ к сети.

#### Сборка
```bash
cargo build --release

# Полученный бинарный файл:
# Windows: ./target/release/claude-code-setup.exe
# Linux/macOS: ./target/release/claude-code-setup
```

### Автоматическая Установка и Диагностика Среды
```bash
# Проверяет предварительные условия, создаёт каркас памяти
./target/release/claude-code-setup install --hooks

# Диагностическое состояние системы и среды
./target/release/claude-code-setup status
```

---

## 📖 4. Использование и Примеры

### Сводная Таблица Команд

| Команда | Описание |
| :--- | :--- |
| `--mcp-mode` (общий флаг) | Запускает бинарный файл как сервер MCP, общающийся по JSON-RPC через stdin/stdout |
| `install [--hooks] [--skip-prereqs]` | Настройка среды, каркас памяти и копирование `.env` |
| `test` / `status` | Диагностика Claude CLI, `.claude.json`, базы памяти и хуков |
| `mcp-list [--target T]` | Перечисляет настроенные серверы MCP согласно цели |
| `mcp-set <srv> [--command C] [--arg A]… [--env K=V]… [--target T]` | Добавляет или обновляет сервер MCP (`--target`: `claude-code`, `project`, `claude-desktop`) |
| `mcp-unset <srv> [--env K]… [--clear-args] [--remove] [--target T]` | Удаляет переменные или полностью убирает сервер (флаг `--remove` обязателен) |
| `mcp-enable <srv>` / `mcp-disable <srv>` | Включает/выключает сервер, не нарушая конфигурацию |
| `memory-note <заголовок> [--body ...] [--dir D]` | Добавляет новую заметку Markdown в базу знаний |
| `memory-index [--source КАТАЛОГ]… [--edge-threshold 0.70]` | Индексирует заметки в движок SQLite + векторы + граф |
| `memory-search <запрос> [--mode keyword\|semantic\|hybrid] [--limit 5] [--min-score 0.30]` | Ищет в памяти в режиме слов FTS5, векторном или гибридном RRF |
| `memory-related <note.md>` | Перечисляет связанные заметки через рёбра графа и ссылки Wikilink |
| `install-hooks [--repo-dir ПУТЬ]` | Устанавливает в репозиторий хук безопасности pre-commit |
| `security-audit [--fix]` | Проводит аудит безопасности; с флагом `--fix` применяет автоисправление |
| `agent-workflow [--branch-type ТИП] --description ОПИСАНИЕ [--files F]…` | Запускает автономный поток ветвей и коммитов Git с защитой ветвей |

Все команды принимают переопределение `--home-dir` для изоляции тестов (кроме `install-hooks` и `agent-workflow`).

### Примеры Сценариев Использования

#### Управление Серверами MCP по Целям
```bash
# Определить сервер MCP на уровне проекта (.mcp.json)
./target/release/claude-code-setup mcp-set github \
  --command "npx" --arg "-y" --arg "@modelcontextprotocol/server-github" \
  --env "GITHUB_TOKEN=$GITHUB_TOKEN" --target project

# Отключить сервер в конфигурации Claude Desktop
./target/release/claude-code-setup mcp-disable github --target claude-desktop

# Полностью убрать сервер (флаг --remove обязателен)
./target/release/claude-code-setup mcp-unset github --remove --target claude-code
```

#### Добавление Заметки в Память и Гибридный Поиск RRF
```bash
./target/release/claude-code-setup memory-note "Архитектурные Решения" --body "Перенос среды выполнения в нативный бинарный файл Rust завершён."
./target/release/claude-code-setup memory-index --edge-threshold 0.70
./target/release/claude-code-setup memory-search "архитектура Rust" --mode hybrid --limit 5 --min-score 0.30
./target/release/claude-code-setup memory-related mimari-kararlar.md
```

---

## 🛡️ 5. Тесты и Ворота Качества

```bash
cargo test
# running 30 tests
# test result: ok. 30 passed; 0 failed; 0 ignored
```

В исходном коде определено **31 тестов**; один из них (`test_enforce_file_permissions_fixes_mode`) помечен `#[cfg(unix)]` и потому не компилируется в Windows. Измерено: **30/30 зелёных в Windows, 31/31 в Unix** (2026-09-02).

Распределение по файлам: `memory_engine.rs` 14, `mcp.rs` 5, `mcp_server.rs` 5, `security.rs` 3, `branch_manager.rs` 2, `installer.rs` 2.

### Стандарты Качества
- **Охват:** многоцелевое управление MCP, сохранность `Value` в JSON, экранирование символов FTS5, гибридное упорядочивание RRF, mean-pooling, разбор Wikilink, регрессия пути кэша представлений, сканирование секретов, сопоставление инструментов MCP с CLI и барьеры защищённых ветвей.
- **Форматирование:** `cargo fmt --all -- --check` → чисто (2026-09-02).
- **Статический анализ:** `cargo clippy --all-targets -- -D warnings` → без предупреждений (2026-09-02).
- **Непрерывная интеграция:** `.github/workflows/rust.yml` выполняет fmt + clippy + test + сборку выпуска на трёх операционных системах (ubuntu, windows, macos). `.github/workflows/release.yml` создаёт бинарные файлы для трёх платформ и пакеты `.mcpb`; этот конвейер использует Python и Node.

---

## 👥 6. Участники

Приведённые числа измерены командой `git shortlog -sne --all` и подсчётом метк `Co-authored-by` в телах коммитов (2026-09-02, всего 45 коммитов).

| Участник | Роль / Ответственность | Измеренный вклад |
| :--- | :--- | :--- |
| **Ercan ER** | Архитектура проекта, перенос на Rust, ведущий разработчик | 41 коммит (автор) |
| **Claude Opus 5** | Автономный агент ИИ, разработка модулей | 14 коммитов (соавтор) |
| **Copilot App** | Помощник ИИ по написанию кода | 11 коммитов (соавтор) |
| **Claude Opus 4.8** | Автономный агент ИИ | 3 коммита (соавтор) |
| **Claude** (версия не указана) | Автономный агент ИИ | 2 коммита (соавтор) |
| **jb_remus** | Первоначальный автор вышестоящего проекта (upstream) | 2 коммита (автор) |
| **Mihenk** | Проверяющий код и судья качества | 1 коммит (автор) |
| **arturo-ebuck** | Участник открытого исходного кода | 1 коммит (автор) |

**Kassam** — это личность агента, записанная в поле `authors` файла `Cargo.toml`; отдельной записи автора в Git у неё нет.

---

## 📄 7. Лицензия и Ресурсы

Этот проект лицензирован по [Лицензии MIT](LICENSE) (Авторское право © 2026 Ercan Er).

### Связанные Документы
- [Руководство по Развёртыванию](DEPLOYMENT_GUIDE.md)
- [Руководство по Ручной Установке](docs/MANUAL_SETUP.md)
- [Руководство по Устранению Неполадок](docs/TROUBLESHOOTING.md)
- [Директивы для Разработчиков](docs/dev/TASK-KASSAM-1-2.md)
