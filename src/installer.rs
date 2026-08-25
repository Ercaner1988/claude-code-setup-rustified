use anyhow::{Context, Result};
use chrono::Local;
use colored::*;
use std::env;
use std::fs;
use std::path::PathBuf;
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

pub fn run_install(skip_prereqs: bool, home_override: Option<String>) -> Result<()> {
    println!("{}", "Claude Code Rust Complete Setup".cyan().bold());
    println!("========================================");

    let home = get_home_dir(home_override)?;
    let current_dir = env::current_dir().context("Failed to get current working directory")?;

    if !skip_prereqs {
        log_info("Checking prerequisites...");
        if check_cmd("git") {
            log_success("Git is installed");
        } else {
            log_warning("Git command not found in PATH");
        }

        if check_cmd("node") {
            log_success("Node.js is installed");
        } else {
            log_warning("Node.js command not found in PATH");
        }

        if check_cmd("python3") || check_cmd("python") {
            log_success("Python is installed");
        } else {
            log_warning("Python command not found in PATH");
        }

        if check_cmd("uv") {
            log_success("UV (Python package manager) is installed");
        } else {
            log_warning("UV command not found in PATH");
        }
    }

    // NOT: Bu komut eskiden repo icindeki `config/` ve `global_memory/` iceriklerini
    // kullanicinin ~/.claude, ~/.config/claude-code ve ~/claude_global_memory dizinlerine
    // KOPYALIYORDU. O icerik ust-projeden (baska bir makinenin kisisel yapilandirmasi)
    // miras kalmisti ve kullanicinin kendi ~/.claude/CLAUDE.md dosyasini eziyordu.
    // Bagimsiz catal ile birlikte o icerik depodan cikarildi; kopyalama da kaldirildi.
    // Artik `install` yalnizca on-kosullari dogrular ve .env kurulumunu yapar.

    log_info("Setting up environment file...");

    let env_src = current_dir.join(".env");
    let env_dst = home.join(".env.claude");
    if env_src.exists() {
        fs::copy(&env_src, &env_dst)?;
        log_success("Configured .env.claude environment file");
    } else {
        log_warning(
            "No .env file found in repository root. Copy .env.example to .env to set up secrets.",
        );
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
