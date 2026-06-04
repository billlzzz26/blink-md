use anyhow::Result;
use blink_md::api::markdown::parse_markdown;

pub async fn run_mcp_server() -> Result<()> {
    // let mut server = Server::new("doc-converter", "1.0.0");
    // server.register_tool(...);

    println!("Starting MCP Server on 0.0.0.0:3000... (Placeholder)");
    // server.listen("0.0.0.0:3000").await?;

    // To make sure parse_markdown is used and no warning is generated:
    let _ = parse_markdown("test");

    Ok(())
}
