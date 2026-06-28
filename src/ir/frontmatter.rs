//! Frontmatter ↔ Notion PropertyValue mapper.
//!
//! Each YAML entry must declare its target Notion property type with an
//! explicit `type:` tag:
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
//! This module exposes [`parse_frontmatter_to_properties`] and
//! [`properties_to_yaml`] which together provide a lossless (modulo
//! styling/relations) round-trip between YAML frontmatter and the
//! `metadata.properties` map of [`crate::ir::metadata::DocumentMetadata`].

use crate::ir::inline::{text, InlineElement};
use crate::ir::metadata::{PropertyValue, SelectOption};
use serde_yaml::Value as YamlValue;
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

/// Notion property types supported by the frontmatter mapper.
///
/// The string form is the wire format written to YAML (`type: title`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropertyType {
    Title,
    RichText,
    Number,
    Select,
    MultiSelect,
    Date,
    Checkbox,
    Url,
    Email,
    /// Catch-all for any [`crate::ir::metadata::PropertyValue`] variant that
    /// does not have a dedicated wire representation. The full variant is
    /// stored under the `value:` key as opaque YAML/JSON.
    Custom,
}

impl PropertyType {
    /// Parse from the YAML tag string (lowercase, snake_case for multi-word).
    pub fn from_tag(tag: &str) -> Option<Self> {
        match tag {
            "title" => Some(Self::Title),
            "rich_text" => Some(Self::RichText),
            "number" => Some(Self::Number),
            "select" => Some(Self::Select),
            "multi_select" => Some(Self::MultiSelect),
            "date" => Some(Self::Date),
            "checkbox" => Some(Self::Checkbox),
            "url" => Some(Self::Url),
            "email" => Some(Self::Email),
            "custom" => Some(Self::Custom),
            _ => None,
        }
    }

    /// Wire format string used as the `type:` value in YAML.
    pub fn as_tag(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::RichText => "rich_text",
            Self::Number => "number",
            Self::Select => "select",
            Self::MultiSelect => "multi_select",
            Self::Date => "date",
            Self::Checkbox => "checkbox",
            Self::Url => "url",
            Self::Email => "email",
            Self::Custom => "custom",
        }
    }
}

impl fmt::Display for PropertyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_tag())
    }
}

/// Errors produced by the frontmatter mapper.
#[derive(Debug, Error)]
pub enum FrontmatterError {
    #[error("invalid YAML in frontmatter block: {0}")]
    InvalidYaml(String),
    #[error("unknown property type: {0}")]
    UnknownPropertyType(String),
    #[error("property '{0}' is missing required field '{1}'")]
    MissingField(String, String),
    #[error("property '{0}' has wrong field type for '{1}': {2}")]
    WrongFieldType(String, String, String),
}

/// Parse a YAML frontmatter block into a map of `PropertyValue`s.
///
/// `yaml` is expected to be the raw text between the two `---` delimiters
/// (see [`crate::api::markdown_frontmatter::detect_frontmatter`]).
pub fn parse_frontmatter_to_properties(
    yaml: &str,
) -> Result<HashMap<String, PropertyValue>, FrontmatterError> {
    let parsed: YamlValue =
        serde_yaml::from_str(yaml).map_err(|e| FrontmatterError::InvalidYaml(e.to_string()))?;

    let mut out = HashMap::new();
    let mapping = match parsed {
        YamlValue::Null => return Ok(out),
        YamlValue::Mapping(m) => m,
        // Top-level scalar or sequence is not a property mapping.
        other => {
            return Err(FrontmatterError::WrongFieldType(
                "<root>".into(),
                "object".into(),
                format!("{:?}", other),
            ))
        }
    };

    for (k, v) in mapping {
        let key = match k {
            YamlValue::String(s) => s,
            other => {
                return Err(FrontmatterError::WrongFieldType(
                    "<key>".into(),
                    "string".into(),
                    format!("{:?}", other),
                ))
            }
        };
        let prop = parse_property(&key, &v)?;
        out.insert(key, prop);
    }
    Ok(out)
}

