pub mod adjustments;
pub mod consts;
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
        adjusted_image = self.apply_light_adjustments(adjusted_image, &adj);
        adjusted_image = self.apply_color_adjustments(adjusted_image, &adj);
        adjusted_image = self.apply_effect_adjustments(adjusted_image, &adj);

        adjusted_image
    }

    fn apply_light_adjustments(&self, mut image: DynamicImage, adj: &Adjustments) -> DynamicImage {
        if adj.exposure != 0.0 {
            image = adjustments::exposure::apply_exposure(&image, adj.exposure);
        }
        if adj.brightness != 0.0 {
            image = adjustments::brightness::apply_brightness(&image, adj.brightness);
        }
        if adj.contrast != 0.0 {
            image = adjustments::contrast::apply_contrast(&image, adj.contrast);
        }
        image
    }

    fn apply_color_adjustments(&self, mut image: DynamicImage, adj: &Adjustments) -> DynamicImage {
        if adj.warmth != 0.0 {
            image = adjustments::warmth::apply_warmth(&image, adj.warmth);
        }
        if adj.vibrance != 0.0 {
            image = adjustments::vibrance::apply_vibrance(&image, adj.vibrance);
        }
        if adj.saturation != 0.0 {
            image = adjustments::saturation::apply_saturation(&image, adj.saturation);
        }
        if adj.hue != 0.0 {
            image = adjustments::hue::apply_hue(&image, adj.hue);
        }
        image
    }

    fn apply_effect_adjustments(&self, mut image: DynamicImage, adj: &Adjustments) -> DynamicImage {
        if adj.fade != 0.0 {
            image = adjustments::fade::apply_fade(&image, adj.fade);
        }
        if adj.grain != 0.0 {
            image = adjustments::grain::apply_grain(&image, adj.grain);
        }
        if adj.noise != 0.0 {
            image = adjustments::noise::apply_noise(&image, adj.noise);
        }
        image
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
