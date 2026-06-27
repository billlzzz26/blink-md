//! Notion API endpoints organized by resource type.
//!
//! This module provides convenient methods on [`NotionClient`](crate::NotionClient)
//! for interacting with Notion's API resources.

pub mod blocks;
pub mod comments;
pub mod databases;
pub mod files;
pub mod markdown;
pub mod pages;
pub mod search;
pub mod trash;
pub mod users;
pub mod views;
pub mod webhooks;
