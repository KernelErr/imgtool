use anyhow::Result;
use image::{DynamicImage, RgbaImage};

// Tile an image into a new image.
pub fn execute(image: &DynamicImage, width: u32, height: u32) -> Result<DynamicImage> {
    let mut canvas = RgbaImage::new(width, height);
    image::imageops::tile(&mut canvas, image);
    Ok(DynamicImage::ImageRgba8(canvas))
}

#[cfg(test)]
mod tests {
    use self::super::*;
    use image::{GenericImageView, GrayImage};

    #[test]
    fn test_rgb_tile() {
        let img = image::open("./tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        let img = execute(&img, 1920, 1080).unwrap();
        assert_eq!(img.dimensions(), (1920, 1080));
    }
    #[test]
    fn test_gray_tile() {
        let img = DynamicImage::ImageLuma8(GrayImage::new(100, 100));
        let img = execute(&img, 1920, 1080).unwrap();
        assert_eq!(img.dimensions(), (1920, 1080));
    }
}
