use crate::client::NotionClient;
use crate::error::Result;
use crate::models::page::Page;
use serde::Serialize;

#[derive(Serialize)]
struct UpdatePageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_trash: Option<bool>,
}

#[derive(Serialize)]
struct CreatePageRequest {
    parent: serde_json::Value,
    properties: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<crate::models::block::Block>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<crate::models::common::Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cover: Option<crate::models::common::FileBlockContent>,
}

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
            icon: None,
            cover: None,
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
        };
        self.request(reqwest::Method::PATCH, &path, Some(&body))
            .await
    }

    pub async fn move_page(
        &self,
        page_id: &str,
        parent: serde_json::Value,
    ) -> Result<Page> {
        let path = format!("/pages/{}", page_id);
        let body = serde_json::json!({
            "parent": parent
        });
        self.request(reqwest::Method::PATCH, &path, Some(&body))
            .await
    }

    pub async fn duplicate_page(&self, page_id: &str) -> Result<Page> {
        let path = format!("/pages/{}/duplicate", page_id);
        self.request(reqwest::Method::POST, &path, None::<&()>).await
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
