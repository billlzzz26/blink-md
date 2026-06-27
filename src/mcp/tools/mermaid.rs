//! Mermaid diagram tools backed by `mermaid-rs-renderer`.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::mcp::core::{
    invalid_args, McpError, McpResult, RequestHandlerExtra, SchemaBuilder, ToolHandler, ToolInfo,
};

#[derive(Debug, Deserialize)]
struct RenderMermaidInput {
    diagram: String,
}

fn render_svg(diagram: &str) -> McpResult<String> {
    mermaid_rs_renderer::render(diagram)
        .map_err(|e| McpError::validation(format!("Render failed: {e}")))
}

/// Render a Mermaid diagram to SVG.
pub struct RenderMermaidSvgTool;

#[async_trait]
impl ToolHandler for RenderMermaidSvgTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: RenderMermaidInput =
            serde_json::from_value(args).map_err(|e| invalid_args("Invalid arguments", e))?;
        let svg = render_svg(&input.diagram)?;
        Ok(json!({ "format": "svg", "content": svg }))
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

// NOTE: a `render_mermaid_png` tool was intentionally omitted. The underlying
// `mermaid-rs-renderer` only produces SVG here, so a PNG tool would have to
// return SVG under a false `format: "png"` contract. Re-add it once real PNG
// rendering is available.

/// List the Mermaid diagram types supported by the renderer.
pub struct ListDiagramTypesTool;

#[async_trait]
impl ToolHandler for ListDiagramTypesTool {
    async fn handle(&self, _args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        Ok(json!({
            "diagram_types": [
                "flowchart", "sequenceDiagram", "classDiagram", "stateDiagram-v2",
                "erDiagram", "gantt", "pie", "gitGraph", "journey", "timeline",
                "mindmap", "quadrantChart", "xychart"
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
