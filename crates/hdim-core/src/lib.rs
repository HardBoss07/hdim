use image::DynamicImage;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

pub fn calculate_resize(img: &DynamicImage, max_size: Size) -> Size {
    use image::GenericImageView;
    let (w, h) = img.dimensions();
    
    // Terminal cells are taller (approx 1:2 ratio)
    // We target a "virtual" canvas that is double the terminal height
    let target_w = max_size.width;
    let target_h = max_size.height * 2; 
    
    let width_ratio = target_w as f64 / w as f64;
    let height_ratio = target_h as f64 / h as f64;
    let ratio = width_ratio.min(height_ratio);
    
    Size {
        width: (w as f64 * ratio) as u32,
        height: (h as f64 * ratio) as u32,
    }
}