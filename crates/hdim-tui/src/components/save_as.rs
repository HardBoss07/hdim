use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub enum ImageFormat {
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

    pub fn all() -> &'static [ImageFormat] {
        &[
            ImageFormat::Png,
            ImageFormat::Jpeg,
            ImageFormat::Gif,
            ImageFormat::Bmp,
        ]
    }

    pub fn to_image_format(&self) -> image::ImageFormat {
        match self {
            ImageFormat::Png => image::ImageFormat::Png,
            ImageFormat::Jpeg => image::ImageFormat::Jpeg,
            ImageFormat::Gif => image::ImageFormat::Gif,
            ImageFormat::Bmp => image::ImageFormat::Bmp,
        }
    }
}

impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageFormat::Png => write!(f, "PNG"),
            ImageFormat::Jpeg => write!(f, "JPEG"),
            ImageFormat::Gif => write!(f, "GIF"),
            ImageFormat::Bmp => write!(f, "BMP"),
        }
    }
}

#[derive(Default)]
pub struct SaveAs {
    selected_format_index: usize,
    file_name_input: String,
    cursor_position: usize,
}

impl SaveAs {
    pub fn new() -> Self {
        SaveAs {
            selected_format_index: 0,
            file_name_input: String::new(),
            cursor_position: 0,
        }
    }

    pub fn set_initial_filename(&mut self, name: &str) {
        self.file_name_input = name.to_string();
        self.cursor_position = name.len();
    }

    pub fn selected_format(&self) -> &ImageFormat {
        &ImageFormat::all()[self.selected_format_index]
    }

    pub fn file_name(&self) -> &str {
        &self.file_name_input
    }

    pub fn on_up(&mut self) {
        if self.selected_format_index > 0 {
            self.selected_format_index -= 1;
        }
    }

    pub fn on_down(&mut self) {
        if self.selected_format_index < ImageFormat::all().len() - 1 {
            self.selected_format_index += 1;
        }
    }

    pub fn on_char(&mut self, character: char) {
        self.file_name_input.insert(self.cursor_position, character);
        self.cursor_position += 1;
    }

    pub fn on_backspace(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.file_name_input.remove(self.cursor_position);
        }
    }

    pub fn on_delete(&mut self) {
        if self.cursor_position < self.file_name_input.len() {
            self.file_name_input.remove(self.cursor_position);
        }
    }

    pub fn on_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn on_right(&mut self) {
        if self.cursor_position < self.file_name_input.len() {
            self.cursor_position += 1;
        }
    }
}

impl Widget for &SaveAs {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let _block = Block::default()
            .borders(Borders::ALL)
            .title(" Save Image As ");
        buffer.set_string(area.x + 1, area.y + 1, "Select format:", Style::default());

        let formats_area = Rect::new(
            area.x + 1,
            area.y + 2,
            area.width - 2,
            ImageFormat::all().len() as u16,
        );

        for (index, format) in ImageFormat::all().iter().enumerate() {
            let style = if index == self.selected_format_index {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            buffer.set_string(
                formats_area.x,
                formats_area.y + index as u16,
                format.to_string(),
                style,
            );
        }

        let filename_label = "Filename: ";
        buffer.set_string(
            area.x + 1,
            formats_area.y + formats_area.height + 1,
            filename_label,
            Style::default(),
        );

        let input_width = area.width.saturating_sub(filename_label.len() as u16 + 2);
        let input_area = Rect::new(
            area.x + 1 + filename_label.len() as u16,
            formats_area.y + formats_area.height + 1,
            input_width,
            1,
        );

        let input_text = self.file_name_input.clone();
        Paragraph::new(input_text).render(input_area, buffer);

        // Render cursor
        if input_area.width > 0 {
            buffer[(input_area.x + self.cursor_position as u16, input_area.y)]
                .set_style(Style::default().add_modifier(Modifier::REVERSED));
        }
    }
}
