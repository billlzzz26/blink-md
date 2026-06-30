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
    /// Extracts the title text from the page's title property.
    ///
    /// Notion database pages store the title under whatever the title column is
    /// named (commonly `Name`), not always a literal `title` key, so this first
    /// tries a `title` key and then falls back to the property whose `type` is
    /// `"title"`. All rich-text segments are concatenated. Returns `"Untitled"`
    /// when no non-empty title is found.
    pub fn title_from_properties(&self) -> String {
        title_from_properties_value(&self.properties)
    }
}

/// Extract the title text from a Notion page `properties` object. Tries a
/// literal `title` key first, then the property whose `type` is `"title"`
/// (e.g. a `Name` column). Returns `"Untitled"` when none is found.
fn title_from_properties_value(properties: &serde_json::Value) -> String {
    fn extract(prop: &serde_json::Value) -> Option<String> {
        let arr = prop.get("title")?.as_array()?;
        let text: String = arr
            .iter()
            .filter_map(|t| t.get("plain_text").and_then(|v| v.as_str()))
            .collect();
        (!text.is_empty()).then_some(text)
    }

    if let Some(title) = properties.get("title").and_then(extract) {
        return title;
    }
    if let Some(obj) = properties.as_object() {
        for prop in obj.values() {
            if prop.get("type").and_then(|t| t.as_str()) == Some("title") {
                if let Some(title) = extract(prop) {
                    return title;
                }
            }
        }
    }
    "Untitled".to_string()
}

#[cfg(test)]
mod tests {
    use super::title_from_properties_value;
    use serde_json::json;

    #[test]
    fn finds_title_under_literal_title_key() {
        let props = json!({ "title": { "title": [{ "plain_text": "Hello" }] } });
        assert_eq!(title_from_properties_value(&props), "Hello");
    }

    #[test]
    fn finds_title_under_named_column() {
        // A database page whose title column is named "Name".
        let props = json!({
            "Name": { "type": "title", "title": [{ "plain_text": "My Page" }] },
            "Tags": { "type": "multi_select", "multi_select": [] }
        });
        assert_eq!(title_from_properties_value(&props), "My Page");
    }

    #[test]
    fn concatenates_rich_text_segments() {
        let props = json!({
            "Name": { "type": "title", "title": [
                { "plain_text": "foo " }, { "plain_text": "bar" }
            ] }
        });
        assert_eq!(title_from_properties_value(&props), "foo bar");
    }

    #[test]
    fn falls_back_to_untitled() {
        let props = json!({ "Tags": { "type": "multi_select", "multi_select": [] } });
        assert_eq!(title_from_properties_value(&props), "Untitled");
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
