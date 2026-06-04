//! Schema-driven synchronization utilities for Notion.

pub mod builder;
pub mod error;
pub mod json_schema;
pub mod schema;

pub use builder::PropertyBuilder;
pub use error::{Result, SyncError};
pub use json_schema::JsonSchema;
pub use schema::{DatabaseSchema, PropertySchema};
