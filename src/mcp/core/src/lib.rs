//! Shared utilities for blink-md MCP servers
//!
//! Provides common types and helper functions for building MCP servers
//! using the pmcp SDK.

use serde_json::{json, Value};

// Re-export pmcp types for convenience
pub use pmcp::types::capabilities::ServerCapabilities;
pub use pmcp::{Error as McpError, RequestHandlerExtra, Result as McpResult, Server, ToolHandler};

/// Re-export pmcp crate for direct access to types
pub use pmcp;

// =============================================================================
// Response Builders
// =============================================================================

/// Create a success response with a JSON payload
pub fn success(payload: Value) -> McpResult<Value> {
    Ok(payload)
}

/// Create an error response
pub fn error(message: impl Into<String>) -> McpResult<Value> {
    Err(McpError::validation(message.into()))
}

// =============================================================================
// Schema Builders
// =============================================================================

/// Builder for tool input schemas
pub struct SchemaBuilder {
    properties: serde_json::Map<String, Value>,
    required: Vec<String>,
}

impl SchemaBuilder {
    /// Create a new schema builder
    pub fn new() -> Self {
        Self {
            properties: serde_json::Map::new(),
            required: Vec::new(),
        }
    }

    /// Add a required string parameter
    pub fn param(mut self, name: impl Into<String>, description: impl Into<String>) -> Self {
        let name = name.into();
        self.properties.insert(
            name.clone(),
            json!({
                "type": "string",
                "description": description.into()
            }),
        );
        self.required.push(name);
        self
    }

    /// Add an optional string parameter
    pub fn optional_param(
        mut self,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        self.properties.insert(
            name.into(),
            json!({
                "type": "string",
                "description": description.into()
            }),
        );
        self
    }

    /// Add a required object parameter
    pub fn object_param(mut self, name: impl Into<String>, description: impl Into<String>) -> Self {
        let name = name.into();
        self.properties.insert(
            name.clone(),
            json!({
                "type": "object",
                "description": description.into()
            }),
        );
        self.required.push(name);
        self
    }

    /// Add a required boolean parameter
    pub fn bool_param(mut self, name: impl Into<String>, description: impl Into<String>) -> Self {
        let name = name.into();
        self.properties.insert(
            name.clone(),
            json!({
                "type": "boolean",
                "description": description.into()
            }),
        );
        self.required.push(name);
        self
    }

    /// Build the schema
    pub fn build(self) -> Value {
        json!({
            "type": "object",
            "properties": self.properties,
            "required": self.required
        })
    }
}

impl Default for SchemaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// CLI Command Runner (for Jules-style bridge servers)
// =============================================================================

use std::process::{Command, Stdio};

/// Check if a command is installed
pub fn is_command_installed(cmd: &str) -> bool {
    Command::new(cmd)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Run a CLI command and return the result as JSON
pub fn run_cli_command(bin: &str, args: &[&str]) -> McpResult<Value> {
    // Dry run support via environment variable
    let env_var = format!("{}_DRY_RUN", bin.to_uppercase());
    if std::env::var(&env_var).unwrap_or_default() == "true" {
        return Ok(json!({
            "dry_run": true,
            "command": format!("{} {}", bin, args.join(" "))
        }));
    }

    let output = Command::new(bin).args(args).output();

    match output {
        Ok(result) => {
            if !result.status.success() {
                let err_msg = String::from_utf8_lossy(&result.stderr).trim().to_string();
                if err_msg.is_empty() {
                    return Err(McpError::validation(format!(
                        "{} command failed with no error message",
                        bin
                    )));
                }
                return Err(McpError::validation(err_msg));
            }

            Ok(json!({
                "stdout": String::from_utf8_lossy(&result.stdout).trim()
            }))
        }
        Err(err) => Err(McpError::validation(err.to_string())),
    }
}

// =============================================================================
// Logging Setup
// =============================================================================

/// Initialize tracing subscriber for MCP server logging
pub fn init_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .try_init();
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        let result = success(json!({ "key": "value" }));
        assert!(result.is_ok());
        assert_eq!(result.unwrap()["key"], "value");
    }

    #[test]
    fn test_error_response() {
        let result: McpResult<Value> = error("something went wrong");
        assert!(result.is_err());
    }

    #[test]
    fn test_schema_builder() {
        let schema = SchemaBuilder::new()
            .param("name", "The name")
            .optional_param("description", "Optional description")
            .object_param("data", "Data object")
            .build();

        assert!(schema["properties"]["name"].is_object());
        assert!(schema["properties"]["description"].is_object());
        assert!(schema["properties"]["data"].is_object());
        assert_eq!(schema["required"].as_array().unwrap().len(), 2);
    }
}
