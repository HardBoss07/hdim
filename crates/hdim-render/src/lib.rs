//! High-performance terminal image renderer.
//!
//! This crate handles the translation of [DynamicImage] buffers into ANSI escape sequences
//! suitable for display in a terminal emulator. It uses "half-block" characters (`▄`)
//! to effectively double the vertical resolution of the terminal grid.

pub mod pixel;
pub mod view;

use anyhow::Result;
use image::DynamicImage;
use std::fmt::Write;

use self::pixel::get_average_rgb;
pub use self::view::View;

/// Renders a portion of an image to an ANSI string using half-block characters.
///
/// This function maps the source image pixels to terminal cells based on the provided [View].
/// It calculates the average color for the top and bottom half of each character cell
/// to determine the foreground and background colors.
///
/// # Arguments
///
/// * `image` - The source [DynamicImage] to render.
/// * `view` - The [View] configuration defining the source region and target dimensions.
///
/// # Errors
///
/// Returns an error if writing to the output string fails (though this is unlikely with `String`).
///
/// # Examples
///
/// ```no_run
/// use hdim_render::{render, View};
/// use image::DynamicImage;
///
/// let img = DynamicImage::new_rgba8(100, 100);
/// let view = View {
///     source_x: 0,
///     source_y: 0,
///     source_width: 100,
///     source_height: 100,
///     target_width: 50,
///     target_height: 25,
/// };
/// let output = render(&img, &view).unwrap();
/// println!("{}", output);
/// ```
pub fn render(image: &DynamicImage, view: &View) -> Result<String> {
    let mut output = String::new();
    let (x_ratio, y_ratio) = view.calculate_scaling();
    let (block_width, block_height) = view.calculate_block_size(x_ratio, y_ratio);

    for y in 0..view.target_height {
        for x in 0..view.target_width {
            let (source_x, source_y_top, source_y_bottom) =
                view.map_to_source(x, y, x_ratio, y_ratio);

            let (top_color, bottom_color) = calculate_cell_colors(
                image,
                source_x,
                source_y_top,
                source_y_bottom,
                block_width,
                block_height,
            );

            write_ansi_cell(&mut output, top_color, bottom_color)?;
        }
        output.push_str("\x1b[0m\n");
    }
    Ok(output)
}

/// Helper to calculate the average RGB values for the top and bottom halves of a character cell.
fn calculate_cell_colors(
    image: &DynamicImage,
    source_x: u32,
    source_y_top: u32,
    source_y_bottom: u32,
    block_width: u32,
    block_height: u32,
) -> ([u8; 3], [u8; 3]) {
    let top_color = get_average_rgb(image, source_x, source_y_top, block_width, block_height);
    let bottom_color = get_average_rgb(image, source_x, source_y_bottom, block_width, block_height);
    (top_color, bottom_color)
}

/// Writes a single ANSI-colored half-block character to the output buffer.
fn write_ansi_cell(output: &mut String, top: [u8; 3], bottom: [u8; 3]) -> Result<()> {
    write!(
        output,
        "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m▄",
        top[0], top[1], top[2], bottom[0], bottom[1], bottom[2]
    )?;
    Ok(())
}
