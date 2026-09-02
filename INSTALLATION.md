**🌍 [Türkçe](INSTALLATION.md) | [English](INSTALLATION.en.md) | [العربية](INSTALLATION.ar.md) | [日本語](INSTALLATION.ja.md) | [中文](INSTALLATION.zh.md) | [Русский](INSTALLATION.ru.md) | [Español](INSTALLATION.es.md)**

# Claude Code Bağımsız Kurulum Kılavuzu (Rust Çekirdekli Tek İkili)

Bu kılavuz, bilgisayarında **Rust veya teknik araçlar kurulu olmayan kullanıcılar dahil** herkesin **Claude Code Setup** (`claude-code-setup`) aracını saniyeler içinde kurup çalıştırabilmesi için adım adım hazırlanmıştır.

> **Dürüstlük notu:** Çalıştırılan ikili dosya saf Rust'tır ve kendi kendine yeterlidir. Ancak bu kılavuzdaki **kurucular Rust değildir**: `install-windows.ps1` bir PowerShell betiği, `install-macos.sh` bir Bash betiğidir. Sürüm paketleri de `package-extension.py` (Python) ile üretilir. GitHub dil istatistiği: **%90,5 Rust, %3,5 Shell, %3,2 Python, %2,8 PowerShell** (ölçülen satır payı: %91,2 Rust, %8,8 PowerShell + Bash + Python).

---

## 🚀 HIZLI BAŞLANGIÇ

Aracı kullanmanın iki yolu var. İkisi birbirinin alternatifi — ikisini de kurmak zorunda değilsin.

### Yol 1 — Claude Desktop eklentisi (önerilen)

Tek dosya indir, sürükle, bitti. Rust, terminal, PATH derdi yok.

1. [Son sürüm sayfasından](https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest) işletim sistemine uyan paketi indir:
   - **Windows** → `claude-code-setup-windows.mcpb`
   - **macOS** → `claude-code-setup-macos.mcpb`
   - **Linux** → `claude-code-setup-linux.mcpb`
2. Claude Desktop → **Settings → Extensions** ekranını aç.
3. İndirdiğin `.mcpb` dosyasını bu ekrana **sürükleyip bırak**.
4. Claude Desktop eklentiyi kurar ve 8 aracı kullanıma açar.

> Paketler platforma özeldir: her biri yalnızca kendi işletim sisteminin
> ikili dosyasını taşır. Yanlış paketi kurarsan eklenti yüklenir ama çalışmaz.

### Yol 2 — Komut satırı aracı (CLI)

Terminalden `claude-code-setup ...` komutlarını çalıştırmak istiyorsan.

#### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/Ercaner1988/claude-code-setup-rustified/main/install-windows.ps1 | iex
```

#### macOS / Linux (Terminal)
```bash
curl -fsSL https://raw.githubusercontent.com/Ercaner1988/claude-code-setup-rustified/main/install-macos.sh | bash
```

**Kurucunun yaptıkları:**
1. ✅ Son sürümün ikili dosyasını GitHub'dan indirir
2. ✅ Kullanıcı dizinine kurar (yönetici yetkisi gerekmez)
3. ✅ PATH'e ekler

**Kurucunun YAPMADIĞI:** Claude Desktop eklentisini kaydetmez. Eklenti için
Yol 1'i kullan — ikisi ayrı şeylerdir.

**Doğrulama** (yeni bir terminal penceresi aç, PATH değişikliği eskilerine yansımaz):
```bash
claude-code-setup --version
claude-code-setup status
```

---

## 📥 Manual Extension Kurulumu (İsteğe Bağlı)

Installerları kullanmak istemiyorsanız:

1. **Claude Code Desktop aç** → Settings → Extensions

2. **"Select extension"** → "Browse extensions folder"

3. **Klasör seç:**
   ```
   C:\Users\{username}\AppData\Roaming\Claude\Claude Extensions
   ```

4. **Yeni klasör oluştur:** `ercaner1988.claude-code-setup`

5. **Dosyaları indir ve kopyala:**
   - [manifest.json](manifest.json) (repo'dan)
   - [Release binary](https://github.com/Ercaner1988/claude-code-setup-rustified/releases) (.exe / macos / linux)
   - [README.md](README.md)
   - [LICENSE](LICENSE)

6. **"Install unpacked extension"** tıkla → Klasörü seç

7. **Claude Code'u restart et** (tamamen kapat & aç)

8. **Settings → Extensions** → "claude-code-setup" kontrol et

---

## 💡 ÖNEMLİ NOT: Rust Kurulu Olması Gerekir mi?

- **HAYIR! Hazır İkili Dosya (.exe) Kullanıyorsanız Rust ŞART DEĞİLDİR:**
  - Bu araç **bağımsız derlenmiş tek bir ikili dosyadır (Single Binary)**.
  - Aracı **çalıştırmak** için bilgisayarınızda Rust, Python veya ek bir betik dili derleyicisi kurulu olmak **zorunda değildir**.
  - Doğrudan `.exe` indirip çalıştırabilirsiniz.

- **YALNIZCA kaynak koddan kendiniz derlemek istiyorsanız** bilgisayarınızda Rust bulunmalıdır. (Aşağıda otomatik Rust kurulum adımı verilmiştir.)

- **Betik bağımlılıkları nerede duruyor?** Aracın kendisinde değil, çevresinde:
  - `install-windows.ps1` → Windows'ta PowerShell gerektirir (Windows'ta hazır gelir).
  - `install-macos.sh` → macOS/Linux'ta Bash gerektirir (ikisinde de hazır gelir).
  - `package-extension.py` → yalnız sürüm çıkarma sırasında (CI) Python gerektirir; son kullanıcıyı ilgilendirmez.
  - `security-audit`/`install-hooks` komutlarının kurduğu Git pre-commit kancası bir **bash betiğidir**; Windows'ta Git for Windows'un bash'i ile çalışır.
  - Semantik arama ilk kullanımda Hugging Face'ten gömme modelini ve ONNX Runtime ikilisini indirir; bundan sonrası çevrimdışıdır.

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

# Tanı doğrulama testlerini koşturun
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
