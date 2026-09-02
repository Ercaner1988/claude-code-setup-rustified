**🌍 [Türkçe](README.md) | [English](README.en.md) | [العربية](README.ar.md) | [日本語](README.ja.md) | [中文](README.zh.md) | [Русский](README.ru.md) | [Español](README.es.md)**

# Claude Code Bağımsız Kurulum (Rust Çekirdekli Tek İkili)

[![Rust](https://img.shields.io/badge/Rust%20çekirdek-%2591-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/Tests-30%20Passed-green.svg)]()

**Claude Code** ortamını yönetmek için geliştirilmiş yerel dağıtım, güvenlik denetimi ve hafıza motoru (`claude-code-setup`). Çalışma zamanı tek bir Rust ikili dosyasıdır; kullanmak için makinede Rust, Python veya Node kurulu olması gerekmez.

### Dürüstlük notu: bu depo %100 Rust değildir

GitHub dil istatistiği ve ölçülen kod dağılımı (2026-09-02): **2891 satır Rust / 279 satır Rust dışı = %91,2 satır payı** (GitHub Linguist: **%90,5 Rust, %3,5 Shell, %3,2 Python, %2,8 PowerShell**).

| Dil / Dosya | Satır | GitHub Payı | Ne zaman çalışır |
| :--- | ---: | ---: | :--- |
| `Rust` (`src/*.rs`, 10 dosya) | 2891 | %90,5 | Çalışma zamanı (CLI + MCP sunucusu) |
| `install-macos.sh` (Shell / Bash) | 121 | %3,5 | Linux/macOS kurulumunda |
| `package-extension.py` (Python) | 87 | %3,2 | Yalnız sürüm çıkarma / `.mcpb` paketleme (CI) |
| `install-windows.ps1` (PowerShell) | 71 | %2,8 | Windows kurulumunda |

Ek bağımlılıklar:
- `src/security.rs` içindeki pre-commit kancası **gömülü bir bash betiği** olarak yazılır (`#!/usr/bin/env bash`) — kancanın çalışması için Git'in bash'i gerekir.
- `.github/workflows/release.yml` sürüm hattı `actions/setup-python` ve `npx @anthropic-ai/mcpb validate` kullanır → **CI zinciri Python + Node'a bağlıdır**.
- Hafıza motorunun gömme katmanı `fastembed` üzerinden **ONNX Runtime'ın önceden derlenmiş C++ ikilisini indirir** (`ort-download-binaries`).

Doğru özet: **çalışma zamanı ikilisi saf Rust; kurulum, paketleme ve sürüm hattı Bash + PowerShell + Python + Node kullanır.**

---

## 🎯 1. Varış Noktası ve Tamamlananlar

- **Tek İkili Çalışma Zamanı:** Miras Bash ve Python *çalışma zamanı* betikleri Rust'a taşındı. Kurulum ve paketleme betikleri (`install-*.{sh,ps1}`, `package-extension.py`) bilinçli olarak korundu; çünkü kurucunun kendisi ikili indirilmeden önce çalışmak zorundadır.
- **Dinamik Yol Normalizasyonu:** Sabit yol tanımları (ör. `/home/jb_remus`) hedef işletim sistemine ve yerel kullanıcı ev dizinine dinamik olarak uyarlanır.
- **Çoklu Hedef Destekli MCP Yönetimi (`--target`):**
  - **Claude Code** (`~/.claude.json`), **Proje** (`./.mcp.json`) ve **Claude Desktop** (`claude_desktop_config.json`) yapılandırmalarını aynı CLI üzerinden yönetebilme.
  - `serde_json::Value` yapısı sayesinde tiplendirilmemiş JSON alanlarını koruyan atomik yazım motoru (`.bak` otomatik yedekli).
- **MCP Sunucu Kipi (`--mcp-mode`):** Aynı ikili, stdin/stdout üzerinden JSON-RPC konuşan bir MCP sunucusuna dönüşür ve `manifest.json`'daki 8 aracı Claude Desktop'a sunar.
- **Hafıza Motoru (SQLite + Vektör + Graf):**
  - **Hızlı Not Ekleme (`memory-note`):** Kebab-case dosya adlarıyla güvenli not oluşturma.
  - **FTS5 Kelime Araması:** SQLite kelime indeksleme ve tırnaklı kaçırma mekanizması.
  - **Yerel Gömme:** `fastembed` (Multilingual-E5-Small) ile kosinüs benzerliği. Model ilk kullanımda Hugging Face'ten indirilip `$HOME/.claude/fastembed_cache` altına yazılır; **bu ilk indirmeden sonra** arama tamamen çevrimdışı çalışır.
  - **Graf Kenarları ve Wikilink:** `[[Not-Adı]]` bağlantıları ve eşik üstü semantik bağlar üzerinden komşuluk araması (`memory-related`).
  - **RRF Hibrit Sıralama:** Reciprocal Rank Fusion (`k=60`) ile FTS5 ve vektör aramalarının birleşimi.
- **Oto-Düzeltmeli Güvenlik Denetimi (`security-audit --fix`):**
  - Yapılandırmalarda açık metin gizli anahtar taraması (`ghp_`, `github_pat_`, `sk-`, `xox[baprs]-`, `AKIA`).
  - Dosya izinlerinin 600'e çekilmesi — **yalnız Unix'te**; Windows'ta ACL tabanlı izinler için bilgi notu basılır, düzeltme yapılmaz.
  - Git pre-commit dal koruma ve gizli anahtar tarama kancasının kurulumu.
- **Otonom Git İş Akışı (`agent-workflow`):**
  - Uzaktaki varsayılan daldan otomatik özellik dalı türetme.
  - Korumalı ana dallara doğrudan push yapılmasını engelleme.

---

## 🏗️ 2. Mimari ve Modüller

```
claude-code-setup-rustified/
├── Cargo.toml                  # Rust bağımlılıkları ve paket tanımları (v0.1.6)
├── manifest.json               # Claude Desktop eklenti bildirimi (8 MCP aracı)
├── icon.png                    # Eklenti simgesi
├── .env.example                # Örnek ortam değişkenleri
├── src/
│   ├── main.rs                 # CLI giriş noktası ve komut yönlendirici (123 satır)
│   ├── cli.rs                  # Clap tabanlı komut, hedef ve bayrak tanımları (222)
│   ├── mcp.rs                  # Çoklu hedef destekli JSON Value korumalı MCP yöneticisi (488)
│   ├── mcp_server.rs           # MCP stdio JSON-RPC sunucusu; 8 aracı CLI'ye eşler (436)
│   ├── memory_engine.rs        # FTS5 + Vektör + Graf + RRF + memory-note motoru (821)
│   ├── installer.rs            # İskelet dizin, tohum README ve .env kurucusu (191)
│   ├── security.rs             # Oto-düzeltmeli güvenlik denetçisi ve kanca yöneticisi (296)
│   ├── branch_manager.rs       # Korumalı dal güvenceli otonom Git iş akışçısı (161)
│   ├── tester.rs               # Sistem ve ortam tanı testi koşturucusu (123)
│   └── agent.rs                # Ajan bütünleşme arayüzü (30)
├── install-windows.ps1         # PowerShell kurucu (Rust DEĞİL)
├── install-macos.sh            # Bash kurucu (Rust DEĞİL)
├── package-extension.py        # .mcpb paketleyici, CI'da çağrılır (Rust DEĞİL)
├── .github/workflows/
│   ├── rust.yml                # fmt + clippy + test + build (ubuntu/windows/macos)
│   └── release.yml             # 3 platform ikili + .mcpb sürüm hattı
└── docs/                       # Kurulum ve sorun giderme kılavuzları
```

### Modül Sorumlulukları
- `src/main.rs`: Komut satırı argümanlarını ayrıştırır; `--mcp-mode` verildiyse denetimi MCP sunucusuna, verilmediyse ilgili modül işlevine aktarır.
- `src/cli.rs`: Clap `Parser` yapısıyla 15 alt komutu, bayrakları (`--target`, `--fix`, `--hooks`, `--mode`, `--min-score`) ve genel `--mcp-mode` bayrağını yönetir.
- `src/mcp.rs`: MCP ayarlarını `--target` parametresine göre (`claude-code`, `project`, `claude-desktop`) okur ve günceller; bilinmeyen alanları silmeden atomik yazım sağlar.
- `src/mcp_server.rs`: stdin/stdout JSON-RPC döngüsünü kurar; `manifest.json`'daki 8 aracı (`mcp_list`, `mcp_add`, `security_audit`, `memory_note`, `memory_index`, `memory_search`, `status`, `test`) gerçek CLI komutlarına eşler. Bu eşleme `her_arac_gercek_bir_cli_komutuna_esleniyor` testiyle kilitlidir.
- `src/memory_engine.rs`: Notları ~1500 karakterlik pencerelere bölerek gömer, ortalamasını alır (mean-pooling); `knowledge_notes` ve `note_edges` SQLite tablolarını yönetir. Gömme önbelleği `$HOME/.claude/fastembed_cache`.
- `src/installer.rs`: `$HOME/claude_global_memory/knowledge` dizinini ve tohum `README.md` dosyasını asla ezmeden oluşturur; `.env` yoksa kopyalar.
- `src/security.rs`: Açık metin sırları tarar, izinleri kontrol eder, `--fix` ile düzeltir ve Git kancasını kurar (kanca gömülü bir bash betiğidir).
- `src/branch_manager.rs`: Otonom dal oluşturma, korumalı dal engeli ve güvenli commit/push süreçlerini yönetir.
- `src/tester.rs`: Sistem tanılaması (`status`) ve test doğrulaması yapar.

---

## 🚀 3. Kurulum ve Yapılandırma

### Hızlı Başlangıç

İki ayrı kurulum var; hangisini istediğine karar ver.

**Claude Desktop eklentisi (önerilen)** — [son sürümden](https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest) işletim sistemine uyan paketi indir, Claude Desktop → Settings → Extensions ekranına sürükle:

| İşletim sistemi | Dosya | Yaklaşık boyut |
|---|---|---|
| Windows | `claude-code-setup-windows.mcpb` | 9 MB |
| macOS | `claude-code-setup-macos.mcpb` | 10 MB |
| Linux | `claude-code-setup-linux.mcpb` | 12 MB |

**Komut satırı aracı** — terminalden kullanmak istersen:

```powershell
irm https://raw.githubusercontent.com/Ercaner1988/claude-code-setup-rustified/main/install-windows.ps1 | iex
```

```bash
curl -fsSL https://raw.githubusercontent.com/Ercaner1988/claude-code-setup-rustified/main/install-macos.sh | bash
```

Bu kurucular PowerShell ve Bash betikleridir (Rust değil); indirdikleri ikili dosyayı kullanıcı dizinine kurup PATH'e ekler (yönetici yetkisi gerekmez). Eklentiyi **kaydetmez** — eklenti için yukarıdaki `.mcpb` yolunu kullan. Doğrulama için yeni bir terminalde `claude-code-setup status`.

Detaylı kurulum için bkz. [INSTALLATION.md](INSTALLATION.md)

---

### Manuel Kurulum: Kaynaktan Derleme

#### Gereksinimler
- **Rust Toolchain:** `rustc` ve `cargo` (1.80+)
- İlk derlemede `fastembed`, ONNX Runtime ikilisini indirir → ağ erişimi gerekir.

#### Derleme
```bash
cargo build --release

# Oluşan ikili dosya:
# Windows: ./target/release/claude-code-setup.exe
# Linux/macOS: ./target/release/claude-code-setup
```

### Otomatik Kurulum ve Ortam Tanısı
```bash
# Ön koşulları kontrol eder, hafıza iskeletini kurar
./target/release/claude-code-setup install --hooks

# Sistem ve ortam tanı durumu
./target/release/claude-code-setup status
```

---

## 📖 4. Kullanım ve Örnekler

### Komut Özet Tablosu

| Komut | Açıklama |
| :--- | :--- |
| `--mcp-mode` (genel bayrak) | İkiliyi stdin/stdout JSON-RPC konuşan MCP sunucusu olarak çalıştırır |
| `install [--hooks] [--skip-prereqs]` | Ortam kurulumu, hafıza iskeleti ve `.env` kopyalama |
| `test` / `status` | Claude CLI, `.claude.json`, hafıza DB ve kanca tanılaması |
| `mcp-list [--target T]` | Yapılandırılmış MCP sunucularını hedefe göre listeler |
| `mcp-set <srv> [--command C] [--arg A]… [--env K=V]… [--target T]` | MCP sunucusu ekler veya günceller (`--target`: `claude-code`, `project`, `claude-desktop`) |
| `mcp-unset <srv> [--env K]… [--clear-args] [--remove] [--target T]` | Değişken siler veya sunucuyu tamamen kaldırır (`--remove` şarttır) |
| `mcp-enable <srv>` / `mcp-disable <srv>` | Yapılandırmayı bozmadan sunucuyu açar/kapatır |
| `memory-note <başlık> [--body ...] [--dir D]` | Bilgi tabanına yeni bir Markdown notu ekler |
| `memory-index [--source DIZIN]… [--edge-threshold 0.70]` | Notları SQLite + Vektör + Graf motoruna indeksler |
| `memory-search <sorgu> [--mode keyword\|semantic\|hybrid] [--limit 5] [--min-score 0.30]` | FTS5 kelime, vektör veya RRF hibrit modunda hafıza araması yapar |
| `memory-related <not.md>` | Graf kenarları ve wikilink bağlantıları üzerinden ilişkili notları listeler |
| `install-hooks [--repo-dir YOL]` | Repoya pre-commit güvenlik kancası kurar |
| `security-audit [--fix]` | Güvenlik denetimi yapar; `--fix` ile oto-düzeltme uygular |
| `agent-workflow [--branch-type TÜR] --description AÇIKLAMA [--files F]…` | Korumalı dal güvenceli otonom Git dal ve commit iş akışını çalıştırır |

Tüm komutlar test yalıtımı için `--home-dir` geçersiz kılmasını kabul eder (`install-hooks` ve `agent-workflow` hariç).

### Örnek Kullanım Senaryoları

#### MCP Sunucularını Hedefe Göre Yönetme
```bash
# Proje seviyesinde (.mcp.json) MCP sunucusu tanımla
./target/release/claude-code-setup mcp-set github \
  --command "npx" --arg "-y" --arg "@modelcontextprotocol/server-github" \
  --env "GITHUB_TOKEN=$GITHUB_TOKEN" --target project

# Claude Desktop yapılandırmasındaki sunucuyu devre dışı bırak
./target/release/claude-code-setup mcp-disable github --target claude-desktop

# Sunucuyu tamamen kaldır (--remove bayrağı zorunludur)
./target/release/claude-code-setup mcp-unset github --remove --target claude-code
```

#### Hafızaya Not Ekleme ve RRF Hibrit Arama
```bash
./target/release/claude-code-setup memory-note "Mimari Kararlar" --body "Rust yerel ikili dönüşümü tamamlandı."
./target/release/claude-code-setup memory-index --edge-threshold 0.70
./target/release/claude-code-setup memory-search "Rust mimari" --mode hybrid --limit 5 --min-score 0.30
./target/release/claude-code-setup memory-related mimari-kararlar.md
```

---

## 🛡️ 5. Test ve Kalite Kapıları

```bash
cargo test
# running 30 tests
# test result: ok. 30 passed; 0 failed; 0 ignored
```

Kaynakta **31 test** tanımlıdır; biri (`test_enforce_file_permissions_fixes_mode`) `#[cfg(unix)]` ile işaretli olduğu için Windows'ta derlenmez. Ölçüm: **Windows'ta 30/30, Unix'te 31/31 yeşil** (2026-09-02).

Dosya kırılımı: `memory_engine.rs` 14, `mcp.rs` 5, `mcp_server.rs` 5, `security.rs` 3, `branch_manager.rs` 2, `installer.rs` 2.

### Kalite Standartları
- **Kapsam:** MCP çoklu hedef yönetimi, JSON `Value` korunumu, FTS5 karakter kaçırma, RRF hibrit sıralama, mean-pooling, wikilink ayrıştırma, gömme önbellek yolu regresyonu, gizli anahtar taraması, MCP araç–CLI eşlemesi ve korumalı dal engelleri.
- **Biçimlendirme:** `cargo fmt --all -- --check` → temiz (2026-09-02).
- **Lint:** `cargo clippy --all-targets -- -D warnings` → uyarı yok (2026-09-02).
- **Sürekli Bütünleşme:** `.github/workflows/rust.yml` üç işletim sisteminde (ubuntu, windows, macos) fmt + clippy + test + release derlemesi koşturur. `.github/workflows/release.yml` üç platform ikilisini ve `.mcpb` paketlerini üretir; bu hat Python ve Node kullanır.

---

## 👥 6. Katkıda Bulunanlar

Sayılar `git shortlog -sne --all` ve commit gövdesindeki `Co-authored-by` etiketlerinin sayımıyla ölçüldü (2026-09-02, toplam 45 commit).

| Katkıda Bulunan | Rol / Sorumluluk | Ölçülen katkı |
| :--- | :--- | :--- |
| **Ercan ER** | Proje mimarisi, Rust dönüşümü, ana geliştirici | 41 commit (yazar) |
| **Claude Opus 5** | Otonom yapay zekâ ajanı, modül geliştirme | 14 commit (eş yazar) |
| **Copilot App** | Yapay zekâ kodlama yardımcısı | 11 commit (eş yazar) |
| **Claude Opus 4.8** | Otonom yapay zekâ ajanı | 3 commit (eş yazar) |
| **Claude** (sürüm belirtilmemiş) | Otonom yapay zekâ ajanı | 2 commit (eş yazar) |
| **jb_remus** | Özgün üst soy (upstream) yazan | 2 commit (yazar) |
| **Mihenk** | Kod denetçisi ve kalite hakemi | 1 commit (yazar) |
| **arturo-ebuck** | Açık kaynak katkıcısı | 1 commit (yazar) |

**Kassam**, `Cargo.toml` `authors` alanında kayıtlı ajan kimliğidir; ayrı bir Git yazar kaydı yoktur.

---

## 📄 7. Lisans ve Kaynaklar

Bu proje [MIT Lisansı](LICENSE) altında lisanslanmıştır (Telif hakkı © 2026 Ercan Er).

### İlgili Belgeler
- [Dağıtım Kılavuzu](DEPLOYMENT_GUIDE.md)
- [Manuel Kurulum Kılavuzu](docs/MANUAL_SETUP.md)
- [Sorun Giderme Kılavuzu](docs/TROUBLESHOOTING.md)
- [Geliştirici Direktifleri](docs/dev/TASK-KASSAM-1-2.md)
