**🌍 [Türkçe](INSTALLATION.md) | [English](INSTALLATION.en.md) | [العربية](INSTALLATION.ar.md) | [日本語](INSTALLATION.ja.md) | [中文](INSTALLATION.zh.md) | [Русский](INSTALLATION.ru.md) | [Español](INSTALLATION.es.md)**

# دليل تثبيت Claude Code المستقل (نواة Rust بملف تنفيذي واحد)

يقدم هذا الدليل تعليمات خطوة بخطوة لتثبيت وتكوين أداة سطر الأوامر **Claude Code Setup** (`claude-code-setup`) المبنية على نواة Rust عبر منصات التشغيل المختلفة.

> **ملاحظة أمانة:** الملف التنفيذي الذي تشغّله مكتوب بلغة Rust خالصة وقائم بذاته. أمّا **أدوات التثبيت في هذا الدليل فليست بلغة Rust**: الملف `install-windows.ps1` نصٌّ برمجي بلغة PowerShell، والملف `install-macos.sh` نصٌّ برمجي بلغة Bash. كما تُنتَج حزم الإصدار بواسطة `package-extension.py` (بلغة Python). إحصاءات لغات GitHub: **%90.5 Rust، %3.5 Shell، %3.2 Python، %2.8 PowerShell** (نسبة الأسطر المقيسة: 91.2% Rust، و8.8% PowerShell + Bash + Python).

---

## 🎯 1. نظرة عامة

- **برنامج تنفيذي واحد:** إلغاء الاعتماد تماماً على سكربتات Shell (`.sh`) أو Python (`.py`).
- **متعدد المنصات:** أداء محلي عالي السرعة على Windows (x64) و Linux (x64) و macOS (x64 / ARM64).
- **بدون اعتمادات خارجية:** التثبيت عن طريق تنزيل البرامج التنفيذية الجاهزة أو البناء عبر `cargo` في ثوانٍ.

---

## 📥 2. الطريقة 1: تنزيل البرنامج التنفيذي الجاهز (موصى به)

قم بتنزيل الملف التنفيذي المناسب لنظام التشغيل لديك مباشرة من صفحة إصدارات GitHub.

### Windows (x64)
التنزيل والتنفيذ عبر PowerShell:
```powershell
# تنزيل الملف التنفيذي للإصدار
Invoke-WebRequest -Uri "https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-windows-x86_64.exe" -OutFile "claude-code-setup.exe"

# تشغيل التثبيت وإعداد الذاكرة
.\claude-code-setup.exe install --hooks
```

### Linux (x64)
التنزيل عبر الطرفية ومنح صلاحيات التنفيذ:
```bash
# تنزيل الملف التنفيذي
curl -LO https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-linux-x86_64

# منح صلاحية التنفيذ
chmod +x claude-code-setup-linux-x86_64

# تشغيل التثبيت
./claude-code-setup-linux-x86_64 install --hooks
```

### macOS (x64)
```bash
# تنزيل الملف التنفيذي
curl -LO https://github.com/Ercaner1988/claude-code-setup-rustified/releases/latest/download/claude-code-setup-macos-x86_64

# منح صلاحية التنفيذ
chmod +x claude-code-setup-macos-x86_64

# تشغيل التثبيت
./claude-code-setup-macos-x86_64 install --hooks
```

---

## 🛠️ 3. الطريقة 2: البناء من المصدر (Cargo)

إذا كانت بيئة Rust مثبيتة لديكم (`cargo` 1.80+):

```bash
# استنساخ المستودع
git clone https://github.com/Ercaner1988/claude-code-setup-rustified.git
cd claude-code-setup-rustified

# بناء النسخة التنفيذية
cargo build --release

# تشغيل التثبيت
./target/release/claude-code-setup install --hooks
```

للتثبيت على مستوى النظام بالكامل عبر cargo:
```bash
cargo install --path .
claude-code-setup install --hooks
```

---

## ⚙️ 4. التحقق والتشخيص بعد التثبيت

التحقق من حالة تشخيص البيئة بعد التثبيت:

```bash
# تشغيل تشخيص البيئة
claude-code-setup status

# تشغيل حزمة اختبارات التشخيص
claude-code-setup test
```

---

## 🛡️ 5. تدقيق الأمان وإعداد خطافات Git

تدقيق أمان التكوينات وتثبيت خطافات حماية الأفرع قبل الالتزام:

```bash
# التدقيق الأمني مع الإصلاح التلقائي
claude-code-setup security-audit --fix

# تثبيت خطاف pre-commit في مستودع
claude-code-setup install-hooks --repo-dir .
```

---

## 📚 6. الوثائق ذات الصلة

- [الوثائق الكاملة (README.md)](README.md)
- [دليل النشر (DEPLOYMENT_GUIDE.md)](DEPLOYMENT_GUIDE.md)
- [دليل استكشاف الأخطاء وإصلاحها (TROUBLESHOOTING.md)](docs/TROUBLESHOOTING.md)
