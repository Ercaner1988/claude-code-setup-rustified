use serde_json::json;
use std::io::{self, BufRead, Write};

#[derive(Debug, serde::Deserialize)]
pub struct JsonRpcRequest {
    pub id: serde_json::Value,
    pub method: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

pub fn run_mcp_mode() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        if line.trim().is_empty() {
            continue;
        }

        match serde_json::from_str::<JsonRpcRequest>(&line) {
            Ok(request) => {
                let response = handle_request(&request);
                writeln!(stdout, "{}", response)?;
                stdout.flush()?;
            }
            Err(_) => {
                let error_response = json!({
                    "jsonrpc": "2.0",
                    "error": {
                        "code": -32700,
                        "message": "Parse error"
                    }
                });
                writeln!(stdout, "{}", error_response)?;
                stdout.flush()?;
            }
        }
    }

    Ok(())
}

fn handle_request(req: &JsonRpcRequest) -> String {
    let response = match req.method.as_str() {
        "initialize" => initialize(&req.id),
        "tools/list" => list_tools(&req.id),
        "tools/call" => call_tool(&req.id, &req.params),
        "resources/list" => list_resources(&req.id),
        _ => json_rpc_error(&req.id, -32601, "Method not found"),
    };

    response.to_string()
}

fn initialize(id: &serde_json::Value) -> serde_json::Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": {
            "protocolVersion": "2024-11-05",
            "capabilities": {
                "tools": {},
                "resources": {}
            },
            "serverInfo": {
                "name": "claude-code-setup",
                "version": env!("CARGO_PKG_VERSION")
            }
        }
    })
}

fn list_tools(id: &serde_json::Value) -> serde_json::Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": {
            "tools": [
                {
                    "name": "mcp_list",
                    "description": "List all configured MCP servers (claude-code, project, claude-desktop)",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "target": {
                                "type": "string",
                                "description": "Config target: claude-code, project, or claude-desktop",
                                "enum": ["claude-code", "project", "claude-desktop"]
                            }
                        }
                    }
                },
                {
                    "name": "mcp_add",
                    "description": "Add a new MCP server",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "target": {
                                "type": "string",
                                "description": "Config target: claude-code, project, or claude-desktop",
                                "enum": ["claude-code", "project", "claude-desktop"]
                            },
                            "name": {
                                "type": "string",
                                "description": "MCP server name"
                            },
                            "command": {
                                "type": "string",
                                "description": "Command to execute"
                            },
                            "args": {
                                "type": "array",
                                "items": { "type": "string" },
                                "description": "Command arguments"
                            }
                        },
                        "required": ["name", "command"]
                    }
                },
                {
                    "name": "security_audit",
                    "description": "Scan for security issues (tokens, permissions, hooks)",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "fix": {
                                "type": "boolean",
                                "description": "Auto-remediate common issues"
                            }
                        }
                    }
                },
                {
                    "name": "memory_note",
                    "description": "Add a semantic memory note",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "text": {
                                "type": "string",
                                "description": "Note content"
                            }
                        },
                        "required": ["text"]
                    }
                },
                {
                    "name": "memory_index",
                    "description": "Index global memory Markdown notes into the searchable database (run before memory_search)",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                },
                {
                    "name": "memory_search",
                    "description": "Search semantic memory and knowledge base",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "query": {
                                "type": "string",
                                "description": "Search query"
                            }
                        },
                        "required": ["query"]
                    }
                },
                {
                    "name": "status",
                    "description": "Real-time health check of Claude Code environment",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                },
                {
                    "name": "test",
                    "description": "Run comprehensive test suite",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                }
            ]
        }
    })
}

/// MCP araclari CLI islevlerinin ta kendisini kullanir. MCP modunda stdout
/// JSON-RPC kanali oldugu icin islevleri dogrudan cagiramayiz (ciktilari akisi
/// bozardi), bu yuzden kendi ikili dosyamizi alt surec olarak calistiriyoruz.
/// ponytail: 7 islevi metin donduren ikizlere bolmek yerine tek yardimci.
fn run_cli(args: &[String]) -> (String, bool) {
    let exe = match std::env::current_exe() {
        Ok(path) => path,
        Err(err) => return (format!("Could not locate own executable: {err}"), true),
    };

    match std::process::Command::new(exe)
        .args(args)
        .env("NO_COLOR", "1")
        .output()
    {
        Ok(out) => {
            let mut text = String::from_utf8_lossy(&out.stdout).trim_end().to_string();
            let stderr = String::from_utf8_lossy(&out.stderr);
            if !stderr.trim().is_empty() {
                if !text.is_empty() {
                    text.push('\n');
                }
                text.push_str(stderr.trim_end());
            }
            if text.is_empty() {
                text = format!(
                    "Command finished with exit code {}",
                    out.status.code().unwrap_or(-1)
                );
            }
            (text, !out.status.success())
        }
        Err(err) => (format!("Failed to run `{}`: {err}", args.join(" ")), true),
    }
}

