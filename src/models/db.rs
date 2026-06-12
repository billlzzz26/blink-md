//! Universal Database Models for notion-rs.
//!
//! These models map 1:1 to the SQL schema in `db/schema.sql` and align with
//! the Universal IR (Intermediate Representation) defined in `src/ir/`.

use crate::ir::{Platform, UniversalBlock};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// A relational representation of a platform user.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbUser {
    pub id: Uuid,
    pub platform: Platform,
    pub external_id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A workspace container.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbWorkspace {
    pub id: Uuid,
    pub name: String,
    pub owner_id: Uuid,
    pub platform: Platform,
    pub external_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A relational representation of a Universal Document.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbDocument {
    pub id: Uuid,
    pub workspace_id: Uuid,

    pub title: Option<String>,
    pub author: Option<String>,
    pub source_platform: Platform,
    pub source_id: String,

    pub properties: Value,      // JSONB map of PropertyValue IR
    pub custom_metadata: Value, // JSONB map

    pub created_time: Option<DateTime<Utc>>,
    pub last_edited_time: Option<DateTime<Utc>>,

    pub in_trash: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A named style for a document.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbStyle {
    pub id: Uuid,
    pub document_id: Uuid,
    pub name: String,
    pub style_type: String, // "text", "block", "code", "table"
    pub config: Value,      // JSONB configuration
}

/// A relational representation of a Universal Block.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DbBlock {
    pub id: Uuid,
    pub document_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub sort_order: f64, // LexoRank for UI vertical ordering

    pub block_type: String,
    pub content: Value,          // JSONB payload of UniversalBlock
    pub raw_data: Option<Value>, // Original platform data

    pub has_children: bool,
    pub in_trash: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DbDocument {
    /// Maps a UniversalDocument IR to a DbDocument model.
    pub fn from_ir(id: Uuid, workspace_id: Uuid, ir: &crate::ir::UniversalDocument) -> Self {
        Self {
            id,
            workspace_id,
            title: ir.metadata.title.clone(),
            author: ir.metadata.author.clone(),
            source_platform: ir.metadata.source_platform.unwrap_or(Platform::Notion),
            source_id: ir.metadata.source_id.clone().unwrap_or_default(),
            properties: serde_json::to_value(&ir.metadata.properties).unwrap_or_default(),
            custom_metadata: serde_json::to_value(&ir.metadata.custom).unwrap_or_default(),
            created_time: ir.metadata.created_time,
            last_edited_time: ir.metadata.last_edited_time,
            in_trash: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl DbBlock {
    /// Maps a UniversalBlock IR to a DbBlock model.
    pub fn from_ir(
        id: Uuid,
        doc_id: Uuid,
        parent_id: Option<Uuid>,
        sort_order: f64,
        ir: &UniversalBlock,
    ) -> Self {
        Self {
            id,
            document_id: doc_id,
            parent_id,
            sort_order,
            block_type: format!("{:?}", ir), // Simplified for identification
            content: serde_json::to_value(ir).unwrap_or_default(),
            raw_data: if let UniversalBlock::Raw { data, .. } = ir {
                Some(data.clone())
            } else {
                None
            },
            has_children: false, // Should be determined during tree traversal
            in_trash: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
