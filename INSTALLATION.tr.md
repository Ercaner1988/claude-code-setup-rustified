**🌍 [Türkçe](INSTALLATION.md) | [English](INSTALLATION.en.md) | [العربية](INSTALLATION.ar.md) | [日本語](INSTALLATION.ja.md) | [中文](INSTALLATION.zh.md) | [Русский](INSTALLATION.ru.md) | [Español](INSTALLATION.es.md)**

# Claude Code Bağımsız Kurulum Kılavuzu (%100 Rust Motoru)

Bu kılavuz, **Claude Code** ortamınızı yöneten **%100 Rust tabanlı** yerel CLI aracını (`claude-code-setup`) farklı platformlarda adım adım nasıl kuracağınızı ve yapılandıracağınızı anlatır.

---

## 🎯 1. Genel Bakış

- **Tek İkili Dosya (Single Binary):** Shell (`.sh`) ve Python betiği bağımlılığı tamamen kaldırılmıştır.
- **Çoklu Platform:** Windows (x64), Linux (x64) ve macOS (x64 / ARM64) üzerinde yerel hızda çalışır.
- **Sıfır Dış Bağımlılık:** Önceden derlenmiş ikili dosyayı indirerek veya `cargo` ile saniyeler içinde kurabilirsiniz.

---

## 📥 2. Yöntem 1: Hazır İkili Dosya İndirme (Tavsiye Edilen)

GitHub Release sayfasından platformunuza uygun ikili dosyayı doğrudan indirebilirsiniz.

### Windows (x64)
PowerShell üzerinden doğrudan çalıştırılabilir ikili dosyayı indirin:
```powershell
# Sürüm ikili dosyasını indirin
Invoke-WebRequest -Uri "https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-windows-x86_64.exe" -OutFile "claude-code-setup.exe"

# Otomatik ortam ve hafıza kurulumunu başlatın
.\claude-code-setup.exe install --hooks
```

### Linux (x64)
Terminal üzerinden doğrudan indirin ve çalıştırma izni verin:
```bash
# İkili dosyayı indirin
curl -LO https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-linux-x86_64

# Çalıştırma izni verin
chmod +x claude-code-setup-linux-x86_64

# Kurulumu koşturun
./claude-code-setup-linux-x86_64 install --hooks
```

### macOS (x64)
```bash
# İkili dosyayı indirin
curl -LO https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-macos-x86_64

# Çalıştırma izni verin
chmod +x claude-code-setup-macos-x86_64

# Kurulumu koşturun
./claude-code-setup-macos-x86_64 install --hooks
```

---

## 🛠️ 3. Yöntem 2: Kaynak Koddan Derleme (Cargo)

Bilgisayarınızda Rust ortamı (`cargo` 1.80+) kuruluysa kaynak koddan derleyebilirsiniz:

```bash
# Repoyu klonlayın
git clone https://github.com/Ercaner1988/claude-code-setup-rustified.git
cd claude-code-setup-rustified

# Release ikilisini derleyin
cargo build --release

# Kurulumu koşturun
./target/release/claude-code-setup install --hooks
```

Sisteminize küresel olarak yüklemek isterseniz:
```bash
cargo install --path .
claude-code-setup install --hooks
```

---

## ⚙️ 4. Kurulum Sonrası Doğrulama ve Tanı

Kurulum tamamlandıktan sonra ortam tanı durumunu kontrol edin:

```bash
# Sistem tanılamasını koşturun
claude-code-setup status

# Tanı test takımlarını çalıştırın
claude-code-setup test
```

---

## 🛡️ 5. Güvenlik Denetimi ve Kanca (Hook) Kurulumu

Proje güvenlik izinlerini denetlemek ve Git pre-commit dal koruma kancalarını kurmak için:

```bash
# Otomatik düzeltmeli güvenlik denetimi
claude-code-setup security-audit --fix

# Pre-commit kancasını hedef repoya kurun
claude-code-setup install-hooks --repo-dir .
```

---

## 📚 6. İlgili Dokümanlar

- [Tam Dokümantasyon (README.md)](README.md)
- [Dağıtım Kılavuzu (DEPLOYMENT_GUIDE.md)](DEPLOYMENT_GUIDE.md)
- [Sorun Giderme Kılavuzu (TROUBLESHOOTING.md)](docs/TROUBLESHOOTING.md)
