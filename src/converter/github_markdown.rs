//! GitHub Flavored Markdown (GFM) Converter
//!
//! Extends the base Markdown converter with GFM-specific features:
//! - Tables with alignment
//! - Task lists
//! - Strikethrough
//! - Alerts (blockquote extensions like > [!NOTE])

use crate::converter::{ConverterError, FromPlatform, ToPlatform};
use crate::ir::{InlineElement, Platform, UniversalBlock, UniversalDocument};

pub struct GithubMarkdownConverter;

impl FromPlatform for GithubMarkdownConverter {
    const PLATFORM: Platform = Platform::Markdown;
    type Input = String;

    fn from_platform(input: Self::Input) -> Result<UniversalDocument, ConverterError> {
        // For now, delegate to basic markdown parser as a starting point
        // we use GFM options in the future enhancement
        crate::converter::markdown::MarkdownConverter::from_platform(input)
    }
}

impl ToPlatform for GithubMarkdownConverter {
    const PLATFORM: Platform = Platform::Markdown;
    type Output = String;

    fn to_platform(doc: &UniversalDocument) -> Result<Self::Output, ConverterError> {
        let mut result = String::new();
        for block in &doc.blocks {
            render_gfm_block(block, 0, &mut result);
            result.push('\n');
        }
        Ok(result.trim_end().to_string())
    }
}

fn render_gfm_block(block: &UniversalBlock, indent: usize, out: &mut String) {
    let tabs = "\t".repeat(indent);
    match block {
        UniversalBlock::Paragraph { content, .. } => {
            out.push_str(&tabs);
            render_gfm_inline(content, out);
        }
        UniversalBlock::Heading { level, content, .. } => {
            out.push_str(&tabs);
            out.push_str(&"#".repeat(*level as usize));
            out.push(' ');
            render_gfm_inline(content, out);
        }
        UniversalBlock::Callout { content, .. } => {
            // GFM Alert syntax: > [!NOTE]
            out.push_str(&tabs);
            out.push_str("> [!NOTE]\n"); 
            for b in content {
                out.push_str(&tabs);
                out.push_str("> ");
                render_gfm_block(b, 0, out);
                out.push('\n');
            }
        }
        UniversalBlock::TaskList { items, .. } => {
            for item in items {
                out.push_str(&tabs);
                let check = if item.checked { "[x] " } else { "[ ] " };
                out.push_str("- ");
                out.push_str(check);
                for (i, b) in item.content.iter().enumerate() {
                    if i > 0 { out.push_str("  "); }
                    render_gfm_block(b, 0, out);
                }
                out.push('\n');
            }
        }
        _ => {
            // Placeholder for other blocks
        }
    }
}

fn render_gfm_inline(content: &[InlineElement], out: &mut String) {
    for element in content {
        match element {
            InlineElement::TextRun { content, style } => {
                if let Some(s) = style {
                    if s.strikethrough == Some(true) { out.push_str("~~"); }
                    out.push_str(content);
                    if s.strikethrough == Some(true) { out.push_str("~~"); }
                } else {
                    out.push_str(content);
                }
            }
            _ => {}
        }
    }
}
