use crate::state::TransformState;
use image::{DynamicImage, GenericImageView};

pub fn apply_transform(image: &DynamicImage, transform: &TransformState) -> DynamicImage {
    let mut transformed = image.clone();

    // 1. Rotation
    match transform.rotation {
        90 => transformed = transformed.rotate90(),
        180 => transformed = transformed.rotate180(),
        270 => transformed = transformed.rotate270(),
        _ => {}
    }

    // 2. Flip
    if transform.flip_horizontal {
        transformed = transformed.fliph();
    }
    if transform.flip_vertical {
        transformed = transformed.flipv();
    }

    // 3. Crop
    let (width, height) = transformed.dimensions();

    // We don't have a way to store "is_percent" in TransformState yet,
    // but the UI could pass very large values or we could change TransformState.
    // However, the task said "Implement relative (%) and absolute (px) cropping (detect % in input)".
    // Since TransformState only stores u32, I might need to change it or use a convention.
    // Let's assume values > 10000 might be percentages * 100? No, that's messy.

    // Better: change TransformState to store strings or have separate fields,
    // or just handle it in the UI and convert to absolute pixels before calling this.
    // But the requirement says "hdim-core: Implement... detect % in input".
    // This implies hdim-core should handle the detection if we pass it a string,
    // or we have a more complex state.

    // Let's stick to absolute pixels for now as requested by the current struct,
    // but I'll add a helper that takes the original dimensions.

    let left = transform.left.min(width);
    let right = transform.right.min(width - left);
    let top = transform.top.min(height);
    let bottom = transform.bottom.min(height - top);

    if left > 0 || right > 0 || top > 0 || bottom > 0 {
        let crop_width = width.saturating_sub(left).saturating_sub(right);
        let crop_height = height.saturating_sub(top).saturating_sub(bottom);
        if crop_width > 0 && crop_height > 0 {
            transformed = transformed.crop_imm(left, top, crop_width, crop_height);
        }
    }

    transformed
}

pub fn calculate_absolute_crop(value_str: &str, total: u32) -> u32 {
    if let Some(stripped) = value_str.strip_suffix('%')
        && let Ok(percent) = stripped.parse::<f32>()
    {
        return ((percent / 100.0) * total as f32).round() as u32;
    }
    value_str.parse::<u32>().unwrap_or(0)
}

/// Helper function to perform flip horizontally.
pub fn flip_horizontal(image: &DynamicImage) -> DynamicImage {
    image.fliph()
}

/// Helper function to perform flip vertically.
pub fn flip_vertical(image: &DynamicImage) -> DynamicImage {
    image.flipv()
}

/// Helper function to rotate 90 degrees clockwise.
pub fn rotate_90(image: &DynamicImage) -> DynamicImage {
    image.rotate90()
}

/// Helper function to rotate 270 degrees clockwise (90 degrees counter-clockwise).
pub fn rotate_270(image: &DynamicImage) -> DynamicImage {
    image.rotate270()
}
