use ansi_to_tui::IntoText;
use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use hdim_render::view::View; // Use the new View struct and render function
use image::{DynamicImage, GenericImageView};
use ratatui::{
    DefaultTerminal, Frame,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::path::PathBuf;
use std::time::{Duration, Instant};

const PAN_AMOUNT_CHARACTERS: u32 = 10; // Number of characters to pan per key press
const ZOOM_FACTOR: f32 = 1.2; // Zoom factor per key press

/// Application state
struct App {
    /// We store the raw DynamicImage so we can re-render it
    raw_image: DynamicImage,
    /// The top-left corner of the viewport on the source image (x, y) in pixels.
    source_pos: (u32, u32),
    /// Zoom level. Represents `source_pixels / terminal_characters`.
    /// A smaller value is more zoomed in.
    zoom: f32,
    /// Track the last time an input was processed to prevent double-triggering
    last_input_time: Instant,
    /// Minimum time between processing consecutive inputs
    input_delay: Duration,
}

impl App {
    fn new(image: DynamicImage, initial_zoom: f32) -> Result<Self> {
        Ok(Self {
            raw_image: image,
            source_pos: (0, 0),
            zoom: initial_zoom,
            last_input_time: Instant::now(),
            input_delay: Duration::from_millis(50), // Reduced for snappier input
        })
    }

    /// Adjusts the zoom level.
    fn zoom(&mut self, factor: f32) {
        self.zoom *= factor;
        // Clamp zoom to a reasonable range
        if self.zoom < 0.01 {
            self.zoom = 0.01;
        }
        self.clamp_source_pos();
    }

    /// Moves the viewport on the source image.
    fn scroll(&mut self, dx: i32, dy: i32) {
        self.source_pos.0 = self.source_pos.0.saturating_add_signed(dx);
        self.source_pos.1 = self.source_pos.1.saturating_add_signed(dy);
        self.clamp_source_pos();
    }

    // Prevents the viewport from going out of bounds of the source image.
    fn clamp_source_pos(&mut self) {
        let (image_width, image_height) = self.raw_image.dimensions();
        if self.source_pos.0 > image_width {
            self.source_pos.0 = image_width;
        }
        if self.source_pos.1 > image_height {
            self.source_pos.1 = image_height;
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let path = PathBuf::from("./crates/hdim-render/tests/images/4k.jpg");
    let image = image::open(&path)
        .map_err(|e| color_eyre::eyre::eyre!("Could not find test image at {:?}: {}", path, e))?;

    // Start with a zoom level that fits the image width to a default 100-column view
    let initial_zoom = image.width() as f32 / 100.0;

    let app = App::new(image, initial_zoom)?;
    let result = run(terminal, app);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, &mut app))?;

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
                    let pan_amount_pixels =
                        (PAN_AMOUNT_CHARACTERS as f32 * app.zoom).round() as i32;

                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => break,
                        // Panning keys
                        KeyCode::Up => app.scroll(0, -pan_amount_pixels),
                        KeyCode::Down => app.scroll(0, pan_amount_pixels),
                        KeyCode::Left => app.scroll(-pan_amount_pixels, 0),
                        KeyCode::Right => app.scroll(pan_amount_pixels, 0),
                        // Zoom keys
                        KeyCode::PageUp => app.zoom(1.0 / ZOOM_FACTOR), // Zoom In
                        KeyCode::PageDown => app.zoom(ZOOM_FACTOR),     // Zoom Out
                        _ => {}
                    }
                }
            } else {
                // If the input delay hasn't passed, drain the queue to prevent event buildup
                while event::poll(Duration::from_millis(0))? {
                    let _ = event::read();
                }
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app: &mut App) {
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
    let (image_width, image_height) = app.raw_image.dimensions();

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

    let image_text = match hdim_render::render(&app.raw_image, &view) {
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
