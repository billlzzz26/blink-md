use anyhow::{anyhow, Result};
use blink_md::converter::markdown_frontmatter::MarkdownWithFrontmatterConverter;
use blink_md::converter::notion::NotionToPlatform;
use blink_md::{FromPlatform, NotionClient, ToPlatform};
use notify::{Event, RecursiveMode, Watcher};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::Instant;

pub async fn start_sync(local_dir: PathBuf, client: NotionClient, notion_db: String) -> Result<()> {
    let (tx, mut rx) = mpsc::channel(100);
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
        if let Ok(event) = res {
            tx.blocking_send(event).ok();
        }
    })?;
    watcher.watch(&local_dir, RecursiveMode::Recursive)?;

    println!("Watching {:?} for changes with debouncing...", local_dir);

    let mut pending_syncs: HashMap<PathBuf, Instant> = HashMap::new();
    let debounce_duration = Duration::from_millis(500);

    loop {
        tokio::select! {
            Some(event) = rx.recv() => {
                for path in event.paths {
                    if path.extension().is_some_and(|ext| ext == "md") {
                        pending_syncs.insert(path, Instant::now() + debounce_duration);
                    }
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                let now = Instant::now();
                let mut to_sync = vec![];

                pending_syncs.retain(|path, &mut deadline| {
                    if now >= deadline {
                        to_sync.push(path.clone());
                        false // remove from pending
                    } else {
                        true // keep in pending
                    }
                });

                for path in to_sync {
                    match sync_file(&path, &client, &notion_db).await {
                        Ok(id) => println!("Synced {:?} to Notion page ID: {}", path, id),
                        Err(e) => eprintln!("Failed to sync {:?}: {}", path, e),
                    }
                }
            }
        }
    }
}

/// Read a single `.md` file, parse its YAML frontmatter into Notion page
/// properties, convert the Markdown body into Notion blocks, and create the
/// page inside `notion_db`.
///
/// Frontmatter keys are written as page properties (the column name must match
/// a property in the target database). The Markdown body — with the `---`
/// delimited YAML block stripped — becomes the page content. When the
/// frontmatter does not supply a `title`-typed property, the file stem is used
/// as a `Name` title so the page is never created untitled.
async fn sync_file(path: &Path, client: &NotionClient, notion_db: &str) -> Result<String> {
    let content = tokio::fs::read_to_string(path).await?;

    let doc = MarkdownWithFrontmatterConverter::from_platform(content)
        .map_err(|e| anyhow!("converting {:?} to IR: {}", path, e))?;
    let request = NotionToPlatform::to_platform(&doc)
        .map_err(|e| anyhow!("converting {:?} to Notion: {}", path, e))?;

    let mut properties = request.properties;
    let file_name = path
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    ensure_title(&mut properties, &file_name);

    let parent = json!({ "database_id": notion_db });
    let page = client
        .create_page(parent, properties, request.children)
        .await?;
    Ok(page.id)
}

/// Notion requires every database page to carry exactly one `title` property.
/// If the frontmatter already supplied one, leave the properties untouched;
/// otherwise fall back to the file stem under a `Name` column (the previous
/// default behaviour, preserved for files without frontmatter).
fn ensure_title(properties: &mut Value, file_name: &str) {
    let Some(obj) = properties.as_object_mut() else {
        return;
    };
    let has_title = obj.values().any(|v| v.get("title").is_some());
    if !has_title {
        obj.insert(
            "Name".to_string(),
            json!({ "title": [ { "text": { "content": file_name } } ] }),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_title_adds_name_when_absent() {
        let mut props = json!({ "Score": { "number": 1 } });
        ensure_title(&mut props, "my-note");
        assert_eq!(
            props["Name"]["title"][0]["text"]["content"],
            json!("my-note")
        );
    }

    #[test]
    fn ensure_title_preserves_existing_title() {
        let mut props = json!({
            "title": { "title": [ { "text": { "content": "From Frontmatter" } } ] }
        });
        ensure_title(&mut props, "my-note");
        assert!(props.get("Name").is_none());
        assert_eq!(
            props["title"]["title"][0]["text"]["content"],
            json!("From Frontmatter")
        );
    }
}
