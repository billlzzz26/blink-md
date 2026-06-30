//! Notion ↔ Universal IR Converter

use crate::converter::{ConverterError, FromPlatform, ToPlatform};
use crate::ir::{
    blocks::{EmbedProvider, ListItem, MediaSource, MentionType, TaskItem, UniversalBlock},
    inline::{InlineElement, TextStyle},
    metadata::{DocumentMetadata, PropertyValue},
    style::StyleSheet,
    table::{TableCell, TableRow, TableRowType},
    Platform, UniversalDocument,
};
use crate::models::{
    block::{Block, BlockType, TableContent, TableRowContent},
    common::{Annotations, FileBlockContent, Icon, MentionObject, RichText, User},
    page::{CreatePageRequest, Page},
};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;

/// Notion API → Universal IR
pub struct NotionFromPlatform;

/// A Notion Page along with its child blocks (recursively).
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PageWithBlocks {
    pub page: Page,
    pub blocks: Vec<Block>,
}

impl FromPlatform for NotionFromPlatform {
    const PLATFORM: Platform = Platform::Notion;
    type Input = PageWithBlocks;

    fn from_platform(input: PageWithBlocks) -> Result<UniversalDocument, ConverterError> {
        let PageWithBlocks { page, blocks } = input;

        // The Notion API (and `get_block_children_recursive`) returns a table as
        // a `Table` block followed by its `TableRow` children flattened as
        // siblings. Re-group them into a single IR `Table` so the rows are not
        // emitted as separate one-row tables.
        let mut ir_blocks = Vec::new();
        let mut iter = blocks.into_iter().peekable();
        while let Some(block) = iter.next() {
            if let BlockType::Table { table } = &block.block_type {
                let has_header = table.has_column_header;
                let mut row_contents: Vec<TableRowContent> = Vec::new();
                if let Some(children) = &table.children {
                    for child in children {
                        if let BlockType::TableRow { table_row } = &child.block_type {
                            row_contents.push(table_row.clone());
                        }
                    }
                }
                while let Some(BlockType::TableRow { .. }) = iter.peek().map(|b| &b.block_type) {
                    if let Some(Block {
                        block_type: BlockType::TableRow { table_row },
                        ..
                    }) = iter.next()
                    {
                        row_contents.push(table_row);
                    }
                }
                ir_blocks.push(table_rows_to_ir(has_header, row_contents));
            } else {
                ir_blocks.push(block_to_ir(&block)?);
            }
        }

        let metadata = DocumentMetadata {
            title: Some(page.title_from_properties()),
            author: Some(page.created_by.id.clone()),
            created_time: Some(page.created_time),
            last_edited_time: Some(page.last_edited_time),
            properties: page
                .properties
                .as_object()
                .map(|obj| {
                    obj.iter()
                        .map(|(k, v)| (k.clone(), property_value_to_ir(v)))
                        .collect()
                })
                .unwrap_or_default(),
            source_platform: Some(Platform::Notion),
            source_id: Some(page.id),
            custom: HashMap::new(),
        };

        let styles = StyleSheet::default();

        Ok(UniversalDocument {
            metadata,
            blocks: ir_blocks,
            styles,
        })
    }
}

/// Universal IR → Notion API
pub struct NotionToPlatform;

impl ToPlatform for NotionToPlatform {
    const PLATFORM: Platform = Platform::Notion;
    type Output = CreatePageRequest;

    fn to_platform(doc: &UniversalDocument) -> Result<Self::Output, ConverterError> {
        let parent = doc
            .metadata
            .source_id
            .as_ref()
            .map(|id| serde_json::json!({ "page_id": id }))
            .unwrap_or(serde_json::json!({ "workspace": true }));

        let properties = properties_from_ir(&doc.metadata.properties)?;
        let children = blocks_to_notion(&doc.blocks)?;

        Ok(CreatePageRequest {
            parent,
            properties,
            children: Some(children),
            ..Default::default()
        })
    }
}

/// Convert Notion RichText to IR InlineElements
pub fn rich_text_to_ir(rich_text: &[RichText]) -> Vec<InlineElement> {
    rich_text.iter().map(rich_text_single_to_ir).collect()
}

