//! Tests for the YAML frontmatter → Notion PropertyValue mapper.
//!
//! The YAML schema uses an explicit `type:` tag per property:
//!
//! ```yaml
//! title:
//!   type: title
//!   value: "My Page"
//! tags:
//!   type: multi_select
//!   values: [rust, notion]
//! score:
//!   type: number
//!   value: 42
//! ```
//!
//! These tests cover `blink_md::ir::frontmatter::{parse_frontmatter_to_properties,
//! properties_to_yaml, PropertyType}`.

use blink_md::ir::frontmatter::{
    parse_frontmatter_to_properties, properties_to_yaml, FrontmatterError, PropertyType,
};
use blink_md::ir::inline::{InlineElement, TextStyle};
use blink_md::ir::metadata::PropertyValue;
use std::collections::HashMap;

/// Helper: parse YAML, unwrap to HashMap.
fn parse_ok(yaml: &str) -> HashMap<String, PropertyValue> {
    parse_frontmatter_to_properties(yaml).expect("parse should succeed")
}

// ---------------------------------------------------------------------------
// parse_frontmatter_to_properties: happy paths for every supported type
// ---------------------------------------------------------------------------

#[test]
fn test_parse_title() {
    let yaml = r#"
title:
  type: title
  value: "My Page"
"#;
    let map = parse_ok(yaml);
    let p = map.get("title").expect("title key");
    match p {
        PropertyValue::Title { title } => {
            assert_eq!(title.len(), 1);
            match &title[0] {
                InlineElement::TextRun { content, .. } => assert_eq!(content, "My Page"),
                _ => panic!("expected TextRun"),
            }
        }
        other => panic!("expected Title, got {:?}", other),
    }
}

#[test]
fn test_parse_rich_text() {
    let yaml = r#"
description:
  type: rich_text
  value: "Body text"
"#;
    let map = parse_ok(yaml);
    match map.get("description").unwrap() {
        PropertyValue::RichText { rich_text } => {
            assert_eq!(rich_text.len(), 1);
            match &rich_text[0] {
                InlineElement::TextRun { content, .. } => assert_eq!(content, "Body text"),
                _ => panic!("expected TextRun"),
            }
        }
        other => panic!("expected RichText, got {:?}", other),
    }
}

#[test]
fn test_parse_number_integer() {
    let yaml = "score:\n  type: number\n  value: 42\n";
    let map = parse_ok(yaml);
    match map.get("score").unwrap() {
        PropertyValue::Number { number } => assert_eq!(*number, Some(42.0)),
        other => panic!("expected Number, got {:?}", other),
    }
}

#[test]
fn test_parse_number_float() {
    let yaml = "ratio:\n  type: number\n  value: 2.5\n";
    let map = parse_ok(yaml);
    match map.get("ratio").unwrap() {
        PropertyValue::Number { number } => assert_eq!(*number, Some(2.5)),
        other => panic!("expected Number, got {:?}", other),
    }
}

#[test]
fn test_parse_select() {
    let yaml = r#"
status:
  type: select
  value: "In Progress"
"#;
    let map = parse_ok(yaml);
    match map.get("status").unwrap() {
        PropertyValue::Select { select } => {
            let s = select.as_ref().expect("select present");
            assert_eq!(s.name, "In Progress");
        }
        other => panic!("expected Select, got {:?}", other),
    }
}

#[test]
fn test_parse_multi_select() {
    let yaml = r#"
tags:
  type: multi_select
  values: [rust, notion, mcp]
"#;
    let map = parse_ok(yaml);
    match map.get("tags").unwrap() {
        PropertyValue::MultiSelect { multi_select } => {
            let names: Vec<&str> = multi_select.iter().map(|o| o.name.as_str()).collect();
            assert_eq!(names, vec!["rust", "notion", "mcp"]);
        }
        other => panic!("expected MultiSelect, got {:?}", other),
    }
}

#[test]
fn test_parse_date_string() {
    let yaml = "due:\n  type: date\n  value: \"2026-07-01\"\n";
    let map = parse_ok(yaml);
    match map.get("due").unwrap() {
        PropertyValue::Date { date } => {
            let d = date.as_ref().expect("date present");
            assert_eq!(d.start, "2026-07-01");
            assert!(d.end.is_none());
        }
        other => panic!("expected Date, got {:?}", other),
    }
}

