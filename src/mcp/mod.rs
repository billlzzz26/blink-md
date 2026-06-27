//! Unified Model Context Protocol (MCP) server for blink-md.
//!
//! This module is only compiled with the `mcp` feature. It bundles every
//! document tool (Notion, Markdown, Lark Sheets, Mermaid) into a single
//! server binary, `blink-md-mcp`, rather than one binary per platform.

pub mod core;
pub mod server;
pub mod tools;
