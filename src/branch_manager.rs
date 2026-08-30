use anyhow::{bail, Context, Result};
use chrono::Local;
use colored::*;
use std::process::Command;

const PROTECTED_BRANCHES: &[&str] = &["main", "master"];

pub fn is_protected_branch(branch: &str) -> bool {
    PROTECTED_BRANCHES.contains(&branch)
}

/// Kebab-case'e çevirir: "Fix Login Bug!" -> "fix-login-bug"
pub fn sanitize_description(description: &str) -> String {
    let mut out = String::with_capacity(description.len());
    let mut last_dash = false;
    for c in description.chars().flat_map(|c| c.to_lowercase()) {
        if c.is_ascii_alphanumeric() {
            out.push(c);
            last_dash = false;
        } else if !last_dash && !out.is_empty() {
            out.push('-');
            last_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    out
}

/// Bir git komutunu çalıştırır; çıkış kodu başarısızsa stderr ile hata verir.
fn run_git(args: &[&str]) -> Result<()> {
    let output = Command::new("git")
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute git {}", args.join(" ")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git {} failed: {}", args.join(" "), stderr.trim());
    }
    Ok(())
}

fn git_stdout(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .with_context(|| format!("Failed to execute git {}", args.join(" ")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git {} failed: {}", args.join(" "), stderr.trim());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn get_current_branch() -> Result<String> {
    git_stdout(&["branch", "--show-current"])
}

/// Remote'un varsayılan dalını çözer (origin/HEAD). Yoksa "main"e düşer.
fn default_remote_branch() -> String {
    if let Ok(reference) = git_stdout(&["symbolic-ref", "--short", "refs/remotes/origin/HEAD"]) {
        if let Some(branch) = reference.strip_prefix("origin/") {
            if !branch.is_empty() {
                return branch.to_string();
            }
        }
    }
    "main".to_string()
}

pub fn ensure_safe_branch() -> Result<String> {
    let current = get_current_branch()?;
    if is_protected_branch(&current) {
        let timestamp = Local::now().format("%Y%m%d-%H%M%S").to_string();
        let safe_branch = format!("work/{}", timestamp);
        println!(
            "{} Protected branch '{}' detected! Creating safe branch '{}'...",
            "⚠".yellow().bold(),
            current,
            safe_branch
        );
        run_git(&["checkout", "-b", &safe_branch])?;
        Ok(safe_branch)
    } else {
        Ok(current)
    }
}

pub fn create_feature_branch(branch_type: &str, description: &str) -> Result<String> {
    let timestamp = Local::now().format("%Y%m%d-%H%M%S").to_string();
    let sanitized_desc = sanitize_description(description);
    let branch_name = format!("{}/{}-{}", branch_type, timestamp, sanitized_desc);

    println!(
        "{} Creating feature branch: {}",
        "✓".green().bold(),
        branch_name.cyan()
    );

    run_git(&["fetch", "origin"])?;
    let base = format!("origin/{}", default_remote_branch());
    run_git(&["checkout", "-b", &branch_name, &base])?;

    Ok(branch_name)
}

pub fn safe_commit(message: &str, files: &[String]) -> Result<()> {
    ensure_safe_branch()?;
    let mut add_args: Vec<&str> = vec!["add", "--"];
    add_args.extend(files.iter().map(|f| f.as_str()));
    run_git(&add_args)?;
    run_git(&["commit", "-m", message])?;
    println!("{} Committed: {}", "✓".green().bold(), message);
    Ok(())
}

pub fn safe_push() -> Result<()> {
    let branch = get_current_branch()?;
    if is_protected_branch(&branch) {
        bail!(
            "BLOCKED: Cannot push directly to protected branch '{}'",
            branch
        );
    }
    run_git(&["push", "-u", "origin", &branch])?;
    println!(
        "{} Pushed branch '{}' to origin",
        "✓".green().bold(),
        branch
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_protected_branch() {
        assert!(is_protected_branch("main"));
        assert!(is_protected_branch("master"));
        assert!(!is_protected_branch("feature/x"));
        assert!(!is_protected_branch("mainly"));
    }

    #[test]
    fn test_sanitize_description() {
        assert_eq!(sanitize_description("Fix Login Bug!"), "fix-login-bug");
        assert_eq!(
            sanitize_description("  multiple   spaces  "),
            "multiple-spaces"
        );
        assert_eq!(sanitize_description("already-good"), "already-good");
        assert_eq!(sanitize_description("!!!"), "");
        assert_eq!(
            sanitize_description("Dots...and---dashes"),
            "dots-and-dashes"
        );
    }
}
