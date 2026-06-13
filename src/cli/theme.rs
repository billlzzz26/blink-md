use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowsTerminalScheme {
    pub name: String,
    pub foreground: String,
    pub background: String,
    pub cursor_color: Option<String>,
    pub selection_background: Option<String>,
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub purple: String,
    pub cyan: String,
    pub white: String,
    pub bright_black: String,
    pub bright_red: String,
    pub bright_green: String,
    pub bright_yellow: String,
    pub bright_blue: String,
    pub bright_purple: String,
    pub bright_cyan: String,
    pub bright_white: String,
}

pub struct Theme {
    pub primary: Color,
    pub accent: Color,
    pub surface: Color,
    pub primary_text: Color,
    pub accent_text: Color,
    pub stone_gray: Color,
    pub error: Color,
    pub success: Color,
    // ANSI colors
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub purple: Color,
    pub cyan: Color,
    pub white: Color,
}

impl Theme {
    pub fn load(name: &str) -> Result<Self, anyhow::Error> {
        let path = format!("src/cli/themes/{}.json", name);
        let content = std::fs::read_to_string(path)?;
        let scheme: WindowsTerminalScheme = serde_json::from_str(&content)?;
        Ok(Self::from_windows_terminal(&scheme))
    }

    pub fn from_windows_terminal(scheme: &WindowsTerminalScheme) -> Self {
        Self {
            primary: parse_hex(&scheme.background).unwrap_or(Color::Reset),
            accent: parse_hex(&scheme.blue).unwrap_or(Color::Blue),
            surface: parse_hex(&scheme.background).unwrap_or(Color::Reset),
            primary_text: parse_hex(&scheme.foreground).unwrap_or(Color::White),
            accent_text: parse_hex(&scheme.background).unwrap_or(Color::Black),
            stone_gray: parse_hex(&scheme.bright_black).unwrap_or(Color::Gray),
            error: parse_hex(&scheme.red).unwrap_or(Color::Red),
            success: parse_hex(&scheme.green).unwrap_or(Color::Green),
            black: parse_hex(&scheme.black).unwrap_or(Color::Black),
            red: parse_hex(&scheme.red).unwrap_or(Color::Red),
            green: parse_hex(&scheme.green).unwrap_or(Color::Green),
            yellow: parse_hex(&scheme.yellow).unwrap_or(Color::Yellow),
            blue: parse_hex(&scheme.blue).unwrap_or(Color::Blue),
            purple: parse_hex(&scheme.purple).unwrap_or(Color::Magenta),
            cyan: parse_hex(&scheme.cyan).unwrap_or(Color::Cyan),
            white: parse_hex(&scheme.white).unwrap_or(Color::White),
        }
    }

    pub fn notion() -> Self {
        Self::load("notion").unwrap_or_else(|_| Self {
            primary: Color::Rgb(55, 53, 47),
            accent: Color::Rgb(35, 131, 226),
            surface: Color::White,
            primary_text: Color::Rgb(55, 53, 47),
            accent_text: Color::White,
            stone_gray: Color::Rgb(155, 154, 151),
            error: Color::Rgb(212, 76, 71),
            success: Color::Rgb(68, 131, 97),
            black: Color::Rgb(55, 53, 47),
            red: Color::Rgb(212, 76, 71),
            green: Color::Rgb(68, 131, 97),
            yellow: Color::Rgb(203, 145, 47),
            blue: Color::Rgb(35, 131, 226),
            purple: Color::Rgb(144, 101, 176),
            cyan: Color::Rgb(51, 126, 169),
            white: Color::White,
        })
    }
}

pub fn parse_hex(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color::Rgb(r, g, b))
}
