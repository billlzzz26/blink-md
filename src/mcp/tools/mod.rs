//! Tool handlers exposed by the unified blink-md MCP server.
//!
//! Each submodule groups the tools for one document domain. They were
//! previously separate per-platform MCP server binaries; they now register
//! into a single server (see [`crate::mcp::server`]).

pub mod lark;
pub mod markdown;
pub mod mermaid;
pub mod notion;
pub mod notion_live;
