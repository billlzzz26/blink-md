//! Notion Database Schema definitions with property validation.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::models::common::{ObjectId, Parent};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSchema {
    pub title: String,
    pub properties: HashMap<String, PropertySchema>,
}

impl DatabaseSchema {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            properties: HashMap::new(),
        }
    }

    pub fn title(mut self, name: &str) -> Self {
        // Notion databases MUST have exactly one title property.
        if self.properties.values().any(|p| matches!(p, PropertySchema::Title)) {
            panic!("Only one title property allowed per database");
        }
        self.properties.insert(name.to_string(), PropertySchema::Title);
        self
    }

    pub fn rich_text(mut self, name: &str) -> Self {
        self.properties.insert(name.to_string(), PropertySchema::RichText);
        self
    }

    pub fn url(mut self, name: &str) -> Self {
        self.properties.insert(name.to_string(), PropertySchema::Url);
        self
    }

    pub fn email(mut self, name: &str) -> Self {
        self.properties.insert(name.to_string(), PropertySchema::Email);
        self
    }

    pub fn number(mut self, name: &str) -> Self {
        self.properties.insert(name.to_string(), PropertySchema::Number);
        self
    }

    pub fn checkbox(mut self, name: &str) -> Self {
        self.properties.insert(name.to_string(), PropertySchema::Checkbox);
        self
    }

    pub fn date(mut self, name: &str) -> Self {
        self.properties.insert(name.to_string(), PropertySchema::Date);
        self
    }

    pub fn select(mut self, name: &str, options: Vec<String>) -> Self {
        self.properties.insert(name.to_string(), PropertySchema::Select { options });
        self
    }

    pub fn multi_select(mut self, name: &str, options: Vec<String>) -> Self {
        self.properties.insert(name.to_string(), PropertySchema::MultiSelect { options });
        self
    }

    pub fn status(mut self, name: &str, options: Vec<String>) -> Self {
        self.properties.insert(name.to_string(), PropertySchema::Status { options });
        self
    }

    pub fn relation(mut self, name: &str, linked_db_id: ObjectId, two_way: bool) -> Self {
        self.properties.insert(name.to_string(), PropertySchema::Relation { linked_db_id, two_way });
        self
    }

    /// Converts the schema into a Notion API compatible body for creating a database.
    pub fn to_notion_create_db_body(&self, parent: &Parent) -> serde_json::Value {
        let mut properties = serde_json::Map::new();
        for (name, schema) in &self.properties {
            let prop = match schema {
                PropertySchema::Title => json!({ "title": {} }),
                PropertySchema::RichText => json!({ "rich_text": {} }),
                PropertySchema::Number => json!({ "number": { "format": "number" } }),
                PropertySchema::Url => json!({ "url": {} }),
                PropertySchema::Email => json!({ "email": {} }),
                PropertySchema::Checkbox => json!({ "checkbox": {} }),
                PropertySchema::Date => json!({ "date": {} }),
                PropertySchema::Select { options } => {
                    let opts: Vec<serde_json::Value> = options.iter().map(|o| json!({"name": o})).collect();
                    json!({ "select": { "options": opts } })
                },
                PropertySchema::MultiSelect { options } => {
                    let opts: Vec<serde_json::Value> = options.iter().map(|o| json!({"name": o})).collect();
                    json!({ "multi_select": { "options": opts } })
                },
                PropertySchema::Status { options } => {
                    let opts: Vec<serde_json::Value> = options.iter().map(|o| json!({"name": o})).collect();
                    json!({ "status": { "options": opts } })
                },
                PropertySchema::Relation { linked_db_id, .. } => {
                    json!({ "relation": { "database_id": linked_db_id } })
                },
            };
            properties.insert(name.clone(), prop);
        }
        json!({
            "parent": parent,
            "title": [{"type": "text", "text": {"content": self.title}}],
            "properties": properties
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertySchema {
    Title,
    RichText,
    Url,
    Email,
    Number,
    Checkbox,
    Date,
    Select { options: Vec<String> },
    MultiSelect { options: Vec<String> },
    Status { options: Vec<String> },
    Relation { linked_db_id: ObjectId, two_way: bool },
}
