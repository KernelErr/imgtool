use anyhow::Result;
use image::{DynamicImage, GenericImageView, RgbaImage};

pub fn execute(image: DynamicImage, width: u32, height: u32) -> Result<DynamicImage> {
    let mut canvas = RgbaImage::new(width, height);
    image::imageops::tile(&mut canvas, &image);
    Ok(DynamicImage::ImageRgba8(canvas))
}

#[test]
fn test_tile() {
    let img = image::open("./tests/images/test_tile.jpg").unwrap();

    let img = execute(img, 1920, 1080).unwrap();
    assert_eq!(img.dimensions(), (1920, 1080));
}
