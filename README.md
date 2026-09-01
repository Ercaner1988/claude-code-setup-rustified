**🌍 [Türkçe](README.md) | [English](README.en.md) | [العربية](README.ar.md) | [日本語](README.ja.md) | [中文](README.zh.md) | [Русский](README.ru.md) | [Español](README.es.md)**

# Claude Code Bağımsız Kurulum (%100 Rust Motoru)

[![Rust](https://img.shields.io/badge/Rust-100%25-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/Tests-24%20Passed-green.svg)]()

**Claude Code** ortamını yüksek performanslı, dinamik ve güvenli bir biçimde yönetmek için geliştirilmiş **%100 Rust tabanlı** yerel dağıtım, güvenlik denetimi ve hafıza motoru (`claude-code-setup.exe`).

Miras Bash (`.sh`) ve Python (`.py`) betiklerinin tamamı kaldırılarak tek bir yerel Rust ikili dosyasına dönüştürülmüştür.

---

## 🎯 1. Varış Noktası ve Tamamlananlar

- **%100 Saf Rust Mimarisi:** Kabuk (Shell) betikleri ve Python çalışma zamanı bağımlılıkları tamamen sıfırlanmıştır.
- **Dinamik Yol Normalizasyonu:** Sabit yol tanımları (ör. `/home/jb_remus`) hedef işletim sistemine ve yerel kullanıcı ev dizinine dinamik olarak uyarlanır.
- **Çoklu Hedef Destekli MCP Yönetimi (`--target`):**
  - **Claude Code** (`~/.claude.json`), **Proje** (`./.mcp.json`) ve **Claude Desktop** (`claude_desktop_config.json`) yapılandırmalarını aynı CLI üzerinden dinamik olarak yönetebilme.
  - `serde_json::Value` yapısı sayesinde tiplendirilmemiş veya modellenmemiş JSON alanlarını koruyan atomik yazım motoru (`.bak` otomatik yedekli).
- **Hafıza Motoru (SQLite + Vektör + Graf):**
  - **Hızlı Not Ekleme (`memory-note`):** Kebab-case dosya adlarıyla güvenli not oluşturma.
  - **FTS5 Kelime Araması:** SQLite kelime indeksleme ve tırnaklı kaçırma mekanizması.
  - **Yerel Gömme (Embeddings):** `fastembed` (Multilingual-E5-Small) ile tamamen çevrimdışı kosinüs benzerliği.
  - **Graf Kenarları ve Wikilink:** `[[Not-Adı]]` bağlantıları ve eşik üstü semantik bağlar üzerinden BFS komşuluk araması (`memory-related`).
  - **RRF Hibrit Sıralama:** Reciprocal Rank Fusion (`k=60`) algoritması ile FTS5 ve vektör aramalarının en doğru birleşimi.
- **Oto-Düzeltmeli Güvenlik Denetimi (`security-audit --fix`):**
  - Yapılandırmalarda açık metin gizli anahtar (token) taraması.
  - Unix sistemlerde dosya izinlerinin otomatik düzeltilmesi.
  - Git pre-commit dal koruma ve gizli anahtar tarama kancalarının otomatik kurulumu.
- **Otonom Git İş Akışı (`agent-workflow`):**
  - Uzaktaki varsayılan daldan otomatik özellik dalı (feature branch) türetme.
  - Korumalı ana dallara doğrudan push yapılmasını engelleme.

---

## 🏗️ 2. Mimari ve Modüller

```
claude-code-complete-setup/
├── Cargo.toml                  # Rust bağımlılıkları ve paket tanımları
├── src/
│   ├── main.rs                 # CLI giriş noktası ve komut yönlendirici
│   ├── cli.rs                  # Clap tabanlı komut, hedef ve bayrak tanımları
│   ├── mcp.rs                  # Çoklu hedef destekli JSON Value korumalı MCP yöneticisi
│   ├── memory_engine.rs        # FTS5 + Vektör + Graf + RRF + memory-note motoru
│   ├── installer.rs            # İskelet dizin, tohum README ve .env kurucusu
│   ├── security.rs             # Oto-düzeltmeli güvenlik denetçisi ve hook yöneticisi
│   ├── branch_manager.rs       # Korumalı dal güvenceli otonom Git iş akışçısı
│   ├── tester.rs               # Sistem ve ortam tanı testi koşturucusu
│   └── agent.rs                # Ajan entegrasyon arayüzü
└── docs/                       # Kurulum ve sorun giderme kılavuzları
```

### Modül Sorumlulukları
- `src/main.rs`: Komut satırı argümanlarını ayrıştırır ve ilgili modül işlevine aktarır.
- `src/cli.rs`: Clap `Parser` yapısıyla tüm alt komutları, bayrakları (`--target`, `--fix`, `--hooks`, `--mode`) ve yardım metinlerini yönetir.
- `src/mcp.rs`: MCP ayarlarını `--target` parametresine göre (`claude-code`, `project`, `claude-desktop`) okur ve günceller; bilinmeyen alanları silmeden atomik yazım sağlar.
- `src/memory_engine.rs`: Notları ~1500 karakterlik pencerelere bölerek gömer, ortalamasını alır (mean-pooling); `knowledge_notes` ve `note_edges` SQLite tablolarını yönetir. `memory-note` ile güvenli not ekler.
- `src/installer.rs`: `~/claude_global_memory/knowledge` dizinini ve tohum `README.md` dosyasını asla ezmeden oluşturur; `.env` yoksa kopyalar.
- `src/security.rs`: Açık metin sırları tarar, izinleri kontrol eder, `--fix` ile otomatik düzeltir ve Git kancalarını kurar.
- `src/branch_manager.rs`: Otonom dal oluşturma, korumalı dal engeli ve güvenli commit/push süreçlerini yönetir.
- `src/tester.rs`: Sistem tanılaması (`status`) ve test doğrulaması gerçekleştirir.

---

## 🚀 3. Kurulum ve Yapılandırma

### Hızlı Başlangıç: Claude Code Extension

#### Windows x64
```powershell
powershell -ExecutionPolicy Bypass -File install-windows.ps1
```

#### macOS x64
```bash
bash install-macos.sh
```

Installerlar otomatik olarak:
1. Latest release'i indir
2. Sisteme kur
3. PATH'e ekle
4. MCP yapılandırması yap
5. Claude Code extension'ı register et

👉 **Ardından:**
- Claude Code Desktop → Settings → Extensions
- "claude-code-setup" arayın
- "Configure" tıklayıp araçları gördüğünü doğrula

Detaylı kurulum için bkz. [INSTALLATION.md](INSTALLATION.md)

---

### Manuel Kurulum: Kaynaktan Derleme

#### Gereksinimler
- **Rust Toolchain:** `rustc` ve `cargo` (1.80+)

#### Derleme
```bash
# Projeyi derleyin
cargo build --release

# Oluşan ikili dosya:
# Windows: ./target/release/claude-code-setup.exe
# Linux/macOS: ./target/release/claude-code-setup
```

### Otomatik Kurulum ve Ortam Tanısı
```bash
# Otomatik kurulumu koşturun (ön koşulları kontrol eder, hafıza iskeletini kurar)
./target/release/claude-code-setup install --hooks

# Sistem ve ortam tanı durumunu kontrol edin
./target/release/claude-code-setup status
```

---

## 📖 4. Kullanım ve Örnekler

### Komut Özet Tablosu

| Komut | Açıklama |
| :--- | :--- |
| `install [--hooks]` | Ortam kurulumu, hafıza iskeleti ve `.env` kopyalama |
| `test` / `status` | Claude CLI, `.claude.json`, hafıza DB ve hook tanılaması |
| `mcp-list [--target T]` | Yapılandırılmış MCP sunucularını hedefe göre listeler |
| `mcp-set <srv> [...] [--target T]` | MCP sunucusu ekler veya günceller (`--target`: `claude-code`, `project`, `claude-desktop`) |
| `mcp-unset <srv> [...] [--remove] [--target T]` | Değişken siler veya sunucuyu tamamen kaldırır (`--remove` şarttır) |
| `mcp-enable <srv>` / `mcp-disable <srv>` | Yapılandırmayı bozmadan sunucuyu açar/kapatır |
| `memory-note <başlık> [--body ...]` | Bilgi tabanına yeni bir Markdown notu ekler |
| `memory-index [--source DIZIN]...` | Notları SQLite + Vektör + Graf motoruna indeksler |
| `memory-search <sorgu> [--mode ...]` | FTS5 Kelime, Vektör veya RRF Hibrit modunda hafıza araması yapar |
| `memory-related <not.md>` | Graf kenarları ve wikilink bağlantıları üzerinden ilişkili notları listeler |
| `install-hooks [--repo-dir YOL]` | Repoya pre-commit güvenlik hook'u kurar |
| `security-audit [--fix]` | Güvenlik denetimi yapar; `--fix` ile oto-düzeltme uygular |
| `agent-workflow [-t TÜR] -d AÇIKLAMA` | Korumalı dal güvenceli otonom Git dal ve commit iş akışını çalıştırır |

### Örnek Kullanım Senaryoları

#### MCP Sunucularını Hedefe Göre Yönetme
```bash
# Proje seviyesinde (.mcp.json) MCP sunucusu tanımlayın
./target/release/claude-code-setup mcp-set github --command "npx" --arg "-y" --arg "@modelcontextprotocol/server-github" --env "GITHUB_TOKEN=ghp_example" --target project

# Claude Desktop yapılandırmasındaki sunucuyu devre dışı bırakın
./target/release/claude-code-setup mcp-disable github --target claude-desktop

# Sunucuyu tamamen kaldırın (--remove bayrağı zorunludur)
./target/release/claude-code-setup mcp-unset github --remove --target claude-code
```

#### Hafızaya Not Ekleme ve RRF Hibrit Arama
```bash
# Yeni bir not ekleyin
./target/release/claude-code-setup memory-note "Mimari Kararlar" --body "Rust yerel ikili dosya dönüşümü tamamlandı."

# Notları indeksleyin
./target/release/claude-code-setup memory-index --edge-threshold 0.70

# RRF Hibrit Arama yapın
./target/release/claude-code-setup memory-search "Rust mimari" --mode hybrid --limit 5

# Semantik ilişkili notları inceleyin
./target/release/claude-code-setup memory-related mimari-kararlar.md
```

---

## 🛡️ 5. Test ve Kalite Kapıları

Projede 24 birim testi bulunmakta ve hepsi yeşil durumdadır:

```bash
cargo test
```

### Kalite Standartları
- **Birim Testleri (24/24 Geçti):** MCP çift hedef yönetimi, Value korunumu, FTS5 karakter kaçırma, RRF hibrit sıralama, mean-pooling, wikilink ayrıştırma, güvenlik gizli anahtar taraması ve korumalı dal engelleri.
- **Biçimlendirme:** `cargo fmt --check`
- **Sürekli Entegrasyon (CI):** Ubuntu, macOS ve Windows üzerinde `.github/workflows/rust.yml` ve `.github/workflows/release.yml` matrisi ile doğrulanır.

---

## 👥 6. Katkıda Bulunanlar

| Katkıda Bulunan | Rol / Sorumluluk | Metrikler |
| :--- | :--- | :--- |
| **Ercan ER** | Proje Mimarisi, Rust Dönüşümü ve Ana Geliştirici | 26 commit |
| **Kassam** | Otonom AI Ajanı, Rust Motoru ve Modül Geliştiricisi | Eş yazar / Katkıcı |
| **Copilot** | AI Kodlama Asistanı | 4 commit |
| **jb_remus** | Orijinal Üst Soy (Upstream) Yazan | 2 commit |
| **Mihenk** | Kod Denetçisi ve Kalite Hakemi | 1 commit |
| **arturo-ebuck** | Açık Kaynak Katkıcısı | 1 commit |

---

## 📄 7. Lisans ve Kaynaklar

Bu proje [MIT Lisansı](LICENSE) altında lisanslanmıştır.

### İlgili Dokümantasyon
- [Dağıtım Kılavuzu](DEPLOYMENT_GUIDE.md)
- [Manuel Kurulum Kılavuzu](docs/MANUAL_SETUP.md)
- [Sorun Giderme Kılavuzu](docs/TROUBLESHOOTING.md)
- [Geliştirici Direktifleri](docs/dev/TASK-KASSAM-1-2.md)
