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
    pub style: Style,
}

impl Slider {
    pub fn new(label: &str, value: f32, min: f32, max: f32) -> Self {
        Self {
            label: label.to_string(),
            value,
            min,
            max,
            style: Style::default(),
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(self.min, self.max);
    }

    pub fn increment(&mut self, step: f32) {
        self.set_value(self.value + step);
    }

    pub fn decrement(&mut self, step: f32) {
        self.set_value(self.value - step);
    }
}

impl Widget for Slider {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title_style = self.style.add_modifier(Modifier::BOLD);
        let value_style = self.style.add_modifier(Modifier::BOLD);

        if area.height < 1 || area.width < 1 {
            return;
        }

        // Calculate the percentage
        let percentage = (self.value - self.min) / (self.max - self.min);
        let filled_width = (area.width as f32 * percentage) as u16;

        // Render the bar
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                let style = if x < area.left() + filled_width {
                    Style::default().bg(ratatui::style::Color::LightBlue)
                } else {
                    Style::default().bg(ratatui::style::Color::DarkGray)
                };
                buf[(x, y)].set_symbol(" ").set_style(style);
            }
        }

        // Render label and value
        let label_span = Span::styled(self.label, title_style);
        let value_span = Span::styled(format!("{:.0}", self.value), value_style);

        let label_width = label_span.width() as u16;
        let value_width = value_span.width() as u16;

        let label_x = area.left();
        let label_y = area.top();

        let value_x = area.right().saturating_sub(value_width);
        let value_y = area.top();

        buf.set_span(label_x, label_y, &label_span, label_width);
        buf.set_span(value_x, value_y, &value_span, value_width);
    }
}
