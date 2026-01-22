use crate::app::{App, AppMode, Tool};
use ansi_to_tui::IntoText;
use color_eyre::eyre::Result;
use hdim_render::view::View;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render(frame: &mut Frame, app: &mut App) {
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(vertical_chunks[1]);

    let main_area = middle_chunks[1];

    // DYNAMICALLY RENDER THE VIEWPORT
    let image_width = app.hdim_image.width;
    let image_height = app.hdim_image.height;

    // The number of source pixels to cover is based on the TUI area size and zoom level.
    // Each character cell is two "pixels" high.
    let source_width = (main_area.width as f32 * app.zoom).round() as u32;
    let source_height = (main_area.height as f32 * app.zoom * 2.0).round() as u32;

    // Clamp source position to prevent scrolling past the edge
    app.source_pos.0 = app
        .source_pos
        .0
        .min(image_width.saturating_sub(source_width));
    app.source_pos.1 = app
        .source_pos
        .1
        .min(image_height.saturating_sub(source_height));

    let view = View {
        source_x: app.source_pos.0,
        source_y: app.source_pos.1,
        source_width,
        source_height,
        target_width: main_area.width as u32,
        target_height: main_area.height as u32,
    };

    let image_text = match hdim_render::render(&app.hdim_image.data, &view) {
        Result::Ok(ansi_string) => ansi_string.into_text().unwrap_or_default(),
        Err(_) => "Error rendering image".into_text().unwrap(),
    };

    // UI shows magnification factor, which is the inverse of the internal zoom ratio
    let magnification = 1.0 / app.zoom;
    let main_title = format!(
        "Main Window - Pos [Y: {}, X: {}] - Zoom: {:.2}x",
        app.source_pos.1, app.source_pos.0, magnification
    );

    frame.render_widget(
        Paragraph::new(" Navigation Bar / Title ")
            .block(Block::default().borders(Borders::ALL).title("Top")),
        vertical_chunks[0],
    );

    let tools = List::new([ListItem::new("1. Crop")])
        .block(Block::default().borders(Borders::ALL).title("Tools"));
    frame.render_widget(tools, middle_chunks[0]);

    frame.render_widget(
        Paragraph::new(image_text).block(Block::default().borders(Borders::ALL).title(main_title)),
        main_area,
    );

    let right_banner_content = if let Some(Tool::Crop) = app.selected_tool {
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

        let crop_list = List::new(crop_items)
            .block(Block::default().borders(Borders::ALL).title("Crop Options"));
        crop_list
    } else {
        List::new(vec![ListItem::new(" Right Banner Content ")])
            .block(Block::default().borders(Borders::ALL).title("Right"))
    };
    frame.render_widget(right_banner_content, middle_chunks[2]);

    let bottom_text = if let Some(Tool::Crop) = app.selected_tool {
        "Tab to switch | Enter to edit/select | Esc to deselect"
    } else {
        " Arrows to Pan | PgUp/PgDn to Zoom | 'q' to Quit "
    };

    frame.render_widget(
        Paragraph::new(bottom_text).block(Block::default().borders(Borders::ALL).title("Bottom")),
        vertical_chunks[2],
    );
}
