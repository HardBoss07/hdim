use crate::app::{ActiveWidget, App, AppMode, Tool};
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::{Duration, Instant};

const PAN_AMOUNT_CHARACTERS: u32 = 10; // Number of characters to pan per key press
const ZOOM_FACTOR: f32 = 1.2; // Zoom factor per key press

pub fn handle_events(app: &mut App) -> Result<bool> {
    if event::poll(Duration::from_millis(16))? {
        if app.last_input_time.elapsed() >= app.input_delay {
            let mut last_key_event = None;
            // Drain the event queue, only keeping the last key press event
            while event::poll(Duration::from_millis(0))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        last_key_event = Some(key);
                    }
                }
            }

            if let Some(key) = last_key_event {
                app.last_input_time = Instant::now();
                // Pan amount is scaled by zoom to maintain consistent screen-space speed
                let pan_amount_pixels = (PAN_AMOUNT_CHARACTERS as f32 * app.zoom).round() as i32;

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
                        KeyCode::Char('q') => return Ok(true),
                        KeyCode::Char('1') => {
                            app.selected_tool = Some(Tool::Crop);
                            app.active_widget = ActiveWidget::Tools;
                        }
                        KeyCode::Esc => {
                            app.selected_tool = None;
                            app.active_widget = ActiveWidget::Main;
                        }
                        KeyCode::Tab => {
                            if let Some(Tool::Crop) = app.selected_tool {
                                app.selected_crop_option_index =
                                    (app.selected_crop_option_index + 1) % 5;
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(Tool::Crop) = app.selected_tool {
                                if app.selected_crop_option_index < 4 {
                                    app.mode = AppMode::EditingCropValue;
                                } else {
                                    // TODO: "Crop from viewport" logic
                                }
                            }
                        }
                        _ => match app.active_widget {
                            ActiveWidget::Main => match key.code {
                                KeyCode::Up => app.scroll(0, -pan_amount_pixels),
                                KeyCode::Down => app.scroll(0, pan_amount_pixels),
                                KeyCode::Left => app.scroll(-pan_amount_pixels, 0),
                                KeyCode::Right => app.scroll(pan_amount_pixels, 0),
                                KeyCode::PageUp => app.zoom(1.0 / ZOOM_FACTOR),
                                KeyCode::PageDown => app.zoom(ZOOM_FACTOR),
                                _ => {}
                            },
                            ActiveWidget::Tools => {}
                        },
                    },
                }
            }
        } else {
            // If the input delay hasn't passed, drain the queue to prevent event buildup
            while event::poll(Duration::from_millis(0))? {
                let _ = event::read();
            }
        }
    }
    Ok(false)
}
