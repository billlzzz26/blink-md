use crate::client::NotionClient;
use crate::error::Result;
use crate::models::common::FileBlockContent;
use serde::Deserialize;

#[derive(Deserialize)]
struct UploadResponse {
    file: FileBlockContent,
}

impl NotionClient {
    /// Upload a file to Notion (used for image, video, etc.)
    /// file_path: path on disk
    /// returns the FileBlockContent which can be used in block creation
    pub async fn upload_file(&self, file_path: &str) -> Result<FileBlockContent> {
        use std::path::Path;
        let file_bytes = tokio::fs::read(file_path).await?;
        let file_name = Path::new(file_path)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("file");

        let part = reqwest::multipart::Part::bytes(file_bytes).file_name(file_name.to_string());

        let form = reqwest::multipart::Form::new().part("file", part);

        let url = format!("{}/files", self.base_url());
        let response = self.http().post(&url).multipart(form).send().await?;

        if response.status().is_success() {
            let upload_resp: UploadResponse = response.json().await?;
            Ok(upload_resp.file)
        } else {
            let status_code = response.status().as_u16();
            let error_msg = response.text().await.unwrap_or_default();
            Err(crate::error::NotionError::Api {
                code: "upload_error".into(),
                message: error_msg,
                status: status_code,
            })
        }
    }
}
