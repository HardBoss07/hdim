//! Fade adjustment.
//!
//! Applies a fade effect by blending the image with a medium gray color.

use image::{DynamicImage, GenericImage, Rgba};
use palette::Srgb;

/// Applies a fade effect to the image.
///
/// This function blends the image's RGB values with a medium gray color (0.5, 0.5, 0.5),
/// reducing contrast in shadows and highlights for a "matte" look.
///
/// # Arguments
///
/// * `image` - A reference to the input [DynamicImage].
/// * `value` - The fade value, typically between 0.0 and 100.0.
///
/// # Returns
///
/// A new [DynamicImage] with the fade effect applied.
///
/// # Examples
///
/// ```no_run
/// use hdim_core::adjustments::fade::apply_fade;
/// use image::DynamicImage;
///
/// let img = DynamicImage::new_rgba8(100, 100);
/// let adjusted = apply_fade(&img, 30.0);
/// ```
pub fn apply_fade(image: &DynamicImage, value: f32) -> DynamicImage {
    let mut cloned_image = image.clone();
    let factor = value / 100.0;
    let fade_color = Srgb::new(0.5, 0.5, 0.5); // Medium gray

    if factor <= 0.0 {
        return cloned_image;
    }

    for (x, y, pixel) in cloned_image.to_rgba8().enumerate_pixels() {
        let srgb = Srgb::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        );

        let new_srgb = Srgb::new(
            srgb.red * (1.0 - factor) + fade_color.red * factor,
            srgb.green * (1.0 - factor) + fade_color.green * factor,
            srgb.blue * (1.0 - factor) + fade_color.blue * factor,
        );

        let r = (new_srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (new_srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (new_srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;

        cloned_image.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
    }

    cloned_image
}
