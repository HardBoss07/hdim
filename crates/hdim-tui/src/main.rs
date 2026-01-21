mod app;
mod events;
mod ui;

use color_eyre::eyre::{Result, eyre};
use hdim_core::HdimImage;
use ratatui::DefaultTerminal;
use std::env;
use std::path::PathBuf;

use crate::app::App;
use crate::events::handle_events;
use crate::ui::render;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let image_path_str = env::args()
        .nth(1)
        .ok_or_else(|| eyre!("No image path provided. Usage: hdim <path/to/image>"))?;
    let image_path = PathBuf::from(image_path_str);

    let hdim_image =
        HdimImage::from_path(&image_path).map_err(|e| color_eyre::eyre::eyre!("{}", e))?;

    // Start with a zoom level that fits the image width to a default 100-column view
    let initial_zoom = hdim_image.width as f32 / 100.0;

    let app = App::new(hdim_image, initial_zoom)?;
    let result = run(terminal, app);

    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, &mut app))?;

        if handle_events(&mut app)? {
            break;
        }
    }
    Ok(())
}
