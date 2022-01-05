use anyhow::Result;
use image::{imageops, DynamicImage};

pub fn execute(image: &DynamicImage, width: u32, height: u32) -> Result<DynamicImage> {
    // Whether to provide the filter option?
    // image.resize will auto-fit the maximum size.
    let image = image.resize(width, height, imageops::FilterType::Nearest);

    Ok(image)
}

#[cfg(test)]
mod tests {

    use crate::image::Image;

    #[test]
    fn test_resize_image() {
        let mut image = Image::new("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        // original size: 3153/4729
        image.resize(300, image.height()).unwrap();
        assert_eq!(image.width(), 300);
        assert_eq!(image.height(), 449);

        let mut image = Image::new("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        image.resize(image.width(), 450).unwrap();
        assert_eq!(image.width(), 300);
        assert_eq!(image.height(), 450);
    }
}
