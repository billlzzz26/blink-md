use blink_md::NotionClient;
use clap::{Parser, Subcommand};

mod cli;

#[derive(Parser)]
#[command(name = "notion-rs")]
#[command(about = "Notion API CLI & TUI (2026-03-11)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage users
    Users {
        #[command(subcommand)]
        action: UserAction,
    },
    /// Manage pages
    Pages {
        #[command(subcommand)]
        action: PageAction,
    },
    /// Manage databases
    Databases {
        #[command(subcommand)]
        action: DatabaseAction,
    },
    /// Manage database views
    Views {
        #[command(subcommand)]
        action: ViewAction,
    },
    /// Manage blocks
    Blocks {
        #[command(subcommand)]
        action: BlockAction,
    },
    /// Manage comments
    Comments {
        #[command(subcommand)]
        action: CommentAction,
    },
    /// Search for pages and databases
    Search {
        /// The query to search for
        query: Option<String>,
    },
    /// Launch interactive TUI
    Tui,
    /// Convert between Markdown/HTML/PDF/DOCX and JSON/Markdown
    Convert {
        #[arg(short, long)]
        input: std::path::PathBuf,
        #[arg(short, long)]
        output: std::path::PathBuf,
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
    },
    /// Sync local folder to Notion
    Sync {
        #[arg(short, long)]
        dir: std::path::PathBuf,
        #[arg(long)]
        notion_db: Option<String>,
    },
    /// Show diff between two files
    Diff {
        old: std::path::PathBuf,
        new: std::path::PathBuf,
    },
    /// Start MCP server
    McpServe,
}

#[derive(Subcommand)]
enum UserAction {
    /// Get current bot/user info
    Me,
    /// List all users (workspace members)
    List,
    /// Get user by ID
    Get { user_id: String },
}

#[derive(Subcommand)]
enum PageAction {
    /// List recent pages
    List,
    /// Get page by ID
    Get { page_id: String },
    /// Create a new page
    Create {
        /// Parent ID (page or database)
        parent_id: String,
        /// Parent type (page_id or database_id)
        #[arg(long, default_value = "page_id")]
        parent_type: String,
        /// JSON string for properties
        properties_json: String,
    },
    /// Update page properties
    Update {
        page_id: String,
        /// JSON string for properties
        properties_json: String,
    },
    /// Move page to a different parent
    Move {
        page_id: String,
        /// New parent ID
        parent_id: String,
        /// New parent type (page_id or database_id)
        #[arg(long, default_value = "page_id")]
        parent_type: String,
    },
    /// Duplicate a page
    Duplicate { page_id: String },
    /// Export page to Notion-flavored Markdown
    Export { page_id: String },
}

#[derive(Subcommand)]
enum DatabaseAction {
    /// Get database by ID
    Get { database_id: String },
    /// Create a new database
    Create {
        /// Parent page ID
        parent_id: String,
        /// JSON string for title (RichText array)
        title_json: String,
        /// JSON string for properties schema
        properties_json: String,
    },
    /// Update database title or schema
    Update {
        database_id: String,
        /// Optional JSON string for title
        #[arg(long)]
        title_json: Option<String>,
        /// Optional JSON string for properties
        #[arg(long)]
        properties_json: Option<String>,
    },
}

#[derive(Subcommand)]
enum ViewAction {
    /// Get view by ID
    Get { view_id: String },
    /// Update view settings
    Update {
        view_id: String,
        /// Optional new name
        #[arg(long)]
        name: Option<String>,
        /// Optional JSON string for view configuration (filters, sorts, etc.)
        #[arg(long)]
        config_json: Option<String>,
    },
    /// Delete a view
    Delete { view_id: String },
}

#[derive(Subcommand)]
enum BlockAction {
    /// Get children of a block
    Children { block_id: String },
    /// Append children to a block
    Append {
        block_id: String,
        /// JSON string for blocks array
        children_json: String,
    },
}

