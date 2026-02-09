#![cfg(feature = "exif")]
use super::util::{get_ascii_from_exif, get_rational_from_exif, get_rational_vec_from_exif};
use exif::{Exif, In, Tag};

#[derive(Clone, Debug)]
pub struct LensExif {
    pub make: Option<String>,
    pub model: Option<String>,
    pub focal_length: Option<f64>,
    pub f_number_range: Option<String>,
}

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
