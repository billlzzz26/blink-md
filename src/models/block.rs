//! Block types and content structures for Notion pages.

use super::common::{FileBlockContent, Icon, ObjectId, RichText, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A Notion block — the fundamental building unit of a page.
///
/// Every block has base metadata (id, timestamps, author, parent) plus
/// a `block_type` field that determines its content structure.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    /// Always `"block"`.
    pub object: String,
    /// Unique identifier.
    pub id: ObjectId,
    /// When the block was created.
    pub created_time: DateTime<Utc>,
    /// When the block was last edited.
    pub last_edited_time: DateTime<Utc>,
    /// The user who created the block.
    pub created_by: User,
    /// The user who last edited the block.
    pub last_edited_by: User,
    /// Whether this block has child blocks.
    pub has_children: bool,
    /// Whether the block is in the trash (from `archived` field).
    #[serde(alias = "archived", default)]
    pub in_trash: bool,
    /// Parent container (page, block, or database).
    pub parent: Option<super::common::Parent>,

    /// The block's type-specific content.
    #[serde(flatten)]
    pub block_type: BlockType,
}

/// The type discriminator for [`Block`].
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum BlockType {
    /// A paragraph with rich text and optional children.
    #[serde(rename = "paragraph")]
    Paragraph { paragraph: TextBlockContent },
    /// Level-1 heading.
    #[serde(rename = "heading_1")]
    Heading1 { heading_1: HeadingContent },
    /// Level-2 heading.
    #[serde(rename = "heading_2")]
    Heading2 { heading_2: HeadingContent },
    /// Level-3 heading.
    #[serde(rename = "heading_3")]
    Heading3 { heading_3: HeadingContent },
    /// Bulleted list item.
    #[serde(rename = "bulleted_list_item")]
    BulletedListItem {
        bulleted_list_item: TextBlockContent,
    },
    /// Numbered list item.
    #[serde(rename = "numbered_list_item")]
    NumberedListItem {
        numbered_list_item: TextBlockContent,
    },
    /// A to-do / checkbox block.
    #[serde(rename = "to_do")]
    ToDo { to_do: ToDoContent },
    /// A toggle block that can expand to show children.
    #[serde(rename = "toggle")]
    Toggle { toggle: TextBlockContent },
    /// A child page created inside a parent page.
    #[serde(rename = "child_page")]
    ChildPage { child_page: ChildPageContent },
    /// A child database created inside a page.
    #[serde(rename = "child_database")]
    ChildDatabase {
        child_database: ChildDatabaseContent,
    },
    /// An embedded URL (YouTube, Figma, etc.).
    #[serde(rename = "embed")]
    Embed { embed: EmbedContent },
    /// An image file block.
    #[serde(rename = "image")]
    Image { image: FileBlockContent },
    /// A video file block.
    #[serde(rename = "video")]
    Video { video: FileBlockContent },
    /// A file attachment block.
    #[serde(rename = "file")]
    File { file: FileBlockContent },
    /// A PDF file block.
    #[serde(rename = "pdf")]
    Pdf { pdf: FileBlockContent },
    /// A bookmark to an external URL.
    #[serde(rename = "bookmark")]
    Bookmark { bookmark: BookmarkContent },
    /// A callout block with icon, rich text, and optional children.
    #[serde(rename = "callout")]
    Callout { callout: CalloutContent },
    /// A quote block.
    #[serde(rename = "quote")]
    Quote { quote: TextBlockContent },
    /// An inline LaTeX equation.
    #[serde(rename = "equation")]
    Equation { equation: EquationContent },
    /// A horizontal divider line.
    #[serde(rename = "divider")]
    Divider {},
    /// A table of contents block.
    #[serde(rename = "table_of_contents")]
    TableOfContents {},
    /// Breadcrumbs navigation block.
    #[serde(rename = "breadcrumb")]
    Breadcrumb {},
    /// A list of columns.
    #[serde(rename = "column_list")]
    ColumnList {},
    /// A single column within a column list.
    #[serde(rename = "column")]
    Column {},
    /// A link preview block.
    #[serde(rename = "link_preview")]
    LinkPreview { link_preview: LinkPreviewContent },
    /// A template block for reusable content.
    #[serde(rename = "template")]
    Template { template: TemplateContent },
    /// A synced block that mirrors another block.
    #[serde(rename = "synced_block")]
    SyncedBlock { synced_block: SyncedBlockContent },
    /// A table block.
    #[serde(rename = "table")]
    Table { table: TableContent },
    /// A row within a table.
    #[serde(rename = "table_row")]
    TableRow { table_row: TableRowContent },
    /// Meeting notes (renamed from "transcription").
    #[serde(rename = "meeting_notes", alias = "transcription")]
    MeetingNotes {
        #[serde(alias = "transcription")]
        meeting_notes: MeetingNotesContent,
    },
    /// Any unrecognized block type.
    #[serde(other)]
    Unknown,
}

