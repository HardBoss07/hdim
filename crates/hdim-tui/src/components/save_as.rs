use crate::theme::ThemeStyles;
use hdim_core::localization::Export as ExportLocalization;
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageFormat {
    #[default]
    Png,
    Jpeg,
    Gif,
    Bmp,
}

impl ImageFormat {
    pub fn extension(&self) -> &str {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpeg",
            ImageFormat::Gif => "gif",
            ImageFormat::Bmp => "bmp",
        }
    }

    pub fn to_image_format(&self) -> image::ImageFormat {
        match self {
            ImageFormat::Png => image::ImageFormat::Png,
            ImageFormat::Jpeg => image::ImageFormat::Jpeg,
            ImageFormat::Gif => image::ImageFormat::Gif,
            ImageFormat::Bmp => image::ImageFormat::Bmp,
        }
    }

    pub fn all() -> &'static [ImageFormat] {
        &[
            ImageFormat::Png,
            ImageFormat::Jpeg,
            ImageFormat::Gif,
            ImageFormat::Bmp,
        ]
    }
}

impl fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageFormat::Png => write!(f, "PNG"),
            ImageFormat::Jpeg => write!(f, "JPEG"),
            ImageFormat::Gif => write!(f, "GIF"),
            ImageFormat::Bmp => write!(f, "BMP"),
        }
    }
}

pub struct SaveAs {
    file_name: String,
    selected_format: ImageFormat,
    cursor_position: usize,
}

impl SaveAs {
    pub fn new() -> Self {
        Self {
            file_name: String::new(),
            selected_format: ImageFormat::default(),
            cursor_position: 0,
        }
    }

    pub fn set_initial_filename(&mut self, name: &str) {
        self.file_name = name.to_string();
        self.cursor_position = self.file_name.len();
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn selected_format(&self) -> ImageFormat {
        self.selected_format
    }

    pub fn on_char(&mut self, c: char) {
        self.file_name.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn on_backspace(&mut self) {
        if self.cursor_position > 0 {
            self.file_name.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }

    pub fn on_delete(&mut self) {
        if self.cursor_position < self.file_name.len() {
            self.file_name.remove(self.cursor_position);
        }
    }

    pub fn on_left(&mut self) {
        self.cursor_position = self.cursor_position.saturating_sub(1);
    }

    pub fn on_right(&mut self) {
        if self.cursor_position < self.file_name.len() {
            self.cursor_position += 1;
        }
    }

    pub fn on_up(&mut self) {
        let formats = ImageFormat::all();
        let current_index = formats
            .iter()
            .position(|&f| f == self.selected_format)
            .unwrap_or(0);
        let next_index = if current_index == 0 {
            formats.len() - 1
        } else {
            current_index - 1
        };
        self.selected_format = formats[next_index];
    }

    pub fn on_down(&mut self) {
        let formats = ImageFormat::all();
        let current_index = formats
            .iter()
            .position(|&f| f == self.selected_format)
            .unwrap_or(0);
        let next_index = (current_index + 1) % formats.len();
        self.selected_format = formats[next_index];
    }

    pub fn widget<'a>(
        &'a self,
        loc: &'a ExportLocalization,
        styles: &'a ThemeStyles,
    ) -> SaveAsWidget<'a> {
        SaveAsWidget {
            state: self,
            loc,
            styles,
        }
    }
}

pub struct SaveAsWidget<'a> {
    state: &'a SaveAs,
    loc: &'a ExportLocalization,
    styles: &'a ThemeStyles,
}

impl<'a> Widget for SaveAsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Spacing
                Constraint::Length(1), // Format selector
                Constraint::Length(2), // Spacing
                Constraint::Length(3), // Filename input
            ])
            .split(area);

        // Render format selector
        let formats = ImageFormat::all();
        let mut final_format_spans = vec![Span::raw(format!("{} ", self.loc.select_format))];
        for f in formats {
            if *f == self.state.selected_format {
                final_format_spans.push(Span::styled(format!(" [{}] ", f), self.styles.highlight));
            } else {
                final_format_spans.push(Span::raw(format!("  {}  ", f)));
            }
        }

        Paragraph::new(Line::from(final_format_spans)).render(layout[1], buf);

        // Render filename input
        let filename_label = self.loc.filename.as_str();
        let input_text = format!("{}{}", filename_label, self.state.file_name);
        let input_widget = Paragraph::new(input_text).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(self.styles.border),
        );

        input_widget.render(layout[3], buf);
    }
}
