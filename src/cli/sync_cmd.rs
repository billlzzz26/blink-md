use anyhow::Result;
use blink_md::api::markdown::parse_markdown;
use blink_md::NotionClient;
use notify::{Event, RecursiveMode, Watcher};
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;
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
                    if let Ok(content) = tokio::fs::read_to_string(&path).await {
                        let blocks = parse_markdown(&content);
                        let parent = json!({ "database_id": notion_db });
                        let file_name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
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
}
