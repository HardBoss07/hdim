#![cfg(feature = "exif")]
use super::util::{get_rational_from_exif, get_rational_vec_from_exif};
use exif::{Exif, In, Tag};

/// Global Positioning System (GPS) coordinates and altitude.
#[derive(Clone, Debug)]
pub struct GpsExif {
    /// Latitude in degrees.
    pub latitude: Option<f64>,
    /// Longitude in degrees.
    pub longitude: Option<f64>,
    /// Altitude in meters.
    pub altitude: Option<f64>,
    /// UTC timestamp of the GPS fix (Hours, Minutes, Seconds).
    pub timestamp: Option<(u8, u8, u8)>,
}

/// Extracts GPS metadata from EXIF tags.
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
