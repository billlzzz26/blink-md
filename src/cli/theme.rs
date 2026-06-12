use ratatui::style::Color;

pub struct Theme {
    #[allow(dead_code)]
    pub primary: Color,
    pub accent: Color,
    pub surface: Color,
    pub primary_text: Color,
    #[allow(dead_code)]
    pub accent_text: Color,
    #[allow(dead_code)]
    pub stone_gray: Color,
    #[allow(dead_code)]
    pub error: Color,
    #[allow(dead_code)]
    pub success: Color,
}

impl Theme {
    pub fn notion() -> Self {
        Self {
            // Primary text: #37352F
            primary: Color::Rgb(55, 53, 47),
            // Accent Blue: #2383E2
            accent: Color::Rgb(35, 131, 226),
            // Surface: #FFFFFF
            surface: Color::White,
            // Standard Text
            primary_text: Color::Rgb(55, 53, 47),
            // Accent Text (for highlights)
            accent_text: Color::White,
            // Stone Gray: #9B9A97 (for inactive tabs/items)
            stone_gray: Color::Rgb(155, 154, 151),
            // Error Color: #D44C47
            error: Color::Rgb(212, 76, 71),
            // Success Color: #448361
            success: Color::Rgb(68, 131, 97),
        }
    }
}
