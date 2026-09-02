**🌍 [Türkçe](README.md) | [English](README.en.md) | [العربية](README.ar.md) | [日本語](README.ja.md) | [中文](README.zh.md) | [Русский](README.ru.md) | [Español](README.es.md)**

# Instalación Independiente de Claude Code (Núcleo Rust, binario único)

[![Rust](https://img.shields.io/badge/Rust%20core-%2591-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/Tests-30%20Passed-green.svg)]()

Herramienta local de despliegue, auditoría de seguridad y motor de memoria (`claude-code-setup`) desarrollada para administrar el entorno de **Claude Code**. En tiempo de ejecución es un único binario de Rust; para usarlo no hace falta tener Rust, Python ni Node instalados en la máquina.

### Nota de honestidad: este repositorio no es 100 % Rust

Estadísticas de idiomas de GitHub y distribución medida del código (2026-09-02): **2891 líneas de Rust / 279 líneas ajenas a Rust = 91,2 % en líneas** (GitHub Linguist: **90,5 % Rust, 3,5 % Shell, 3,2 % Python, 2,8 % PowerShell**).

| Idioma / Archivo | Líneas | Cuota GitHub | Cuándo se ejecuta |
| :--- | ---: | ---: | :--- |
| `Rust` (`src/*.rs`, 10 archivos) | 2891 | 90,5 % | Tiempo de ejecución (CLI + servidor MCP) |
| `install-macos.sh` (Shell / Bash) | 121 | 3,5 % | Durante la instalación en Linux/macOS |
| `package-extension.py` (Python) | 87 | 3,2 % | Solo al publicar versiones / empaquetar `.mcpb` (CI) |
| `install-windows.ps1` (PowerShell) | 71 | 2,8 % | Durante la instalación en Windows |

Dependencias adicionales:
- El gancho pre-commit de `src/security.rs` se escribe como un **script bash incorporado** (`#!/usr/bin/env bash`); para funcionar necesita el bash que acompaña a Git.
- El canal de publicación `.github/workflows/release.yml` usa `actions/setup-python` y `npx @anthropic-ai/mcpb validate` → **la cadena de CI depende de Python y Node**.
- La capa de representaciones vectoriales del motor de memoria **descarga el binario de C++ precompilado de ONNX Runtime** a través de `fastembed` (`ort-download-binaries`).

Resumen exacto: **el binario en tiempo de ejecución es Rust puro; la instalación, el empaquetado y el canal de publicación usan Bash + PowerShell + Python + Node.**

---

## 🎯 1. Destino y Trabajo Completado

- **Tiempo de ejecución de un solo binario:** los scripts heredados de Bash y Python *en tiempo de ejecución* se migraron a Rust. Los scripts de instalación y empaquetado (`install-*.{sh,ps1}`, `package-extension.py`) se conservaron a propósito, porque el instalador mismo debe funcionar antes de que se descargue el binario.
- **Normalización dinámica de rutas:** las rutas fijadas en el código (p. ej. `/home/jb_remus`) se adaptan dinámicamente al sistema operativo de destino y al directorio personal del usuario local.
- **Gestión de MCP con varios destinos (`--target`):**
  - Administrar las configuraciones de **Claude Code** (`~/.claude.json`), **proyecto** (`./.mcp.json`) y **Claude Desktop** (`claude_desktop_config.json`) desde una sola CLI.
  - Motor de escritura atómica que preserva los campos JSON sin tipar gracias a la estructura `serde_json::Value` (con copia de respaldo `.bak` automática).
- **Modo servidor MCP (`--mcp-mode`):** el mismo binario se convierte en un servidor MCP que habla JSON-RPC por stdin/stdout y ofrece a Claude Desktop las 8 herramientas declaradas en `manifest.json`.
- **Motor de memoria (SQLite + vectores + grafo):**
  - **Adición rápida de notas (`memory-note`):** creación segura de notas con nombres de archivo en estilo kebab-case.
  - **Búsqueda por palabras con FTS5:** indexación léxica de SQLite con mecanismo de escape de comillas.
  - **Representaciones vectoriales locales:** similitud del coseno mediante `fastembed` (Multilingual-E5-Small). El modelo se descarga de Hugging Face en el primer uso y se escribe en `$HOME/.claude/fastembed_cache`; **tras esa primera descarga**, la búsqueda funciona por completo sin conexión.
  - **Aristas del grafo y Wikilink:** búsqueda por vecindad (`memory-related`) mediante enlaces `[[Nombre-de-Nota]]` y vínculos semánticos por encima del umbral.
  - **Ordenación híbrida RRF:** fusión de los resultados de FTS5 y de la búsqueda vectorial con Reciprocal Rank Fusion (`k=60`).
- **Auditoría de seguridad con autocorrección (`security-audit --fix`):**
  - Rastreo de secretos en texto claro dentro de las configuraciones (`ghp_`, `github_pat_`, `sk-`, `xox[baprs]-`, `AKIA`).
  - Ajuste de los permisos de archivo a 600 — **solo en Unix**; en Windows se imprime una nota informativa sobre los permisos basados en ACL y no se aplica corrección alguna.
  - Instalación del gancho pre-commit de Git para proteger ramas y rastrear secretos.
- **Flujo de trabajo Git autónomo (`agent-workflow`):**
  - Creación automática de una rama de característica a partir de la rama predeterminada remota.
  - Bloqueo del envío (push) directo a las ramas principales protegidas.

---

## 🏗️ 2. Arquitectura y Módulos

```
claude-code-setup-rustified/
├── Cargo.toml                  # Dependencias de Rust y definiciones del paquete (v0.1.6)
├── manifest.json               # Manifiesto de la extensión de Claude Desktop (8 herramientas MCP)
├── icon.png                    # Icono de la extensión
├── .env.example                # Muestra de variables de entorno
├── src/
│   ├── main.rs                 # Punto de entrada de la CLI y despachador de órdenes (123 líneas)
│   ├── cli.rs                  # Definiciones de órdenes, destinos e indicadores con Clap (222)
│   ├── mcp.rs                  # Gestor MCP multidestino que preserva los valores JSON (488)
│   ├── mcp_server.rs           # Servidor MCP stdio JSON-RPC; asigna 8 herramientas a la CLI (436)
│   ├── memory_engine.rs        # Motor FTS5 + vectores + grafo + RRF + memory-note (821)
│   ├── installer.rs            # Instalador del esqueleto de directorios, README inicial y .env (191)
│   ├── security.rs             # Auditor de seguridad con autocorrección y gestor de ganchos (296)
│   ├── branch_manager.rs       # Ejecutor de flujo Git autónomo con protección de ramas (161)
│   ├── tester.rs               # Ejecutor de pruebas de diagnóstico del sistema y del entorno (123)
│   └── agent.rs                # Interfaz de integración del agente (30)
├── install-windows.ps1         # Instalador en PowerShell (NO es Rust)
├── install-macos.sh            # Instalador en Bash (NO es Rust)
├── package-extension.py        # Empaquetador .mcpb, invocado en CI (NO es Rust)
├── .github/workflows/
│   ├── rust.yml                # fmt + clippy + test + build (ubuntu/windows/macos)
│   └── release.yml             # Canal de publicación de binarios para 3 plataformas y .mcpb
└── docs/                       # Guías de instalación y resolución de problemas
```

### Responsabilidades de los Módulos
- `src/main.rs`: analiza los argumentos de la línea de órdenes; si se indica `--mcp-mode` cede el control al servidor MCP, y en caso contrario a la función del módulo correspondiente.
- `src/cli.rs`: mediante la estructura `Parser` de Clap administra 15 subórdenes, los indicadores (`--target`, `--fix`, `--hooks`, `--mode`, `--min-score`) y el indicador general `--mcp-mode`.
- `src/mcp.rs`: lee y actualiza los ajustes de MCP según el parámetro `--target` (`claude-code`, `project`, `claude-desktop`); garantiza escritura atómica sin eliminar los campos desconocidos.
- `src/mcp_server.rs`: establece el bucle JSON-RPC sobre stdin/stdout; asigna las 8 herramientas de `manifest.json` (`mcp_list`, `mcp_add`, `security_audit`, `memory_note`, `memory_index`, `memory_search`, `status`, `test`) a órdenes reales de la CLI. Esta asignación queda fijada por la prueba `her_arac_gercek_bir_cli_komutuna_esleniyor`.
- `src/memory_engine.rs`: divide las notas en ventanas de unos 1500 caracteres, calcula sus representaciones vectoriales y las promedia (mean-pooling); administra las tablas SQLite `knowledge_notes` y `note_edges`. La caché de representaciones está en `$HOME/.claude/fastembed_cache`.
- `src/installer.rs`: crea el directorio `$HOME/claude_global_memory/knowledge` y el archivo `README.md` inicial sin sobrescribirlos nunca; copia `.env` si no existe.
- `src/security.rs`: rastrea secretos en texto claro, comprueba los permisos, los corrige con `--fix` e instala el gancho de Git (el gancho es un script bash incorporado).
- `src/branch_manager.rs`: administra la creación autónoma de ramas, la barrera de ramas protegidas y los procesos seguros de commit/push.
- `src/tester.rs`: realiza el diagnóstico del sistema (`status`) y la verificación de pruebas.

---

## 🚀 3. Instalación y Configuración

### Inicio Rápido

Hay dos instalaciones distintas; decide cuál quieres.

**Extensión de Claude Desktop (recomendado)** — descarga el paquete correspondiente a tu sistema operativo desde la [última publicación](https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest) y arrástralo a la pantalla Claude Desktop → Settings → Extensions:

| Sistema operativo | Archivo | Tamaño aproximado |
|---|---|---|
| Windows | `claude-code-setup-windows.mcpb` | 9 MB |
| macOS | `claude-code-setup-macos.mcpb` | 10 MB |
| Linux | `claude-code-setup-linux.mcpb` | 12 MB |

**Herramienta de línea de órdenes** — si quieres usarla desde la terminal:

```powershell
irm https://raw.githubusercontent.com/Ercaner1988/claude-code-setup-rustified/main/install-windows.ps1 | iex
```

```bash
curl -fsSL https://raw.githubusercontent.com/Ercaner1988/claude-code-setup-rustified/main/install-macos.sh | bash
```

Estos instaladores son scripts de PowerShell y Bash (no de Rust); instalan el binario descargado en el directorio del usuario y lo añaden a PATH (no se requieren privilegios de administrador). **No registran** la extensión: para la extensión usa la vía `.mcpb` de arriba. Para verificar, ejecuta `claude-code-setup status` en una terminal nueva.

Para la instalación detallada, véase [INSTALLATION.es.md](INSTALLATION.es.md)

---

### Instalación Manual: Compilar desde el Código Fuente

#### Requisitos
- **Cadena de herramientas de Rust:** `rustc` y `cargo` (1.80 o superior)
- En la primera compilación, `fastembed` descarga el binario de ONNX Runtime → se requiere acceso a la red.

#### Compilación
```bash
cargo build --release

# Binario resultante:
# Windows: ./target/release/claude-code-setup.exe
# Linux/macOS: ./target/release/claude-code-setup
```

### Instalación Automática y Diagnóstico del Entorno
```bash
# Comprueba los requisitos previos e instala el esqueleto de memoria
./target/release/claude-code-setup install --hooks

# Estado de diagnóstico del sistema y del entorno
./target/release/claude-code-setup status
```

---

## 📖 4. Uso y Ejemplos

### Tabla Resumen de Órdenes

| Orden | Descripción |
| :--- | :--- |
| `--mcp-mode` (indicador general) | Ejecuta el binario como servidor MCP que habla JSON-RPC por stdin/stdout |
| `install [--hooks] [--skip-prereqs]` | Configuración del entorno, esqueleto de memoria y copia de `.env` |
| `test` / `status` | Diagnóstico de la CLI de Claude, `.claude.json`, la base de memoria y los ganchos |
| `mcp-list [--target T]` | Enumera los servidores MCP configurados según el destino |
| `mcp-set <srv> [--command C] [--arg A]… [--env K=V]… [--target T]` | Añade o actualiza un servidor MCP (`--target`: `claude-code`, `project`, `claude-desktop`) |
| `mcp-unset <srv> [--env K]… [--clear-args] [--remove] [--target T]` | Elimina variables o retira el servidor por completo (el indicador `--remove` es obligatorio) |
| `mcp-enable <srv>` / `mcp-disable <srv>` | Activa/desactiva el servidor sin dañar la configuración |
| `memory-note <título> [--body ...] [--dir D]` | Añade una nueva nota Markdown a la base de conocimiento |
| `memory-index [--source DIRECTORIO]… [--edge-threshold 0.70]` | Indexa las notas en el motor SQLite + vectores + grafo |
| `memory-search <consulta> [--mode keyword\|semantic\|hybrid] [--limit 5] [--min-score 0.30]` | Busca en la memoria en modo léxico FTS5, vectorial o híbrido RRF |
| `memory-related <note.md>` | Enumera las notas relacionadas mediante aristas del grafo y enlaces Wikilink |
| `install-hooks [--repo-dir RUTA]` | Instala el gancho de seguridad pre-commit en el repositorio |
| `security-audit [--fix]` | Realiza una auditoría de seguridad; aplica autocorrección con `--fix` |
| `agent-workflow [--branch-type TIPO] --description DESCRIPCIÓN [--files F]…` | Ejecuta el flujo autónomo de ramas y commits de Git con protección de ramas |

Todas las órdenes aceptan la anulación `--home-dir` para aislar las pruebas (excepto `install-hooks` y `agent-workflow`).

### Ejemplos de Uso

#### Administrar Servidores MCP por Destino
```bash
# Definir un servidor MCP a nivel de proyecto (.mcp.json)
./target/release/claude-code-setup mcp-set github \
  --command "npx" --arg "-y" --arg "@modelcontextprotocol/server-github" \
  --env "GITHUB_TOKEN=$GITHUB_TOKEN" --target project

# Desactivar el servidor en la configuración de Claude Desktop
./target/release/claude-code-setup mcp-disable github --target claude-desktop

# Retirar el servidor por completo (el indicador --remove es obligatorio)
./target/release/claude-code-setup mcp-unset github --remove --target claude-code
```

#### Añadir Notas a la Memoria y Búsqueda Híbrida RRF
```bash
./target/release/claude-code-setup memory-note "Decisiones de Arquitectura" --body "La migración del tiempo de ejecución a un binario nativo de Rust está completa."
./target/release/claude-code-setup memory-index --edge-threshold 0.70
./target/release/claude-code-setup memory-search "arquitectura Rust" --mode hybrid --limit 5 --min-score 0.30
./target/release/claude-code-setup memory-related mimari-kararlar.md
```

---

## 🛡️ 5. Pruebas y Puertas de Calidad

```bash
cargo test
# running 30 tests
# test result: ok. 30 passed; 0 failed; 0 ignored
```

En el código fuente hay **31 pruebas** definidas; una de ellas (`test_enforce_file_permissions_fixes_mode`) está marcada con `#[cfg(unix)]` y por tanto no se compila en Windows. Medición: **30/30 en verde en Windows y 31/31 en Unix** (2026-09-02).

Desglose por archivo: `memory_engine.rs` 14, `mcp.rs` 5, `mcp_server.rs` 5, `security.rs` 3, `branch_manager.rs` 2, `installer.rs` 2.

### Estándares de Calidad
- **Cobertura:** gestión MCP multidestino, preservación del `Value` de JSON, escape de caracteres en FTS5, ordenación híbrida RRF, mean-pooling, análisis de Wikilink, regresión de la ruta de caché de representaciones, rastreo de secretos, asignación de herramientas MCP a la CLI y barreras de ramas protegidas.
- **Formato:** `cargo fmt --all -- --check` → limpio (2026-09-02).
- **Análisis estático:** `cargo clippy --all-targets -- -D warnings` → sin advertencias (2026-09-02).
- **Integración continua:** `.github/workflows/rust.yml` ejecuta fmt + clippy + test + compilación de publicación en tres sistemas operativos (ubuntu, windows, macos). `.github/workflows/release.yml` produce los binarios de las tres plataformas y los paquetes `.mcpb`; ese canal usa Python y Node.

---

## 👥 6. Colaboradores

Las cifras se midieron con `git shortlog -sne --all` y contando las etiquetas `Co-authored-by` en los cuerpos de los commits (2026-09-02, 45 commits en total).

| Colaborador | Función / Responsabilidad | Aportación medida |
| :--- | :--- | :--- |
| **Ercan ER** | Arquitectura del proyecto, migración a Rust, desarrollador principal | 41 commits (autor) |
| **Claude Opus 5** | Agente de IA autónomo, desarrollo de módulos | 14 commits (coautor) |
| **Copilot App** | Asistente de programación con IA | 11 commits (coautor) |
| **Claude Opus 4.8** | Agente de IA autónomo | 3 commits (coautor) |
| **Claude** (versión sin especificar) | Agente de IA autónomo | 2 commits (coautor) |
| **jb_remus** | Autor original del proyecto de origen (upstream) | 2 commits (autor) |
| **Mihenk** | Revisor de código y árbitro de calidad | 1 commit (autor) |
| **arturo-ebuck** | Colaborador de código abierto | 1 commit (autor) |

**Kassam** es la identidad de agente registrada en el campo `authors` de `Cargo.toml`; no tiene un registro de autor propio en Git.

---

## 📄 7. Licencia y Recursos

Este proyecto está bajo la [Licencia MIT](LICENSE) (Derechos de autor © 2026 Ercan Er).

### Documentos Relacionados
- [Guía de Despliegue](DEPLOYMENT_GUIDE.md)
- [Guía de Instalación Manual](docs/MANUAL_SETUP.md)
- [Guía de Resolución de Problemas](docs/TROUBLESHOOTING.md)
- [Directivas para Desarrolladores](docs/dev/TASK-KASSAM-1-2.md)
