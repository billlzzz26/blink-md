//! JSON Schema Builder for Tool input/output definitions.

use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct JsonSchema {
    schema: Value,
}

impl JsonSchema {
    /// Start building a new JSON object schema.
    pub fn object() -> Self {
        Self {
            schema: json!({
                "type": "object",
                "properties": {},
                "required": [],
                "additionalProperties": false
            }),
        }
    }

    pub fn string(self, name: &str) -> Self {
        self.add_property(name, json!({"type": "string"}))
    }

    pub fn number(self, name: &str) -> Self {
        self.add_property(name, json!({"type": "number"}))
    }

    pub fn integer(self, name: &str) -> Self {
        self.add_property(name, json!({"type": "integer"}))
    }

    pub fn boolean(self, name: &str) -> Self {
        self.add_property(name, json!({"type": "boolean"}))
    }

    pub fn array(self, name: &str, items: Value) -> Self {
        self.add_property(name, json!({"type": "array", "items": items}))
    }

    pub fn datetime(self, name: &str) -> Self {
        self.add_property(name, json!({"type": "string", "format": "date-time"}))
    }

    pub fn email(self, name: &str) -> Self {
        self.add_property(name, json!({"type": "string", "format": "email"}))
    }

    pub fn ipv4(self, name: &str) -> Self {
        self.add_property(name, json!({"type": "string", "format": "ipv4"}))
    }

    pub fn uuid(self, name: &str) -> Self {
        self.add_property(name, json!({"type": "string", "format": "uuid"}))
    }

    pub fn nullable(self, name: &str, value_schema: Value) -> Self {
        let schema = json!({
            "anyOf": [
                value_schema,
                {"type": "null"}
            ]
        });
        self.add_property(name, schema)
    }

    fn add_property(mut self, name: &str, prop_schema: Value) -> Self {
        self.schema["properties"][name] = prop_schema;
        if let Some(required) = self.schema["required"].as_array_mut() {
            required.push(json!(name));
        }
        self
    }

    /// Returns the final JSON schema as a [`Value`].
    pub fn build(self) -> Value {
        self.schema
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_schema_builder() {
        let schema = JsonSchema::object().string("name").number("age").build();

        assert_eq!(schema["type"], "object");
        assert!(schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("name")));
        assert!(schema["required"]
            .as_array()
            .unwrap()
            .contains(&json!("age")));
        assert_eq!(schema["properties"]["name"]["type"], "string");
    }
}
