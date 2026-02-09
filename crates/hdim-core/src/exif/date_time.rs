#![cfg(feature = "exif")]
use super::util::get_ascii_from_exif;
use exif::{Exif, In, Tag};

#[derive(Clone, Debug)]
pub struct DateTimeExif {
    pub original: Option<String>,
    pub digitized: Option<String>,
    pub modified: Option<String>,
}

pub fn get_date_time_exif(exif: &Exif) -> Option<DateTimeExif> {
    Some(DateTimeExif {
        original: get_ascii_from_exif(exif, Tag::DateTimeOriginal, In::PRIMARY),
        digitized: get_ascii_from_exif(exif, Tag::DateTimeDigitized, In::PRIMARY),
        modified: get_ascii_from_exif(exif, Tag::DateTime, In::PRIMARY),
    })
}
