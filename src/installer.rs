use anyhow::{Context, Result};
use chrono::Local;
use colored::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn get_home_dir(override_path: Option<String>) -> Result<PathBuf> {
    if let Some(path) = override_path {
        Ok(PathBuf::from(path))
    } else {
        env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .map(PathBuf::from)
            .context("Could not determine user home directory")
    }
}

fn log_info(msg: &str) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    println!("{} {} {}", format!("[{}]", now).blue(), "INFO:".bold(), msg);
}

fn log_success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

fn log_warning(msg: &str) {
    println!("{} {}", "⚠".yellow().bold(), msg);
}

fn check_cmd(cmd: &str) -> bool {
    Command::new(cmd).arg("--version").output().is_ok()
}

const KNOWLEDGE_README: &str = r#"# Knowledge Base

Bu dizin, `claude-code-setup` hafıza motorunun indekslediği markdown notlarını tutar.

- Not eklemek için: `claude-code-setup memory-note "Başlık" --body "İçerik"`
- Notları indekslemek için: `claude-code-setup memory-index`
- Aramak için: `claude-code-setup memory-search "sorgu"`
- İlişkili notlar: `claude-code-setup memory-related <dosya.md>`

Notlar arasında `[[Diğer Not]]` biçiminde wikilink kullanabilirsiniz; graf kenarına dönüşür.
"#;

/// Knowledge dizin iskeletini oluşturur; tohum README yalnız yoksa yazılır.
/// Oluşturulduysa true döner.
fn ensure_knowledge_skeleton(home: &Path) -> Result<bool> {
    let knowledge = home.join("claude_global_memory").join("knowledge");
    fs::create_dir_all(&knowledge).with_context(|| format!("Failed to create {:?}", knowledge))?;
    let readme = knowledge.join("README.md");
    if readme.exists() {
        return Ok(false);
    }
    fs::write(&readme, KNOWLEDGE_README)?;
    Ok(true)
}

/// `.env.example` → `.env` kopyalar; `.env` varsa ASLA ezmez.
/// Kopyalandıysa true döner.
fn ensure_env_file(dir: &Path) -> Result<bool> {
    let example = dir.join(".env.example");
    let env_file = dir.join(".env");
    if env_file.exists() || !example.exists() {
        return Ok(false);
    }
    fs::copy(&example, &env_file)?;
    Ok(true)
}

pub fn run_install(skip_prereqs: bool, hooks: bool, home_override: Option<String>) -> Result<()> {
    println!("{}", "Claude Code Rust Complete Setup".cyan().bold());
    println!("========================================");

    let home = get_home_dir(home_override)?;
    let current_dir = env::current_dir().context("Failed to get current working directory")?;

    if !skip_prereqs {
        log_info("Checking prerequisites...");
        for (cmd, label) in [
            ("git", "Git"),
            ("node", "Node.js"),
            ("uv", "UV (Python package manager)"),
            ("claude", "Claude Code CLI"),
        ] {
            if check_cmd(cmd) {
                log_success(&format!("{} is installed", label));
            } else {
                log_warning(&format!("{} command not found in PATH", label));
            }
        }
        if check_cmd("python3") || check_cmd("python") {
            log_success("Python is installed");
        } else {
            log_warning("Python command not found in PATH");
        }
    }

    // NOT: Bu komut eskiden repo icindeki `config/` ve `global_memory/` iceriklerini
    // kullanicinin ~/.claude, ~/.config/claude-code ve ~/claude_global_memory dizinlerine
    // KOPYALIYORDU. O icerik ust-projeden (baska bir makinenin kisisel yapilandirmasi)
    // miras kalmisti ve kullanicinin kendi ~/.claude/CLAUDE.md dosyasini eziyordu.
    // Bagimsiz catal ile birlikte o icerik depodan cikarildi; kopyalama da kaldirildi.

    log_info("Setting up global memory knowledge base...");
    if ensure_knowledge_skeleton(&home)? {
        log_success("Created ~/claude_global_memory/knowledge with seed README");
    } else {
        log_success("Knowledge base already present (untouched)");
    }

    log_info("Setting up environment file...");
    if ensure_env_file(&current_dir)? {
        log_success("Created .env from .env.example — fill in your API keys");
    } else if current_dir.join(".env").exists() {
        log_success(".env already exists (never overwritten)");
    } else {
        log_warning("No .env.example found in repository root");
    }

    if hooks {
        log_info("Installing pre-commit security hooks into current repository...");
        crate::security::install_git_hooks(None)?;
    }

    println!("========================================");
    println!(
        "{}",
        "✅ Setup completed successfully via Rust engine!"
            .green()
            .bold()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_knowledge_skeleton_creates_and_preserves() {
        let dir = tempfile::tempdir().unwrap();
        let home = dir.path();

        // İlk çağrı: iskelet + tohum README
        assert!(ensure_knowledge_skeleton(home).unwrap());
        let readme = home
            .join("claude_global_memory")
            .join("knowledge")
            .join("README.md");
        assert!(readme.exists());

        // Kullanıcı içeriği değiştirdi — ikinci çağrı EZMEMELİ
        fs::write(&readme, "# benim notum").unwrap();
        assert!(!ensure_knowledge_skeleton(home).unwrap());
        assert_eq!(fs::read_to_string(&readme).unwrap(), "# benim notum");
    }

    #[test]
    fn test_ensure_env_file_never_overwrites() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();

        // .env.example yokken no-op
        assert!(!ensure_env_file(root).unwrap());

        fs::write(root.join(".env.example"), "A=example").unwrap();
        assert!(ensure_env_file(root).unwrap());
        assert_eq!(fs::read_to_string(root.join(".env")).unwrap(), "A=example");

        // Kullanıcı .env'i düzenledi — ezilmemeli
        fs::write(root.join(".env"), "A=secret").unwrap();
        assert!(!ensure_env_file(root).unwrap());
        assert_eq!(fs::read_to_string(root.join(".env")).unwrap(), "A=secret");
    }
}