#[test]
fn test_parse_checkbox_true() {
    let yaml = "done:\n  type: checkbox\n  value: true\n";
    let map = parse_ok(yaml);
    match map.get("done").unwrap() {
        PropertyValue::Checkbox { checkbox } => assert!(*checkbox),
        other => panic!("expected Checkbox, got {:?}", other),
    }
}

#[test]
fn test_parse_checkbox_false() {
    let yaml = "done:\n  type: checkbox\n  value: false\n";
    let map = parse_ok(yaml);
    match map.get("done").unwrap() {
        PropertyValue::Checkbox { checkbox } => assert!(!(*checkbox)),
        other => panic!("expected Checkbox, got {:?}", other),
    }
}

#[test]
fn test_parse_url() {
    let yaml = "site:\n  type: url\n  value: \"https://example.com\"\n";
    let map = parse_ok(yaml);
    match map.get("site").unwrap() {
        PropertyValue::Url { url } => assert_eq!(url.as_deref(), Some("https://example.com")),
        other => panic!("expected Url, got {:?}", other),
    }
}

#[test]
fn test_parse_email() {
    let yaml = "contact:\n  type: email\n  value: \"a@b.com\"\n";
    let map = parse_ok(yaml);
    match map.get("contact").unwrap() {
        PropertyValue::Email { email } => assert_eq!(email.as_deref(), Some("a@b.com")),
        other => panic!("expected Email, got {:?}", other),
    }
}

#[test]
fn test_parse_multiple_properties() {
    let yaml = r#"
title:
  type: title
  value: "Page"
score:
  type: number
  value: 7
done:
  type: checkbox
  value: true
"#;
    let map = parse_ok(yaml);
    assert_eq!(map.len(), 3);
    assert!(matches!(
        map.get("title"),
        Some(PropertyValue::Title { .. })
    ));
    assert!(matches!(
        map.get("score"),
        Some(PropertyValue::Number { .. })
    ));
    assert!(matches!(
        map.get("done"),
        Some(PropertyValue::Checkbox { .. })
    ));
}

#[test]
fn test_parse_empty_yaml_returns_empty_map() {
    let map = parse_ok("");
    assert!(map.is_empty());
}

// ---------------------------------------------------------------------------
// parse_frontmatter_to_properties: error paths
// ---------------------------------------------------------------------------

#[test]
fn test_parse_unknown_type_errors() {
    let yaml = "x:\n  type: bogus\n  value: 1\n";
    let err = parse_frontmatter_to_properties(yaml).unwrap_err();
    match err {
        FrontmatterError::UnknownPropertyType(t) => assert_eq!(t, "bogus"),
        other => panic!("expected UnknownPropertyType, got {:?}", other),
    }
}

#[test]
fn test_parse_title_missing_value_errors() {
    let yaml = "title:\n  type: title\n";
    let err = parse_frontmatter_to_properties(yaml).unwrap_err();
    assert!(matches!(err, FrontmatterError::MissingField(_, _)));
}

#[test]
fn test_parse_multi_select_requires_values_key_not_value() {
    // `multi_select` uses `values:` (plural), not `value:`.
    let yaml = "tags:\n  type: multi_select\n  value: [a, b]\n";
    let err = parse_frontmatter_to_properties(yaml).unwrap_err();
    // Either missing `values` field OR wrong field type — both acceptable.
    assert!(
        matches!(
            err,
            FrontmatterError::MissingField(_, _) | FrontmatterError::WrongFieldType(_, _, _)
        ),
        "got {:?}",
        err
    );
}

#[test]
fn test_parse_checkbox_wrong_value_type_errors() {
    // checkbox expects bool; pass a string
    let yaml = "done:\n  type: checkbox\n  value: \"yes\"\n";
    let err = parse_frontmatter_to_properties(yaml).unwrap_err();
    assert!(matches!(err, FrontmatterError::WrongFieldType(_, _, _)));
}

