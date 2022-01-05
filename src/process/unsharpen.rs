use crate::define_operation;
use anyhow::{anyhow, Result};
use image::DynamicImage;

define_operation!(
    #[doc = "Tile an image into a new image."]
    unsharpen(image),
    sigma: f32,
    threshold: i32,
    { Ok(Some(image.unsharpen(sigma, threshold))) }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsharpen() {
        let img = image::open("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        let img_clone = img.clone();
        let unsharpened_img = execute(&img, OperationArg(1.5, 1)).unwrap().unwrap();

        assert_eq!(img, img_clone);
        assert_ne!(img, unsharpened_img);
    }
}
