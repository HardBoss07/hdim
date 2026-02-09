#![cfg(feature = "exif")]
use super::util::get_ascii_from_exif;
use exif::{Exif, In, Tag};

#[derive(Clone, Debug)]
pub struct CameraExif {
    pub make: Option<String>,
    pub model: Option<String>,
    pub software: Option<String>,
}

pub fn get_camera_exif(exif: &Exif) -> Option<CameraExif> {
    Some(CameraExif {
        make: get_ascii_from_exif(exif, Tag::Make, In::PRIMARY),
        model: get_ascii_from_exif(exif, Tag::Model, In::PRIMARY),
        software: get_ascii_from_exif(exif, Tag::Software, In::PRIMARY),
    })
}
