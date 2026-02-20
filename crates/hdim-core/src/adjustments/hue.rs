use image::DynamicImage;

pub fn apply_hue(image: &DynamicImage, value: f32) -> DynamicImage {
    let value = (value * 1.8) as i32;
    image.huerotate(value)
}
