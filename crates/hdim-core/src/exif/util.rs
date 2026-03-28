use exif::{Exif, Field, In, Tag, Value};

pub fn get_ascii(field: Option<&Field>) -> Option<String> {
    match &field?.value {
        Value::Ascii(values) => values
            .first()
            .and_then(|bytes| String::from_utf8(bytes.clone()).ok()),
        _ => None,
    }
}

pub fn get_ascii_from_exif(exif: &Exif, tag: Tag, image_file_directory: In) -> Option<String> {
    let field = exif.get_field(tag, image_file_directory);
    get_ascii(field)
}

pub fn get_rational(field: Option<&Field>) -> Option<f64> {
    match &field?.value {
        Value::Rational(values) => values.first().map(|rational| rational.to_f64()),
        _ => None,
    }
}

pub fn get_rational_from_exif(exif: &Exif, tag: Tag, image_file_directory: In) -> Option<f64> {
    let field = exif.get_field(tag, image_file_directory);
    get_rational(field)
}

pub fn get_rational_vec(field: Option<&Field>) -> Option<Vec<f64>> {
    match &field?.value {
        Value::Rational(values) => Some(values.iter().map(|rational| rational.to_f64()).collect()),
        _ => None,
    }
}

pub fn get_rational_vec_from_exif(
    exif: &Exif,
    tag: Tag,
    image_file_directory: In,
) -> Option<Vec<f64>> {
    let field = exif.get_field(tag, image_file_directory);
    get_rational_vec(field)
}

pub fn get_u16_from_exif(exif: &Exif, tag: Tag, image_file_directory: In) -> Option<u16> {
    exif.get_field(tag, image_file_directory)
        .and_then(|field| field.value.get_uint(0).map(|value| value as u16))
}

pub fn get_u32_from_exif(exif: &Exif, tag: Tag, image_file_directory: In) -> Option<u32> {
    exif.get_field(tag, image_file_directory)
        .and_then(|field| field.value.get_uint(0))
}
