use hdim_render::render_half_block;
use image::{DynamicImage, RgbaImage, Rgba};

#[test]
fn test_render_snapshot() {
    // 1. Setup Data
    let mut buffer = RgbaImage::new(2, 2);
    buffer.put_pixel(0, 0, Rgba([255, 0, 0, 255]));   // Red
    buffer.put_pixel(1, 0, Rgba([0, 0, 255, 255]));   // Blue
    buffer.put_pixel(0, 1, Rgba([0, 255, 0, 255]));   // Green
    buffer.put_pixel(1, 1, Rgba([255, 255, 255, 255])); // White
    let img = DynamicImage::ImageRgba8(buffer);

    // 2. Run System Under Test
    let output = render_half_block(&img).unwrap();
    
    println!("{}", output);
    println!("{:?}", output);
    // 3. Verify with Snapshot
    insta::assert_snapshot!(output);
}