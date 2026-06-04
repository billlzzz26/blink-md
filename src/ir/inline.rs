//! Inline Elements (Rich Text)
//!
//! Represents inline formatting within blocks: text runs, mentions, equations, links

use crate::ir::MentionType;
use serde::{Deserialize, Serialize};

/// Inline element within a block
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InlineElement {
    /// Styled text run
    TextRun {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<TextStyle>,
    },
    /// Mention of user, page, database, date, etc.
    Mention {
        mention_type: MentionType,
        target: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<TextStyle>,
    },
    /// Inline LaTeX equation
    Equation {
        expression: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        style: Option<TextStyle>,
    },
    /// Hard line break
    HardBreak,
    /// Soft line break (space)
    SoftBreak,
}

/// Text formatting style
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TextStyle {
    /// Bold text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    /// Italic text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    /// Strikethrough text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    /// Underlined text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    /// Inline code formatting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<bool>,
    /// Color identifier (e.g., "default", "blue", "red_background")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Link URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

impl TextStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bold(mut self) -> Self {
        self.bold = Some(true);
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = Some(true);
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = Some(true);
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = Some(true);
        self
    }

    pub fn code(mut self) -> Self {
        self.code = Some(true);
        self
    }

    pub fn color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn link(mut self, url: impl Into<String>) -> Self {
        self.link = Some(url.into());
        self
    }
}

/// Helper for creating text runs
pub fn text(content: impl Into<String>) -> InlineElement {
    InlineElement::TextRun {
        content: content.into(),
        style: None,
    }
}

pub fn styled_text(content: impl Into<String>, style: TextStyle) -> InlineElement {
    InlineElement::TextRun {
        content: content.into(),
        style: Some(style),
    }
}

/// Helper for creating mentions
pub fn mention(mention_type: MentionType, target: impl Into<String>) -> InlineElement {
    InlineElement::Mention {
        mention_type,
        target: target.into(),
        label: None,
        style: None,
    }
}

pub fn labeled_mention(
    mention_type: MentionType,
    target: impl Into<String>,
    label: impl Into<String>,
) -> InlineElement {
    InlineElement::Mention {
        mention_type,
        target: target.into(),
        label: Some(label.into()),
        style: None,
    }
}

/// Helper for equations
pub fn equation(expr: impl Into<String>) -> InlineElement {
    InlineElement::Equation {
        expression: expr.into(),
        style: None,
    }
}

pub fn hard_break() -> InlineElement {
    InlineElement::HardBreak
}

pub fn soft_break() -> InlineElement {
    InlineElement::SoftBreak
}
