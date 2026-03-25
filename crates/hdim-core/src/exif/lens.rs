#![cfg(feature = "exif")]
use super::util::{get_ascii_from_exif, get_rational_from_exif, get_rational_vec_from_exif};
use exif::{Exif, In, Tag};

/// Lens specification and identification metadata.
#[derive(Clone, Debug)]
pub struct LensExif {
    /// Manufacturer of the lens.
    pub make: Option<String>,
    /// Model name or number of the lens.
    pub model: Option<String>,
    /// Focal length of the lens at the time of capture.
    pub focal_length: Option<f64>,
    /// Range of supported aperture values (e.g., "f/2.8 - f/22").
    pub f_number_range: Option<String>,
}

/// Extracts lens-related metadata from EXIF tags.
pub fn get_lens_exif(exif: &Exif) -> Option<LensExif> {
    let lens_spec = get_rational_vec_from_exif(exif, Tag::LensSpecification, In::PRIMARY);
    Some(LensExif {
        make: get_ascii_from_exif(exif, Tag::LensMake, In::PRIMARY),
        model: get_ascii_from_exif(exif, Tag::LensModel, In::PRIMARY),
        focal_length: get_rational_from_exif(exif, Tag::FocalLength, In::PRIMARY),
        f_number_range: lens_spec.map(|values| {
            let minimum_f_stop = values
                .get(2)
                .map(|f_stop| format!("{:.1}", f_stop))
                .unwrap_or_default();
            let maximum_f_stop = values
                .get(3)
                .map(|f_stop| format!("{:.1}", f_stop))
                .unwrap_or_default();
            format!("f/{} - f/{}", minimum_f_stop, maximum_f_stop)
        }),
    })
}
