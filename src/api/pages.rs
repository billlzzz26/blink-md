use crate::client::NotionClient;
use crate::error::Result;
use crate::models::page::{CreatePageRequest, Page, UpdatePageRequest};

impl NotionClient {
    pub async fn create_page(
        &self,
        parent: serde_json::Value,
        properties: serde_json::Value,
        children: Option<Vec<crate::models::block::Block>>,
    ) -> Result<Page> {
        let body = CreatePageRequest {
            parent,
            properties,
            children,
            ..Default::default()
        };
        self.request(reqwest::Method::POST, "/pages", Some(&body))
            .await
    }

    pub async fn get_page(&self, page_id: &str) -> Result<Page> {
        let path = format!("/pages/{}", page_id);
        self.request(reqwest::Method::GET, &path, None::<&()>).await
    }

    pub async fn update_page(
        &self,
        page_id: &str,
        properties: Option<serde_json::Value>,
        in_trash: Option<bool>,
    ) -> Result<Page> {
        let path = format!("/pages/{}", page_id);
        let body = UpdatePageRequest {
            properties,
            in_trash,
            ..Default::default()
        };
        self.request(reqwest::Method::PATCH, &path, Some(&body))
            .await
    }

    pub async fn move_page(&self, page_id: &str, parent: serde_json::Value) -> Result<Page> {
        let path = format!("/pages/{}", page_id);
        let body = serde_json::json!({
            "parent": parent
        });
        self.request(reqwest::Method::PATCH, &path, Some(&body))
            .await
    }

    pub async fn duplicate_page(&self, page_id: &str) -> Result<Page> {
        let path = format!("/pages/{}/duplicate", page_id);
        self.request(reqwest::Method::POST, &path, None::<&()>)
            .await
    }

    pub async fn get_page_property(
        &self,
        page_id: &str,
        property_id: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("/pages/{}/properties/{}", page_id, property_id);
        self.request(reqwest::Method::GET, &path, None::<&()>).await
    }
}
