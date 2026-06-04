use anyhow::Result;
use notify::{Event, RecursiveMode, Watcher};
use notion_rs::api::markdown::parse_markdown;
use notion_rs::NotionClient;
use serde_json::json;
use std::path::PathBuf;
use tokio::sync::mpsc;

pub async fn start_sync(local_dir: PathBuf, client: NotionClient, notion_db: String) -> Result<()> {
    let (tx, mut rx) = mpsc::channel(100);
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
        if let Ok(event) = res {
            tx.blocking_send(event).ok();
        }
    })?;
    watcher.watch(&local_dir, RecursiveMode::Recursive)?;

    println!("Watching {:?} for changes...", local_dir);

    loop {
        if let Some(event) = rx.recv().await {
            for path in &event.paths {
                if path.extension().is_some_and(|ext| ext == "md") {
                    let content = tokio::fs::read_to_string(&path).await?;
                    let blocks = parse_markdown(&content);

                    // Sync to Notion database as a new page (simplified)
                    let parent = json!({ "database_id": notion_db });

                    let file_name = path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let properties = json!({
                        "Name": {
                            "title": [
                                {
                                    "text": {
                                        "content": file_name
                                    }
                                }
                            ]
                        }
                    });

                    match client.create_page(parent, properties, Some(blocks)).await {
                        Ok(page) => println!("Synced {:?} to Notion page ID: {}", path, page.id),
                        Err(e) => eprintln!("Failed to sync {:?}: {}", path, e),
                    }
                }
            }
        }
    }
}