// ── Content structs ───────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextBlockContent {
    pub rich_text: Vec<RichText>,
    pub color: String,
    pub children: Option<Vec<Block>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HeadingContent {
    pub rich_text: Vec<RichText>,
    pub color: String,
    pub is_toggleable: bool,
    pub children: Option<Vec<Block>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToDoContent {
    pub rich_text: Vec<RichText>,
    pub checked: bool,
    pub color: String,
    pub children: Option<Vec<Block>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChildPageContent {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChildDatabaseContent {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbedContent {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookmarkContent {
    pub url: String,
    pub caption: Vec<RichText>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CalloutContent {
    pub rich_text: Vec<RichText>,
    pub icon: Option<Icon>,
    pub color: String,
    pub children: Option<Vec<Block>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EquationContent {
    pub expression: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinkPreviewContent {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateContent {
    pub rich_text: Vec<RichText>,
    pub children: Option<Vec<Block>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncedBlockContent {
    pub synced_from: Option<SyncedFrom>,
    pub children: Option<Vec<Block>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncedFrom {
    pub block_id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableContent {
    pub table_width: u32,
    pub has_column_header: bool,
    pub has_row_header: bool,
    pub children: Option<Vec<Block>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableRowContent {
    pub cells: Vec<Vec<RichText>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeetingNotesContent {
    pub rich_text: Vec<RichText>,
    pub children: Option<Vec<Block>>,
}

// ── Position (2026-03-11) ─────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    #[serde(flatten)]
    pub position_type: PositionType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum PositionType {
    #[serde(rename = "after_block")]
    AfterBlock { after_block: BlockIdRef },
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "end")]
    End,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockIdRef {
    pub id: ObjectId,
}

// ── Helper ────────────────────────────────────────────────────────

impl Block {
    pub fn type_str(&self) -> &str {
        match &self.block_type {
            BlockType::Paragraph { .. } => "paragraph",
            BlockType::Heading1 { .. } => "heading_1",
            BlockType::Heading2 { .. } => "heading_2",
            BlockType::Heading3 { .. } => "heading_3",
            BlockType::BulletedListItem { .. } => "bulleted_list_item",
            BlockType::NumberedListItem { .. } => "numbered_list_item",
            BlockType::ToDo { .. } => "to_do",
            BlockType::Toggle { .. } => "toggle",
            BlockType::ChildPage { .. } => "child_page",
            BlockType::ChildDatabase { .. } => "child_database",
            BlockType::Embed { .. } => "embed",
            BlockType::Image { .. } => "image",
            BlockType::Video { .. } => "video",
            BlockType::File { .. } => "file",
            BlockType::Pdf { .. } => "pdf",
            BlockType::Bookmark { .. } => "bookmark",
            BlockType::Callout { .. } => "callout",
            BlockType::Quote { .. } => "quote",
            BlockType::Equation { .. } => "equation",
            BlockType::Divider { .. } => "divider",
            BlockType::TableOfContents { .. } => "table_of_contents",
            BlockType::Breadcrumb { .. } => "breadcrumb",
            BlockType::ColumnList { .. } => "column_list",
            BlockType::Column { .. } => "column",
            BlockType::LinkPreview { .. } => "link_preview",
            BlockType::Template { .. } => "template",
            BlockType::SyncedBlock { .. } => "synced_block",
            BlockType::Table { .. } => "table",
            BlockType::TableRow { .. } => "table_row",
            BlockType::MeetingNotes { .. } => "meeting_notes",
            BlockType::Unknown => "unknown",
        }
    }
}