fn rich_text_single_to_ir(rt: &RichText) -> InlineElement {
    match rt {
        RichText::Text {
            text, annotations, ..
        } => {
            let mut style = TextStyle::new();
            if let Some(ann) = annotations {
                if ann.bold {
                    style = style.bold();
                }
                if ann.italic {
                    style = style.italic();
                }
                if ann.strikethrough {
                    style = style.strikethrough();
                }
                if ann.underline {
                    style = style.underline();
                }
                if ann.code {
                    style = style.code();
                }
                if !ann.color.is_empty() && ann.color != "default" {
                    style = style.color(ann.color.clone());
                }
            }
            if let Some(url) = rt.href() {
                style = style.link(url.to_string());
            }
            InlineElement::TextRun {
                content: text.content.clone(),
                style: Some(style),
            }
        }
        RichText::Mention {
            mention,
            annotations,
            ..
        } => {
            let mut style = TextStyle::new();
            if let Some(ann) = annotations {
                if ann.bold {
                    style = style.bold();
                }
                if ann.italic {
                    style = style.italic();
                }
                if ann.strikethrough {
                    style = style.strikethrough();
                }
                if ann.underline {
                    style = style.underline();
                }
                if ann.code {
                    style = style.code();
                }
                if !ann.color.is_empty() && ann.color != "default" {
                    style = style.color(ann.color.clone());
                }
            }
            if let Some(url) = rt.href() {
                style = style.link(url.to_string());
            }
            InlineElement::Mention {
                mention_type: mention_object_to_type(mention),
                target: mention_object_to_target(mention),
                label: Some(rt.plain_text().to_string()),
                style: Some(style),
            }
        }
        RichText::Equation {
            equation,
            annotations,
            ..
        } => {
            let mut style = TextStyle::new();
            if let Some(ann) = annotations {
                if ann.bold {
                    style = style.bold();
                }
                if ann.italic {
                    style = style.italic();
                }
                if ann.strikethrough {
                    style = style.strikethrough();
                }
                if ann.underline {
                    style = style.underline();
                }
                if ann.code {
                    style = style.code();
                }
                if !ann.color.is_empty() && ann.color != "default" {
                    style = style.color(ann.color.clone());
                }
            }
            if let Some(url) = rt.href() {
                style = style.link(url.to_string());
            }
            InlineElement::Equation {
                expression: equation.expression.clone(),
                style: Some(style),
            }
        }
    }
}

/// Convert MentionObject to MentionType
fn mention_object_to_type(mention: &MentionObject) -> MentionType {
    match mention {
        MentionObject::User { .. } => MentionType::User,
        MentionObject::Page { .. } => MentionType::Page,
        MentionObject::Database { .. } => MentionType::Database,
        MentionObject::Date { .. } => MentionType::Date,
        MentionObject::LinkPreview { .. } => MentionType::LinkPreview,
    }
}

/// Convert MentionObject to target string
fn mention_object_to_target(mention: &MentionObject) -> String {
    match mention {
        MentionObject::User { user } => user.id.clone(),
        MentionObject::Page { page } => page.id.clone(),
        MentionObject::Database { database } => database.id.clone(),
        MentionObject::Date { date } => date.to_string(),
        MentionObject::LinkPreview { url } => url.clone(),
    }
}

