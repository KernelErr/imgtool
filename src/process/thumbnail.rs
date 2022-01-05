use anyhow::Result;
use image::DynamicImage;

pub fn execute(image: &DynamicImage) -> Result<DynamicImage> {
    let thumbnail = image.thumbnail(100, 100);
    Ok(thumbnail)
}

#[cfg(test)]
mod tests {
    use crate::image::Image;

    #[test]
    fn test_thumbnail() {
        let mut image = Image::new("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        image.thumbnail().unwrap();
        assert_eq!(image.height(), 100);
    }
}
