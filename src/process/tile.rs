use crate::define_operation;
use anyhow::{anyhow, Result};
use image::{DynamicImage, RgbaImage};

define_operation!(
    #[doc = "Tile an image into a new image."]
    tile(image),
    width: u32,
    height: u32,
    {
        let mut canvas = RgbaImage::new(width, height);
        image::imageops::tile(&mut canvas, image);
        Ok(Some(DynamicImage::ImageRgba8(canvas)))
    }
);

#[cfg(test)]
mod tests {
    use self::super::*;
    use image::{GenericImageView, GrayImage};

    #[test]
    fn test_rgb_tile() {
        let img = image::open("./tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        let img = execute(&img, OperationArg(1920, 1080)).unwrap().unwrap();
        assert_eq!(img.dimensions(), (1920, 1080));
    }

    #[test]
    fn test_gray_tile() {
        let img = DynamicImage::ImageLuma8(GrayImage::new(100, 100));
        let img = execute(&img, OperationArg(1920, 1080)).unwrap().unwrap();
        assert_eq!(img.dimensions(), (1920, 1080));
    }
}
