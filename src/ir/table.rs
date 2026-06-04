//! Table Structures

use crate::ir::inline::InlineElement;
use crate::ir::style::StyleRef;
use serde::{Deserialize, Serialize};

/// Table cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableCell {
    pub content: Vec<InlineElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colspan: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rowspan: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<StyleRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<CellAlignment>,
}

/// Cell alignment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CellAlignment {
    Left,
    Center,
    Right,
}

/// Row type for header vs body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TableRowType {
    Header,
    Body,
}

/// Table row (re-exported from blocks.rs for convenience)
pub use crate::ir::blocks::TableRow;
