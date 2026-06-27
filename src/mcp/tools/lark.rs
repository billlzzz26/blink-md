//! Lark/Feishu Sheets tools: CSV <-> Universal IR.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::converter::lark_sheets::LarkSheetAdapter;
use crate::mcp::core::{
    invalid_args, McpError, McpResult, RequestHandlerExtra, SchemaBuilder, ToolHandler, ToolInfo,
};
use crate::{FromPlatform, ToPlatform};

#[derive(Debug, Deserialize)]
struct CsvToIrInput {
    csv_data: String,
}

#[derive(Debug, Deserialize)]
struct IrJsonInput {
    ir_json: Value,
}

/// Convert CSV data into a Universal IR document.
pub struct CsvToIrTool;

#[async_trait]
impl ToolHandler for CsvToIrTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: CsvToIrInput =
            serde_json::from_value(args).map_err(|e| invalid_args("Invalid arguments", e))?;
        let doc = LarkSheetAdapter::from_platform(input.csv_data)
            .map_err(|e| McpError::validation(format!("Conversion failed: {e}")))?;
        Ok(json!({
            "document": serde_json::to_value(&doc).unwrap_or_else(|_| json!({})),
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

/// Convert a Universal IR document into CSV.
pub struct IrToCsvTool;

#[async_trait]
impl ToolHandler for IrToCsvTool {
    async fn handle(&self, args: Value, _extra: RequestHandlerExtra) -> McpResult<Value> {
        let input: IrJsonInput =
            serde_json::from_value(args).map_err(|e| invalid_args("Invalid arguments", e))?;
        let doc: crate::UniversalDocument = serde_json::from_value(input.ir_json)
            .map_err(|e| invalid_args("Invalid IR JSON", e))?;
        let csv = LarkSheetAdapter::to_platform(&doc)
            .map_err(|e| McpError::validation(format!("Conversion failed: {e}")))?;
        Ok(json!({ "csv": csv }))
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

/// Report Lark/Feishu Sheets capabilities.
pub struct ListLarkPlatformsTool;

#[async_trait]
impl ToolHandler for ListLarkPlatformsTool {
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
