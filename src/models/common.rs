//! Common types shared across all Notion API models.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// An object identifier (UUID format).
pub type ObjectId = String;

/// A Notion user (person or bot).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct User {
    /// Always `"user"`.
    pub object: String,
    /// Unique identifier.
    pub id: ObjectId,
    /// Whether this user is a person or a bot.
    #[serde(flatten)]
    pub user_type: UserType,
    /// Display name (null for bots without a name).
    pub name: Option<String>,
    /// URL of the user's avatar image.
    pub avatar_url: Option<String>,
}

/// The type discriminator for [`User`].
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UserType {
    /// A human user with an email address.
    Person { person: PersonInfo },
    /// A bot integration.
    Bot { bot: BotInfo },
}

/// Information about a person user.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonInfo {
    /// The user's email address.
    pub email: Option<String>,
}

/// Information about a bot user.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BotInfo {
    /// Optional owner of the bot.
    pub owner: Option<Owner>,
    /// The name of the workspace the bot is integrated with.
    pub workspace_name: Option<String>,
}

/// The owner of a bot.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Owner {
    /// Owned by the workspace.
    Workspace { workspace: bool },
    /// Owned by a specific user (Box to break recursion).
    User { user: Box<User> },
}

/// The parent container for a Notion resource.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parent {
    /// Which kind of parent this is.
    #[serde(flatten)]
    pub parent_type: ParentType,
}

/// The type discriminator for [`Parent`].
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ParentType {
    /// A page inside a database.
    DatabaseId { database_id: ObjectId },
    /// A block inside a page.
    PageId { page_id: ObjectId },
    /// A block inside another block.
    BlockId { block_id: ObjectId },
    /// A top-level workspace resource.
    Workspace { workspace: bool },
}

/// A rich text element returned by Notion.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RichText {
    /// Plain text with optional link and annotations.
    Text {
        text: TextContent,
        #[serde(default)]
        annotations: Option<Annotations>,
        plain_text: Option<String>,
        href: Option<String>,
    },
    /// A mention of a user, page, database, or date.
    Mention {
        mention: MentionObject,
        #[serde(default)]
        annotations: Option<Annotations>,
        plain_text: Option<String>,
        href: Option<String>,
    },
    /// An inline LaTeX equation.
    Equation {
        equation: EquationContent,
        #[serde(default)]
        annotations: Option<Annotations>,
        plain_text: Option<String>,
        href: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextContent {
    pub content: String,
    pub link: Option<Link>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EquationContent {
    pub expression: String,
}

/// A URL link attached to rich text.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Link {
    /// The target URL.
    pub url: String,
}

/// Text formatting annotations.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Annotations {
    /// Bold text.
    pub bold: bool,
    /// Italic text.
    pub italic: bool,
    /// Strikethrough text.
    pub strikethrough: bool,
    /// Underlined text.
    pub underline: bool,
    /// Monospace code format.
    pub code: bool,
    /// Color identifier (e.g. `"default"`, `"blue_background"`).
    pub color: String,
}

/// A mention target inside rich text.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum MentionObject {
    /// Mention of a user.
    User { user: User },
    /// Mention of a page.
    Page { page: PageMention },
    /// Mention of a database.
    Database { database: DatabaseMention },
    /// Mention of a date range.
    Date { date: NaiveDate },
    /// Mention of a URL preview.
    LinkPreview { url: String },
}

/// A page reference used in mentions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageMention {
    /// The page ID.
    pub id: ObjectId,
}

/// A database reference used in mentions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseMention {
    /// The database ID.
    pub id: ObjectId,
}

/// File or image block content shared across models.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileBlockContent {
    #[serde(flatten)]
    pub file_type: FileType,
}

/// The type of file (external URL or uploaded file).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum FileType {
    /// An external URL (e.g. images hosted elsewhere).
    External { external: ExternalFile },
    /// A file uploaded to Notion.
    Uploaded { file: UploadedFile },
}

/// An external file reference with a URL.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternalFile {
    /// The file URL.
    pub url: String,
}

/// A file uploaded to Notion.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UploadedFile {
    /// The file download URL.
    pub url: String,
    /// When the URL expires (for uploaded files).
    pub expiry_time: Option<chrono::DateTime<chrono::Utc>>,
}

/// An icon on a page, database, or callout block.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Icon {
    /// An emoji icon (e.g. `"📝"`).
    Emoji { emoji: String },
    /// An external image URL.
    External { external: ExternalFile },
    /// A file uploaded to Notion.
    File { file: UploadedFile },
}
