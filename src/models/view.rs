//! Notion view configurations for databases.

use serde::{Deserialize, Serialize};

/// A database view resource returned by the API.
///
/// Views define how a database's data is displayed (table, board, gallery, etc.).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct View {
    /// Always `"view"`.
    pub object: String,
    /// Unique identifier.
    pub id: String,
    /// Display name of the view.
    pub name: String,
    /// The view type and its configuration.
    #[serde(flatten)]
    pub view_type: ViewType,
}

/// The type of database view and its configuration.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ViewType {
    /// Table view — rows displayed in columns.
    Table { table: TableConfig },
    /// Board view — cards grouped by a property.
    Board { board: BoardConfig },
    /// Gallery view — card grid with cover images.
    Gallery { gallery: GalleryConfig },
    /// List view — simple vertical list.
    List { list: ListConfig },
    /// Calendar view — events displayed on a calendar.
    Calendar { calendar: CalendarConfig },
    /// Timeline view — Gantt-style horizontal timeline.
    Timeline { timeline: TimelineConfig },
}

/// Configuration for a table view.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TableConfig {
    /// Property visibility and order settings.
    pub properties: serde_json::Value,
}

/// Configuration for a board view.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BoardConfig {
    /// Property to group cards by.
    pub group_by: serde_json::Value,
}

/// Configuration for a gallery view.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GalleryConfig {
    /// Cover image configuration.
    pub cover: serde_json::Value,
}

/// Configuration for a list view.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ListConfig {
    /// Whether to show cover images.
    pub show: Option<bool>,
}

/// Configuration for a calendar view.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CalendarConfig {
    /// Date property to use for calendar display.
    pub date: Option<serde_json::Value>,
}

/// Configuration for a timeline view.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TimelineConfig {
    /// Property to group items by.
    pub group_by: serde_json::Value,
}
