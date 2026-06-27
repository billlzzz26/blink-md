//! Live Notion API tools (require a `NotionClient`).
//!
//! These were previously a standalone `notion-universal-mcp` server wired into
//! the `blink-md mcp-serve` CLI command. They now live alongside the other
//! tools so the unified `blink-md-mcp` server exposes a single, complete tool
//! surface. They are only registered when a `NotionClient` is available
//! (i.e. `NOTION_TOKEN` is set).

use std::sync::Arc;

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::api::trash::Resource;
use crate::client::NotionClient;
use crate::mcp::core::{
    invalid_args, McpError, McpResult, RequestHandlerExtra, SchemaBuilder, ToolHandler, ToolInfo,
};

fn internal(e: impl std::fmt::Display) -> McpError {
    McpError::internal(e.to_string())
}

/// Search Notion pages and databases.
pub struct SearchTool {
    pub client: Arc<NotionClient>,
}

#[derive(Deserialize)]
struct SearchInput {
    query: Option<String>,
}

#[async_trait]
impl ToolHandler for SearchTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: SearchInput =
            serde_json::from_value(args).map_err(|e| invalid_args("Invalid args", e))?;
        let results = self
            .client
            .search(input.query, None, None, None, None)
            .await
            .map_err(internal)?;
        serde_json::to_value(results).map_err(internal)
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "notion_search",
            Some("Search for pages and databases in Notion".to_string()),
            SchemaBuilder::new()
                .optional_param("query", "Text query to search for")
                .build(),
        ))
    }
}

/// Retrieve a Notion page by ID.
pub struct GetPageTool {
    pub client: Arc<NotionClient>,
}

#[derive(Deserialize)]
struct GetPageInput {
    page_id: String,
}

#[async_trait]
impl ToolHandler for GetPageTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: GetPageInput =
            serde_json::from_value(args).map_err(|e| invalid_args("Invalid args", e))?;
        let page = self
            .client
            .get_page(&input.page_id)
            .await
            .map_err(internal)?;
        serde_json::to_value(page).map_err(internal)
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "notion_get_page",
            Some("Retrieve a specific page by ID".to_string()),
            SchemaBuilder::new()
                .param("page_id", "The ID of the page to retrieve")
                .build(),
        ))
    }
}

/// Create a Notion page under a page or database parent.
pub struct CreatePageTool {
    pub client: Arc<NotionClient>,
}

#[derive(Deserialize)]
struct CreatePageInput {
    parent_id: String,
    parent_type: String, // "page_id" or "database_id"
    properties: Value,
    children: Option<Value>,
}

#[async_trait]
impl ToolHandler for CreatePageTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: CreatePageInput =
            serde_json::from_value(args).map_err(|e| invalid_args("Invalid args", e))?;
        let parent = json!({ input.parent_type: input.parent_id });
        let children = input.children.and_then(|v| serde_json::from_value(v).ok());
        let page = self
            .client
            .create_page(parent, input.properties, children)
            .await
            .map_err(internal)?;
        serde_json::to_value(page).map_err(internal)
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
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

/// List the child blocks of a page or block.
pub struct GetBlocksTool {
    pub client: Arc<NotionClient>,
}

#[derive(Deserialize)]
struct GetBlocksInput {
    block_id: String,
}

#[async_trait]
impl ToolHandler for GetBlocksTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: GetBlocksInput =
            serde_json::from_value(args).map_err(|e| invalid_args("Invalid args", e))?;
        let list = self
            .client
            .get_block_children(&input.block_id, None, None)
            .await
            .map_err(internal)?;
        serde_json::to_value(list).map_err(internal)
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "notion_get_block_children",
            Some("List children blocks for a page or block".to_string()),
            SchemaBuilder::new()
                .param("block_id", "The ID of the block/page to list children for")
                .build(),
        ))
    }
}

/// Move a Notion resource to the trash (or permanently delete it) through the
/// unified trash lifecycle.
pub struct TrashTool {
    pub client: Arc<NotionClient>,
}

#[derive(Deserialize)]
struct TrashInput {
    resource: String, // "page" | "block" | "view" | "webhook"
    id: String,
    #[serde(default)]
    restore: bool,
}

fn parse_resource(name: &str) -> McpResult<Resource> {
    match name.to_ascii_lowercase().as_str() {
        "page" => Ok(Resource::Page),
        "block" => Ok(Resource::Block),
        "view" => Ok(Resource::View),
        "webhook" => Ok(Resource::Webhook),
        other => Err(McpError::validation(format!(
            "Unknown resource '{other}' (expected page|block|view|webhook)"
        ))),
    }
}

#[async_trait]
impl ToolHandler for TrashTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: TrashInput =
            serde_json::from_value(args).map_err(|e| invalid_args("Invalid args", e))?;
        let resource = parse_resource(&input.resource)?;
        let result = if input.restore {
            self.client.restore(resource, &input.id).await
        } else {
            self.client.trash(resource, &input.id).await
        };
        result.map_err(internal)
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "notion_trash",
            Some(
                "Trash, restore, or permanently delete a Notion resource \
                 (page/block/view/webhook) via the unified trash lifecycle"
                    .to_string(),
            ),
            SchemaBuilder::new()
                .param("resource", "Resource type: page, block, view, or webhook")
                .param("id", "The resource ID")
                .optional_param("restore", "Set to true to restore instead of trash")
                .build(),
        ))
    }
}
