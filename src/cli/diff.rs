use anyhow::Result;
use blink_md::api::markdown::{parse_markdown, ToMarkdown};
use similar::{ChangeTag, TextDiff};
use std::path::PathBuf;

pub async fn run_diff(old: PathBuf, new: PathBuf) -> Result<()> {
    let old_content = tokio::fs::read_to_string(&old).await?;
    let new_content = tokio::fs::read_to_string(&new).await?;

    let old_blocks = parse_markdown(&old_content);
    let new_blocks = parse_markdown(&new_content);

    let mut old_md = String::new();
    for block in old_blocks {
        old_md.push_str(&block.to_markdown(0));
        old_md.push('\n');
    }

    let mut new_md = String::new();
    for block in new_blocks {
        new_md.push_str(&block.to_markdown(0));
        new_md.push('\n');
    }

    let diff = TextDiff::from_lines(&old_md, &new_md);
    let mut output = String::new();
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "- ",
            ChangeTag::Insert => "+ ",
            ChangeTag::Equal => "  ",
        };
        output.push_str(&format!("{}{}", sign, change));
    }

    println!("{}", output);
    Ok(())
}
