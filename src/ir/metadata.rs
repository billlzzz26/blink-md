//! Document Metadata

use crate::ir::Platform;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Document-level metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocumentMetadata {
    /// Document title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Document author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<DateTime<Utc>>,
    /// Last edited timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_edited_time: Option<DateTime<Utc>>,
    /// Platform-specific properties (Notion page properties, Google Docs metadata, etc.)
    #[serde(default)]
    pub properties: HashMap<String, PropertyValue>,
    /// Source platform this document came from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_platform: Option<Platform>,
    /// Source identifier (page_id, file_path, doc_id, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// Custom metadata
    #[serde(default)]
    pub custom: HashMap<String, serde_json::Value>,
}

/// Property value types for platform-specific properties
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PropertyValue {
    Title {
        title: Vec<crate::ir::inline::InlineElement>,
    },
    RichText {
        rich_text: Vec<crate::ir::inline::InlineElement>,
    },
    Number {
        number: Option<f64>,
    },
    Select {
        select: Option<SelectOption>,
    },
    MultiSelect {
        multi_select: Vec<SelectOption>,
    },
    Date {
        date: Option<DateValue>,
    },
    Checkbox {
        checkbox: bool,
    },
    Url {
        url: Option<String>,
    },
    Email {
        email: Option<String>,
    },
    PhoneNumber {
        phone_number: Option<String>,
    },
    Relation {
        relation: Vec<String>,
    },
    Files {
        files: Vec<FileValue>,
    },
    CreatedTime {
        created_time: DateTime<Utc>,
    },
    CreatedBy {
        created_by: String,
    },
    LastEditedTime {
        last_edited_time: DateTime<Utc>,
    },
    LastEditedBy {
        last_edited_by: String,
    },
    Formula {
        formula: FormulaValue,
    },
    Rollup {
        rollup: RollupValue,
    },
    UniqueId {
        unique_id: UniqueIdValue,
    },
    Verification {
        verification: VerificationValue,
    },
    Status {
        status: StatusValue,
    },
    Custom {
        key: String,
        value: serde_json::Value,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectOption {
    pub id: Option<String>,
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateValue {
    pub start: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileValue {
    pub name: String,
    #[serde(flatten)]
    pub file_type: FileType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FileType {
    External { external: ExternalFile },
    Uploaded { file: UploadedFile },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalFile {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadedFile {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormulaValue {
    pub value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollupValue {
    pub value_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueIdValue {
    pub prefix: Option<String>,
    pub number: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationValue {
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusValue {
    pub name: String,
    pub color: Option<String>,
}
