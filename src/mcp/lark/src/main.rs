//! Lark/Feishu Sheets MCP Server
//!
//! Provides tools for CSV/Sheets operations:
//! - csv_to_ir: Convert CSV to Universal IR
//! - ir_to_csv: Convert Universal IR to CSV
//! - list_lark_platforms: Show Lark/Feishu capabilities

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

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
struct CsvToIrInput {
    csv_data: String,
}

#[derive(Debug, Deserialize)]
struct IrToCsvInput {
    #[serde(rename = "ir_json")]
    ir_json: Value,
}

// =============================================================================
// CSV to IR Tool
// =============================================================================

/// Convert CSV to Universal IR
struct CsvToIrTool;

#[async_trait]
impl ToolHandler for CsvToIrTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: CsvToIrInput = serde_json::from_value(args)
            .map_err(|e| McpError::validation(format!("Invalid arguments: {}", e)))?;

        // Use LarkSheetAdapter through FromPlatform trait
        let doc = blink_md::converter::lark_sheets::LarkSheetAdapter::from_platform(input.csv_data)
            .map_err(|e| McpError::validation(format!("Conversion failed: {}", e)))?;

        Ok(json!({
            "document": serde_json::to_value(&doc).unwrap_or(json!({})),
            "block_count": doc.blocks.len()
        }))
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "csv_to_ir",
            Some("Convert CSV data to Universal IR".to_string()),
            SchemaBuilder::new()
                .param("csv_data", "CSV formatted string data")
                .build(),
        ))
    }
}

// =============================================================================
// IR to CSV Tool
// =============================================================================

/// Convert Universal IR to CSV
struct IrToCsvTool;

#[async_trait]
impl ToolHandler for IrToCsvTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: IrToCsvInput = serde_json::from_value(args)
            .map_err(|e| McpError::validation(format!("Invalid arguments: {}", e)))?;

        let doc: blink_md::UniversalDocument = serde_json::from_value(input.ir_json)
            .map_err(|e| McpError::validation(format!("Invalid IR JSON: {}", e)))?;

        let csv = blink_md::converter::lark_sheets::LarkSheetAdapter::to_platform(&doc)
            .map_err(|e| McpError::validation(format!("Conversion failed: {}", e)))?;

        Ok(json!({
            "csv": csv
        }))
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "ir_to_csv",
            Some("Convert Universal IR to CSV".to_string()),
            SchemaBuilder::new()
                .object_param("ir_json", "UniversalDocument JSON")
                .build(),
        ))
    }
}

// =============================================================================
// List Lark Platforms Tool
// =============================================================================

/// List supported platforms
struct ListPlatformsTool;

#[async_trait]
impl ToolHandler for ListPlatformsTool {
    async fn handle(&self, _args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        Ok(json!({
            "lark_sheets_support": true,
            "csv_import_export": true,
            "formats": ["csv", "json"]
        }))
    }

    fn metadata(&self) -> Option<ToolInfo> {
        Some(ToolInfo::new(
            "list_lark_platforms",
            Some("List Lark/Feishu Sheets capabilities".to_string()),
            SchemaBuilder::new().build(),
        ))
    }
}

// =============================================================================
// Main
// =============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--check") {
        println!("Checking dependencies...");
        println!("  OK: blink-md core library available");
        return Ok(());
    }

    init_logging();

    let server = Server::builder()
        .name("lark-mcp-server")
        .version("0.1.0")
        .capabilities(ServerCapabilities::tools_only())
        .tool("csv_to_ir", CsvToIrTool)
        .tool("ir_to_csv", IrToCsvTool)
        .tool("list_lark_platforms", ListPlatformsTool)
        .build()?;

    server.run_stdio().await?;
    Ok(())
}
