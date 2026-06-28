//! Markdown converter that also recognises a YAML frontmatter block at
//! the start of the input and round-trips it through
//! `crate::ir::metadata::DocumentMetadata::properties`.
//!
//! This is **not** a new block-level converter; it composes:
//!
//! 1. [`crate::api::markdown_frontmatter::detect_frontmatter`] to split
//!    a leading `---`-delimited YAML block from the Markdown body.
//! 2. [`crate::ir::frontmatter::parse_frontmatter_to_properties`] /
//!    [`crate::ir::frontmatter::properties_to_yaml`] to convert YAML ↔
//!    `PropertyValue` map (explicit `type:` tagged YAML).
//! 3. [`MarkdownConverter`] to convert the Markdown body ↔ blocks.
//!
//! Output format produced by [`to_platform`]:
//!
//! ```text
//! ---
//! title:
//!   type: title
//!   value: "My Page"
//! ---
//! # Heading
//!
//! Body.
//! ```
//!
//! If the input has no frontmatter block (or only a body), the YAML
//! header is omitted entirely. If the input has only a frontmatter
//! block (empty body), the closing `---` is still emitted and the
//! document is well-formed.

use crate::api::markdown_frontmatter::{
    detect_frontmatter, FrontmatterBlock, FrontmatterError as ApiFrontmatterError,
};
use crate::converter::markdown::MarkdownConverter;
use crate::converter::{ConverterError, FromPlatform, ToPlatform};
use crate::ir::frontmatter::{
    parse_frontmatter_to_properties, properties_to_yaml, FrontmatterError as IrFrontmatterError,
};
use crate::ir::{Platform, UniversalDocument};

/// Converter for Markdown that may carry a YAML frontmatter block.
///
/// Implements [`FromPlatform`] and [`ToPlatform`] over `String` input/output.
pub struct MarkdownWithFrontmatterConverter;

impl FromPlatform for MarkdownWithFrontmatterConverter {
    const PLATFORM: Platform = Platform::Markdown;
    type Input = String;

    fn from_platform(input: Self::Input) -> Result<UniversalDocument, ConverterError> {
        let (yaml_text, body) = match detect_frontmatter(&input)
            .map_err(|e| ConverterError::InvalidData(format!("frontmatter: {}", e)))?
        {
            Some(FrontmatterBlock { yaml, content }) => (yaml, content),
            None => (String::new(), input),
        };

        // Always start from MarkdownConverter for the body so we inherit its
        // block conversion behaviour; for an empty body we still produce a
        // valid (empty-blocks) UniversalDocument and merge properties below.
        let mut doc = MarkdownConverter::from_platform(body)?;

        if !yaml_text.trim().is_empty() {
            let properties = parse_frontmatter_to_properties(&yaml_text)
                .map_err(frontmatter_err_to_converter_err)?;
            doc.metadata.properties.extend(properties);
        }

        Ok(doc)
    }
}

impl ToPlatform for MarkdownWithFrontmatterConverter {
    const PLATFORM: Platform = Platform::Markdown;
    type Output = String;

    fn to_platform(doc: &UniversalDocument) -> Result<Self::Output, ConverterError> {
        let mut out = String::new();
        if !doc.metadata.properties.is_empty() {
            let yaml = properties_to_yaml(&doc.metadata.properties)
                .map_err(frontmatter_err_to_converter_err)?;
            if !yaml.trim().is_empty() {
                out.push_str("---\n");
                out.push_str(&yaml);
                if !out.ends_with("---\n") {
                    if out.ends_with('\n') {
                        out.pop();
                    }
                    out.push_str("\n---\n");
                } else {
                    // `properties_to_yaml` already ends with `\n`; the closing
                    // delimiter line must be added separately.
                    out.push_str("---\n");
                }
            }
        }
        let body = MarkdownConverter::to_platform(doc)?;
        out.push_str(&body);
        out.push('\n');
        Ok(out)
    }
}

fn frontmatter_err_to_converter_err(e: IrFrontmatterError) -> ConverterError {
    let prefix = match &e {
        IrFrontmatterError::InvalidYaml(_) => "invalid YAML",
        IrFrontmatterError::UnknownPropertyType(_) => "unknown property type",
        IrFrontmatterError::MissingField(_, _) => "missing frontmatter field",
        IrFrontmatterError::WrongFieldType(_, _, _) => "wrong frontmatter field type",
    };
    ConverterError::ConversionFailed(format!("{}: {}", prefix, e))
}

// Note: the `From<ApiFrontmatterError> for ConverterError` is kept here for
// future use by callers that want to differentiate the empty-block edge case
// without the body-branch shortcut.
impl From<ApiFrontmatterError> for ConverterError {
    fn from(e: ApiFrontmatterError) -> Self {
        ConverterError::InvalidData(format!("frontmatter: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_roundtrip_short() {
        let input = "---\ntitle:\n  type: title\n  value: \"X\"\n---\n# Hi\n".to_string();
        let doc = MarkdownWithFrontmatterConverter::from_platform(input).unwrap();
        let out = MarkdownWithFrontmatterConverter::to_platform(&doc).unwrap();
        assert!(out.starts_with("---\n"));
        assert!(out.contains("type: title"));
        assert!(out.contains("# Hi"));
    }
}
