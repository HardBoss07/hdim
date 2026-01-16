use image::{GenericImageView, DynamicImage};
use anyhow::Result;
use std::fmt::Write;

pub fn render_half_block(img: &DynamicImage) -> Result<String> {
    let (w, h) = img.dimensions();
    let mut output = String::new();

    for y in (0..h).step_by(2) {
        for x in 0..w {
            let top = img.get_pixel(x, y);
            // Handle bottom pixel boundary check
            let bot = if y + 1 < h {
                img.get_pixel(x, y + 1)
            } else {
                top 
            };

            write!(
                output,
                "\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m▄",
                top[0], top[1], top[2],
                bot[0], bot[1], bot[2]
            )?;
        }
        output.push_str("\x1b[0m\n");
    }
    Ok(output)
}