//! Errors returned by the Notion API SDK.

use thiserror::Error;

/// The main error type for the Notion API SDK.
#[derive(Error, Debug)]
pub enum NotionError {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// Notion API error response.
    #[error("Notion API error ({code}): {message}")]
    Api {
        /// The error code returned by Notion.
        code: String,
        /// The error message returned by Notion.
        message: String,
        /// The HTTP status code.
        status: u16,
    },

    /// IO error (e.g. file upload).
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON deserialization error.
    #[error("Deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    /// A required field was missing.
    #[error("Missing field: {0}")]
    MissingField(&'static str),

    /// Unauthorized — check your API token.
    #[error("Unauthorized")]
    Unauthorized,

    /// Resource not found.
    #[error("Not found")]
    NotFound,

    /// The requested operation is not supported for this resource type.
    #[error("Unsupported operation: {0}")]
    Unsupported(&'static str),

    /// Auto-pagination hit its safety cap before the server stopped reporting
    /// more pages, so the result set would otherwise be silently truncated.
    #[error(
        "pagination exceeded the {limit}-page safety limit; \
         narrow the query/filter or paginate manually with `search`"
    )]
    PaginationLimitExceeded {
        /// The page cap that was reached.
        limit: usize,
    },
}

/// A convenience type alias for `Result<T, NotionError>`.
pub type Result<T> = std::result::Result<T, NotionError>;
