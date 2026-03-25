#![cfg(feature = "exif")]
use super::util::get_ascii_from_exif;
use exif::{Exif, In, Tag};

/// Timestamp metadata from EXIF tags.
#[derive(Clone, Debug)]
pub struct DateTimeExif {
    /// The date and time when the original image data was generated.
    pub original: Option<String>,
    /// The date and time when the image was stored as digital data.
    pub digitized: Option<String>,
    /// The date and time the file was changed.
    pub modified: Option<String>,
}

/// Extracts date and time information from EXIF data.
pub fn get_date_time_exif(exif: &Exif) -> Option<DateTimeExif> {
    Some(DateTimeExif {
        original: get_ascii_from_exif(exif, Tag::DateTimeOriginal, In::PRIMARY),
        digitized: get_ascii_from_exif(exif, Tag::DateTimeDigitized, In::PRIMARY),
        modified: get_ascii_from_exif(exif, Tag::DateTime, In::PRIMARY),
    })
}
