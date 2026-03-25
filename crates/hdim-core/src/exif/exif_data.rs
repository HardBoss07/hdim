#![cfg(feature = "exif")]
use super::{
    CameraExif, DateTimeExif, ExposureExif, GpsExif, ImageExif, LensExif, get_camera_exif,
    get_date_time_exif, get_exposure_exif, get_gps_exif, get_image_exif, get_lens_exif,
};
use crate::exif::util::get_u16_from_exif;
use exif::{In, Reader, Tag};
use std::io::{Read, Seek};

/// Container for all parsed EXIF metadata categories.
///
/// This struct aggregates various metadata sections (camera, lens, GPS, etc.)
/// into a single, optional-field structure.
#[derive(Clone, Debug)]
pub struct ExifData {
    /// Image orientation (e.g., TopLeft).
    pub orientation: Option<u16>,
    /// Date and time information.
    pub datetime: Option<DateTimeExif>,
    /// Camera make and model details.
    pub camera: Option<CameraExif>,
    /// Exposure settings (ISO, shutter speed, aperture).
    pub exposure: Option<ExposureExif>,
    /// Lens specification details.
    pub lens: Option<LensExif>,
    /// GPS coordinates and altitude.
    pub gps: Option<GpsExif>,
    /// Image dimensions and resolution.
    pub image: Option<ImageExif>,
}

impl ExifData {
    /// Extracts EXIF data from a readable and seekable source (e.g., a file).
    ///
    /// # Errors
    ///
    /// Returns an error if the EXIF data cannot be found or parsed from the container.
    pub fn get_exif_data<R: Read + Seek>(reader: R) -> anyhow::Result<Self> {
        let exif = Reader::new().read_from_container(&mut std::io::BufReader::new(reader))?;

        Ok(Self {
            orientation: get_u16_from_exif(&exif, Tag::Orientation, In::PRIMARY),
            datetime: get_date_time_exif(&exif),
            camera: get_camera_exif(&exif),
            exposure: get_exposure_exif(&exif),
            lens: get_lens_exif(&exif),
            gps: get_gps_exif(&exif),
            image: get_image_exif(&exif),
        })
    }
}
