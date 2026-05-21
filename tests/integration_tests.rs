//! Integration tests for the Notion API modules
//!
//! These tests mock the HTTP layer using `wiremock` to avoid making
//! actual API calls.  If `wiremock` isn't available, the tests compile
//! but are marked `#[ignore]`.

use notion_rs::client::NotionClient;

/// Helper that builds a client pointed at a local wiremock server.
#[allow(dead_code)]
fn mock_client(server: &wiremock::MockServer) -> NotionClient {
    NotionClient::new("test_token").with_base_url(server.uri())
}

// ─── users ───────────────────────────────────────────────────────────

#[cfg(test)]
mod users_tests {
    use super::*;

    #[tokio::test]
    #[ignore = "requires wiremock"]
    async fn get_me_returns_user() {
        let server = wiremock::MockServer::start().await;
        let client = mock_client(&server);

        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/users/me"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(
                r#"{
                        "object":"user",
                        "id":"user-1",
                        "type":"person",
                        "person":{"email":"a@b.com"},
                        "name":"Alice"
                    }"#,
            ))
            .mount(&server)
            .await;

        let user = client.get_me().await.unwrap();
        assert_eq!(user.id, "user-1");
        assert_eq!(user.name, Some("Alice".to_string()));
    }
}

// ─── blocks ──────────────────────────────────────────────────────────

#[cfg(test)]
mod blocks_tests {
    use super::*;

    #[tokio::test]
    #[ignore = "requires wiremock"]
    async fn get_block_children_parses_list() {
        let server = wiremock::MockServer::start().await;
        let client = mock_client(&server);

        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/blocks/page-1/children"))
            .respond_with(
                wiremock::ResponseTemplate::new(200)
                    .set_body_string(r#"{
                        "object":"list",
                        "results":[{
                            "object":"block",
                            "id":"b1",
                            "type":"paragraph",
                            "paragraph":{"rich_text":[],"color":"default"},
                            "has_children":false,
                            "created_time":"2026-01-01T00:00:00.000Z",
                            "last_edited_time":"2026-01-01T00:00:00.000Z",
                            "created_by":{"object":"user","id":"u1","type":"person","person":{"email":"a@b.com"}},
                            "last_edited_by":{"object":"user","id":"u1","type":"person","person":{"email":"a@b.com"}},
                            "parent":{"type":"page_id","page_id":"page-1"},
                            "in_trash":false
                        }]
                    }"#),
            )
            .mount(&server)
            .await;

        let blocks = client.get_block_children("page-1").await.unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].id, "b1");
        assert!(!blocks[0].in_trash);
    }
}

// ─── pages ───────────────────────────────────────────────────────────

#[cfg(test)]
mod pages_tests {
    use super::*;

    #[tokio::test]
    #[ignore = "requires wiremock"]
    async fn get_page_returns_page() {
        let server = wiremock::MockServer::start().await;
        let client = mock_client(&server);

        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/pages/page-1"))
            .respond_with(
                wiremock::ResponseTemplate::new(200)
                    .set_body_string(r#"{
                        "object":"page",
                        "id":"page-1",
                        "created_time":"2026-01-01T00:00:00.000Z",
                        "last_edited_time":"2026-01-01T00:00:00.000Z",
                        "created_by":{"object":"user","id":"u1","type":"person","person":{"email":"a@b.com"}},
                        "last_edited_by":{"object":"user","id":"u1","type":"person","person":{"email":"a@b.com"}},
                        "parent":{"type":"database_id","database_id":"db-1"},
                        "archived":false,
                        "properties":{"title":{"title":[{"type":"text","text":{"content":"Hello"},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"Hello"}]}},
                        "url":"https://notion.so/page-1"
                    }"#),
            )
            .mount(&server)
            .await;

        let page = client.get_page("page-1").await.unwrap();
        assert_eq!(page.id, "page-1");
        assert_eq!(page.title_from_properties(), "Hello");
    }
}

// ─── databases ───────────────────────────────────────────────────────

#[cfg(test)]
mod databases_tests {
    use super::*;

    #[tokio::test]
    #[ignore = "requires wiremock"]
    async fn get_database_returns_database() {
        let server = wiremock::MockServer::start().await;
        let client = mock_client(&server);

        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/databases/db-1"))
            .respond_with(
                wiremock::ResponseTemplate::new(200)
                    .set_body_string(r#"{
                        "object":"database",
                        "id":"db-1",
                        "created_time":"2026-01-01T00:00:00.000Z",
                        "last_edited_time":"2026-01-01T00:00:00.000Z",
                        "created_by":{"object":"user","id":"u1","type":"person","person":{"email":"a@b.com"}},
                        "last_edited_by":{"object":"user","id":"u1","type":"person","person":{"email":"a@b.com"}},
                        "title":[{"type":"text","text":{"content":"Tasks"},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"Tasks"}],
                        "description":[],
                        "icon":null,
                        "cover":null,
                        "properties":{"Name":{"title":{}},"Status":{"select":{"options":[]}}},
                        "parent":{"type":"workspace","workspace":true},
                        "url":"https://notion.so/db-1",
                        "is_inline":false,
                        "in_trash":false
                    }"#),
            )
            .mount(&server)
            .await;

        let db = client.get_database("db-1").await.unwrap();
        assert_eq!(db.id, "db-1");
        assert_eq!(db.title_text(), "Tasks");
    }
}

