use crate::consts::CONTRAST_MID_POINT;
use image::{DynamicImage, GenericImage, Rgba};
use palette::{FromColor, Lch, Srgb};

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
