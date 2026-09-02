**🌍 [Türkçe](INSTALLATION.md) | [English](INSTALLATION.en.md) | [العربية](INSTALLATION.ar.md) | [日本語](INSTALLATION.ja.md) | [中文](INSTALLATION.zh.md) | [Русский](INSTALLATION.ru.md) | [Español](INSTALLATION.es.md)**

# Руководство по Установке Claude Code (Ядро на Rust, единый бинарный файл)

Данное руководство содержит пошаговые инструкции по установке и настройке CLI-инструмента **Claude Code Setup** (`claude-code-setup`) с ядром на Rust для различных операционных систем.

> **Замечание о честности:** Запускаемый бинарный файл написан на чистом Rust и является самодостаточным. Однако **установщики в этом руководстве написаны не на Rust**: `install-windows.ps1` — это сценарий PowerShell, а `install-macos.sh` — сценарий Bash. Пакеты выпуска создаются с помощью `package-extension.py` (Python). Статистика языков GitHub: **90,5% Rust, 3,5% Shell, 3,2% Python, 2,8% PowerShell** (измеренное соотношение строк: 91,2% Rust, 8,8% PowerShell + Bash + Python).

---

## 🎯 1. Обзор

- **Единый Исполняемый Файл (Single Binary):** Полное отсутствие зависимостей от скриптов Shell (`.sh`) и Python (`.py`).
- **Кроссплатформенность:** Нативная скорость работы на Windows (x64), Linux (x64) и macOS (x64 / ARM64).
- **Нулевые Внешние Зависимости:** Установка путем скачивания готового исполняемого файла или сборки через `cargo` за считанные секунды.

---

## 📥 2. Способ 1: Скачивание Готового Файла (Рекомендуется)

Скачайте скомпилированный бинарный файл непосредственно со страницы GitHub Releases.

### Windows (x64)
Скачайте и запустите через PowerShell:
```powershell
# Скачать бинарный файл релиза
Invoke-WebRequest -Uri "https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-windows-x86_64.exe" -OutFile "claude-code-setup.exe"

# Запустить автоматическую установку
.\claude-code-setup.exe install --hooks
```

### Linux (x64)
```bash
# Скачать бинарный файл
curl -LO https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-linux-x86_64

# Предоставить права на выполнение
chmod +x claude-code-setup-linux-x86_64

# Запустить установку
./claude-code-setup-linux-x86_64 install --hooks
```

### macOS (x64)
```bash
# Скачать бинарный файл
curl -LO https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-macos-x86_64

# Предоставить права на выполнение
chmod +x claude-code-setup-macos-x86_64

# Запустить установку
./claude-code-setup-macos-x86_64 install --hooks
```

---

## 🛠️ 3. Способ 2: Сборка из Исходного Кода (Cargo)

Если на вашем компьютере установлен инструментарий Rust (`cargo` 1.80+):

```bash
# Клонировать репозиторий
git clone https://github.com/Ercaner1988/claude-code-setup-rustified.git
cd claude-code-setup-rustified

# Собрать бинарный файл релиза
cargo build --release

# Запустить установку
./target/release/claude-code-setup install --hooks
```

Для глобальной установки в систему:
```bash
cargo install --path .
claude-code-setup install --hooks
```

---

## ⚙️ 4. Проверка и Диагностика после Установки

```bash
# Диагностика состояния среды
claude-code-setup status

# Запуск диагностических тестов
claude-code-setup test
```

---

## 🛡️ 5. Аудит Безопасности и Настройка Хуков Git

```bash
# Аудит безопасности с авто-исправлением
claude-code-setup security-audit --fix

# Установка хука pre-commit в репозиторий
claude-code-setup install-hooks --repo-dir .
```

---

## 📚 6. Связанная Документация

- [Полная Документация (README.md)](README.md)
- [Руководство по Развертыванию (DEPLOYMENT_GUIDE.md)](DEPLOYMENT_GUIDE.md)
- [Руководство по Устранению Неполадок (TROUBLESHOOTING.md)](docs/TROUBLESHOOTING.md)
