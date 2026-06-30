//! Phase E — export a Notion page to a Markdown file with a YAML frontmatter
//! header.
//!
//! This is the inverse of [`crate::cli::sync_cmd`]: it fetches a page and its
//! blocks, maps the page's Notion properties back into typed
//! [`PropertyValue`]s, and renders the result through
//! [`MarkdownWithFrontmatterConverter`] so the on-disk file carries a typed
//! `---` YAML header followed by the Markdown body.
//!
//! The output file is named `<slug>-<page-id>.md`, where `<slug>` is derived
//! from the page title, so re-exporting a page is idempotent and two pages
//! that share a title never collide.

use anyhow::{anyhow, Result};
use blink_md::converter::markdown_frontmatter::MarkdownWithFrontmatterConverter;
use blink_md::converter::notion::{NotionFromPlatform, PageWithBlocks};
use blink_md::ir::inline::text;
use blink_md::ir::metadata::{DateValue, PropertyValue, SelectOption};
use blink_md::{FromPlatform, NotionClient, ToPlatform};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Export a single Notion page to `<out_dir>/<slug>-<page-id>.md`.
///
/// Returns the path that was written.
pub async fn export_page_to_md(
    client: &NotionClient,
    page_id: &str,
    out_dir: &Path,
) -> Result<PathBuf> {
    let page = client.get_page(page_id).await?;
    let blocks = client.get_block_children_recursive(page_id).await?;

    // Capture the typed properties before the page is moved into the IR
    // conversion; `NotionFromPlatform` stores them only as opaque `Custom`
    // values, so we map them ourselves to preserve the YAML round-trip.
    let typed_properties = notion_properties_to_ir(&page.properties);
    let slug = slugify(&page.title_from_properties());

    let input = PageWithBlocks { page, blocks };
    let mut doc = NotionFromPlatform::from_platform(input)
        .map_err(|e| anyhow!("converting page {} to IR: {}", page_id, e))?;
    doc.metadata.properties = typed_properties;

    let markdown = MarkdownWithFrontmatterConverter::to_platform(&doc)
        .map_err(|e| anyhow!("rendering page {} to Markdown: {}", page_id, e))?;

    tokio::fs::create_dir_all(out_dir).await?;
    let file_name = format!("{}-{}.md", slug, page_id);
    let out_path = out_dir.join(file_name);
    tokio::fs::write(&out_path, markdown).await?;
    Ok(out_path)
}

/// Map a Notion page's `properties` object into typed [`PropertyValue`]s.
///
/// Only the property kinds with a frontmatter wire format (see
/// [`crate::ir::frontmatter`]) are mapped to dedicated variants; anything
/// else is preserved verbatim as a [`PropertyValue::Custom`] so no data is
/// lost on export.
fn notion_properties_to_ir(properties: &Value) -> HashMap<String, PropertyValue> {
    let mut out = HashMap::new();
    let Some(obj) = properties.as_object() else {
        return out;
    };
    for (name, value) in obj {
        out.insert(name.clone(), notion_property_to_ir(name, value));
    }
    out
}

fn notion_property_to_ir(name: &str, value: &Value) -> PropertyValue {
    let kind = value.get("type").and_then(Value::as_str);
    match kind {
        Some("title") => PropertyValue::Title {
            title: vec![text(plain_text(value.get("title")))],
        },
        Some("rich_text") => PropertyValue::RichText {
            rich_text: vec![text(plain_text(value.get("rich_text")))],
        },
        Some("number") => PropertyValue::Number {
            number: value.get("number").and_then(Value::as_f64),
        },
        Some("select") => PropertyValue::Select {
            select: value
                .get("select")
                .and_then(|s| s.get("name"))
                .and_then(Value::as_str)
                .map(|n| SelectOption {
                    id: None,
                    name: n.to_string(),
                    color: None,
                }),
        },
        Some("multi_select") => PropertyValue::MultiSelect {
            multi_select: value
                .get("multi_select")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(|o| o.get("name").and_then(Value::as_str))
                        .map(|n| SelectOption {
                            id: None,
                            name: n.to_string(),
                            color: None,
                        })
                        .collect()
                })
                .unwrap_or_default(),
        },
        Some("date") => PropertyValue::Date {
            date: value
                .get("date")
                .and_then(|d| d.get("start"))
                .and_then(Value::as_str)
                .map(|start| DateValue {
                    start: start.to_string(),
                    end: value
                        .get("date")
                        .and_then(|d| d.get("end"))
                        .and_then(Value::as_str)
                        .map(str::to_string),
                    time_zone: None,
                }),
        },
        Some("checkbox") => PropertyValue::Checkbox {
            checkbox: value
                .get("checkbox")
                .and_then(Value::as_bool)
                .unwrap_or(false),
        },
        Some("url") => PropertyValue::Url {
            url: value.get("url").and_then(Value::as_str).map(str::to_string),
        },
        Some("email") => PropertyValue::Email {
            email: value
                .get("email")
                .and_then(Value::as_str)
                .map(str::to_string),
        },
        // Unmapped property kinds round-trip as opaque custom values.
        _ => PropertyValue::Custom {
            key: name.to_string(),
            value: value.clone(),
        },
    }
}

