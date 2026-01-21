use crate::app::App;
use ansi_to_tui::IntoText;
use color_eyre::eyre::Result;
use hdim_render::view::View;
use ratatui::{
    Frame,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
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
    frame.render_widget(
        Paragraph::new(" Left Banner Content ")
            .block(Block::default().borders(Borders::ALL).title("Left")),
        middle_chunks[0],
    );
    frame.render_widget(
        Paragraph::new(image_text).block(Block::default().borders(Borders::ALL).title(main_title)),
        main_area,
    );
    frame.render_widget(
        Paragraph::new(" Right Banner Content ")
            .block(Block::default().borders(Borders::ALL).title("Right")),
        middle_chunks[2],
    );
    frame.render_widget(
        Paragraph::new(" Arrows to Pan | PgUp/PgDn to Zoom | 'q' to Quit ")
            .block(Block::default().borders(Borders::ALL).title("Bottom")),
        vertical_chunks[2],
    );
}
