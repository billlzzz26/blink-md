//! Notion MCP Server
//!
//! Provides tools for Notion API operations and Universal IR conversion:
//! - get_notion_page_blocks: Fetch a page with its child blocks
//! - convert_md_to_ir: Convert Markdown to Universal IR
//! - convert_ir_to_md: Convert Universal IR to Markdown
//! - list_platforms: List supported conversion platforms

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use blink_md::client::NotionClient;
use blink_md::converter::markdown::MarkdownConverter;
use blink_md::{FromPlatform, ToPlatform};

use mcp_core::pmcp::types::ToolInfo;
use mcp_core::{
    init_logging, McpError, McpResult, RequestHandlerExtra, SchemaBuilder, Server,
    ServerCapabilities, ToolHandler,
};

// =============================================================================
// Input Types
// =============================================================================

#[derive(Debug, Deserialize)]
struct GetNotionPageBlocksInput {
    #[serde(rename = "notion_token")]
    token: String,
    #[serde(rename = "page_id")]
    page_id: String,
}

#[derive(Debug, Deserialize)]
struct ConvertMdToIrInput {
    content: String,
}

#[derive(Debug, Deserialize)]
struct ConvertIrToMdInput {
    #[serde(rename = "ir_json")]
    ir_json: Value,
}

// =============================================================================
// Get Notion Page Blocks Tool
// =============================================================================

/// Fetch a Notion page with its child blocks
struct GetNotionPageBlocksTool;

#[async_trait]
impl ToolHandler for GetNotionPageBlocksTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: GetNotionPageBlocksInput = serde_json::from_value(args)
            .map_err(|e| McpError::validation(format!("Invalid arguments: {}", e)))?;

        let client = NotionClient::new(&input.token);

        // Fetch page
        let page = client
            .get_page(&input.page_id)
            .await
            .map_err(|e| McpError::validation(format!("Failed to fetch page: {}", e)))?;

        // Fetch blocks
        let blocks_list = client
            .get_block_children(&input.page_id, None, None)
            .await
            .map_err(|e| McpError::validation(format!("Failed to fetch blocks: {}", e)))?;

        Ok(json!({
            "page": serde_json::to_value(&page).unwrap_or(json!({})),
            "blocks": serde_json::to_value(&blocks_list.results).unwrap_or(json!([])),
            "block_count": blocks_list.results.len()
        }))
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "get_notion_page_blocks",
            Some("Fetch a Notion page with its child blocks".to_string()),
            SchemaBuilder::new()
                .param("notion_token", "Notion integration token")
                .param("page_id", "Notion page ID")
                .build(),
        ))
    }
}

// =============================================================================
// Convert Markdown to IR Tool
// =============================================================================

/// Convert Markdown to Universal IR
struct ConvertMdToIrTool;

#[async_trait]
impl ToolHandler for ConvertMdToIrTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: ConvertMdToIrInput = serde_json::from_value(args)
            .map_err(|e| McpError::validation(format!("Invalid arguments: {}", e)))?;

        let doc = MarkdownConverter::from_platform(input.content)
            .map_err(|e| McpError::validation(format!("Conversion failed: {}", e)))?;

        Ok(json!({
            "document": serde_json::to_value(&doc).unwrap_or(json!({})),
            "block_count": doc.blocks.len()
        }))
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "convert_md_to_ir",
            Some("Convert Markdown to Universal IR".to_string()),
            SchemaBuilder::new()
                .param("content", "Markdown content")
                .build(),
        ))
    }
}

// =============================================================================
// Convert IR to Markdown Tool
// =============================================================================

/// Convert Universal IR to Markdown
struct ConvertIrToMdTool;

#[async_trait]
impl ToolHandler for ConvertIrToMdTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: ConvertIrToMdInput = serde_json::from_value(args)
            .map_err(|e| McpError::validation(format!("Invalid arguments: {}", e)))?;

        // Deserialize IR to UniversalDocument
        let doc: blink_md::UniversalDocument = serde_json::from_value(input.ir_json)
            .map_err(|e| McpError::validation(format!("Invalid IR JSON: {}", e)))?;

        let md = MarkdownConverter::to_platform(&doc)
            .map_err(|e| McpError::validation(format!("Conversion failed: {}", e)))?;

        Ok(json!({
            "markdown": md
        }))
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "convert_ir_to_md",
            Some("Convert Universal IR to Markdown".to_string()),
            SchemaBuilder::new()
                .object_param("ir_json", "UniversalDocument JSON")
                .build(),
        ))
    }
}

// =============================================================================
// List Platforms Tool
// =============================================================================

/// List supported conversion platforms
struct ListPlatformsTool;

fn get_supported_platforms() -> Vec<&'static str> {
    vec![
        "notion",
        "markdown",
        "sheets",
        "github",
        "lark",
        "google_docs",
        "pdf",
        "html",
        "docx",
    ]
}

#[async_trait]
impl ToolHandler for ListPlatformsTool {
    async fn handle(&self, _args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        Ok(json!({
            "platforms": get_supported_platforms(),
            "source_formats": ["markdown", "csv"],
            "target_formats": ["markdown", "notion", "json"]
        }))
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "list_platforms",
            Some("List supported conversion platforms".to_string()),
            SchemaBuilder::new().build(),
        ))
    }
}

// =============================================================================
// Main
// =============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Handle --check flag for dependency verification
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--check") {
        println!("Checking dependencies...");
        println!("  OK: blink-md core library available");
        return Ok(());
    }

    init_logging();

    let server = Server::builder()
        .name("notion-mcp-server")
        .version("0.1.0")
        .capabilities(ServerCapabilities::tools_only())
        .tool("get_notion_page_blocks", GetNotionPageBlocksTool)
        .tool("convert_md_to_ir", ConvertMdToIrTool)
        .tool("convert_ir_to_md", ConvertIrToMdTool)
        .tool("list_platforms", ListPlatformsTool)
        .build()?;

    server.run_stdio().await?;

    Ok(())
}
