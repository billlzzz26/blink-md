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
    pub async fn search(
        &self,
        query: Option<String>,
        filter: Option<serde_json::Value>,
    ) -> Result<Vec<serde_json::Value>> {
        let body = SearchRequest {
            query,
            sort: None,
            filter,
            start_cursor: None,
            page_size: None,
        };
        #[derive(serde::Deserialize)]
        struct SearchResponse {
            results: Vec<serde_json::Value>,
        }
        let resp: SearchResponse = self
            .request(reqwest::Method::POST, "/search", Some(&body))
            .await?;
        Ok(resp.results)
    }
}
