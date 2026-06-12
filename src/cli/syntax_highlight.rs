use ratatui::style::{Color, Style};
use syntect::{easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet};

#[allow(dead_code)]
pub struct SyntaxHighlighter {
    ps: SyntaxSet,
    ts: ThemeSet,
}

impl SyntaxHighlighter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            ps: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_defaults(),
        }
    }

    #[allow(dead_code)]
    pub fn highlight(&self, code: &str, lang: &str, theme_name: &str) -> Vec<Vec<(Style, String)>> {
        let syntax = self
            .ps
            .find_syntax_by_token(lang)
            .or_else(|| self.ps.find_syntax_by_extension(lang))
            .unwrap_or_else(|| self.ps.find_syntax_plain_text());

        let theme = self
            .ts
            .themes
            .get(theme_name)
            .unwrap_or_else(|| &self.ts.themes["base16-ocean.dark"]);

        let mut highlighter = HighlightLines::new(syntax, theme);
        let mut lines = Vec::new();

        for line in code.lines() {
            let highlighted = highlighter
                .highlight_line(line, &self.ps)
                .unwrap_or_default();
            let mut styled_line = Vec::new();
            for (style, text) in highlighted {
                let ratatui_style = Style::default()
                    .fg(Color::Rgb(
                        style.foreground.r,
                        style.foreground.g,
                        style.foreground.b,
                    ))
                    .bg(Color::Rgb(
                        style.background.r,
                        style.background.g,
                        style.background.b,
                    ));
                if style
                    .font_style
                    .contains(syntect::highlighting::FontStyle::BOLD)
                {
                    styled_line.push((
                        ratatui_style.add_modifier(ratatui::style::Modifier::BOLD),
                        text.to_string(),
                    ));
                } else if style
                    .font_style
                    .contains(syntect::highlighting::FontStyle::ITALIC)
                {
                    styled_line.push((
                        ratatui_style.add_modifier(ratatui::style::Modifier::ITALIC),
                        text.to_string(),
                    ));
                } else if style
                    .font_style
                    .contains(syntect::highlighting::FontStyle::UNDERLINE)
                {
                    styled_line.push((
                        ratatui_style.add_modifier(ratatui::style::Modifier::UNDERLINED),
                        text.to_string(),
                    ));
                } else {
                    styled_line.push((ratatui_style, text.to_string()));
                }
            }
            lines.push(styled_line);
        }
        lines
    }

    #[allow(dead_code)]
    pub fn available_themes(&self) -> Vec<String> {
        self.ts.themes.keys().cloned().collect()
    }

    #[allow(dead_code)]
    pub fn available_languages(&self) -> Vec<String> {
        self.ps.syntaxes().iter().map(|s| s.name.clone()).collect()
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}
