use crate::client::NotionClient;
use crate::error::Result;
use crate::models::common::{ObjectId, RichText, User};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub object: String,
    pub id: ObjectId,
    pub parent: CommentParent,
    pub discussion_id: ObjectId,
    pub rich_text: Vec<RichText>,
    pub created_time: DateTime<Utc>,
    pub last_edited_time: DateTime<Utc>,
    pub created_by: User,
    pub last_edited_by: User,
    pub resolved: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum CommentParent {
    #[serde(rename = "page_id")]
    PageId { page_id: ObjectId },
    #[serde(rename = "block_id")]
    BlockId { block_id: ObjectId },
}

#[derive(Serialize)]
struct CreateCommentRequest {
    parent: CommentParent,
    rich_text: Vec<RichText>,
}

#[derive(Serialize)]
struct ListCommentsQuery {
    block_id: ObjectId,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<u32>,
}

impl NotionClient {
    pub async fn create_comment(
        &self,
        parent: CommentParent,
        rich_text: Vec<RichText>,
    ) -> Result<Comment> {
        let body = CreateCommentRequest { parent, rich_text };
        self.request(reqwest::Method::POST, "/comments", Some(&body))
            .await
    }

    pub async fn list_comments(&self, block_id: &str) -> Result<Vec<Comment>> {
        let query = ListCommentsQuery {
            block_id: block_id.to_string(),
            start_cursor: None,
            page_size: None,
        };
        #[derive(Deserialize)]
        struct ListResponse {
            results: Vec<Comment>,
        }
        let resp: ListResponse = self
            .request(reqwest::Method::GET, "/comments", Some(&query))
            .await?;
        Ok(resp.results)
    }
}
