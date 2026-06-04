use anyhow::Result;
use notion_rs::api::markdown::ToMarkdown;
use notion_rs::api::universal_parser::markdown_to_universal;
use notion_rs::models::universal_mapper::universal_to_notion_type;
use std::path::PathBuf;

pub async fn run_convert(
    input: PathBuf,
    output: PathBuf,
    from: Option<String>,
    to: Option<String>,
) -> Result<()> {
    let content = tokio::fs::read_to_string(&input).await.unwrap_or_default();
    let from_fmt = from.unwrap_or_else(|| {
        input
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("markdown")
            .to_string()
    });

    let universal_blocks = match from_fmt.as_str() {
        "markdown" | "md" => markdown_to_universal(&content)?,
        _ => anyhow::bail!("Unsupported source format for universal mapping"),
    };

    let output_str = match to.as_deref() {
        Some("json") | None => serde_json::to_string_pretty(&universal_blocks)?,
        Some("markdown") => {
            // Convert universal back to notion blocks first to reuse ToMarkdown trait
            let mut md = String::new();
            for ub in universal_blocks {
                let nb_type = universal_to_notion_type(&ub);
                // Create a dummy notion block to use to_markdown
                let nb = notion_rs::models::block::Block {
                    object: "block".to_string(),
                    id: "temp".to_string(),
                    created_time: chrono::Utc::now(),
                    last_edited_time: chrono::Utc::now(),
                    created_by: dummy_user(),
                    last_edited_by: dummy_user(),
                    has_children: false,
                    in_trash: false,
                    parent: None,
                    block_type: nb_type,
                };
                md.push_str(&nb.to_markdown(0));
                md.push('\n');
            }
            md
        }
        _ => anyhow::bail!("Unsupported target format"),
    };

    tokio::fs::write(&output, output_str).await?;
    println!("Converted {:?} -> {:?}", input, output);
    Ok(())
}

fn dummy_user() -> notion_rs::models::common::User {
    notion_rs::models::common::User {
        object: "user".to_string(),
        id: "dummy".to_string(),
        name: None,
        avatar_url: None,
        user_type: notion_rs::models::common::UserType::Bot {
            bot: notion_rs::models::common::BotInfo {
                owner: None,
                workspace_name: None,
            },
        },
    }
}
