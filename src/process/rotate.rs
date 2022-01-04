use crate::define_operation;
use anyhow::{anyhow, Result};
use image::DynamicImage;
use std::str::FromStr;

define_operation!(
    #[doc = "Rotate image. Only whole multiples of 90 degrees are supported."]
    rotate(image),
    angle: i32,
    {
        let mut angle = angle % 360;

        if angle % 90 != 0 {
            return Err(anyhow!("Only whole multiples of 90 are supported"));
        }

        let mut rotating_image = image.clone();
        while angle > 1 {
            if angle / 270 != 0 {
                rotating_image = rotating_image.rotate270();
                angle /= 270;
            } else if angle / 180 != 0 {
                rotating_image = rotating_image.rotate180();
                angle /= 180;
            } else {
                rotating_image = rotating_image.rotate90();
                angle /= 90;
            }
        }

        Ok(Some(rotating_image))
    }
);

#[cfg(test)]
mod tests {
    use crate::process::rotate::{execute, OperationArg};

    #[test]
    fn test_rotate_image() {
        let image = image::open("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        execute(&image, OperationArg(90)).unwrap();
        execute(&image, OperationArg(180)).unwrap();
        execute(&image, OperationArg(270)).unwrap();
        assert!(execute(&image, OperationArg(1)).is_err());
    }
}