#[derive(Subcommand)]
enum CommentAction {
    /// List comments on a block/page
    List { block_id: String },
    /// Create a comment
    Create {
        /// Target block/page ID
        id: String,
        /// Target type (page_id or block_id)
        #[arg(long, default_value = "page_id")]
        target_type: String,
        /// JSON string for comment text (RichText array)
        text_json: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let token = std::env::var("NOTION_TOKEN").map_err(|_| {
        anyhow::anyhow!(
            "NOTION_TOKEN environment variable not set. Please set it to your Notion API token."
        )
    })?;
    let client = NotionClient::new(&token);

    match cli.command {
        Commands::Users { action } => match action {
            UserAction::Me => {
                let user = client.get_me().await?;
                println!("{}", serde_json::to_string_pretty(&user)?);
            }
            UserAction::List => {
                let users = client.list_users().await?;
                println!("{}", serde_json::to_string_pretty(&users)?);
            }
            UserAction::Get { user_id } => {
                let user = client.get_user(&user_id).await?;
                println!("{}", serde_json::to_string_pretty(&user)?);
            }
        },
        Commands::Pages { action } => match action {
            PageAction::List => {
                let results = client
                    .search(
                        None,
                        Some(serde_json::json!({
                            "property": "object",
                            "value": "page"
                        })),
                        None,
                        None,
                        None,
                    )
                    .await?;
                print_search_results(results.results);
            }
            PageAction::Get { page_id } => {
                let page = client.get_page(&page_id).await?;
                println!("{}", serde_json::to_string_pretty(&page)?);
            }
            PageAction::Create {
                parent_id,
                parent_type,
                properties_json,
            } => {
                let parent = serde_json::json!({ parent_type: parent_id });
                let properties = serde_json::from_str(&properties_json)?;
                let page = client.create_page(parent, properties, None).await?;
                println!("{}", serde_json::to_string_pretty(&page)?);
            }
            PageAction::Update {
                page_id,
                properties_json,
            } => {
                let properties = serde_json::from_str(&properties_json)?;
                let page = client.update_page(&page_id, Some(properties), None).await?;
                println!("{}", serde_json::to_string_pretty(&page)?);
            }
            PageAction::Move {
                page_id,
                parent_id,
                parent_type,
            } => {
                let parent = serde_json::json!({ parent_type: parent_id });
                let page = client.move_page(&page_id, parent).await?;
                println!("{}", serde_json::to_string_pretty(&page)?);
            }
            PageAction::Duplicate { page_id } => {
                let page = client.duplicate_page(&page_id).await?;
                println!("{}", serde_json::to_string_pretty(&page)?);
            }
            PageAction::Export { page_id } => {
                use blink_md::api::markdown::ToMarkdown;
                let list = client.get_block_children(&page_id, None, None).await?;
                for block in list.results {
                    println!("{}", block.to_markdown(0));
                }
            }
        },
        Commands::Databases { action } => match action {
            DatabaseAction::Get { database_id } => {
                let db = client.get_database(&database_id).await?;
                println!("{}", serde_json::to_string_pretty(&db)?);
            }
            DatabaseAction::Create {
                parent_id,
                title_json,
                properties_json,
            } => {
                let parent = serde_json::json!({ "page_id": parent_id });
                let title = serde_json::from_str(&title_json)?;
                let properties = serde_json::from_str(&properties_json)?;
                let db = client.create_database(parent, title, properties).await?;
                println!("{}", serde_json::to_string_pretty(&db)?);
            }
            DatabaseAction::Update {
                database_id,
                title_json,
                properties_json,
            } => {
                let title = title_json.map(|t| serde_json::from_str(&t)).transpose()?;
                let properties = properties_json
                    .map(|p| serde_json::from_str(&p))
                    .transpose()?;
                let db = client
                    .update_database(&database_id, title, properties)
                    .await?;
                println!("{}", serde_json::to_string_pretty(&db)?);
            }
        },
        Commands::Views { action } => match action {
            ViewAction::Get { view_id } => {
                let view = client.get_view(&view_id).await?;
                println!("{}", serde_json::to_string_pretty(&view)?);
            }
            ViewAction::Update {
                view_id,
                name,
                config_json,
            } => {
                let config = config_json.map(|c| serde_json::from_str(&c)).transpose()?;
                let view = client.update_view(&view_id, name, config).await?;
                println!("{}", serde_json::to_string_pretty(&view)?);
            }
            ViewAction::Delete { view_id } => {
                let result = client.delete_view(&view_id).await?;
                println!("{}", serde_json::to_string_pretty(&result)?);
            }
        },
        Commands::Blocks { action } => match action {
            BlockAction::Children { block_id } => {
                let list = client.get_block_children(&block_id, None, None).await?;
                println!("{}", serde_json::to_string_pretty(&list.results)?);
            }
            BlockAction::Append {
                block_id,
                children_json,
            } => {
                let children = serde_json::from_str(&children_json)?;
                let list = client
                    .append_block_children(&block_id, children, None)
                    .await?;
                println!("{}", serde_json::to_string_pretty(&list.results)?);
            }
        },
        Commands::Comments { action } => match action {
            CommentAction::List { block_id } => {
                let comments = client.list_comments(&block_id).await?;
                println!("{}", serde_json::to_string_pretty(&comments)?);
            }
            CommentAction::Create {
                id,
                target_type,
                text_json,
            } => {
                use blink_md::api::comments::CommentParent;
                let parent = if target_type == "page_id" {
                    CommentParent::PageId { page_id: id }
                } else {
                    CommentParent::BlockId { block_id: id }
                };
                let rich_text = serde_json::from_str(&text_json)?;
                let comment = client.create_comment(parent, rich_text).await?;
                println!("{}", serde_json::to_string_pretty(&comment)?);
            }
        },
        Commands::Search { query } => {
            let results = client.search(query, None, None, None, None).await?;
            print_search_results(results.results);
        }
        Commands::Tui => {
            cli::run_tui(client).await?;
        }
        Commands::Convert {
            input,
            output,
            from,
            to,
        } => {
            cli::convert::run_convert(input, output, from, to).await?;
        }
        Commands::Sync { dir, notion_db } => {
            let db = notion_db
                .or_else(|| std::env::var("NOTION_DB_ID").ok())
                .expect("NOTION_DB_ID is required for sync");
            cli::sync_cmd::start_sync(dir, client, db).await?;
        }
        Commands::Diff { old, new } => {
            cli::diff::run_diff(old, new).await?;
        }
        Commands::McpServe => {
            cli::mcp::run_mcp_server().await?;
        }
    }

    Ok(())
}

fn print_search_results(results: Vec<serde_json::Value>) {
    println!("{:<40} | {:<36} | {:<10}", "Title", "ID", "Type");
    println!("{:-<40}-+-{:-<36}-+-{:-<10}", "", "", "");
    for res in results {
        let id = res.get("id").and_then(|v| v.as_str()).unwrap_or("N/A");
        let obj_type = res
            .get("object")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let title = if obj_type == "page" {
            res.get("properties")
                .and_then(|p| p.get("title"))
                .or_else(|| {
                    // Try to find any property with "title" type if the key is not "title"
                    res.get("properties")
                        .and_then(|p| p.as_object())
                        .and_then(|obj| {
                            obj.values()
                                .find(|v| v.get("type").and_then(|t| t.as_str()) == Some("title"))
                        })
                })
                .and_then(|t| t.get("title"))
                .and_then(|t| t.as_array())
                .and_then(|arr| arr.first())
                .and_then(|t| t.get("plain_text"))
                .and_then(|v| v.as_str())
                .unwrap_or("Untitled")
        } else if obj_type == "database" {
            res.get("title")
                .and_then(|t| t.as_array())
                .and_then(|arr| arr.first())
                .and_then(|t| t.get("plain_text"))
                .and_then(|v| v.as_str())
                .unwrap_or("Untitled Database")
        } else {
            "N/A"
        };

        println!("{:<40} | {:<36} | {:<10}", title, id, obj_type);
    }
}
