use anyhow::Result;
use colored::*;
use rusqlite::Connection;
use serde_json::Value;
use std::env;
use std::fs;
use std::process::Command;

use crate::installer::get_home_dir;
use crate::memory_engine::get_db_path;

pub fn run_tests(home_override: Option<String>) -> Result<()> {
    println!(
        "{}",
        "Claude Code Deployment Diagnostic Suite".cyan().bold()
    );
    println!("===============================================");

    let home = get_home_dir(home_override.clone())?;

    // Test 1: Claude CLI
    print!("Testing Claude CLI... ");
    if Command::new("claude").arg("--version").output().is_ok() {
        println!("{}", "✓ Installed".green());
    } else {
        println!("{}", "✗ Not found in PATH".red());
    }

    // Test 2: Claude Code user config (~/.claude.json) + MCP sunucu sayısı
    print!("Testing Claude Code config... ");
    let claude_json = home.join(".claude.json");
    if claude_json.exists() {
        match fs::read_to_string(&claude_json)
            .ok()
            .and_then(|c| serde_json::from_str::<Value>(&c).ok())
        {
            Some(v) => {
                let count = v
                    .get("mcpServers")
                    .and_then(|s| s.as_object())
                    .map(|o| o.len())
                    .unwrap_or(0);
                println!(
                    "{} ({} MCP servers in ~/.claude.json)",
                    "✓ Found".green(),
                    count
                );
            }
            None => println!("{}", "⚠ Present but invalid JSON".yellow()),
        }
    } else {
        println!("{}", "✗ Missing (~/.claude.json)".red());
    }

    // Test 3: Proje-seviyesi .mcp.json (varsa bilgi)
    print!("Testing project .mcp.json... ");
    let project_cfg = env::current_dir()?.join(".mcp.json");
    if project_cfg.exists() {
        println!("{}", "✓ Found in current directory".green());
    } else {
        println!("{}", "- None in current directory".dimmed());
    }

    // Test 4: Memory engine DB + not sayısı
    print!("Testing Memory Engine... ");
    let db_path = get_db_path(home_override)?;
    if db_path.exists() {
        match Connection::open(&db_path).and_then(|conn| {
            conn.query_row("SELECT COUNT(*) FROM knowledge_notes", [], |row| {
                row.get::<_, i64>(0)
            })
        }) {
            Ok(count) => println!("{} ({} notes indexed)", "✓ Found".green(), count),
            Err(_) => println!("{} (database present, not yet indexed)", "⚠".yellow()),
        }
    } else {
        println!("{}", "✗ Missing (run memory-index)".red());
    }

    // Test 5: Embedding model cache
    print!("Testing embedding model cache... ");
    let cache_candidates = [
        home.join(".cache").join("fastembed"),
        home.join(".cache").join("huggingface"),
    ];
    if cache_candidates.iter().any(|p| p.exists()) {
        println!("{}", "✓ Model cached locally".green());
    } else {
        println!(
            "{}",
            "⚠ Not downloaded yet (first memory-index/search will fetch it)".yellow()
        );
    }

    // Test 6: Pre-commit hook (mevcut repo)
    print!("Testing pre-commit security hook... ");
    let hook = env::current_dir()?.join(".git").join("hooks").join("pre-commit");
    if hook.exists() {
        println!("{}", "✓ Installed".green());
    } else {
        println!(
            "{}",
            "⚠ Missing (run install-hooks or security-audit --fix)".yellow()
        );
    }

    // Test 7: Ortam değişkenleri
    print!("Testing Environment Variables... ");
    if env::var("GITHUB_TOKEN").is_ok() || env::var("ANTHROPIC_API_KEY").is_ok() {
        println!("{}", "✓ Set".green());
    } else {
        println!(
            "{}",
            "⚠ API Keys not set in shell environment".yellow()
        );
    }

    println!("===============================================");
    println!("{}", "Diagnostic verification completed!".green().bold());

    Ok(())
}