// ─── search ──────────────────────────────────────────────────────────

#[cfg(test)]
mod search_tests {
    use super::*;

    #[tokio::test]
    #[ignore = "requires wiremock"]
    async fn search_returns_results() {
        let server = wiremock::MockServer::start().await;
        let client = mock_client(&server);

        wiremock::Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path("/search"))
            .respond_with(
                wiremock::ResponseTemplate::new(200)
                    .set_body_string(r#"{
                        "object":"list",
                        "results":[{"object":"page","id":"page-1","type":"page","properties":{"title":{"title":[]}},"created_time":"2026-01-01T00:00:00.000Z","last_edited_time":"2026-01-01T00:00:00.000Z","created_by":{"object":"user","id":"u1","type":"person","person":{}},"last_edited_by":{"object":"user","id":"u1","type":"person","person":{}},"parent":{"type":"workspace","workspace":true},"archived":false,"url":"https://notion.so/page-1"}]
                    }"#),
            )
            .mount(&server)
            .await;

        let results = client
            .search(Some("hello".to_string()), None)
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0]["id"], "page-1");
    }
}

// ─── comments ────────────────────────────────────────────────────────

#[cfg(test)]
mod comments_tests {
    use super::*;

    #[tokio::test]
    #[ignore = "requires wiremock"]
    async fn list_comments_returns_comments() {
        let server = wiremock::MockServer::start().await;
        let client = mock_client(&server);

        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/comments"))
            .respond_with(
                wiremock::ResponseTemplate::new(200)
                    .set_body_string(r#"{
                        "object":"list",
                        "results":[{
                            "object":"comment",
                            "id":"c1",
                            "parent":{"type":"page_id","page_id":"page-1"},
                            "discussion_id":"disc-1",
                            "rich_text":[{"type":"text","text":{"content":"Nice!"},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"Nice!"}],
                            "created_time":"2026-01-01T00:00:00.000Z",
                            "last_edited_time":"2026-01-01T00:00:00.000Z",
                            "created_by":{"object":"user","id":"u1","type":"person","person":{"email":"a@b.com"}},
                            "last_edited_by":{"object":"user","id":"u1","type":"person","person":{"email":"a@b.com"}},
                            "resolved":false
                        }]
                    }"#),
            )
            .mount(&server)
            .await;

        let comments = client.list_comments("block-1").await.unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].id, "c1");
        assert!(!comments[0].resolved);
    }
}

// ─── views ───────────────────────────────────────────────────────────

#[cfg(test)]
mod views_tests {
    use super::*;

    #[tokio::test]
    #[ignore = "requires wiremock"]
    async fn get_view_returns_view() {
        let server = wiremock::MockServer::start().await;
        let client = mock_client(&server);

        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/views/view-1"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(
                r#"{
                        "object":"view",
                        "id":"view-1",
                        "name":"Default view",
                        "type":"table",
                        "table":{}
                    }"#,
            ))
            .mount(&server)
            .await;

        let view = client.get_view("view-1").await.unwrap();
        assert_eq!(view.id, "view-1");
        assert_eq!(view.name, "Default view");
    }
}

// ─── webhooks ────────────────────────────────────────────────────────

#[cfg(test)]
mod webhooks_tests {
    use super::*;

    #[tokio::test]
    #[ignore = "requires wiremock"]
    async fn list_webhooks_returns_empty() {
        let server = wiremock::MockServer::start().await;
        let client = mock_client(&server);

        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/webhooks"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(r#"{"results":[]}"#))
            .mount(&server)
            .await;

        let webhooks = client.list_webhooks().await.unwrap();
        assert!(webhooks.is_empty());
    }
}

// ─── error handling ──────────────────────────────────────────────────

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[tokio::test]
    #[ignore = "requires wiremock"]
    async fn api_error_returns_notion_error() {
        let server = wiremock::MockServer::start().await;
        let client = mock_client(&server);

        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/users/me"))
            .respond_with(wiremock::ResponseTemplate::new(401).set_body_string(
                r#"{
                        "object":"error",
                        "code":"unauthorized",
                        "message":"Invalid API token"
                    }"#,
            ))
            .mount(&server)
            .await;

        let result = client.get_me().await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        let err_msg = err.to_string();
        assert!(
            err_msg.contains("unauthorized") || err_msg.contains("Invalid API token"),
            "Error message should contain context: {err_msg}"
        );
    }
}
