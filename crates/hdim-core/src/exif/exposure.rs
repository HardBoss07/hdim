#![cfg(feature = "exif")]
use super::util::{get_rational_from_exif, get_u16_from_exif, get_u32_from_exif};
use exif::{Exif, In, Tag};

/// Exposure settings used to capture the image.
#[derive(Clone, Debug)]
pub struct ExposureExif {
    /// Exposure time in seconds (e.g., 0.01 for 1/100s).
    pub exposure_time: Option<f64>,
    /// The F number (aperture value).
    pub f_number: Option<f64>,
    /// ISO speed rating.
    pub iso: Option<u32>,
    /// Exposure bias (compensation) value in EV.
    pub exposure_bias: Option<f64>,
    /// Metering mode used (e.g., Pattern, Spot).
    pub metering_mode: Option<u16>,
    /// Flash status (fired or not).
    pub flash: Option<u16>,
    /// Focal length of the lens in millimeters.
    pub focal_length: Option<f64>,
    /// White balance setting (Auto or Manual).
    pub white_balance: Option<u16>,
}

/// Extracts exposure-related metadata from EXIF tags.
pub fn get_exposure_exif(exif: &Exif) -> Option<ExposureExif> {
    Some(ExposureExif {
        exposure_time: get_rational_from_exif(exif, Tag::ExposureTime, In::PRIMARY),
        f_number: get_rational_from_exif(exif, Tag::FNumber, In::PRIMARY),
        iso: get_u32_from_exif(exif, Tag::ISOSpeed, In::PRIMARY),
        exposure_bias: get_rational_from_exif(exif, Tag::ExposureBiasValue, In::PRIMARY),
        metering_mode: get_u16_from_exif(exif, Tag::MeteringMode, In::PRIMARY),
        flash: get_u16_from_exif(exif, Tag::Flash, In::PRIMARY),
        focal_length: get_rational_from_exif(exif, Tag::FocalLength, In::PRIMARY),
        white_balance: get_u16_from_exif(exif, Tag::WhiteBalance, In::PRIMARY),
    })
}
