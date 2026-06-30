use crate::client::NotionClient;
use crate::error::Result;
use crate::models::common::ObjectId;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use subtle::ConstantTimeEq;

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

// ─── Incoming webhook deliveries ─────────────────────────────────────────

/// The category of a Notion webhook delivery, derived from the payload's
/// `type` field (e.g. `page.content_updated`).
///
/// New event types Notion may add in the future are preserved verbatim as
/// [`WebhookEventType::Other`] rather than failing to parse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebhookEventType {
    PageCreated,
    PageContentUpdated,
    PagePropertiesUpdated,
    PageMoved,
    PageDeleted,
    PageUndeleted,
    DatabaseCreated,
    DatabaseContentUpdated,
    DatabaseSchemaUpdated,
    DatabaseMoved,
    DatabaseDeleted,
    DatabaseUndeleted,
    CommentCreated,
    CommentUpdated,
    CommentDeleted,
    /// Any event type not enumerated above; holds the raw wire string.
    Other(String),
}

impl From<&str> for WebhookEventType {
    fn from(s: &str) -> Self {
        match s {
            "page.created" => Self::PageCreated,
            "page.content_updated" => Self::PageContentUpdated,
            "page.properties_updated" => Self::PagePropertiesUpdated,
            "page.moved" => Self::PageMoved,
            "page.deleted" => Self::PageDeleted,
            "page.undeleted" => Self::PageUndeleted,
            "database.created" => Self::DatabaseCreated,
            "database.content_updated" => Self::DatabaseContentUpdated,
            "database.schema_updated" => Self::DatabaseSchemaUpdated,
            "database.moved" => Self::DatabaseMoved,
            "database.deleted" => Self::DatabaseDeleted,
            "database.undeleted" => Self::DatabaseUndeleted,
            "comment.created" => Self::CommentCreated,
            "comment.updated" => Self::CommentUpdated,
            "comment.deleted" => Self::CommentDeleted,
            other => Self::Other(other.to_string()),
        }
    }
}

/// The entity (page, database, block, …) a webhook event refers to.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct WebhookEntity {
    pub id: String,
    #[serde(rename = "type")]
    pub entity_type: String,
}

/// A parsed Notion webhook event delivery.
#[derive(Debug, Clone, Deserialize)]
pub struct WebhookEvent {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default)]
    pub workspace_id: Option<String>,
    /// Raw `type` string, e.g. `page.content_updated`.
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(default)]
    pub entity: Option<WebhookEntity>,
    /// Event-specific payload (shape varies by `type`); kept as raw JSON.
    #[serde(default)]
    pub data: serde_json::Value,
}

impl WebhookEvent {
    /// The typed category of this event.
    pub fn kind(&self) -> WebhookEventType {
        WebhookEventType::from(self.event_type.as_str())
    }
}

/// The one-time verification handshake Notion sends when a subscription is
/// first created. Echo the token back (or store it) to confirm the endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct WebhookVerification {
    pub verification_token: String,
}

/// A decoded webhook request body: either the initial verification handshake
/// or a normal event delivery.
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum WebhookPayload {
    /// Subscription verification handshake (`{ "verification_token": "..." }`).
    Verification(WebhookVerification),
    /// A normal event delivery.
    Event(Box<WebhookEvent>),
}

/// Parse a raw webhook request body into a [`WebhookPayload`].
///
/// Verification handshakes are matched first; anything else is treated as an
/// event delivery (which requires a `type` field).
pub fn parse_webhook_payload(body: &[u8]) -> Result<WebhookPayload> {
    Ok(serde_json::from_slice(body)?)
}

/// Verify a Notion webhook signature.
///
/// Notion signs every delivery with HMAC-SHA256 over the **raw request body**,
/// keyed by the subscription's verification token, and sends it in the
/// `X-Notion-Signature` header formatted as `sha256=<hex>`. Returns `true`
/// only when the header is well-formed and matches the recomputed signature.
/// The comparison is constant-time to avoid leaking timing information.
pub fn verify_webhook_signature(
    verification_token: &str,
    body: &[u8],
    signature_header: &str,
) -> bool {
    let Some(expected_hex) = signature_header.strip_prefix("sha256=") else {
        return false;
    };
    let Ok(expected) = hex::decode(expected_hex) else {
        return false;
    };
    // `new_from_slice` only errors for impossible key lengths (HMAC accepts any).
    let Ok(mut mac) = Hmac::<Sha256>::new_from_slice(verification_token.as_bytes()) else {
        return false;
    };
    mac.update(body);
    let computed = mac.finalize().into_bytes();
    computed.as_slice().ct_eq(expected.as_slice()).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// HMAC-SHA256 of `body` keyed by `token`, formatted as Notion's header.
    fn sign(token: &str, body: &[u8]) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(token.as_bytes()).unwrap();
        mac.update(body);
        format!("sha256={}", hex::encode(mac.finalize().into_bytes()))
    }

    #[test]
    fn valid_signature_verifies() {
        let token = "secret_token";
        let body = br#"{"type":"page.created"}"#;
        let header = sign(token, body);
        assert!(verify_webhook_signature(token, body, &header));
    }

    #[test]
    fn tampered_body_fails() {
        let token = "secret_token";
        let header = sign(token, br#"{"type":"page.created"}"#);
        assert!(!verify_webhook_signature(
            token,
            br#"{"type":"page.deleted"}"#,
            &header
        ));
    }

    #[test]
    fn wrong_token_fails() {
        let body = br#"{"type":"page.created"}"#;
        let header = sign("real", body);
        assert!(!verify_webhook_signature("attacker", body, &header));
    }

    #[test]
    fn malformed_header_fails() {
        let token = "t";
        let body = b"{}";
        assert!(!verify_webhook_signature(token, body, "deadbeef")); // no sha256= prefix
        assert!(!verify_webhook_signature(token, body, "sha256=zzzz")); // non-hex
    }

    #[test]
    fn parses_verification_handshake() {
        let body = br#"{"verification_token":"secret_abc"}"#;
        match parse_webhook_payload(body).unwrap() {
            WebhookPayload::Verification(v) => assert_eq!(v.verification_token, "secret_abc"),
            other => panic!("expected verification, got {:?}", other),
        }
    }

    #[test]
    fn parses_event_with_typed_kind() {
        let body = br#"{
            "id":"evt-1",
            "timestamp":"2026-06-30T00:00:00.000Z",
            "workspace_id":"ws-1",
            "type":"page.content_updated",
            "entity":{"id":"page-1","type":"page"},
            "data":{"updated_blocks":[{"id":"b1"}]}
        }"#;
        match parse_webhook_payload(body).unwrap() {
            WebhookPayload::Event(e) => {
                assert_eq!(e.kind(), WebhookEventType::PageContentUpdated);
                assert_eq!(e.entity.as_ref().unwrap().id, "page-1");
                assert_eq!(e.id.as_deref(), Some("evt-1"));
            }
            other => panic!("expected event, got {:?}", other),
        }
    }

    #[test]
    fn unknown_event_type_preserved() {
        let body = br#"{"type":"page.future_thing","entity":{"id":"p","type":"page"}}"#;
        match parse_webhook_payload(body).unwrap() {
            WebhookPayload::Event(e) => {
                assert_eq!(
                    e.kind(),
                    WebhookEventType::Other("page.future_thing".into())
                );
            }
            other => panic!("expected event, got {:?}", other),
        }
    }
}
