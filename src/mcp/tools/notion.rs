//! Notion + Universal IR tools.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::converter::markdown::MarkdownConverter;
use crate::mcp::core::{
    invalid_args, McpError, McpResult, RequestHandlerExtra, SchemaBuilder, ToolHandler, ToolInfo,
};
use crate::{FromPlatform, ToPlatform};

// NOTE: fetching a live page + its blocks lives in `notion_live.rs`
// (`get_notion_page_blocks`) so it reuses the server-side `NotionClient` bound
// to `NOTION_TOKEN` instead of accepting a token through tool arguments.

#[derive(Debug, Deserialize)]
struct ContentInput {
    content: String,
}

#[derive(Debug, Deserialize)]
struct IrJsonInput {
    ir_json: Value,
}

/// Convert Markdown into a Universal IR document.
pub struct ConvertMdToIrTool;

#[async_trait]
impl ToolHandler for ConvertMdToIrTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: ContentInput =
            serde_json::from_value(args).map_err(|e| invalid_args("Invalid arguments", e))?;
        let doc = MarkdownConverter::from_platform(input.content)
            .map_err(|e| McpError::validation(format!("Conversion failed: {e}")))?;
        let document = serde_json::to_value(&doc)
            .map_err(|e| McpError::internal(format!("Serialization failed: {e}")))?;
        Ok(json!({ "document": document, "block_count": doc.blocks.len() }))
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

/// Convert a Universal IR document back into Markdown.
pub struct ConvertIrToMdTool;

#[async_trait]
impl ToolHandler for ConvertIrToMdTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: IrJsonInput =
            serde_json::from_value(args).map_err(|e| invalid_args("Invalid arguments", e))?;
        let doc: crate::UniversalDocument = serde_json::from_value(input.ir_json)
            .map_err(|e| invalid_args("Invalid IR JSON", e))?;
        let md = MarkdownConverter::to_platform(&doc)
            .map_err(|e| McpError::validation(format!("Conversion failed: {e}")))?;
        Ok(json!({ "markdown": md }))
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

/// List every conversion platform the engine understands.
pub struct ListPlatformsTool;

#[async_trait]
impl ToolHandler for ListPlatformsTool {
    async fn handle(&self, _args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        Ok(json!({
            "platforms": [
                "notion", "markdown", "sheets", "github", "lark",
                "google_docs", "pdf", "html", "docx"
            ],
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
