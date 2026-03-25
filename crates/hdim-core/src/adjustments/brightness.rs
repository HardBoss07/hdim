//! Brightness adjustment.
//!
//! Increases or decreases the overall lightness of an image.

use image::{DynamicImage, GenericImage, Rgba};
use palette::{FromColor, Lch, Srgb};

/// Adjusts the overall lightness of an image.
///
/// This transformation uses the Lch color space to modify the 'l' (lightness)
/// channel, providing a more perceptually uniform adjustment than simple RGB scaling.
///
/// # Arguments
///
/// * `image` - A reference to the input [DynamicImage].
/// * `value` - The brightness value, typically between -100.0 and 100.0.
///
/// # Returns
///
/// A new [DynamicImage] with the brightness adjustment applied.
///
/// # Examples
///
/// ```no_run
/// use hdim_core::adjustments::brightness::apply_brightness;
/// use image::DynamicImage;
///
/// let img = DynamicImage::new_rgba8(100, 100);
/// let adjusted = apply_brightness(&img, 50.0);
/// ```
pub fn apply_brightness(image: &DynamicImage, value: f32) -> DynamicImage {
    let mut cloned_image = image.clone();
    let factor = value * 0.5; // Map -100..100 to -50..50 (Lch 'l' is 0-100)

    for (x, y, pixel) in cloned_image.to_rgba8().enumerate_pixels() {
        let srgb = Srgb::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        );

        let mut lch: Lch = Lch::from_color(srgb);
        lch.l += factor;
        lch.l = lch.l.clamp(0.0, 100.0); // Clamp L to its valid range

        let new_srgb = Srgb::from_color(lch);

        let r = (new_srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (new_srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (new_srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;

        cloned_image.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
    }

    cloned_image
}
