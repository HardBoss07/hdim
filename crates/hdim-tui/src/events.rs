use crate::app::{ActiveWidget, App, AppMode};
use crate::components::adjustment_panel::AdjustmentsChange;
use crate::components::crop::handle_crop_events;
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use hdim_core::consts::{PAN_AMOUNT_CHARACTERS, ZOOM_FACTOR};
use hdim_core::state::Tool;
use std::path::PathBuf;
use std::time::{Duration, Instant};

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
        AppMode::EditingAdjustmentValue => match key.code {
            KeyCode::Char(c) if c.is_ascii_digit() || c == '-' || c == '.' => {
                app.adjustment_input.push(c);
            }
            KeyCode::Backspace => {
                app.adjustment_input.pop();
            }
            KeyCode::Enter => {
                if let Ok(value) = app.adjustment_input.parse::<f32>() {
                    let (min, max) = app.adjustment_panel.get_selected_slider_bounds();
                    let clamped_value = value.clamp(min, max);
                    app.adjustment_panel
                        .update_selected_slider_value(clamped_value);

                    let new_adjustments = app.adjustment_panel.get_adjustments();
                    app.hdim_image.adjustments = new_adjustments;
                    app.hdim_image.history.record_adjustments(new_adjustments);
                }
                app.adjustment_input.clear();
                app.mode = AppMode::Normal;
            }
            KeyCode::Esc => {
                app.adjustment_input.clear();
                app.mode = AppMode::Normal;
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
                    let output_format = format.to_image_format();

                    let adjusted_image = app.hdim_image.apply_adjustments();

                    let output_path =
                        PathBuf::from(format!("{}.{}", file_name, format.extension()));
                    match adjusted_image.save_with_format(&output_path, output_format) {
                        Ok(_) => {
                            // Optionally, provide feedback to the user that save was successful
                            // For now, just switch back to normal mode
                        }
                        Err(e) => {
                            // Handle save error, e.g., display error message
                            eprintln!("Error saving image: {:?}", e);
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
            KeyCode::Char('4') => {
                // Keybinding for Adjustments
                app.active_widget = ActiveWidget::Adjustments;
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
                        ActiveWidget::Adjustments => {
                            if let Some(change) = app.adjustment_panel.handle_event(key) {
                                match change {
                                    AdjustmentsChange::Updated(new_adjustments) => {
                                        app.hdim_image.adjustments = new_adjustments;
                                        app.hdim_image.history.record_adjustments(new_adjustments);
                                    }
                                    AdjustmentsChange::Undo(_adjustments) => {
                                        // _adjustments is current state from panel, not prev
                                        if let Some(prev_adjustments) =
                                            app.hdim_image.history.undo()
                                        {
                                            app.hdim_image.adjustments = prev_adjustments;
                                            app.adjustment_panel
                                                .update_sliders_from_adjustments(prev_adjustments);
                                        }
                                    }
                                    AdjustmentsChange::Redo(_adjustments) => {
                                        // _adjustments is current state from panel, not next
                                        if let Some(next_adjustments) =
                                            app.hdim_image.history.redo()
                                        {
                                            app.hdim_image.adjustments = next_adjustments;
                                            app.adjustment_panel
                                                .update_sliders_from_adjustments(next_adjustments);
                                        }
                                    }
                                    AdjustmentsChange::EnterPressed => {
                                        app.mode = AppMode::EditingAdjustmentValue;
                                        app.adjustment_input.clear();
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        },
    }
}
