use ratatui::style::Color;

pub struct Palette {
    pub background: Color,
    pub foreground: Color,
    pub surface: Color,
    pub border: Color,
    pub accent: Color,
    pub muted: Color,
}

pub const ZINC_PALETTE: Palette = Palette {
    background: Color::Rgb(24, 24, 27),    // Zinc 950
    foreground: Color::Rgb(250, 250, 250), // Zinc 50
    surface: Color::Rgb(39, 39, 42),       // Zinc 800
    border: Color::Rgb(63, 63, 70),        // Zinc 700
    accent: Color::Rgb(16, 185, 129),      // Emerald 500
    muted: Color::Rgb(113, 113, 122),      // Zinc 500
};

pub const SLATE_PALETTE: Palette = Palette {
    background: Color::Rgb(15, 23, 42),    // Slate 950
    foreground: Color::Rgb(248, 250, 252), // Slate 50
    surface: Color::Rgb(30, 41, 59),       // Slate 800
    border: Color::Rgb(51, 65, 85),        // Slate 700
    accent: Color::Rgb(56, 189, 248),      // Sky 400
    muted: Color::Rgb(100, 116, 139),      // Slate 500
};

pub fn get_palette(name: &str) -> Palette {
    match name.to_lowercase().as_str() {
        "slate" => SLATE_PALETTE,
        _ => ZINC_PALETTE,
    }
}
