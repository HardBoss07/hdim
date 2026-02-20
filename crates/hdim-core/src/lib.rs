pub mod adjustments;
#[cfg(feature = "exif")]
pub mod exif;
pub mod history;
pub mod state;
pub mod utils;
use crate::history::history::History;
use anyhow::Result;
use image::{DynamicImage, GenericImageView};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Adjustments {
    pub brightness: f32,
    pub contrast: f32,
    pub exposure: f32,
    pub fade: f32,
    pub grain: f32,
    pub hue: f32,
    pub noise: f32,
    pub saturation: f32,
    pub vibrance: f32,
    pub warmth: f32,
}

impl Default for Adjustments {
    fn default() -> Self {
        Self {
            brightness: 0.0,
            contrast: 0.0,
            exposure: 0.0,
            fade: 0.0,
            grain: 0.0,
            hue: 0.0,
            noise: 0.0,
            saturation: 0.0,
            vibrance: 0.0,
            warmth: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HdimImage {
    pub path: PathBuf,
    pub data: DynamicImage,
    pub width: u32,
    pub height: u32,
    pub adjustments: Adjustments,
    pub history: History, // Refer to it as History
}

impl HdimImage {
    pub fn from_path(path: &Path) -> Result<Self> {
        let data = image::open(path)?;
        let (width, height) = data.dimensions();
        let adjustments = Adjustments::default();

        Ok(HdimImage {
            path: path.to_path_buf(),
            data,
            width,
            height,
            adjustments,
            history: History::new(adjustments), // Refer to it as History
        })
    }

    pub fn apply_adjustments(&self) -> DynamicImage {
        let mut adjusted_image = self.data.clone();
        let adj = self.adjustments;

        // Order of application matters
        if adj.exposure != 0.0 {
            adjusted_image = adjustments::exposure::apply_exposure(&adjusted_image, adj.exposure);
        }
        if adj.warmth != 0.0 {
            adjusted_image = adjustments::warmth::apply_warmth(&adjusted_image, adj.warmth);
        }
        if adj.contrast != 0.0 {
            adjusted_image = adjustments::contrast::apply_contrast(&adjusted_image, adj.contrast);
        }
        // Vibrance and Saturation might interact, applying them sequentially
        // or choosing one over the other if both are non-zero.
        // For now, apply both if set.
        if adj.vibrance != 0.0 {
            adjusted_image = adjustments::vibrance::apply_vibrance(&adjusted_image, adj.vibrance);
        }
        if adj.saturation != 0.0 {
            adjusted_image =
                adjustments::saturation::apply_saturation(&adjusted_image, adj.saturation);
        }
        if adj.hue != 0.0 {
            adjusted_image = adjustments::hue::apply_hue(&adjusted_image, adj.hue);
        }
        if adj.brightness != 0.0 {
            adjusted_image =
                adjustments::brightness::apply_brightness(&adjusted_image, adj.brightness);
        }
        if adj.fade != 0.0 {
            adjusted_image = adjustments::fade::apply_fade(&adjusted_image, adj.fade);
        }
        if adj.grain != 0.0 {
            adjusted_image = adjustments::grain::apply_grain(&adjusted_image, adj.grain);
        }
        if adj.noise != 0.0 {
            adjusted_image = adjustments::noise::apply_noise(&adjusted_image, adj.noise);
        }

        adjusted_image
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
