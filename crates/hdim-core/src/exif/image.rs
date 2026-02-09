#![cfg(feature = "exif")]
use super::util::{get_rational_from_exif, get_u16_from_exif, get_u32_from_exif};
use exif::{Exif, In, Tag};

#[derive(Clone, Debug)]
pub struct ImageExif {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub resolution_unit: Option<u16>,
    pub x_resolution: Option<f64>,
    pub y_resolution: Option<f64>,
}

pub fn get_image_exif(exif: &Exif) -> Option<ImageExif> {
    Some(ImageExif {
        width: get_u32_from_exif(exif, Tag::ImageWidth, In::PRIMARY),
        height: get_u32_from_exif(exif, Tag::ImageLength, In::PRIMARY),
        resolution_unit: get_u16_from_exif(exif, Tag::ResolutionUnit, In::PRIMARY),
        x_resolution: get_rational_from_exif(exif, Tag::XResolution, In::PRIMARY),
        y_resolution: get_rational_from_exif(exif, Tag::YResolution, In::PRIMARY),
    })
}
