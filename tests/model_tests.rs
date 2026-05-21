//! Unit tests for common types (User, Owner, Parent, RichText)

use notion_rs::models::common::{
    BotInfo, MentionObject, Owner, Parent, ParentType, PersonInfo, RichText, User, UserType,
};

#[test]
fn test_user_person() {
    let json = r#"{
        "object": "user",
        "id": "user-123",
        "type": "person",
        "person": {
            "email": "test@example.com"
        },
        "name": "Test User",
        "avatar_url": "https://example.com/avatar.png"
    }"#;

    let user: User = serde_json::from_str(json).unwrap();
    assert_eq!(user.id, "user-123");
    assert_eq!(user.name, Some("Test User".to_string()));
    match user.user_type {
        UserType::Person { person } => {
            assert_eq!(person.email, Some("test@example.com".to_string()));
        }
        _ => panic!("Expected Person variant"),
    }
}

#[test]
fn test_user_bot() {
    let json = r#"{
        "object": "user",
        "id": "bot-123",
        "type": "bot",
        "bot": {
            "owner": {
                "type": "workspace",
                "workspace": true
            }
        },
        "name": "Test Bot"
    }"#;

    let user: User = serde_json::from_str(json).unwrap();
    assert_eq!(user.id, "bot-123");
    match user.user_type {
        UserType::Bot { bot } => {
            assert!(bot.owner.is_some());
        }
        _ => panic!("Expected Bot variant"),
    }
}

#[test]
fn test_owner_workspace() {
    let json = r#"{
        "type": "workspace",
        "workspace": true
    }"#;

    let owner: Owner = serde_json::from_str(json).unwrap();
    match owner {
        Owner::Workspace { workspace } => {
            assert!(workspace);
        }
        _ => panic!("Expected Workspace variant"),
    }
}

#[test]
fn test_parent_database() {
    let json = r#"{
        "type": "database_id",
        "database_id": "db-123"
    }"#;

    let parent: Parent = serde_json::from_str(json).unwrap();
    match parent.parent_type {
        ParentType::DatabaseId { database_id } => {
            assert_eq!(database_id, "db-123");
        }
        _ => panic!("Expected DatabaseId variant"),
    }
}

#[test]
fn test_parent_page() {
    let json = r#"{
        "type": "page_id",
        "page_id": "page-456"
    }"#;

    let parent: Parent = serde_json::from_str(json).unwrap();
    match parent.parent_type {
        ParentType::PageId { page_id } => {
            assert_eq!(page_id, "page-456");
        }
        _ => panic!("Expected PageId variant"),
    }
}

#[test]
fn test_rich_text() {
    let json = r#"{
        "type": "text",
        "rich_text": {
            "content": "Hello World",
            "link": null
        }
    }"#;

    let rich_text: RichText = serde_json::from_str(json).unwrap();
    match rich_text {
        RichText::Text { content, .. } => {
            assert_eq!(content, "Hello World");
        }
        _ => panic!("Expected Text variant"),
    }
}
