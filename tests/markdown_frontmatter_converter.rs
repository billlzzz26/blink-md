//! Tests for `MarkdownWithFrontmatterConverter` — Markdown+YAML ↔ UniversalDocument.
//!
//! The converter composes three existing pieces:
//!   1. `detect_frontmatter` to split YAML block from body
//!   2. `parse_frontmatter_to_properties` / `properties_to_yaml` for properties
//!   3. `MarkdownConverter` for the body blocks
//!
//! These tests cover both halves (`from_platform`, `to_platform`) and the
//! round-trip for typical Markdown+YAML documents.

use blink_md::converter::markdown_frontmatter::MarkdownWithFrontmatterConverter;
use blink_md::converter::{FromPlatform, ToPlatform};
use blink_md::ir::inline;
use blink_md::ir::metadata::{DocumentMetadata, PropertyValue};
use blink_md::ir::{StyleSheet, UniversalBlock, UniversalDocument};

// ---------------------------------------------------------------------------
// from_platform: with YAML frontmatter
// ---------------------------------------------------------------------------

#[test]
fn test_from_platform_extracts_yaml_into_metadata_properties() {
    let input = "\
---
title:
  type: title
  value: \"My Page\"
status:
  type: select
  value: \"In Progress\"
---
# Hello
";
    let doc = MarkdownWithFrontmatterConverter::from_platform(input.to_string()).expect("parse");
    assert_eq!(doc.metadata.properties.len(), 2);
    assert!(matches!(
        doc.metadata.properties.get("title"),
        Some(PropertyValue::Title { .. })
    ));
    assert!(matches!(
        doc.metadata.properties.get("status"),
        Some(PropertyValue::Select { .. })
    ));
}

#[test]
fn test_from_platform_extracts_body_blocks() {
    let input = "\
---
title:
  type: title
  value: \"Page\"
---
# Heading

Paragraph body.
";
    let doc = MarkdownWithFrontmatterConverter::from_platform(input.to_string()).expect("parse");
    // Heading + Paragraph
    assert!(doc.blocks.len() >= 2, "got {} blocks", doc.blocks.len());
    assert!(matches!(
        doc.blocks[0],
        UniversalBlock::Heading { level: 1, .. }
    ));
}

#[test]
fn test_from_platform_without_frontmatter_works() {
    let input = "# Just markdown\n\nNo frontmatter.\n";
    let doc = MarkdownWithFrontmatterConverter::from_platform(input.to_string()).expect("parse");
    assert!(doc.metadata.properties.is_empty());
    assert!(!doc.blocks.is_empty());
}

#[test]
fn test_from_platform_with_only_yaml_no_body() {
    let input = "\
---
title:
  type: title
  value: \"Header Only\"
---
";
    let doc = MarkdownWithFrontmatterConverter::from_platform(input.to_string()).expect("parse");
    assert_eq!(doc.metadata.properties.len(), 1);
    assert!(doc.blocks.is_empty());
}

