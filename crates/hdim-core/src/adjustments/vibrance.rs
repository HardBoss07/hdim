use image::{DynamicImage, GenericImage, Rgba};
use palette::{FromColor, Lch, Srgb};

pub fn apply_vibrance(image: &DynamicImage, value: f32) -> DynamicImage {
    let mut cloned_image = image.clone();
    let factor = value / 100.0;
    const MAX_CHROMA: f32 = 150.0;

    for (x, y, pixel) in cloned_image.to_rgba8().enumerate_pixels() {
        let srgb = Srgb::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        );

        let mut lch: Lch = Lch::from_color(srgb);

        let scaling_factor = 1.0 - (lch.chroma / MAX_CHROMA).powf(2.0);
        lch.chroma *= 1.0 + factor * scaling_factor;

        let new_srgb = Srgb::from_color(lch);

        let r = (new_srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (new_srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (new_srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;

        cloned_image.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
    }

    cloned_image
}