#[test]
fn test_parse_number_wrong_value_type_errors() {
    let yaml = "score:\n  type: number\n  value: \"forty-two\"\n";
    let err = parse_frontmatter_to_properties(yaml).unwrap_err();
    assert!(matches!(err, FrontmatterError::WrongFieldType(_, _, _)));
}

#[test]
fn test_parse_invalid_yaml_errors() {
    let yaml = "title: [unclosed\n  type: title\n";
    let err = parse_frontmatter_to_properties(yaml).unwrap_err();
    assert!(matches!(err, FrontmatterError::InvalidYaml(_)));
}

#[test]
fn test_parse_top_level_scalar_not_object_errors() {
    // A bare string at top level is not a property mapping.
    let yaml = "\"just a string\"\n";
    let err = parse_frontmatter_to_properties(yaml).unwrap_err();
    assert!(matches!(
        err,
        FrontmatterError::MissingField(_, _) | FrontmatterError::WrongFieldType(_, _, _)
    ));
}

// ---------------------------------------------------------------------------
// properties_to_yaml: round-trip
// ---------------------------------------------------------------------------

fn make_title(name: &str) -> PropertyValue {
    PropertyValue::Title {
        title: vec![InlineElement::TextRun {
            content: name.to_string(),
            style: Some(TextStyle::default()),
        }],
    }
}

#[test]
fn test_serialize_title() {
    let mut map = HashMap::new();
    map.insert("title".to_string(), make_title("Hi"));
    let yaml = properties_to_yaml(&map).expect("serialize");
    // Round-trip back to ensure the value is preserved.
    let parsed = parse_ok(&yaml);
    assert!(matches!(
        parsed.get("title"),
        Some(PropertyValue::Title { .. })
    ));
}

#[test]
fn test_serialize_multiple_types_and_roundtrip() {
    let mut map = HashMap::new();
    map.insert("title".to_string(), make_title("Roundtrip"));
    map.insert(
        "score".to_string(),
        PropertyValue::Number { number: Some(99.0) },
    );
    map.insert(
        "done".to_string(),
        PropertyValue::Checkbox { checkbox: true },
    );
    map.insert(
        "site".to_string(),
        PropertyValue::Url {
            url: Some("https://x.test".to_string()),
        },
    );

    let yaml = properties_to_yaml(&map).expect("serialize");
    let parsed = parse_ok(&yaml);
    assert_eq!(parsed.len(), 4);
    assert!(matches!(
        parsed.get("title"),
        Some(PropertyValue::Title { .. })
    ));
    assert!(matches!(
        parsed.get("score"),
        Some(PropertyValue::Number { number: Some(v) }) if (*v - 99.0).abs() < 1e-9
    ));
    assert!(matches!(
        parsed.get("done"),
        Some(PropertyValue::Checkbox { checkbox: true })
    ));
    assert!(matches!(
        parsed.get("site"),
        Some(PropertyValue::Url { url: Some(_) })
    ));
}

#[test]
fn test_serialize_empty_map_returns_empty_string() {
    let map = HashMap::new();
    let yaml = properties_to_yaml(&map).expect("serialize");
    // Empty YAML is acceptable — parser returns an empty map for it.
    let parsed = parse_ok(&yaml);
    assert!(parsed.is_empty());
}

// ---------------------------------------------------------------------------
// PropertyType enum exposes known variants
// ---------------------------------------------------------------------------

#[test]
fn test_property_type_enum_variants_are_distinct() {
    let variants = [
        PropertyType::Title,
        PropertyType::RichText,
        PropertyType::Number,
        PropertyType::Select,
        PropertyType::MultiSelect,
        PropertyType::Date,
        PropertyType::Checkbox,
        PropertyType::Url,
        PropertyType::Email,
    ];
    // Just ensure all variants exist and are distinct via Debug + equality.
    for (i, a) in variants.iter().enumerate() {
        for (j, b) in variants.iter().enumerate() {
            if i == j {
                assert_eq!(a, b);
            } else {
                assert_ne!(a, b);
            }
        }
    }
}

#[test]
fn test_property_type_serializes_to_snake_case_string() {
    // The serializer writes the tag in lowercase; this test pins that contract.
    assert_eq!(format!("{:?}", PropertyType::Title), "Title");
    assert_eq!(format!("{:?}", PropertyType::MultiSelect), "MultiSelect");
    // Snake-case round-trip via Display:
    assert_eq!(format!("{}", PropertyType::RichText), "rich_text");
    assert_eq!(format!("{}", PropertyType::MultiSelect), "multi_select");
}

