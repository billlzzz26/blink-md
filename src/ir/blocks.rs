//! Universal Block Types
//!
//! Canonical block types covering Notion API 2026-03-11, CommonMark, GFM, Lark, Google Docs block types

use crate::ir::{inline, style, table, Platform};
use serde::{Deserialize, Serialize};

/// List item with nested blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    pub content: Vec<UniversalBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<style::StyleRef>,
}

/// Task list item (checkbox)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskItem {
    pub content: Vec<UniversalBlock>,
    pub checked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<style::StyleRef>,
}

/// Table row
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableRow {
    pub cells: Vec<table::TableCell>,
    pub row_type: table::TableRowType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<style::StyleRef>,
}

/// All block types that exist across platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UniversalBlock {
    // ========== Text & Structure ==========
    /// Paragraph with rich text content
    Paragraph {
        content: Vec<inline::InlineElement>,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },
    /// Heading level 1-6
    Heading {
        level: u8,
        content: Vec<inline::InlineElement>,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },
    /// Fenced or indented code block
    CodeBlock {
        language: Option<String>,
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },
    /// Blockquote — can contain nested blocks
    Quote {
        content: Vec<UniversalBlock>,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },

    // ========== Lists ==========
    BulletList {
        items: Vec<ListItem>,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },
    OrderedList {
        items: Vec<ListItem>,
        start: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },
    TaskList {
        items: Vec<TaskItem>,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },

    // ========== Media ==========
    Image {
        src: MediaSource,
        #[serde(skip_serializing_if = "Option::is_none")]
        alt: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<Vec<inline::InlineElement>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },
    Video {
        src: MediaSource,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<Vec<inline::InlineElement>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },
    File {
        src: MediaSource,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },

    // ========== Tables ==========
    Table {
        rows: Vec<TableRow>,
        #[serde(skip_serializing_if = "Option::is_none")]
        header: Option<Vec<table::TableCell>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },

    // ========== Platform Extensions (Notion-specific, preserved losslessly) ==========
    Callout {
        icon: Option<String>,
        color: Option<String>,
        content: Vec<UniversalBlock>,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },
    Toggle {
        summary: Vec<inline::InlineElement>,
        content: Vec<UniversalBlock>,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },
    Columns {
        columns: Vec<Vec<UniversalBlock>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },
    PageBreak,
    TableOfContents {
        depth: u8,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },

    // ========== Embedded Content ==========
    Embed {
        url: String,
        provider: EmbedProvider,
        #[serde(skip_serializing_if = "Option::is_none")]
        fallback: Option<Vec<UniversalBlock>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },
    Mention {
        mention_type: MentionType,
        target: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<style::StyleRef>,
    },

    // ========== Raw Platform Data (never lose info) ==========
    /// Preserves platform-specific data that doesn't map to universal types
    Raw {
        platform: Platform,
        data: serde_json::Value,
    },
}

/// Media source variants
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MediaSource {
    /// External URL (Notion external, Lark file token, etc.)
    External { url: String },
    /// Uploaded file with expiry (Notion uploaded, Google Docs)
    Uploaded {
        url: String,
        expiry_time: Option<String>,
    },
    /// Base64 encoded data (for embedding)
    Base64 { data: String, mime_type: String },
}

/// Embed provider for oEmbed-style embeds
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmbedProvider {
    YouTube,
    Figma,
    Twitter,
    GitHub,
    Loom,
    Miro,
    Whimsical,
    Framer,
    Other(String),
}

/// Mention types across platforms
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MentionType {
    User,
    Page,
    Database,
    Date,
    DateRange,
    LinkPreview,
    UserGroup,
    Channel,     // Lark/Slack
    Document,    // Google Docs
    Issue,       // GitHub
    PullRequest, // GitHub
}
