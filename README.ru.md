**🌍 [Türkçe](README.md) | [English](README.en.md) | [العربية](README.ar.md) | [日本語](README.ja.md) | [中文](README.zh.md) | [Русский](README.ru.md) | [Español](README.es.md)**

# Claude Code Автономный Установщик (100% Rust Движок)

[![Rust](https://img.shields.io/badge/Rust-100%25-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/Tests-24%20Passed-green.svg)]()

Высокопроизводительный **100% Rust-нативный** инструмент развертывания, аудита безопасности и движок памяти (`claude-code-setup.exe`) для **Claude Code**.

Все устаревшие скрипты Bash (`.sh`) и Python (`.py`) были полностью удалены и рефакторены в единый исполняемый CLI-инструмент.

---

## 🎯 1. Назначение и Возможности

- **100% Чистая Архитектура Rust:** Полное отсутствие зависимостей от Shell-скриптов и среды Python.
- **Динамическая Нормализация Путей:** Жестко заданные пути (например, `/home/jb_remus`) автоматически адаптируются к целевой ОС и локальной домашней директории.
- **Мультицелевое Управление MCP (`--target`):**
  - Динамическое управление серверами MCP для **Claude Code** (`~/.claude.json`), **Проекта** (`./.mcp.json`) и **Claude Desktop** (`claude_desktop_config.json`).
  - Сохранение произвольных полей JSON и создание автоматических резервных копий `.bak`.
- **Быстрый Движок Памяти на SQLite (Векторы + Графы):**
  - **Быстрое Добавление Заметок (`memory-note`):** Безопасное создание заметок в формате Markdown без перезаписи существующих файлов.
  - **Полнотекстовый Поиск FTS5:** Поиск с автоматическим экранированием спецсимволов.
  - **Локальные Векторы (Embeddings):** Автономный расчет косинусного сходства через `fastembed` (Multilingual-E5-Small).
  - **Графовые Связи и Wikilink:** Поиск по графу (BFS) по ссылкам `[[Имя-Заметки]]` и семантическим связям (`memory-related`).
  - **Ранжирование Гибрида RRF:** Алгоритм Reciprocal Rank Fusion (`k=60`) для объединения результатов ключевых слов и векторов.
- **Аудит Безопасности с Авто-Исправлением (`security-audit --fix`):**
  - Сканирование конфигураций на наличие открытых ключей (tokens).
  - Автоматическое исправление прав доступа к файлам на Unix-системах.
  - Установка хуков Git pre-commit для защиты веток и сканирования секретов.
- **Безопасный Автономный Процесс Git (`agent-workflow`):**
  - Автоматическое создание веток функций из удаленных веток по умолчанию.
  - Запрет прямой отправки (push) в защищенные главные ветки.

---

## 🏗️ 2. Архитектура и Модули

```
claude-code-complete-setup/
├── Cargo.toml                  # Манифест проекта и зависимости Rust
├── src/
│   ├── main.rs                 # Точка входа CLI и маршрутизатор команд
│   ├── cli.rs                  # Определение команд, целей и флагов на Clap
│   ├── mcp.rs                  # Мультицелевой менеджер MCP с сохранением структуры JSON
│   ├── memory_engine.rs        # Движок FTS5 + Векторы + Графы + RRF + memory-note
│   ├── installer.rs            # Создание структуры, начального README и файла .env
│   ├── security.rs             # Аудит безопасности с авто-исправлением и менеджер хуков
│   ├── branch_manager.rs       # Исполнитель процесса Git для защищенных веток
│   ├── tester.rs               # Запуск диагностического набора тестов
│   └── agent.rs                # Интерфейс интеграции агентов
└── docs/                       # Руководства по установке и отладке
```

### Обязанности Модулей
- `src/main.rs`: Разбирает аргументы командной строки и направляет выполнение в соответствующие модули.
- `src/cli.rs`: Управляет командами, опциями (`--target`, `--fix`, `--hooks`, `--mode`) и справкой через Clap.
- `src/mcp.rs`: Считывает и обновляет конфигурацию MCP на основе `--target` (`claude-code`, `project`, `claude-desktop`), сохраняя произвольные поля.
- `src/memory_engine.rs`: Разбивает текст на фрагменты, управляет таблицами `knowledge_notes` и `note_edges` в SQLite. Безопасно добавляет заметки через `memory-note`.
- `src/installer.rs`: Создает директорию `~/claude_global_memory/knowledge` и начальный `README.md` без перезаписи существующих файлов.
- `src/security.rs`: Проверяет секреты и права доступа, применяет `--fix` и устанавливает хуки безопасности.
- `src/branch_manager.rs`: Автоматизирует создание веток и проверяет ограничения защищенных веток.
- `src/tester.rs`: Выполняет диагностические проверки (`status` и `test`).

---

## 🚀 3. Установка и Настройка

### Требования
- **Rust Toolchain:** `rustc` и `cargo` (1.80+)

### Компиляция
```bash
# Сборка релизного бинарного файла
cargo build --release

# Итоговый бинарный файл:
# Windows: ./target/release/claude-code-setup.exe
# Linux/macOS: ./target/release/claude-code-setup
```

### Автоматическая Установка и Диагностика
```bash
# Запуск автоматической установки и установка хуков безопасности
./target/release/claude-code-setup install --hooks

# Запуск проверки состояния среды
./target/release/claude-code-setup status
```

---

## 📖 4. Использование и Примеры

### Сводная Таблица Команд

| Команда | Описание |
| :--- | :--- |
| `install [--hooks]` | Полная установка, создание структуры памяти и файла `.env` |
| `test` / `status` | Диагностика Claude CLI, `.claude.json`, базы памяти и хуков |
| `mcp-list [--target T]` | Вывод списка серверов MCP для указанной цели |
| `mcp-set <srv> [...] [--target T]` | Добавление/обновление сервера MCP (`--target`: `claude-code`, `project`, `claude-desktop`) |
| `mcp-unset <srv> [...] [--remove] [--target T]` | Удаление полей; полное удаление сервера требует `--remove` |
| `mcp-enable <srv>` / `mcp-disable <srv>` | Включение/отключение сервера без удаления его конфигурации |
| `memory-note <заголовок> [--body ...]` | Безопасное добавление новой заметки в базу знаний |
| `memory-index [--source ДИР]...` | Индексация заметок в SQLite + Векторный + Графовый движок |
| `memory-search <запрос> [--mode ...]` | Поиск в режиме FTS5, Векторов или Гибрида RRF |
| `memory-related <заметка.md>` | Отображение связанных заметок через связи графа |
| `install-hooks [--repo-dir ПУТЬ]` | Установка хука безопасности Git pre-commit в репозиторий |
| `security-audit [--fix]` | Аудит безопасности и прав; `--fix` применяет авто-исправление |
| `agent-workflow [-t ТИП] -d ОПИСАНИЕ` | Выполнение процесса веток и коммитов с защитой веток |

### Примеры Сценариев

#### Управление Серверами MCP по Целям
```bash
# Настройка сервера MCP на уровне проекта (.mcp.json)
./target/release/claude-code-setup mcp-set github --command "npx" --arg "-y" --arg "@modelcontextprotocol/server-github" --env "GITHUB_TOKEN=ghp_example" --target project

# Отключение сервера в конфигурации Claude Desktop
./target/release/claude-code-setup mcp-disable github --target claude-desktop

# Полное удаление сервера (требуется флаг --remove)
./target/release/claude-code-setup mcp-unset github --remove --target claude-code
```

#### Добавление Заметок и Гибридный Поиск
```bash
# Добавление новой заметки
./target/release/claude-code-setup memory-note "Архитектурные Решения" --body "Переход на единый бинарный файл Rust завершен."

# Индексация заметок
./target/release/claude-code-setup memory-index --edge-threshold 0.70

# Выполнение гибридного поиска RRF
./target/release/claude-code-setup memory-search "Rust архитектура" --mode hybrid --limit 5

# Запрос связанных заметок
./target/release/claude-code-setup memory-related architecture-decisions.md
```

---

## 🛡️ 5. Ворота Качества и Тестирование

Проект содержит 24 модульных теста, и все они успешно пройдены:

```bash
cargo test
```

### Стандарты Качества
- **Модульные Тесты (24/24 Пройдено):** Мультицелевое управление MCP, сохранение JSON Value, экранирование FTS5, гибридный поиск RRF, усреднение векторов, извлечение wikilinks, аудит секретов и защита веток.
- **Форматирование:** Проверяется с помощью `cargo fmt --check`
- **Непрерывная Интеграция (CI):** Подтверждено на Ubuntu, macOS и Windows через `.github/workflows/rust.yml` и `.github/workflows/release.yml`.

---

## 👥 6. Участники

| Участник | Роль / Ответственность | Метрики |
| :--- | :--- | :--- |
| **Ercan ER** | Главный архитектор, миграция на Rust и основной разработчик | 26 commits |
| **Kassam** | Автономный ИИ-агент, разработчик движка Rust и модулей | Соавтор / Участник |
| **Copilot** | ИИ-помощник по написанию кода | 4 commits |
| **jb_remus** | Автор оригинального проекта (Upstream) | 2 commits |
| **Mihenk** | Аудитор кода и рецензент качества | 1 commit |
| **arturo-ebuck** | Участник Open Source сообщества | 1 commit |

---

## 📄 7. Лицензия и Ресурсы

Распространяется под [Лицензией MIT](LICENSE).

### Связанная Документация
- [Руководство по Развертыванию](DEPLOYMENT_GUIDE.md)
- [Руководство по Ручной Установке](docs/MANUAL_SETUP.md)
- [Руководство по Устранению Неполадок](docs/TROUBLESHOOTING.md)
- [Директивы Разработчиков](docs/dev/TASK-KASSAM-1-2.md)
