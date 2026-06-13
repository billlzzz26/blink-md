use anyhow::Result;
use blink_md::converter::markdown::MarkdownConverter;
use blink_md::converter::notion::NotionToPlatform;
use blink_md::converter::{FromPlatform, ToPlatform};
use std::path::PathBuf;

pub async fn run_convert(
    input: PathBuf,
    output: PathBuf,
    from: Option<String>,
    to: Option<String>,
) -> Result<()> {
    let content = tokio::fs::read_to_string(&input).await?;
    let from_fmt = from.unwrap_or_else(|| {
        input
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("markdown")
            .to_string()
    });

    let doc = match from_fmt.as_str() {
        "markdown" | "md" => MarkdownConverter::from_platform(content)?,
        _ => anyhow::bail!("Unsupported source format: {}", from_fmt),
    };

    let to_fmt = to.unwrap_or_else(|| {
        output
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("json")
            .to_string()
    });

    let output_str = match to_fmt.as_str() {
        "json" => serde_json::to_string_pretty(&doc)?,
        "markdown" | "md" => MarkdownConverter::to_platform(&doc)?,
        "notion" => {
            let request = NotionToPlatform::to_platform(&doc)?;
            serde_json::to_string_pretty(&request)?
        }
        _ => anyhow::bail!("Unsupported target format: {}", to_fmt),
    };

    tokio::fs::write(&output, output_str).await?;
    println!("Converted {:?} -> {:?} (using Universal IR)", input, output);
    Ok(())
}
