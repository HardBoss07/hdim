#[cfg(feature = "exif")]
pub mod exif;
pub mod state;
use anyhow::Result;
use image::{DynamicImage, GenericImageView};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct HdimImage {
    pub path: PathBuf,
    pub data: DynamicImage,
    pub width: u32,
    pub height: u32,
}

impl HdimImage {
    pub fn from_path(path: &Path) -> Result<Self> {
        let data = image::open(path)?;
        let (width, height) = data.dimensions();

        Ok(HdimImage {
            path: path.to_path_buf(),
            data,
            width,
            height,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

pub fn calculate_resize(image: &DynamicImage, max_size: Size) -> Size {
    let (width, height) = image.dimensions();

    // Terminal cells are taller (approx 1:2 ratio)
    // We target a "virtual" canvas that is double the terminal height
    let target_width = max_size.width;
    let target_height = max_size.height * 2;

    let width_ratio = target_width as f64 / width as f64;
    let height_ratio = target_height as f64 / height as f64;
    let ratio = width_ratio.min(height_ratio);

    Size {
        width: (width as f64 * ratio) as u32,
        height: (height as f64 * ratio) as u32,
    }
}
