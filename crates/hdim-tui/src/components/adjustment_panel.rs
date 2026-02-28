use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use hdim_core::Adjustments;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders},
};

use crate::theme::STYLES;

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
            Slider::new("Fade", initial_adjustments.fade, 0.0, 100.0),
            Slider::new("Grain", initial_adjustments.grain, 0.0, 100.0),
            Slider::new("Hue", initial_adjustments.hue, -100.0, 100.0),
            Slider::new("Noise", initial_adjustments.noise, 0.0, 100.0),
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
                if self.sliders[self.selected_slider].decrement(step) {
                    self.update_adjustments();
                    change = Some(AdjustmentsChange::Updated(self.adjustments));
                }
            }
            KeyCode::Right => {
                if self.sliders[self.selected_slider].increment(step) {
                    self.update_adjustments();
                    change = Some(AdjustmentsChange::Updated(self.adjustments));
                }
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
        let current_slider_value = self.sliders[self.selected_slider].value;
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
        if self.sliders[self.selected_slider].set_value(value) {
            self.update_adjustments();
        }
    }

    pub fn render(
        &mut self,
        frame: &mut ratatui::Frame,
        area: ratatui::layout::Rect,
        is_editing_value: bool,
        input_string: &str,
    ) {
        let main_block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(STYLES.border)
            .title(" Adjustments ");

        let inner_area = main_block.inner(area);
        frame.render_widget(main_block, area);

        let constraints = self
            .sliders
            .iter()
            .map(|_| Constraint::Length(3))
            .collect::<Vec<_>>();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_area);

        for (i, slider) in self.sliders.iter_mut().enumerate() {
            let chunk = chunks[i];
            let is_selected = i == self.selected_slider;

            let render_slider = slider.clone().focused(is_selected);

            frame.render_widget(render_slider, chunk);

            if is_selected && is_editing_value {
                // Overlay the input string over the value area
                let value_span = ratatui::text::Span::styled(
                    format!("_{}_", input_string),
                    STYLES.accent.add_modifier(ratatui::style::Modifier::BOLD),
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
