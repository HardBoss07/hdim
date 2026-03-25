//! Contrast adjustment.
//!
//! Increases or decreases the overall contrast of an image by adjusting lightness relative to a midpoint.

use crate::consts::CONTRAST_MID_POINT;
use image::{DynamicImage, GenericImage, Rgba};
use palette::{FromColor, Lch, Srgb};

/// Applies a contrast adjustment to the image.
///
/// This function uses the Lch color space to adjust the lightness component relative to a midpoint.
/// It effectively stretches or compresses the lightness range around the defined midpoint.
///
/// # Arguments
///
/// * `image` - A reference to the input [DynamicImage].
/// * `value` - The contrast value, typically between -100.0 and 100.0.
///
/// # Returns
///
/// A new [DynamicImage] with the contrast adjustment applied.
///
/// # Examples
///
/// ```no_run
/// use hdim_core::adjustments::contrast::apply_contrast;
/// use image::DynamicImage;
///
/// let img = DynamicImage::new_rgba8(100, 100);
/// let adjusted = apply_contrast(&img, 20.0);
/// ```
pub fn apply_contrast(image: &DynamicImage, value: f32) -> DynamicImage {
    let mut cloned_image = image.clone();
    let factor = value / 100.0 * 2.0; // Map -100..100 to -2.0..2.0

    for (x, y, pixel) in cloned_image.to_rgba8().enumerate_pixels() {
        let srgb = Srgb::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        );

        let mut lch: Lch = Lch::from_color(srgb);

        // Adjust lightness away from midpoint
        lch.l = CONTRAST_MID_POINT + (lch.l - CONTRAST_MID_POINT) * (1.0 + factor);
        lch.l = lch.l.clamp(0.0, 100.0); // Clamp L to its valid range

        let new_srgb = Srgb::from_color(lch);

        let r = (new_srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (new_srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (new_srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;

        cloned_image.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
    }

    cloned_image
}
