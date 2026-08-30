mod agent;
mod branch_manager;
mod cli;
mod installer;
mod mcp;
mod memory_engine;
mod security;
mod tester;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install {
            skip_prereqs,
            home_dir,
        } => {
            installer::run_install(skip_prereqs, home_dir)?;
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
        Commands::InstallHooks { repo_dir } => {
            security::install_git_hooks(repo_dir)?;
        }
        Commands::SecurityAudit { home_dir } => {
            security::run_security_audit(home_dir)?;
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
