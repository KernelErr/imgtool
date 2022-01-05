use crate::define_operation;
use anyhow::{anyhow, Result};
use image::{imageops, DynamicImage};

define_operation!(
    #[doc = "Resize image."]
    resize(image),
    width: u32,
    height: u32,
    {
        let resized_image = image.resize(width, height, imageops::FilterType::Lanczos3);
        Ok(Some(resized_image))
    }
);

#[cfg(test)]
mod tests {
    use crate::process::resize::{execute, OperationArg};
    use image::GenericImageView;

    #[test]
    fn test_resize_image() {
        let mut image = image::open("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        // original size: 3153/4729
        image = execute(&image, OperationArg(300, image.height()))
            .unwrap()
            .unwrap();
        assert_eq!(image.width(), 300);
        assert_eq!(image.height(), 449);

        let mut image = image::open("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        image = execute(&image, OperationArg(image.width(), 450))
            .unwrap()
            .unwrap();
        assert_eq!(image.width(), 300);
        assert_eq!(image.height(), 450);
    }
}
