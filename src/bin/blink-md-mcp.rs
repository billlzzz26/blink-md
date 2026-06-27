//! `blink-md-mcp` — the unified blink-md MCP server.
//!
//! Exposes every document tool (Notion, Markdown, Lark Sheets, Mermaid) over a
//! single stdio MCP connection. Requires the `mcp` feature.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // `--check` / `--setup` let installers verify the binary without launching
    // the long-running stdio server.
    if std::env::args().any(|arg| arg == "--check" || arg == "--setup") {
        println!("blink-md-mcp {}: OK", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    blink_md::mcp::server::run().await
}
