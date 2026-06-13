use crate::client::NotionClient;
use crate::error::Result;
use crate::models::common::FileBlockContent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct CreateFileUploadRequest {
    name: String,
    content_type: String,
}

#[derive(Debug, Deserialize)]
pub struct FileUpload {
    pub id: String,
    pub upload_url: String,
    pub expiry_time: String,
}

impl NotionClient {
    /// Step 1: Create a file upload session
    pub async fn create_file_upload(&self, name: &str, content_type: &str) -> Result<FileUpload> {
        let body = CreateFileUploadRequest {
            name: name.to_string(),
            content_type: content_type.to_string(),
        };
        self.request(reqwest::Method::POST, "/files/upload", Some(&body))
            .await
    }

    /// Step 2: Upload the actual file bytes to the provided URL
    pub async fn upload_file_bytes(&self, upload_url: &str, bytes: Vec<u8>) -> Result<()> {
        let client = reqwest::Client::new();
        client
            .put(upload_url)
            .body(bytes)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    /// Step 3: Complete the file upload
    pub async fn complete_file_upload(&self, file_upload_id: &str) -> Result<FileBlockContent> {
        let path = format!("/files/upload/{}/complete", file_upload_id);
        self.request(reqwest::Method::POST, &path, None::<&()>)
            .await
    }

    /// Convenience method to perform all 3 steps of a file upload
    pub async fn upload_file(
        &self,
        path: &std::path::Path,
        content_type: &str,
    ) -> Result<FileBlockContent> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file")
            .to_string();
        let bytes = tokio::fs::read(path).await?;

        let upload = self.create_file_upload(&name, content_type).await?;
        self.upload_file_bytes(&upload.upload_url, bytes).await?;
        self.complete_file_upload(&upload.id).await
    }
}
