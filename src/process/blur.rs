use anyhow::{anyhow, Result};
use image::DynamicImage;

pub fn execute(image: &DynamicImage, sigma: f32) -> Result<DynamicImage> {
    if sigma <= 0.0 {
        return Err(anyhow!("Only positive sigma is supported"));
    }
    Ok(image.blur(sigma))
}

#[cfg(test)]
mod tests {
    use crate::image::Image;

    #[test]
    fn test_blur_image() {
        let image = Image::new("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        let image = image.blur(5.0).unwrap();
        assert!(image.blur(-1.0).is_err());
    }
}
