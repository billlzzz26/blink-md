use crate::client::NotionClient;
use crate::error::Result;
use crate::models::common::User;

impl NotionClient {
    pub async fn get_me(&self) -> Result<User> {
        self.request(reqwest::Method::GET, "/users/me", None::<&()>)
            .await
    }

    pub async fn get_user(&self, user_id: &str) -> Result<User> {
        let path = format!("/users/{}", user_id);
        self.request(reqwest::Method::GET, &path, None::<&()>).await
    }

    pub async fn list_users(&self) -> Result<Vec<User>> {
        // Notion returns a paginated list with object: "list"
        #[derive(serde::Deserialize)]
        struct ListResponse {
            results: Vec<User>,
            // next_cursor: Option<String>,
            // has_more: bool,
        }
        let resp: ListResponse = self
            .request(reqwest::Method::GET, "/users", None::<&()>)
            .await?;
        Ok(resp.results)
    }
}
