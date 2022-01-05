use anyhow::{anyhow, Result};
use image::DynamicImage;
use std::fs;

pub fn execute(image: DynamicImage, path: &str) -> Result<()> {
    if path.ends_with("webp") {
        let encoder = match webp::Encoder::from_image(&image) {
            Ok(encoder) => encoder,
            Err(err) => {
                return Err(anyhow!("{}", err));
            }
        };
        let webp = encoder.encode_lossless();
        fs::write(path, webp.iter())?;
    } else {
        image.save(path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::image::Image;
    use std::path::Path;

    #[test]
    fn test_convert_jpg_webp() {
        let image = Image::new("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        let path =
            Path::new("tests/images/ryan-yao-VURwPtZqyF4-unsplash.webp").with_extension("webp");
        image.convert("webp", false).unwrap();
        assert!(path.exists());
        std::fs::remove_file(path).unwrap();
    }
}
