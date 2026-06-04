use blink_md::models::block::{BlockIdRef, BlockType, Position, PositionType};
use blink_md::models::page::Page;
use serde_json::json;

#[test]
fn test_in_trash_serialization_and_alias() {
    // Test that "in_trash" is serialized and "archived" is accepted as an alias
    let json_data = json!({
        "object": "page",
        "id": "page_id",
        "created_time": "2026-03-11T12:00:00Z",
        "last_edited_time": "2026-03-11T12:00:00Z",
        "created_by": {"object": "user", "id": "u1", "type": "person", "person": {}},
        "last_edited_by": {"object": "user", "id": "u1", "type": "person", "person": {}},
        "parent": {"type": "workspace", "workspace": true},
        "archived": true, // Using the old field name
        "properties": {},
        "url": "https://notion.so/page"
    });

    let page: Page = serde_json::from_value(json_data).expect("Should deserialize with alias");
    assert!(page.in_trash);

    let serialized = serde_json::to_value(&page).unwrap();
    assert_eq!(serialized["in_trash"], true);
    // Ensure "archived" is NOT in the serialized output (clean 2026 format)
    assert!(serialized.get("archived").is_none());
}

#[test]
fn test_position_object_serialization() {
    // Verify that Position uses the 2026 nested object format
    let pos = Position {
        position_type: PositionType::AfterBlock {
            after_block: BlockIdRef {
                id: "target_id".to_string(),
            },
        },
    };

    let serialized = serde_json::to_value(&pos).unwrap();
    assert_eq!(serialized["type"], "after_block");
    assert_eq!(serialized["after_block"]["id"], "target_id");
}

#[test]
fn test_meeting_notes_rename() {
    // Verify that "meeting_notes" is the primary type and "transcription" is an alias
    let json_old = json!({
        "type": "transcription",
        "transcription": {
            "rich_text": [{"type": "text", "text": {"content": "Hello"}}]
        }
    });

    let block_type: BlockType =
        serde_json::from_value(json_old).expect("Should accept transcription alias");
    match block_type {
        BlockType::MeetingNotes { .. } => {}
        _ => panic!("Expected MeetingNotes"),
    }

    let serialized = serde_json::to_value(&block_type).unwrap();
    assert_eq!(serialized["type"], "meeting_notes");
}
