use crate::app::{App, AppMode};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem},
};

pub fn render_crop_options<'a>(app: &'a App) -> List<'a> {
    let crop_options = ["Left", "Right", "Top", "Bottom", "Crop from viewport"];
    let crop_items: Vec<ListItem> = crop_options
        .iter()
        .enumerate()
        .map(|(i, &option)| {
            let mut text = if i < 4 {
                let value = match i {
                    0 => app.crop_state.left,
                    1 => app.crop_state.right,
                    2 => app.crop_state.top,
                    3 => app.crop_state.bottom,
                    _ => unreachable!(),
                };
                format!("{}: {}", option, value)
            } else {
                option.to_string()
            };

            if app.mode == AppMode::EditingCropValue && app.selected_crop_option_index == i {
                text.push_str(&format!(" {}", app.crop_input));
            }

            let mut item = ListItem::new(text);
            if app.selected_crop_option_index == i {
                item = item.style(Style::default().add_modifier(Modifier::REVERSED));
            }
            item
        })
        .collect();

    List::new(crop_items).block(Block::default().borders(Borders::ALL).title("Crop Options"))
}

pub fn handle_crop_events(key: KeyEvent, app: &mut App) {
    match app.mode {
        AppMode::EditingCropValue => match key.code {
            KeyCode::Char(c) if c.is_ascii_digit() => {
                app.crop_input.push(c);
            }
            KeyCode::Backspace => {
                app.crop_input.pop();
            }
            KeyCode::Enter => {
                if let Ok(value) = app.crop_input.parse::<u32>() {
                    match app.selected_crop_option_index {
                        0 => app.crop_state.left = value,
                        1 => app.crop_state.right = value,
                        2 => app.crop_state.top = value,
                        3 => app.crop_state.bottom = value,
                        _ => {}
                    }
                }
                app.crop_input.clear();
                app.mode = AppMode::Normal;
            }
            KeyCode::Esc => {
                app.crop_input.clear();
                app.mode = AppMode::Normal;
            }
            _ => {}
        },
        AppMode::Normal => match key.code {
            KeyCode::Tab => {
                app.selected_crop_option_index = (app.selected_crop_option_index + 1) % 5;
            }
            KeyCode::Enter => {
                if app.selected_crop_option_index < 4 {
                    app.mode = AppMode::EditingCropValue;
                } else {
                    // TODO: "Crop from viewport" logic
                }
            }
            _ => {}
        },
        AppMode::ExifView | AppMode::Saving | AppMode::EditingAdjustmentValue => {} // Do nothing in these modes
    }
}
