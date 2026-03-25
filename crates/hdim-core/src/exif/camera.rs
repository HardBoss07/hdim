#![cfg(feature = "exif")]
use super::util::get_ascii_from_exif;
use exif::{Exif, In, Tag};

/// Camera identification metadata.
#[derive(Clone, Debug)]
pub struct CameraExif {
    /// Manufacturer of the camera (e.g., "Canon", "Sony").
    pub make: Option<String>,
    /// Model name or number (e.g., "EOS 5D Mark IV").
    pub model: Option<String>,
    /// Software or firmware version used to create the image.
    pub software: Option<String>,
}

/// Extracts camera-related EXIF data.
pub fn get_camera_exif(exif: &Exif) -> Option<CameraExif> {
    Some(CameraExif {
        make: get_ascii_from_exif(exif, Tag::Make, In::PRIMARY),
        model: get_ascii_from_exif(exif, Tag::Model, In::PRIMARY),
        software: get_ascii_from_exif(exif, Tag::Software, In::PRIMARY),
    })
}
