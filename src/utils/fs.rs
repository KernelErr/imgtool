use anyhow::Result;
use image::{io::Reader as ImageReader, DynamicImage, ImageFormat};

/// Reads an image from a file.
pub fn read_image(path: &str) -> Result<DynamicImage> {
    let reader = ImageReader::open(path)?;
    let img = reader.decode()?;
    Ok(img)
}

/// Save an image to a file with specified format.
///
/// If format is none, the format will be inferred from the file extension.
pub fn save_image(image: DynamicImage, path: &str, format: Option<ImageFormat>) -> Result<()> {
    if let Some(format) = format {
        Ok(image.save_with_format(path, format)?)
    } else {
        Ok(image.save(path)?)
    }
}
