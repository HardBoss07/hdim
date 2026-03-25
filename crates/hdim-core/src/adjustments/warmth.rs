//! Warmth adjustment.
//!
//! Adjusts the color temperature of the image by shifting its hue.

use image::{DynamicImage, GenericImage, Rgba};
use palette::{FromColor, Lch, Srgb};

/// Applies a warmth adjustment to the image.
///
/// This function uses the Lch color space to shift the hue component.
/// Positive values shift towards yellow (warmer), while negative values shift towards blue (cooler).
///
/// # Arguments
///
/// * `image` - A reference to the input [DynamicImage].
/// * `value` - The warmth value, typically between -100.0 and 100.0.
///             The value is mapped from -100..100 to -40..40 degrees of hue shift.
///
/// # Returns
///
/// A new [DynamicImage] with the warmth adjustment applied.
///
/// # Examples
///
/// ```no_run
/// use hdim_core::adjustments::warmth::apply_warmth;
/// use image::DynamicImage;
///
/// let img = DynamicImage::new_rgba8(100, 100);
/// let adjusted = apply_warmth(&img, 30.0);
/// ```
pub fn apply_warmth(image: &DynamicImage, value: f32) -> DynamicImage {
    let mut cloned_image = image.clone();
    let factor = value * 0.4; // Map -100..100 to -40..40 degrees

    for (x, y, pixel) in cloned_image.to_rgba8().enumerate_pixels() {
        let srgb = Srgb::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        );

        let mut lch: Lch = Lch::from_color(srgb);
        lch.hue = lch.hue + factor;

        let new_srgb = Srgb::from_color(lch);

        let r = (new_srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (new_srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (new_srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;

        cloned_image.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
    }

    cloned_image
}
