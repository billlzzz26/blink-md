//! Markdown Converter
//!
//! Implementation of FromPlatform and ToPlatform traits for Markdown (CommonMark + GFM).

use crate::api::markdown::parse_markdown;
use crate::converter::{ConverterError, FromPlatform, ToPlatform};
use crate::ir::blocks::{ListItem, TableRow, TaskItem};
use crate::ir::inline::{InlineElement, TextStyle};
use crate::ir::table::{CellAlignment, TableCell, TableRowType};
use crate::ir::{DocumentMetadata, Platform, StyleSheet, UniversalBlock, UniversalDocument};

pub struct MarkdownConverter;

impl FromPlatform for MarkdownConverter {
    const PLATFORM: Platform = Platform::Markdown;
    type Input = String;

    fn from_platform(input: Self::Input) -> Result<UniversalDocument, ConverterError> {
        let notion_blocks = parse_markdown(&input);

        let mut universal_blocks = Vec::new();
        for block in notion_blocks {
            universal_blocks.push(bridge_notion_block_to_universal(block));
        }

        Ok(UniversalDocument {
            metadata: DocumentMetadata::default(),
            blocks: universal_blocks,
            styles: StyleSheet::default(),
        })
    }
}

impl ToPlatform for MarkdownConverter {
    const PLATFORM: Platform = Platform::Markdown;
    type Output = String;

    fn to_platform(doc: &UniversalDocument) -> Result<Self::Output, ConverterError> {
        let mut result = String::new();
        for block in &doc.blocks {
            render_universal_block(block, 0, &mut result);
            result.push('\n');
        }
        Ok(result.trim_end().to_string())
    }
}

