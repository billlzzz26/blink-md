use crate::client::NotionClient;
use crate::error::Result;
use crate::models::database::Database;
use crate::models::datasource::DataSource;
use serde::Serialize;

#[derive(Serialize)]
struct QueryBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sorts: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<u32>,
}

#[derive(Serialize)]
struct CreateDatabaseRequest {
    parent: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<crate::models::common::Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cover: Option<crate::models::common::FileBlockContent>,
    title: Vec<crate::models::common::RichText>,
    properties: serde_json::Value,
}

#[derive(Serialize)]
struct UpdateDatabaseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Vec<crate::models::common::RichText>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Vec<crate::models::common::RichText>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_trash: Option<bool>,
}

impl NotionClient {
    pub async fn get_database(&self, database_id: &str) -> Result<Database> {
        let path = format!("/databases/{}", database_id);
        self.request(reqwest::Method::GET, &path, None::<&()>).await
    }

    pub async fn create_database(
        &self,
        parent: serde_json::Value,
        title: Vec<crate::models::common::RichText>,
        properties: serde_json::Value,
    ) -> Result<Database> {
        let body = CreateDatabaseRequest {
            parent,
            icon: None,
            cover: None,
            title,
            properties,
        };
        self.request(reqwest::Method::POST, "/databases", Some(&body))
            .await
    }

    pub async fn update_database(
        &self,
        database_id: &str,
        title: Option<Vec<crate::models::common::RichText>>,
        properties: Option<serde_json::Value>,
    ) -> Result<Database> {
        let path = format!("/databases/{}", database_id);
        let body = UpdateDatabaseRequest {
            title,
            description: None,
            properties,
            in_trash: None,
        };
        self.request(reqwest::Method::PATCH, &path, Some(&body))
            .await
    }

    pub async fn query_database(
        &self,
        database_id: &str,
        filter: Option<serde_json::Value>,
        sorts: Option<Vec<serde_json::Value>>,
        start_cursor: Option<String>,
        page_size: Option<u32>,
    ) -> Result<crate::models::common::List<serde_json::Value>> {
        let path = format!("/databases/{}/query", database_id);
        let body = QueryBody {
            filter,
            sorts,
            start_cursor,
            page_size,
        };
        self.request(reqwest::Method::POST, &path, Some(&body))
            .await
    }

    pub async fn query_data_source(
        &self,
        data_source_id: &str,
        filter: Option<serde_json::Value>,
        sorts: Option<Vec<serde_json::Value>>,
        start_cursor: Option<String>,
        page_size: Option<u32>,
    ) -> Result<crate::models::common::List<serde_json::Value>> {
        let path = format!("/data_sources/{}/query", data_source_id);
        let body = QueryBody {
            filter,
            sorts,
            start_cursor,
            page_size,
        };
        self.request(reqwest::Method::POST, &path, Some(&body))
            .await
    }

    pub async fn get_data_source(&self, data_source_id: &str) -> Result<DataSource> {
        let path = format!("/data_sources/{}", data_source_id);
        self.request(reqwest::Method::GET, &path, None::<&()>).await
    }

    pub async fn list_data_sources(&self) -> Result<Vec<DataSource>> {
        self.collect_all(|cursor| async move {
            let mut path = "/data_sources".to_string();
            if let Some(c) = cursor {
                path = format!("{}?start_cursor={}", path, c);
            }
            self.request::<crate::models::common::List<DataSource>>(
                reqwest::Method::GET,
                &path,
                None::<&()>,
            )
            .await
        })
        .await
    }
}