/// Convert Notion Block to UniversalBlock
pub fn block_to_ir(block: &Block) -> Result<UniversalBlock, ConverterError> {
    match &block.block_type {
        BlockType::Paragraph { paragraph } => Ok(UniversalBlock::Paragraph {
            content: rich_text_to_ir(&paragraph.rich_text),
            style: None,
        }),
        BlockType::Heading1 { heading_1 } => Ok(UniversalBlock::Heading {
            level: 1,
            content: rich_text_to_ir(&heading_1.rich_text),
            style: None,
        }),
        BlockType::Heading2 { heading_2 } => Ok(UniversalBlock::Heading {
            level: 2,
            content: rich_text_to_ir(&heading_2.rich_text),
            style: None,
        }),
        BlockType::Heading3 { heading_3 } => Ok(UniversalBlock::Heading {
            level: 3,
            content: rich_text_to_ir(&heading_3.rich_text),
            style: None,
        }),
        BlockType::CodeBlock { code } => Ok(UniversalBlock::CodeBlock {
            language: Some(code.language.clone()),
            content: rich_text_to_ir(&code.rich_text)
                .iter()
                .map(|e| match e {
                    InlineElement::TextRun { content, .. } => content.clone(),
                    _ => String::new(),
                })
                .collect::<Vec<_>>()
                .join(""),
            style: None,
        }),
        BlockType::BulletedListItem { bulleted_list_item } => Ok(UniversalBlock::BulletList {
            items: vec![ListItem {
                content: vec![UniversalBlock::Paragraph {
                    content: rich_text_to_ir(&bulleted_list_item.rich_text),
                    style: None,
                }],
                style: None,
            }],
            style: None,
        }),
        BlockType::NumberedListItem { numbered_list_item } => Ok(UniversalBlock::OrderedList {
            items: vec![ListItem {
                content: vec![UniversalBlock::Paragraph {
                    content: rich_text_to_ir(&numbered_list_item.rich_text),
                    style: None,
                }],
                style: None,
            }],
            start: 1,
            style: None,
        }),
        BlockType::ToDo { to_do } => Ok(UniversalBlock::TaskList {
            items: vec![TaskItem {
                content: vec![UniversalBlock::Paragraph {
                    content: rich_text_to_ir(&to_do.rich_text),
                    style: None,
                }],
                checked: to_do.checked,
                style: None,
            }],
            style: None,
        }),
        BlockType::Toggle { toggle } => Ok(UniversalBlock::Toggle {
            summary: rich_text_to_ir(&toggle.rich_text),
            content: vec![],
            style: None,
        }),
        BlockType::Callout { callout } => Ok(UniversalBlock::Callout {
            icon: callout.icon.as_ref().and_then(|i| match i {
                Icon::Emoji { emoji } => Some(emoji.clone()),
                _ => None,
            }),
            color: Some(callout.color.clone()),
            content: vec![UniversalBlock::Paragraph {
                content: rich_text_to_ir(&callout.rich_text),
                style: None,
            }],
            style: None,
        }),
        BlockType::Quote { quote } => Ok(UniversalBlock::Quote {
            content: vec![UniversalBlock::Paragraph {
                content: rich_text_to_ir(&quote.rich_text),
                style: None,
            }],
            style: None,
        }),
        BlockType::Image { image } => Ok(UniversalBlock::Image {
            src: file_content_to_media_source(image)?,
            alt: None,
            caption: None,
            style: None,
        }),
        BlockType::Video { video } => Ok(UniversalBlock::Video {
            src: file_content_to_media_source(video)?,
            caption: None,
            style: None,
        }),
        BlockType::File { file } => Ok(UniversalBlock::File {
            src: file_content_to_media_source(file)?,
            name: "file".to_string(),
            style: None,
        }),
        BlockType::Pdf { pdf } => Ok(UniversalBlock::File {
            src: file_content_to_media_source(pdf)?,
            name: "document.pdf".to_string(),
            style: None,
        }),
        BlockType::Bookmark { bookmark } => Ok(UniversalBlock::Embed {
            url: bookmark.url.clone(),
            provider: EmbedProvider::Other("bookmark".to_string()),
            fallback: None,
            style: None,
        }),
        BlockType::Embed { embed } => Ok(UniversalBlock::Embed {
            url: embed.url.clone(),
            provider: EmbedProvider::Other("embed".to_string()),
            fallback: None,
            style: None,
        }),
        BlockType::Table { .. } => Ok(UniversalBlock::Table {
            rows: vec![],
            header: None,
            style: None,
        }),
        BlockType::TableRow { table_row } => {
            let cells: Vec<TableCell> = table_row
                .cells
                .iter()
                .map(|cell| TableCell {
                    content: cell
                        .iter()
                        .map(|rt| InlineElement::TextRun {
                            content: rt.plain_text().to_string(),
                            style: None,
                        })
                        .collect(),
                    colspan: None,
                    rowspan: None,
                    style: None,
                    align: None,
                })
                .collect();
            Ok(UniversalBlock::Table {
                rows: vec![TableRow {
                    cells,
                    row_type: TableRowType::Body,
                    style: None,
                }],
                header: None,
                style: None,
            })
        }
        _ => Ok(UniversalBlock::Raw {
            platform: Platform::Notion,
            data: serde_json::json!({ "type": "unsupported" }),
        }),
    }
}

