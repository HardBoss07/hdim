use crate::theme::{SLIDER_FILL, SLIDER_HANDLE, SLIDER_TRACK, STYLES, THEME};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::Span,
    widgets::Widget,
};

#[derive(Clone)]
pub struct Slider {
    pub label: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub is_focused: bool,
}

impl Slider {
    pub fn new(label: &str, value: f32, min: f32, max: f32) -> Self {
        Self {
            label: label.to_string(),
            value,
            min,
            max,
            is_focused: false,
        }
    }

    pub fn focused(mut self, focused: bool) -> Self {
        self.is_focused = focused;
        self
    }

    pub fn set_value(&mut self, value: f32) -> bool {
        let old_value = self.value;
        self.value = value.clamp(self.min, self.max);
        (self.value - old_value).abs() > f32::EPSILON
    }

    pub fn increment(&mut self, step: f32) -> bool {
        self.set_value(self.value + step)
    }

    pub fn decrement(&mut self, step: f32) -> bool {
        self.set_value(self.value - step)
    }
}

impl Widget for Slider {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height < 2 || area.width < 4 {
            return;
        }

        let base_style = if self.is_focused {
            STYLES.accent
        } else {
            Style::default().fg(THEME.foreground)
        };

        // 1. Render Label and Value (Top Row)
        let label_span = Span::styled(
            &self.label,
            if self.is_focused {
                base_style.add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(THEME.muted)
            },
        );
        let value_span = Span::styled(
            format!("{:.0}", self.value),
            base_style.add_modifier(Modifier::BOLD),
        );

        buf.set_span(area.x, area.y, &label_span, label_span.width() as u16);
        buf.set_span(
            area.right().saturating_sub(value_span.width() as u16),
            area.y,
            &value_span,
            value_span.width() as u16,
        );

        // 2. Render Track (Bottom Row)
        let track_y = area.y + 1;
        let track_width = area.width;
        let percentage = (self.value - self.min) / (self.max - self.min);
        let handle_pos = (track_width as f32 * percentage).round() as u16;
        let handle_pos = handle_pos.clamp(0, track_width.saturating_sub(1));

        for x in 0..track_width {
            let symbol = if x < handle_pos {
                SLIDER_FILL
            } else if x == handle_pos {
                SLIDER_HANDLE
            } else {
                SLIDER_TRACK
            };

            let style = if x <= handle_pos {
                if self.is_focused {
                    STYLES.accent
                } else {
                    Style::default().fg(THEME.foreground)
                }
            } else {
                Style::default().fg(THEME.surface)
            };

            buf.get_mut(area.x + x, track_y)
                .set_symbol(symbol)
                .set_style(style);
        }
    }
}
