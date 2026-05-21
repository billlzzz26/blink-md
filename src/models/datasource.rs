//! A Notion data source resource.

use super::common::{ObjectId, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A data source resource (external data connection in Notion).
///
/// Data sources allow Notion to connect to external data
/// providers like Google Sheets, Jira, etc.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataSource {
    /// Always `"data_source"`.
    pub object: String,
    /// Unique identifier.
    pub id: ObjectId,
    /// When the data source was created.
    pub created_time: DateTime<Utc>,
    /// When the data source was last edited.
    pub last_edited_time: DateTime<Utc>,
    /// The user who created the data source.
    pub created_by: User,
    /// The user who last edited the data source.
    pub last_edited_by: User,
    /// Parent container.
    pub parent: super::common::Parent,
    /// Whether the data source is in the trash.
    #[serde(alias = "archived", default)]
    pub in_trash: bool,
    /// Data source properties/configuration.
    pub properties: serde_json::Value,
    /// Notion URL to the data source.
    pub url: String,
}