fn parse_property(name: &str, v: &YamlValue) -> Result<PropertyValue, FrontmatterError> {
    let m = match v {
        YamlValue::Mapping(m) => m,
        other => {
            return Err(FrontmatterError::WrongFieldType(
                name.into(),
                "object with `type:`".into(),
                format!("{:?}", other),
            ))
        }
    };

    let type_val = m
        .get(YamlValue::String("type".into()))
        .ok_or_else(|| FrontmatterError::MissingField(name.into(), "type".into()))?;
    let type_str = match type_val {
        YamlValue::String(s) => s,
        other => {
            return Err(FrontmatterError::WrongFieldType(
                name.into(),
                "type".into(),
                format!("{:?}", other),
            ))
        }
    };
    let pt = PropertyType::from_tag(type_str)
        .ok_or_else(|| FrontmatterError::UnknownPropertyType(type_str.clone()))?;

    match pt {
        PropertyType::Title => {
            let val = require_string(name, m, "value")?;
            Ok(PropertyValue::Title {
                title: vec![text(val)],
            })
        }
        PropertyType::RichText => {
            let val = require_string(name, m, "value")?;
            Ok(PropertyValue::RichText {
                rich_text: vec![text(val)],
            })
        }
        PropertyType::Number => {
            let val = m
                .get(YamlValue::String("value".into()))
                .ok_or_else(|| FrontmatterError::MissingField(name.into(), "value".into()))?;
            let n = match val {
                YamlValue::Number(n) => n.as_f64().ok_or_else(|| {
                    FrontmatterError::WrongFieldType(
                        name.into(),
                        "value".into(),
                        format!("{:?}", n),
                    )
                })?,
                YamlValue::String(s) => s.parse::<f64>().map_err(|_| {
                    FrontmatterError::WrongFieldType(name.into(), "value".into(), s.clone())
                })?,
                // Preserve `number: null` rather than coercing to 0.
                YamlValue::Null => {
                    return Ok(PropertyValue::Number { number: None });
                }
                other => {
                    return Err(FrontmatterError::WrongFieldType(
                        name.into(),
                        "value".into(),
                        format!("{:?}", other),
                    ))
                }
            };
            Ok(PropertyValue::Number { number: Some(n) })
        }
        PropertyType::Select => {
            match m.get(YamlValue::String("value".into())) {
                None | Some(YamlValue::Null) => Ok(PropertyValue::Select { select: None }),
                Some(YamlValue::String(s)) => Ok(PropertyValue::Select {
                    select: Some(SelectOption {
                        id: None,
                        name: s.clone(),
                        color: None,
                    }),
                }),
                Some(other) => Err(FrontmatterError::WrongFieldType(
                    name.into(),
                    "value".into(),
                    format!("{:?}", other),
                )),
            }
        }
        PropertyType::MultiSelect => {
            let val = m
                .get(YamlValue::String("values".into()))
                .ok_or_else(|| FrontmatterError::MissingField(name.into(), "values".into()))?;
            let seq = match val {
                YamlValue::Sequence(s) => s,
                other => {
                    return Err(FrontmatterError::WrongFieldType(
                        name.into(),
                        "values".into(),
                        format!("{:?}", other),
                    ))
                }
            };
            let mut opts = Vec::with_capacity(seq.len());
            for item in seq {
                match item {
                    YamlValue::String(s) => opts.push(SelectOption {
                        id: None,
                        name: s.clone(),
                        color: None,
                    }),
                    other => {
                        return Err(FrontmatterError::WrongFieldType(
                            name.into(),
                            "values[].name".into(),
                            format!("{:?}", other),
                        ))
                    }
                }
            }
            Ok(PropertyValue::MultiSelect { multi_select: opts })
        }
        PropertyType::Date => {
            match m.get(YamlValue::String("value".into())) {
                None | Some(YamlValue::Null) => Ok(PropertyValue::Date { date: None }),
                Some(YamlValue::String(s)) => Ok(PropertyValue::Date {
                    date: Some(crate::ir::metadata::DateValue {
                        start: s.clone(),
                        end: None,
                        time_zone: None,
                    }),
                }),
                Some(other) => Err(FrontmatterError::WrongFieldType(
                    name.into(),
                    "value".into(),
                    format!("{:?}", other),
                )),
            }
        }
        PropertyType::Checkbox => {
            let val = m
                .get(YamlValue::String("value".into()))
                .ok_or_else(|| FrontmatterError::MissingField(name.into(), "value".into()))?;
            match val {
                YamlValue::Bool(b) => Ok(PropertyValue::Checkbox { checkbox: *b }),
                other => Err(FrontmatterError::WrongFieldType(
                    name.into(),
                    "value".into(),
                    format!("{:?}", other),
                )),
            }
        }
        PropertyType::Url => {
            match m.get(YamlValue::String("value".into())) {
                None | Some(YamlValue::Null) => Ok(PropertyValue::Url { url: None }),
                Some(YamlValue::String(s)) => Ok(PropertyValue::Url { url: Some(s.clone()) }),
                Some(other) => Err(FrontmatterError::WrongFieldType(
                    name.into(),
                    "value".into(),
                    format!("{:?}", other),
                )),
            }
        }
        PropertyType::Email => {
            match m.get(YamlValue::String("value".into())) {
                None | Some(YamlValue::Null) => Ok(PropertyValue::Email { email: None }),
                Some(YamlValue::String(s)) => Ok(PropertyValue::Email { email: Some(s.clone()) }),
                Some(other) => Err(FrontmatterError::WrongFieldType(
                    name.into(),
                    "value".into(),
                    format!("{:?}", other),
                )),
            }
        }
        // Catch-all for opaque / not-yet-supported property values. The
        // payload is stored as a generic JSON value so it round-trips even
        // when the original variant (Relation, Formula, Rollup, …) has no
        // dedicated wire schema.
        PropertyType::Custom => {
            let val = m.get(YamlValue::String("value".into())).cloned();
            let json = match val {
                None | Some(YamlValue::Null) => serde_json::Value::Null,
                Some(v) => yaml_to_json(name, v)?,
            };
            Ok(PropertyValue::Custom {
                key: name.to_string(),
                value: json,
            })
        }
    }
}

