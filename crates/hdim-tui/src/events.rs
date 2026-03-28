use crate::app::{ActiveWidget, App, AppMode};
use crate::components::adjustment_panel::AdjustmentsChange;
use crate::components::settings::SettingsView;
use crate::components::transform::handle_transform_events;
use crate::config::Language;
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use hdim_core::consts::{PAN_AMOUNT_CHARACTERS, ZOOM_FACTOR};
use hdim_core::state::Tool;
use std::path::PathBuf;
use std::time::{Duration, Instant};

fn is_continuous_event(key: &KeyEvent) -> bool {
    matches!(
        key.code,
        KeyCode::Up
            | KeyCode::Down
            | KeyCode::Left
            | KeyCode::Right
            | KeyCode::PageUp
            | KeyCode::PageDown
    )
}

pub fn handle_events(app: &mut App) -> Result<bool> {
    if event::poll(Duration::from_millis(16))? {
        let mut key_events = Vec::new();
        while event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    key_events.push(key);
                }
            }
        }

        for key in key_events {
            let is_continuous = is_continuous_event(&key);
            let elapsed = app.last_input_time.elapsed();

            if !is_continuous || elapsed >= app.input_delay {
                app.last_input_time = Instant::now();

                // Priority handling for ConfirmQuit mode to avoid other handlers interfering
                if app.mode == AppMode::ConfirmQuit {
                    match key.code {
                        KeyCode::Char('y') | KeyCode::Enter => return Ok(true),
                        KeyCode::Char('n') | KeyCode::Esc => {
                            app.mode = AppMode::Normal;
                            continue;
                        }
                        _ => continue,
                    }
                }

                if app.mode == AppMode::ConfirmTransformCancel {
                    match key.code {
                        KeyCode::Char('y') | KeyCode::Enter => {
                            app.has_unapplied_transform = false;
                            app.transform_state = hdim_core::state::TransformState::default();
                            app.selected_tool = None;
                            app.active_widget = ActiveWidget::Main;
                            app.mode = AppMode::Normal;
                            app.show_right_toolbar = false;
                            continue;
                        }
                        KeyCode::Char('n') | KeyCode::Esc => {
                            app.mode = AppMode::Normal;
                            continue;
                        }
                        _ => continue,
                    }
                }

                handle_key_press(app, key);

                // Global Quit Handler (Ctrl+q)
                if key.code == KeyCode::Char('q') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    if app.has_unsaved_changes() {
                        app.mode = AppMode::ConfirmQuit;
                    } else {
                        return Ok(true);
                    }
                }
            }
        }
    }
    Ok(false)
}

