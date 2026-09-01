**🌍 [Türkçe](INSTALLATION.md) | [English](INSTALLATION.en.md) | [العربية](INSTALLATION.ar.md) | [日本語](INSTALLATION.ja.md) | [中文](INSTALLATION.zh.md) | [Русский](INSTALLATION.ru.md) | [Español](INSTALLATION.es.md)**

# Claude Code Bağımsız Kurulum Kılavuzu (%100 Rust Motoru)

Bu kılavuz, hiç komut satırı deneyimi olmayan kullanıcılar dahil herkesin **Claude Code Setup** (`claude-code-setup`) aracını saniyeler içinde bilgisayarına kurup çalıştırabilmesi için adım adım hazırlanmıştır.

---

## 🎯 1. Genel Bakış

- **Tek İkili Dosya (Single Binary):** Shell (`.sh`) ve Python betiği bağımlılığı tamamen kaldırılmıştır.
- **Çoklu Platform:** Windows (x64), Linux (x64) ve macOS (x64 / ARM64) üzerinde yerel hızda çalışır.
- **Sıfır Dış Bağımlılık:** Önceden derlenmiş ikili dosyayı indirerek veya `cargo` ile saniyeler içinde kurabilirsiniz.

---

## 📥 2. Adım Adım Kurulum (Yeni Başlayanlar İçin)

### 🪟 Windows Kullanıcıları İçin (Adım Adım)

1. **PowerShell'i Açın:**
   - Klavyenizden **`Windows Tuşu + R`** kombinasyonuna basın (Çalıştır penceresi açılır).
   - Kutucuğa `powershell` yazıp **Enter** tuşuna basın. (Mavi renkli komut penceresi açılacaktır).

2. **Tek Satırlık Kurulum Komutunu Yapıştırın:**
   - Aşağıdaki gri kutunun sağındaki kopyala butonuna tıklayın veya metni seçip kopyalayın.
   - Açılan mavi PowerShell penceresine **sağ tıklayarak yapıştırın** ve **Enter**'a basın:

```powershell
Invoke-WebRequest -Uri "https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-windows-x86_64.exe" -OutFile "claude-code-setup.exe"; .\claude-code-setup.exe install --hooks
```

3. **Tamamlandı!**
   - Kurulum aracı otomatik olarak çalışacak, dizin yapısını (`~/claude_global_memory/knowledge`) kuracak ve ortamı doğrulayacaktır.

---

### 🍏 macOS Kullanıcıları İçin (Adım Adım)

1. **Terminal'i Açın:**
   - Klavyenizden **`Cmd (⌘) + Space`** tuşlarına basarak Spotlight aramasını açın.
   - `Terminal` yazıp **Enter** tuşuna basın.

2. **Kurulum Komutunu Yapıştırın ve Çalıştırın:**
   - Aşağıdaki komutu kopyalayın, Terminal penceresine yapıştırıp **Enter** tuşuna basın:

```bash
curl -LO https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-macos-x86_64 && chmod +x claude-code-setup-macos-x86_64 && ./claude-code-setup-macos-x86_64 install --hooks
```

---

### 🐧 Linux Kullanıcıları İçin (Adım Adım)

1. **Terminal'i Açın:**
   - Klavyenizden **`Ctrl + Alt + T`** kısayoluna basın veya uygulama menüsünden **Terminal**'i seçin.

2. **Kurulum Komutunu Yapıştırın ve Çalıştırın:**
   - Aşağıdaki komutu yapıştırıp **Enter** tuşuna basın:

```bash
curl -LO https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-linux-x86_64 && chmod +x claude-code-setup-linux-x86_64 && ./claude-code-setup-linux-x86_64 install --hooks
```

---

## 🛠️ 3. Geliştiriciler İçin: Kaynak Koddan Derleme (Cargo)

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

---

## ⚙️ 4. Kurulum Sonrası Doğrulama ve Tanı

Kurulum tamamlandıktan sonra ortam tanı durumunu kontrol etmek için komut pencerenize yazabilirsiniz:

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
