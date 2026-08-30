use anyhow::{Context, Result};
use colored::*;
use regex::Regex;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::installer::get_home_dir;
use crate::mcp::{resolve_config_path, McpTarget};

const PRE_COMMIT_HOOK_CONTENT: &str = r#"#!/usr/bin/env bash
# Rust-generated pre-commit security hook for Claude Code setup
set -e

BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")

if [ "$BRANCH" = "main" ] || [ "$BRANCH" = "master" ]; then
    echo -e "\033[0;31m[SECURITY ERROR] Direct commit to '$BRANCH' branch is prohibited by security policy!\033[0m"
    echo "Please create a feature branch and open a PR instead."
    exit 1
fi

# Secret leakage scanner
if git diff --cached | grep -iE '(api_key|secret_key|password|github_token)\s*=\s*["'\''][a-zA-Z0-9_-]{16,}["'\'']'; then
    echo -e "\033[0;31m[SECURITY ERROR] Potential hardcoded API secret detected in staged changes!\033[0m"
    exit 1
fi

echo -e "\033[0;32m[SECURITY CHECK] Pre-commit security verification passed!\033[0m"
"#;

/// Bilinen gizli anahtar desenleri (ad, regex)
const SECRET_PATTERNS: &[(&str, &str)] = &[
    ("GitHub PAT (ghp_)", r"ghp_[A-Za-z0-9]{20,}"),
    ("GitHub fine-grained PAT", r"github_pat_[A-Za-z0-9_]{20,}"),
    ("OpenAI/Anthropic tarzı key (sk-)", r"sk-[A-Za-z0-9_-]{16,}"),
    ("Slack token (xox*)", r"xox[baprs]-[A-Za-z0-9-]{10,}"),
    ("AWS Access Key (AKIA)", r"AKIA[0-9A-Z]{16}"),
];

/// Metindeki gizli anahtar desenlerini bulur; (desen adı, maskeli eşleşme) döner.
pub fn find_secrets(text: &str) -> Vec<(String, String)> {
    let mut findings = Vec::new();
    for (name, pattern) in SECRET_PATTERNS {
        let re = Regex::new(pattern).unwrap();
        for m in re.find_iter(text) {
            let s = m.as_str();
            let masked = if s.len() > 8 { &s[..8] } else { &s[..4] };
            findings.push((name.to_string(), format!("{}…", masked)));
        }
    }
    findings
}

pub fn install_git_hooks(repo_dir: Option<String>) -> Result<()> {
    let target_dir = if let Some(dir) = repo_dir {
        PathBuf::from(dir)
    } else {
        env::current_dir().context("Failed to get current working directory")?
    };

    let git_hooks_dir = target_dir.join(".git").join("hooks");

    if !git_hooks_dir.exists() {
        println!(
            "{} Git hooks directory not found at {:?}",
            "✗".red(),
            git_hooks_dir
        );
        println!("Ensure this is a valid Git repository root.");
        return Ok(());
    }

    let pre_commit_path = git_hooks_dir.join("pre-commit");
    fs::write(&pre_commit_path, PRE_COMMIT_HOOK_CONTENT)?;

    println!(
        "{} Successfully installed pre-commit security & branch protection hook at {:?}",
        "✓".green().bold(),
        pre_commit_path
    );

    Ok(())
}

/// Unix'te dosya iznini 600 yapar; başka platformlarda no-op.
#[cfg(unix)]
fn enforce_file_permissions(path: &Path) -> Result<bool> {
    use std::os::unix::fs::PermissionsExt;
    let perms = fs::metadata(path)?.permissions();
    if perms.mode() & 0o777 != 0o600 {
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
        return Ok(true);
    }
    Ok(false)
}

fn scan_config_for_secrets(path: &Path, label: &str, findings: &mut usize) {
    if !path.exists() {
        return;
    }
    match fs::read_to_string(path) {
        Ok(content) => {
            let secrets = find_secrets(&content);
            if secrets.is_empty() {
                println!("{} {} — no plaintext secrets detected", "✓".green(), label);
            } else {
                for (name, masked) in &secrets {
                    *findings += 1;
                    println!(
                        "{} {} — PLAINTEXT SECRET: {} ({})",
                        "✗".red().bold(),
                        label,
                        name,
                        masked
                    );
                }
                println!(
                    "  {} Rotate these tokens immediately and move them to environment variables.",
                    "→".yellow()
                );
            }
        }
        Err(e) => println!("{} {} — could not read: {}", "⚠".yellow(), label, e),
    }
}

pub fn run_security_audit(fix: bool, home_override: Option<String>) -> Result<()> {
    let home = get_home_dir(home_override.clone())?;
    println!("{}", "Claude Code Security Audit".cyan().bold());
    if fix {
        println!("{}", "(--fix mode: auto-remediation enabled)".yellow());
    }
    println!("========================================");

    let mut findings = 0usize;

    // 1) MCP yapılandırmalarında plaintext secret taraması
    println!("{}", "1. Scanning MCP configs for plaintext secrets...".bold());
    let configs: Vec<(PathBuf, &str)> = vec![
        (home.join(".claude.json"), "Claude Code user config"),
        (
            env::current_dir()?.join(".mcp.json"),
            "Project .mcp.json",
        ),
        (
            resolve_config_path(McpTarget::ClaudeDesktop, home_override)?,
            "Claude Desktop config",
        ),
    ];
    for (path, label) in &configs {
        scan_config_for_secrets(path, label, &mut findings);
    }

    // 2) Config dosya izinleri (Unix: 600; Windows: bilgi notu)
    println!("{}", "2. Checking config file permissions...".bold());
    for (path, label) in &configs {
        if !path.exists() {
            continue;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = fs::metadata(path)?.permissions().mode() & 0o777;
            if mode == 0o600 {
                println!("{} {} — permissions 600", "✓".green(), label);
            } else if fix {
                enforce_file_permissions(path)?;
                println!(
                    "{} {} — permissions fixed ({:o} -> 600)",
                    "✓".green(),
                    label,
                    mode
                );
            } else {
                findings += 1;
                println!(
                    "{} {} — permissions {:o} (expected 600; run with --fix)",
                    "⚠".yellow(),
                    label,
                    mode
                );
            }
        }
        #[cfg(not(unix))]
        {
            println!(
                "{} {} — ACL-based permissions (manual review advised)",
                "ℹ".blue(),
                label
            );
        }
    }

    // 3) Mevcut repo'da pre-commit hook kurulu mu
    println!("{}", "3. Checking pre-commit security hook...".bold());
    let current_dir = env::current_dir()?;
    let hook_path = current_dir.join(".git").join("hooks").join("pre-commit");
    if hook_path.exists() {
        println!("{} pre-commit hook installed", "✓".green());
    } else if fix && current_dir.join(".git").is_dir() {
        install_git_hooks(None)?;
        println!("{} pre-commit hook installed by --fix", "✓".green());
    } else {
        findings += 1;
        println!(
            "{} pre-commit hook missing (run 'security-audit --fix' or 'install-hooks')",
            "⚠".yellow()
        );
    }

    // 4) Korumalı dal kontrolü
    println!("{}", "4. Checking current Git branch...".bold());
    if let Ok(output) = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
    {
        if output.status.success() {
            let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("Current Git branch: {}", branch.yellow().bold());
            if branch == "main" || branch == "master" {
                findings += 1;
                println!(
                    "{} Working directly on main branch! Use feature branches.",
                    "⚠".yellow()
                );
            }
        }
    }

    println!("========================================");
    if findings == 0 {
        println!("{}", "Security audit complete: no findings!".green().bold());
    } else {
        println!(
            "{}",
            format!("Security audit complete: {} finding(s).", findings)
                .yellow()
                .bold()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_secrets_matches_known_patterns() {
        let text = r#"
            token = "ghp_abcdefghij1234567890abcd"
            other = "sk-proj-abcdef1234567890"
            slack = "xoxb-1234567890-abcdefghij"
            aws = "AKIAIOSFODNN7EXAMPLE"
        "#;
        let findings = find_secrets(text);
        assert!(findings.iter().any(|(n, _)| n.contains("ghp_")));
        assert!(findings.iter().any(|(n, _)| n.contains("sk-")));
        assert!(findings.iter().any(|(n, _)| n.contains("Slack")));
        assert!(findings.iter().any(|(n, _)| n.contains("AWS")));
        // Maskeleme tam sırrı içermemeli
        for (_, masked) in &findings {
            assert!(masked.ends_with('…'));
            assert!(masked.chars().count() <= 9);
        }
    }

    #[test]
    fn test_find_secrets_clean_text() {
        let text = "hello world, no secrets here. API_KEY=${FROM_ENV}";
        assert!(find_secrets(text).is_empty());
    }

    #[cfg(unix)]
    #[test]
    fn test_enforce_file_permissions_fixes_mode() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("cfg.json");
        fs::write(&file, "{}").unwrap();
        fs::set_permissions(&file, fs::Permissions::from_mode(0o644)).unwrap();

        let changed = enforce_file_permissions(&file).unwrap();
        assert!(changed);
        let mode = fs::metadata(&file).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);

        // İkinci çağrı no-op
        let changed_again = enforce_file_permissions(&file).unwrap();
        assert!(!changed_again);
    }
}
