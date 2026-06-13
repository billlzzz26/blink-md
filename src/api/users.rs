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
        self.collect_all(|cursor| async move {
            let mut path = "/users".to_string();
            if let Some(c) = cursor {
                path = format!("{}?start_cursor={}", path, c);
            }
            self.request(reqwest::Method::GET, &path, None::<&()>).await
        })
        .await
    }
}
