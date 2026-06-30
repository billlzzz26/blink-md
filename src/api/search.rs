use crate::client::NotionClient;
use crate::error::Result;
use serde::Serialize;

#[derive(Serialize)]
struct SearchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<u32>,
}

impl NotionClient {
    /// Upper bound on the number of pages [`search_all`](Self::search_all)
    /// will fetch, as a safety valve against a server that never stops
    /// reporting `has_more`. At the 100-results page size this caps a single
    /// call at 100k results.
    pub const SEARCH_ALL_MAX_PAGES: usize = 1_000;

    /// Search for pages and databases in Notion.
    ///
    /// Supports text query, sorting, filtering by object type, and pagination.
    pub async fn search(
        &self,
        query: Option<String>,
        filter: Option<serde_json::Value>,
        sort: Option<serde_json::Value>,
        start_cursor: Option<String>,
        page_size: Option<u32>,
    ) -> Result<crate::models::common::List<serde_json::Value>> {
        let body = SearchRequest {
            query,
            sort,
            filter,
            start_cursor,
            page_size,
        };
        self.request(reqwest::Method::POST, "/search", Some(&body))
            .await
    }

    /// Search and automatically follow pagination, returning every matching
    /// result across all pages.
    ///
    /// Repeatedly calls [`search`](Self::search) with the previous page's
    /// `next_cursor` until `has_more` is `false`, requesting the maximum page
    /// size (100). Use [`search`](Self::search) directly when you need
    /// page-by-page control.
    ///
    /// Pagination is hard-capped at [`Self::SEARCH_ALL_MAX_PAGES`] pages so a
    /// server that keeps reporting `has_more: true` (with a fresh cursor each
    /// time) cannot drive unbounded iteration or memory growth.
    pub async fn search_all(
        &self,
        query: Option<String>,
        filter: Option<serde_json::Value>,
        sort: Option<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>> {
        let mut all = Vec::new();
        let mut cursor: Option<String> = None;
        for _ in 0..Self::SEARCH_ALL_MAX_PAGES {
            let page = self
                .search(
                    query.clone(),
                    filter.clone(),
                    sort.clone(),
                    cursor.take(),
                    Some(100),
                )
                .await?;
            all.extend(page.results);
            if !page.has_more {
                break;
            }
            // Defend against a server that reports `has_more` without a cursor.
            match page.next_cursor {
                Some(next) => cursor = Some(next),
                None => break,
            }
        }
        Ok(all)
    }
}
