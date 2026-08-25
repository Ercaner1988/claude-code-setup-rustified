use anyhow::{bail, Context, Result};
use colored::*;
use serde_json::{json, Map, Value};
use std::fs;
use std::path::{Path, PathBuf};

use crate::installer::get_home_dir;

pub fn resolve_config_path(home_override: Option<String>) -> Result<PathBuf> {
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

pub fn list_mcp_servers(home_override: Option<String>) -> Result<()> {
    let path = resolve_config_path(home_override)?;
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
    home_override: Option<String>,
) -> Result<()> {
    let path = resolve_config_path(home_override)?;
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
    home_override: Option<String>,
) -> Result<()> {
    let path = resolve_config_path(home_override)?;
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

pub fn mcp_toggle(server_name: &str, disable: bool, home_override: Option<String>) -> Result<()> {
    let path = resolve_config_path(home_override)?;
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
        let result = mcp_unset("my_srv", vec![], false, false, home_override.clone());
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
        mcp_unset("my_srv", vec![], false, true, home_override.clone()).unwrap();

        // Sunucu silinmiş olmalı
        let val: Value = serde_json::from_str(&fs::read_to_string(&cfg_path).unwrap()).unwrap();
        assert!(val["mcpServers"]["my_srv"].is_null());

        // .bak oluşmuş olmalı
        assert!(cfg_dir.join("claude_desktop_config.json.bak").exists());
    }
}
