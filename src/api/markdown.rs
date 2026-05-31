use crate::models::block::{Block, BlockType, HeadingContent, TextBlockContent, ToDoContent, CalloutContent};
use crate::models::common::{RichText, TextContent, UserType, BotInfo, Annotations, Icon};
use pulldown_cmark::{Parser, Event, Tag, TagEnd, Options};
use chrono::Utc;

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
                    Icon::Emoji { emoji } => emoji.clone(),
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

struct ParserState {
    bold: bool,
    italic: bool,
    strikethrough: bool,
    code: bool,
    underline: bool,
    color: String,
    container_stack: Vec<ContainerType>,
    current_todo_checked: Option<bool>,
    current_callout_icon: Option<String>,
    current_callout_color: Option<String>,
}

enum ContainerType {
    BulletedList,
    NumberedList,
    Quote,
    Callout,
}

impl ParserState {
    fn new() -> Self {
        Self {
            bold: false,
            italic: false,
            strikethrough: false,
            code: false,
            underline: false,
            color: "default".to_string(),
            container_stack: Vec::new(),
            current_todo_checked: None,
            current_callout_icon: None,
            current_callout_color: None,
        }
    }

    fn to_annotations(&self) -> Annotations {
        Annotations {
            bold: self.bold,
            italic: self.italic,
            strikethrough: self.strikethrough,
            underline: self.underline,
            code: self.code,
            color: self.color.clone(),
        }
    }

    fn is_in_list(&self) -> bool {
        self.container_stack.iter().any(|c| matches!(c, ContainerType::BulletedList | ContainerType::NumberedList))
    }

    fn is_in_quote(&self) -> bool {
        self.container_stack.iter().any(|c| matches!(c, ContainerType::Quote))
    }

    fn is_in_callout(&self) -> bool {
        self.container_stack.iter().any(|c| matches!(c, ContainerType::Callout))
    }
}

