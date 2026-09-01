use serde_json::json;
use std::io::{self, BufRead, Write};

#[derive(Debug)]
pub struct McpServer;

#[derive(Debug, serde::Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
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

fn call_tool(id: &serde_json::Value, params: &serde_json::Value) -> serde_json::Value {
    let tool_name = params
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    let result = match tool_name {
        "mcp_list" => {
            json!({
                "type": "text",
                "text": "MCP servers can be listed using: claude-code-setup mcp-list [--target claude-code|project|claude-desktop]"
            })
        }
        "status" => {
            json!({
                "type": "text",
                "text": "Status check: Use 'claude-code-setup status' for detailed environment diagnostics"
            })
        }
        "security_audit" => {
            let fix = params
                .get("arguments")
                .and_then(|a| a.get("fix"))
                .and_then(|f| f.as_bool())
                .unwrap_or(false);

            let cmd = if fix {
                "claude-code-setup security-audit --fix"
            } else {
                "claude-code-setup security-audit"
            };

            json!({
                "type": "text",
                "text": format!("Run: {}", cmd)
            })
        }
        "memory_note" => {
            let text = params
                .get("arguments")
                .and_then(|a| a.get("text"))
                .and_then(|t| t.as_str())
                .unwrap_or("");

            json!({
                "type": "text",
                "text": format!("Add memory note: claude-code-setup memory-note \"{}\"", text)
            })
        }
        "memory_search" => {
            let query = params
                .get("arguments")
                .and_then(|a| a.get("query"))
                .and_then(|q| q.as_str())
                .unwrap_or("");

            json!({
                "type": "text",
                "text": format!("Search memory: claude-code-setup memory-search \"{}\"", query)
            })
        }
        _ => {
            json!({
                "type": "text",
                "text": format!("Tool '{}' not implemented in MCP mode", tool_name)
            })
        }
    };

    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": {
            "type": "tool_result",
            "content": [result]
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
}
