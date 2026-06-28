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
            let val = require_string(name, m, "value")?;
            Ok(PropertyValue::Select {
                select: Some(SelectOption {
                    id: None,
                    name: val.to_string(),
                    color: None,
                }),
            })
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
            let val = require_string(name, m, "value")?;
            Ok(PropertyValue::Date {
                date: Some(crate::ir::metadata::DateValue {
                    start: val.to_string(),
                    end: None,
                    time_zone: None,
                }),
            })
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
            let val = require_string(name, m, "value")?;
            Ok(PropertyValue::Url {
                url: Some(val.to_string()),
            })
        }
        PropertyType::Email => {
            let val = require_string(name, m, "value")?;
            Ok(PropertyValue::Email {
                email: Some(val.to_string()),
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

/// Serialize a properties map back to YAML frontmatter text.
///
/// The output is suitable to be wrapped between `---\n...\n---\n` delimiters.
pub fn properties_to_yaml(
    props: &HashMap<String, PropertyValue>,
) -> Result<String, FrontmatterError> {
    let mut root = serde_yaml::Mapping::new();
    for (key, value) in props {
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
                let v = match number {
                    Some(n) => serde_yaml::Number::from(*n),
                    None => serde_yaml::Number::from(0),
                };
                entry.insert(YamlValue::String("value".into()), YamlValue::Number(v));
            }
            PropertyValue::Select { select } => {
                entry.insert(
                    YamlValue::String("type".into()),
                    YamlValue::String(PropertyType::Select.as_tag().into()),
                );
                let name = select.as_ref().map(|o| o.name.clone()).unwrap_or_default();
                entry.insert(YamlValue::String("value".into()), YamlValue::String(name));
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
                let s = date.as_ref().map(|d| d.start.clone()).unwrap_or_default();
                entry.insert(YamlValue::String("value".into()), YamlValue::String(s));
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
                    YamlValue::String(url.clone().unwrap_or_default()),
                );
            }
            PropertyValue::Email { email } => {
                entry.insert(
                    YamlValue::String("type".into()),
                    YamlValue::String(PropertyType::Email.as_tag().into()),
                );
                entry.insert(
                    YamlValue::String("value".into()),
                    YamlValue::String(email.clone().unwrap_or_default()),
                );
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
        if let InlineElement::TextRun { content, .. } = el {
            s.push_str(content);
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
