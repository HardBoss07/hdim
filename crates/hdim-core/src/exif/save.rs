use anyhow::Result;
use img_parts::ImageEXIF;
use std::path::Path;

/// Extracts EXIF bytes from an image file for use in saving.
///
/// If `strip` is true, sensitive data is removed.
/// Currently, stripping removes the entire EXIF segment, relying on the image
/// encoder to preserve essential dimensions in the format-specific headers.
pub fn get_exif_bytes_for_save(path: &Path, strip: bool) -> Result<Option<Vec<u8>>> {
    if strip {
        // When stripping, we return None to indicate no EXIF segment should be injected.
        // The image crate's encoder will still write the necessary width/height
        // in the standard JPEG/PNG headers.
        return Ok(None);
    }

    let bytes = std::fs::read(path)?;

    // Try to extract EXIF from JPEG
    if let Ok(jpeg) = img_parts::jpeg::Jpeg::from_bytes(bytes.clone().into())
        && let Some(exif) = jpeg.exif()
    {
        let exif_vec: Vec<u8> = exif.to_vec();
        return Ok(Some(exif_vec));
    }

    // Try to extract EXIF from PNG
    if let Ok(png) = img_parts::png::Png::from_bytes(bytes.into())
        && let Some(exif) = png.exif()
    {
        let exif_vec: Vec<u8> = exif.to_vec();
        return Ok(Some(exif_vec));
    }

    Ok(None)
}
