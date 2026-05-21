//! # notion-rs
//!
//! An unofficial Notion API SDK for Rust (version 2026-03-11).
//!
//! ## Quick Start
//!
//! ```ignore
//! use notion_rs::NotionClient;
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
pub mod error;
pub mod models;

pub use client::NotionClient;
pub use error::NotionError;
