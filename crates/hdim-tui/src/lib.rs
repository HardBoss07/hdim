pub mod app;
pub mod components;
pub mod config;
pub mod events;
pub mod theme;
pub mod ui;

use app::App;
use hdim_core::HdimImage;
use ratatui::DefaultTerminal;
use std::env;
use std::path::PathBuf;

use crate::events::handle_events;
use crate::ui::render;

pub fn run() -> anyhow::Result<()> {
    color_eyre::install().map_err(|e| anyhow::anyhow!("{}", e))?;
    let terminal = ratatui::init();

    let image_path_str = env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("No image path provided. Usage: hdim <path/to/image>"))?;
    let image_path = PathBuf::from(image_path_str);

    let hdim_image = HdimImage::from_path(&image_path).map_err(|e| anyhow::anyhow!("{}", e))?;

    // Start with a zoom level that fits the image width to a default 100-column view
    let initial_zoom = hdim_image.width as f32 / 100.0;

    let app = App::new(hdim_image, initial_zoom).map_err(|e| anyhow::anyhow!("{}", e))?;
    let result = run_app(terminal, app);

    ratatui::restore();
    result
}

fn run_app(mut terminal: DefaultTerminal, mut app: App) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| render(frame, &mut app))?;

        if handle_events(&mut app).map_err(|e| anyhow::anyhow!("{}", e))? {
            break;
        }
    }
    Ok(())
}
