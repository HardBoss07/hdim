use crate::app::{ActiveWidget, App, AppMode};
use crate::components::crop::render_crop_options;
use crate::components::save_as::SaveAs;
use ansi_to_tui::IntoText;
use color_eyre::eyre::Result;
use hdim_core::state::Tool;
use hdim_core::HdimImage;
use hdim_core::utils::file_name_from_path;
use hdim_render::view::View;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

pub fn render(frame: &mut Frame, app: &mut App) {
    let global_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Top Navigation Bar
            Constraint::Min(0),    // Middle section (left, main, right)
            Constraint::Length(3), // Bottom Navigation Bar
        ])
        .split(frame.area());

    let top_nav_area = global_layout[0];
    let middle_section_area = global_layout[1];
    let bottom_nav_area = global_layout[2];

    // Determine constraints for the middle section based on right toolbar visibility
    let middle_constraints = [
        Constraint::Length(20), // Left Toolbar
        Constraint::Min(0),     // Main Content
        Constraint::Length(20), // Right Toolbar
    ];

    let middle_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(middle_constraints)
        .split(middle_section_area);

    let left_toolbar_area = middle_layout[0];
    let main_area = middle_layout[1];
    let right_toolbar_area = middle_layout[2];

    // RENDER THE VIEWPORT
    let (source_width, source_height) =
        app.calculate_viewport(main_area.width as u32, main_area.height as u32);

    // Get adjusted image
    app.hdim_image.adjustments = app.adjustment_panel.get_adjustments();
    let adjusted_image = app.hdim_image.apply_adjustments();

    let view = View {
        source_x: app.source_pos.0,
        source_y: app.source_pos.1,
        source_width,
        source_height,
        target_width: main_area.width as u32,
        target_height: main_area.height as u32,
    };

    let image_text = match hdim_render::render(&adjusted_image, &view) {
        Result::Ok(ansi_string) => ansi_string.into_text().unwrap_or_default(),
        Err(_) => "Error rendering image".into_text().unwrap(),
    };

    let magnification = 1.0 / app.zoom;
    let main_title = format!(
        "Main Window - Pos [Y: {}, X: {}] - Zoom: {:.2}x",
        app.source_pos.1, app.source_pos.0, magnification
    );

    // Render Top Navigation Bar
    let image_name = file_name_from_path(&app.hdim_image.path).unwrap_or_else(|| "Unknown".to_string());
    let top_nav_title = format!("hdim - {}", image_name);
    let top_nav_content = format!(" Editing: {} ", image_name);
    frame.render_widget(
        Paragraph::new(top_nav_content)
            .block(Block::default().borders(Borders::ALL).title(top_nav_title)),
        top_nav_area,
    );

    // Render Left Toolbar
    let tools_list_items = [
        ListItem::new("1. Crop"),
        ListItem::new("2. Exif"),
        ListItem::new("3. Save As"),
        ListItem::new("4. Adjustments"),
    ];
    let tools =
        List::new(tools_list_items).block(Block::default().borders(Borders::ALL).title("Tools"));
    frame.render_widget(tools, left_toolbar_area);

    // Render Main Content
    frame.render_widget(
        Paragraph::new(image_text).block(Block::default().borders(Borders::ALL).title(main_title)),
        main_area,
    );

    // Render Right Toolbar (if visible)
    if app.show_right_toolbar {
        match app.active_widget {
            ActiveWidget::Adjustments => {
                let is_editing = app.mode == AppMode::EditingAdjustmentValue;
                app.adjustment_panel.render(
                    frame,
                    right_toolbar_area,
                    is_editing,
                    &app.adjustment_input,
                );
            }
            ActiveWidget::RightToolbar => {
                // This should be the variant for the right toolbar
                match app.mode {
                    AppMode::ExifView => {
                        if let Some(exif_view) = &mut app.exif_view {
                            let mut list = exif_view.widget();
                            // No need to check active_widget == RightToolbar again here
                            list = list
                                .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
                            frame.render_stateful_widget(
                                list,
                                right_toolbar_area,
                                &mut exif_view.state,
                            );
                        } else {
                            frame.render_widget(
                                List::new(vec![ListItem::new("No EXIF data available.")]).block(
                                    Block::default().borders(Borders::ALL).title("EXIF Data"),
                                ),
                                right_toolbar_area,
                            );
                        }
                    }
                    AppMode::Normal
                    | AppMode::EditingCropValue
                    | AppMode::Saving
                    | AppMode::EditingAdjustmentValue => {
                        if let Some(Tool::Crop) = app.selected_tool {
                            frame.render_widget(render_crop_options(app), right_toolbar_area);
                        } else {
                            frame.render_widget(
                                List::new(vec![ListItem::new("Right Toolbar Content")])
                                    .block(Block::default().borders(Borders::ALL).title("Right")),
                                right_toolbar_area,
                            );
                        }
                    }
                }
            }
            _ => {
                // For other ActiveWidgets that don't use the right toolbar specifically, render nothing or a default
                frame.render_widget(Block::default().borders(Borders::NONE), right_toolbar_area);
            }
        };
    } else {
        // Render an empty block if the right toolbar is not explicitly shown
        frame.render_widget(Block::default().borders(Borders::NONE), right_toolbar_area);
    }

    // Render Bottom Navigation Bar
    let bottom_text = match app.active_widget {
        // Changed to match active_widget
        ActiveWidget::Adjustments => {
            "Up/Down to select slider | Left/Right to change value | Esc to exit adjustments"
        }
        _ => match app.mode {
            AppMode::Normal if app.selected_tool.is_some() => {
                "Tab to switch | Enter to edit/select | Esc to deselect"
            }
            AppMode::ExifView => "Up/Down to scroll | Esc to deselect",
            AppMode::Saving => {
                "Up/Down to select format | Left/Right to move cursor | Enter to Save | Esc to Cancel"
            }
            _ => " Arrows to Pan | PgUp/PgDn to Zoom | 'q' to Quit ",
        },
    };

    frame.render_widget(
        Paragraph::new(bottom_text).block(Block::default().borders(Borders::ALL).title("Bottom")),
        bottom_nav_area,
    );

    if let AppMode::Saving = app.mode {
        render_save_as_popup(frame, &app.save_as);
    }
}

fn render_save_as_popup(frame: &mut Frame, save_as_state: &SaveAs) {
    let size = frame.area();
    let popup_area = Rect::new(
        size.width.saturating_sub(60) / 2,
        size.height.saturating_sub(10) / 2,
        60,
        10,
    );

    frame.render_widget(Clear, popup_area); // This clears the background
    frame.render_widget(save_as_state, popup_area);
}
