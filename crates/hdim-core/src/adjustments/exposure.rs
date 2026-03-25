//! Exposure adjustment.
//!
//! Simulates changes in camera exposure by scaling the RGB values.

use image::{DynamicImage, GenericImage, Rgba};
use palette::Srgb;

/// Applies an exposure adjustment to the image.
///
/// This function scales the RGB values of each pixel using a power-of-two factor
/// derived from the input value, simulating stops of light.
///
/// # Arguments
///
/// * `image` - A reference to the input [DynamicImage].
/// * `value` - The exposure value, typically between -100.0 and 100.0.
///
/// # Returns
///
/// A new [DynamicImage] with the exposure adjustment applied.
///
/// # Examples
///
/// ```no_run
/// use hdim_core::adjustments::exposure::apply_exposure;
/// use image::DynamicImage;
///
/// let img = DynamicImage::new_rgba8(100, 100);
/// let adjusted = apply_exposure(&img, 100.0);
/// ```
pub fn apply_exposure(image: &DynamicImage, value: f32) -> DynamicImage {
    let mut cloned_image = image.clone();
    let factor = 2.0f32.powf(value / 100.0);

    for (x, y, pixel) in cloned_image.to_rgba8().enumerate_pixels() {
        let srgb = Srgb::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        );

        let new_srgb = Srgb::new(srgb.red * factor, srgb.green * factor, srgb.blue * factor);

        let r = (new_srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (new_srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (new_srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;

        cloned_image.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
    }

    cloned_image
}
