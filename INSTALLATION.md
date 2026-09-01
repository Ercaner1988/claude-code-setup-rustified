# Kurulum Rehberi

## Windows x64

### Seçenek 1: Otomatik Installer (Recommended)

```powershell
powershell -ExecutionPolicy Bypass -File install-windows.ps1
```

**Yapacakları:**
- Latest release'i indir
- `C:\Program Files\ClaudeCodeSetup` klasörüne kur
- PATH'e ekle
- `.mcp.json` konfigürasyonunu hazırla

### Seçenek 2: Manuel İndirme

1. GitHub Releases'e git: https://github.com/Ercaner1988/claude-code-setup-rustified/releases
2. `claude-code-setup-windows-x86_64.exe` indir
3. `C:\Program Files\ClaudeCodeSetup` klasörüne taşı
4. PowerShell'i yeniden başlat
5. Çalıştır: `claude-code-setup install`

---

## macOS x64

### Seçenek 1: Otomatik Installer (Recommended)

```bash
bash install-macos.sh
```

**Yapacakları:**
- Latest release'i indir
- `~/.local/bin` klasörüne kur
- PATH'e ekle (zsh/bash)
- `.mcp.json` konfigürasyonunu hazırla

### Seçenek 2: Homebrew (Coming Soon)

```bash
brew install Ercaner1988/tap/claude-code-setup
```

### Seçenek 3: Manuel İndirme

```bash
# Download
curl -L https://github.com/Ercaner1988/claude-code-setup-rustified/releases/download/v0.1.0/claude-code-setup-macos-x86_64 -o /usr/local/bin/claude-code-setup
chmod +x /usr/local/bin/claude-code-setup

# Configure
claude-code-setup install
```

---

## Post-Installation

Her platform'da kurulumdan sonra:

```bash
# Version doğrula
claude-code-setup --version

# MCP sunucuları listele
claude-code-setup mcp-list

# Security audit çalıştır
claude-code-setup security-audit

# Memory engine test et
claude-code-setup memory-note "Hoşgeldiniz!"
```

---

## Sorun Giderme

### "claude-code-setup: command not found"
- **Windows:** PowerShell'i yeniden başlat
- **macOS:** `source ~/.zshrc` çalıştır (veya Terminal yeniden başlat)

### "Permission denied"
- **Windows:** Administrator olarak PowerShell aç
- **macOS:** `chmod +x ~/.local/bin/claude-code-setup`

### Build hatası
Bkz. [TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md)

---

## Geliştirici Kurulumu

Kaynaktan derlemek için (Rust 1.70+):

```bash
git clone https://github.com/Ercaner1988/claude-code-setup-rustified.git
cd claude-code-setup-rustified
cargo build --release
./target/release/claude-code-setup install
```
