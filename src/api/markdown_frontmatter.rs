//! YAML Frontmatter detection and parsing for Markdown files.
//!
//! A Markdown frontmatter is a YAML block at the very start of a file,
//! delimited by `---` lines, e.g.:
//!
//! ```text
//! ---
//! title: Hello
//! tags: [rust, notion]
//! ---
//! # Body starts here
//! ```
//!
//! This module exposes [`detect_frontmatter`] which extracts the raw YAML
//! text and the remaining Markdown body. Structured parsing of YAML into
//! Notion [`PropertyValue`](crate::ir::metadata::PropertyValue) lives in
//! the IR layer (see [`crate::ir::frontmatter`]).
//!
//! Design choice: an unterminated frontmatter (no closing `---`) is
//! treated as plain Markdown (i.e. the function returns `None`) so that
//! ordinary documents starting with prose never get falsely classified.

use thiserror::Error;

/// The result of a successful frontmatter detection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontmatterBlock {
    /// Raw YAML text between the two `---` delimiters (no trailing newline trimmed).
    pub yaml: String,
    /// The Markdown body that follows the closing `---` line.
    pub content: String,
}

/// Errors that can occur while extracting a frontmatter block.
#[derive(Debug, Error)]
pub enum FrontmatterError {
    /// The YAML content was extracted but failed to parse as valid YAML.
    /// This is reserved for future use when structured parsing is added
    /// at this layer; for now detection only validates delimiter shape.
    #[error("invalid YAML in frontmatter block: {0}")]
    InvalidYaml(String),
}

/// Detect a YAML frontmatter block at the start of `input`.
///
/// Returns:
/// - `Ok(Some(block))` if the input begins with a `---` line followed by
///   a closing `---` line,
/// - `Ok(None)` if the input does not start with a `---` line, or if a
///   starting `---` is never closed,
/// - `Err(_)` only when the structured YAML inside the block is malformed
///   (currently unreachable for detection-only calls; reserved).
///
/// The function is tolerant of CRLF line endings and treats `---` lines
/// that appear later in the body as ordinary content.
pub fn detect_frontmatter(input: &str) -> Result<Option<FrontmatterBlock>, FrontmatterError> {
    // Normalize: we operate on lines, so split on either CR or LF boundaries
    // by converting CRLF to LF up-front.
    let normalized = input.replace("\r\n", "\n");
    let mut lines = normalized.split('\n');

    // The very first line must be exactly `---` (with optional trailing whitespace).
    let first = match lines.next() {
        Some(line) => line,
        None => return Ok(None),
    };
    if first.trim_end() != "---" {
        return Ok(None);
    }

    // Find the closing `---` line.
    let mut yaml_lines: Vec<&str> = Vec::new();
    let mut closed = false;
    let mut consumed = 1; // first `---`
    for line in lines {
        consumed += 1;
        if line.trim_end() == "---" {
            closed = true;
            break;
        }
        yaml_lines.push(line);
    }

    if !closed {
        return Ok(None);
    }

    // Reconstruct content: everything after the closing `---` line.
    // We must use the *normalized* form because we have already split on `\n`.
    let prefix_len: usize = normalized
        .split('\n')
        .take(consumed)
        .map(|l| l.len() + 1) // +1 for the `\n`
        .sum();
    let content = normalized[prefix_len.min(normalized.len())..].to_string();

    Ok(Some(FrontmatterBlock {
        yaml: yaml_lines.join("\n"),
        content,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_no_frontmatter() {
        assert!(detect_frontmatter("# hi").unwrap().is_none());
    }

    #[test]
    fn smoke_with_frontmatter() {
        let r = detect_frontmatter("---\nk: v\n---\nbody").unwrap();
        assert!(r.is_some());
        let b = r.unwrap();
        assert_eq!(b.yaml, "k: v");
        assert_eq!(b.content, "body");
    }
}
