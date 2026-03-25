//! Saturation adjustment.
//!
//! Increases or decreases the intensity of colors in the image.

use image::{DynamicImage, GenericImage, Rgba};
use palette::{FromColor, Lch, Srgb};

/// Applies a saturation adjustment to the image.
///
/// This function uses the Lch color space to scale the chroma component of each pixel,
/// making colors more or less intense.
///
/// # Arguments
///
/// * `image` - A reference to the input [DynamicImage].
/// * `value` - The saturation value, typically between -100.0 and 100.0.
///
/// # Returns
///
/// A new [DynamicImage] with the saturation adjustment applied.
///
/// # Examples
///
/// ```no_run
/// use hdim_core::adjustments::saturation::apply_saturation;
/// use image::DynamicImage;
///
/// let img = DynamicImage::new_rgba8(100, 100);
/// let adjusted = apply_saturation(&img, 50.0);
/// ```
pub fn apply_saturation(image: &DynamicImage, value: f32) -> DynamicImage {
    let mut cloned_image = image.clone();
    let factor = value / 100.0;

    for (x, y, pixel) in cloned_image.to_rgba8().enumerate_pixels() {
        let srgb = Srgb::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        );

        let mut lch: Lch = Lch::from_color(srgb);
        lch.chroma *= 1.0 + factor;

        let new_srgb = Srgb::from_color(lch);

        let r = (new_srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (new_srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (new_srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;

        cloned_image.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
    }

    cloned_image
}
