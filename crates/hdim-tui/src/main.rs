use ansi_to_tui::IntoText;
use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event, KeyCode};
use hdim_render::Renderer;
use image::DynamicImage;
use ratatui::{
    DefaultTerminal, Frame,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::path::PathBuf;

/// Application state to track the processed image and current scroll position
struct App {
    /// We store the raw DynamicImage so we can re-render it at different zoom levels
    raw_image: DynamicImage,
    /// The processed TUI text
    image_text: Text<'static>,
    /// scroll.0 is vertical (y), scroll.1 is horizontal (x)
    scroll: (u16, u16),
    /// Zoom factor (maps to Renderer's area_size)
    zoom_factor: u32,
}

impl App {
    fn new(img: DynamicImage) -> Result<Self> {
        let zoom_factor = 2; // Default zoom
        let text = App::process_image(&img, zoom_factor)?;
        Ok(Self {
            raw_image: img,
            image_text: text,
            scroll: (0, 0),
            zoom_factor,
        })
    }

    /// Helper to run the Renderer and convert to Ratatui Text
    fn process_image(img: &DynamicImage, zoom: u32) -> Result<Text<'static>> {
        let renderer = Renderer::new(zoom);
        // Run System Under Test / Renderer
        let ansi_string = renderer.render(img).expect("Rendering failed");
        Ok(ansi_string.into_text()?)
    }

    /// Updates the image_text when zoom changes
    fn update_zoom(&mut self, delta: i32) -> Result<()> {
        let new_zoom = if delta > 0 {
            self.zoom_factor.saturating_add(delta as u32)
        } else {
            self.zoom_factor.saturating_sub(delta.abs() as u32)
        };

        // Ensure zoom is at least 1 to avoid division by zero in the renderer
        let new_zoom = new_zoom.max(1);

        if new_zoom != self.zoom_factor {
            self.zoom_factor = new_zoom;
            self.image_text = Self::process_image(&self.raw_image, self.zoom_factor)?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // Initialize Terminal
    let terminal = ratatui::init();

    // Load image once before entering the loop
    let path = PathBuf::from("./crates/hdim-render/tests/images/WindowsXP.png");
    let img = image::open(&path)
        .map_err(|e| color_eyre::eyre::eyre!("Could not find test image at {:?}: {}", path, e))?;

    let app = App::new(img)?;
    let result = run(terminal, app);

    // Restore Terminal
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, mut app: App) -> Result<()> {
    loop {
        // Rendering
        terminal.draw(|frame| render(frame, &app))?;

        // Input handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                // Exit keys
                KeyCode::Esc | KeyCode::Char('q') => {
                    break;
                }

                // Vertical Scrolling (Up/Down)
                KeyCode::Up => {
                    app.scroll.0 = app.scroll.0.saturating_sub(1);
                }
                KeyCode::Down => {
                    app.scroll.0 = app.scroll.0.saturating_add(1);
                }

                // Horizontal Scrolling (Left/Right)
                KeyCode::Left => {
                    app.scroll.1 = app.scroll.1.saturating_sub(1);
                }
                KeyCode::Right => {
                    app.scroll.1 = app.scroll.1.saturating_add(1);
                }

                // Zooming (PageUp/PageDown)
                KeyCode::PageUp => {
                    app.update_zoom(1)?;
                }
                KeyCode::PageDown => {
                    app.update_zoom(-1)?;
                }

                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app: &App) {
    // Vertical Chunks (Top, Middle, Bottom)
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header: fixed height
            Constraint::Min(0),    // Main Content: takes up remaining space
            Constraint::Length(3), // Footer: fixed height
        ])
        .split(frame.area());

    // Horizontal Chunks inside Middle: Left (20%), Main (60%), Right (20%)
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Left Sidebar
            Constraint::Percentage(60), // Main Window
            Constraint::Percentage(20), // Right Sidebar
        ])
        .split(vertical_chunks[1]);

    // Widgets

    // Top Navbar
    frame.render_widget(
        Paragraph::new(" Navigation Bar / Title ")
            .block(Block::default().borders(Borders::ALL).title("Top")),
        vertical_chunks[0],
    );

    // Left Sidebar
    frame.render_widget(
        Paragraph::new(" Left Banner Content ")
            .block(Block::default().borders(Borders::ALL).title("Left")),
        middle_chunks[0],
    );

    // Main Window (Center) - Displaying the scrollable, zoomable image
    let main_title = format!(
        "Main Window - Scroll [Y: {}, X: {}] - Zoom: {}x",
        app.scroll.0, app.scroll.1, app.zoom_factor
    );

    frame.render_widget(
        Paragraph::new(app.image_text.clone())
            .block(Block::default().borders(Borders::ALL).title(main_title))
            .scroll(app.scroll),
        middle_chunks[1],
    );

    // Right Sidebar
    frame.render_widget(
        Paragraph::new(" Right Banner Content ")
            .block(Block::default().borders(Borders::ALL).title("Right")),
        middle_chunks[2],
    );

    // Bottom Toolbar
    frame.render_widget(
        Paragraph::new(" Arrows to Scroll | PgUp/PgDn to Zoom | 'q' to Quit ")
            .block(Block::default().borders(Borders::ALL).title("Bottom")),
        vertical_chunks[2],
    );
}
