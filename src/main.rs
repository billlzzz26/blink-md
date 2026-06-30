use blink_md::NotionClient;
use clap::{Parser, Subcommand};

mod cli;

use cli::output::{
    field_block_type, field_id, field_last_edited, field_object, field_title, field_url,
    print_object, print_records, Column, OutputFormat,
};

#[derive(Parser)]
#[command(name = "blink-md")]
#[command(version)]
#[command(about = concat!("Notion API CLI & TUI (Target: ", env!("BUILD_TARGET_OS"), ", Env: ", env!("BUILD_ENVIRONMENT"), ")"))]
#[command(after_help = "EXAMPLES:
    # Launch interactive TUI
    blink-md tui

    # Search for pages
    blink-md search \"Meeting Notes\"

    # Convert Markdown to Notion-flavored JSON
    blink-md convert -i README.md -o page.json --to notion

    # Sync local directory to Notion database
    blink-md sync --dir ./docs --notion-db 1234567890abcdef

    # Upgrade to the latest version
    blink-md upgrade")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output format for results: `table` (human) or `json` (scripting).
    #[arg(long, value_enum, default_value_t = OutputFormat::Table, global = true)]
    format: OutputFormat,

    /// Print verbose diagnostics (full error cause chain) to stderr.
    #[arg(short, long, global = true)]
    verbose: bool,
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
    /// Export a Notion page to a Markdown file with a YAML frontmatter header
    ExportPage {
        /// Page ID to export
        page_id: String,
        /// Directory to write the `<slug>-<page-id>.md` file into
        #[arg(short, long, default_value = ".")]
        out_dir: std::path::PathBuf,
    },
    /// Show diff between two files
    Diff {
        old: std::path::PathBuf,
        new: std::path::PathBuf,
    },
    /// Upgrade blink-md to the latest version
    Upgrade,
    /// Start MCP server
    McpServe,
    /// Generate persona/recipe skill bundles from the registry
    GenerateSkills {
        /// Directory to write generated skills into
        #[arg(long, default_value = "skills")]
        output_dir: String,
    },
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
async fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    let verbose = cli.verbose;
    match run(cli).await {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(err) => {
            print_error(&err, verbose);
            std::process::ExitCode::FAILURE
        }
    }
}

