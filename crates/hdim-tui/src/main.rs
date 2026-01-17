use ansi_to_tui::IntoText;
use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event, KeyCode};
use hdim_render::Renderer;
use ratatui::{
    DefaultTerminal, Frame,
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::path::PathBuf;

/// Application state to track the processed image and current scroll position
struct App {
    image_text: Text<'static>,
    /// scroll.0 is vertical (y), scroll.1 is horizontal (x)
    scroll: (u16, u16),
}

fn main() -> Result<()> {
    color_eyre::install()?;

    // 1. Pre-process image to ANSI and then to Ratatui Text
    let raw_ansi = generate_image()?;
    let image_text = raw_ansi.into_text()?;

    let app = App {
        image_text,
        scroll: (0, 0),
    };

    // 2. Initialize Terminal
    let terminal = ratatui::init();
    let result = run(terminal, app);

    // 3. Restore Terminal
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                // Exit keys
                KeyCode::Esc | KeyCode::Char('q') => break,

                // Vertical Scrolling (Up/Down)
                KeyCode::Up | KeyCode::Char('k') => {
                    app.scroll.0 = app.scroll.0.saturating_sub(1);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app.scroll.0 = app.scroll.0.saturating_add(1);
                }

                // Horizontal Scrolling (Left/Right)
                KeyCode::Left | KeyCode::Char('h') => {
                    app.scroll.1 = app.scroll.1.saturating_sub(1);
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    app.scroll.1 = app.scroll.1.saturating_add(1);
                }

                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app: &App) {
    // Vertical Chunks: Top (3), Middle (Fill), Bottom (3)
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // Horizontal Chunks inside Middle: Left (20%), Main (60%), Right (20%)
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(vertical_chunks[1]);

    // 1. Top Navbar
    frame.render_widget(
        Paragraph::new(" Navigation Bar / Title ")
            .block(Block::default().borders(Borders::ALL).title("Top")),
        vertical_chunks[0],
    );

    // 2. Left Sidebar
    frame.render_widget(
        Paragraph::new(" Left Banner Content ")
            .block(Block::default().borders(Borders::ALL).title("Left")),
        middle_chunks[0],
    );

    // 3. Main Window (The Scrollable Image)
    // We display the current scroll coordinates in the title for feedback
    let main_title = format!(
        "Main Window - Scroll [Y: {}, X: {}]",
        app.scroll.0, app.scroll.1
    );

    frame.render_widget(
        Paragraph::new(app.image_text.clone())
            .block(Block::default().borders(Borders::ALL).title(main_title))
            .scroll(app.scroll),
        middle_chunks[1],
    );

    // 4. Right Sidebar
    frame.render_widget(
        Paragraph::new(" Right Banner Content ")
            .block(Block::default().borders(Borders::ALL).title("Right")),
        middle_chunks[2],
    );

    // 5. Bottom Toolbar
    frame.render_widget(
        Paragraph::new(" Arrows to Scroll | 'q' or 'Esc' to Quit ")
            .block(Block::default().borders(Borders::ALL).title("Bottom")),
        vertical_chunks[2],
    );
}

fn generate_image() -> Result<String> {
    let path = PathBuf::from("./crates/hdim-render/tests/images/WindowsXP.png");

    let img = image::open(&path)
        .map_err(|e| color_eyre::eyre::eyre!("Could not find image at {:?}: {}", path, e))?;

    let renderer = Renderer::new(2);
    let output = renderer.render(&img).expect("Rendering failed");

    Ok(output)
}
