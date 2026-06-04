//! Style System
//!
//! Named styles for consistent formatting across the document

use crate::ir::inline::TextStyle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Collection of named styles
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StyleSheet {
    pub styles: HashMap<String, Style>,
}

/// Reference to a named style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleRef {
    pub name: String,
}

impl StyleRef {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Named style definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub name: String,
    #[serde(flatten)]
    pub kind: StyleKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StyleKind {
    Text(TextStyle),
    Block(BlockStyle),
    Code(CodeStyle),
    Table(TableStyle),
}

/// Text formatting style (re-exported from inline for convenience)
pub use crate::ir::inline::TextStyle as StyleTextStyle;

/// Block-level style
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<BorderStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Spacing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<Spacing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<BlockAlignment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorderStyle {
    pub width: f32,
    pub color: String,
    pub radius: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockAlignment {
    Left,
    Center,
    Right,
    Justify,
}

/// Code block style
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_numbers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrap: Option<bool>,
}

/// Table style
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_row: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub striped: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bordered: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_widths: Option<Vec<u32>>,
}
