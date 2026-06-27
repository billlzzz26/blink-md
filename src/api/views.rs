use crate::client::NotionClient;
use crate::error::Result;
use crate::models::view::View;

impl NotionClient {
    pub async fn get_view(&self, view_id: &str) -> Result<View> {
        let path = format!("/views/{}", view_id);
        self.request(reqwest::Method::GET, &path, None::<&()>).await
    }

    pub async fn update_view(
        &self,
        view_id: &str,
        name: Option<String>,
        view_type_config: Option<serde_json::Value>,
    ) -> Result<View> {
        let path = format!("/views/{}", view_id);
        let mut body = serde_json::Map::new();
        if let Some(n) = name {
            body.insert("name".to_string(), serde_json::Value::String(n));
        }
        if let Some(config) = view_type_config {
            // Merging flatten view_type fields
            if let Some(obj) = config.as_object() {
                for (k, v) in obj {
                    body.insert(k.clone(), v.clone());
                }
            }
        }
        self.request(reqwest::Method::PATCH, &path, Some(&body))
            .await
    }

    /// Permanently delete a view (views do not support soft-delete).
    ///
    /// Delegates to the unified trash lifecycle in [`crate::api::trash`].
    pub async fn delete_view(&self, view_id: &str) -> Result<serde_json::Value> {
        self.delete_permanently(crate::api::trash::Resource::View, view_id)
            .await
    }
}
