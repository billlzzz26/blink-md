//! # notion-rs
//!
//! An unofficial Notion API SDK for Rust (version 2026-03-11).
//!
//! ## Quick Start
//!
//! ```ignore
//! use blink_md::NotionClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = NotionClient::new("secret_xxx");
//!     let me = client.get_me().await.unwrap();
//!     println!("{:#?}", me);
//! }
//! ```
//!
//! ## Features
//!
//! - Complete type mapping for Notion API 2026-03-11
//! - Users, Pages, Blocks, Databases, Data Sources, Views
//! - Search, Comments, File Uploads, Webhooks
//! - CLI and TUI applications included

pub mod api;
pub mod client;
pub mod converter;
pub mod error;
pub mod ir;
pub mod models;
pub mod sync;

#[cfg(feature = "mcp")]
pub mod mcp;

pub use client::NotionClient;
pub use converter::{ConverterError, ConverterRegistry, FromPlatform, ToPlatform};
pub use error::{NotionError, Result};
pub use ir::{
    DocumentMetadata, InlineElement, Platform, StyleRef, StyleSheet, TextStyle, UniversalBlock,
    UniversalDocument,
};
