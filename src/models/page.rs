//! A Notion page resource.

use super::common::{FileBlockContent, Icon, ObjectId, Parent, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A Notion page resource returned by the API.
///
/// Pages live inside databases or at the workspace root and contain
/// structured properties (title, select fields, dates, etc.).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Page {
    /// Always `"page"`.
    pub object: String,
    /// Unique identifier.
    pub id: ObjectId,
    /// When the page was created.
    pub created_time: DateTime<Utc>,
    /// When the page was last edited.
    pub last_edited_time: DateTime<Utc>,
    /// The user who created the page.
    pub created_by: User,
    /// The user who last edited the page.
    pub last_edited_by: User,
    /// The database this page belongs to.
    pub parent: Parent,
    /// Whether the page is in the trash (from `archived` field).
    #[serde(alias = "archived", default)]
    pub in_trash: bool,
    /// Optional page icon (emoji or image).
    pub icon: Option<Icon>,
    /// Optional cover image.
    pub cover: Option<FileBlockContent>,
    /// Page properties as a JSON object (title, select, date, etc.).
    pub properties: serde_json::Value,
    /// Notion URL to the page.
    pub url: String,
    /// Public URL if the page is shared publicly.
    pub public_url: Option<String>,
}

impl Page {
    /// Extracts the title text from the page's `title` property.
    ///
    /// Returns `"Untitled"` when the title field is empty or missing.
    pub fn title_from_properties(&self) -> String {
        self.properties
            .get("title")
            .and_then(|t| t.get("title"))
            .and_then(|t| t.as_array())
            .and_then(|arr| arr.first())
            .and_then(|t| t.get("plain_text"))
            .and_then(|v| v.as_str())
            .unwrap_or("Untitled")
            .to_string()
    }
}

/// Request for creating a new Notion page.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreatePageRequest {
    pub parent: serde_json::Value,
    pub properties: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<crate::models::block::Block>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<FileBlockContent>,
}

/// Request for updating an existing Notion page.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdatePageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_trash: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<FileBlockContent>,
}
