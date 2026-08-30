use anyhow::{bail, Context, Result};
use clap::ValueEnum;
use colored::*;
use serde_json::{json, Map, Value};
use std::fs;
use std::path::{Path, PathBuf};

use crate::installer::get_home_dir;

/// MCP yapılandırma hedefi: Claude Code (kullanıcı), proje veya Claude Desktop.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum McpTarget {
    /// Claude Code user config (~/.claude.json)
    ClaudeCode,
    /// Project-scoped config (./.mcp.json in current directory)
    Project,
    /// Claude Desktop config (claude_desktop_config.json)
    ClaudeDesktop,
}

pub fn resolve_config_path(target: McpTarget, home_override: Option<String>) -> Result<PathBuf> {
    match target {
        McpTarget::ClaudeCode => {
            let home = get_home_dir(home_override)?;
            Ok(home.join(".claude.json"))
        }
        McpTarget::Project => Ok(std::env::current_dir()
            .context("Failed to get current working directory")?
            .join(".mcp.json")),
        McpTarget::ClaudeDesktop => {
            let home = get_home_dir(home_override)?;
            let primary = home
                .join(".config")
                .join("claude-code")
                .join("claude_desktop_config.json");
            if primary.exists() {
                return Ok(primary);
            }
            let alt = home
                .join("AppData")
                .join("Roaming")
                .join("Claude")
                .join("claude_desktop_config.json");
            if alt.exists() {
                return Ok(alt);
            }
            Ok(primary)
        }
    }
}

fn read_json_value(path: &Path) -> Result<Value> {
    if !path.exists() {
        bail!("MCP configuration file not found at {:?}", path);
    }
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read MCP config from {:?}", path))?;
    let val: Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from {:?}", path))?;
    Ok(val)
}

fn save_json_value_atomically(path: &Path, val: &Value) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    if path.exists() {
        let bak_path = path.with_extension("json.bak");
        fs::copy(path, &bak_path)
            .with_context(|| format!("Failed to create backup at {:?}", bak_path))?;
    }

    let pretty_str = serde_json::to_string_pretty(val)?;
    let temp_path = path.with_extension("tmp");
    fs::write(&temp_path, pretty_str)?;
    fs::rename(&temp_path, path)
        .with_context(|| format!("Failed to atomically replace {:?}", path))?;

    Ok(())
}

