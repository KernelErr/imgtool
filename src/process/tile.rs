use anyhow::Result;
use image::{DynamicImage, RgbaImage};

pub fn execute(image: DynamicImage) -> Result<DynamicImage> {
    let mut canvas = RgbaImage::new(1920, 1080);
    image::imageops::tile(&mut canvas, &image);
    Ok(image)
}
