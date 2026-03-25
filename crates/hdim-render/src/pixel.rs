//! Pixel manipulation utilities.
//!
//! Provides functions for calculating average colors over image regions, essential for
//! downscaling images to fit the terminal grid.

use image::{DynamicImage, GenericImageView};

/// Calculates the average RGB color for a specific rectangular area of the image.
///
/// This function iterates over the pixels within the specified bounds and computes
/// the arithmetic mean of the Red, Green, and Blue channels.
///
/// # Arguments
///
/// * `image` - A reference to the input [DynamicImage].
/// * `start_x` - The top-left X coordinate of the area.
/// * `start_y` - The top-left Y coordinate of the area.
/// * `width` - The width of the area to sample.
/// * `height` - The height of the area to sample.
///
/// # Returns
///
/// An array `[r, g, b]` representing the average color. Returns `[0, 0, 0]` (black)
/// if the area contains no pixels or is out of bounds.
pub fn get_average_rgb(
    image: &DynamicImage,
    start_x: u32,
    start_y: u32,
    width: u32,
    height: u32,
) -> [u8; 3] {
    let (image_width, image_height) = image.dimensions();
    let mut r_total: u64 = 0;
    let mut g_total: u64 = 0;
    let mut b_total: u64 = 0;
    let mut count: u64 = 0;

    // Define the boundaries of the area to iterate over, clamping to image dimensions
    let end_y = (start_y + height).min(image_height);
    let end_x = (start_x + width).min(image_width);

    for pixel_y in start_y..end_y {
        for pixel_x in start_x..end_x {
            let pixel = image.get_pixel(pixel_x, pixel_y);
            r_total += pixel[0] as u64;
            g_total += pixel[1] as u64;
            b_total += pixel[2] as u64;
            count += 1;
        }
    }

    if count == 0 {
        return [0, 0, 0];
    }

    [
        (r_total / count) as u8,
        (g_total / count) as u8,
        (b_total / count) as u8,
    ]
}
