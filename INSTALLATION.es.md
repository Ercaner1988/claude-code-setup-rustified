**🌍 [Türkçe](INSTALLATION.md) | [English](INSTALLATION.en.md) | [العربية](INSTALLATION.ar.md) | [日本語](INSTALLATION.ja.md) | [中文](INSTALLATION.zh.md) | [Русский](INSTALLATION.ru.md) | [Español](INSTALLATION.es.md)**

# Guía de Instalación Independiente de Claude Code (Motor 100% Rust)

Esta guía proporciona instrucciones paso a paso para instalar y configurar la herramienta CLI **Claude Code Setup** (`claude-code-setup`), **100% nativa en Rust**, en diferentes plataformas.

---

## 🎯 1. Descripción General

- **Un solo binario (Single Binary):** Eliminación total de dependencias de scripts Shell (`.sh`) y Python (`.py`).
- **Multiplataforma:** Rendimiento nativo en Windows (x64), Linux (x64) y macOS (x64 / ARM64).
- **Cero dependencias externas:** Instale descargando los binarios precompilados o mediante `cargo` en segundos.

---

## 📥 2. Método 1: Descarga de Binarios Precompilados (Recomendado)

Descargue el archivo ejecutable correspondiente a su sistema operativo directamente desde la página de versiones en GitHub Releases.

### Windows (x64)
Descargue y ejecute vía PowerShell:
```powershell
# Descargar el ejecutable del lanzamiento
Invoke-WebRequest -Uri "https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-windows-x86_64.exe" -OutFile "claude-code-setup.exe"

# Ejecutar la instalación automática
.\claude-code-setup.exe install --hooks
```

### Linux (x64)
```bash
# Descargar el binario
curl -LO https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-linux-x86_64

# Dar permisos de ejecución
chmod +x claude-code-setup-linux-x86_64

# Ejecutar la instalación
./claude-code-setup-linux-x86_64 install --hooks
```

### macOS (x64)
```bash
# Descargar el binario
curl -LO https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-macos-x86_64

# Dar permisos de ejecución
chmod +x claude-code-setup-macos-x86_64

# Ejecutar la instalación
./claude-code-setup-macos-x86_64 install --hooks
```

---

## 🛠️ 3. Método 2: Compilación desde el Código Fuente (Cargo)

Si su sistema dispone del entorno Rust (`cargo` 1.80+):

```bash
# Clonar el repositorio
git clone https://github.com/Ercaner1988/claude-code-setup-rustified.git
cd claude-code-setup-rustified

# Compilar el binario de producción
cargo build --release

# Ejecutar la instalación
./target/release/claude-code-setup install --hooks
```

Para instalarlo globalmente en su sistema:
```bash
cargo install --path .
claude-code-setup install --hooks
```

---

## ⚙️ 4. Verificación y Diagnóstico Posterior a la Instalación

```bash
# Diagnóstico del entorno
claude-code-setup status

# Suite de pruebas de diagnóstico
claude-code-setup test
```

---

## 🛡️ 5. Auditoría de Seguridad y Configuración de Ganchos Git

```bash
# Auditoría de seguridad con auto-reparación
claude-code-setup security-audit --fix

# Instalar el gancho pre-commit en un repositorio
claude-code-setup install-hooks --repo-dir .
```

---

## 📚 6. Documentación Relacionada

- [Documentación Completa (README.md)](README.md)
- [Guía de Despliegue (DEPLOYMENT_GUIDE.md)](DEPLOYMENT_GUIDE.md)
- [Guía de Resolución de Problemas (TROUBLESHOOTING.md)](docs/TROUBLESHOOTING.md)
