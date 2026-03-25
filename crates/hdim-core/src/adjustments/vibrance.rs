//! Vibrance adjustment.
//!
//! Increases the intensity of muted colors more than already saturated colors, preventing clipping.

use crate::consts::VIBRANCE_MAX_CHROMA;
use image::{DynamicImage, GenericImage, Rgba};
use palette::{FromColor, Lch, Srgb};

/// Applies a vibrance adjustment to the image.
///
/// This function uses the Lch color space to scale the chroma component.
/// It applies more scaling to pixels with lower chroma to avoid over-saturating already vivid areas,
/// acting as a "smart saturation".
///
/// # Arguments
///
/// * `image` - A reference to the input [DynamicImage].
/// * `value` - The vibrance value, typically between -100.0 and 100.0.
///
/// # Returns
///
/// A new [DynamicImage] with the vibrance adjustment applied.
///
/// # Examples
///
/// ```no_run
/// use hdim_core::adjustments::vibrance::apply_vibrance;
/// use image::DynamicImage;
///
/// let img = DynamicImage::new_rgba8(100, 100);
/// let adjusted = apply_vibrance(&img, 40.0);
/// ```
pub fn apply_vibrance(image: &DynamicImage, value: f32) -> DynamicImage {
    let mut cloned_image = image.clone();
    let factor = value / 100.0;

    for (x, y, pixel) in cloned_image.to_rgba8().enumerate_pixels() {
        let srgb = Srgb::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        );

        let mut lch: Lch = Lch::from_color(srgb);

        let scaling_factor = 1.0 - (lch.chroma / VIBRANCE_MAX_CHROMA).powf(2.0);
        lch.chroma *= 1.0 + factor * scaling_factor;

        let new_srgb = Srgb::from_color(lch);

        let r = (new_srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (new_srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (new_srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;

        cloned_image.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
    }

    cloned_image
}
