use crate::theme::{SLIDER_FILL, SLIDER_HANDLE, SLIDER_TRACK, ThemeStyles};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

pub struct Slider {
    pub label: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
}

impl Slider {
    pub fn new(label: String, value: f32, min: f32, max: f32) -> Self {
        Self {
            label,
            value,
            min,
            max,
        }
    }

    pub fn increment(&mut self, amount: f32) {
        self.value = (self.value + amount).min(self.max);
    }

    pub fn decrement(&mut self, amount: f32) {
        self.value = (self.value - amount).max(self.min);
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer, is_selected: bool, styles: &ThemeStyles) {
        let border_style = if is_selected {
            styles.border_active
        } else {
            styles.border
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(self.label.as_str())
            .title_style(if is_selected {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            });

        let inner_area = block.inner(area);
        block.render(area, buf);

        // Render slider track
        let track_y = inner_area.y;
        let track_width = inner_area.width.saturating_sub(10); // Leave space for value display

        let normalized_value = (self.value - self.min) / (self.max - self.min);
        let handle_pos = (normalized_value * (track_width as f32 - 1.0)).round() as u16;

        for x in 0..track_width {
            let symbol = if x < handle_pos {
                SLIDER_FILL
            } else if x == handle_pos {
                SLIDER_HANDLE
            } else {
                SLIDER_TRACK
            };

            let style = if x <= handle_pos {
                styles.accent
            } else {
                styles.text_dim
            };

            buf.cell_mut((area.x + 1 + x, track_y))
                .map(|cell| cell.set_symbol(symbol).set_style(style));
        }

        // Render value text
        let value_text = format!("{:.0}", self.value);
        let value_x = area.x + 1 + track_width + 1;
        buf.set_string(
            value_x,
            track_y,
            format!("{:>4}", value_text),
            Style::default().fg(Color::Yellow),
        );
    }
}
