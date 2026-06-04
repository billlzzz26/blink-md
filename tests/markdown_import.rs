use blink_md::api::markdown::parse_markdown;
use blink_md::models::block::BlockType;

#[test]
fn test_parse_simple_paragraph() {
    let md = "Hello world";
    let blocks = parse_markdown(md);
    assert_eq!(blocks.len(), 1);
    match &blocks[0].block_type {
        BlockType::Paragraph { paragraph } => {
            assert_eq!(paragraph.rich_text[0].plain_text(), "Hello world");
        }
        _ => panic!("Expected paragraph"),
    }
}

#[test]
fn test_parse_formatted_paragraph() {
    let md = "Hello **bold** and *italic*";
    let blocks = parse_markdown(md);
    assert_eq!(blocks.len(), 1);
    match &blocks[0].block_type {
        BlockType::Paragraph { paragraph } => {
            assert_eq!(paragraph.rich_text.len(), 4);
            assert_eq!(paragraph.rich_text[0].plain_text(), "Hello ");
            assert_eq!(paragraph.rich_text[1].plain_text(), "bold");
            assert_eq!(paragraph.rich_text[2].plain_text(), " and ");
            assert_eq!(paragraph.rich_text[3].plain_text(), "italic");
        }
        _ => panic!("Expected paragraph"),
    }
}

#[test]
fn test_parse_heading() {
    let md = "# Heading 1\n## Heading 2";
    let blocks = parse_markdown(md);
    assert_eq!(blocks.len(), 2);
    match &blocks[0].block_type {
        BlockType::Heading1 { heading_1 } => {
            assert_eq!(heading_1.rich_text[0].plain_text(), "Heading 1");
        }
        _ => panic!("Expected Heading 1"),
    }
    match &blocks[1].block_type {
        BlockType::Heading2 { heading_2 } => {
            assert_eq!(heading_2.rich_text[0].plain_text(), "Heading 2");
        }
        _ => panic!("Expected Heading 2"),
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
        _ => panic!("Expected ToDo"),
    }
    match &blocks[1].block_type {
        BlockType::ToDo { to_do } => {
            assert_eq!(to_do.rich_text[0].plain_text(), "Task 2");
            assert!(to_do.checked);
        }
        _ => panic!("Expected ToDo"),
    }
}

#[test]
fn test_parse_divider_and_quote() {
    let md = "---\n> This is a quote";
    let blocks = parse_markdown(md);
    assert_eq!(blocks.len(), 2);
    match &blocks[0].block_type {
        BlockType::Divider {} => {}
        _ => panic!("Expected Divider"),
    }
    match &blocks[1].block_type {
        BlockType::Quote { quote } => {
            assert_eq!(quote.rich_text[0].plain_text(), "This is a quote");
        }
        _ => panic!("Expected Quote"),
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
            if let Some(blink_md::models::common::Icon::Emoji { emoji }) = &callout.icon {
                assert_eq!(emoji, "💡");
            } else {
                panic!("Expected emoji icon");
            }
            assert_eq!(callout.color, "blue_background");
        }
        _ => panic!("Expected Callout block, got {:?}", blocks[0].block_type),
    }
}

#[test]
fn test_mention_to_markdown() {
    use blink_md::api::markdown::mention_to_markdown;
    use blink_md::models::common::{
        DatabaseMention, MentionObject, PageMention, PersonInfo, User, UserType,
    };

    // User mention
    let user = User {
        object: "user".to_string(),
        id: "user-123".to_string(),
        name: Some("Alice".to_string()),
        avatar_url: None,
        user_type: UserType::Person {
            person: PersonInfo {
                email: Some("alice@example.com".to_string()),
            },
        },
    };
    let mention = MentionObject::User { user };
    let markdown = mention_to_markdown(&mention);
    assert_eq!(
        markdown,
        r#"<mention-user url="https://www.notion.so/user-123">Alice</mention-user>"#
    );

    // Page mention
    let page = PageMention {
        id: "page-456".to_string(),
    };
    let mention = MentionObject::Page { page };
    let markdown = mention_to_markdown(&mention);
    assert_eq!(
        markdown,
        r#"<mention-page url="https://www.notion.so/page-456">Page</mention-page>"#
    );

    // Database mention
    let db = DatabaseMention {
        id: "db-789".to_string(),
    };
    let mention = MentionObject::Database { database: db };
    let markdown = mention_to_markdown(&mention);
    assert_eq!(
        markdown,
        r#"<mention-database url="https://www.notion.so/db-789">Database</mention-database>"#
    );

    // Date mention
    let date = serde_json::json!({ "start": "2024-01-01" });
    let mention = MentionObject::Date { date };
    let markdown = mention_to_markdown(&mention);
    assert_eq!(markdown, r#"<mention-date start="2024-01-01" />"#);
}