pub fn list_mcp_servers(target: McpTarget, home_override: Option<String>) -> Result<()> {
    let path = resolve_config_path(target, home_override)?;
    if !path.exists() {
        println!("{} MCP config file not found at {:?}", "✗".red(), path);
        return Ok(());
    }

    let val = read_json_value(&path)?;
    let empty_map = Map::new();
    let servers = val
        .get("mcpServers")
        .and_then(|s| s.as_object())
        .unwrap_or(&empty_map);

    println!("{}", "Configured MCP Servers".cyan().bold());
    println!("Target: {}", format!("{:?}", target).dimmed());
    println!("Config: {}", path.display().to_string().dimmed());
    println!("========================================");

    for (name, server) in servers {
        let is_disabled = server
            .get("disabled")
            .and_then(|d| d.as_bool())
            .unwrap_or(false);
        let status_str = if is_disabled {
            " [DISABLED]".red().to_string()
        } else {
            "".to_string()
        };

        let cmd = server.get("command").and_then(|c| c.as_str()).unwrap_or("");
        let args: Vec<String> = server
            .get("args")
            .and_then(|a| a.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        println!(
            "• {}{}: {} {}",
            name.green().bold(),
            status_str,
            cmd,
            args.join(" ")
        );

        if let Some(env_map) = server.get("env").and_then(|e| e.as_object()) {
            for (k, v) in env_map {
                println!("    env: {}={}", k.dimmed(), v);
            }
        }
    }
    println!("========================================");
    println!(
        "Total servers: {}",
        servers.len().to_string().yellow().bold()
    );
    Ok(())
}

pub fn mcp_set(
    server_name: &str,
    command: Option<String>,
    args: Vec<String>,
    env_vars: Vec<String>,
    target: McpTarget,
    home_override: Option<String>,
) -> Result<()> {
    let path = resolve_config_path(target, home_override)?;
    let mut val = if path.exists() {
        read_json_value(&path)?
    } else {
        json!({ "mcpServers": {} })
    };

    if !val.is_object() {
        val = json!({ "mcpServers": {} });
    }

    if val.get("mcpServers").is_none() {
        val["mcpServers"] = json!({});
    }

    let servers = val["mcpServers"]
        .as_object_mut()
        .context("mcpServers is not an object")?;

    let server_val = servers.entry(server_name).or_insert_with(|| json!({}));
    if !server_val.is_object() {
        *server_val = json!({});
    }

    if let Some(cmd) = command {
        server_val["command"] = json!(cmd);
    }

    if !args.is_empty() {
        server_val["args"] = json!(args);
    }

    if !env_vars.is_empty() {
        if server_val.get("env").is_none() {
            server_val["env"] = json!({});
        }
        let env_obj = server_val["env"]
            .as_object_mut()
            .context("env is not an object")?;
        for kv in env_vars {
            let mut parts = kv.splitn(2, '=');
            let k = parts.next().unwrap_or("").trim();
            let v = parts.next().unwrap_or("").trim();
            if !k.is_empty() {
                env_obj.insert(k.to_string(), json!(v));
            }
        }
    }

    save_json_value_atomically(&path, &val)?;
    println!(
        "{} Updated server '{}' in MCP configuration.",
        "✓".green().bold(),
        server_name
    );
    Ok(())
}

pub fn mcp_unset(
    server_name: &str,
    env_keys: Vec<String>,
    clear_args: bool,
    remove: bool,
    target: McpTarget,
    home_override: Option<String>,
) -> Result<()> {
    let path = resolve_config_path(target, home_override)?;
    let mut val = read_json_value(&path)?;

    let servers = val
        .get_mut("mcpServers")
        .and_then(|s| s.as_object_mut())
        .context("mcpServers not found")?;

    if !servers.contains_key(server_name) {
        bail!("Server '{}' not found in config", server_name);
    }

    // A3: Bayraksız çağrıda sunucuyu silme — --remove şart
    if remove {
        servers.remove(server_name);
        save_json_value_atomically(&path, &val)?;
        println!(
            "{} Removed server '{}' completely.",
            "✓".green().bold(),
            server_name
        );
        return Ok(());
    }

    if env_keys.is_empty() && !clear_args {
        bail!(
            "No fields specified for server '{}'. Use --env or --clear-args to modify fields, or --remove to delete the server entirely.",
            server_name
        );
    }

    let server_val = servers.get_mut(server_name).unwrap();

    if clear_args {
        if let Some(obj) = server_val.as_object_mut() {
            obj.remove("args");
        }
    }

    if !env_keys.is_empty() {
        if let Some(env_obj) = server_val.get_mut("env").and_then(|e| e.as_object_mut()) {
            for k in env_keys {
                env_obj.remove(&k);
            }
        }
    }
    println!(
        "{} Updated fields for server '{}'.",
        "✓".green().bold(),
        server_name
    );

    save_json_value_atomically(&path, &val)?;
    Ok(())
}

pub fn mcp_toggle(
    server_name: &str,
    disable: bool,
    target: McpTarget,
    home_override: Option<String>,
) -> Result<()> {
    let path = resolve_config_path(target, home_override)?;
    let mut val = read_json_value(&path)?;

    let servers = val
        .get_mut("mcpServers")
        .and_then(|s| s.as_object_mut())
        .context("mcpServers not found")?;

    if let Some(server_val) = servers.get_mut(server_name) {
        if disable {
            server_val["disabled"] = json!(true);
            println!("{} Disabled server '{}'.", "✓".yellow().bold(), server_name);
        } else if let Some(obj) = server_val.as_object_mut() {
            obj.remove("disabled");
            println!("{} Enabled server '{}'.", "✓".green().bold(), server_name);
        }
        save_json_value_atomically(&path, &val)?;
    } else {
        bail!("Server '{}' not found in config", server_name);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_value_preservation_roundtrip() {
        let raw = r#"{
            "mcpServers": {
                "custom_srv": {
                    "command": "node",
                    "disabled": true,
                    "unknown_field": 12345,
                    "env": {
                        "FOO": "bar"
                    }
                }
            }
        }"#;

        let dir = tempdir().unwrap();
        let cfg_dir = dir.path().join(".config").join("claude-code");
        fs::create_dir_all(&cfg_dir).unwrap();
        let cfg_path = cfg_dir.join("claude_desktop_config.json");
        fs::write(&cfg_path, raw).unwrap();

        let home_override = Some(dir.path().to_string_lossy().to_string());

        // mcp_set ile yeni env ekle
        mcp_set(
            "custom_srv",
            None,
            vec![],
            vec!["NEW_KEY=val".to_string()],
            McpTarget::ClaudeDesktop,
            home_override.clone(),
        )
        .unwrap();

        let updated_raw = fs::read_to_string(&cfg_path).unwrap();
        let val: Value = serde_json::from_str(&updated_raw).unwrap();

        // Bilinmeyen alan ve disabled korundu mu?
        assert_eq!(val["mcpServers"]["custom_srv"]["unknown_field"], 12345);
        assert_eq!(val["mcpServers"]["custom_srv"]["disabled"], true);
        assert_eq!(val["mcpServers"]["custom_srv"]["env"]["NEW_KEY"], "val");
        assert_eq!(val["mcpServers"]["custom_srv"]["env"]["FOO"], "bar");

        // .bak dosyası oluştu mu?
        assert!(cfg_dir.join("claude_desktop_config.json.bak").exists());
    }

    #[test]
    fn test_mcp_unset_without_remove_flag_rejects() {
        let raw = r#"{"mcpServers": {"my_srv": {"command": "node"}}}"#;

        let dir = tempdir().unwrap();
        let cfg_dir = dir.path().join(".config").join("claude-code");
        fs::create_dir_all(&cfg_dir).unwrap();
        let cfg_path = cfg_dir.join("claude_desktop_config.json");
        fs::write(&cfg_path, raw).unwrap();

        let home_override = Some(dir.path().to_string_lossy().to_string());

        // A3: bayraksız çağrı reddedilmeli
        let result = mcp_unset(
            "my_srv",
            vec![],
            false,
            false,
            McpTarget::ClaudeDesktop,
            home_override.clone(),
        );
        assert!(result.is_err());

        // Sunucu hâlâ yerinde olmalı
        let val: Value = serde_json::from_str(&fs::read_to_string(&cfg_path).unwrap()).unwrap();
        assert!(val["mcpServers"]["my_srv"].is_object());
    }

    #[test]
    fn test_mcp_unset_with_remove_flag_deletes() {
        let raw = r#"{"mcpServers": {"my_srv": {"command": "node", "custom_field": 42}}}"#;

        let dir = tempdir().unwrap();
        let cfg_dir = dir.path().join(".config").join("claude-code");
        fs::create_dir_all(&cfg_dir).unwrap();
        let cfg_path = cfg_dir.join("claude_desktop_config.json");
        fs::write(&cfg_path, raw).unwrap();

        let home_override = Some(dir.path().to_string_lossy().to_string());

        // --remove ile sil
        mcp_unset(
            "my_srv",
            vec![],
            false,
            true,
            McpTarget::ClaudeDesktop,
            home_override.clone(),
        )
        .unwrap();

        // Sunucu silinmiş olmalı
        let val: Value = serde_json::from_str(&fs::read_to_string(&cfg_path).unwrap()).unwrap();
        assert!(val["mcpServers"]["my_srv"].is_null());

        // .bak oluşmuş olmalı
        assert!(cfg_dir.join("claude_desktop_config.json.bak").exists());
    }

    #[test]
    fn test_resolve_config_path_targets() {
        let dir = tempdir().unwrap();
        let home_override = Some(dir.path().to_string_lossy().to_string());

        let cc = resolve_config_path(McpTarget::ClaudeCode, home_override.clone()).unwrap();
        assert_eq!(cc, dir.path().join(".claude.json"));

        let desktop = resolve_config_path(McpTarget::ClaudeDesktop, home_override.clone()).unwrap();
        assert!(desktop.ends_with("claude_desktop_config.json"));

        let project = resolve_config_path(McpTarget::Project, None).unwrap();
        assert!(project.ends_with(".mcp.json"));
    }

    #[test]
    fn test_claude_code_target_preserves_unknown_fields() {
        // ~/.claude.json benzeri: mcpServers dışındaki alanlar korunmalı
        let raw = r#"{
            "numStartups": 3,
            "oauthAccount": {"emailAddress": "x@y.z"},
            "mcpServers": {
                "existing_srv": {"command": "node", "args": ["server.js"]}
            }
        }"#;

        let dir = tempdir().unwrap();
        let cfg_path = dir.path().join(".claude.json");
        fs::write(&cfg_path, raw).unwrap();
        let home_override = Some(dir.path().to_string_lossy().to_string());

        mcp_set(
            "new_srv",
            Some("npx".to_string()),
            vec!["-y".to_string(), "pkg".to_string()],
            vec!["API_KEY=abc".to_string()],
            McpTarget::ClaudeCode,
            home_override.clone(),
        )
        .unwrap();

        let val: Value = serde_json::from_str(&fs::read_to_string(&cfg_path).unwrap()).unwrap();
        // Bilinmeyen üst-seviye alanlar korundu mu?
        assert_eq!(val["numStartups"], 3);
        assert_eq!(val["oauthAccount"]["emailAddress"], "x@y.z");
        // Eski sunucu korundu mu?
        assert_eq!(val["mcpServers"]["existing_srv"]["command"], "node");
        // Yeni sunucu doğru mu?
        assert_eq!(val["mcpServers"]["new_srv"]["command"], "npx");
        assert_eq!(val["mcpServers"]["new_srv"]["args"], json!(["-y", "pkg"]));
        assert_eq!(val["mcpServers"]["new_srv"]["env"]["API_KEY"], "abc");

        // .bak oluşmuş olmalı
        assert!(cfg_path.with_extension("json.bak").exists());

        // mcp_toggle ile disable/enable
        mcp_toggle(
            "existing_srv",
            true,
            McpTarget::ClaudeCode,
            home_override.clone(),
        )
        .unwrap();
        let val: Value = serde_json::from_str(&fs::read_to_string(&cfg_path).unwrap()).unwrap();
        assert_eq!(val["mcpServers"]["existing_srv"]["disabled"], true);
        assert_eq!(val["numStartups"], 3);
        mcp_toggle("existing_srv", false, McpTarget::ClaudeCode, home_override).unwrap();
        let val: Value = serde_json::from_str(&fs::read_to_string(&cfg_path).unwrap()).unwrap();
        assert!(val["mcpServers"]["existing_srv"]["disabled"].is_null());
    }
}
