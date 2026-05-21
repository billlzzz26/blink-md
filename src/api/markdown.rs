use crate::models::block::{Block, BlockType, HeadingContent, TextBlockContent, ToDoContent};
use crate::models::common::RichText;

/// A trait for types that can be rendered as Notion-flavored Markdown.
pub trait ToMarkdown {
    /// Render the object as a Markdown string with the given indentation level.
    fn to_markdown(&self, indent: usize) -> String;
}

impl ToMarkdown for Vec<RichText> {
    fn to_markdown(&self, _indent: usize) -> String {
        let mut result = String::new();
        for rich in self {
            match rich {
                RichText::Text {
                    text,
                    annotations,
                    ..
                } => {
                    let mut content = text.content.clone();
                    
                    // Apply annotations
                    if let Some(ann) = annotations {
                        if ann.bold {
                            content = format!("**{}**", content);
                        }
                        if ann.italic {
                            content = format!("*{}*", content);
                        }
                        if ann.strikethrough {
                            content = format!("~~{}~~", content);
                        }
                        if ann.code {
                            content = format!("`{}`", content);
                        }
                        if ann.underline {
                            content = format!("<span underline=\"true\">{}</span>", content);
                        }
                        if ann.color != "default" {
                            content = format!("<span color=\"{}\">{}</span>", ann.color, content);
                        }
                    }

                    // Apply link
                    if let Some(l) = &text.link {
                        content = format!("[{}]({})", content, l.url);
                    }

                    result.push_str(&content);
                }
                RichText::Mention { .. } => {
                    // TODO: Implement mentions properly based on spec
                    result.push_str("[Mention]");
                }
                RichText::Equation { equation, .. } => {
                    result.push_str(&format!("$`{}`$", equation.expression));
                }
            }
        }
        result
    }
}

impl ToMarkdown for Block {
    fn to_markdown(&self, indent: usize) -> String {
        let tabs = "\t".repeat(indent);
        let mut result = String::new();

        match &self.block_type {
            BlockType::Paragraph { paragraph } => {
                result.push_str(&format!("{}{}", tabs, render_text_content(paragraph, indent)));
            }
            BlockType::Heading1 { heading_1 } => {
                result.push_str(&format!("{}# {}", tabs, render_heading_content(heading_1, indent)));
            }
            BlockType::Heading2 { heading_2 } => {
                result.push_str(&format!("{}## {}", tabs, render_heading_content(heading_2, indent)));
            }
            BlockType::Heading3 { heading_3 } => {
                result.push_str(&format!("{}### {}", tabs, render_heading_content(heading_3, indent)));
            }
            BlockType::BulletedListItem { bulleted_list_item } => {
                result.push_str(&format!("{}- {}", tabs, render_text_content(bulleted_list_item, indent)));
            }
            BlockType::NumberedListItem { numbered_list_item } => {
                result.push_str(&format!("{}1. {}", tabs, render_text_content(numbered_list_item, indent)));
            }
            BlockType::ToDo { to_do } => {
                let check = if to_do.checked { "x" } else { " " };
                result.push_str(&format!("{}- [{}] {}", tabs, check, render_todo_content(to_do, indent)));
            }
            BlockType::Divider {} => {
                result.push_str(&format!("{}---", tabs));
            }
            BlockType::Quote { quote } => {
                result.push_str(&format!("{}> {}", tabs, render_text_content(quote, indent)));
            }
            BlockType::Callout { callout } => {
                let icon_str = callout.icon.as_ref().map(|i| match i {
                    crate::models::common::Icon::Emoji { emoji } => emoji.clone(),
                    _ => "ℹ️".to_string(), // Default
                }).unwrap_or_else(|| "ℹ️".to_string());
                
                result.push_str(&format!("{}<callout icon=\"{}\" color=\"{}\">\n", tabs, icon_str, callout.color));
                result.push_str(&format!("{}\t{}\n", tabs, callout.rich_text.to_markdown(0)));
                if let Some(children) = &callout.children {
                    for child in children {
                        result.push_str(&child.to_markdown(indent + 1));
                        result.push('\n');
                    }
                }
                result.push_str(&format!("{}</callout>", tabs));
            }
            _ => {
                result.push_str(&format!("{}<!-- Unsupported block type: {} -->", tabs, self.type_str()));
            }
        }

        result
    }
}

fn render_text_content(content: &TextBlockContent, indent: usize) -> String {
    let mut out = content.rich_text.to_markdown(0);
    if content.color != "default" {
        out = format!("{} {{color=\"{}\"}}", out, content.color);
    }
    if let Some(children) = &content.children {
        out.push('\n');
        for child in children {
            out.push_str(&child.to_markdown(indent + 1));
            out.push('\n');
        }
    }
    out
}

fn render_heading_content(content: &HeadingContent, indent: usize) -> String {
    let mut out = content.rich_text.to_markdown(0);
    let mut attrs = vec![];
    if content.color != "default" {
        attrs.push(format!("color=\"{}\"", content.color));
    }
    if content.is_toggleable {
        attrs.push("toggle=\"true\"".to_string());
    }
    
    if !attrs.is_empty() {
        out = format!("{} {{{}}}", out, attrs.join(" "));
    }

    if let Some(children) = &content.children {
        out.push('\n');
        for child in children {
            out.push_str(&child.to_markdown(indent + 1));
            out.push('\n');
        }
    }
    out
}

fn render_todo_content(content: &ToDoContent, indent: usize) -> String {
    let mut out = content.rich_text.to_markdown(0);
    if content.color != "default" {
        out = format!("{} {{color=\"{}\"}}", out, content.color);
    }
    if let Some(children) = &content.children {
        out.push('\n');
        for child in children {
            out.push_str(&child.to_markdown(indent + 1));
            out.push('\n');
        }
    }
    out
}
