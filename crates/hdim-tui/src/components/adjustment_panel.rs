use super::sliders::slider::Slider;
use crate::theme::ThemeStyles;
use crossterm::event::{KeyCode, KeyEvent};
use hdim_core::Adjustments;
use hdim_core::localization::Adjustments as AdjustmentsLocalization;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub enum AdjustmentsChange {
    Updated,
    EnterPressed,
}

pub struct AdjustmentPanel {
    pub sliders: Vec<Slider>,
    pub selected_index: usize,
}

impl AdjustmentPanel {
    pub fn new(initial_adjustments: Adjustments, loc: &AdjustmentsLocalization) -> Self {
        Self {
            sliders: vec![
                Slider::new(
                    loc.brightness.clone(),
                    initial_adjustments.brightness,
                    -100.0,
                    100.0,
                ),
                Slider::new(
                    loc.contrast.clone(),
                    initial_adjustments.contrast,
                    -100.0,
                    100.0,
                ),
                Slider::new(
                    loc.exposure.clone(),
                    initial_adjustments.exposure,
                    -100.0,
                    100.0,
                ),
                Slider::new(loc.fade.clone(), initial_adjustments.fade, 0.0, 100.0),
                Slider::new(loc.grain.clone(), initial_adjustments.grain, 0.0, 100.0),
                Slider::new(loc.hue.clone(), initial_adjustments.hue, -100.0, 100.0),
                Slider::new(loc.noise.clone(), initial_adjustments.noise, 0.0, 100.0),
                Slider::new(
                    loc.saturation.clone(),
                    initial_adjustments.saturation,
                    -100.0,
                    100.0,
                ),
                Slider::new(
                    loc.vibrance.clone(),
                    initial_adjustments.vibrance,
                    -100.0,
                    100.0,
                ),
                Slider::new(
                    loc.warmth.clone(),
                    initial_adjustments.warmth,
                    -100.0,
                    100.0,
                ),
            ],
            selected_index: 0,
        }
    }

    pub fn handle_event(&mut self, key: KeyEvent) -> Option<AdjustmentsChange> {
        match key.code {
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_index < self.sliders.len() - 1 {
                    self.selected_index += 1;
                }
            }
            KeyCode::Left => {
                self.sliders[self.selected_index].decrement(1.0);
                return Some(AdjustmentsChange::Updated);
            }
            KeyCode::Right => {
                self.sliders[self.selected_index].increment(1.0);
                return Some(AdjustmentsChange::Updated);
            }
            KeyCode::Enter => {
                return Some(AdjustmentsChange::EnterPressed);
            }
            _ => {}
        }
        None
    }

    pub fn get_adjustments(&self) -> Adjustments {
        Adjustments {
            brightness: self.sliders[0].value,
            contrast: self.sliders[1].value,
            exposure: self.sliders[2].value,
            fade: self.sliders[3].value,
            grain: self.sliders[4].value,
            hue: self.sliders[5].value,
            noise: self.sliders[6].value,
            saturation: self.sliders[7].value,
            vibrance: self.sliders[8].value,
            warmth: self.sliders[9].value,
        }
    }

    pub fn update_sliders_from_adjustments(&mut self, adj: Adjustments) {
        self.sliders[0].value = adj.brightness;
        self.sliders[1].value = adj.contrast;
        self.sliders[2].value = adj.exposure;
        self.sliders[3].value = adj.fade;
        self.sliders[4].value = adj.grain;
        self.sliders[5].value = adj.hue;
        self.sliders[6].value = adj.noise;
        self.sliders[7].value = adj.saturation;
        self.sliders[8].value = adj.vibrance;
        self.sliders[9].value = adj.warmth;
    }

    pub fn get_selected_slider_bounds(&self) -> (f32, f32) {
        let slider = &self.sliders[self.selected_index];
        (slider.min, slider.max)
    }

    pub fn update_selected_slider_value(&mut self, value: f32) {
        self.sliders[self.selected_index].value = value;
    }

    pub fn render(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        is_editing_value: bool,
        input_string: &str,
        loc: &AdjustmentsLocalization,
        styles: &ThemeStyles,
    ) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                self.sliders
                    .iter()
                    .map(|_| Constraint::Length(3))
                    .collect::<Vec<_>>(),
            )
            .split(area);

        for (i, slider) in self.sliders.iter_mut().enumerate() {
            let is_selected = i == self.selected_index;
            slider.render(layout[i], frame.buffer_mut(), is_selected, styles);

            if is_selected && is_editing_value {
                let input_area = Rect::new(
                    layout[i].x + layout[i].width.saturating_sub(15),
                    layout[i].y + 1,
                    12,
                    1,
                );
                let input_widget = Paragraph::new(format!("_{}_", input_string)).style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                );
                frame.render_widget(input_widget, input_area);
            }
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .title(loc.title.as_str())
            .border_style(styles.border);

        frame.render_widget(block, area);
    }
}
