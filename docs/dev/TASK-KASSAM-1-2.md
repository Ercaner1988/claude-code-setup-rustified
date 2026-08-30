# Kassam Görev Direktifi — Özellik 1 (Dinamik MCP) + Özellik 2 (Semantik + Graph Memory)

Yönlendiren: Claude (Ercan onayladı, 2026). Uygulayan: **Kassam**. İnceleyen: Claude.
Bu dosya kendine-yeter — başka bir yere bakmana gerek yok.

## ÖNCE ÇÖZ (kod yazmadan yanıtla)
1. **Özellik 3 durumu:** Bu ağaçta `src/security.rs::run_security_audit` hâlâ **yalnız-rapor** (oto-fix yok). "Hallettik" denen oto-fix hangi branch'te? Merge/rebuild edilmediyse bu binary'de yok. Teyit et; gerekiyorsa main'e getir.
2. **Embedding motoru:** `fastembed` (onnxruntime C++ runtime iner) mi, yoksa `candle` (saf-Rust, "100% Rust" kimliğine sadık) mı? Öneri: fastembed (en kısa yol); onnxruntime bağımlılığı kabul değilse candle.

## Kurallar (ponytail — inceleme bunları zorlayacak)
- Mevcut yapıyı YENİDEN KULLAN: `installer::get_home_dir`, `mcp::McpServerConfig`, `memory_engine` SQLite bağlantı deseni.
- Tek-implementasyonlu soyutlama YOK; graph DB YOK; ANN index YOK (bu ölçekte); ayrı vektör-store sunucusu YOK; yeni DB dosyası YOK.
- Önemsiz-olmayan her parça **1 koşulabilir test** bırakır (`cargo test`).

---

## Özellik 2 — Semantik + Graph Memory
Dosyalar: `src/memory_engine.rs`, `src/cli.rs`, `src/main.rs`, `Cargo.toml`.

1. Embedding = **yerel, çevrimdışı** (yukarıdaki karar). MiniLM-L6-v2. Uzak API/OpenAI YOK.
2. Depo = **mevcut** `~/.claude/memory_index.db`. `knowledge_notes`'a `embedding BLOB` kolonu ekle; yeni tablo `note_edges(src TEXT, dst TEXT, tur TEXT, agirlik REAL)`.
3. Semantik arama = **brute-force kosinüs** (lineer). Yorum ekle: `// ponytail: lineer kosinüs; not > ~5k olursa ANN ekle`.
4. Graph kenarları: (a) `[[wikilink]]` ayrıştır (deterministik); (b) kosinüs > eşik → semantik kenar. Komşu/en-kısa-yol = Rust BFS.
5. Keyword yolu: mevcut `LIKE`'ı **FTS5**'e yükselt (bundled SQLite destekler, 0 yeni bağımlılık).
6. CLI: `memory-index` embedding+kenar üretir; `memory-search <q> [--semantic|--keyword|--hybrid]`; yeni `memory-related <note>`.

Kabul kriteri: geçici knowledge dizini indeksle → `--semantic` sıralı sonuç; `memory-related` wikilink komşuları; FTS5 keyword çalışır; kosinüs birim testi geçer.

---

## Özellik 1 — Dinamik MCP Parametre Yönetimi
Dosyalar: `src/mcp.rs`, `src/cli.rs`, `src/main.rs`. İki-konum tespitini paylaşılan `resolve_config_path()`'e çıkar.

1. CLI: `mcp-set <server> [--command X] [--arg A]... [--env K=V]...`, `mcp-unset <server> [--env K] [--arg ...]`, `mcp-enable/--disable <server>`. `mcp-list` kalır.
2. **KRİTİK:** düzenlemeyi `serde_json::Value` üstünden yap (bkz. `normalize_mcp_config`), **tipli-struct round-trip DEĞİL** — yoksa modellenmeyen alanlar (ör. `disabled`) sessizce silinir.
3. Yazmadan önce `.bak` yedek + atomik yaz (temp+rename). Bozuk JSON'da çökme, hata bildir.
4. enable/disable = silmeden (`disabled: true` bayrağı).
5. Fırsatken düzelt: `normalize_mcp_config` koruması `/mnt/...`'i geçirip değiştirmiyor (ölü dal).

Kabul kriteri: elle eklenmiş bilinmeyen alan `mcp-set`/`mcp-unset` sonrası KORUNUR; `.bak` oluşur; bozuk JSON çökmez; Value-koruyan round-trip testi geçer.

---

## Teslim
`cargo build --release` temiz + `cargo test` geçer + PR/diff. Claude diff'i inceler: ponytail korkulukları, MCP Value-koruma, secret/yol işleme.

---
---

# TUR 2 — Denetim sonrası düzeltme + kalite

