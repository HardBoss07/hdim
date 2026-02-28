use ratatui::style::Color;

pub struct Palette {
    pub background: Color,
    pub foreground: Color,
    pub surface: Color,
    pub border: Color,
    pub accent: Color,
    pub accent_dim: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub muted: Color,
}

pub const ZINC_PALETTE: Palette = Palette {
    background: Color::Rgb(24, 24, 27),    // Zinc 950
    foreground: Color::Rgb(250, 250, 250), // Zinc 50
    surface: Color::Rgb(39, 39, 42),       // Zinc 800
    border: Color::Rgb(63, 63, 70),        // Zinc 700
    accent: Color::Rgb(16, 185, 129),      // Emerald 500
    accent_dim: Color::Rgb(6, 95, 70),     // Emerald 800
    success: Color::Rgb(34, 197, 94),      // Green 500
    warning: Color::Rgb(234, 179, 8),      // Yellow 500
    error: Color::Rgb(239, 68, 68),        // Red 500
    muted: Color::Rgb(113, 113, 122),      // Zinc 500
};

pub const THEME: Palette = ZINC_PALETTE;