/// Build an IR [`UniversalBlock::Table`] from Notion `TableRow` contents.
///
/// The first row is marked as a header when `has_header` is set, matching the
/// Notion table's `has_column_header` flag.
fn table_rows_to_ir(has_header: bool, rows: Vec<TableRowContent>) -> UniversalBlock {
    let rows = rows
        .into_iter()
        .enumerate()
        .map(|(i, tr)| {
            let cells = tr
                .cells
                .into_iter()
                .map(|cell| TableCell {
                    content: cell
                        .iter()
                        .map(|rt| InlineElement::TextRun {
                            content: rt.plain_text().to_string(),
                            style: None,
                        })
                        .collect(),
                    colspan: None,
                    rowspan: None,
                    style: None,
                    align: None,
                })
                .collect();
            let row_type = if i == 0 && has_header {
                TableRowType::Header
            } else {
                TableRowType::Body
            };
            TableRow {
                cells,
                row_type,
                style: None,
            }
        })
        .collect();
    UniversalBlock::Table {
        rows,
        header: None,
        style: None,
    }
}

/// Convert Universal IR blocks to Notion Blocks
pub fn blocks_to_notion(blocks: &[UniversalBlock]) -> Result<Vec<Block>, ConverterError> {
    blocks.iter().map(block_ir_to_notion).collect()
}

