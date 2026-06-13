use anyhow::Result;
use async_trait::async_trait;
use blink_md::NotionClient;
use mcp_core::{
    init_logging, McpError, McpResult, RequestHandlerExtra, SchemaBuilder, Server,
    ServerCapabilities, ToolHandler,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

pub async fn run_mcp_server() -> Result<()> {
    let token = std::env::var("NOTION_TOKEN").map_err(|_| {
        anyhow::anyhow!("NOTION_TOKEN environment variable not set. Required for MCP server.")
    })?;

    init_logging();

    let client = Arc::new(NotionClient::new(token));

    let server = Server::builder()
        .name("notion-universal-mcp")
        .version("0.3.0")
        .capabilities(ServerCapabilities::tools_only())
        .tool("notion_search", SearchTool { client: client.clone() })
        .tool("notion_get_page", GetPageTool { client: client.clone() })
        .tool("notion_create_page", CreatePageTool { client: client.clone() })
        .tool("notion_get_block_children", GetBlocksTool { client: client.clone() })
        .build()?;

    println!("Notion MCP Server starting...");
    server.run_stdio().await?;

    Ok(())
}

// ─── Tools ──────────────────────────────────────────────────────────────────

struct SearchTool {
    client: Arc<NotionClient>,
}

#[derive(Deserialize)]
struct SearchInput {
    query: Option<String>,
}

#[async_trait]
impl ToolHandler for SearchTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: SearchInput = serde_json::from_value(args)
            .map_err(|e| McpError::validation(format!("Invalid args: {}", e)))?;
        
        let results = self.client.search(input.query, None, None, None, None).await
            .map_err(|e| McpError::internal(e.to_string()))?;
        
        Ok(serde_json::to_value(results).unwrap())
    }

    fn metadata(&self) -> Option<mcp_core::pmcp::types::ToolInfo> {
        Some(mcp_core::pmcp::types::ToolInfo::new(
            "notion_search",
            Some("Search for pages and databases in Notion".to_string()),
            SchemaBuilder::new()
                .optional_param("query", "Text query to search for")
                .build(),
        ))
    }
}

struct GetPageTool {
    client: Arc<NotionClient>,
}

#[derive(Deserialize)]
struct GetPageInput {
    page_id: String,
}

#[async_trait]
impl ToolHandler for GetPageTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: GetPageInput = serde_json::from_value(args)
            .map_err(|e| McpError::validation(format!("Invalid args: {}", e)))?;
        
        let page = self.client.get_page(&input.page_id).await
            .map_err(|e| McpError::internal(e.to_string()))?;
        
        Ok(serde_json::to_value(page).unwrap())
    }

    fn metadata(&self) -> Option<mcp_core::pmcp::types::ToolInfo> {
        Some(mcp_core::pmcp::types::ToolInfo::new(
            "notion_get_page",
            Some("Retrieve a specific page by ID".to_string()),
            SchemaBuilder::new()
                .param("page_id", "The ID of the page to retrieve")
                .build(),
        ))
    }
}

struct CreatePageTool {
    client: Arc<NotionClient>,
}

#[derive(Deserialize)]
struct CreatePageInput {
    parent_id: String,
    parent_type: String, // "page_id" or "database_id"
    properties: Value,
    children: Option<Value>, // Array of blocks
}

#[async_trait]
impl ToolHandler for CreatePageTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: CreatePageInput = serde_json::from_value(args)
            .map_err(|e| McpError::validation(format!("Invalid args: {}", e)))?;
        
        let parent = json!({ input.parent_type: input.parent_id });
        let children = input.children.and_then(|v| serde_json::from_value(v).ok());
        
        let page = self.client.create_page(parent, input.properties, children).await
            .map_err(|e| McpError::internal(e.to_string()))?;
        
        Ok(serde_json::to_value(page).unwrap())
    }

    fn metadata(&self) -> Option<mcp_core::pmcp::types::ToolInfo> {
        Some(mcp_core::pmcp::types::ToolInfo::new(
            "notion_create_page",
            Some("Create a new page in a database or as a child of another page".to_string()),
            SchemaBuilder::new()
                .param("parent_id", "ID of the parent container")
                .param("parent_type", "Type of parent ('page_id' or 'database_id')")
                .object_param("properties", "Notion page properties JSON")
                .build(),
        ))
    }
}

struct GetBlocksTool {
    client: Arc<NotionClient>,
}

#[derive(Deserialize)]
struct GetBlocksInput {
    block_id: String,
}

#[async_trait]
impl ToolHandler for GetBlocksTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: GetBlocksInput = serde_json::from_value(args)
            .map_err(|e| McpError::validation(format!("Invalid args: {}", e)))?;
        
        let list = self.client.get_block_children(&input.block_id, None, None).await
            .map_err(|e| McpError::internal(e.to_string()))?;
        
        Ok(serde_json::to_value(list).unwrap())
    }

    fn metadata(&self) -> Option<mcp_core::pmcp::types::ToolInfo> {
        Some(mcp_core::pmcp::types::ToolInfo::new(
            "notion_get_block_children",
            Some("List children blocks for a page or block".to_string()),
            SchemaBuilder::new()
                .param("block_id", "The ID of the block/page to list children for")
                .build(),
        ))
    }
}
