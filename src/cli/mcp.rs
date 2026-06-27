//! `blink-md mcp-serve` — thin wrapper around the unified MCP server.
//!
//! The actual tool definitions live in [`blink_md::mcp`]; this entry point and
//! the standalone `blink-md-mcp` binary both run the same server, so there is a
//! single source of truth for the tool surface.

use anyhow::Result;

pub async fn run_mcp_server() -> Result<()> {
    blink_md::mcp::server::run()
        .await
        .map_err(|e| anyhow::anyhow!(e.to_string()))
}
