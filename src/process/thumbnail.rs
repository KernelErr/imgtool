use crate::define_operation;
use anyhow::{anyhow, Result};
use image::DynamicImage;

define_operation!(
    #[doc = "Generate thumbnail."]
    thumbnail(image),
    width: u32,
    height: u32,
    {
        let thumbnail = image.thumbnail(100, 100);
        Ok(Some(thumbnail))
    }
);

#[cfg(test)]
mod tests {
    use crate::process::resize::{execute, OperationArg};
    use image::GenericImageView;

    #[test]
    fn test_generate_thumbnail() {
        let mut image = image::open("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        image = execute(&image, OperationArg(100, 100)).unwrap().unwrap();
        assert_eq!(image.height(), 100);
    }
}
