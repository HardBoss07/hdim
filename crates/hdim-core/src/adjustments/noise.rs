use image::{DynamicImage, GenericImage, Rgba};
use rand::Rng;

pub fn apply_noise(image: &DynamicImage, value: f32) -> DynamicImage {
    if value <= 0.0 {
        return image.clone();
    }
    let mut cloned_image = image.clone();
    let amount = value as i16;
    let mut rng = rand::thread_rng();

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
