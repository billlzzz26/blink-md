use notion_rs::models::block::{Block, BlockType};
use notion_rs::api::markdown::parse_markdown;

#[test]
fn test_parse_simple_paragraph() {
    let md = "Hello Notion";
    let blocks = parse_markdown(md);
    
    assert_eq!(blocks.len(), 1);
    match &blocks[0].block_type {
        BlockType::Paragraph { paragraph } => {
            assert_eq!(paragraph.rich_text.len(), 1);
            assert_eq!(paragraph.rich_text[0].plain_text(), "Hello Notion");
        }
        _ => panic!("Expected Paragraph block"),
    }
}

#[test]
fn test_parse_heading() {
    let md = "# My Title";
    let blocks = parse_markdown(md);
    
    assert_eq!(blocks.len(), 1);
    match &blocks[0].block_type {
        BlockType::Heading1 { heading_1 } => {
            assert_eq!(heading_1.rich_text.len(), 1);
            assert_eq!(heading_1.rich_text[0].plain_text(), "My Title");
        }
        _ => panic!("Expected Heading1 block, got {:?}", blocks[0].block_type),
    }
}

#[test]
fn test_parse_formatted_paragraph() {
    let md = "This is **bold** and *italic*.";
    let blocks = parse_markdown(md);
    
    assert_eq!(blocks.len(), 1);
    match &blocks[0].block_type {
        BlockType::Paragraph { paragraph } => {
            assert!(paragraph.rich_text.len() >= 3);
            let bold_part = paragraph.rich_text.iter().find(|r| r.plain_text() == "bold").expect("bold part not found");
            if let notion_rs::models::common::RichText::Text { annotations, .. } = bold_part {
                assert!(annotations.as_ref().unwrap().bold);
            }
            let italic_part = paragraph.rich_text.iter().find(|r| r.plain_text() == "italic").expect("italic part not found");
            if let notion_rs::models::common::RichText::Text { annotations, .. } = italic_part {
                assert!(annotations.as_ref().unwrap().italic);
            }
        }
        _ => panic!("Expected Paragraph block"),
    }
}

#[test]
fn test_parse_bulleted_list() {
    let md = "- Item 1\n- Item 2";
    let blocks = parse_markdown(md);
    
    assert_eq!(blocks.len(), 2);
    match &blocks[0].block_type {
        BlockType::BulletedListItem { bulleted_list_item } => {
            assert_eq!(bulleted_list_item.rich_text[0].plain_text(), "Item 1");
        }
        _ => panic!("Expected BulletedListItem"),
    }
}

#[test]
fn test_parse_todo_list() {
    let md = "- [ ] Task 1\n- [x] Task 2";
    let blocks = parse_markdown(md);
    
    assert_eq!(blocks.len(), 2);
    match &blocks[0].block_type {
        BlockType::ToDo { to_do } => {
            assert_eq!(to_do.rich_text[0].plain_text(), "Task 1");
            assert!(!to_do.checked);
        }
        _ => panic!("Expected ToDo block, got {:?}", blocks[0].block_type),
    }
}

#[test]
fn test_parse_divider_and_quote() {
    let md = "---\n> This is a quote";
    let blocks = parse_markdown(md);
    
    assert_eq!(blocks.len(), 2);
    assert!(matches!(blocks[0].block_type, BlockType::Divider { .. }));
    match &blocks[1].block_type {
        BlockType::Quote { quote } => {
            assert_eq!(quote.rich_text[0].plain_text(), "This is a quote");
        }
        _ => panic!("Expected Quote block"),
    }
}

#[test]
fn test_parse_callout() {
    let md = "<callout icon=\"💡\" color=\"blue_background\">Attention!</callout>";
    let blocks = parse_markdown(md);
    
    assert_eq!(blocks.len(), 1);
    match &blocks[0].block_type {
        BlockType::Callout { callout } => {
            assert_eq!(callout.rich_text[0].plain_text(), "Attention!");
            if let Some(notion_rs::models::common::Icon::Emoji { emoji }) = &callout.icon {
                assert_eq!(emoji, "💡");
            } else {
                panic!("Expected emoji icon");
            }
            assert_eq!(callout.color, "blue_background");
        }
        _ => panic!("Expected Callout block, got {:?}", blocks[0].block_type),
    }
}