fn handle_key_press(app: &mut App, key: KeyEvent) {
    let pan_amount_pixels = (PAN_AMOUNT_CHARACTERS as f32 * app.zoom).round() as i32;

    // Global Keybinds (Available in most modes)
    if app.mode != AppMode::ConfirmQuit {
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            match key.code {
                KeyCode::Char('z') => {
                    if app.hdim_image.undo() {
                        let new_adjustments = app.hdim_image.adjustments;
                        app.adjustment_panel
                            .update_sliders_from_adjustments(new_adjustments);
                        app.update_adjustments();
                    }
                    return;
                }
                KeyCode::Char('y') => {
                    if app.hdim_image.redo() {
                        let new_adjustments = app.hdim_image.adjustments;
                        app.adjustment_panel
                            .update_sliders_from_adjustments(new_adjustments);
                        app.update_adjustments();
                    }
                    return;
                }
                KeyCode::Char('s') => {
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
                _ => {}
            }
        } else if key
            .modifiers
            .contains(KeyModifiers::CONTROL | KeyModifiers::SHIFT)
        {
            if key.code == KeyCode::Char('Z') || key.code == KeyCode::Char('z') {
                if app.hdim_image.redo() {
                    let new_adjustments = app.hdim_image.adjustments;
                    app.adjustment_panel
                        .update_sliders_from_adjustments(new_adjustments);
                    app.update_adjustments();
                }
                return;
            }
        }
    }

    // Global tool switching (available in most modes except Saving, ConfirmQuit, Settings, and Editing)
    if app.mode != AppMode::Saving
        && app.mode != AppMode::ConfirmQuit
        && app.mode != AppMode::Settings
        && app.mode != AppMode::EditingAdjustmentValue
        && app.mode != AppMode::EditingTransformValue
    {
        match key.code {
            KeyCode::Char('1') => {
                app.selected_tool = Some(Tool::Transform);
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
            KeyCode::Char('5') => {
                app.settings_view = Some(SettingsView::new(app));
                app.mode = AppMode::Settings;
                return;
            }
            KeyCode::Esc
                if app.mode != AppMode::EditingAdjustmentValue
                    && app.mode != AppMode::EditingTransformValue =>
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
        AppMode::ConfirmQuit | AppMode::ConfirmTransformCancel => {
            // Handled in handle_events, but required for exhaustiveness
        }
        AppMode::Settings => {
            if let Some(settings_view) = &mut app.settings_view {
                match key.code {
                    KeyCode::Up => {
                        if settings_view.selected_index > 0 {
                            settings_view.selected_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if settings_view.selected_index < 2 {
                            settings_view.selected_index += 1;
                        }
                    }
                    KeyCode::Left | KeyCode::Right => {
                        match settings_view.selected_index {
                            0 => {
                                // Toggle language
                                settings_view.selected_language =
                                    match settings_view.selected_language {
                                        Language::English => Language::German,
                                        Language::German => Language::English,
                                    };
                            }
                            1 => {
                                // Toggle theme
                                settings_view.selected_theme_index =
                                    (settings_view.selected_theme_index + 1) % 2;
                            }
                            2 => {
                                // Toggle strip_exif
                                settings_view.strip_exif = !settings_view.strip_exif;
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Char('s') | KeyCode::Enter => {
                        app.config.language = settings_view.selected_language.clone();
                        app.config.theme = match settings_view.selected_theme_index {
                            0 => "zinc".to_string(),
                            1 => "slate".to_string(),
                            _ => "zinc".to_string(),
                        };
                        app.config.strip_exif = settings_view.strip_exif;
                        let _ = app.config.save();
                        app.refresh_localization();
                        app.mode = AppMode::Normal;
                        app.settings_view = None;
                    }
                    KeyCode::Esc => {
                        app.mode = AppMode::Normal;
                        app.settings_view = None;
                    }
                    _ => {}
                }
            }
        }
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
                    app.hdim_image.record_state();
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
        AppMode::EditingTransformValue => match key.code {
            KeyCode::Char(c) if c.is_ascii_digit() => {
                app.transform_input.push(c);
            }
            KeyCode::Backspace => {
                app.transform_input.pop();
            }
            KeyCode::Enter => {
                if let Ok(value) = app.transform_input.parse::<u32>() {
                    match app.selected_transform_option_index {
                        0 => app.transform_state.left = value,
                        1 => app.transform_state.right = value,
                        2 => app.transform_state.top = value,
                        3 => app.transform_state.bottom = value,
                        _ => {}
                    }
                }
                app.transform_input.clear();
                app.mode = AppMode::Normal;
            }
            KeyCode::Esc => {
                app.transform_input.clear();
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
                match app.hdim_image.save_with_exif(
                    &output_path,
                    output_format,
                    app.config.strip_exif,
                ) {
                    Ok(_) => {
                        app.mark_saved();
                    }
                    Err(e) => {
                        eprintln!("Error saving image: {:?}", e);
                    }
                }
                app.mode = AppMode::Normal;
            }
            _ => {}
        },
        AppMode::Normal => match key.code {
            _ => {
                if let Some(Tool::Transform) = app.selected_tool {
                    handle_transform_events(key, app);
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
                                    AdjustmentsChange::Updated => {
                                        app.update_adjustments();
                                        app.hdim_image.record_state();
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
