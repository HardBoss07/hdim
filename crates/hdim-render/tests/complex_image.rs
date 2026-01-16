use hdim_render::Renderer;
use image::{RgbaImage, Rgba};

#[test]
fn test_complex_render() {
    let raw_img = generate_benchmark_image();
    let img = image::DynamicImage::ImageRgba8(raw_img);

    // Test with area_size 1 (High Res)
    let renderer = Renderer::new(2);
    let output = renderer.render(&img).unwrap();
    
    // Debug statement
    println!("{}", output);

    // Verify with snapshot
    insta::assert_snapshot!(output);
}

fn generate_benchmark_image() -> RgbaImage {
    let size = 256;
    let mut img = RgbaImage::new(size, size);

    for y in 0..size {
        for x in 0..size {
            // 1. Background Gradient (Blue to Black)
            let b = (y as f32 / size as f32 * 255.0) as u8;
            let mut color = [0, 0, b, 255];

            // 2. Horizontal & Vertical White Lines (Grid test)
            if x % 32 == 0 || y % 32 == 0 {
                color = [200, 200, 200, 255];
            }

            // 3. Diagonal Red Line (Check aliasing/stepping)
            if x == y || x == (size - y - 1) {
                color = [255, 0, 0, 255];
            }

            // 4. Color Circles (Center check)
            let dx = x as i32 - (size / 2) as i32;
            let dy = y as i32 - (size / 2) as i32;
            let dist_sq = dx * dx + dy * dy;

            if dist_sq < 2500 { // Large Green Circle
                color = [0, 255, 0, 255];
            }
            if dist_sq < 400 {  // Small Yellow Core
                color = [255, 255, 0, 255];
            }

            // 5. Brightness Ramp (Top edge)
            if y < 10 {
                let brightness = (x as f32 / size as f32 * 255.0) as u8;
                color = [brightness, brightness, brightness, 255];
            }

            img.put_pixel(x, y, Rgba(color));
        }
    }
    img
}