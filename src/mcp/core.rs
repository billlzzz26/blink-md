//! Shared building blocks for the unified blink-md MCP server.
//!
//! Previously this lived in a separate `mcp-core` workspace crate. It now
//! ships as an in-crate module so the whole project builds as a single crate
//! with one MCP server binary (`blink-md-mcp`) instead of one binary per
//! platform.

use serde_json::{json, Value};

// Re-export the pmcp types the tool handlers need, so individual tool modules
// only have to depend on `crate::mcp::core`.
pub use pmcp::types::capabilities::ServerCapabilities;
pub use pmcp::types::ToolInfo;
pub use pmcp::{Error as McpError, RequestHandlerExtra, Result as McpResult, Server, ToolHandler};

/// Re-export the pmcp crate for direct access to less common types.
pub use pmcp;

// =============================================================================
// Schema Builder
// =============================================================================

/// Fluent builder for a tool's JSON input schema.
#[derive(Default)]
pub struct SchemaBuilder {
    properties: serde_json::Map<String, Value>,
    required: Vec<String>,
}

impl SchemaBuilder {
    /// Create an empty schema builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a required string parameter.
    pub fn param(self, name: impl Into<String>, description: impl Into<String>) -> Self {
        self.insert(name, description, "string", true)
    }

    /// Add an optional string parameter.
    pub fn optional_param(self, name: impl Into<String>, description: impl Into<String>) -> Self {
        self.insert(name, description, "string", false)
    }

    /// Add an optional boolean parameter.
    pub fn optional_bool_param(
        self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.insert(name, description, "boolean", false)
    }

    /// Add an optional array-of-objects parameter.
    pub fn optional_array_param(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.properties.insert(
            name.into(),
            json!({
                "type": "array",
                "items": { "type": "object" },
                "description": description.into(),
            }),
        );
        self
    }

    /// Add a required object parameter.
    pub fn object_param(self, name: impl Into<String>, description: impl Into<String>) -> Self {
        self.insert(name, description, "object", true)
    }

    /// Add a required boolean parameter.
    pub fn bool_param(self, name: impl Into<String>, description: impl Into<String>) -> Self {
        self.insert(name, description, "boolean", true)
    }

    fn insert(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
        ty: &str,
        required: bool,
    ) -> Self {
        let name = name.into();
        self.properties.insert(
            name.clone(),
            json!({ "type": ty, "description": description.into() }),
        );
        if required {
            self.required.push(name);
        }
        self
    }

    /// Finish building and return the JSON schema value.
    pub fn build(self) -> Value {
        json!({
            "type": "object",
            "properties": self.properties,
            "required": self.required,
        })
    }
}

// =============================================================================
// Logging
// =============================================================================

/// Initialise a tracing subscriber for the MCP server, honouring `RUST_LOG`.
pub fn init_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .try_init();
}

/// Convenience helper for building a validation error from any displayable value.
pub fn invalid_args(context: &str, err: impl std::fmt::Display) -> McpError {
    McpError::validation(format!("{context}: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_builder_tracks_required_fields() {
        let schema = SchemaBuilder::new()
            .param("name", "The name")
            .optional_param("description", "Optional description")
            .object_param("data", "Data object")
            .build();

        assert!(schema["properties"]["name"].is_object());
        assert!(schema["properties"]["description"].is_object());
        assert!(schema["properties"]["data"].is_object());
        // name + data are required, description is not.
        assert_eq!(schema["required"].as_array().unwrap().len(), 2);
    }
}
