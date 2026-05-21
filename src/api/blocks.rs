use crate::client::NotionClient;
use crate::error::Result;
use crate::models::block::*;
use serde::Serialize;

#[derive(Serialize)]
struct AppendChildrenRequest {
    children: Vec<Block>,
    #[serde(skip_serializing_if = "Option::is_none")]
    position: Option<Position>,
}

impl NotionClient {
    pub async fn get_block_children(&self, block_id: &str) -> Result<Vec<Block>> {
        let path = format!("/blocks/{}/children", block_id);
        #[derive(serde::Deserialize)]
        struct ListResponse {
            results: Vec<Block>,
        }
        let resp: ListResponse = self
            .request(reqwest::Method::GET, &path, None::<&()>)
            .await?;
        Ok(resp.results)
    }

    pub async fn append_block_children(
        &self,
        block_id: &str,
        children: Vec<Block>,
        position: Option<Position>,
    ) -> Result<Vec<Block>> {
        let path = format!("/blocks/{}/children", block_id);
        let body = AppendChildrenRequest { children, position };
        #[derive(serde::Deserialize)]
        struct AppendResponse {
            results: Vec<Block>,
        }
        let resp: AppendResponse = self
            .request(reqwest::Method::PATCH, &path, Some(&body))
            .await?;
        Ok(resp.results)
    }

    pub async fn update_block(&self, block_id: &str, block: &Block) -> Result<Block> {
        let path = format!("/blocks/{}", block_id);
        self.request(reqwest::Method::PATCH, &path, Some(block))
            .await
    }

    pub async fn delete_block(&self, block_id: &str) -> Result<Block> {
        // Trash the block (sets in_trash to true)
        let path = format!("/blocks/{}", block_id);
        let body = serde_json::json!({ "in_trash": true });
        self.request(reqwest::Method::PATCH, &path, Some(&body))
            .await
    }
}
