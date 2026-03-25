#![cfg(feature = "exif")]
use super::util::{get_rational_from_exif, get_u16_from_exif, get_u32_from_exif};
use exif::{Exif, In, Tag};

/// Basic image dimensions and resolution metadata.
#[derive(Clone, Debug)]
pub struct ImageExif {
    /// Width of the image in pixels.
    pub width: Option<u32>,
    /// Height of the image in pixels.
    pub height: Option<u32>,
    /// Unit of resolution (e.g., inches or centimeters).
    pub resolution_unit: Option<u16>,
    /// Horizontal resolution (pixels per unit).
    pub x_resolution: Option<f64>,
    /// Vertical resolution (pixels per unit).
    pub y_resolution: Option<f64>,
}

/// Extracts basic image dimension metadata from EXIF tags.
pub fn get_image_exif(exif: &Exif) -> Option<ImageExif> {
    Some(ImageExif {
        width: get_u32_from_exif(exif, Tag::ImageWidth, In::PRIMARY),
        height: get_u32_from_exif(exif, Tag::ImageLength, In::PRIMARY),
        resolution_unit: get_u16_from_exif(exif, Tag::ResolutionUnit, In::PRIMARY),
        x_resolution: get_rational_from_exif(exif, Tag::XResolution, In::PRIMARY),
        y_resolution: get_rational_from_exif(exif, Tag::YResolution, In::PRIMARY),
    })
}
