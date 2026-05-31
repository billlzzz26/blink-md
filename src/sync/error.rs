//! Error types for the sync module.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Schema validation failed: {0}")]
    ValidationError(String),

    #[error("Missing required property: {0}")]
    MissingProperty(String),

    #[error("Invalid value format: {0}")]
    InvalidFormat(String),

    #[error("Notion API error: {0}")]
    Notion(#[from] crate::error::NotionError),

    #[error("Serialization error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, SyncError>;