fn block_ir_to_notion(block: &UniversalBlock) -> Result<Block, ConverterError> {
    match block {
        UniversalBlock::Paragraph { content, .. } => Ok(Block {
            object: "block".to_string(),
            id: "temp".to_string(),
            created_time: Utc::now(),
            last_edited_time: Utc::now(),
            created_by: User::default(),
            last_edited_by: User::default(),
            has_children: false,
            in_trash: false,
            parent: None,
            block_type: BlockType::Paragraph {
                paragraph: crate::models::block::TextBlockContent {
                    rich_text: inline_to_rich_text(content),
                    color: "default".to_string(),
                    children: None,
                },
            },
        }),
        UniversalBlock::Heading { level, content, .. } => {
            let block_type = match level {
                1 => BlockType::Heading1 {
                    heading_1: heading_content(content),
                },
                2 => BlockType::Heading2 {
                    heading_2: heading_content(content),
                },
                3 => BlockType::Heading3 {
                    heading_3: heading_content(content),
                },
                _ => BlockType::Paragraph {
                    paragraph: paragraph_content(content),
                },
            };
            Ok(Block {
                object: "block".to_string(),
                id: "temp".to_string(),
                created_time: Utc::now(),
                last_edited_time: Utc::now(),
                created_by: User::default(),
                last_edited_by: User::default(),
                has_children: false,
                in_trash: false,
                parent: None,
                block_type,
            })
        }
        UniversalBlock::CodeBlock {
            language, content, ..
        } => Ok(Block {
            object: "block".to_string(),
            id: "temp".to_string(),
            created_time: Utc::now(),
            last_edited_time: Utc::now(),
            created_by: User::default(),
            last_edited_by: User::default(),
            has_children: false,
            in_trash: false,
            parent: None,
            block_type: BlockType::CodeBlock {
                code: crate::models::block::CodeBlockContent {
                    rich_text: vec![RichText::Text {
                        text: crate::models::common::TextContent {
                            content: content.clone(),
                            link: None,
                        },
                        annotations: None,
                        plain_text: Some(content.clone()),
                        href: None,
                    }],
                    caption: vec![],
                    language: language.clone().unwrap_or_default(),
                },
            },
        }),
        UniversalBlock::BulletList { items, .. } => Ok(Block {
            object: "block".to_string(),
            id: "temp".to_string(),
            created_time: Utc::now(),
            last_edited_time: Utc::now(),
            created_by: User::default(),
            last_edited_by: User::default(),
            has_children: !items.is_empty(),
            in_trash: false,
            parent: None,
            block_type: BlockType::BulletedListItem {
                bulleted_list_item: crate::models::block::TextBlockContent {
                    rich_text: items
                        .first()
                        .map(|i| {
                            inline_to_rich_text(match i.content.first() {
                                Some(UniversalBlock::Paragraph { content, .. }) => content,
                                _ => &[],
                            })
                        })
                        .unwrap_or_default(),
                    color: "default".to_string(),
                    children: None,
                },
            },
        }),
        UniversalBlock::OrderedList { items, .. } => Ok(Block {
            object: "block".to_string(),
            id: "temp".to_string(),
            created_time: Utc::now(),
            last_edited_time: Utc::now(),
            created_by: User::default(),
            last_edited_by: User::default(),
            has_children: !items.is_empty(),
            in_trash: false,
            parent: None,
            block_type: BlockType::NumberedListItem {
                numbered_list_item: crate::models::block::TextBlockContent {
                    rich_text: items
                        .first()
                        .map(|i| {
                            inline_to_rich_text(match i.content.first() {
                                Some(UniversalBlock::Paragraph { content, .. }) => content,
                                _ => &[],
                            })
                        })
                        .unwrap_or_default(),
                    color: "default".to_string(),
                    children: None,
                },
            },
        }),
        UniversalBlock::TaskList { items, .. } => Ok(Block {
            object: "block".to_string(),
            id: "temp".to_string(),
            created_time: Utc::now(),
            last_edited_time: Utc::now(),
            created_by: User::default(),
            last_edited_by: User::default(),
            has_children: !items.is_empty(),
            in_trash: false,
            parent: None,
            block_type: BlockType::ToDo {
                to_do: crate::models::block::ToDoContent {
                    rich_text: items
                        .first()
                        .map(|i| {
                            inline_to_rich_text(match i.content.first() {
                                Some(UniversalBlock::Paragraph { content, .. }) => content,
                                _ => &[],
                            })
                        })
                        .unwrap_or_default(),
                    checked: items.first().map(|i| i.checked).unwrap_or(false),
                    color: "default".to_string(),
                    children: None,
                },
            },
        }),
        UniversalBlock::Toggle { summary, .. } => Ok(Block {
            object: "block".to_string(),
            id: "temp".to_string(),
            created_time: Utc::now(),
            last_edited_time: Utc::now(),
            created_by: User::default(),
            last_edited_by: User::default(),
            has_children: true,
            in_trash: false,
            parent: None,
            block_type: BlockType::Toggle {
                toggle: crate::models::block::TextBlockContent {
                    rich_text: inline_to_rich_text(summary),
                    color: "default".to_string(),
                    children: None,
                },
            },
        }),
        UniversalBlock::Callout {
            icon,
            color,
            content,
            ..
        } => Ok(Block {
            object: "block".to_string(),
            id: "temp".to_string(),
            created_time: Utc::now(),
            last_edited_time: Utc::now(),
            created_by: User::default(),
            last_edited_by: User::default(),
            has_children: !content.is_empty(),
            in_trash: false,
            parent: None,
            block_type: BlockType::Callout {
                callout: crate::models::block::CalloutContent {
                    rich_text: content
                        .first()
                        .map(|c| match c {
                            UniversalBlock::Paragraph { content, .. } => {
                                inline_to_rich_text(content)
                            }
                            _ => vec![],
                        })
                        .unwrap_or_default(),
                    icon: icon.as_ref().map(|e| Icon::Emoji { emoji: e.clone() }),
                    color: color.clone().unwrap_or_else(|| "default".to_string()),
                    children: None,
                },
            },
        }),
        UniversalBlock::Quote { content, .. } => Ok(Block {
            object: "block".to_string(),
            id: "temp".to_string(),
            created_time: Utc::now(),
            last_edited_time: Utc::now(),
            created_by: User::default(),
            last_edited_by: User::default(),
            has_children: false,
            in_trash: false,
            parent: None,
            block_type: BlockType::Quote {
                quote: crate::models::block::TextBlockContent {
                    rich_text: inline_to_rich_text(
                        &content
                            .first()
                            .map(|c| match c {
                                UniversalBlock::Paragraph { content, .. } => content.clone(),
                                _ => vec![],
                            })
                            .unwrap_or_default(),
                    ),
                    color: "default".to_string(),
                    children: None,
                },
            },
        }),
        UniversalBlock::Image { src, .. } => Ok(Block {
            object: "block".to_string(),
            id: "temp".to_string(),
            created_time: Utc::now(),
            last_edited_time: Utc::now(),
            created_by: User::default(),
            last_edited_by: User::default(),
            has_children: false,
            in_trash: false,
            parent: None,
            block_type: BlockType::Image {
                image: file_block_content_from_media(src)?,
            },
        }),
        UniversalBlock::Video { src, .. } => Ok(Block {
            object: "block".to_string(),
            id: "temp".to_string(),
            created_time: Utc::now(),
            last_edited_time: Utc::now(),
            created_by: User::default(),
            last_edited_by: User::default(),
            has_children: false,
            in_trash: false,
            parent: None,
            block_type: BlockType::Video {
                video: file_block_content_from_media(src)?,
            },
        }),
        UniversalBlock::Table { rows, header, .. } => {
            // Notion requires every TableRow to have exactly `table_width`
            // cells, so compute the width up front and pad ragged rows.
            let table_width = rows
                .iter()
                .map(|r| r.cells.len())
                .chain(header.iter().map(|h| h.len()))
                .max()
                .unwrap_or(0);
            let mut notion_rows: Vec<Block> = Vec::new();
            let mut has_column_header = false;
            if let Some(cells) = header {
                has_column_header = true;
                notion_rows.push(table_row_to_notion(cells, table_width));
            }
            for row in rows {
                if matches!(row.row_type, TableRowType::Header) {
                    has_column_header = true;
                }
                notion_rows.push(table_row_to_notion(&row.cells, table_width));
            }
            let table_width = table_width as u32;
            Ok(Block {
                object: "block".to_string(),
                id: "temp".to_string(),
                created_time: Utc::now(),
                last_edited_time: Utc::now(),
                created_by: User::default(),
                last_edited_by: User::default(),
                has_children: !notion_rows.is_empty(),
                in_trash: false,
                parent: None,
                block_type: BlockType::Table {
                    table: TableContent {
                        table_width,
                        has_column_header,
                        has_row_header: false,
                        children: Some(notion_rows),
                    },
                },
            })
        }
        _ => Err(ConverterError::ConversionFailed(
            "Block type not yet implemented for Notion export".to_string(),
        )),
    }
}

