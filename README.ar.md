**🌍 [Türkçe](README.md) | [English](README.en.md) | [العربية](README.ar.md) | [日本語](README.ja.md) | [中文](README.zh.md) | [Русский](README.ru.md) | [Español](README.es.md)**

# تنصيب Claude Code المستقل (نواة Rust بملف تنفيذي واحد)

[![Rust](https://img.shields.io/badge/Rust%20core-%2591-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/Tests-30%20Passed-green.svg)]()

محرّك محلّي للنشر والتدقيق الأمني وإدارة الذاكرة (`claude-code-setup`)، طُوِّر لإدارة بيئة **Claude Code**. وزمن التشغيل ملفٌّ تنفيذيٌّ واحدٌ مكتوبٌ بلغة Rust؛ ولا يلزم لتشغيله وجود Rust أو Python أو Node على الجهاز.

### ملاحظة أمانة: هذا المستودع ليس بنسبة 100% Rust

إحصاءات لغات GitHub وتوزيع الشيفرة المقيس (2026-09-02): **2891 سطرًا بلغة Rust / 279 سطرًا بغير Rust = 91.2% نسبة الأسطر** (إحصاء GitHub Linguist: **%90.5 Rust، %3.5 Shell، %3.2 Python، %2.8 PowerShell**).

| اللغة / الملف | الأسطر | حصة GitHub | متى يعمل |
| :--- | ---: | ---: | :--- |
| `Rust` (`src/*.rs`، 10 ملفات) | 2891 | %90.5 | زمن التشغيل (أداة CLI + خادم MCP) |
| `install-macos.sh` (Shell / Bash) | 121 | %3.5 | أثناء التنصيب على Linux/macOS |
| `package-extension.py` (Python) | 87 | %3.2 | عند إصدار النسخ وتحزيم `.mcpb` فقط (CI) |
| `install-windows.ps1` (PowerShell) | 71 | %2.8 | أثناء التنصيب على Windows |

تبعيّاتٌ إضافية:
- خُطّاف ما قبل الإيداع (pre-commit) في `src/security.rs` يُكتَب على هيئة **نصٍّ برمجيٍّ مضمَّنٍ بلغة Bash** (`#!/usr/bin/env bash`)، ويحتاج تشغيلُه إلى bash المرافق لـ Git.
- يستعمل خطُّ الإصدار `.github/workflows/release.yml` كلًّا من `actions/setup-python` و`npx @anthropic-ai/mcpb validate`، أي أنّ **سلسلة التكامل المستمرّ تعتمد على Python وNode**.
- تُنزِّل طبقةُ التغميد (embedding) في محرّك الذاكرة **الملفَّ التنفيذيَّ المُصرَّف سلفًا بلغة C++ الخاصَّ بـ ONNX Runtime** عبر `fastembed` (خيار `ort-download-binaries`).

الخلاصة الصحيحة: **الملفُّ التنفيذيُّ في زمن التشغيل مكتوبٌ بلغة Rust خالصة؛ أمّا التنصيب والتحزيم وخطُّ الإصدار فتستعمل Bash وPowerShell وPython وNode.**

---

## 🎯 1. الغاية وما تحقَّق

- **زمن تشغيلٍ بملفٍّ واحد:** نُقلَت نصوصُ Bash وPython الموروثة الخاصّة *بزمن التشغيل* إلى Rust. أمّا نصوصُ التنصيب والتحزيم (`install-*.{sh,ps1}` و`package-extension.py`) فقد أُبقيت عن قصد، لأنّ المُنصِّب نفسه لا بدّ أن يعمل قبل تنزيل الملفّ التنفيذي.
- **تسويةُ المسارات الحيويّة:** تتكيّف تعريفاتُ المسارات الجامدة (مثل `/home/jb_remus`) تكيُّفًا حيويًّا مع نظام التشغيل الهدف ومع مجلّد المستخدم المحلّي.
- **إدارةُ MCP متعدّدة الأهداف (`--target`):**
  - إدارةُ إعدادات **Claude Code** (`~/.claude.json`) و**المشروع** (`./.mcp.json`) و**Claude Desktop** (`claude_desktop_config.json`) من واجهةِ أوامرَ واحدة.
  - محرّكُ كتابةٍ ذرّيٌّ يحفظ الحقولَ غير المُنمَّطة في JSON بفضل بنية `serde_json::Value` (مع نسخةٍ احتياطيّةٍ تلقائيّة `.bak`).
- **طَورُ خادم MCP (`--mcp-mode`):** يتحوّل الملفُّ التنفيذيُّ نفسُه إلى خادمِ MCP يتحدّث JSON-RPC عبر stdin/stdout، ويعرض الأدواتِ الثمانيَ المُعلَنة في `manifest.json` على Claude Desktop.
- **محرّكُ الذاكرة (SQLite + متجهات + مخطَّط بياني):**
  - **إضافةُ ملحوظةٍ سريعة (`memory-note`):** إنشاءُ ملحوظاتٍ آمنٍ بأسماءِ ملفّاتٍ من نمط kebab-case.
  - **بحثُ FTS5 بالكلمات:** فهرسةُ كلماتٍ في SQLite مع آليّةِ تهريبٍ لعلامات التنصيص.
  - **تغميدٌ محلّي:** تشابهٌ جيبيٌّ (cosine) عبر `fastembed` (طراز Multilingual-E5-Small). يُنزَّل الطرازُ من Hugging Face عند أوّل استعمال ويُكتَب في `$HOME/.claude/fastembed_cache`؛ **وبعد هذا التنزيل الأوّل** يعمل البحث دون اتّصالٍ بالشبكة تمامًا.
  - **حوافُّ المخطَّط والوصلاتُ الويكيّة:** بحثُ الجوار (`memory-related`) عبر وصلات `[[اسم-الملحوظة]]` والروابطِ الدلاليّةِ فوق العتبة.
  - **ترتيبٌ هجينٌ بـ RRF:** دمجُ نتائج FTS5 والمتجهات بطريقة Reciprocal Rank Fusion (`k=60`).
- **تدقيقٌ أمنيٌّ ذاتيُّ التصحيح (`security-audit --fix`):**
  - فحصُ الأسرار المكتوبةِ نصًّا صريحًا في ملفّات الإعداد (`ghp_` و`github_pat_` و`sk-` و`xox[baprs]-` و`AKIA`).
  - تضييقُ صلاحيّات الملفّات إلى 600 — **في أنظمة Unix وحدها**؛ أمّا في Windows فتُطبَع ملحوظةٌ إعلاميّةٌ عن الصلاحيّات القائمة على ACL دون إجراء أيِّ تصحيح.
  - تنصيبُ خُطّافِ Git لحماية الفروع وفحص الأسرار قبل الإيداع.
- **سيرُ عملِ Git الذاتيّ (`agent-workflow`):**
  - استخراجُ فرعِ ميزةٍ تلقائيًّا من الفرع الافتراضيِّ البعيد.
  - منعُ الدفع المباشر إلى الفروع الرئيسة المحميّة.

---

## 🏗️ 2. البنية والوحدات

```
claude-code-setup-rustified/
├── Cargo.toml                  # تبعيّات Rust وتعريفات الحزمة (v0.1.6)
├── manifest.json               # بيانُ إضافة Claude Desktop (8 أدوات MCP)
├── icon.png                    # أيقونةُ الإضافة
├── .env.example                # نموذجُ متغيّرات البيئة
├── src/
│   ├── main.rs                 # نقطةُ دخول الأوامر ومُوجِّهُها (123 سطرًا)
│   ├── cli.rs                  # تعريفاتُ الأوامر والأهداف والأعلام بـ Clap (222)
│   ├── mcp.rs                  # مديرُ MCP متعدّدُ الأهداف حافظٌ لقيم JSON (488)
│   ├── mcp_server.rs           # خادمُ MCP بـ stdio JSON-RPC؛ يربط 8 أدواتٍ بالأوامر (436)
│   ├── memory_engine.rs        # محرّكُ FTS5 + متجهات + مخطَّط + RRF + memory-note (821)
│   ├── installer.rs            # مُنشِئُ الهيكل ومِلفِّ README الأوّليّ و.env (191)
│   ├── security.rs             # مدقّقٌ أمنيٌّ ذاتيُّ التصحيح ومديرُ الخُطّافات (296)
│   ├── branch_manager.rs       # مُنفِّذُ سير عمل Git بضماناتِ الفروع المحميّة (161)
│   ├── tester.rs               # مُشغِّلُ اختباراتِ تشخيصِ النظام والبيئة (123)
│   └── agent.rs                # واجهةُ تكامل الوكيل (30)
├── install-windows.ps1         # مُنصِّب PowerShell (ليس Rust)
├── install-macos.sh            # مُنصِّب Bash (ليس Rust)
├── package-extension.py        # مُحزِّم .mcpb يُستدعى في CI (ليس Rust)
├── .github/workflows/
│   ├── rust.yml                # fmt + clippy + test + build (ubuntu/windows/macos)
│   └── release.yml             # خطُّ إصدارِ الملفّات التنفيذيّة الثلاثة وحزم .mcpb
└── docs/                       # أدلّةُ التنصيب ومعالجةِ المشكلات
```

### مسؤوليّاتُ الوحدات
- `src/main.rs`: يُحلِّل معاملاتِ سطر الأوامر؛ فإن أُعطي `--mcp-mode` سلَّم التحكّمَ إلى خادم MCP، وإلّا سلَّمه إلى دالّةِ الوحدة المعنيّة.
- `src/cli.rs`: يُدير 15 أمرًا فرعيًّا والأعلامَ (`--target` و`--fix` و`--hooks` و`--mode` و`--min-score`) والعلَمَ العامَّ `--mcp-mode` عبر بنية `Parser` من Clap.
- `src/mcp.rs`: يقرأ إعداداتِ MCP ويُحدِّثها وفق معامل `--target` (`claude-code` و`project` و`claude-desktop`)، ويكتب كتابةً ذرّيّةً دون حذف الحقول المجهولة.
- `src/mcp_server.rs`: يُقيم حلقةَ JSON-RPC على stdin/stdout؛ ويربط الأدواتِ الثمانيَ في `manifest.json` (`mcp_list` و`mcp_add` و`security_audit` و`memory_note` و`memory_index` و`memory_search` و`status` و`test`) بأوامرِ الواجهةِ الحقيقيّة. وهذا الربطُ مُقيَّدٌ باختبار `her_arac_gercek_bir_cli_komutuna_esleniyor`.
- `src/memory_engine.rs`: يُغمِّد الملحوظاتِ بتقسيمها إلى نوافذَ نحوِ 1500 حرفٍ ثمّ يأخذ متوسّطَها (mean-pooling)؛ ويُدير جدولَي SQLite: `knowledge_notes` و`note_edges`. وذاكرةُ التغميد المؤقّتة في `$HOME/.claude/fastembed_cache`.
- `src/installer.rs`: يُنشئ مجلّد `$HOME/claude_global_memory/knowledge` وملفَّ `README.md` الأوّليَّ دون أن يطمسهما أبدًا؛ وينسخ `.env` إن لم يكن موجودًا.
- `src/security.rs`: يفحص الأسرارَ الصريحة، ويتحقّق من الصلاحيّات، ويصحّحها بـ `--fix`، ويُنصِّب خُطّافَ Git (والخُطّافُ نصٌّ برمجيٌّ مضمَّنٌ بلغة Bash).
- `src/branch_manager.rs`: يُدير إنشاءَ الفروع الذاتيَّ، وحاجزَ الفروع المحميّة، وعمليّاتِ الإيداع والدفع الآمنة.
- `src/tester.rs`: يُجري تشخيصَ النظام (`status`) والتحقّقَ من الاختبارات.

---

## 🚀 3. التنصيب والإعداد

### بدايةٌ سريعة

هناك تنصيبان مختلفان؛ فاختر ما تريد.

**إضافةُ Claude Desktop (المُستحسَن)** — نزِّل الحزمةَ الموافقةَ لنظام تشغيلك من [آخر إصدار](https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest)، ثمّ اسحبها إلى Claude Desktop → Settings → Extensions:

| نظام التشغيل | الملف | الحجم التقريبي |
|---|---|---|
| Windows | `claude-code-setup-windows.mcpb` | 9 م.ب |
| macOS | `claude-code-setup-macos.mcpb` | 10 م.ب |
| Linux | `claude-code-setup-linux.mcpb` | 12 م.ب |

**أداةُ سطر الأوامر** — إن أردت استعمالها من الطرفيّة:

```powershell
irm https://raw.githubusercontent.com/Ercaner1988/claude-code-setup-rustified/main/install-windows.ps1 | iex
```

```bash
curl -fsSL https://raw.githubusercontent.com/Ercaner1988/claude-code-setup-rustified/main/install-macos.sh | bash
```

هذان المُنصِّبان نصّان برمجيّان بلغتَي PowerShell وBash (لا Rust)؛ يُنصِّبان الملفَّ التنفيذيَّ المُنزَّل في مجلّد المستخدم ويُضيفانه إلى PATH (ولا يلزم امتيازُ المدير). وهما **لا يُسجّلان** الإضافة؛ فللإضافة استعمل مسار `.mcpb` أعلاه. وللتحقّق نفِّذ `claude-code-setup status` في طرفيّةٍ جديدة.

لتفاصيل التنصيب انظر [INSTALLATION.ar.md](INSTALLATION.ar.md)

---

### التنصيبُ اليدويّ: البناءُ من المصدر

#### المتطلّبات
- **سلسلةُ أدوات Rust:** `rustc` و`cargo` (إصدار 1.80 أو أحدث)
- في البناء الأوّل يُنزِّل `fastembed` الملفَّ التنفيذيَّ الخاصَّ بـ ONNX Runtime، فيلزم اتّصالٌ بالشبكة.

#### البناء
```bash
cargo build --release

# الملفُّ التنفيذيُّ الناتج:
# Windows: ./target/release/claude-code-setup.exe
# Linux/macOS: ./target/release/claude-code-setup
```

### التنصيبُ التلقائيُّ وتشخيصُ البيئة
```bash
# يتحقّق من الشروط المسبقة ويُنشئ هيكلَ الذاكرة
./target/release/claude-code-setup install --hooks

# حالةُ تشخيصِ النظام والبيئة
./target/release/claude-code-setup status
```

---

## 📖 4. الاستعمالُ والأمثلة

### جدولُ الأوامر الموجَز

| الأمر | الوصف |
| :--- | :--- |
| `--mcp-mode` (علَمٌ عامّ) | يُشغِّل الملفَّ التنفيذيَّ خادمَ MCP يتحدّث JSON-RPC عبر stdin/stdout |
| `install [--hooks] [--skip-prereqs]` | إعدادُ البيئة وهيكلُ الذاكرة ونسخُ `.env` |
| `test` / `status` | تشخيصُ واجهةِ Claude و`.claude.json` وقاعدةِ الذاكرة والخُطّافات |
| `mcp-list [--target T]` | يسرد خوادمَ MCP المُعدَّة وفق الهدف |
| `mcp-set <srv> [--command C] [--arg A]… [--env K=V]… [--target T]` | يُضيف خادمَ MCP أو يُحدِّثه (`--target`: `claude-code` أو `project` أو `claude-desktop`) |
| `mcp-unset <srv> [--env K]… [--clear-args] [--remove] [--target T]` | يحذف متغيّراتٍ أو يُزيل الخادمَ كلَّه (والعلَمُ `--remove` واجب) |
| `mcp-enable <srv>` / `mcp-disable <srv>` | يُفعِّل الخادمَ أو يُعطّله دون إفساد الإعداد |
| `memory-note <العنوان> [--body ...] [--dir D]` | يُضيف ملحوظةَ Markdown جديدةً إلى قاعدة المعرفة |
| `memory-index [--source المجلّد]… [--edge-threshold 0.70]` | يفهرس الملحوظاتِ في محرّك SQLite + المتجهات + المخطَّط |
| `memory-search <الاستفهام> [--mode keyword\|semantic\|hybrid] [--limit 5] [--min-score 0.30]` | يبحث في الذاكرة بطَور كلمات FTS5 أو المتجهات أو الهجين RRF |
| `memory-related <note.md>` | يسرد الملحوظاتِ المتّصلةَ عبر حوافِّ المخطَّط والوصلاتِ الويكيّة |
| `install-hooks [--repo-dir المسار]` | يُنصِّب خُطّافَ الأمان قبل الإيداع في المستودع |
| `security-audit [--fix]` | يُجري تدقيقًا أمنيًّا؛ ويُطبِّق التصحيحَ الذاتيَّ مع `--fix` |
| `agent-workflow [--branch-type النوع] --description الوصف [--files F]…` | يُشغِّل سيرَ عملِ الفروع والإيداع الذاتيَّ بضماناتِ الفروع المحميّة |

تقبل جميعُ الأوامر تجاوزَ `--home-dir` لعزل الاختبارات (خلا `install-hooks` و`agent-workflow`).

### أمثلةٌ تطبيقيّة

#### إدارةُ خوادم MCP وفق الهدف
```bash
# تعريفُ خادم MCP على مستوى المشروع (.mcp.json)
./target/release/claude-code-setup mcp-set github \
  --command "npx" --arg "-y" --arg "@modelcontextprotocol/server-github" \
  --env "GITHUB_TOKEN=$GITHUB_TOKEN" --target project

# تعطيلُ الخادم في إعداد Claude Desktop
./target/release/claude-code-setup mcp-disable github --target claude-desktop

# إزالةُ الخادم كلَّه (العلَمُ --remove إلزاميّ)
./target/release/claude-code-setup mcp-unset github --remove --target claude-code
```

#### إضافةُ ملحوظةٍ إلى الذاكرة والبحثُ الهجين بـ RRF
```bash
./target/release/claude-code-setup memory-note "القرارات البنيويّة" --body "تمّ نقلُ زمن التشغيل إلى ملفٍّ تنفيذيٍّ بلغة Rust."
./target/release/claude-code-setup memory-index --edge-threshold 0.70
./target/release/claude-code-setup memory-search "بنية Rust" --mode hybrid --limit 5 --min-score 0.30
./target/release/claude-code-setup memory-related mimari-kararlar.md
```

---

## 🛡️ 5. الاختباراتُ وبوّاباتُ الجَودة

```bash
cargo test
# running 30 tests
# test result: ok. 30 passed; 0 failed; 0 ignored
```

في المصدر **31 اختبارًا** مُعرَّفًا؛ أحدها (`test_enforce_file_permissions_fixes_mode`) موسومٌ بـ `#[cfg(unix)]` فلا يُصرَّف على Windows. والمقيس: **30/30 خُضرةً على Windows، و31/31 على Unix** (2026-09-02).

التوزيعُ حسب الملفّات: `memory_engine.rs` 14، و`mcp.rs` 5، و`mcp_server.rs` 5، و`security.rs` 3، و`branch_manager.rs` 2، و`installer.rs` 2.

### معاييرُ الجَودة
- **التغطية:** إدارةُ MCP متعدّدةُ الأهداف، وحفظُ قيم JSON، وتهريبُ محارف FTS5، والترتيبُ الهجين بـ RRF، وmean-pooling، وتحليلُ الوصلات الويكيّة، وانحدارُ مسار ذاكرة التغميد، وفحصُ الأسرار، وربطُ أدوات MCP بالأوامر، وحواجزُ الفروع المحميّة.
- **التنسيق:** `cargo fmt --all -- --check` ← نظيف (2026-09-02).
- **التحليلُ الساكن:** `cargo clippy --all-targets -- -D warnings` ← بلا تحذيرات (2026-09-02).
- **التكاملُ المستمرّ:** يُشغِّل `.github/workflows/rust.yml` أوامرَ fmt وclippy واختبارٍ وبناءِ إصدارٍ على ثلاثة أنظمة (ubuntu وwindows وmacos). ويُنتج `.github/workflows/release.yml` الملفّاتِ التنفيذيّةَ للمنصّات الثلاث وحزمَ `.mcpb`؛ وهذا الخطُّ يستعمل Python وNode.

---

## 👥 6. المساهمون

قِيسَت الأرقامُ الآتية بأمر `git shortlog -sne --all` وبعدِّ وسوم `Co-authored-by` في متون الإيداعات (2026-09-02، بمجموع 45 إيداعًا).

| المساهم | الدور / المسؤوليّة | المساهمةُ المقيسة |
| :--- | :--- | :--- |
| **Ercan ER** | بنيةُ المشروع، والنقلُ إلى Rust، والتطويرُ الرئيس | 41 إيداعًا (مؤلّف) |
| **Claude Opus 5** | وكيلُ ذكاءٍ اصطناعيٍّ ذاتيّ، وتطويرُ الوحدات | 14 إيداعًا (مؤلّفٌ مشارك) |
| **Copilot App** | مساعدُ برمجةٍ بالذكاء الاصطناعيّ | 11 إيداعًا (مؤلّفٌ مشارك) |
| **Claude Opus 4.8** | وكيلُ ذكاءٍ اصطناعيٍّ ذاتيّ | 3 إيداعات (مؤلّفٌ مشارك) |
| **Claude** (بلا تحديد إصدار) | وكيلُ ذكاءٍ اصطناعيٍّ ذاتيّ | إيداعان (مؤلّفٌ مشارك) |
| **jb_remus** | المؤلّفُ الأصليُّ للأصل الأعلى (upstream) | إيداعان (مؤلّف) |
| **Mihenk** | مدقّقُ الشِّفرة وحَكَمُ الجَودة | إيداعٌ واحد (مؤلّف) |
| **arturo-ebuck** | مساهمٌ في المصدر المفتوح | إيداعٌ واحد (مؤلّف) |

و**Kassam** هويّةُ الوكيل المسجَّلةُ في حقل `authors` من `Cargo.toml`؛ وليس له سجلُّ مؤلِّفٍ مستقلٌّ في Git.

---

## 📄 7. الرخصةُ والمصادر

هذا المشروع مُرخَّصٌ بموجب [رخصة MIT](LICENSE) (حقوق النشر © 2026 Ercan Er).

### وثائقُ ذاتُ صلة
- [دليلُ النشر](DEPLOYMENT_GUIDE.md)
- [دليلُ التنصيب اليدويّ](docs/MANUAL_SETUP.md)
- [دليلُ معالجةِ المشكلات](docs/TROUBLESHOOTING.md)
- [توجيهاتُ المطوّرين](docs/dev/TASK-KASSAM-1-2.md)