/// Parses a Markdown string into a sequence of Notion blocks.
pub fn parse_markdown(md: &str) -> Vec<Block> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(md, options);
    let mut blocks = Vec::new();
    let mut current_rich_text = Vec::new();
    let mut state = ParserState::new();

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Strong => state.bold = true,
                Tag::Emphasis => state.italic = true,
                Tag::Strikethrough => state.strikethrough = true,
                Tag::CodeBlock(_) => state.code = true,
                Tag::List(first_item_number) => {
                    if first_item_number.is_some() {
                        state.container_stack.push(ContainerType::NumberedList);
                    } else {
                        state.container_stack.push(ContainerType::BulletedList);
                    }
                }
                Tag::BlockQuote(_) => state.container_stack.push(ContainerType::Quote),
                _ => {}
            },
            Event::End(tag_end) => match tag_end {
                TagEnd::Strong => state.bold = false,
                TagEnd::Emphasis => state.italic = false,
                TagEnd::Strikethrough => state.strikethrough = false,
                TagEnd::CodeBlock => state.code = false,
                TagEnd::List(_is_ordered) => {
                    state.container_stack.pop();
                }
                TagEnd::BlockQuote(_) => {
                    state.container_stack.pop();
                    blocks.push(create_block(BlockType::Quote {
                        quote: TextBlockContent {
                            rich_text: current_rich_text.drain(..).collect(),
                            color: "default".to_string(),
                            children: None,
                        },
                    }));
                }
                TagEnd::Paragraph => {
                    if !state.is_in_list() && !state.is_in_quote() && !state.is_in_callout() {
                        blocks.push(create_block(BlockType::Paragraph {
                            paragraph: TextBlockContent {
                                rich_text: current_rich_text.drain(..).collect(),
                                color: "default".to_string(),
                                children: None,
                            },
                        }));
                    } else if state.is_in_quote() || state.is_in_callout() {
                        // In a quote/callout, we add a newline if there's already text
                        if !current_rich_text.is_empty() {
                            current_rich_text.push(RichText::Text {
                                text: TextContent { content: "\n".into(), link: None },
                                annotations: Some(state.to_annotations()),
                                plain_text: Some("\n".into()),
                                href: None,
                            });
                        }
                    }
                }
                TagEnd::Item => {
                    let block_type = if let Some(checked) = state.current_todo_checked.take() {
                        BlockType::ToDo {
                            to_do: ToDoContent {
                                rich_text: current_rich_text.drain(..).collect(),
                                checked,
                                color: "default".to_string(),
                                children: None,
                            },
                        }
                    } else {
                        match state.container_stack.last() {
                            Some(ContainerType::NumberedList) => BlockType::NumberedListItem {
                                numbered_list_item: TextBlockContent {
                                    rich_text: current_rich_text.drain(..).collect(),
                                    color: "default".to_string(),
                                    children: None,
                                },
                            },
                            _ => BlockType::BulletedListItem {
                                bulleted_list_item: TextBlockContent {
                                    rich_text: current_rich_text.drain(..).collect(),
                                    color: "default".to_string(),
                                    children: None,
                                },
                            },
                        }
                    };
                    blocks.push(create_block(block_type));
                }
                TagEnd::Heading(level) => {
                    let block_type = match level {
                        pulldown_cmark::HeadingLevel::H1 => BlockType::Heading1 {
                            heading_1: HeadingContent {
                                rich_text: current_rich_text.drain(..).collect(),
                                color: "default".to_string(),
                                is_toggleable: false,
                                children: None,
                            },
                        },
                        pulldown_cmark::HeadingLevel::H2 => BlockType::Heading2 {
                            heading_2: HeadingContent {
                                rich_text: current_rich_text.drain(..).collect(),
                                color: "default".to_string(),
                                is_toggleable: false,
                                children: None,
                            },
                        },
                        _ => BlockType::Heading3 {
                            heading_3: HeadingContent {
                                rich_text: current_rich_text.drain(..).collect(),
                                color: "default".to_string(),
                                is_toggleable: false,
                                children: None,
                            },
                        },
                    };
                    blocks.push(create_block(block_type));
                }
                _ => {}
            },
            Event::Text(text) => {
                current_rich_text.push(RichText::Text {
                    text: TextContent {
                        content: text.to_string(),
                        link: None,
                    },
                    annotations: Some(state.to_annotations()),
                    plain_text: Some(text.to_string()),
                    href: None,
                });
            }
            Event::Code(code) => {
                current_rich_text.push(RichText::Text {
                    text: TextContent {
                        content: code.to_string(),
                        link: None,
                    },
                    annotations: Some(Annotations {
                        code: true,
                        ..state.to_annotations()
                    }),
                    plain_text: Some(code.to_string()),
                    href: None,
                });
            }
            Event::TaskListMarker(checked) => {
                state.current_todo_checked = Some(checked);
            }
            Event::Rule => {
                blocks.push(create_block(BlockType::Divider {}));
            }
            Event::Html(html) => {
                if html.starts_with("<callout") {
                    state.container_stack.push(ContainerType::Callout);
                    // Extract icon and color (simple parsing)
                    if let Some(icon) = extract_attr(&html, "icon") {
                        state.current_callout_icon = Some(icon);
                    }
                    if let Some(color) = extract_attr(&html, "color") {
                        state.current_callout_color = Some(color);
                    }
                } else if html.starts_with("</callout>") {
                    state.container_stack.pop();
                    let icon = state.current_callout_icon.take().map(|e| Icon::Emoji { emoji: e });
                    let color = state.current_callout_color.take().unwrap_or_else(|| "default".to_string());
                    
                    blocks.push(create_block(BlockType::Callout {
                        callout: CalloutContent {
                            rich_text: current_rich_text.drain(..).collect(),
                            icon,
                            color,
                            children: None,
                        },
                    }));
                }
            }
            _ => {}
        }
    }

    blocks
}

fn extract_attr(html: &str, attr: &str) -> Option<String> {
    let pattern = format!("{}=\"", attr);
    if let Some(start) = html.find(&pattern) {
        let start = start + pattern.len();
        if let Some(end) = html[start..].find('"') {
            return Some(html[start..start+end].to_string());
        }
    }
    None
}

fn create_block(block_type: BlockType) -> Block {
    Block {
        object: "block".to_string(),
        id: "temp-id".to_string(),
        created_time: Utc::now(),
        last_edited_time: Utc::now(),
        created_by: dummy_user(),
        last_edited_by: dummy_user(),
        has_children: false,
        in_trash: false,
        parent: None,
        block_type,
    }
}

fn dummy_user() -> crate::models::common::User {
    crate::models::common::User {
        object: "user".to_string(),
        id: "dummy".to_string(),
        name: None,
        avatar_url: None,
        user_type: UserType::Bot { 
            bot: BotInfo { 
                owner: None, 
                workspace_name: None 
            } 
        },
    }
}