/// Build a Notion `TableRow` block from IR table cells, padding to `width`
/// with empty cells so Notion does not reject a ragged row.
fn table_row_to_notion(cells: &[TableCell], width: usize) -> Block {
    let mut row_cells: Vec<Vec<RichText>> = cells
        .iter()
        .map(|c| inline_to_rich_text(&c.content))
        .collect();
    row_cells.resize_with(width.max(row_cells.len()), Vec::new);
    Block {
        object: "block".to_string(),
        id: "temp".to_string(),
        created_time: Utc::now(),
        last_edited_time: Utc::now(),
        created_by: User::default(),
        last_edited_by: User::default(),
        has_children: false,
        in_trash: false,
        parent: None,
        block_type: BlockType::TableRow {
            table_row: TableRowContent { cells: row_cells },
        },
    }
}

fn heading_content(content: &[InlineElement]) -> crate::models::block::HeadingContent {
    crate::models::block::HeadingContent {
        rich_text: inline_to_rich_text(content),
        color: "default".to_string(),
        is_toggleable: false,
        children: None,
    }
}

fn paragraph_content(content: &[InlineElement]) -> crate::models::block::TextBlockContent {
    crate::models::block::TextBlockContent {
        rich_text: inline_to_rich_text(content),
        color: "default".to_string(),
        children: None,
    }
}

/// Convert IR InlineElements to Notion RichText
fn inline_to_rich_text(elements: &[InlineElement]) -> Vec<RichText> {
    elements.iter().map(inline_single_to_rich_text).collect()
}

fn inline_single_to_rich_text(elem: &InlineElement) -> RichText {
    match elem {
        InlineElement::TextRun { content, style } => {
            let mut annotations = Annotations::default();
            if let Some(s) = style {
                if s.bold == Some(true) {
                    annotations.bold = true;
                }
                if s.italic == Some(true) {
                    annotations.italic = true;
                }
                if s.strikethrough == Some(true) {
                    annotations.strikethrough = true;
                }
                if s.underline == Some(true) {
                    annotations.underline = true;
                }
                if s.code == Some(true) {
                    annotations.code = true;
                }
                if let Some(color) = &s.color {
                    annotations.color = color.clone();
                }
            }
            RichText::Text {
                text: crate::models::common::TextContent {
                    content: content.clone(),
                    link: style
                        .as_ref()
                        .and_then(|s| s.link.as_ref())
                        .map(|url| crate::models::common::Link { url: url.clone() }),
                },
                annotations: Some(annotations),
                plain_text: Some(content.clone()),
                href: None,
            }
        }
        _ => RichText::Text {
            text: crate::models::common::TextContent {
                content: "unsupported".to_string(),
                link: None,
            },
            annotations: None,
            plain_text: Some("unsupported".to_string()),
            href: None,
        },
    }
}

