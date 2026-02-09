#![cfg(feature = "exif")]
use super::util::{get_rational_from_exif, get_u16_from_exif, get_u32_from_exif};
use exif::{Exif, In, Tag};

#[derive(Clone, Debug)]
pub struct ExposureExif {
    pub exposure_time: Option<f64>,
    pub f_number: Option<f64>,
    pub iso: Option<u32>,
    pub exposure_bias: Option<f64>,
    pub metering_mode: Option<u16>,
    pub flash: Option<u16>,
    pub focal_length: Option<f64>,
    pub white_balance: Option<u16>,
}

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
