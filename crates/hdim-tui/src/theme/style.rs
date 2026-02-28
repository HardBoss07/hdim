use super::colors::THEME;
use ratatui::style::{Modifier, Style};

pub struct ThemeStyles {
    pub base: Style,
    pub border: Style,
    pub border_active: Style,
    pub highlight: Style,
    pub text_dim: Style,
    pub accent: Style,
}

pub const STYLES: ThemeStyles = ThemeStyles {
    base: Style::new().fg(THEME.foreground).bg(THEME.background),
    border: Style::new().fg(THEME.border),
    border_active: Style::new().fg(THEME.accent),
    highlight: Style::new()
        .fg(THEME.foreground)
        .bg(THEME.surface)
        .add_modifier(Modifier::BOLD),
    text_dim: Style::new().fg(THEME.muted),
    accent: Style::new().fg(THEME.accent),
};