/// Convert FileBlockContent to MediaSource
fn file_content_to_media_source(file: &FileBlockContent) -> Result<MediaSource, ConverterError> {
    match &file.file_type {
        crate::models::common::FileType::External { external } => Ok(MediaSource::External {
            url: external.url.clone(),
        }),
        crate::models::common::FileType::Uploaded { file } => Ok(MediaSource::Uploaded {
            url: file.url.clone(),
            expiry_time: file.expiry_time.map(|t| t.to_string()),
        }),
    }
}

/// Convert MediaSource to FileBlockContent
fn file_block_content_from_media(src: &MediaSource) -> Result<FileBlockContent, ConverterError> {
    match src {
        MediaSource::External { url } => Ok(FileBlockContent {
            file_type: crate::models::common::FileType::External {
                external: crate::models::common::ExternalFile { url: url.clone() },
            },
        }),
        MediaSource::Uploaded { url, expiry_time } => Ok(FileBlockContent {
            file_type: crate::models::common::FileType::Uploaded {
                file: crate::models::common::UploadedFile {
                    url: url.clone(),
                    expiry_time: expiry_time
                        .as_ref()
                        .and_then(|e| DateTime::parse_from_rfc3339(e).ok())
                        .map(|d| d.with_timezone(&Utc)),
                },
            },
        }),
        MediaSource::Base64 { .. } => Err(ConverterError::ConversionFailed(
            "Base64 media not supported for Notion export".to_string(),
        )),
    }
}

/// Convert Notion properties to IR PropertyValue
fn property_value_to_ir(value: &Value) -> PropertyValue {
    PropertyValue::Custom {
        key: "unknown".to_string(),
        value: value.clone(),
    }
}

/// Convert IR PropertyValue to Notion properties JSON
fn properties_from_ir(props: &HashMap<String, PropertyValue>) -> Result<Value, ConverterError> {
    let mut map = serde_json::Map::new();
    for (key, value) in props {
        map.insert(key.clone(), property_value_from_ir(value)?);
    }
    Ok(Value::Object(map))
}

