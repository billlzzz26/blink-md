//! Schema-driven synchronization utilities for Notion.

pub mod json_schema;
pub mod schema;
pub mod builder;
pub mod error;

pub use json_schema::JsonSchema;
pub use schema::{DatabaseSchema, PropertySchema};
pub use builder::PropertyBuilder;
pub use error::{SyncError, Result};
