use super::colors::Palette;
use ratatui::style::{Modifier, Style};

#[derive(Debug, Clone, Copy)]
pub struct ThemeStyles {
    pub base: Style,
    pub border: Style,
    pub border_active: Style,
    pub highlight: Style,
    pub text_dim: Style,
    pub accent: Style,
    pub inverted: Style,
}

impl ThemeStyles {
    pub fn new(theme: &Palette) -> Self {
        Self {
            base: Style::new().fg(theme.foreground).bg(theme.background),
            border: Style::new().fg(theme.border),
            border_active: Style::new().fg(theme.accent),
            highlight: Style::new()
                .fg(theme.foreground)
                .bg(theme.surface)
                .add_modifier(Modifier::BOLD),
            text_dim: Style::new().fg(theme.muted),
            accent: Style::new().fg(theme.accent),
            inverted: Style::new()
                .bg(theme.accent)
                .fg(theme.background)
                .add_modifier(Modifier::BOLD),
        }
    }
}
