use clap::{Parser, Subcommand};

use crate::mcp::McpTarget;

#[derive(Parser)]
#[command(name = "claude-code-setup")]
#[command(about = "Bagimsiz Rust CLI: Claude Code ortam kurulumu, dinamik MCP yonetimi, semantik + graf hafiza motoru", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Verify prerequisites and set up memory skeleton + .env file
    Install {
        #[arg(short, long, help = "Skip prerequisite installation checks")]
        skip_prereqs: bool,

        #[arg(
            long,
            help = "Also install pre-commit security hooks into the current repo"
        )]
        hooks: bool,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Run full deployment verification diagnostic test suite
    Test {
        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// List all configured MCP servers and environment settings
    McpList {
        #[arg(long, value_enum, default_value_t = McpTarget::ClaudeCode, help = "Config target")]
        target: McpTarget,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Set or update configuration fields for an MCP server dynamically
    McpSet {
        #[arg(help = "Server name")]
        server: String,

        #[arg(short, long, help = "Command binary")]
        command: Option<String>,

        #[arg(short, long, help = "Arguments (multiple allowed)")]
        arg: Vec<String>,

        #[arg(short, long, help = "Environment variables (KEY=VALUE)")]
        env: Vec<String>,

        #[arg(long, value_enum, default_value_t = McpTarget::ClaudeCode, help = "Config target")]
        target: McpTarget,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Unset configuration fields or remove an MCP server
    McpUnset {
        #[arg(help = "Server name")]
        server: String,

        #[arg(short, long, help = "Environment variable keys to remove")]
        env: Vec<String>,

        #[arg(long, help = "Clear all command arguments")]
        clear_args: bool,

        /// Completely remove the server (required for deletion)
        #[arg(long, help = "Completely remove the server from config")]
        remove: bool,

        #[arg(long, value_enum, default_value_t = McpTarget::ClaudeCode, help = "Config target")]
        target: McpTarget,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Enable a disabled MCP server
    McpEnable {
        #[arg(help = "Server name")]
        server: String,

        #[arg(long, value_enum, default_value_t = McpTarget::ClaudeCode, help = "Config target")]
        target: McpTarget,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Disable an MCP server without removing its configuration
    McpDisable {
        #[arg(help = "Server name")]
        server: String,

        #[arg(long, value_enum, default_value_t = McpTarget::ClaudeCode, help = "Config target")]
        target: McpTarget,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Index global memory Markdown files into SQLite database
    MemoryIndex {
        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,

        /// Semantic edge threshold (default: 0.70)
        #[arg(
            long,
            default_value_t = 0.70,
            help = "Cosine similarity threshold for semantic edges"
        )]
        edge_threshold: f32,

        /// Source directory to index (repeatable). Defaults to <home>/claude_global_memory/knowledge
        #[arg(
            long,
            help = "Directory of .md notes to index; repeat for multiple sources"
        )]
        source: Vec<String>,
    },

    /// Search indexed global memory notes
    MemorySearch {
        #[arg(help = "Search query keyword")]
        query: String,

        #[arg(
            short,
            long,
            default_value = "hybrid",
            help = "Search mode: keyword, semantic, hybrid"
        )]
        mode: String,

        /// Maximum number of results (default: 5)
        #[arg(short, long, default_value_t = 5, help = "Maximum results to return")]
        limit: usize,

        /// Minimum score threshold (default: 0.30)
        #[arg(long, default_value_t = 0.30, help = "Minimum score threshold")]
        min_score: f64,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Show related notes via graph edges (wikilinks + semantic ties)
    MemoryRelated {
        #[arg(help = "Target note filename (e.g. SYSTEM-STATUS-AND-SETUP.md)")]
        note: String,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Add a new markdown note to the global memory knowledge base
    MemoryNote {
        #[arg(help = "Note title (used as # heading; kebab-case filename derived)")]
        title: String,

        #[arg(short, long, help = "Note body text")]
        body: Option<String>,

        #[arg(
            long,
            help = "Target directory override (default: <home>/claude_global_memory/knowledge)"
        )]
        dir: Option<String>,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Install security & Git pre-commit branch protection hooks
    InstallHooks {
        #[arg(short, long, help = "Target repository path")]
        repo_dir: Option<String>,
    },

    /// Run security audit on active configurations and Git state
    SecurityAudit {
        #[arg(long, help = "Auto-fix findings where possible (permissions, hooks)")]
        fix: bool,

        #[arg(long, help = "Custom home directory override")]
        home_dir: Option<String>,
    },

    /// Execute autonomous repository manager workflow
    AgentWorkflow {
        #[arg(short, long, default_value = "feature", help = "Branch type prefix")]
        branch_type: String,

        #[arg(short, long, help = "Workflow description")]
        description: String,

        #[arg(short, long, help = "Files to commit")]
        files: Vec<String>,
    },

    /// Display environment status summary
    Status,
}
