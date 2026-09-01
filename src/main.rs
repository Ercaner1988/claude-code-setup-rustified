mod agent;
mod branch_manager;
mod cli;
mod installer;
mod mcp;
mod mcp_server;
mod memory_engine;
mod security;
mod tester;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    // MCP mode: stdin/stdout JSON-RPC protocol
    if cli.mcp_mode {
        return mcp_server::run_mcp_mode();
    }

    // ponytail: subcommand yoksa yardim bas, panik yerine
    let Some(command) = cli.command else {
        Cli::command().print_help()?;
        return Ok(());
    };

    match command {
        Commands::Install {
            skip_prereqs,
            hooks,
            home_dir,
        } => {
            installer::run_install(skip_prereqs, hooks, home_dir)?;
        }
        Commands::Test { home_dir } => {
            tester::run_tests(home_dir)?;
        }
        Commands::McpList { target, home_dir } => {
            mcp::list_mcp_servers(target, home_dir)?;
        }
        Commands::McpSet {
            server,
            command,
            arg,
            env,
            target,
            home_dir,
        } => {
            mcp::mcp_set(&server, command, arg, env, target, home_dir)?;
        }
        Commands::McpUnset {
            server,
            env,
            clear_args,
            remove,
            target,
            home_dir,
        } => {
            mcp::mcp_unset(&server, env, clear_args, remove, target, home_dir)?;
        }
        Commands::McpEnable {
            server,
            target,
            home_dir,
        } => {
            mcp::mcp_toggle(&server, false, target, home_dir)?;
        }
        Commands::McpDisable {
            server,
            target,
            home_dir,
        } => {
            mcp::mcp_toggle(&server, true, target, home_dir)?;
        }
        Commands::MemoryIndex {
            home_dir,
            edge_threshold,
            source,
        } => {
            memory_engine::index_memory(home_dir, edge_threshold, source)?;
        }
        Commands::MemorySearch {
            query,
            mode,
            limit,
            min_score,
            home_dir,
        } => {
            memory_engine::search_memory(&query, &mode, home_dir, limit, min_score)?;
        }
        Commands::MemoryRelated { note, home_dir } => {
            memory_engine::get_related_notes(&note, home_dir)?;
        }
        Commands::MemoryNote {
            title,
            body,
            dir,
            home_dir,
        } => {
            memory_engine::add_memory_note(&title, body.as_deref(), dir, home_dir)?;
        }
        Commands::InstallHooks { repo_dir } => {
            security::install_git_hooks(repo_dir)?;
        }
        Commands::SecurityAudit { fix, home_dir } => {
            security::run_security_audit(fix, home_dir)?;
        }
        Commands::AgentWorkflow {
            branch_type,
            description,
            files,
        } => {
            agent::run_agent_workflow(&branch_type, &description, &files)?;
        }
        Commands::Status => {
            tester::run_tests(None)?;
        }
    }

    Ok(())
}
