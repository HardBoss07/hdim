#![cfg(feature = "exif")]
use super::util::{get_rational_from_exif, get_rational_vec_from_exif};
use exif::{Exif, In, Tag};

#[derive(Clone, Debug)]
pub struct GpsExif {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub timestamp: Option<(u8, u8, u8)>,
}

pub fn get_gps_exif(exif: &Exif) -> Option<GpsExif> {
    Some(GpsExif {
        latitude: get_rational_from_exif(exif, Tag::GPSLatitude, In::PRIMARY),
        longitude: get_rational_from_exif(exif, Tag::GPSLongitude, In::PRIMARY),
        altitude: get_rational_from_exif(exif, Tag::GPSAltitude, In::PRIMARY),
        timestamp: get_rational_vec_from_exif(exif, Tag::GPSTimeStamp, In::PRIMARY).map(|values| {
            (
                values.get(0).map_or(0, |&second| second as u8),
                values.get(1).map_or(0, |&second| second as u8),
                values.get(2).map_or(0, |&second| second as u8),
            )
        }),
    })
}