// ---------------------------------------------------------------------------
// Issue 5: Number null must round-trip (not silently become 0)
// ---------------------------------------------------------------------------

#[test]
fn test_serialize_number_null_roundtrips() {
    let mut map = HashMap::new();
    map.insert("score".to_string(), PropertyValue::Number { number: None });
    let yaml = properties_to_yaml(&map).unwrap();
    // The output must NOT coerce null to a literal number like `0` or `0.0`.
    assert!(
        !yaml.contains(": 0\n") && !yaml.contains(": 0.0\n"),
        "expected null number to be preserved, got: {}",
        yaml
    );
    // It should serialize as YAML null. `serde_yaml` emits either `null`,
    // `~`, or omits the key depending on configuration; accept any of those.
    assert!(
        yaml.contains("null") || yaml.contains("~") || !yaml.contains("value:"),
        "expected null in serialized YAML, got: {}",
        yaml
    );

    // And the value must round-trip back as `Number { number: None }`.
    let parsed = parse_frontmatter_to_properties(&yaml).unwrap();
    match parsed.get("score").unwrap() {
        PropertyValue::Number { number: None } => {}
        other => panic!("expected Number {{ number: None }}, got {:?}", other),
    }
}

#[test]
fn test_parse_number_null_value() {
    // Direct parse of `value: null` must produce `Number { number: None }`.
    let yaml = "score:\n  type: number\n  value: null\n";
    let map = parse_ok(yaml);
    match map.get("score").unwrap() {
        PropertyValue::Number { number: None } => {}
        other => panic!("expected Number {{ number: None }}, got {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// Issue 6: `type: custom` must round-trip via PropertyValue::Custom
// ---------------------------------------------------------------------------

#[test]
fn test_serialize_custom_roundtrips() {
    let mut map = HashMap::new();
    map.insert(
        "weird".to_string(),
        PropertyValue::Custom {
            key: "weird".to_string(),
            value: serde_json::json!({ "rel": ["page-1"], "type": "relation" }),
        },
    );
    let yaml = properties_to_yaml(&map).unwrap();
    assert!(
        yaml.contains("type: custom"),
        "expected `type: custom` in serialized YAML, got: {}",
        yaml
    );
    let parsed = parse_frontmatter_to_properties(&yaml).unwrap();
    match parsed.get("weird").unwrap() {
        PropertyValue::Custom { key, value } => {
            assert_eq!(key, "weird");
            assert_eq!(value["rel"][0], "page-1");
            assert_eq!(value["type"], "relation");
        }
        other => panic!("expected PropertyValue::Custom, got {:?}", other),
    }
}

#[test]
fn test_parse_custom_with_null_value() {
    // `type: custom` with `value: null` must parse to a Custom variant
    // carrying `serde_json::Value::Null`.
    let yaml = "weird:\n  type: custom\n  value: null\n";
    let map = parse_ok(yaml);
    match map.get("weird").unwrap() {
        PropertyValue::Custom { key, value } => {
            assert_eq!(key, "weird");
            assert!(value.is_null(), "expected null, got {}", value);
        }
        other => panic!("expected PropertyValue::Custom, got {:?}", other),
    }
}

#[test]
fn test_unknown_property_variant_serializes_as_custom() {
    // PropertyValue::Relation is not a dedicated wire type, so it must be
    // serialized via the `custom` fallback and round-trip back as
    // PropertyValue::Custom.
    let mut map = HashMap::new();
    map.insert(
        "refs".to_string(),
        PropertyValue::Relation {
            relation: vec!["page-1".to_string(), "page-2".to_string()],
        },
    );
    let yaml = properties_to_yaml(&map).unwrap();
    assert!(yaml.contains("type: custom"), "got: {}", yaml);
    let parsed = parse_frontmatter_to_properties(&yaml).unwrap();
    assert!(
        matches!(parsed.get("refs"), Some(PropertyValue::Custom { .. })),
        "expected Custom, got {:?}",
        parsed.get("refs")
    );
}
