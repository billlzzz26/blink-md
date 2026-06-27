//! Unified soft-delete (trash) lifecycle for Notion resources.
//!
//! Notion deletes things in two different ways: pages and blocks are
//! *soft-deleted* by toggling an `in_trash` flag (and can be restored), while
//! views and webhooks are *hard-deleted* outright. Historically those paths
//! were scattered across `delete_block`, `update_page`, `delete_view` and
//! `delete_webhook`, each re-implementing the request by hand.
//!
//! This module centralises that lifecycle behind a single [`Resource`] enum so
//! callers can `trash`, `restore`, or `delete_permanently` any resource through
//! one consistent API. The legacy typed helpers still exist and now delegate
//! here.

use crate::client::NotionClient;
use crate::error::{NotionError, Result};
use serde_json::{json, Value};

/// A Notion resource that participates in the trash lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resource {
    /// A page — reversible soft-delete via the `in_trash` flag.
    Page,
    /// A block — reversible soft-delete via the `in_trash` flag.
    Block,
    /// A database view — hard delete only.
    View,
    /// A webhook subscription — hard delete only.
    Webhook,
}

impl Resource {
    /// REST collection segment for this resource.
    fn collection(self) -> &'static str {
        match self {
            Resource::Page => "pages",
            Resource::Block => "blocks",
            Resource::View => "views",
            Resource::Webhook => "webhooks",
        }
    }

    /// Whether this resource supports reversible soft-delete (`in_trash`).
    ///
    /// Views and webhooks can only be permanently deleted.
    pub fn supports_restore(self) -> bool {
        matches!(self, Resource::Page | Resource::Block)
    }

    fn path(self, id: &str) -> String {
        format!("/{}/{}", self.collection(), id)
    }
}

impl NotionClient {
    /// Move a resource to the trash.
    ///
    /// Pages and blocks are soft-deleted (reversible with [`restore`]); views
    /// and webhooks — which the API only supports hard-deleting — are removed
    /// permanently.
    ///
    /// [`restore`]: NotionClient::restore
    pub async fn trash(&self, resource: Resource, id: &str) -> Result<Value> {
        let path = resource.path(id);
        if resource.supports_restore() {
            self.request(
                reqwest::Method::PATCH,
                &path,
                Some(&json!({ "in_trash": true })),
            )
            .await
        } else {
            self.request(reqwest::Method::DELETE, &path, None::<&()>)
                .await
        }
    }

    /// Restore a previously trashed page or block (`in_trash = false`).
    ///
    /// Returns [`NotionError::Unsupported`] for resources that cannot be
    /// restored (views, webhooks).
    pub async fn restore(&self, resource: Resource, id: &str) -> Result<Value> {
        if !resource.supports_restore() {
            return Err(NotionError::Unsupported(
                "this resource type cannot be restored from the trash",
            ));
        }
        self.request(
            reqwest::Method::PATCH,
            &resource.path(id),
            Some(&json!({ "in_trash": false })),
        )
        .await
    }

    /// Permanently delete a resource regardless of type.
    pub async fn delete_permanently(&self, resource: Resource, id: &str) -> Result<Value> {
        self.request(reqwest::Method::DELETE, &resource.path(id), None::<&()>)
            .await
    }
}

/// A model that carries trash state, unifying Notion's legacy `archived` field
/// and the current `in_trash` field behind one accessor.
pub trait Trashable {
    /// Whether the resource is currently in the trash.
    fn is_trashed(&self) -> bool;
}

macro_rules! impl_trashable {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl Trashable for $ty {
                fn is_trashed(&self) -> bool {
                    self.in_trash
                }
            }
        )+
    };
}

impl_trashable!(
    crate::models::page::Page,
    crate::models::block::Block,
    crate::models::database::Database,
    crate::models::datasource::DataSource,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restore_support_matches_resource_kind() {
        assert!(Resource::Page.supports_restore());
        assert!(Resource::Block.supports_restore());
        assert!(!Resource::View.supports_restore());
        assert!(!Resource::Webhook.supports_restore());
    }

    #[test]
    fn paths_are_built_from_collection() {
        assert_eq!(Resource::Page.path("abc"), "/pages/abc");
        assert_eq!(Resource::Webhook.path("xyz"), "/webhooks/xyz");
    }
}
