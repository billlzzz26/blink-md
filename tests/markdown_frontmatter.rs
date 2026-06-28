//! Tests for YAML frontmatter block detection and parsing.
//!
//! Frontmatter is a YAML block at the start of a Markdown file,
//! delimited by `---` lines, e.g.:
//!
//! ```text
//! ---
//! title: Hello
//! tags: [a, b]
//! ---
//! # Body starts here
//! ```
//!
//! These tests cover `blink_md::api::markdown_frontmatter::detect_frontmatter`.

use blink_md::api::markdown_frontmatter::{detect_frontmatter, FrontmatterBlock, FrontmatterError};

/// Helper: parse frontmatter and unwrap, panic with file/line on error.
fn detect_ok(input: &str) -> Option<FrontmatterBlock> {
    detect_frontmatter(input).expect("detect_frontmatter should not error on valid input")
}

#[test]
fn test_detect_returns_none_when_no_frontmatter() {
    let input = "# Hello\n\nBody content.\n";
    let result = detect_ok(input);
    assert!(
        result.is_none(),
        "expected None for plain markdown, got {:?}",
        result
    );
}

#[test]
fn test_detect_returns_none_when_not_starting_with_dashes() {
    // `---` exists mid-file but not at the very beginning -> no frontmatter
    let input = "Some prose\n---\nNot a frontmatter block\n";
    let result = detect_ok(input);
    assert!(result.is_none());
}

#[test]
fn test_detect_simple_frontmatter() {
    let input = "---\ntitle: Hello\n---\n# Body\n";
    let result = detect_ok(input).expect("expected frontmatter");
    assert_eq!(result.yaml.trim(), "title: Hello");
    assert_eq!(result.content, "# Body\n");
}

#[test]
fn test_detect_multi_line_yaml() {
    let input = "\
---
title: My Page
status: In Progress
tags:
  - rust
  - notion
---
# Heading

Body.
";
    let result = detect_ok(input).expect("expected frontmatter");
    assert!(result.yaml.contains("title: My Page"));
    assert!(result.yaml.contains("status: In Progress"));
    assert!(result.yaml.contains("tags:"));
    assert_eq!(result.content, "# Heading\n\nBody.\n");
}

#[test]
fn test_detect_empty_yaml_block() {
    let input = "---\n---\n# Body\n";
    let result = detect_ok(input).expect("expected frontmatter for empty yaml block");
    assert_eq!(result.yaml.trim(), "");
    assert_eq!(result.content, "# Body\n");
}

#[test]
fn test_detect_frontmatter_with_blank_lines_after() {
    let input = "---\ntitle: Foo\n---\n\n\n# Body\n";
    let result = detect_ok(input).expect("expected frontmatter");
    assert_eq!(result.yaml.trim(), "title: Foo");
    assert_eq!(result.content, "\n\n# Body\n");
}

#[test]
fn test_detect_no_closing_dashes_returns_none() {
    // Unterminated frontmatter is treated as plain markdown (per design choice).
    let input = "---\ntitle: Foo\n# Body\n";
    let result = detect_ok(input);
    assert!(
        result.is_none(),
        "unterminated frontmatter should not match"
    );
}

#[test]
fn test_detect_unclosed_frontmatter_does_not_panic_on_long_input() {
    // Long input without closing `---` should still return None, not panic.
    let body = "Lorem ipsum ".repeat(1000);
    let input = format!("---\ntitle: Foo\n{}", body);
    let result = detect_ok(&input);
    assert!(result.is_none());
}

#[test]
fn test_detect_preserves_inline_dashes_in_body() {
    // Body that contains `---` (e.g., a setext heading underline or divider)
    // should not be mistaken for the closing frontmatter delimiter.
    let input = "---\ntitle: Foo\n---\nSome text\n---\nMore text\n";
    let result = detect_ok(input).expect("expected frontmatter");
    assert_eq!(result.yaml.trim(), "title: Foo");
    assert_eq!(result.content, "Some text\n---\nMore text\n");
}

#[test]
fn test_detect_carriage_return_line_endings() {
    // Accept CRLF line endings for cross-platform files; lines are normalized
    // to LF in both the extracted YAML and the remaining content.
    let input = "---\r\ntitle: Foo\r\n---\r\n# Body\r\n";
    let result = detect_ok(input).expect("expected frontmatter");
    assert!(result.yaml.contains("title: Foo"));
    assert_eq!(result.content, "# Body\n");
}

#[test]
fn test_detect_empty_input() {
    let result = detect_ok("");
    assert!(result.is_none());
}

#[test]
fn test_detect_only_opening_dashes() {
    let input = "---\n";
    let result = detect_ok(input);
    assert!(result.is_none());
}

#[test]
fn test_detect_yaml_block_with_colons_in_values() {
    // Values can contain colons; only line-starting `---` should close the block.
    let input = "---\nurl: https://example.com/path:1\n---\n# Body\n";
    let result = detect_ok(input).expect("expected frontmatter");
    assert!(result.yaml.contains("https://example.com/path:1"));
    assert_eq!(result.content, "# Body\n");
}

#[test]
fn test_frontmatter_block_struct_has_yaml_and_content_fields() {
    // Compile-time-ish check: ensure the struct exposes the expected fields.
    let block = FrontmatterBlock {
        yaml: "k: v".to_string(),
        content: "body".to_string(),
    };
    assert_eq!(block.yaml, "k: v");
    assert_eq!(block.content, "body");
}

#[test]
fn test_frontmatter_error_type_is_exported() {
    // The error type should be reachable from outside the module so that
    // callers (Phase B) can match on it when YAML structure validation fails.
    let err = FrontmatterError::InvalidYaml("boom".to_string());
    let msg = format!("{}", err);
    assert!(msg.contains("boom"));
}

// ---------------------------------------------------------------------------
// Issue 9: detection must reject syntactically invalid YAML
// ---------------------------------------------------------------------------

#[test]
fn test_detect_frontmatter_error_on_invalid_yaml() {
    // Delimiters are present, but the body between them is not valid YAML.
    // `detect_frontmatter` must surface this as `InvalidYaml` rather than
    // silently claiming a block exists.
    let input = "---\nbad: [unclosed\n---\nbody\n";
    let result = detect_frontmatter(input);
    match result {
        Err(FrontmatterError::InvalidYaml(_)) => {}
        other => panic!("expected InvalidYaml error, got {:?}", other),
    }
}

#[test]
fn test_detect_frontmatter_with_valid_yaml_still_succeeds() {
    // Sanity: the new YAML-validity check does not regress the happy path.
    let input = "---\nfoo: bar\n---\nbody\n";
    let block = detect_ok(input).expect("valid YAML should be detected");
    assert_eq!(block.yaml.trim(), "foo: bar");
    assert_eq!(block.content, "body\n");
}
