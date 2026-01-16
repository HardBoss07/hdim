use hdim_core::{calculate_resize, Size};
use image::{DynamicImage, RgbaImage};

fn make_dummy_image(w: u32, h: u32) -> DynamicImage {
    DynamicImage::ImageRgba8(RgbaImage::new(w, h))
}

#[test]
fn test_resize_logic_external() {
    let img = make_dummy_image(100, 100);
    let max = Size { width: 50, height: 50 };
    
    let result = calculate_resize(&img, max);
    
    assert!(result.width <= 50);
    // Remember our logic doubles the height budget for terminal cells
    assert!(result.height <= 100); 
}