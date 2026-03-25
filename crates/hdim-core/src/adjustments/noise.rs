//! Noise adjustment.
//!
//! Adds random color noise to each RGB channel of the image.

use crate::consts::RNG_SEED;
use image::{DynamicImage, GenericImage, Rgba};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Applies a noise effect to the image.
///
/// This function adds random values to the red, green, and blue channels of each pixel.
/// It uses a fixed seed [StdRng] to ensure deterministic output for the same input value.
///
/// # Arguments
///
/// * `image` - A reference to the input [DynamicImage].
/// * `value` - The noise intensity, typically between 0.0 and 100.0.
///
/// # Returns
///
/// A new [DynamicImage] with the noise effect applied.
///
/// # Examples
///
/// ```no_run
/// use hdim_core::adjustments::noise::apply_noise;
/// use image::DynamicImage;
///
/// let img = DynamicImage::new_rgba8(100, 100);
/// let adjusted = apply_noise(&img, 15.0);
/// ```
pub fn apply_noise(image: &DynamicImage, value: f32) -> DynamicImage {
    if value <= 0.0 {
        return image.clone();
    }
    let mut cloned_image = image.clone();
    let amount = value as i16;
    // Use a fixed seed to ensure deterministic noise for the same value
    let mut rng = StdRng::seed_from_u64(RNG_SEED);

    for (x, y, pixel) in cloned_image.to_rgba8().enumerate_pixels() {
        let r_noise = rng.gen_range(-amount..amount);
        let g_noise = rng.gen_range(-amount..amount);
        let b_noise = rng.gen_range(-amount..amount);

        let r = (pixel[0] as i16 + r_noise).clamp(0, 255) as u8;
        let g = (pixel[1] as i16 + g_noise).clamp(0, 255) as u8;
        let b = (pixel[2] as i16 + b_noise).clamp(0, 255) as u8;

        cloned_image.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
    }

    cloned_image
}
