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

    // Global tool switching (available in most modes except Saving)
    if app.mode != AppMode::Saving {
        match key.code {
            KeyCode::Char('1') => {
                app.selected_tool = Some(Tool::Crop);
                app.active_widget = ActiveWidget::RightToolbar;
                app.mode = AppMode::Normal;
                app.show_right_toolbar = true;
                return;
            }
            KeyCode::Char('2') => {
                app.selected_tool = Some(Tool::Exif);
                app.mode = AppMode::ExifView;
                app.active_widget = ActiveWidget::RightToolbar;
                app.show_right_toolbar = true;
                if let Some(exif_view) = &mut app.exif_view {
                    exif_view.state.select(Some(0));
                }
                return;
            }
            KeyCode::Char('s') | KeyCode::Char('3') => {
                app.mode = AppMode::Saving;
                let original_filename = app
                    .hdim_image
                    .path
                    .file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_default();
                app.save_as.set_initial_filename(&original_filename);
                return;
            }
            KeyCode::Char('4') => {
                app.active_widget = ActiveWidget::Adjustments;
                app.selected_tool = None;
                app.mode = AppMode::Normal;
                app.show_right_toolbar = true;
                return;
            }
            KeyCode::Esc
                if app.mode != AppMode::EditingAdjustmentValue
                    && app.mode != AppMode::EditingCropValue =>
            {
                app.selected_tool = None;
                app.active_widget = ActiveWidget::Main;
                app.mode = AppMode::Normal;
                app.show_right_toolbar = false;
                return;
            }
            _ => {}
        }
    }

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
                app.show_right_toolbar = false;
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

                    app.update_adjustments();
                    let new_adjustments = app.hdim_image.adjustments;
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
        AppMode::Saving => match key.code {
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

                let output_path = PathBuf::from(format!("{}.{}", file_name, format.extension()));
                match app
                    .cached_image
                    .save_with_format(&output_path, output_format)
                {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error saving image: {:?}", e);
                    }
                }
                app.mode = AppMode::Normal;
            }
            _ => {}
        },
        AppMode::Normal => match key.code {
            KeyCode::Char('q') => {}
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
                                    AdjustmentsChange::Updated(_) => {
                                        app.update_adjustments();
                                        app.hdim_image
                                            .history
                                            .record_adjustments(app.hdim_image.adjustments);
                                    }
                                    AdjustmentsChange::Undo(_) => {
                                        if let Some(prev_adjustments) =
                                            app.hdim_image.history.undo()
                                        {
                                            app.adjustment_panel
                                                .update_sliders_from_adjustments(prev_adjustments);
                                            app.update_adjustments();
                                        }
                                    }
                                    AdjustmentsChange::Redo(_) => {
                                        if let Some(next_adjustments) =
                                            app.hdim_image.history.redo()
                                        {
                                            app.adjustment_panel
                                                .update_sliders_from_adjustments(next_adjustments);
                                            app.update_adjustments();
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
