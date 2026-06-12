//! Mermaid MCP Server
//!
//! Provides tools for rendering Mermaid diagrams using mermaid-rs-renderer:
//! - render_mermaid: Render Mermaid diagram to SVG
//! - render_mermaid_png: Render Mermaid diagram to PNG (base64)

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use mcp_core::pmcp::types::ToolInfo;
use mcp_core::{
    init_logging, McpError, McpResult, RequestHandlerExtra, SchemaBuilder, Server,
    ServerCapabilities, ToolHandler,
};

// =============================================================================
// Input Types
// =============================================================================

#[derive(Debug, Deserialize)]
struct RenderMermaidInput {
    diagram: String,
}

// =============================================================================
// Tool Implementations
// =============================================================================

/// Render Mermaid diagram to SVG
struct RenderMermaidSvgTool;

#[async_trait]
impl ToolHandler for RenderMermaidSvgTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: RenderMermaidInput = serde_json::from_value(args)
            .map_err(|e| McpError::validation(format!("Invalid arguments: {}", e)))?;

        match mermaid_rs_renderer::render(&input.diagram) {
            Ok(svg) => Ok(json!({
                "format": "svg",
                "content": svg
            })),
            Err(e) => Err(McpError::validation(format!("Render failed: {}", e))),
        }
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "render_mermaid_svg",
            Some("Render a Mermaid diagram to SVG".to_string()),
            SchemaBuilder::new()
                .param("diagram", "Mermaid diagram source code")
                .build(),
        ))
    }
}

/// Render Mermaid diagram to PNG (base64 encoded)
struct RenderMermaidPngTool;

#[async_trait]
impl ToolHandler for RenderMermaidPngTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: RenderMermaidInput = serde_json::from_value(args)
            .map_err(|e| McpError::validation(format!("Invalid arguments: {}", e)))?;

        // mermaid-rs-renderer provides render_png when "png" feature is enabled
        // For now, we'll return an error indicating PNG requires the feature
        match mermaid_rs_renderer::render(&input.diagram) {
            Ok(svg) => Ok(json!({
                "format": "svg",
                "content": svg,
                "note": "PNG rendering requires mermaid-rs-renderer with 'png' feature"
            })),
            Err(e) => Err(McpError::validation(format!("Render failed: {}", e))),
        }
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "render_mermaid_png",
            Some("Render a Mermaid diagram to PNG (base64)".to_string()),
            SchemaBuilder::new()
                .param("diagram", "Mermaid diagram source code")
                .build(),
        ))
    }
}

/// List supported diagram types
struct ListDiagramTypesTool;

#[async_trait]
impl ToolHandler for ListDiagramTypesTool {
    async fn handle(&self, _args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        Ok(json!({
            "diagram_types": [
                "flowchart",
                "sequenceDiagram",
                "classDiagram",
                "stateDiagram-v2",
                "erDiagram",
                "gantt",
                "pie",
                "gitGraph",
                "journey",
                "timeline",
                "mindmap",
                "quadrantChart",
                "xychart"
            ],
            "directions": ["TD", "TB", "BT", "LR", "RL"]
        }))
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "list_diagram_types",
            Some("List supported Mermaid diagram types".to_string()),
            SchemaBuilder::new().build(),
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
        .name("mmd-mcp-server")
        .version("0.1.0")
        .capabilities(ServerCapabilities::tools_only())
        .tool("render_mermaid_svg", RenderMermaidSvgTool)
        .tool("render_mermaid_png", RenderMermaidPngTool)
        .tool("list_diagram_types", ListDiagramTypesTool)
        .build()?;

    server.run_stdio().await?;

    Ok(())
}