#[test]
fn test_from_platform_invalid_yaml_returns_error() {
    let input = "\
---
title: [unclosed
type: title
---
# body
";
    let doc = MarkdownWithFrontmatterConverter::from_platform(input.to_string());
    assert!(doc.is_err(), "expected conversion error for invalid YAML");
}

#[test]
fn test_from_platform_unknown_property_type_returns_error() {
    let input = "\
---
weird:
  type: not_a_real_type
  value: 1
---
# body
";
    let doc = MarkdownWithFrontmatterConverter::from_platform(input.to_string());
    assert!(
        doc.is_err(),
        "expected conversion error for unknown property type"
    );
}

// ---------------------------------------------------------------------------
// to_platform: with metadata properties
// ---------------------------------------------------------------------------

fn make_doc_with_props(
    blocks: Vec<UniversalBlock>,
    props: Vec<(&str, PropertyValue)>,
) -> UniversalDocument {
    let mut properties = std::collections::HashMap::new();
    for (k, v) in props {
        properties.insert(k.to_string(), v);
    }
    UniversalDocument {
        metadata: DocumentMetadata {
            properties,
            ..DocumentMetadata::default()
        },
        blocks,
        styles: StyleSheet::default(),
    }
}

#[test]
fn test_to_platform_emits_yaml_header_and_body() {
    let doc = make_doc_with_props(
        vec![UniversalBlock::Heading {
            level: 1,
            content: vec![inline::text("Title".to_string())],
            style: None,
        }],
        vec![(
            "title",
            PropertyValue::Title {
                title: vec![inline::text("My Page".to_string())],
            },
        )],
    );
    let out = MarkdownWithFrontmatterConverter::to_platform(&doc).expect("render");
    // Must start with `---\n` and contain a closing `---\n` before the body.
    assert!(out.starts_with("---\n"), "got: {}", out);
    assert!(out.contains("\n---\n"), "got: {}", out);
    // Body content is present.
    assert!(out.contains("Title"), "got: {}", out);
}

#[test]
fn test_to_platform_without_properties_omits_yaml_header() {
    let doc = UniversalDocument {
        metadata: DocumentMetadata::default(),
        blocks: vec![UniversalBlock::Heading {
            level: 1,
            content: vec![inline::text("Title".to_string())],
            style: None,
        }],
        styles: StyleSheet::default(),
    };
    let out = MarkdownWithFrontmatterConverter::to_platform(&doc).expect("render");
    assert!(
        !out.starts_with("---\n"),
        "should not emit YAML header, got: {}",
        out
    );
    assert!(out.contains("Title"));
}

#[test]
fn test_to_platform_yaml_block_in_body_does_not_double_close() {
    // Sanity: a body containing `---` (e.g., a divider or setext underline)
    // should not be mistaken for the frontmatter closing delimiter.
    let doc = make_doc_with_props(
        vec![
            UniversalBlock::Heading {
                level: 1,
                content: vec![inline::text("Top".to_string())],
                style: None,
            },
            // Render a paragraph whose text contains a literal `---` sequence
            // so that the rendered Markdown body itself contains `---`. The
            // frontmatter closer detection must still emit exactly one
            // opening and one closing delimiter despite the body text.
            UniversalBlock::Paragraph {
                content: vec![inline::text("Divider-like: ---".to_string())],
                style: None,
            },
            UniversalBlock::Paragraph {
                content: vec![inline::text("After".to_string())],
                style: None,
            },
        ],
        vec![(
            "title",
            PropertyValue::Title {
                title: vec![inline::text("Doc".to_string())],
            },
        )],
    );
    let out = MarkdownWithFrontmatterConverter::to_platform(&doc).expect("render");
    // Exactly one opening frontmatter delimiter at the very start.
    assert!(out.starts_with("---\n"));
    // Exactly one closing `---` line. The body contains a `---` mid-line, so
    // a naïve regex over `\n---\n` would over-count; the only full-line
    // delimiter should be the frontmatter closer.
    assert_eq!(
        out.matches("\n---\n").count(),
        1,
        "expected exactly one closing `---` delimiter, got: {}",
        out
    );
    // The body still carries a literal `---` somewhere (so we know we
    // genuinely exercised the body-`---` case, not a stripped one).
    assert!(
        out.contains("Divider-like: ---"),
        "body `---` was lost from render: {}",
        out
    );
    // The property type tag and a body word are present.
    assert!(out.contains("type: title"));
    assert!(out.contains("Top"));
    assert!(out.contains("After"));
}

// ---------------------------------------------------------------------------
// Round-trip
// ---------------------------------------------------------------------------

#[test]
fn test_roundtrip_preserves_properties_and_blocks() {
    let input = "\
---
title:
  type: title
  value: \"My Page\"
score:
  type: number
  value: 7
done:
  type: checkbox
  value: true
---
# Heading

Body paragraph.
";
    let doc = MarkdownWithFrontmatterConverter::from_platform(input.to_string()).expect("parse");
    let out = MarkdownWithFrontmatterConverter::to_platform(&doc).expect("render");
    // Re-parse to ensure stability.
    let doc2 = MarkdownWithFrontmatterConverter::from_platform(out.clone()).expect("re-parse");
    assert_eq!(doc2.metadata.properties.len(), 3);
    assert!(matches!(
        doc2.metadata.properties.get("title"),
        Some(PropertyValue::Title { .. })
    ));
    assert!(matches!(
        doc2.metadata.properties.get("score"),
        Some(PropertyValue::Number { .. })
    ));
    assert!(matches!(
        doc2.metadata.properties.get("done"),
        Some(PropertyValue::Checkbox { .. })
    ));
    // Body blocks must also survive the round-trip — a converter bug that
    // dropped or rewrote body blocks would otherwise go undetected.
    assert!(
        doc2.blocks.len() >= 2,
        "blocks were dropped during round-trip, got {} blocks",
        doc2.blocks.len()
    );
    // Heading is preserved at the right level.
    assert!(
        matches!(doc2.blocks[0], UniversalBlock::Heading { level: 1, .. }),
        "expected first block to be a level-1 Heading, got {:?}",
        doc2.blocks[0]
    );
    // Paragraph block content is preserved (look for "Body paragraph." text).
    let has_body_paragraph = doc2.blocks.iter().any(|b| {
        if let UniversalBlock::Paragraph { content, .. } = b {
            content.iter().any(|el| {
                matches!(
                    el,
                    blink_md::ir::inline::InlineElement::TextRun { content, .. }
                        if content == "Body paragraph."
                )
            })
        } else {
            false
        }
    });
    assert!(
        has_body_paragraph,
        "paragraph block content was lost during round-trip"
    );
}

#[test]
fn test_roundtrip_without_frontmatter_preserves_body() {
    let input = "# Hello\n\nWorld.\n";
    let doc = MarkdownWithFrontmatterConverter::from_platform(input.to_string()).expect("parse");
    let out = MarkdownWithFrontmatterConverter::to_platform(&doc).expect("render");
    // No YAML header on either side.
    assert!(!out.starts_with("---\n"));
    assert!(out.contains("Hello"));
    assert!(out.contains("World"));
}

#[test]
fn test_roundtrip_property_keys_order_is_stable() {
    // Repeatedly rendering should not duplicate or shuffle properties.
    let doc = make_doc_with_props(
        vec![],
        vec![
            ("a", PropertyValue::Number { number: Some(1.0) }),
            ("b", PropertyValue::Checkbox { checkbox: false }),
            (
                "c",
                PropertyValue::Url {
                    url: Some("https://x.test".to_string()),
                },
            ),
        ],
    );
    let r1 = MarkdownWithFrontmatterConverter::to_platform(&doc).unwrap();
    let r2 = MarkdownWithFrontmatterConverter::to_platform(&doc).unwrap();
    assert_eq!(r1, r2, "to_platform should be deterministic");
    // Re-parse and re-emit: still stable.
    let d2 = MarkdownWithFrontmatterConverter::from_platform(r1.clone()).expect("re-parse");
    let r3 = MarkdownWithFrontmatterConverter::to_platform(&d2).unwrap();
    assert_eq!(r1, r3, "round-trip should be idempotent");
}
