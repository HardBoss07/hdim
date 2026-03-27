use crate::app::{App, AppMode};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem};

pub fn render_transform_options<'a>(app: &'a App) -> List<'a> {
    let loc = &app.localization.transform;
    let transform_options = [
        loc.left.as_str(),
        loc.right.as_str(),
        loc.top.as_str(),
        loc.bottom.as_str(),
        loc.rotate_left.as_str(),
        loc.rotate_right.as_str(),
        loc.flip_horizontal.as_str(),
        loc.flip_vertical.as_str(),
        loc.from_viewport.as_str(),
        loc.apply.as_str(),
    ];
    let transform_items: Vec<ListItem> = transform_options
        .iter()
        .enumerate()
        .map(|(i, &option)| {
            let is_selected = app.selected_transform_option_index == i;
            let mut spans = vec![Span::raw(match i {
                0..=3 => {
                    let value = match i {
                        0 => app.transform_state.left,
                        1 => app.transform_state.right,
                        2 => app.transform_state.top,
                        3 => app.transform_state.bottom,
                        _ => unreachable!(),
                    };
                    format!("{}: {}", option, value)
                }
                6 => {
                    format!(
                        "{}: {}",
                        option,
                        if app.transform_state.flip_horizontal {
                            "ON"
                        } else {
                            "OFF"
                        }
                    )
                }
                7 => {
                    format!(
                        "{}: {}",
                        option,
                        if app.transform_state.flip_vertical {
                            "ON"
                        } else {
                            "OFF"
                        }
                    )
                }
                _ => option.to_string(),
            })];

            if app.mode == AppMode::EditingTransformValue && is_selected {
                spans.push(Span::styled(
                    format!(" {}", app.transform_input),
                    Style::default().fg(Color::Yellow),
                ));
            } else if is_selected {
                let tooltip = match i {
                    0..=3 => " (Enter to edit)",
                    4..=8 => " (Enter to toggle)",
                    9 => " (Enter to apply)",
                    _ => "",
                };
                spans.push(Span::styled(
                    tooltip,
                    Style::default()
                        .fg(Color::Gray)
                        .add_modifier(Modifier::ITALIC),
                ));
            }

            let mut item = ListItem::new(Line::from(spans));
            if is_selected {
                item = item.style(app.styles.highlight);
            }
            item
        })
        .collect();

    List::new(transform_items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(app.styles.border)
            .title(loc.title.as_str()),
    )
}

pub fn handle_transform_events(key: KeyEvent, app: &mut App) {
    match app.mode {
        AppMode::EditingTransformValue => match key.code {
            KeyCode::Char(c) if c.is_ascii_digit() || c == '%' => {
                app.transform_input.push(c);
            }
            KeyCode::Backspace => {
                app.transform_input.pop();
            }
            KeyCode::Enter => {
                let input = app.transform_input.clone();

                let total = match app.selected_transform_option_index {
                    0 | 1 => app.hdim_image.width,
                    2 | 3 => app.hdim_image.height,
                    _ => 0,
                };

                let value = hdim_core::transform::calculate_absolute_crop(&input, total);

                match app.selected_transform_option_index {
                    0 => app.transform_state.left = value,
                    1 => app.transform_state.right = value,
                    2 => app.transform_state.top = value,
                    3 => app.transform_state.bottom = value,
                    _ => {}
                }
                app.has_unapplied_transform = true;
                app.transform_input.clear();
                app.mode = AppMode::Normal;
            }
            KeyCode::Esc => {
                app.transform_input.clear();
                app.mode = AppMode::Normal;
            }
            _ => {}
        },
        AppMode::Normal => match key.code {
            KeyCode::Up => {
                if app.selected_transform_option_index > 0 {
                    app.selected_transform_option_index -= 1;
                } else {
                    app.selected_transform_option_index = 9;
                }
            }
            KeyCode::Down | KeyCode::Tab => {
                app.selected_transform_option_index =
                    (app.selected_transform_option_index + 1) % 10;
            }
            KeyCode::Enter => {
                match app.selected_transform_option_index {
                    0..=3 => {
                        app.mode = AppMode::EditingTransformValue;
                    }
                    4 => {
                        // Rotate Left
                        app.transform_state.rotation =
                            (app.transform_state.rotation - 90 + 360) % 360;
                        app.has_unapplied_transform = true;
                    }
                    5 => {
                        // Rotate Right
                        app.transform_state.rotation = (app.transform_state.rotation + 90) % 360;
                        app.has_unapplied_transform = true;
                    }
                    6 => {
                        // Flip Horizontal
                        app.transform_state.flip_horizontal = !app.transform_state.flip_horizontal;
                        app.has_unapplied_transform = true;
                    }
                    7 => {
                        // Flip Vertical
                        app.transform_state.flip_vertical = !app.transform_state.flip_vertical;
                        app.has_unapplied_transform = true;
                    }
                    8 => {
                        app.crop_from_viewport();
                        app.has_unapplied_transform = true;
                    }
                    9 => {
                        // Apply Transform
                        let transform_state = app.transform_state;
                        app.hdim_image.transform_image(&transform_state);
                        // Reset transform state after applying
                        app.transform_state = hdim_core::state::TransformState::default();
                        app.has_unapplied_transform = false;
                        app.update_adjustments(); // Refresh cache
                        app.selected_tool = None;
                        app.mode = AppMode::Normal;
                        app.active_widget = crate::app::ActiveWidget::Main;
                    }
                    _ => {}
                }
            }
            KeyCode::Esc => {
                if app.has_unapplied_transform {
                    app.mode = AppMode::ConfirmTransformCancel;
                } else {
                    app.selected_tool = None;
                    app.active_widget = crate::app::ActiveWidget::Main;
                    app.show_right_toolbar = false;
                }
            }
            _ => {}
        },
        _ => {}
    }
}
