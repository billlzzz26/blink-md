//! Markdown tools: parse Notion-flavored Markdown and render blocks back out.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::api::markdown::{parse_markdown, ToMarkdown};
use crate::mcp::core::{
    invalid_args, McpResult, RequestHandlerExtra, SchemaBuilder, ToolHandler, ToolInfo,
};
use crate::models::block::Block;

#[derive(Debug, Deserialize)]
struct ParseMarkdownInput {
    content: String,
}

#[derive(Debug, Deserialize)]
struct ToMarkdownInput {
    block: Value,
}

/// Parse Notion-flavored Markdown into blocks.
pub struct ParseMarkdownTool;

#[async_trait]
impl ToolHandler for ParseMarkdownTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: ParseMarkdownInput =
            serde_json::from_value(args).map_err(|e| invalid_args("Invalid arguments", e))?;
        let blocks = parse_markdown(&input.content);
        Ok(json!({ "blocks": blocks, "count": blocks.len() }))
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

/// Convert a single block to Notion-flavored Markdown.
pub struct ToMarkdownTool;

#[async_trait]
impl ToolHandler for ToMarkdownTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: ToMarkdownInput =
            serde_json::from_value(args).map_err(|e| invalid_args("Invalid arguments", e))?;
        let block: Block =
            serde_json::from_value(input.block).map_err(|e| invalid_args("Invalid block", e))?;
        Ok(json!({ "markdown": block.to_markdown(0) }))
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
