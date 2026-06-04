use chrono::Utc;
use blink_md::api::markdown::ToMarkdown;
use blink_md::models::block::{Block, BlockType, HeadingContent, TextBlockContent};
use blink_md::models::common::{
    Annotations, Link, PersonInfo, RichText, TextContent, User, UserType,
};

#[test]
fn test_heading_1_to_markdown() {
    let rich_text = vec![RichText::Text {
        text: TextContent {
            content: "Hello World".to_string(),
            link: None,
        },
        annotations: None,
        plain_text: None,
        href: None,
    }];

    let block = Block {
        object: "block".to_string(),
        id: "id".to_string(),
        created_time: Utc::now(),
        last_edited_time: Utc::now(),
        created_by: mock_user(),
        last_edited_by: mock_user(),
        has_children: false,
        in_trash: false,
        parent: None,
        block_type: BlockType::Heading1 {
            heading_1: HeadingContent {
                rich_text,
                color: "default".to_string(),
                is_toggleable: false,
                children: None,
            },
        },
    };

    assert_eq!(block.to_markdown(0), "# Hello World");
}

fn mock_user() -> User {
    User {
        object: "user".to_string(),
        id: "user_id".to_string(),
        user_type: UserType::Person {
            person: PersonInfo { email: None },
        },
        name: Some("Mock User".to_string()),
        avatar_url: None,
    }
}

#[test]
fn test_complex_rich_text_to_markdown() {
    let rich_text = vec![
        RichText::Text {
            text: TextContent {
                content: "Bold ".to_string(),
                link: None,
            },
            annotations: Some(Annotations {
                bold: true,
                italic: false,
                strikethrough: false,
                underline: false,
                code: false,
                color: "default".to_string(),
            }),
            plain_text: None,
            href: None,
        },
        RichText::Text {
            text: TextContent {
                content: "Italic Link".to_string(),
                link: Some(Link {
                    url: "https://example.com".to_string(),
                }),
            },
            annotations: Some(Annotations {
                bold: false,
                italic: true,
                strikethrough: false,
                underline: false,
                code: false,
                color: "red".to_string(),
            }),
            plain_text: None,
            href: None,
        },
    ];

    let rendered = rich_text.to_markdown(0);
    println!("Rendered: {}", rendered);
    assert!(rendered.contains("**Bold **"));
    assert!(rendered.contains("https://example.com"));
    assert!(rendered.contains("<span color=\"red\">"));
}

#[test]
fn test_callout_with_children() {
    let rich_text = vec![RichText::Text {
        text: TextContent {
            content: "Callout Title".to_string(),
            link: None,
        },
        annotations: None,
        plain_text: None,
        href: None,
    }];

    let child = Block {
        object: "block".to_string(),
        id: "child_id".to_string(),
        created_time: Utc::now(),
        last_edited_time: Utc::now(),
        created_by: mock_user(),
        last_edited_by: mock_user(),
        has_children: false,
        in_trash: false,
        parent: None,
        block_type: BlockType::Paragraph {
            paragraph: TextBlockContent {
                rich_text: vec![RichText::Text {
                    text: TextContent {
                        content: "Child Paragraph".to_string(),
                        link: None,
                    },
                    annotations: None,
                    plain_text: None,
                    href: None,
                }],
                color: "default".to_string(),
                children: None,
            },
        },
    };

    let callout = Block {
        object: "block".to_string(),
        id: "callout_id".to_string(),
        created_time: Utc::now(),
        last_edited_time: Utc::now(),
        created_by: mock_user(),
        last_edited_by: mock_user(),
        has_children: true,
        in_trash: false,
        parent: None,
        block_type: BlockType::Callout {
            callout: blink_md::models::block::CalloutContent {
                rich_text,
                icon: Some(blink_md::models::common::Icon::Emoji {
                    emoji: "💡".to_string(),
                }),
                color: "blue_background".to_string(),
                children: Some(vec![child]),
            },
        },
    };

    let rendered = callout.to_markdown(0);
    println!("Rendered Callout:\n{}", rendered);

    assert!(rendered.contains("<callout icon=\"💡\" color=\"blue_background\">"));
    assert!(rendered.contains("\tCallout Title"));
    assert!(rendered.contains("\tChild Paragraph"));
    assert!(rendered.contains("</callout>"));
}
