use crate::define_operation;
use anyhow::{anyhow, Result};
use image::DynamicImage;
use std::str::FromStr;

define_operation!(
    #[doc = "Blur image."]
    blur(image),
    sigma: f32,
    {
        if sigma <= 0.0 {
            return Err(anyhow!("Only positive sigma is supported"));
        }
        Ok(Some(image.blur(sigma)))
    }
);

#[cfg(test)]
mod tests {
    use crate::process::blur::{execute, OperationArg};

    #[test]
    fn test_blur_image() {
        let image = image::open("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        execute(&image, OperationArg(1.0)).unwrap().unwrap();

        assert!(execute(&image, OperationArg(-1.0)).is_err());
    }
}
