**🌍 [Türkçe](README.md) | [English](README.en.md) | [العربية](README.ar.md) | [日本語](README.ja.md) | [中文](README.zh.md) | [Русский](README.ru.md) | [Español](README.es.md)**

# Claude Code الإعداد المستقل (محرك Rust بنسبة 100%)

[![Rust](https://img.shields.io/badge/Rust-100%25-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/Tests-24%20Passed-green.svg)]()

نظام نشر وإدارة وتدقيق أمني ومحرك ذاكرة عالي الأداء يعتمد **بنسبة 100% على لغة Rust** لبيئة **Claude Code** (`claude-code-setup.exe`).

تمت إزالة جميع سكربتات Bash (`.sh`) و Python (`.py`) القديمة بالكامل وتحويلها إلى برنامج تنفيذي محلي موحد بلغة Rust.

---

## 🎯 1. الغرض والميزات

- **بنية Rust خالصة 100%:** إلغاء كافة الاعتمادات على سكربتات Shell وبيئة تشغيل Python.
- **تطبيع المسارات تلقائياً:** تحويل مسارات النظام الثابتة (مثل `/home/jb_remus`) تلقائياً إلى البيئة المحلية ودليل المستخدم الرئيسي.
- **إدارة MCP متعددة الأهداف (`--target`):**
  - إدارة خوادم MCP عبر إعدادات **Claude Code** (`~/.claude.json`) و**المشروع** (`./.mcp.json`) و**Claude Desktop** (`claude_desktop_config.json`).
  - الحفاظ على الحقول غير النمطية في JSON وإنشاء نسخ احتياطية تلقائية `.bak`.
- **محرك ذاكرة سريع قائم على SQLite (متجهات + رسم بياني):**
  - **إضافة ملاحظات سريعة (`memory-note`):** إضافة ملاحظات Markdown بأسماء kebab-case دون الكتابة فوق الملفات الموجودة.
  - **بحث الكلمات المفتاحية FTS5:** بحث نصي كامل مع هروب تلقائي لرموز الاستعلام الخاصة.
  - **التضمين المحلي:** حساب تشابه جيب التمام محلياً وبدون اتصال عبر `fastembed` (Multilingual-E5-Small).
  - **حواف الرسم البياني وروابط Wikilink:** تنقل الجوار عبر إشارات `[[Note-Name]]` والروابط الدلالية (`memory-related`).
  - **الترتيب الهجين RRF:** استخدام خوارزمية Reciprocal Rank Fusion (`k=60`) لدمج نتائج البحث بأعلى دقة.
- **تدقيق أمني مع إصلاح تلقائي (`security-audit --fix`):**
  - فحص الرموز السرية المكشوفة في ملفات التكوين.
  - فرض وتصحيح صلاحيات الملفات تلقائياً على أنظمة Unix.
  - تثبيت خطافات pre-commit لحماية الأفرع وفحص الأسرار.
- **سير عمل Git آمن ومستقل (`agent-workflow`):**
  - إنشاء أفرع الميزات تلقائياً من الأفرع الافتراضية البعيدة.
  - حظر الدفع المباشر إلى الأفرع الرئيسية المحمية.

---

## 🏗️ 2. البنية والموديولات

```
claude-code-complete-setup/
├── Cargo.toml                  # بيان المشروع واعتمادات Rust
├── src/
│   ├── main.rs                 # نقطة الدخول والموجه
│   ├── cli.rs                  # تعريفات الأوامر والخيارات والأهداف عبر Clap
│   ├── mcp.rs                  # مدير MCP متعدد الأهداف للحفاظ على قيم JSON
│   ├── memory_engine.rs        # محرك FTS5 + متجهات + رسم بياني + RRF + memory-note
│   ├── installer.rs            # إنشاء الهيكل وتثبيت README الابتدائي وملف .env
│   ├── security.rs             # مدقق الأمان مع الإصلاح التلقائي وإدارة الخطافات
│   ├── branch_manager.rs       # مشغل سير عمل أفرع Git المحمية
│   ├── tester.rs               # مشغل حزمة التشخيص والاختبار
│   └── agent.rs                # واجهة تكامل الوكلاء
└── docs/                       # أدلة التثبيت واستكشاف الأخطاء وإصلاحها
```

### مسؤوليات الموديولات
- `src/main.rs`: تحليل وسائط سطر الأوامر وتوجيه التنفيذ إلى الموديول المناسب.
- `src/cli.rs`: إدارة الأوامر والخيارات (`--target`, `--fix`, `--hooks`, `--mode`) ونصوص المساعدة عبر Clap.
- `src/mcp.rs`: قراءة وتحديث تكوينات MCP بناءً على الهدف المSelected (`claude-code`, `project`, `claude-desktop`) مع الحفاظ على الحقول المخصصة.
- `src/memory_engine.rs`: تقسيم النصوص إلى قطع وإدارة جداول SQLite (`knowledge_notes` و `note_edges`) وإضافة الملاحظات بأمان عبر `memory-note`.
- `src/installer.rs`: إنشاء دليل `~/claude_global_memory/knowledge` وملف `README.md` الابتدائي دون إتلاف الملفات القديمة.
- `src/security.rs`: تدقيق الأسرار والأذونات، وتطبيق الإصلاح التلقائي بـ `--fix` وتثبيت خطافات الأمان.
- `src/branch_manager.rs`: أتمتة إنشاء الأفرع والحفاظ على حماية الأفرع الرئيسية.
- `src/tester.rs`: تشغيل اختبارات التحقق والتشخيص (`status` و `test`).

---

## 🚀 3. التثبيت والإعداد

### المتطلبات الأساسية
- **أدوات Rust:** `rustc` و `cargo` (الإصدار 1.80+)

### التجميع
```bash
# بناء النسخة التنفيذية
cargo build --release

# الملف التنفيذي الناتج:
# Windows: ./target/release/claude-code-setup.exe
# Linux/macOS: ./target/release/claude-code-setup
```

### التثبيت التلقائي والتشخيص
```bash
# تشغيل الإعداد التلقائي وتثبيت خطافات الأمان
./target/release/claude-code-setup install --hooks

# تشغيل تشخيص البيئة
./target/release/claude-code-setup status
```

---

## 📖 4. الاستخدام والأمثلة

### جدول ملخص الأوامر

| الأمر | الوصف |
| :--- | :--- |
| `install [--hooks]` | التثبيت الكامل وإنشاء هيكل الذاكرة وإعداد `.env` |
| `test` / `status` | تشخيص البيئة وخوادم MCP والذاكرة والخطافات |
| `mcp-list [--target T]` | سرد خوادم MCP المكونة بناءً على الهدف |
| `mcp-set <srv> [...] [--target T]` | إنشاء/تحديث خادم MCP (`--target`: `claude-code`, `project`, `claude-desktop`) |
| `mcp-unset <srv> [...] [--remove] [--target T]` | إزالة حقول؛ حذف الخادم يتطلب `--remove` |
| `mcp-enable <srv>` / `mcp-disable <srv>` | تفعيل/تعطيل خادم دون حذف إعداداته |
| `memory-note <عنوان> [--body ...]` | إضافة ملاحظة Markdown جديدة بأمان |
| `memory-index [--source DIR]...` | فهرسة الملاحظات في محرك المتجهات والرسم البياني |
| `memory-search <استعلام> [--mode ...]` | البحث في الملاحظات المفهرسة بـ FTS5 أو المتجهات أو RRF الهجين |
| `memory-related <ملاحظة.md>` | عرض الملاحظات ذات الصلة عبر حواف الرسم البياني |
| `install-hooks [--repo-dir PATH]` | تثبيت خطاف pre-commit الأمني في المستودع |
| `security-audit [--fix]` | تدقيق أمان الصلاحيات والأسرار؛ `--fix` يطبق الإصلاح التلقائي |
| `agent-workflow [-t TYPE] -d DESC` | تنفيذ سير عمل الأفرع والالتزام المستقل مع حماية الأفرع الرئيسية |

### أمثلة على سيناريوهات الاستخدام

#### إدارة خوادم MCP حسب الهدف
```bash
# إعداد خادم MCP على مستوى المشروع (.mcp.json)
./target/release/claude-code-setup mcp-set github --command "npx" --arg "-y" --arg "@modelcontextprotocol/server-github" --env "GITHUB_TOKEN=ghp_example" --target project

# تعطيل خادم في إعدادات Claude Desktop
./target/release/claude-code-setup mcp-disable github --target claude-desktop

# إزالة الخادم بالكامل (يتطلب علم --remove)
./target/release/claude-code-setup mcp-unset github --remove --target claude-code
```

#### إضافة ملاحظة ذاكرة والبحث الهجين
```bash
# إضافة ملاحظة جديدة
./target/release/claude-code-setup memory-note "قرارات البنية" --body "تم الانتهاء من التحويل إلى Rust بالكامل."

# فهرسة ملاحظات المعرفة
./target/release/claude-code-setup memory-index --edge-threshold 0.70

# تشغيل البحث الهجين RRF
./target/release/claude-code-setup memory-search "بنية Rust" --mode hybrid --limit 5

# الاستعلام عن الملاحظات المرتبطة
./target/release/claude-code-setup memory-related architecture-decisions.md
```

---

## 🛡️ 5. بوابات الجودة والاختبارات

يتضمن المشروع 24 اختبار وحدة، وجميعها ناجحة حالياً:

```bash
cargo test
```

### معايير الجودة
- **اختبارات الوحدة (24/24 ناجح):** التحقق من إدارة MCP متعددة الأهداف، والحفاظ على JSON Value، والهروب في FTS5، والترتيب الهجين RRF، واستخراج الروابط، وفحص الأسرار وحماية الأفرع.
- **التنسيق:** مفروض عبر `cargo fmt --check`
- **التكامل المستمر (CI):** مفحوص على Ubuntu وmacOS وWindows عبر `.github/workflows/rust.yml` و `.github/workflows/release.yml`.

---

## 👥 6. المساهمون

| المساهم | الدور / المسؤولية | المقاييس |
| :--- | :--- | :--- |
| **Ercan ER** | المهندس الرئيسي، والتحويل إلى Rust، والمطور الأساسي | 26 commits |
| **Kassam** | وكيل الذكاء الاصطناعي المستقل ومطور محرك Rust والموديولات | مؤلف مشارك / مساهم |
| **Copilot** | مساعد البرمجة بالذكاء الاصطناعي | 4 commits |
| **jb_remus** | المؤلف الأصلي للمشروع | 2 commits |
| **Mihenk** | مدقق الكود ومراجع الجودة | 1 commit |
| **arturo-ebuck** | مساهم في المصدر المفتوح | 1 commit |

---

## 📄 7. الترخيص والمراجع

موزع تحت [رخصة MIT](LICENSE).

### الوثائق ذات الصلة
- [دليل النشر](DEPLOYMENT_GUIDE.md)
- [دليل التثبيت اليدوي](docs/MANUAL_SETUP.md)
- [دليل استكشاف الأخطاء وإصلاحها](docs/TROUBLESHOOTING.md)
- [توجيهات المطورين](docs/dev/TASK-KASSAM-1-2.md)
