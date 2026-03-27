//! Grain adjustment.
//!
//! Adds film-like grain to the image by introducing random noise to the lightness component.

use crate::consts::RNG_SEED;
use image::DynamicImage;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Applies a grain effect to the image.
///
/// This function adds random noise to the lightness component of each pixel in the Lch color space.
/// It uses a fixed seed [StdRng] to ensure deterministic output for the same input value.
///
/// # Arguments
///
/// * `image` - A reference to the input [DynamicImage].
/// * `value` - The grain intensity, typically between 0.0 and 100.0.
///
/// # Returns
///
/// A new [DynamicImage] with the grain effect applied.
///
/// # Examples
///
/// ```no_run
/// use hdim_core::adjustments::grain::apply_grain;
/// use image::DynamicImage;
///
/// let img = DynamicImage::new_rgba8(100, 100);
/// let adjusted = apply_grain(&img, 25.0);
/// ```
pub fn apply_grain(image: &DynamicImage, value: f32) -> DynamicImage {
    if value <= 0.0 {
        return image.clone();
    }
    let mut rgba_image = image.to_rgba8();
    let amount = (value * 2.55) as i16; // Scale 0-100 to 0-255 range
    let mut rng = StdRng::seed_from_u64(RNG_SEED);

    for pixel in rgba_image.pixels_mut() {
        let gray_noise = rng.gen_range(-amount..amount);

        // Apply same noise to all channels for gray/luminance noise
        pixel[0] = (pixel[0] as i16 + gray_noise).clamp(0, 255) as u8;
        pixel[1] = (pixel[1] as i16 + gray_noise).clamp(0, 255) as u8;
        pixel[2] = (pixel[2] as i16 + gray_noise).clamp(0, 255) as u8;
    }

    DynamicImage::ImageRgba8(rgba_image)
}
