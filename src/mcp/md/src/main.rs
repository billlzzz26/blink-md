//! Markdown MCP Server
//!
//! Provides tools for parsing and generating Notion-flavored Markdown:
//! - parse_markdown: Parse Markdown into blocks
//! - to_markdown: Convert blocks to Markdown

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use blink_md::api::markdown::{parse_markdown, ToMarkdown};
use blink_md::models::block::Block;

use mcp_core::pmcp::types::ToolInfo;
use mcp_core::{
    init_logging, McpError, McpResult, RequestHandlerExtra, SchemaBuilder, Server,
    ServerCapabilities, ToolHandler,
};

// =============================================================================
// Input Types
// =============================================================================

#[derive(Debug, Deserialize)]
struct ParseMarkdownInput {
    content: String,
}

#[derive(Debug, Deserialize)]
struct ToMarkdownInput {
    block: Value,
}

// =============================================================================
// Tool Implementations
// =============================================================================

/// Parse Notion-flavored Markdown into blocks
struct ParseMarkdownTool;

#[async_trait]
impl ToolHandler for ParseMarkdownTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: ParseMarkdownInput = serde_json::from_value(args)
            .map_err(|e| McpError::validation(format!("Invalid arguments: {}", e)))?;
        let blocks = parse_markdown(&input.content);
        Ok(json!({
            "blocks": blocks,
            "count": blocks.len()
        }))
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "parse_markdown",
            Some("Parse Notion-flavored Markdown into blocks".to_string()),
            SchemaBuilder::new()
                .param("content", "Markdown content to parse")
                .build(),
        ))
    }
}

/// Convert a block to Notion-flavored Markdown
struct ToMarkdownTool;

#[async_trait]
impl ToolHandler for ToMarkdownTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: ToMarkdownInput = serde_json::from_value(args)
            .map_err(|e| McpError::validation(format!("Invalid arguments: {}", e)))?;
        let block: Block = serde_json::from_value(input.block)
            .map_err(|e| McpError::validation(format!("Invalid block: {}", e)))?;
        let md = block.to_markdown(0);
        Ok(json!({ "markdown": md }))
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "to_markdown",
            Some("Convert a block to Notion-flavored Markdown".to_string()),
            SchemaBuilder::new()
                .object_param("block", "Block JSON object")
                .build(),
        ))
    }
}

// =============================================================================
// Main
// =============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let server = Server::builder()
        .name("md-mcp-server")
        .version("0.1.0")
        .capabilities(ServerCapabilities::tools_only())
        .tool("parse_markdown", ParseMarkdownTool)
        .tool("to_markdown", ToMarkdownTool)
        .build()?;

    server.run_stdio().await?;

    Ok(())
}
