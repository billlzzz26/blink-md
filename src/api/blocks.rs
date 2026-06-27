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
    pub async fn get_block_children(
        &self,
        block_id: &str,
        start_cursor: Option<String>,
        page_size: Option<u32>,
    ) -> Result<crate::models::common::List<Block>> {
        let mut path = format!("/blocks/{}/children", block_id);
        let mut params = Vec::new();
        if let Some(cursor) = start_cursor {
            params.push(format!("start_cursor={}", cursor));
        }
        if let Some(size) = page_size {
            params.push(format!("page_size={}", size));
        }
        if !params.is_empty() {
            path.push('?');
            path.push_str(&params.join("&"));
        }
        self.request(reqwest::Method::GET, &path, None::<&()>).await
    }

    pub async fn append_block_children(
        &self,
        block_id: &str,
        children: Vec<Block>,
        position: Option<Position>,
    ) -> Result<crate::models::common::List<Block>> {
        let path = format!("/blocks/{}/children", block_id);
        let body = AppendChildrenRequest { children, position };
        self.request(reqwest::Method::PATCH, &path, Some(&body))
            .await
    }

    pub async fn update_block(&self, block_id: &str, block: &Block) -> Result<Block> {
        let path = format!("/blocks/{}", block_id);
        self.request(reqwest::Method::PATCH, &path, Some(block))
            .await
    }

    /// Soft-delete a block by moving it to the trash (`in_trash = true`).
    ///
    /// Typed counterpart of the unified trash lifecycle in
    /// [`crate::api::trash`]; this delegates to
    /// [`trash`](crate::client::NotionClient::trash) so there is one
    /// source of truth for the request shape. Undo it with
    /// `client.restore(Resource::Block, id)`.
    pub async fn delete_block(&self, block_id: &str) -> Result<Block> {
        let value = self
            .trash(crate::api::trash::Resource::Block, block_id)
            .await?;
        Ok(serde_json::from_value(value)?)
    }

    /// Fetches all children for a block recursively.
    ///
    /// This follows the `has_children` flag and continues fetching until
    /// the entire tree is retrieved.
    pub async fn get_block_children_recursive(&self, block_id: &str) -> Result<Vec<Block>> {
        let mut all_blocks = Vec::new();
        let mut cursor = None;

        loop {
            let list = self.get_block_children(block_id, cursor, None).await?;
            for block in list.results {
                if block.has_children {
                    let children = Box::pin(self.get_block_children_recursive(&block.id)).await?;
                    // Note: Notion API doesn't return children inline,
                    // we might want a different model if we want to preserve the tree in memory.
                    // For now, we just flatten or let the caller handle it.
                    // But if we want to store in DB, we need parent_id references.
                    all_blocks.push(block.clone());
                    all_blocks.extend(children);
                } else {
                    all_blocks.push(block);
                }
            }

            if list.has_more {
                cursor = list.next_cursor;
            } else {
                break;
            }
        }

        Ok(all_blocks)
    }
}
