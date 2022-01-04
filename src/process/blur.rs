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
        let mut image = Image::new("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        image.blur(1.5).unwrap();
        assert!(image.blur(-1.0).is_err());
    }
}
