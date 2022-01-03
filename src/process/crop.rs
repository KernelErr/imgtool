use anyhow::Result;
use image::DynamicImage;

pub fn execute(
    image: &DynamicImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<DynamicImage> {
    let copy = image.crop_imm(x, y, width, height);
    Ok(copy)
}

#[cfg(test)]
mod tests {
    use crate::image::Image;

    #[test]
    fn test_crop_image() {
        let mut image = Image::new("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        image.crop(0, 0, 10, 10).unwrap();
        assert_eq!(image.width(), 10);
        assert_eq!(image.height(), 10);
    }
}
