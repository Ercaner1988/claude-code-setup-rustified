# Claude Code Setup — نسخة Rust

أداة سطر أوامر واحدة مبنية **بنسبة 100% بلغة Rust** لإدارة بيئة **Claude Code**: إدارة ديناميكية لخوادم MCP، محرك ذاكرة محلي دلالي قائم على الرسم البياني، تدقيق أمني، وسير عمل Git آمن.

English: [README.md](README.md) · Türkçe: [README.tr.md](README.tr.md)

## الميزات

1. **إدارة MCP ديناميكية** — إدارة خوادم MCP عبر إعدادات **Claude Code** (`~/.claude.json`) و**المشروع** (`.mcp.json`) و**Claude Desktop** باستخدام `--target`؛ التعديلات تحافظ على الحقول غير المعروفة، وتُكتب بشكل ذرّي مع نسخة احتياطية `.bak`.
2. **محرك ذاكرة محلي** — فهرسة ملاحظات Markdown في SQLite: بحث بالكلمات المفتاحية FTS5، تضمينات محلية (Multilingual-E5-Small عبر fastembed، دون اتصال بالكامل)، ترتيب هجين RRF، ورسم بياني للروابط والتشابه الدلالي (`memory-related`).
3. **تدقيق أمني مع إصلاح تلقائي** — فحص الرموز السرية المكشوفة، فرض صلاحيات الملفات على Unix، وتثبيت خطافات pre-commit للحماية (`security-audit --fix`).
4. **سير عمل Git آمن** — `agent-workflow` ينشئ فروع ميزات من الفرع الافتراضي البعيد ويرفض الدفع إلى الفروع المحمية؛ جميع أخطاء git تظهر بوضوح.

## البناء والاستخدام

يتطلب Rust toolchain (الإصدار 1.80+). يعمل على Windows وLinux وmacOS.

```bash
# البناء
cargo build --release

# التحقق من المتطلبات وإعداد هيكل الذاكرة وملف .env
./target/release/claude-code-setup install

# تشغيل تشخيص البيئة
./target/release/claude-code-setup status
```

## الأوامر

| الأمر | الوصف |
| :--- | :--- |
| `install [--hooks]` | التحقق من المتطلبات، إنشاء `~/claude_global_memory/knowledge` (دون الكتابة فوق الموجود)، إنشاء `.env` من `.env.example` عند غيابه |
| `test` / `status` | تشخيص البيئة: Claude CLI، `~/.claude.json`، قاعدة بيانات الذاكرة، ذاكرة النموذج المؤقتة، الخطافات، متغيرات البيئة |
| `mcp-list [--target T]` | سرد خوادم MCP المكونة |
| `mcp-set <srv> [--command X] [--arg A]... [--env K=V]... [--target T]` | إنشاء/تحديث خادم MCP |
| `mcp-unset <srv> [--env K]... [--clear-args] [--remove] [--target T]` | إزالة حقول؛ حذف الخادم يتطلب `--remove` |
| `mcp-enable <srv>` / `mcp-disable <srv> [--target T]` | تفعيل/تعطيل خادم دون حذف إعداداته |
| `memory-note <عنوان> [--body ...]` | إضافة ملاحظة إلى قاعدة المعرفة (اسم ملف kebab-case، دون الكتابة فوق الموجود) |
| `memory-index [--source DIR]... [--edge-threshold 0.70]` | فهرسة الملاحظات في SQLite (تضمينات + حواف الرسم البياني) |
| `memory-search <استعلام> [--mode keyword\|semantic\|hybrid] [--limit 5] [--min-score 0.30]` | البحث في الملاحظات المفهرسة (الافتراضي: RRF هجين) |
| `memory-related <ملاحظة.md>` | عرض الملاحظات ذات الصلة عبر حواف الرسم البياني (BFS، قفزتان) |
| `install-hooks [--repo-dir PATH]` | تثبيت خطاف pre-commit الأمني في مستودع |
| `security-audit [--fix]` | فحص الأسرار، التحقق من الصلاحيات (Unix)، التحقق من الخطافات والفرع |
| `agent-workflow [-t TYPE] -d DESC [-f FILE]...` | إنشاء فرع ميزة، commit للملفات، ودفعها — مع حماية الفروع المحمية |

قيم `--target`: `claude-code` (الافتراضي، `~/.claude.json`)، `project` (`./.mcp.json`)، `claude-desktop` (`claude_desktop_config.json`).

## ملاحظات محرك الذاكرة

- دليل المعرفة الافتراضي: `~/claude_global_memory/knowledge` (ينشئه `install`؛ أضف ملاحظات بـ `memory-note`).
- نموذج التضمين (~100 ميغابايت) يُحمَّل عند أول `memory-index`/`memory-search` ويُخزَّن محلياً؛ بعدها يعمل كل شيء دون اتصال.
- البحث الخطي بتشابه جيب التمام مقصود عند هذا الحجم؛ مكان إضافة فهرس ANN موضَّح في الكود عند نمو عدد الملاحظات إلى الآلاف.

## الأمان

- لا أسرار في المستودع؛ `.env` مستثنى من git ولا يُكتب فوقه أبداً.
- كل كتابة إعدادات ذرّية (temp + rename) وتترك نسخة احتياطية `.bak`.
- `mcp-unset <srv>` بدون خيارات يرفض التنفيذ — الحذف التدميري يتطلب `--remove`.