/// Arac cagrisini CLI argumanlarina cevirir.
fn tool_to_cli_args(tool: &str, args: &serde_json::Value) -> Result<Vec<String>, String> {
    let text_of = |key: &str| {
        args.get(key)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string()
    };

    let target = match text_of("target") {
        t if t.is_empty() => "claude-code".to_string(),
        t => t,
    };

    match tool {
        "status" => Ok(vec!["status".to_string()]),
        "test" => Ok(vec!["test".to_string()]),
        "mcp_list" => Ok(vec!["mcp-list".to_string(), "--target".to_string(), target]),
        "mcp_add" => {
            let name = text_of("name");
            let command = text_of("command");
            if name.is_empty() || command.is_empty() {
                return Err("mcp_add requires both 'name' and 'command'".to_string());
            }
            let mut out = vec![
                "mcp-set".to_string(),
                name,
                "--command".to_string(),
                command,
            ];
            if let Some(list) = args.get("args").and_then(|a| a.as_array()) {
                for item in list.iter().filter_map(|a| a.as_str()) {
                    out.push("--arg".to_string());
                    out.push(item.to_string());
                }
            }
            out.push("--target".to_string());
            out.push(target);
            Ok(out)
        }
        "security_audit" => {
            let mut out = vec!["security-audit".to_string()];
            if args.get("fix").and_then(|f| f.as_bool()).unwrap_or(false) {
                out.push("--fix".to_string());
            }
            Ok(out)
        }
        "memory_note" => {
            let text = text_of("text");
            if text.is_empty() {
                return Err("memory_note requires 'text'".to_string());
            }
            // Ilk satir baslik olur (dosya adi ondan turetiliyor), tamami govde.
            let title = text.lines().next().unwrap_or(&text).trim().to_string();
            let mut out = vec!["memory-note".to_string(), title.clone()];
            if title != text {
                out.push("--body".to_string());
                out.push(text);
            }
            Ok(out)
        }
        "memory_index" => Ok(vec!["memory-index".to_string()]),
        "memory_search" => {
            let query = text_of("query");
            if query.is_empty() {
                return Err("memory_search requires 'query'".to_string());
            }
            Ok(vec!["memory-search".to_string(), query])
        }
        other => Err(format!("Unknown tool '{other}'")),
    }
}

fn call_tool(id: &serde_json::Value, params: &serde_json::Value) -> serde_json::Value {
    let tool_name = params
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    let args = params
        .get("arguments")
        .cloned()
        .unwrap_or_else(|| json!({}));

    let (text, is_error) = match tool_to_cli_args(tool_name, &args) {
        Ok(cli_args) => run_cli(&cli_args),
        Err(message) => (message, true),
    };

    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": {
            "type": "tool_result",
            "content": [{ "type": "text", "text": text }],
            "isError": is_error
        }
    })
}

fn list_resources(id: &serde_json::Value) -> serde_json::Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": {
            "resources": [
                {
                    "uri": "claude-code://help",
                    "name": "Help",
                    "description": "claude-code-setup CLI help and usage"
                },
                {
                    "uri": "claude-code://config",
                    "name": "Configuration",
                    "description": "Current Claude Code setup configuration"
                }
            ]
        }
    })
}

fn json_rpc_error(id: &serde_json::Value, code: i32, message: &str) -> serde_json::Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": code,
            "message": message
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        let id = json!(1);
        let resp = initialize(&id);
        assert_eq!(
            resp.get("result")
                .and_then(|r| r.get("serverInfo"))
                .and_then(|s| s.get("name"))
                .and_then(|n| n.as_str()),
            Some("claude-code-setup")
        );
    }

    #[test]
    fn test_list_tools() {
        let id = json!(2);
        let resp = list_tools(&id);
        let tools = resp.get("result").and_then(|r| r.get("tools"));
        assert!(tools.is_some());
    }

    #[test]
    fn her_arac_gercek_bir_cli_komutuna_esleniyor() {
        // Yer tutucu metin donemine geri dusmeyi engeller: tools/list'te ilan
        // edilen her arac, calistirilabilir bir CLI alt komutu uretmeli.
        let declared = list_tools(&json!(1));
        let tools = declared["result"]["tools"].as_array().unwrap().clone();
        assert!(!tools.is_empty());

        let sample = json!({
            "name": "denek",
            "command": "echo",
            "target": "claude-code",
            "text": "baslik",
            "query": "sorgu"
        });

        for tool in tools {
            let name = tool["name"].as_str().unwrap();
            let args = tool_to_cli_args(name, &sample)
                .unwrap_or_else(|e| panic!("{name} eslenemedi: {e}"));
            assert!(!args.is_empty(), "{name} bos arguman uretti");
            assert!(
                !args[0].contains('_'),
                "{name} alt komutu tire kullanmali, bulundu: {}",
                args[0]
            );
        }
    }

    #[test]
    fn bilinmeyen_arac_ve_eksik_parametre_hata_verir() {
        assert!(tool_to_cli_args("olmayan_arac", &json!({})).is_err());
        assert!(tool_to_cli_args("memory_search", &json!({})).is_err());
        assert!(tool_to_cli_args("mcp_add", &json!({ "name": "x" })).is_err());
    }

    #[test]
    fn memory_note_ilk_satiri_baslik_kalanini_govde_yapar() {
        let args = tool_to_cli_args(
            "memory_note",
            &json!({ "text": "Baslik
Govde metni" }),
        )
        .unwrap();
        assert_eq!(args[0], "memory-note");
        assert_eq!(args[1], "Baslik");
        assert_eq!(args[2], "--body");
    }
}
