use crate::client::NotionClient;
use crate::error::Result;
use crate::models::common::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Webhook {
    pub id: ObjectId,
    pub url: String,
    pub events: Vec<String>,
    pub active: bool,
}

#[derive(Serialize)]
struct CreateWebhookRequest {
    url: String,
    events: Vec<String>,
}

impl NotionClient {
    pub async fn list_webhooks(&self) -> Result<Vec<Webhook>> {
        #[derive(Deserialize)]
        struct ListResponse {
            results: Vec<Webhook>,
        }
        let resp: ListResponse = self
            .request(reqwest::Method::GET, "/webhooks", None::<&()>)
            .await?;
        Ok(resp.results)
    }

    pub async fn create_webhook(&self, url: String, events: Vec<String>) -> Result<Webhook> {
        let body = CreateWebhookRequest { url, events };
        self.request(reqwest::Method::POST, "/webhooks", Some(&body))
            .await
    }

    pub async fn delete_webhook(&self, webhook_id: &str) -> Result<Webhook> {
        let path = format!("/webhooks/{}", webhook_id);
        self.request(reqwest::Method::DELETE, &path, None::<&()>)
            .await
    }
}