Tur 1 (`56596f8`) **kabul edildi**: binary derleniyor, model iniyor, 2 test geçiyor.
Doğru yapılmış (KORU, geri alma): Value-tabanlı MCP düzenleme · `.bak` + atomik yazım · `disabled` bayrağı · `resolve_config_path` · `embedding BLOB` + `note_edges` aynı DB'de · FTS5 tablosu · kosinüs/wikilink/semantik kenar · BFS · 2 test. `McpServerConfig` struct'ını silip tamamen `Value`'ya geçmen direktiften sapmaydı ama **daha doğru** — öyle kalsın.

Aşağıdakiler denetimde bulundu. Ercan kapsamı onayladı: **A (kusurlar) + B (kalite), hepsi.**

## ÖNCE YANITLA (kod yazmadan)
**Özellik 3 nerede?** `src/security.rs::run_security_audit` hâlâ salt-rapor; `56596f8`'de yalnız rustfmt değişikliği var. "Hallettik" denen oto-fix hangi dalda/repoda kaldı? Varsa main'e getir; yoksa kayıp demektir → ayrı iş olarak planlanacak. **Bu turda oto-fix YAZMA.**

## A. Gerçek kusurlar
**A1 — Sahte hybrid, üstelik varsayılan.** `search_hybrid()` doğrudan `search_semantic()` çağırıyor; `--mode` varsayılanı `"hybrid"`. Yani `memory-search X` saf semantik, FTS5 ölü.
→ `search_keyword`/`search_semantic` **yazdırmayı bıraksın**, `Vec<(filename, title, score)>` döndürsün; tek `render()` bassın (yazdırma tekrarını da siler). Hybrid = **RRF** (`k=60`, `Σ 1/(k+rank)`), ~30 satır, yeni bağımlılık yok.

**A2 — FTS5 hatası sessizce yutuluyor.** `if let Ok(rows) = rows` → `-`, `"`, `*`, `:` içeren sorgu sözdizim hatası verir, kullanıcı hata yerine **"0 sonuç"** görür (sessiz yanlış cevap).
→ Hatayı `?` ile yükselt + sorguyu kaçır: her sözcüğü çift tırnağa al, boşlukla birleştir (`foo-bar baz` → `"foo-bar" "baz"`).

**A3 — `mcp-unset <server>` bayraksız çağrılırsa sunucuyu tümüyle siliyor.** Yıkıcı varsayılan.
→ Tam silme **`--remove` bayrağı** şart olsun; bayraksız+alansız çağrıda açıklayıcı hata döndür.

## B. Kalite
**B1 — Chunking yok** (bge-small ~512 token; uzun not sessizce kesiliyor) → içeriği ~1500 karakterlik pencerelere böl, her parçayı göm, **ortalamasını al** (mean-pool). Şema değişmez.
**B2 — Eşikler gömülü** → `memory-index --edge-threshold` (0.70), `memory-search --limit` (5) `--min-score` (0.30).
**B3 — Hayalet wikilink kenarı** (`[[X]]` var olmayan nota) → indekslemede dosya adlarının `HashSet`'ini topla, yalnız var olan hedefe kenar yaz, atlananların sayısını bildir.
**B4 — Ölü `/mnt/` dalı**: guard geçiyor ama `replace` yalnız `/home/jb_remus` → no-op. `/mnt/` guard'ını **sil** (silme > ekleme).
**B5 — Wikilink testi yok** → regex'i saf fonksiyona çıkar (`fn extract_wikilinks(content: &str) -> Vec<String>`) ve test et.

## Korkuluklar (değişmedi)
Tek-implementasyonlu soyutlama yok · ANN/graph DB/vektör-sunucu yok · **yeni bağımlılık yok** (RRF/kaçırma/chunking hepsi stdlib) · `get_home_dir` + `resolve_config_path` + Value deseni yeniden kullanılır · Value-tabanlı MCP düzenlemesi tipli-struct'a **geri döndürülmez**.

## Tur 2 kabul kriterleri
- `cargo test`: kosinüs · **wikilink ayrıştırma (yeni)** · MCP Value-koruma · **FTS5 kaçırma (yeni)** · **RRF sıralaması (yeni)**.
- `memory-search "foo-bar"` sessizce 0 dönmez (hata ya da sonuç).
- Varsayılan `hybrid` hem keyword hem semantik isabet gösterir; `--keyword`/`--semantic` ayrı çalışır.
- `[[yok-boyle-not]]` içeren not → `memory-related` onu listelemez, atlanan kenar sayısı raporlanır.
- 512 token'ı aşan not indekslenir; sonundaki içerikle de semantik olarak bulunur (mean-pool çalışıyor).
- `mcp-unset srv` (bayraksız) reddeder; `--remove` siler + `.bak` oluşur; `mcp-set` sonrası bilinmeyen alan korunur.
