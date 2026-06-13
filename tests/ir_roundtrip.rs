use blink_md::converter::markdown::MarkdownConverter;
use blink_md::converter::notion::{NotionFromPlatform, NotionToPlatform, PageWithBlocks};
use blink_md::converter::{ConverterError, FromPlatform, ToPlatform};
use blink_md::ir::{Platform, UniversalDocument};
use blink_md::models::block::{Block, BlockType};
use blink_md::models::page::Page;

/// Helper to perform a roundtrip and return both IR and final output
pub fn perform_roundtrip<F, T>(
    input: F::Input,
) -> Result<(UniversalDocument, T::Output), ConverterError>
where
    F: FromPlatform,
    T: ToPlatform,
{
    let ir = F::from_platform(input)?;
    let output = T::to_platform(&ir)?;
    Ok((ir, output))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_markdown_ir_roundtrip_simple() {
        let input = "# Heading\n\nParagraph with **bold** text.".to_string();
        let (ir, output) =
            perform_roundtrip::<MarkdownConverter, MarkdownConverter>(input.clone()).unwrap();

        assert_eq!(ir.blocks.len(), 2);
        // Markdown output might have slight formatting differences, but should be semantically equivalent
        assert!(output.contains("# Heading"));
        assert!(output.contains("Paragraph with **bold** text"));
    }

    #[test]
    fn test_cross_platform_roundtrip() {
        // Markdown -> IR -> Notion Request
        let input = "## Notion Task\n- [ ] Todo item".to_string();
        let (ir, notion_req) =
            perform_roundtrip::<MarkdownConverter, NotionToPlatform>(input).unwrap();

        assert_eq!(ir.blocks.len(), 2);
        let children = notion_req.children.unwrap();
        assert_eq!(children.len(), 1); // TaskList is one block in IR but might be mapped differently

        if let BlockType::ToDo { to_do } = &children[0].block_type {
            assert_eq!(to_do.checked, false);
            assert_eq!(to_do.rich_text[0].plain_text(), "Todo item");
        } else {
            panic!("Expected ToDo block");
        }
    }
}
