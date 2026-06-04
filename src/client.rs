//! The Notion API HTTP client.

use crate::error::{NotionError, Result};
use governor::{Quota, RateLimiter};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client as HttpClient, Method, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::num::NonZeroU32;
use std::sync::Arc;
use std::time::Duration;
use tokio_retry::Retry;

/// An HTTP client for the Notion API (version 2026-03-11).
#[derive(Clone)]
pub struct NotionClient {
    http: HttpClient,
    token: String,
    base_url: String,
    limiter: Arc<
        RateLimiter<
            governor::state::NotKeyed,
            governor::state::InMemoryState,
            governor::clock::DefaultClock,
        >,
    >,
}

impl NotionClient {
    /// Creates a new Notion API client with the given integration token.
    pub fn new(token: impl Into<String>) -> Self {
        let token = token.into();
        let mut headers = HeaderMap::new();
        let mut auth_value =
            HeaderValue::from_str(&format!("Bearer {}", token)).expect("Invalid token format");
        auth_value.set_sensitive(true);
        headers.insert(AUTHORIZATION, auth_value);
        headers.insert("Notion-Version", HeaderValue::from_static("2026-03-11"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let http = HttpClient::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

        // Notion default limit is 3 requests per second.
        // We set it slightly lower for safety, or let user configure later.
        let quota = Quota::per_second(NonZeroU32::new(3).unwrap());
        let limiter = Arc::new(RateLimiter::direct(quota));

        Self {
            http,
            token,
            base_url: "https://api.notion.com/v1".to_string(),
            limiter,
        }
    }

    /// Overrides the base API URL (useful for testing).
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Performs an authenticated HTTP request with rate limiting and retry logic.
    pub async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&impl Serialize>,
    ) -> Result<T> {
        // Retry strategy: 5s, 7s, 10s as requested
        let strategy = vec![
            Duration::from_secs(5),
            Duration::from_secs(7),
            Duration::from_secs(10),
        ]
        .into_iter();

        Retry::spawn(strategy, || async {
            // Wait for rate limiter slot
            self.limiter.until_ready().await;

            let url = format!("{}{}", self.base_url, path);
            let mut req = self.http.request(method.clone(), &url);

            if let Some(body) = body {
                req = req.json(body);
            }

            let response = req.send().await.map_err(|e| NotionError::Api {
                code: "request_failed".to_string(),
                message: e.to_string(),
                status: 0,
            })?;

            Self::process_response(response).await
        })
        .await
    }

    async fn process_response<T: DeserializeOwned>(response: Response) -> Result<T> {
        let status = response.status();
        if status.is_success() {
            let body = response.json().await?;
            Ok(body)
        } else {
            let code = status.as_u16();
            let error_body: serde_json::Value = response.json().await.unwrap_or_default();
            let message = error_body["message"]
                .as_str()
                .unwrap_or("Unknown error")
                .to_string();
            let api_code = error_body["code"].as_str().unwrap_or("unknown").to_string();
            Err(NotionError::Api {
                code: api_code,
                message,
                status: code,
            })
        }
    }

    /// Returns the API token used by this client.
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Returns the base URL configured for this client.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub(crate) fn http(&self) -> &HttpClient {
        &self.http
    }
}
