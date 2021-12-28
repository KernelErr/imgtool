use anyhow::{anyhow, Result};
use image::DynamicImage;
use std::fs;

pub fn execute(image: DynamicImage, path: &str, delete: bool) -> Result<()> {
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
    if delete {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::image::Image;
    use tempfile::tempdir;

    #[test]
    fn test_convert_jpg_webp() {
        let image = Image::new("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("ryan-yao-VURwPtZqyF4-unsplash.webp");
        image.convert(&file_path.to_str().unwrap(), false).unwrap();
        assert!(file_path.exists());
    }
}