fn bridge_notion_block_to_universal(block: crate::models::block::Block) -> UniversalBlock {
    use crate::models::block::BlockType as NBT;

    match block.block_type {
        NBT::Paragraph { paragraph } => UniversalBlock::Paragraph {
            content: bridge_rich_text_to_inline(paragraph.rich_text),
            style: None,
        },
        NBT::Heading1 { heading_1 } => UniversalBlock::Heading {
            level: 1,
            content: bridge_rich_text_to_inline(heading_1.rich_text),
            style: None,
        },
        NBT::Heading2 { heading_2 } => UniversalBlock::Heading {
            level: 2,
            content: bridge_rich_text_to_inline(heading_2.rich_text),
            style: None,
        },
        NBT::Heading3 { heading_3 } => UniversalBlock::Heading {
            level: 3,
            content: bridge_rich_text_to_inline(heading_3.rich_text),
            style: None,
        },
        NBT::BulletedListItem { bulleted_list_item } => UniversalBlock::BulletList {
            items: vec![ListItem {
                content: vec![UniversalBlock::Paragraph {
                    content: bridge_rich_text_to_inline(bulleted_list_item.rich_text),
                    style: None,
                }],
                style: None,
            }],
            style: None,
        },
        NBT::NumberedListItem { numbered_list_item } => UniversalBlock::OrderedList {
            items: vec![ListItem {
                content: vec![UniversalBlock::Paragraph {
                    content: bridge_rich_text_to_inline(numbered_list_item.rich_text),
                    style: None,
                }],
                style: None,
            }],
            start: 1,
            style: None,
        },
        NBT::ToDo { to_do } => UniversalBlock::TaskList {
            items: vec![TaskItem {
                content: vec![UniversalBlock::Paragraph {
                    content: bridge_rich_text_to_inline(to_do.rich_text),
                    style: None,
                }],
                checked: to_do.checked,
                style: None,
            }],
            style: None,
        },
        NBT::Divider {} => UniversalBlock::PageBreak,
        NBT::Quote { quote } => UniversalBlock::Quote {
            content: vec![UniversalBlock::Paragraph {
                content: bridge_rich_text_to_inline(quote.rich_text),
                style: None,
            }],
            style: None,
        },
        NBT::Callout { callout } => UniversalBlock::Callout {
            icon: callout.icon.map(|i| match i {
                crate::models::common::Icon::Emoji { emoji } => emoji,
                _ => "ℹ️".to_string(),
            }),
            color: Some(callout.color),
            content: vec![UniversalBlock::Paragraph {
                content: bridge_rich_text_to_inline(callout.rich_text),
                style: None,
            }],
            style: None,
        },
        NBT::Table { table } => {
            let has_header = table.has_column_header;
            let rows = table
                .children
                .unwrap_or_default()
                .into_iter()
                .enumerate()
                .map(|(i, row_block)| {
                    let cells = match row_block.block_type {
                        NBT::TableRow { table_row } => table_row
                            .cells
                            .into_iter()
                            .map(|cell| TableCell {
                                content: bridge_rich_text_to_inline(cell),
                                colspan: None,
                                rowspan: None,
                                style: None,
                                align: None,
                            })
                            .collect(),
                        _ => Vec::new(),
                    };
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
        _ => UniversalBlock::Raw {
            platform: Platform::Notion,
            data: serde_json::to_value(block).unwrap_or(serde_json::Value::Null),
        },
    }
}

fn bridge_rich_text_to_inline(
    rich_texts: Vec<crate::models::common::RichText>,
) -> Vec<InlineElement> {
    rich_texts
        .into_iter()
        .map(|rt| match rt {
            crate::models::common::RichText::Text {
                text, annotations, ..
            } => {
                let mut style = TextStyle::default();
                if let Some(ann) = annotations {
                    style.bold = Some(ann.bold);
                    style.italic = Some(ann.italic);
                    style.strikethrough = Some(ann.strikethrough);
                    style.code = Some(ann.code);
                    if ann.color != "default" {
                        style.color = Some(ann.color);
                    }
                }
                if let Some(link) = text.link {
                    style.link = Some(link.url);
                }
                InlineElement::TextRun {
                    content: text.content,
                    style: Some(style),
                }
            }
            _ => InlineElement::TextRun {
                content: "[Unsupported RichText]".to_string(),
                style: Some(TextStyle::default()),
            },
        })
        .collect()
}

fn render_universal_block(block: &UniversalBlock, indent: usize, out: &mut String) {
    let tabs = "\t".repeat(indent);
    match block {
        UniversalBlock::Paragraph { content, .. } => {
            out.push_str(&tabs);
            render_inline(content, out);
        }
        UniversalBlock::Heading { level, content, .. } => {
            let mut line = format!("{} {} ", tabs, "#".repeat(*level as usize));
            render_inline(content, &mut line);
            out.push_str(line.trim_start());
        }
        UniversalBlock::Quote { content, .. } => {
            for b in content {
                out.push_str(&tabs);
                out.push_str("> ");
                render_universal_block(b, 0, out);
                out.push('\n');
            }
            if !content.is_empty() {
                out.truncate(out.len() - 1); // remove last newline
            }
        }
        UniversalBlock::BulletList { items, .. } => {
            for item in items {
                for (i, b) in item.content.iter().enumerate() {
                    out.push_str(&tabs);
                    if i == 0 {
                        out.push_str("- ");
                    } else {
                        out.push_str("  ");
                    }
                    render_universal_block(b, 0, out);
                    out.push('\n');
                }
            }
            if !items.is_empty() {
                out.truncate(out.len() - 1);
            }
        }
        UniversalBlock::Table { rows, header, .. } => {
            render_table(rows, header.as_deref(), &tabs, out);
        }
        _ => {
            out.push_str(&tabs);
            out.push_str("<!-- Unsupported UniversalBlock -->");
        }
    }
}

/// Render an IR table as a GFM pipe table. The first row (either the optional
/// `header` field or the first `TableRow`) becomes the header line; column
/// alignment is read from the header cells. No trailing newline is emitted —
/// the caller adds block separators.
fn render_table(rows: &[TableRow], header: Option<&[TableCell]>, tabs: &str, out: &mut String) {
    // Normalise to a single row list whose first entry is the header.
    let mut all_rows: Vec<TableRow> = Vec::new();
    if let Some(cells) = header {
        all_rows.push(TableRow {
            cells: cells.to_vec(),
            row_type: TableRowType::Header,
            style: None,
        });
    }
    all_rows.extend(rows.iter().cloned());
    if all_rows.is_empty() {
        return;
    }

    let ncols = all_rows.iter().map(|r| r.cells.len()).max().unwrap_or(0);
    let render_row = |row: &TableRow, out: &mut String| {
        out.push_str(tabs);
        out.push('|');
        for c in 0..ncols {
            out.push(' ');
            if let Some(cell) = row.cells.get(c) {
                let mut s = String::new();
                render_inline(&cell.content, &mut s);
                out.push_str(&escape_cell(&s));
            }
            out.push_str(" |");
        }
    };

    let head = &all_rows[0];
    render_row(head, out);
    out.push('\n');

    // Separator line with per-column alignment markers from the header cells.
    out.push_str(tabs);
    out.push('|');
    for c in 0..ncols {
        out.push(' ');
        out.push_str(
            match head.cells.get(c).and_then(|cell| cell.align.as_ref()) {
                Some(CellAlignment::Left) => ":---",
                Some(CellAlignment::Center) => ":---:",
                Some(CellAlignment::Right) => "---:",
                None => "---",
            },
        );
        out.push_str(" |");
    }

    for row in &all_rows[1..] {
        out.push('\n');
        render_row(row, out);
    }
}

/// Escape a rendered cell so it is safe inside a GFM pipe table: literal pipes
/// are backslash-escaped and newlines are flattened to spaces.
fn escape_cell(s: &str) -> String {
    s.replace('|', "\\|").replace(['\n', '\r'], " ")
}

fn render_inline(content: &[InlineElement], out: &mut String) {
    for element in content {
        if let InlineElement::TextRun { content, style } = element {
            if let Some(s) = style {
                if s.bold == Some(true) {
                    out.push_str("**");
                }
                if s.italic == Some(true) {
                    out.push('*');
                }
                if s.code == Some(true) {
                    out.push('`');
                }
                if let Some(link) = &s.link {
                    out.push('[');
                    out.push_str(content);
                    out.push_str("](");
                    out.push_str(link);
                    out.push(')');
                } else {
                    out.push_str(content);
                }
                if s.code == Some(true) {
                    out.push('`');
                }
                if s.italic == Some(true) {
                    out.push('*');
                }
                if s.bold == Some(true) {
                    out.push_str("**");
                }
            } else {
                out.push_str(content);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_to_universal_ir() {
        let md = "# Heading 1\nHello **bold**";
        let doc = MarkdownConverter::from_platform(md.to_string()).unwrap();

        assert_eq!(doc.blocks.len(), 2);

        match &doc.blocks[0] {
            UniversalBlock::Heading { level, .. } => assert_eq!(*level, 1),
            _ => panic!("Expected heading"),
        }

        match &doc.blocks[1] {
            UniversalBlock::Paragraph { content, .. } => {
                assert_eq!(content.len(), 2);
                if let InlineElement::TextRun {
                    style: Some(style), ..
                } = &content[1]
                {
                    assert_eq!(style.bold, Some(true));
                } else {
                    panic!("Expected text run with bold style");
                }
            }
            _ => panic!("Expected paragraph"),
        }
    }

    #[test]
    fn test_universal_ir_to_markdown() {
        let doc = UniversalDocument {
            metadata: DocumentMetadata::default(),
            blocks: vec![
                UniversalBlock::Heading {
                    level: 1,
                    content: vec![InlineElement::TextRun {
                        content: "Title".to_string(),
                        style: None,
                    }],
                    style: None,
                },
                UniversalBlock::Paragraph {
                    content: vec![InlineElement::TextRun {
                        content: "Body".to_string(),
                        style: Some(TextStyle {
                            bold: Some(true),
                            ..TextStyle::default()
                        }),
                    }],
                    style: None,
                },
            ],
            styles: StyleSheet::default(),
        };

        let md = MarkdownConverter::to_platform(&doc).unwrap();
        assert_eq!(md, "# Title\n**Body**");
    }

    #[test]
    fn parses_gfm_table_into_ir() {
        let md = "| Name | Age |\n| --- | --- |\n| Alice | 30 |\n| Bob | 25 |";
        let doc = MarkdownConverter::from_platform(md.to_string()).unwrap();

        let table = doc
            .blocks
            .iter()
            .find_map(|b| match b {
                UniversalBlock::Table { rows, .. } => Some(rows),
                _ => None,
            })
            .expect("expected a table block");

        // 1 header row + 2 body rows.
        assert_eq!(table.len(), 3);
        assert!(matches!(table[0].row_type, TableRowType::Header));
        assert!(matches!(table[1].row_type, TableRowType::Body));
        assert_eq!(table[0].cells.len(), 2);

        let cell_text = |cell: &TableCell| {
            let mut s = String::new();
            render_inline(&cell.content, &mut s);
            s
        };
        assert_eq!(cell_text(&table[0].cells[0]), "Name");
        assert_eq!(cell_text(&table[2].cells[1]), "25");
    }

    #[test]
    fn renders_ir_table_to_gfm() {
        let doc = UniversalDocument {
            metadata: DocumentMetadata::default(),
            blocks: vec![UniversalBlock::Table {
                rows: vec![
                    TableRow {
                        cells: vec![
                            TableCell {
                                content: vec![InlineElement::TextRun {
                                    content: "Name".to_string(),
                                    style: None,
                                }],
                                colspan: None,
                                rowspan: None,
                                style: None,
                                align: None,
                            },
                            TableCell {
                                content: vec![InlineElement::TextRun {
                                    content: "Age".to_string(),
                                    style: None,
                                }],
                                colspan: None,
                                rowspan: None,
                                style: None,
                                align: Some(CellAlignment::Right),
                            },
                        ],
                        row_type: TableRowType::Header,
                        style: None,
                    },
                    TableRow {
                        cells: vec![
                            TableCell {
                                content: vec![InlineElement::TextRun {
                                    content: "Alice".to_string(),
                                    style: None,
                                }],
                                colspan: None,
                                rowspan: None,
                                style: None,
                                align: None,
                            },
                            TableCell {
                                content: vec![InlineElement::TextRun {
                                    content: "30".to_string(),
                                    style: None,
                                }],
                                colspan: None,
                                rowspan: None,
                                style: None,
                                align: None,
                            },
                        ],
                        row_type: TableRowType::Body,
                        style: None,
                    },
                ],
                header: None,
                style: None,
            }],
            styles: StyleSheet::default(),
        };

        let md = MarkdownConverter::to_platform(&doc).unwrap();
        assert_eq!(md, "| Name | Age |\n| --- | ---: |\n| Alice | 30 |");
    }

    #[test]
    fn table_roundtrips_through_ir() {
        let md = "| A | B |\n| --- | --- |\n| 1 | 2 |";
        let doc = MarkdownConverter::from_platform(md.to_string()).unwrap();
        let out = MarkdownConverter::to_platform(&doc).unwrap();
        assert_eq!(out, md);
    }

    #[test]
    fn escapes_pipes_in_cells() {
        assert_eq!(escape_cell("a|b"), "a\\|b");
        assert_eq!(escape_cell("line1\nline2"), "line1 line2");
    }
}
