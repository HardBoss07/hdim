use crate::consts::RNG_SEED;
use image::{DynamicImage, GenericImage, Rgba};
use palette::{FromColor, Lch, Srgb};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn apply_grain(image: &DynamicImage, value: f32) -> DynamicImage {
    if value <= 0.0 {
        return image.clone();
    }
    let mut cloned_image = image.clone();
    let amount = value / 100.0 * 10.0; // Max lightness change
    let mut rng = StdRng::seed_from_u64(RNG_SEED);

    for (x, y, pixel) in cloned_image.to_rgba8().enumerate_pixels() {
        let srgb = Srgb::new(
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        );

        let mut lch: Lch = Lch::from_color(srgb);
        let noise = rng.gen_range(-amount..amount);
        lch.l += noise;

        let new_srgb = Srgb::from_color(lch);

        let r = (new_srgb.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let g = (new_srgb.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let b = (new_srgb.blue * 255.0).round().clamp(0.0, 255.0) as u8;

        cloned_image.put_pixel(x, y, Rgba([r, g, b, pixel[3]]));
    }

    cloned_image
}
