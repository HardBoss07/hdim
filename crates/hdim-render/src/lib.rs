pub mod pixel;
pub mod view;

use anyhow::Result;
use image::DynamicImage;
use std::fmt::Write;

use self::pixel::get_average_rgb;
pub use self::view::View;

/// Renders a portion of an image to a string using half-block characters.
///
/// The rendering is defined by the `View` struct, which maps a source rectangle
/// from the image to a target area in the terminal.
pub fn render(image: &DynamicImage, view: &View) -> Result<String> {
    let mut output = String::new();

    // Calculate the number of source pixels that correspond to one terminal character cell.
    // Use floating point for precision to avoid cumulative errors.
    let x_ratio = view.source_width as f32 / view.target_width as f32;
    let y_ratio = view.source_height as f32 / view.target_height as f32;

    // Since each character cell represents two vertical pixels (top and bottom half),
    // the height of the pixel block for one half is half the y_ratio.
    // However, the loop iterates `target_height` times, and each iteration handles a full character,
    // which covers `y_ratio` pixels in height. The logic inside handles the two halves.
    let top_block_height = (y_ratio / 2.0).round().max(1.0) as u32;
    let bottom_block_height = top_block_height; // Keep it simple for now
    let block_width = x_ratio.round().max(1.0) as u32;

    for y in 0..view.target_height {
        for x in 0..view.target_width {
            // Calculate the source pixel coordinates for the top half-block
            let source_pixel_x = view.source_x + (x as f32 * x_ratio) as u32;
            let source_pixel_y_top = view.source_y + (y as f32 * y_ratio) as u32;

            let top = get_average_rgb(
                image,
                source_pixel_x,
                source_pixel_y_top,
                block_width,
                top_block_height,
            );

            // Calculate the source pixel coordinates for the bottom half-block
            let source_pixel_y_bot = source_pixel_y_top + top_block_height;

            let bot = get_average_rgb(
                image,
                source_pixel_x,
                source_pixel_y_bot,
                block_width,
                bottom_block_height,
            );

            write!(
                output,
                "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m▄",
                top[0], top[1], top[2], bot[0], bot[1], bot[2]
            )?;
        }
        output.push_str("\x1b[0m\n");
    }
    Ok(output)
}