/// Render an error to stderr in the conventional CLI style: a red `error:`
/// prefix (only on a TTY and when `NO_COLOR` is unset) followed by the message
/// and its cause chain. With `--verbose`, print the full `{:?}` form instead
/// (cause chain plus any backtrace), matching how tools like cargo behave.
fn print_error(err: &anyhow::Error, verbose: bool) {
    use std::io::IsTerminal;
    let color = std::io::stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none();
    let prefix = if color {
        "\x1b[31;1merror:\x1b[0m"
    } else {
        "error:"
    };
    if verbose {
        eprintln!("{prefix} {err:?}");
    } else {
        eprint!("{prefix} {err}");
        for cause in err.chain().skip(1) {
            eprint!(": {cause}");
        }
        eprintln!();
    }
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    let output = cli.format;

    // Commands that don't talk to the Notion API run before the NOTION_TOKEN
    // requirement so they work with no credentials — a user converting or
    // diffing a local file should never be asked for a token they don't need.
    // The MCP server binds the Notion client server-side only when NOTION_TOKEN
    // is present.
    if let Commands::GenerateSkills { output_dir } = &cli.command {
        let args = vec!["--output-dir".to_string(), output_dir.clone()];
        return blink_md::sync::generate_skills::handle_generate_skills(&args).await;
    }
    #[cfg(feature = "mcp")]
    if let Commands::McpServe = &cli.command {
        return cli::mcp::run_mcp_server().await;
    }
    if let Commands::Convert {
        input,
        output: out_path,
        from,
        to,
    } = &cli.command
    {
        return cli::convert::run_convert(
            input.clone(),
            out_path.clone(),
            from.clone(),
            to.clone(),
        )
        .await;
    }
    if let Commands::Diff { old, new } = &cli.command {
        return cli::diff::run_diff(old.clone(), new.clone()).await;
    }
    if let Commands::Upgrade = &cli.command {
        return handle_upgrade().await;
    }

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
                print_object(output, &serde_json::to_value(&user)?, USER_FIELDS);
            }
            UserAction::List => {
                let users = client.list_users().await?;
                print_records(
                    output,
                    &as_records(serde_json::to_value(&users)?),
                    USER_FIELDS,
                );
            }
            UserAction::Get { user_id } => {
                let user = client.get_user(&user_id).await?;
                print_object(output, &serde_json::to_value(&user)?, USER_FIELDS);
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
                print_records(output, &results.results, OBJECT_FIELDS);
            }
            PageAction::Get { page_id } => {
                let page = client.get_page(&page_id).await?;
                print_object(output, &serde_json::to_value(&page)?, PAGE_FIELDS);
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
                print_object(output, &serde_json::to_value(&db)?, PAGE_FIELDS);
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
                print_records(
                    output,
                    &as_records(serde_json::to_value(&list.results)?),
                    BLOCK_FIELDS,
                );
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
            print_records(output, &results.results, OBJECT_FIELDS);
        }
        Commands::Tui => {
            cli::run_tui(client).await?;
        }
        Commands::Sync { dir, notion_db } => {
            let db = notion_db
                .or_else(|| std::env::var("NOTION_DB_ID").ok())
                .expect("NOTION_DB_ID is required for sync");
            cli::sync_cmd::start_sync(dir, client, db).await?;
        }
        Commands::ExportPage { page_id, out_dir } => {
            let path = cli::export_cmd::export_page_to_md(&client, &page_id, &out_dir).await?;
            println!("Exported page {} to {}", page_id, path.display());
        }
        Commands::Convert { .. } | Commands::Diff { .. } | Commands::Upgrade => {
            unreachable!("offline commands are dispatched before client initialization")
        }
        Commands::McpServe => {
            // `mcp` builds dispatch this before the client is created; this arm
            // only runs in builds without the feature.
            #[cfg(feature = "mcp")]
            unreachable!("mcp-serve is dispatched before client initialization");
            #[cfg(not(feature = "mcp"))]
            anyhow::bail!(
                "This build was compiled without the `mcp` feature. \
                 Use the `blink-md-mcp` binary, or rebuild with `--features mcp`."
            );
        }
        Commands::GenerateSkills { .. } => {
            unreachable!("generate-skills is dispatched before client initialization")
        }
    }

    Ok(())
}

/// Columns for a mixed search/list of pages and databases.
const OBJECT_FIELDS: &[Column] = &[
    Column::new("Title", field_title),
    Column::new("Type", field_object),
    Column::new("ID", field_id),
    Column::new("Last edited", field_last_edited),
];

/// Detailed fields for a single page/database.
const PAGE_FIELDS: &[Column] = &[
    Column::new("Title", field_title),
    Column::new("Type", field_object),
    Column::new("ID", field_id),
    Column::new("URL", field_url),
    Column::new("Last edited", field_last_edited),
];

/// Columns for users.
const USER_FIELDS: &[Column] = &[
    Column::new("Name", field_title),
    Column::new("Type", field_object),
    Column::new("ID", field_id),
];

/// Columns for a block listing.
const BLOCK_FIELDS: &[Column] = &[
    Column::new("Type", field_block_type),
    Column::new("ID", field_id),
    Column::new("Last edited", field_last_edited),
];

/// Normalize an API response into a flat list of record values: an array as-is,
/// an object's `results` array (paginated lists), or a single object wrapped in
/// a one-element list.
fn as_records(value: serde_json::Value) -> Vec<serde_json::Value> {
    match value {
        serde_json::Value::Array(items) => items,
        serde_json::Value::Object(mut map) => match map.remove("results") {
            Some(serde_json::Value::Array(items)) => items,
            _ => vec![serde_json::Value::Object(map)],
        },
        other => vec![other],
    }
}

async fn handle_upgrade() -> anyhow::Result<()> {
    println!("Checking for updates...");
    let status = self_update::backends::github::Update::configure()
        .repo_owner("billlzzz26")
        .repo_name("blink-md")
        .bin_name("blink-md")
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;

    println!("Update status: {}", status.version());
    Ok(())
}
