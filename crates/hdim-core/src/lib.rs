//! Core logic for High Definition Image Manipulator (hdim).
//!
//! This crate provides the foundational data structures and image processing
//! algorithms used throughout the hdim workspace. It handles image loading,
//! adjustment state management, and the undo/redo history stack.

pub mod adjustments;
pub mod consts;
#[cfg(feature = "exif")]
pub mod exif;
pub mod history;
pub mod localization;
pub mod state;
pub mod transform;
pub mod utils;
use crate::history::History;
use crate::state::TransformState;
use anyhow::Result;
use image::{DynamicImage, GenericImageView};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Represents the set of image adjustments that can be applied to a [HdimImage].
///
/// All adjustments are stored as `f32` values, typically in the range of -100.0 to 100.0,
/// although the exact interpretation depends on the specific adjustment implementation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Adjustments {
    /// Overall lightness of the image.
    pub brightness: f32,
    /// Difference between light and dark areas.
    pub contrast: f32,
    /// Overall light captured in the image.
    pub exposure: f32,
    /// Reduces contrast in shadows for a "matte" look.
    pub fade: f32,
    /// Adds simulated analog texture.
    pub grain: f32,
    /// Shifts the entire color spectrum.
    pub hue: f32,
    /// Adds digital luminance/chroma noise.
    pub noise: f32,
    /// Intensity of colors.
    pub saturation: f32,
    /// Smart saturation that protects skin tones.
    pub vibrance: f32,
    /// Shift between blue (cool) and yellow (warm) tones.
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

/// The primary image structure used for editing sessions.
///
/// `HdimImage` wraps a [DynamicImage] and maintains its adjustment state
/// and modification history.
#[derive(Debug, Clone)]
pub struct HdimImage {
    /// Original path of the image on disk.
    pub path: PathBuf,
    /// Raw image data loaded into memory.
    pub data: Arc<DynamicImage>,
    /// Width of the image in pixels.
    pub width: u32,
    /// Height of the image in pixels.
    pub height: u32,
    /// Current set of adjustments applied to the image.
    pub adjustments: Adjustments,
    /// History of image states for undo/redo functionality.
    pub history: History,
}

impl HdimImage {
    /// Creates a new [HdimImage] by loading it from a file path.
    ///
    /// # Errors
    ///
    /// Returns an error if the image cannot be opened or decoded by the `image` crate.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hdim_core::HdimImage;
    /// use std::path::Path;
    ///
    /// let path = Path::new("tests/images/4k.jpg");
    /// let hdim_image = HdimImage::from_path(path).unwrap();
    /// ```
    pub fn from_path(path: &Path) -> Result<Self> {
        let data = image::open(path)?;
        let (width, height) = data.dimensions();
        let adjustments = Adjustments::default();
        let data_arc = Arc::new(data);

        Ok(HdimImage {
            path: path.to_path_buf(),
            data: data_arc.clone(),
            width,
            height,
            adjustments,
            history: History::new(data_arc, adjustments),
        })
    }

    /// Records the current image state and adjustments into history.
    pub fn record_state(&mut self) {
        self.history
            .record_state(self.data.clone(), self.adjustments);
    }

    /// Moves one step backward in history and updates the current state.
    pub fn undo(&mut self) -> bool {
        if let Some(state) = self.history.undo() {
            self.data = state.data;
            self.adjustments = state.adjustments;
            let (width, height) = self.data.dimensions();
            self.width = width;
            self.height = height;
            true
        } else {
            false
        }
    }

    /// Moves one step forward in history and updates the current state.
    pub fn redo(&mut self) -> bool {
        if let Some(state) = self.history.redo() {
            self.data = state.data;
            self.adjustments = state.adjustments;
            let (width, height) = self.data.dimensions();
            self.width = width;
            self.height = height;
            true
        } else {
            false
        }
    }

    /// Permanently applies a [TransformState] to the base image data.
    pub fn transform_image(&mut self, transform: &TransformState) {
        let new_data = transform::apply_transform(&self.data, transform);
        let (width, height) = new_data.dimensions();
        self.data = Arc::new(new_data);
        self.width = width;
        self.height = height;
        // After a transform, we should record the new state
        self.record_state();
    }

    /// Applies the current set of [Adjustments] to the raw image data.
    ///
    /// This method performs a sequential application of light, color, and effect
    /// transformations, returning a new [DynamicImage].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use hdim_core::HdimImage;
    /// use std::path::Path;
    ///
    /// let path = Path::new("tests/images/4k.jpg");
    /// let mut hdim_image = HdimImage::from_path(path).unwrap();
    /// hdim_image.adjustments.brightness = 10.0;
    /// let adjusted = hdim_image.apply_adjustments();
    /// ```
    pub fn apply_adjustments(&self) -> DynamicImage {
        let mut adjusted_image = (*self.data).clone();
        let adj = self.adjustments;

        // Order of application matters
        adjusted_image = self.apply_light_adjustments(adjusted_image, &adj);
        adjusted_image = self.apply_color_adjustments(adjusted_image, &adj);
        adjusted_image = self.apply_effect_adjustments(adjusted_image, &adj);

        adjusted_image
    }

    /// Internal helper to apply light-based transformations (exposure, brightness, contrast).
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

    /// Internal helper to apply color-based transformations (warmth, vibrance, saturation, hue).
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

    /// Internal helper to apply effects (fade, grain, noise).
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

    /// Saves the image with the current adjustments and handles EXIF metadata.
    ///
    /// If the `exif` feature is enabled, this method will attempt to preserve or strip
    /// EXIF data based on the `strip` argument.
    ///
    /// # Arguments
    ///
    /// * `path` - The destination path for the saved image.
    /// * `format` - The image format to use for saving.
    /// * `strip` - Whether to strip sensitive EXIF data.
    ///
    /// # Errors
    ///
    /// Returns an error if the image cannot be saved or if EXIF handling fails.
    pub fn save_with_exif(
        &self,
        path: &Path,
        format: image::ImageFormat,
        strip: bool,
    ) -> Result<()> {
        let adjusted_image = self.apply_adjustments();

        #[cfg(feature = "exif")]
        {
            use img_parts::ImageEXIF;
            if let Some(exif_bytes) = exif::get_exif_bytes_for_save(&self.path, strip)? {
                let mut buffer = std::io::Cursor::new(Vec::new());
                adjusted_image.write_to(&mut buffer, format)?;
                let mut bytes = buffer.into_inner();

                if format == image::ImageFormat::Jpeg {
                    let mut jpeg = img_parts::jpeg::Jpeg::from_bytes(bytes.into())?;
                    jpeg.set_exif(Some(exif_bytes.into()));
                    let mut out = Vec::new();
                    jpeg.encoder().write_to(&mut out)?;
                    bytes = out;
                } else if format == image::ImageFormat::Png {
                    let mut png = img_parts::png::Png::from_bytes(bytes.into())?;
                    png.set_exif(Some(exif_bytes.into()));
                    let mut out = Vec::new();
                    png.encoder().write_to(&mut out)?;
                    bytes = out;
                }

                std::fs::write(path, bytes)?;
                return Ok(());
            }
        }

        adjusted_image.save_with_format(path, format)?;
        Ok(())
    }
}

/// Simple width and height dimensions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    /// Width in pixels or units.
    pub width: u32,
    /// Height in pixels or units.
    pub height: u32,
}

/// Calculates the target dimensions for resizing an image to fit within a maximum size.
///
/// It accounts for the non-square aspect ratio of terminal character cells
/// (approximately 1:2) by doubling the target height.
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
