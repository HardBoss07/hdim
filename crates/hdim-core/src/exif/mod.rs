//! EXIF metadata parsing and management.
//!
//! This module provides tools to extract, structure, and display EXIF metadata
//! from image files using the `kamadak-exif` crate. It handles various metadata
//! categories such as camera settings, GPS coordinates, and lens information.

#![cfg(feature = "exif")]
pub mod camera;
pub mod date_time;
pub mod exif_data;
pub mod exposure;
pub mod gps;
pub mod image;
pub mod lens;
pub mod save;
pub mod util;

pub use camera::{CameraExif, get_camera_exif};
pub use date_time::{DateTimeExif, get_date_time_exif};
pub use exif_data::ExifData;
pub use exposure::{ExposureExif, get_exposure_exif};
pub use gps::{GpsExif, get_gps_exif};
pub use image::{ImageExif, get_image_exif};
pub use lens::{LensExif, get_lens_exif};
pub use save::get_exif_bytes_for_save;
pub use util::{get_ascii, get_rational, get_rational_vec};
