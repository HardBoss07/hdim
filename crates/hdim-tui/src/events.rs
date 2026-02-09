use crate::app::{ActiveWidget, App, AppMode};
use crate::components::crop::handle_crop_events;
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use hdim_core::state::Tool;
use std::path::PathBuf;
use std::time::{Duration, Instant};

const PAN_AMOUNT_CHARACTERS: u32 = 10; // Number of characters to pan per key press
const ZOOM_FACTOR: f32 = 1.2; // Zoom factor per key press

fn drain_event_queue_keeping_last_key() -> Result<Option<KeyEvent>> {
    let mut last_key_event = None;
    while event::poll(Duration::from_millis(0))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                last_key_event = Some(key);
            }
        }
    }
    Ok(last_key_event)
}

fn drain_event_queue() -> Result<()> {
    while event::poll(Duration::from_millis(0))? {
        let _ = event::read();
    }
    Ok(())
}

pub fn handle_events(app: &mut App) -> Result<bool> {
    if event::poll(Duration::from_millis(16))? {
        if app.last_input_time.elapsed() >= app.input_delay {
            let last_key_event = drain_event_queue_keeping_last_key()?;

            if let Some(key) = last_key_event {
                app.last_input_time = Instant::now();
                handle_key_press(app, key);
                if key.code == KeyCode::Char('q') {
                    return Ok(true);
                }
            }
        } else {
            drain_event_queue()?;
        }
    }
    Ok(false)
}

fn handle_key_press(app: &mut App, key: KeyEvent) {
    let pan_amount_pixels = (PAN_AMOUNT_CHARACTERS as f32 * app.zoom).round() as i32;

    match app.mode {
        AppMode::ExifView => match key.code {
            KeyCode::Up => {
                if let Some(exif_view) = app.exif_view.as_mut() {
                    exif_view.previous();
                }
            }
            KeyCode::Down => {
                if let Some(exif_view) = app.exif_view.as_mut() {
                    exif_view.next();
                }
            }
            KeyCode::Esc => {
                app.mode = AppMode::Normal;
                app.active_widget = ActiveWidget::Main;
            }
            _ => {}
        },
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
        AppMode::Saving => {
            match key.code {
                KeyCode::Up => app.save_as.on_up(),
                KeyCode::Down => app.save_as.on_down(),
                KeyCode::Left => app.save_as.on_left(),
                KeyCode::Right => app.save_as.on_right(),
                KeyCode::Backspace => app.save_as.on_backspace(),
                KeyCode::Delete => app.save_as.on_delete(),
                KeyCode::Char(c) => app.save_as.on_char(c),
                KeyCode::Esc => {
                    app.mode = AppMode::Normal;
                }
                KeyCode::Enter => {
                    let file_name = app.save_as.file_name();
                    let format = app.save_as.selected_format();
                    let output_format = format.to_image_format(); // Changed to to_image_format

                    let current_image_path = &app.hdim_image.path;
                    let image_result = image::open(current_image_path);

                    match image_result {
                        Ok(dynamic_image) => {
                            let output_path =
                                PathBuf::from(format!("{}.{}", file_name, format.extension()));
                            match dynamic_image.save_with_format(&output_path, output_format) {
                                // Pass output_format directly
                                Ok(_) => {
                                    // Optionally, provide feedback to the user that save was successful
                                    // For now, just switch back to normal mode
                                }
                                Err(e) => {
                                    // Handle save error, e.g., display error message
                                    eprintln!("Error saving image: {:?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            // Handle image loading error
                            eprintln!("Error loading image for saving: {:?}", e);
                        }
                    }
                    app.mode = AppMode::Normal;
                }
                _ => {}
            }
        }
        AppMode::Normal => match key.code {
            KeyCode::Char('q') => {
                // This is now handled in the main loop for a more responsive exit.
            }
            KeyCode::Char('1') => {
                app.selected_tool = Some(Tool::Crop);
                app.active_widget = ActiveWidget::RightToolbar;
            }
            KeyCode::Char('2') => {
                app.selected_tool = Some(Tool::Exif);
                app.mode = AppMode::ExifView;
                app.active_widget = ActiveWidget::RightToolbar;
                if let Some(exif_view) = &mut app.exif_view {
                    exif_view.state.select(Some(0));
                }
            }
            KeyCode::Char('s') | KeyCode::Char('3') => {
                // New keybinding for Save As
                app.mode = AppMode::Saving;
                let original_filename = app
                    .hdim_image
                    .path
                    .file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_default();
                app.save_as.set_initial_filename(&original_filename);
            }
            KeyCode::Esc => {
                app.selected_tool = None;
                app.active_widget = ActiveWidget::Main;
            }
            _ => {
                if let Some(Tool::Crop) = app.selected_tool {
                    handle_crop_events(key, app);
                } else {
                    match app.active_widget {
                        ActiveWidget::Main => match key.code {
                            KeyCode::Up => app.scroll(0, -pan_amount_pixels),
                            KeyCode::Down => app.scroll(0, pan_amount_pixels),
                            KeyCode::Left => app.scroll(-pan_amount_pixels, 0),
                            KeyCode::Right => app.scroll(pan_amount_pixels, 0),
                            KeyCode::PageUp => app.zoom(1.0 / ZOOM_FACTOR),
                            KeyCode::PageDown => app.zoom(ZOOM_FACTOR),
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        },
    }
}
