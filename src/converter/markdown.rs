//! Markdown Converter
//!
//! Implementation of FromPlatform and ToPlatform traits for Markdown (CommonMark + GFM).

use crate::converter::{ConverterError, FromPlatform, ToPlatform};
use crate::ir::{Platform, UniversalDocument, UniversalBlock, DocumentMetadata, StyleSheet};
use crate::ir::inline::{InlineElement, TextStyle};
use crate::ir::blocks::{ListItem, TaskItem};
use crate::api::markdown::parse_markdown;

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
        _ => UniversalBlock::Raw {
            platform: Platform::Notion,
            data: serde_json::to_value(block).unwrap_or(serde_json::Value::Null),
        },
    }
}

fn bridge_rich_text_to_inline(rich_texts: Vec<crate::models::common::RichText>) -> Vec<InlineElement> {
    rich_texts.into_iter().map(|rt| {
        match rt {
            crate::models::common::RichText::Text { text, annotations, .. } => {
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
        }
    }).collect()
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
        _ => {
            out.push_str(&tabs);
            out.push_str("<!-- Unsupported UniversalBlock -->");
        }
    }
}

fn render_inline(content: &[InlineElement], out: &mut String) {
    for element in content {
        match element {
            InlineElement::TextRun { content, style } => {
                if let Some(s) = style {
                    if s.bold == Some(true) { out.push_str("**"); }
                    if s.italic == Some(true) { out.push_str("*"); }
                    if s.code == Some(true) { out.push_str("`") }
                    if let Some(link) = &s.link {
                        out.push('[');
                        out.push_str(content);
                        out.push_str("](");
                        out.push_str(link);
                        out.push(')');
                    } else {
                        out.push_str(content);
                    }
                    if s.code == Some(true) { out.push_str("`") }
                    if s.italic == Some(true) { out.push_str("*"); }
                    if s.bold == Some(true) { out.push_str("**"); }
                } else {
                    out.push_str(content);
                }
            }
            _ => {}
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
                if let InlineElement::TextRun { style: Some(style), .. } = &content[1] {
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
}