fn property_value_from_ir(value: &PropertyValue) -> Result<Value, ConverterError> {
    match value {
        PropertyValue::Title { title } => Ok(serde_json::json!({
            "title": inline_to_rich_text(title)
        })),
        PropertyValue::RichText { rich_text } => Ok(serde_json::json!({
            "rich_text": inline_to_rich_text(rich_text)
        })),
        PropertyValue::Number { number } => Ok(serde_json::json!({ "number": number })),
        PropertyValue::Select { select } => Ok(serde_json::json!({ "select": select })),
        PropertyValue::MultiSelect { multi_select } => {
            Ok(serde_json::json!({ "multi_select": multi_select }))
        }
        PropertyValue::Date { date } => Ok(serde_json::json!({ "date": date })),
        PropertyValue::Checkbox { checkbox } => Ok(serde_json::json!({ "checkbox": checkbox })),
        PropertyValue::Url { url } => Ok(serde_json::json!({ "url": url })),
        PropertyValue::Email { email } => Ok(serde_json::json!({ "email": email })),
        PropertyValue::PhoneNumber { phone_number } => {
            Ok(serde_json::json!({ "phone_number": phone_number }))
        }
        PropertyValue::Relation { relation } => Ok(
            serde_json::json!({ "relation": relation.iter().map(|id| serde_json::json!({ "id": id })).collect::<Vec<_>>() }),
        ),
        PropertyValue::Files { files } => Ok(serde_json::json!({ "files": files })),
        // A `Custom` value already holds the full Notion property body
        // (e.g. `{"people": [...]}` or `{"relation": [...]}`); emit it as-is.
        // `properties_from_ir` keys it under the property name, so wrapping it
        // again here would double-nest and corrupt the property.
        PropertyValue::Custom { value, .. } => Ok(value.clone()),
        _ => Err(ConverterError::ConversionFailed(format!(
            "Property type not yet implemented: {:?}",
            value
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ir_cell(text: &str) -> TableCell {
        TableCell {
            content: vec![InlineElement::TextRun {
                content: text.to_string(),
                style: None,
            }],
            colspan: None,
            rowspan: None,
            style: None,
            align: None,
        }
    }

    fn notion_cell(text: &str) -> Vec<RichText> {
        vec![RichText::Text {
            text: crate::models::common::TextContent {
                content: text.to_string(),
                link: None,
            },
            annotations: None,
            plain_text: Some(text.to_string()),
            href: None,
        }]
    }

    #[test]
    fn ir_table_converts_to_notion_table_block() {
        let table = UniversalBlock::Table {
            rows: vec![
                TableRow {
                    cells: vec![ir_cell("Name"), ir_cell("Age")],
                    row_type: TableRowType::Header,
                    style: None,
                },
                TableRow {
                    cells: vec![ir_cell("Alice"), ir_cell("30")],
                    row_type: TableRowType::Body,
                    style: None,
                },
            ],
            header: None,
            style: None,
        };

        let block = block_ir_to_notion(&table).unwrap();
        match block.block_type {
            BlockType::Table { table } => {
                assert_eq!(table.table_width, 2);
                assert!(table.has_column_header);
                let children = table.children.expect("rows");
                assert_eq!(children.len(), 2);
                assert!(matches!(children[0].block_type, BlockType::TableRow { .. }));
            }
            other => panic!("expected Table, got {:?}", other),
        }
    }

    #[test]
    fn flattened_notion_rows_group_into_single_ir_table() {
        let rows = vec![
            TableRowContent {
                cells: vec![notion_cell("Name"), notion_cell("Age")],
            },
            TableRowContent {
                cells: vec![notion_cell("Alice"), notion_cell("30")],
            },
        ];
        let block = table_rows_to_ir(true, rows);
        match block {
            UniversalBlock::Table { rows, .. } => {
                assert_eq!(rows.len(), 2);
                assert!(matches!(rows[0].row_type, TableRowType::Header));
                assert!(matches!(rows[1].row_type, TableRowType::Body));
                let mut s = String::new();
                for el in &rows[1].cells[0].content {
                    if let InlineElement::TextRun { content, .. } = el {
                        s.push_str(content);
                    }
                }
                assert_eq!(s, "Alice");
            }
            other => panic!("expected Table, got {:?}", other),
        }
    }

    #[test]
    fn ragged_ir_rows_are_padded_to_table_width() {
        let table = UniversalBlock::Table {
            rows: vec![
                TableRow {
                    cells: vec![ir_cell("A"), ir_cell("B"), ir_cell("C")],
                    row_type: TableRowType::Header,
                    style: None,
                },
                TableRow {
                    cells: vec![ir_cell("1")],
                    row_type: TableRowType::Body,
                    style: None,
                },
            ],
            header: None,
            style: None,
        };
        let block = block_ir_to_notion(&table).unwrap();
        match block.block_type {
            BlockType::Table { table } => {
                assert_eq!(table.table_width, 3);
                let children = table.children.expect("rows");
                // Every emitted TableRow must carry exactly `table_width` cells.
                for child in &children {
                    match &child.block_type {
                        BlockType::TableRow { table_row } => {
                            assert_eq!(table_row.cells.len(), 3);
                        }
                        other => panic!("expected TableRow, got {:?}", other),
                    }
                }
            }
            other => panic!("expected Table, got {:?}", other),
        }
    }

    #[test]
    fn custom_property_emits_body_without_double_wrapping() {
        let custom = PropertyValue::Custom {
            key: "People".to_string(),
            value: serde_json::json!({ "people": [{ "id": "u1" }] }),
        };
        let json = property_value_from_ir(&custom).unwrap();
        // Must be the bare property body, not `{ "People": { ... } }`.
        assert_eq!(json, serde_json::json!({ "people": [{ "id": "u1" }] }));
    }
}
