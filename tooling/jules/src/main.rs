//! Jules MCP Server
//!
//! Bridge between Jules AI agent and Hermes, providing tools for:
//! - Starting and managing Jules tasks
//! - Listing Jules sessions and repos
//! - Pulling changes from Jules
//! - Querying Hermes agent

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use mcp_core::pmcp::types::ToolInfo;
use mcp_core::{
    init_logging, is_command_installed, run_cli_command, McpError, McpResult, RequestHandlerExtra,
    SchemaBuilder, Server, ServerCapabilities, ToolHandler,
};

// =============================================================================
// Input Types
// =============================================================================

#[derive(Debug, Deserialize)]
struct StartNewJulesTaskInput {
    repo_name: String,
    user_task_description: String,
}

#[derive(Debug, Deserialize)]
struct PullJulesChangesInput {
    session_id: String,
}

#[derive(Debug, Deserialize)]
struct HermesQueryInput {
    prompt: String,
    #[serde(default)]
    yolo: bool,
}

// =============================================================================
// Tool Implementations
// =============================================================================

/// Start a new Jules task
struct StartNewJulesTaskTool;

#[async_trait]
impl ToolHandler for StartNewJulesTaskTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: StartNewJulesTaskInput = serde_json::from_value(args)
            .map_err(|e| McpError::validation(format!("Invalid arguments: {}", e)))?;
        run_cli_command(
            "jules",
            &[
                "remote",
                "new",
                "--repo",
                &input.repo_name,
                "--session",
                &input.user_task_description,
            ],
        )
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "start_new_jules_task",
            Some("Start a new Jules task".to_string()),
            SchemaBuilder::new()
                .param("repo_name", "Repository name (owner/repo)")
                .param("user_task_description", "Task description")
                .build(),
        ))
    }
}

/// List Jules sessions
struct ListJulesSessionsTool;

#[async_trait]
impl ToolHandler for ListJulesSessionsTool {
    async fn handle(&self, _args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        run_cli_command("jules", &["remote", "list", "--session"])
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "list_jules_sessions",
            Some("List all Jules sessions".to_string()),
            SchemaBuilder::new().build(),
        ))
    }
}

/// List Jules repos
struct ListJulesReposTool;

#[async_trait]
impl ToolHandler for ListJulesReposTool {
    async fn handle(&self, _args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        run_cli_command("jules", &["remote", "list", "--repo"])
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "list_jules_repos",
            Some("List all Jules repos".to_string()),
            SchemaBuilder::new().build(),
        ))
    }
}

/// Pull changes from Jules
struct PullJulesChangesTool;

#[async_trait]
impl ToolHandler for PullJulesChangesTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: PullJulesChangesInput = serde_json::from_value(args)
            .map_err(|e| McpError::validation(format!("Invalid arguments: {}", e)))?;
        run_cli_command("jules", &["remote", "pull", "--session", &input.session_id])
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "pull_jules_changes",
            Some("Pull changes from a Jules session".to_string()),
            SchemaBuilder::new()
                .param("session_id", "Jules session ID")
                .build(),
        ))
    }
}

/// Query Hermes agent
struct HermesQueryTool;

#[async_trait]
impl ToolHandler for HermesQueryTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: HermesQueryInput = serde_json::from_value(args)
            .map_err(|e| McpError::validation(format!("Invalid arguments: {}", e)))?;
        let mut cmd_args = vec!["chat", "-q", &input.prompt];
        if input.yolo {
            cmd_args.push("--yolo");
        }
        run_cli_command("hermes", &cmd_args)
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "hermes_query",
            Some("Send a query to Hermes agent".to_string()),
            SchemaBuilder::new()
                .param("prompt", "The prompt to send")
                .optional_param("yolo", "Enable YOLO mode (auto-approve)")
                .build(),
        ))
    }
}

/// List Hermes skills
struct HermesListSkillsTool;

#[async_trait]
impl ToolHandler for HermesListSkillsTool {
    async fn handle(&self, _args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        run_cli_command("hermes", &["skills", "list"])
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "hermes_list_skills",
            Some("List all Hermes skills".to_string()),
            SchemaBuilder::new().build(),
        ))
    }
}

/// Get Hermes cron status
struct HermesCronStatusTool;

#[async_trait]
impl ToolHandler for HermesCronStatusTool {
    async fn handle(&self, _args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        run_cli_command("hermes", &["cron", "list"])
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "hermes_cron_status",
            Some("Get Hermes cron job status".to_string()),
            SchemaBuilder::new().build(),
        ))
    }
}

/// Check dependencies
struct CheckDependenciesTool;

#[async_trait]
impl ToolHandler for CheckDependenciesTool {
    async fn handle(&self, _args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let mut missing = Vec::new();
        if !is_command_installed("jules") {
            missing.push("jules");
        }
        if !is_command_installed("hermes") {
            missing.push("hermes");
        }
        if missing.is_empty() {
            Ok(json!({
                "status": "ok",
                "message": "All dependencies found"
            }))
        } else {
            Err(McpError::validation(format!(
                "Missing dependencies: {}",
                missing.join(", ")
            )))
        }
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "check_dependencies",
            Some("Check if required CLI tools are installed".to_string()),
            SchemaBuilder::new().build(),
        ))
    }
}

// =============================================================================
// Main
// =============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Handle --setup/--check flag
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--setup" || arg == "--check") {
        println!("Checking dependencies...");
        if !is_command_installed("jules") {
            println!("  WARNING: Jules CLI not found");
        } else {
            println!("  OK: Jules CLI found");
        }
        if !is_command_installed("hermes") {
            println!("  WARNING: Hermes Agent not found");
        } else {
            println!("  OK: Hermes Agent found");
        }
        return Ok(());
    }

    init_logging();

    let server = Server::builder()
        .name("jules-mcp-server")
        .version("0.2.0")
        .capabilities(ServerCapabilities::tools_only())
        .tool("start_new_jules_task", StartNewJulesTaskTool)
        .tool("list_jules_sessions", ListJulesSessionsTool)
        .tool("list_jules_repos", ListJulesReposTool)
        .tool("pull_jules_changes", PullJulesChangesTool)
        .tool("hermes_query", HermesQueryTool)
        .tool("hermes_list_skills", HermesListSkillsTool)
        .tool("hermes_cron_status", HermesCronStatusTool)
        .tool("check_dependencies", CheckDependenciesTool)
        .build()?;

    server.run_stdio().await?;

    Ok(())
}
