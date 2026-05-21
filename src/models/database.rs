//! A Notion database resource.

use super::common::{FileBlockContent, Icon, ObjectId, RichText, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A Notion database resource returned by the API.
///
/// Databases store collections of pages with typed properties
/// (title, select, multi-select, date, checkbox, etc.).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
    /// Always `"database"`.
    pub object: String,
    /// Unique identifier.
    pub id: ObjectId,
    /// When the database was created.
    pub created_time: DateTime<Utc>,
    /// When the database was last edited.
    pub last_edited_time: DateTime<Utc>,
    /// The user who created the database.
    pub created_by: User,
    /// The user who last edited the database.
    pub last_edited_by: User,
    /// Display title as rich text.
    pub title: Vec<RichText>,
    /// Optional description as rich text.
    pub description: Vec<RichText>,
    /// Optional database icon.
    pub icon: Option<Icon>,
    /// Optional cover image.
    pub cover: Option<FileBlockContent>,
    /// Whether the database is in the trash.
    #[serde(alias = "archived", default)]
    pub in_trash: bool,
    /// Property schema definition.
    pub properties: serde_json::Value,
    /// Parent container (workspace or another page).
    pub parent: super::common::Parent,
    /// Notion URL to the database.
    pub url: String,
    /// Whether the database is displayed inline on a page.
    pub is_inline: bool,
}

impl Database {
    /// Extracts the display title text from the database's `title` field.
    pub fn title_text(&self) -> String {
        self.title
            .iter()
            .map(|r| match r {
                RichText::Text { text, .. } => text.content.clone(),
                _ => String::new(),
            })
            .collect()
    }
}
