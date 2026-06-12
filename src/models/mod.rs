//! Notion API data models (version 2026-03-11).
//!
//! This module contains structs and enums that map directly to Notion API
//! response types.  All types implement `Serialize` and `Deserialize`
//! so they can be used with `serde_json`.

pub mod block;
pub mod common;
pub mod database;
pub mod datasource;
pub mod db;
pub mod page;
pub mod view;
