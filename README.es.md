**🌍 [Türkçe](README.md) | [English](README.en.md) | [العربية](README.ar.md) | [日本語](README.ja.md) | [中文](README.zh.md) | [Русский](README.ru.md) | [Español](README.es.md)**

# Instalación Independiente de Claude Code (Motor 100% Rust)

[![Rust](https://img.shields.io/badge/Rust-100%25-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/Tests-24%20Passed-green.svg)]()

Un sistema de despliegue, gestión, auditoría de seguridad y motor de memoria de alto rendimiento en un solo binario **100% nativo en Rust** (`claude-code-setup.exe`) para **Claude Code**.

Todos los scripts heredados en Bash (`.sh`) y Python (`.py`) han sido completamente eliminados y refactorizados en una herramienta CLI unificada en Rust.

---

## 🎯 1. Propósito y Características

- **Arquitectura 100% Rust Pura:** Cero dependencias de scripts de Shell o entornos de ejecución Python.
- **Normalización Dinámica de Rutas:** Los patrones de rutas estáticas (por ejemplo, `/home/jb_remus`) se adaptan automáticamente al sistema operativo de destino y al directorio personal local.
- **Gestión Dinámica de MCP Multi-Objetivo (`--target`):**
  - Administre dinámicamente servidores MCP en **Claude Code** (`~/.claude.json`), **Proyecto** (`./.mcp.json`) y **Claude Desktop** (`claude_desktop_config.json`).
  - Administrador de configuración JSON que preserva campos no modelados con respaldos automáticos `.bak`.
- **Motor de Memoria Rápido basado en SQLite (Vectores + Grafos):**
  - **Creación Rápida de Notas (`memory-note`):** Añada notas Markdown de forma segura con nombres de archivo en formato kebab-case sin sobrescribir archivos existentes.
  - **Búsqueda por Palabras Clave FTS5:** Búsqueda de texto completo con escape automático de sintaxis especial.
  - **Incrustaciones Locales:** Cálculo sin conexión de similitud de coseno mediante `fastembed` (Multilingual-E5-Small).
  - **Aristas de Grafo y Wikilinks:** Exploración de vecindades BFS a través de referencias `[[Nombre-Nota]]` y enlaces semánticos (`memory-related`).
  - **Clasificación Híbrida RRF:** Algoritmo Reciprocal Rank Fusion (`k=60`) que combina resultados de búsqueda con máxima precisión.
- **Auditoría de Seguridad con Auto-Reparación (`security-audit --fix`):**
  - Escanea archivos de configuración en busca de tokens secretos en texto plano.
  - Corrige automáticamente los permisos de archivos en sistemas Unix.
  - Instala ganchos pre-commit de Git para la protección de ramas y el escaneo de secretos.
- **Flujo de Trabajo Autónomo y Seguro en Git (`agent-workflow`):**
  - Automatiza la creación de ramas de características desde la rama remota por defecto.
  - Evita envíos (push) directos a las ramas principales protegidas.

---

## 🏗️ 2. Arquitectura y Módulos

```
claude-code-complete-setup/
├── Cargo.toml                  # Manifiesto del proyecto y dependencias de Rust
├── src/
│   ├── main.rs                 # Punto de entrada CLI y enrutador de comandos
│   ├── cli.rs                  # Definiciones de comandos, objetivos y banderas con Clap
│   ├── mcp.rs                  # Gestor de MCP multi-objetivo que preserva valores JSON
│   ├── memory_engine.rs        # Motor híbrido FTS5 + Vectores + Grafos + RRF + memory-note
│   ├── installer.rs            # Directorio esqueleto, README inicial y creador de .env
│   ├── security.rs             # Auditor de seguridad con auto-reparación y gestor de ganchos
│   ├── branch_manager.rs       # Ejecutor de flujo de trabajo Git para ramas protegidas
│   ├── tester.rs               # Ejecutor de la suite de pruebas de diagnóstico
│   └── agent.rs                # Interfaz de integración de agentes
└── docs/                       # Guías de instalación y resolución de problemas
```

### Responsabilidades de los Módulos
- `src/main.rs`: Analiza los argumentos de la línea de comandos y los redirige al módulo correspondiente.
- `src/cli.rs`: Gestiona los subcomandos, opciones (`--target`, `--fix`, `--hooks`, `--mode`) y ayuda mediante Clap.
- `src/mcp.rs`: Lee y actualiza la configuración de MCP según el objetivo indicado (`claude-code`, `project`, `claude-desktop`), manteniendo los campos personalizados.
- `src/memory_engine.rs`: Divide el texto en fragmentos, gestiona las tablas SQLite `knowledge_notes` y `note_edges` y añade notas de forma segura vía `memory-note`.
- `src/installer.rs`: Inicializa el directorio `~/claude_global_memory/knowledge` y el archivo `README.md` sin sobrescribir archivos previos.
- `src/security.rs`: Audita permisos y secretos, aplica `--fix` e instala ganchos de seguridad.
- `src/branch_manager.rs`: Automatiza la creación de ramas y verifica las protecciones de ramas principales.
- `src/tester.rs`: Ejecuta las pruebas de diagnóstico (`status` y `test`).

---

## 🚀 3. Instalación y Configuración

### Requisitos Previos
- **Herramientas de Rust:** `rustc` y `cargo` (1.80 o superior)

### Compilación
```bash
# Compilar el binario de producción
cargo build --release

# Binario resultante:
# Windows: ./target/release/claude-code-setup.exe
# Linux/macOS: ./target/release/claude-code-setup
```

### Instalación Automatizada y Diagnóstico
```bash
# Ejecutar la instalación automática e instalar ganchos de seguridad
./target/release/claude-code-setup install --hooks

# Ejecutar el diagnóstico del entorno
./target/release/claude-code-setup status
```

---

## 📖 4. Uso y Ejemplos

### Tabla Resumen de Comandos

| Comando | Descripción |
| :--- | :--- |
| `install [--hooks]` | Instalación completa, creación del esqueleto de memoria y archivo `.env` |
| `test` / `status` | Diagnóstico de Claude CLI, `.claude.json`, BD de memoria y ganchos |
| `mcp-list [--target T]` | Enumera los servidores MCP configurados según el objetivo |
| `mcp-set <srv> [...] [--target T]` | Añade o actualiza un servidor MCP (`--target`: `claude-code`, `project`, `claude-desktop`) |
| `mcp-unset <srv> [...] [--remove] [--target T]` | Elimina campos; la eliminación completa requiere `--remove` |
| `mcp-enable <srv>` / `mcp-disable <srv>` | Habilita o desactiva un servidor sin perder su configuración |
| `memory-note <título> [--body ...]` | Añade de forma segura una nueva nota Markdown a la base de conocimiento |
| `memory-index [--source DIR]...` | Indexa notas en el motor SQLite + Vectores + Grafos |
| `memory-search <consulta> [--mode ...]` | Busca notas usando FTS5, Vectores o modo Híbrido RRF |
| `memory-related <nota.md>` | Muestra notas relacionadas a través de aristas de grafo |
| `install-hooks [--repo-dir RUTA]` | Instala el gancho de seguridad pre-commit en un repositorio |
| `security-audit [--fix]` | Audita la seguridad y secretos; `--fix` aplica auto-reparación |
| `agent-workflow [-t TIPO] -d DESC` | Ejecuta el flujo de trabajo de Git con protección de ramas |

### Ejemplos de Escenarios

#### Gestión de Servidores MCP por Objetivo
```bash
# Configurar un servidor MCP a nivel de proyecto (.mcp.json)
./target/release/claude-code-setup mcp-set github --command "npx" --arg "-y" --arg "@modelcontextprotocol/server-github" --env "GITHUB_TOKEN=ghp_example" --target project

# Desactivar un servidor en la configuración de Claude Desktop
./target/release/claude-code-setup mcp-disable github --target claude-desktop

# Eliminar completamente el servidor (se requiere la opción --remove)
./target/release/claude-code-setup mcp-unset github --remove --target claude-code
```

#### Añadir Notas de Memoria y Búsqueda Híbrida
```bash
# Añadir una nueva nota
./target/release/claude-code-setup memory-note "Decisiones de Arquitectura" --body "Refactorización a binario único en Rust completada."

# Indexar notas de conocimiento
./target/release/claude-code-setup memory-index --edge-threshold 0.70

# Ejecutar Búsqueda Híbrida RRF
./target/release/claude-code-setup memory-search "Arquitectura Rust" --mode hybrid --limit 5

# Consultar notas relacionadas
./target/release/claude-code-setup memory-related architecture-decisions.md
```

---

## 🛡️ 5. Puertas de Calidad y Pruebas

El proyecto incluye 24 pruebas unitarias y todas están aprobadas:

```bash
cargo test
```

### Estándares de Calidad
- **Pruebas Unitarias (24/24 Aprobadas):** Gestión multi-objetivo de MCP, preservación de JSON Value, escape en FTS5, clasificación híbrida RRF, mean-pooling, extracción de wikilinks, auditoría de secretos y protección de ramas.
- **Formato:** Validado mediante `cargo fmt --check`
- **Integración Continua (CI):** Confirmado en Ubuntu, macOS y Windows a través de `.github/workflows/rust.yml` y `.github/workflows/release.yml`.

---

## 👥 6. Colaboradores

| Colaborador | Rol / Responsabilidad | Métricas |
| :--- | :--- | :--- |
| **Ercan ER** | Arquitecto Principal, Migración a Rust y Desarrollador Principal | 26 commits |
| **Kassam** | Agente de IA Autónomo, Desarrollador del Motor Rust y Módulos | Coautor / Colaborador |
| **Copilot** | Asistente de Código IA | 4 commits |
| **jb_remus** | Autor Original del Proyecto (Upstream) | 2 commits |
| **Mihenk** | Auditor de Código y Revisor de Calidad | 1 commit |
| **arturo-ebuck** | Colaborador de Código Abierto | 1 commit |

---

## 📄 7. Licencia y Recursos

Distribuido bajo la [Licencia MIT](LICENSE).

### Documentación Relacionada
- [Guía de Despliegue](DEPLOYMENT_GUIDE.md)
- [Guía de Instalación Manual](docs/MANUAL_SETUP.md)
- [Guía de Resolución de Problemas](docs/TROUBLESHOOTING.md)
- [Directivas para Desarrolladores](docs/dev/TASK-KASSAM-1-2.md)
