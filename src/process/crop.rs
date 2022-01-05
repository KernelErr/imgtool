use crate::define_operation;
use anyhow::{anyhow, Result};
use image::DynamicImage;

define_operation!(
    #[doc = "Crop image."]
    crop(image),
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    {
        let copy = image.crop_imm(x, y, width, height);
        Ok(Some(copy))
    }
);

#[cfg(test)]
mod tests {
    use crate::process::crop::{execute, OperationArg};
    use image::GenericImageView;

    #[test]
    fn test_crop_image() {
        let image = image::open("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        let image = execute(&image, OperationArg(0, 0, 10, 10))
            .unwrap()
            .unwrap();
        assert_eq!(image.dimensions(), (10, 10));
    }
}
