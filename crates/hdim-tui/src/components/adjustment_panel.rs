use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use hdim_core::Adjustments;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
};

use super::sliders::slider::Slider;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AdjustmentsChange {
    Updated(Adjustments),
    Undo(Adjustments),
    Redo(Adjustments),
    EnterPressed,
}

pub struct AdjustmentPanel {
    sliders: Vec<Slider>,
    selected_slider: usize,
    adjustments: Adjustments,
}

impl AdjustmentPanel {
    pub fn new(initial_adjustments: Adjustments) -> Self {
        let sliders = vec![
            Slider::new("Brightness", initial_adjustments.brightness, -100.0, 100.0),
            Slider::new("Contrast", initial_adjustments.contrast, -100.0, 100.0),
            Slider::new("Exposure", initial_adjustments.exposure, -100.0, 100.0),
            Slider::new("Fade", initial_adjustments.fade, 0.0, 100.0), // Fade is typically 0 to 100
            Slider::new("Grain", initial_adjustments.grain, 0.0, 100.0), // Grain is typically 0 to 100
            Slider::new("Hue", initial_adjustments.hue, -100.0, 100.0),
            Slider::new("Noise", initial_adjustments.noise, 0.0, 100.0), // Noise is typically 0 to 100
            Slider::new("Saturation", initial_adjustments.saturation, -100.0, 100.0),
            Slider::new("Vibrance", initial_adjustments.vibrance, -100.0, 100.0),
            Slider::new("Warmth", initial_adjustments.warmth, -100.0, 100.0),
        ];

        Self {
            sliders,
            selected_slider: 0,
            adjustments: initial_adjustments,
        }
    }

    pub fn handle_event(&mut self, key: KeyEvent) -> Option<AdjustmentsChange> {
        let mut change: Option<AdjustmentsChange> = None;
        let step = if key.modifiers.contains(KeyModifiers::CONTROL) {
            10.0
        } else {
            1.0
        };

        match key.code {
            KeyCode::Up => {
                self.selected_slider = self.selected_slider.saturating_sub(1);
            }
            KeyCode::Down => {
                if self.selected_slider < self.sliders.len() - 1 {
                    self.selected_slider += 1;
                }
            }
            KeyCode::Left => {
                self.sliders[self.selected_slider].decrement(step);
                self.update_adjustments();
                change = Some(AdjustmentsChange::Updated(self.adjustments));
            }
            KeyCode::Right => {
                self.sliders[self.selected_slider].increment(step);
                self.update_adjustments();
                change = Some(AdjustmentsChange::Updated(self.adjustments));
            }
            KeyCode::Enter => {
                change = Some(AdjustmentsChange::EnterPressed);
            }
            KeyCode::Char('z') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                change = Some(AdjustmentsChange::Undo(self.adjustments));
            }
            KeyCode::Char('y') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                change = Some(AdjustmentsChange::Redo(self.adjustments));
            }
            _ => {}
        }
        change
    }

    fn update_adjustments(&mut self) {
        let current_slider_value = self.sliders[self.selected_slider].value();
        match self.selected_slider {
            0 => self.adjustments.brightness = current_slider_value,
            1 => self.adjustments.contrast = current_slider_value,
            2 => self.adjustments.exposure = current_slider_value,
            3 => self.adjustments.fade = current_slider_value,
            4 => self.adjustments.grain = current_slider_value,
            5 => self.adjustments.hue = current_slider_value,
            6 => self.adjustments.noise = current_slider_value,
            7 => self.adjustments.saturation = current_slider_value,
            8 => self.adjustments.vibrance = current_slider_value,
            9 => self.adjustments.warmth = current_slider_value,
            _ => {}
        }
    }

    pub fn update_sliders_from_adjustments(&mut self, new_adjustments: Adjustments) {
        self.adjustments = new_adjustments;
        self.sliders[0].set_value(self.adjustments.brightness);
        self.sliders[1].set_value(self.adjustments.contrast);
        self.sliders[2].set_value(self.adjustments.exposure);
        self.sliders[3].set_value(self.adjustments.fade);
        self.sliders[4].set_value(self.adjustments.grain);
        self.sliders[5].set_value(self.adjustments.hue);
        self.sliders[6].set_value(self.adjustments.noise);
        self.sliders[7].set_value(self.adjustments.saturation);
        self.sliders[8].set_value(self.adjustments.vibrance);
        self.sliders[9].set_value(self.adjustments.warmth);
    }

    pub fn get_adjustments(&self) -> Adjustments {
        self.adjustments
    }

    pub fn get_selected_slider_bounds(&self) -> (f32, f32) {
        let slider = &self.sliders[self.selected_slider];
        (slider.min, slider.max)
    }

    pub fn update_selected_slider_value(&mut self, value: f32) {
        self.sliders[self.selected_slider].set_value(value);
        self.update_adjustments();
    }

    pub fn render(
        &mut self,
        frame: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        is_editing_value: bool,
        input_string: &str,
    ) {
        let main_block = Block::default().borders(Borders::ALL).title("Adjustments");

        let inner_area = main_block.inner(area);
        frame.render_widget(main_block, area);

        let constraints = self
            .sliders
            .iter()
            .map(|_| Constraint::Length(3))
            .collect::<Vec<_>>(); // Each slider needs more height for label and bar

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_area);

        for (i, slider) in self.sliders.iter_mut().enumerate() {
            let chunk = chunks[i];
            let is_selected = i == self.selected_slider;

            let display_value = if is_selected && is_editing_value {
                input_string.to_string()
            } else {
                format!("{:.0}", slider.value)
            };

            let label_with_value = format!("{}: {}", slider.label, display_value);

            let render_slider = Slider::new(&slider.label, slider.value, slider.min, slider.max)
                .style(if is_selected {
                    ratatui::style::Style::default()
                        .fg(ratatui::style::Color::Yellow)
                        .add_modifier(ratatui::style::Modifier::BOLD)
                } else {
                    ratatui::style::Style::default()
                });

            // We need a way to show the input string.
            // Let's modify Slider's render to optionally take an override for the value display.
            // Or just render it here manually?
            // Actually, Slider::render renders the label and value at the top.
            // If I change the label to include the value when editing, it might look weird because Slider also renders the value.
            // Let's look at Slider::render again.
            render_slider.render(chunk, frame.buffer_mut());

            if is_selected && is_editing_value {
                // Overlay the input string over the value area
                let value_span = ratatui::text::Span::styled(
                    format!("__{}__", input_string),
                    ratatui::style::Style::default()
                        .fg(ratatui::style::Color::Magenta)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                );
                let value_width = value_span.width() as u16;
                let value_x = chunk.right().saturating_sub(value_width);
                let value_y = chunk.top();
                frame
                    .buffer_mut()
                    .set_span(value_x, value_y, &value_span, value_width);
            }
        }
    }
}
