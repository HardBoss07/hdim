//! Hue adjustment.
//!
//! Rotates the colors of the image around the color wheel.

use image::DynamicImage;

/// Applies a hue rotation to the image.
///
/// This function rotates the hue of all pixels in the image.
/// The input value is mapped from -100..100 to -180..180 degrees.
///
/// # Arguments
///
/// * `image` - A reference to the input [DynamicImage].
/// * `value` - The hue rotation value, typically between -100.0 and 100.0.
///
/// # Returns
///
/// A new [DynamicImage] with the hue rotation applied.
///
/// # Examples
///
/// ```no_run
/// use hdim_core::adjustments::hue::apply_hue;
/// use image::DynamicImage;
///
/// let img = DynamicImage::new_rgba8(100, 100);
/// let adjusted = apply_hue(&img, 50.0);
/// ```
pub fn apply_hue(image: &DynamicImage, value: f32) -> DynamicImage {
    let value = (value * 1.8) as i32;
    image.huerotate(value)
}
