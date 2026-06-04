//! Property Value Builder for constructing type-safe Notion page properties.

use crate::sync::error::{Result, SyncError};
use crate::sync::schema::PropertySchema;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PropertyBuilder {
    schema: HashMap<String, PropertySchema>,
    values: HashMap<String, serde_json::Value>,
}

impl PropertyBuilder {
    /// Creates a new builder based on a provided schema.
    pub fn new(schema: HashMap<String, PropertySchema>) -> Self {
        Self {
            schema,
            values: HashMap::new(),
        }
    }

    pub fn date(mut self, name: &str, value: &str) -> Result<Self> {
        // basic validation format YYYY-MM-DD
        if !value.chars().all(|c| c.is_ascii_digit() || c == '-') || value.len() != 10 {
            return Err(SyncError::InvalidFormat(format!(
                "Date must be YYYY-MM-DD: {}",
                value
            )));
        }
        self.values.insert(
            name.to_string(),
            json!({
                "date": { "start": value }
            }),
        );
        Ok(self)
    }

    pub fn date_time(mut self, name: &str, start: &str, end: Option<&str>) -> Result<Self> {
        if !start.contains('T') {
            return Err(SyncError::InvalidFormat(format!(
                "datetime must be ISO 8601: {}",
                start
            )));
        }
        let mut val = json!({ "start": start });
        if let Some(end) = end {
            val["end"] = json!(end);
        }
        self.values.insert(name.to_string(), json!({ "date": val }));
        Ok(self)
    }

    pub fn number(mut self, name: &str, value: f64) -> Result<Self> {
        if value.is_nan() {
            self.values.insert(name.to_string(), json!(null));
        } else {
            self.values
                .insert(name.to_string(), json!({ "number": value }));
        }
        Ok(self)
    }

    pub fn rich_text(mut self, name: &str, text: &str) -> Self {
        self.values.insert(
            name.to_string(),
            json!({
                "rich_text": [{
                    "type": "text",
                    "text": { "content": text }
                }]
            }),
        );
        self
    }

    pub fn title(mut self, name: &str, text: &str) -> Self {
        self.values.insert(
            name.to_string(),
            json!({
                "title": [{
                    "type": "text",
                    "text": { "content": text }
                }]
            }),
        );
        self
    }

    pub fn relation(mut self, name: &str, page_ids: Vec<String>) -> Self {
        let ids: Vec<serde_json::Value> =
            page_ids.into_iter().map(|id| json!({"id": id})).collect();
        self.values
            .insert(name.to_string(), json!({ "relation": ids }));
        self
    }

    pub fn checkbox(mut self, name: &str, value: bool) -> Self {
        self.values
            .insert(name.to_string(), json!({ "checkbox": value }));
        self
    }

    pub fn select(mut self, name: &str, option: &str) -> Self {
        self.values
            .insert(name.to_string(), json!({ "select": { "name": option } }));
        self
    }

    /// Validates and returns the constructed properties.
    pub fn build(self) -> Result<HashMap<String, serde_json::Value>> {
        for prop_name in self.schema.keys() {
            if !self.values.contains_key(prop_name) {
                // In some cases title might be optional if generated, but usually it's required.
                // We'll treat all schema properties as required for this builder.
                return Err(SyncError::MissingProperty(prop_name.clone()));
            }
        }
        Ok(self.values)
    }
}