/// Concatenate the `plain_text` of a Notion rich-text array into a plain
/// string. Returns an empty string when the field is missing or empty.
fn plain_text(rich_text: Option<&Value>) -> String {
    rich_text
        .and_then(Value::as_array)
        .map(|arr| {
            arr.iter()
                .filter_map(|rt| rt.get("plain_text").and_then(Value::as_str))
                .collect::<String>()
        })
        .unwrap_or_default()
}

/// Turn a page title into a filesystem-friendly slug: lowercase, with runs of
/// non-alphanumeric characters collapsed to a single `-` and leading/trailing
/// dashes trimmed. Falls back to `untitled` for an empty result.
fn slugify(title: &str) -> String {
    let mut slug = String::with_capacity(title.len());
    let mut prev_dash = false;
    for ch in title.chars() {
        if ch.is_alphanumeric() {
            for lc in ch.to_lowercase() {
                slug.push(lc);
            }
            prev_dash = false;
        } else if !prev_dash {
            slug.push('-');
            prev_dash = true;
        }
    }
    let trimmed = slug.trim_matches('-');
    if trimmed.is_empty() {
        "untitled".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn slugify_basic() {
        assert_eq!(slugify("Hello, World!"), "hello-world");
        assert_eq!(slugify("  Spaced  Out  "), "spaced-out");
        assert_eq!(slugify("***"), "untitled");
        assert_eq!(slugify(""), "untitled");
    }

    #[test]
    fn maps_common_property_types() {
        let props = json!({
            "Name": { "type": "title", "title": [ { "plain_text": "Hi" } ] },
            "Score": { "type": "number", "number": 42.0 },
            "Tags": { "type": "multi_select", "multi_select": [ { "name": "a" }, { "name": "b" } ] },
            "Done": { "type": "checkbox", "checkbox": true },
            "Link": { "type": "url", "url": "https://example.com" }
        });
        let mapped = notion_properties_to_ir(&props);

        assert!(matches!(
            mapped.get("Name"),
            Some(PropertyValue::Title { .. })
        ));
        assert!(matches!(
            mapped.get("Score"),
            Some(PropertyValue::Number { number: Some(n) }) if (*n - 42.0).abs() < f64::EPSILON
        ));
        match mapped.get("Tags") {
            Some(PropertyValue::MultiSelect { multi_select }) => {
                assert_eq!(multi_select.len(), 2);
                assert_eq!(multi_select[0].name, "a");
            }
            other => panic!("expected MultiSelect, got {:?}", other),
        }
        assert!(matches!(
            mapped.get("Done"),
            Some(PropertyValue::Checkbox { checkbox: true })
        ));
        assert!(matches!(
            mapped.get("Link"),
            Some(PropertyValue::Url { url: Some(_) })
        ));
    }

    #[test]
    fn unknown_type_falls_back_to_custom() {
        let props = json!({
            "People": { "type": "people", "people": [ { "id": "u1" } ] }
        });
        let mapped = notion_properties_to_ir(&props);
        assert!(matches!(
            mapped.get("People"),
            Some(PropertyValue::Custom { .. })
        ));
    }
}
