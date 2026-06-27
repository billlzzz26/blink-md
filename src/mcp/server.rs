//! The unified blink-md MCP server.
//!
//! A single server exposes every tool that used to be split across the
//! `notion`, `md`, `mmd` and `lark` MCP server binaries. The CLI entry point
//! lives in `src/bin/blink-md-mcp.rs`.

use std::sync::Arc;

use crate::client::NotionClient;
use crate::mcp::core::{init_logging, Server, ServerCapabilities};
use crate::mcp::tools;

/// Build the unified MCP server with all blink-md tools registered.
///
/// Stateless conversion/rendering tools are always available. The live Notion
/// API tools (search, get/create page, list children, trash) are registered
/// only when `NOTION_TOKEN` is set, since they need an authenticated client.
pub fn build() -> Result<Server, Box<dyn std::error::Error>> {
    let mut builder = Server::builder()
        .name("blink-md-mcp")
        .version(env!("CARGO_PKG_VERSION"))
        .capabilities(ServerCapabilities::tools_only())
        // Markdown
        .tool("parse_markdown", tools::markdown::ParseMarkdownTool)
        .tool("to_markdown", tools::markdown::ToMarkdownTool)
        // Notion + Universal IR (stateless conversion)
        .tool("convert_md_to_ir", tools::notion::ConvertMdToIrTool)
        .tool("convert_ir_to_md", tools::notion::ConvertIrToMdTool)
        .tool("list_platforms", tools::notion::ListPlatformsTool)
        // Lark / Feishu Sheets
        .tool("csv_to_ir", tools::lark::CsvToIrTool)
        .tool("ir_to_csv", tools::lark::IrToCsvTool)
        .tool("list_lark_platforms", tools::lark::ListLarkPlatformsTool)
        // Mermaid
        .tool("render_mermaid_svg", tools::mermaid::RenderMermaidSvgTool)
        .tool("list_diagram_types", tools::mermaid::ListDiagramTypesTool);

    if let Ok(token) = std::env::var("NOTION_TOKEN") {
        use tools::notion_live::{
            CreatePageTool, GetBlocksTool, GetPageBlocksTool, GetPageTool, SearchTool, TrashTool,
        };
        let client = Arc::new(NotionClient::new(token));
        builder = builder
            .tool(
                "get_notion_page_blocks",
                GetPageBlocksTool {
                    client: client.clone(),
                },
            )
            .tool(
                "notion_search",
                SearchTool {
                    client: client.clone(),
                },
            )
            .tool(
                "notion_get_page",
                GetPageTool {
                    client: client.clone(),
                },
            )
            .tool(
                "notion_create_page",
                CreatePageTool {
                    client: client.clone(),
                },
            )
            .tool(
                "notion_get_block_children",
                GetBlocksTool {
                    client: client.clone(),
                },
            )
            .tool("notion_trash", TrashTool { client });
    }

    Ok(builder.build()?)
}

/// Build and run the server over stdio until the client disconnects.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();
    build()?.run_stdio().await?;
    Ok(())
}
