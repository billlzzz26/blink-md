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
}
