# Claude Code Setup — Rustified

**Claude Code** ortamınızı yönetmek için tek dosyalık, **%100 Rust** CLI: dinamik MCP sunucu yönetimi, yerel semantik + graf hafıza motoru, güvenlik denetimi ve güvenli Git iş akışları.

English: [README.md](README.md) · العربية: [README.ar.md](README.ar.md)

## Özellikler

1. **Dinamik MCP yönetimi** — **Claude Code** (`~/.claude.json`), **proje** (`.mcp.json`) ve **Claude Desktop** yapılandırmalarını `--target` ile yönetin; düzenlemeler bilinmeyen alanları korur, atomik yazılır ve `.bak` yedeği bırakır.
2. **Yerel hafıza motoru** — Markdown notlarını SQLite'a indeksler: FTS5 anahtar kelime arama, yerel embedding'ler (fastembed ile Multilingual-E5-Small, tamamen çevrimdışı), RRF hibrit sıralama ve wikilink + semantik benzerlik grafiği (`memory-related`).
3. **Oto-düzeltmeli güvenlik denetimi** — yapılandırmalarda açık metin token taraması, Unix'te dosya izni düzeltme ve pre-commit dal-koruma/gizli-anahtar hook'ları (`security-audit --fix`).
4. **Güvenli otonom Git iş akışı** — `agent-workflow` uzaktaki varsayılan daldan özellik dalı açar ve korumalı dallara push'u reddeder; tüm git hataları yüzeye çıkar.

## Derleme ve Kullanım

Rust toolchain (1.80+) gerekir. Windows, Linux ve macOS'ta çalışır.

```bash
# İkili dosyayı derleyin
cargo build --release

# Ön koşulları doğrulayın, hafıza iskeletini ve .env'i kurun
./target/release/claude-code-setup.exe install

# Ortam tanılamasını çalıştırın
./target/release/claude-code-setup.exe status
```

## Komutlar

| Komut | Açıklama |
| :--- | :--- |
| `install [--hooks]` | Ön koşulları doğrular, `~/claude_global_memory/knowledge` oluşturur (tohum README, asla ezmez), `.env` yoksa `.env.example`'dan üretir |
| `test` / `status` | Ortam tanılaması: Claude CLI, `~/.claude.json`, hafıza DB, model önbelleği, hook'lar, ortam değişkenleri |
| `mcp-list [--target H]` | Yapılandırılmış MCP sunucularını listeler |
| `mcp-set <srv> [--command X] [--arg A]... [--env K=V]... [--target H]` | MCP sunucusu ekler/günceller |
| `mcp-unset <srv> [--env K]... [--clear-args] [--remove] [--target H]` | Alan kaldırır; sunucuyu silmek `--remove` gerektirir |
| `mcp-enable <srv>` / `mcp-disable <srv> [--target H]` | Yapılandırmayı silmeden açar/kapatır |
| `memory-note <başlık> [--body ...]` | Bilgi tabanına not ekler (kebab-case dosya adı, asla ezmez) |
| `memory-index [--source DİZİN]... [--edge-threshold 0.70]` | Notları SQLite'a indeksler (embedding + graf kenarları) |
| `memory-search <sorgu> [--mode keyword\|semantic\|hybrid] [--limit 5] [--min-score 0.30]` | İndekslenen notlarda arama (varsayılan: hibrit RRF) |
| `memory-related <not.md>` | Graf kenarları üzerinden ilişkili notları gösterir (BFS, 2 atlama) |
| `install-hooks [--repo-dir YOL]` | Repoya pre-commit güvenlik hook'u kurar |
| `security-audit [--fix]` | Gizli anahtar taraması, izin denetimi (Unix), hook denetimi, dal denetimi |
| `agent-workflow [-t TÜR] -d AÇIKLAMA [-f DOSYA]...` | Özellik dalı açar, dosyaları commit'ler, push'lar — korumalı dal güvencesiyle |

`--target` değerleri: `claude-code` (varsayılan, `~/.claude.json`), `project` (`./.mcp.json`), `claude-desktop` (`claude_desktop_config.json`).

## Hafıza Motoru Notları

- Varsayılan bilgi dizini: `~/claude_global_memory/knowledge` (`install` oluşturur; `memory-note` ile not ekleyin).
- Embedding modeli (~100 MB) ilk `memory-index`/`memory-search`'te iner ve yerel önbelleğe alınır; sonrasında tamamen çevrimdışı çalışır.
- Doğrusal kosinüs araması bu ölçekte bilinçli bir tercihtir; not sayısı binleri aşarsa ANN indeksi eklenecek yer kodda işaretlidir.

## Güvenlik

- Depoda gizli anahtar yoktur; `.env` git tarafından yok sayılır ve asla ezilmez.
- Her yapılandırma yazımı atomiktir (temp + rename) ve `.bak` yedeği bırakır.
- `mcp-unset <srv>` bayraksız çağrıyı reddeder — yıkıcı silme `--remove` gerektirir.
