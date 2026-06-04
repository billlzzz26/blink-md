//! Universal Document Intermediate Representation (IR)
//!
//! Platform-agnostic document model for lossless conversion between
//! Notion, GitHub Markdown, Lark, Google Docs, PDF, HTML, Docx, and more.

pub mod blocks;
pub mod inline;
pub mod metadata;
pub mod style;
pub mod table;

pub use crate::converter::{ConverterError, ConverterRegistry, FromPlatform, ToPlatform};
pub use blocks::{EmbedProvider, ListItem, MediaSource, MentionType, TaskItem, UniversalBlock};
pub use inline::{
    equation, hard_break, labeled_mention, mention, soft_break, styled_text, text, InlineElement,
    TextStyle,
};
pub use metadata::DocumentMetadata;
pub use style::{BlockStyle, CodeStyle, Style, StyleRef, StyleSheet, StyleTextStyle, TableStyle};
pub use table::{TableCell, TableRow, TableRowType};

use serde::{Deserialize, Serialize};

/// The universal document — root of all conversions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalDocument {
    /// Document metadata (title, author, timestamps, properties)
    pub metadata: DocumentMetadata,
    /// Ordered sequence of blocks
    pub blocks: Vec<UniversalBlock>,
    /// Named styles referenced by blocks
    pub styles: StyleSheet,
}

/// Platform identifier for Raw block preservation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Notion,
    GitHub,
    Lark,
    GoogleDocs,
    Markdown,
    Pdf,
    Sheets,
    Html,
    Docx,
}
