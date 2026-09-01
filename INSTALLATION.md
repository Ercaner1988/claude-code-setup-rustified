**🌍 [Türkçe](INSTALLATION.md) | [English](INSTALLATION.en.md) | [العربية](INSTALLATION.ar.md) | [日本語](INSTALLATION.ja.md) | [中文](INSTALLATION.zh.md) | [Русский](INSTALLATION.ru.md) | [Español](INSTALLATION.es.md)**

# Claude Code Bağımsız Kurulum Kılavuzu (%100 Rust Motoru)

Bu kılavuz, bilgisayarında **Rust veya teknik araçlar kurulu olmayan kullanıcılar dahil** herkesin **Claude Code Setup** (`claude-code-setup`) aracını saniyeler içinde kurup çalıştırabilmesi için adım adım hazırlanmıştır.

---

## 💡 ÖNEMLİ NOT: Rust Kurulu Olması Gerekir mi?

- **HAYIR! Hazır İkili Dosya (.exe) Kullanıyorsanız Rust ŞART DEĞİLDİR:**
  - Bu araç **%100 bağımsız derlenmiş tek bir ikili dosyadır (Single Binary)**. 
  - Bilgisayarınızda Rust, Python veya ek bir betik dili derleyicisi kurulu olmak **zorunda değildir**.
  - Doğrudan `.exe` indirip çalıştırabilirsiniz.

- **YALNIZCA kaynak koddan kendiniz derlemek istiyorsanız** bilgisayarınızda Rust bulunmalıdır. (Aşağıda otomatik Rust kurulum adımı verilmiştir).

---

## 🛠️ 1. Ön Gereksinimler (İsteğe Bağlı & Otomatik Kurulumlar)

### A. Claude Code CLI (Ortamınızda Yoksa)
Eğer Claude Code CLI henüz bilgisayarınızda kurulu değilse Terminal/PowerShell'e şunu yazın:
```bash
npm install -g @anthropic/claude-code-cli
```

### B. Rust Toolchain (Yalnızca Kaynak Koddan Derleyecekler İçin)
Bilgisayarınıza Rust kurmak isterseniz:

- **Windows İçin (PowerShell):**
  ```powershell
  Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "rustup-init.exe"; .\rustup-init.exe -y
  ```

- **macOS / Linux İçin (Terminal):**
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  ```

---

## 📥 2. Adım Adım Hızlı Kurulum (Hazır İkili Dosya - En Kolay Yol)

### 🪟 Windows Kullanıcıları İçin (Adım Adım)

1. **PowerShell'i Açın:**
   - Klavyenizden **`Windows Tuşu + R`** kombinasyonuna basın (Çalıştır penceresi açılır).
   - Kutucuğa `powershell` yazıp **Enter** tuşuna basın. (Mavi renkli komut penceresi açılır).

2. **Tek Satırlık Kurulum Komutunu Yapıştırın ve Çalıştırın:**
   - Aşağıdaki komutu kopyalayın, PowerShell penceresine **sağ tıklayarak yapıştırın** ve **Enter**'a basın:

```powershell
Invoke-WebRequest -Uri "https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-windows-x86_64.exe" -OutFile "claude-code-setup.exe"; .\claude-code-setup.exe install --hooks
```

3. **Tamamlandı!**
   - İkili dosya inecek, otomatik olarak dizin yapısını (`~/claude_global_memory/knowledge`) oluşturacak ve ortamı doğrulayacaktır.

---

### 🍏 macOS Kullanıcıları İçin (Adım Adım)

1. **Terminal'i Açın:**
   - Klavyenizden **`Cmd (⌘) + Space`** tuşlarına basarak Spotlight aramasını açın.
   - `Terminal` yazıp **Enter** tuşuna basın.

2. **Kurulum Komutunu Yapıştırın ve Çalıştırın:**

```bash
curl -LO https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-macos-x86_64 && chmod +x claude-code-setup-macos-x86_64 && ./claude-code-setup-macos-x86_64 install --hooks
```

---

### 🐧 Linux Kullanıcıları İçin (Adım Adım)

1. **Terminal'i Açın:**
   - Klavyenizden **`Ctrl + Alt + T`** kısayoluna basın.

2. **Kurulum Komutunu Yapıştırın ve Çalıştırın:**

```bash
curl -LO https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-linux-x86_64 && chmod +x claude-code-setup-linux-x86_64 && ./claude-code-setup-linux-x86_64 install --hooks
```

---

## ⚙️ 3. Kurulum Sonrası Doğrulama ve Tanı

```bash
# Ortam tanılamasını çalıştırın
claude-code-setup status

# Tanı doğrulama testlerini коşturun
claude-code-setup test
```

---

## 🛡️ 4. Güvenlik Denetimi ve Kanca (Hook) Kurulumu

```bash
# Otomatik düzeltmeli güvenlik denetimi
claude-code-setup security-audit --fix

# Pre-commit kancasını hedef repoya kurun
claude-code-setup install-hooks --repo-dir .
```

---

## 📚 5. İlgili Dokümanlar

- [Tam Dokümantasyon (README.md)](README.md)
- [Dağıtım Kılavuzu (DEPLOYMENT_GUIDE.md)](DEPLOYMENT_GUIDE.md)
- [Sorun Giderme Kılavuzu (TROUBLESHOOTING.md)](docs/TROUBLESHOOTING.md)
