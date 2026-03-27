use crate::app::App;
use crate::config::Language;
use crate::theme::ThemeStyles;
use hdim_core::localization::Settings as SettingsLocalization;
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub struct SettingsView {
    pub selected_index: usize,
    pub selected_language: Language,
    pub selected_theme_index: usize,
    pub strip_exif: bool,
}

impl SettingsView {
    pub fn new(app: &App) -> Self {
        let themes = ["zinc", "slate"];
        let theme_index = themes
            .iter()
            .position(|&t| t == app.config.theme)
            .unwrap_or(0);

        Self {
            selected_index: 0,
            selected_language: app.config.language.clone(),
            selected_theme_index: theme_index,
            strip_exif: app.config.strip_exif,
        }
    }

    pub fn render(
        &self,
        area: Rect,
        buf: &mut Buffer,
        loc: &SettingsLocalization,
        styles: &ThemeStyles,
    ) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Language
                Constraint::Length(3), // Theme
                Constraint::Length(3), // Strip EXIF
                Constraint::Length(1), // Spacing
                Constraint::Length(1), // Help/Actions
            ])
            .split(area);

        // Language Selection
        let languages = [Language::English, Language::German];
        let mut final_lang_spans = vec![Span::raw(&loc.language)];
        for l in languages {
            let label = match l {
                Language::English => " English ",
                Language::German => " Deutsch ",
            };
            if l == self.selected_language {
                final_lang_spans.push(Span::styled(label, styles.highlight));
            } else {
                final_lang_spans.push(Span::raw(label));
            }
        }

        let lang_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(if self.selected_index == 0 {
                styles.border_active
            } else {
                styles.border
            });

        Paragraph::new(Line::from(final_lang_spans))
            .block(lang_block)
            .render(layout[0], buf);

        // Theme Selection
        let themes = ["Zinc", "Slate"];
        let mut theme_spans = vec![Span::raw(&loc.theme)];
        for (i, t) in themes.iter().enumerate() {
            if i == self.selected_theme_index {
                theme_spans.push(Span::styled(format!(" [{}] ", t), styles.highlight));
            } else {
                theme_spans.push(Span::raw(format!("  {}  ", t)));
            }
        }

        let theme_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(if self.selected_index == 1 {
                styles.border_active
            } else {
                styles.border
            });

        Paragraph::new(Line::from(theme_spans))
            .block(theme_block)
            .render(layout[1], buf);

        // Strip EXIF Toggle
        let strip_label = if self.strip_exif { " [ON] " } else { " [OFF] " };
        let strip_spans = vec![
            Span::raw(&loc.strip_exif),
            Span::styled(strip_label, styles.highlight),
        ];

        let strip_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(if self.selected_index == 2 {
                styles.border_active
            } else {
                styles.border
            });

        Paragraph::new(Line::from(strip_spans))
            .block(strip_block)
            .render(layout[2], buf);

        // Help / Actions
        let actions = Line::from(vec![
            Span::styled(&loc.save, styles.highlight),
            Span::raw("   "),
            Span::styled(&loc.cancel, styles.highlight),
        ]);
        Paragraph::new(actions)
            .alignment(Alignment::Center)
            .render(layout[4], buf);
    }
}