fn require_string<'a>(
    name: &str,
    m: &'a serde_yaml::Mapping,
    field: &str,
) -> Result<&'a str, FrontmatterError> {
    let v = m
        .get(YamlValue::String(field.into()))
        .ok_or_else(|| FrontmatterError::MissingField(name.into(), field.into()))?;
    match v {
        YamlValue::String(s) => Ok(s.as_str()),
        other => Err(FrontmatterError::WrongFieldType(
            name.into(),
            field.into(),
            format!("{:?}", other),
        )),
    }
}

/// Convert a [`serde_yaml::Value`] to a [`serde_json::Value`] via the common
/// self-describing serialization format. Both types implement `Serialize`,
/// and their serializations are compatible, so we round-trip through JSON.
fn yaml_to_json(name: &str, yaml: YamlValue) -> Result<serde_json::Value, FrontmatterError> {
    let s = serde_json::to_string(&yaml).map_err(|e| {
        FrontmatterError::WrongFieldType(
            name.into(),
            "value".into(),
            format!("custom value is not representable as JSON: {e}"),
        )
    })?;
    serde_json::from_str(&s).map_err(|e| FrontmatterError::InvalidYaml(e.to_string()))
}

/// Serialize a properties map back to YAML frontmatter text.
///
/// The output is suitable to be wrapped between `---\n...\n---\n` delimiters.
///
/// Keys are written in **sorted order** so the output is deterministic and
/// stable across round-trips through `parse_frontmatter_to_properties` →
/// `properties_to_yaml`. This matters for syncing/diffing files on disk.
pub fn properties_to_yaml(
    props: &HashMap<String, PropertyValue>,
) -> Result<String, FrontmatterError> {
    let mut root = serde_yaml::Mapping::new();
    // Sort keys for deterministic output.
    let mut keys: Vec<&String> = props.keys().collect();
    keys.sort();
    for key in keys {
        let value = props.get(key).expect("key present");
        let mut entry = serde_yaml::Mapping::new();
        match value {
            PropertyValue::Title { title } | PropertyValue::RichText { rich_text: title } => {
                let pt = match value {
                    PropertyValue::Title { .. } => PropertyType::Title,
                    _ => PropertyType::RichText,
                };
                entry.insert(
                    YamlValue::String("type".into()),
                    YamlValue::String(pt.as_tag().into()),
                );
                let s = inline_to_plain(title);
                entry.insert(YamlValue::String("value".into()), YamlValue::String(s));
            }
            PropertyValue::Number { number } => {
                entry.insert(
                    YamlValue::String("type".into()),
                    YamlValue::String(PropertyType::Number.as_tag().into()),
                );
                // Preserve a null number rather than silently coercing it
                // to 0; round-trip fidelity matters.
                let v = match number {
                    Some(n) => YamlValue::Number(serde_yaml::Number::from(*n)),
                    None => YamlValue::Null,
                };
                entry.insert(YamlValue::String("value".into()), v);
            }
            PropertyValue::Select { select } => {
                entry.insert(
                    YamlValue::String("type".into()),
                    YamlValue::String(PropertyType::Select.as_tag().into()),
                );
                entry.insert(
                    YamlValue::String("value".into()),
                    select
                        .as_ref()
                        .map(|o| YamlValue::String(o.name.clone()))
                        .unwrap_or(YamlValue::Null),
                );
            }
            PropertyValue::MultiSelect { multi_select } => {
                entry.insert(
                    YamlValue::String("type".into()),
                    YamlValue::String(PropertyType::MultiSelect.as_tag().into()),
                );
                let seq: Vec<YamlValue> = multi_select
                    .iter()
                    .map(|o| YamlValue::String(o.name.clone()))
                    .collect();
                entry.insert(YamlValue::String("values".into()), YamlValue::Sequence(seq));
            }
            PropertyValue::Date { date } => {
                entry.insert(
                    YamlValue::String("type".into()),
                    YamlValue::String(PropertyType::Date.as_tag().into()),
                );
                entry.insert(
                    YamlValue::String("value".into()),
                    date.as_ref()
                        .map(|d| YamlValue::String(d.start.clone()))
                        .unwrap_or(YamlValue::Null),
                );
            }
            PropertyValue::Checkbox { checkbox } => {
                entry.insert(
                    YamlValue::String("type".into()),
                    YamlValue::String(PropertyType::Checkbox.as_tag().into()),
                );
                entry.insert(
                    YamlValue::String("value".into()),
                    YamlValue::Bool(*checkbox),
                );
            }
            PropertyValue::Url { url } => {
                entry.insert(
                    YamlValue::String("type".into()),
                    YamlValue::String(PropertyType::Url.as_tag().into()),
                );
                entry.insert(
                    YamlValue::String("value".into()),
                    url.as_ref()
                        .map(|u| YamlValue::String(u.clone()))
                        .unwrap_or(YamlValue::Null),
                );
            }
            PropertyValue::Email { email } => {
                entry.insert(
                    YamlValue::String("type".into()),
                    YamlValue::String(PropertyType::Email.as_tag().into()),
                );
                entry.insert(
                    YamlValue::String("value".into()),
                    email
                        .as_ref()
                        .map(|e| YamlValue::String(e.clone()))
                        .unwrap_or(YamlValue::Null),
                );
            }
            // Dedicated branch for explicit `PropertyValue::Custom` payloads:
            // emit `type: custom` with the inner JSON value as YAML. This
            // keeps the wire shape identical to the catch-all below, but
            // avoids the cost (and potential double-wrapping) of re-running
            // `serde_json::to_value` on a tagged-enum serialization that
            // already contains a nested `type` field.
            PropertyValue::Custom { value, .. } => {
                entry.insert(
                    YamlValue::String("type".into()),
                    YamlValue::String(PropertyType::Custom.as_tag().into()),
                );
                let yaml_val = serde_yaml::to_value(value.clone()).unwrap_or(YamlValue::Null);
                entry.insert(YamlValue::String("value".into()), yaml_val);
            }
            // Property types not yet representable in frontmatter are
            // serialized as a Custom passthrough so we don't lose data.
            other => {
                entry.insert(
                    YamlValue::String("type".into()),
                    YamlValue::String("custom".into()),
                );
                let json = serde_json::to_value(other).unwrap_or(serde_json::Value::Null);
                let yaml_val = serde_yaml::to_value(json).unwrap_or(YamlValue::Null);
                entry.insert(YamlValue::String("value".into()), yaml_val);
            }
        }
        root.insert(YamlValue::String(key.clone()), YamlValue::Mapping(entry));
    }

    if root.is_empty() {
        return Ok(String::new());
    }
    let yaml = serde_yaml::to_string(&YamlValue::Mapping(root))
        .map_err(|e| FrontmatterError::InvalidYaml(e.to_string()))?;
    Ok(yaml)
}

fn inline_to_plain(inlines: &[InlineElement]) -> String {
    let mut s = String::new();
    for el in inlines {
        match el {
            InlineElement::TextRun { content, .. } => s.push_str(content),
            InlineElement::Equation { expression, .. } => s.push_str(expression),
            InlineElement::Mention { label, target, .. } => {
                s.push_str(label.as_deref().unwrap_or(target));
            }
            InlineElement::HardBreak => s.push('\n'),
            InlineElement::SoftBreak => s.push(' '),
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_parse_and_serialize() {
        let yaml = "title:\n  type: title\n  value: \"Hello\"\n";
        let m = parse_frontmatter_to_properties(yaml).unwrap();
        assert!(m.contains_key("title"));
        let out = properties_to_yaml(&m).unwrap();
        assert!(out.contains("type: title"));
        assert!(out.contains("Hello"));
    }
}
