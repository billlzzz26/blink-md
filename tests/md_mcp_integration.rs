//! Integration test for md-mcp-server parse_markdown tool

#[cfg(test)]
mod tests {
    use blink_md::api::markdown::{parse_markdown, ToMarkdown};

    #[test]
    fn test_parse_simple_markdown() {
        let md = "# Heading 1\n\nParagraph text with **bold** and *italic*.";
        let blocks = parse_markdown(md);
        
        assert!(!blocks.is_empty());
        assert!(blocks.iter().any(|b| matches!(b.block_type, blink_md::models::block::BlockType::Heading1 { .. })));
    }

    #[test]
    fn test_parse_bullet_list() {
        let md = "- Item 1\n- Item 2\n- Item 3";
        let blocks = parse_markdown(md);
        
        assert_eq!(blocks.len(), 3);
    }

    #[test]
    fn test_parse_todo_list() {
        let md = "- [x] Done task\n- [ ] Pending task";
        let blocks = parse_markdown(md);
        
        assert_eq!(blocks.len(), 2);
    }

    #[test]
    fn test_roundtrip_markdown() {
        let md = "Hello **world**!";
        let blocks = parse_markdown(md);
        
        // Convert back to markdown
        let output: String = blocks.iter()
            .map(|b| b.to_markdown(0))
            .collect();
        
        assert!(output.contains("world"));
    }
}