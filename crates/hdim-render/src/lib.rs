pub mod pixel;
pub mod view;

use anyhow::Result;
use image::DynamicImage;
use std::fmt::Write;

use self::pixel::get_average_rgb;
pub use self::view::View;

/// Renders a portion of an image to a string using half-block characters.
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

fn write_ansi_cell(output: &mut String, top: [u8; 3], bottom: [u8; 3]) -> Result<()> {
    write!(
        output,
        "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m▄",
        top[0], top[1], top[2], bottom[0], bottom[1], bottom[2]
    )?;
    Ok(())
}
