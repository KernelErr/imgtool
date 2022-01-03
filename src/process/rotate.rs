use anyhow::{anyhow, Result};
use image::DynamicImage;
use std::{fs, ops::Div};

pub fn execute(image: DynamicImage, angle: i32) -> Result<DynamicImage> {
    let mut angle = angle % 360;

    if angle % 90 != 0 {
        return Err(anyhow!("Only whole multiples of 90 are supported"));
    }

    let mut rotating_image = image;
    while angle > 0 {
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

    Ok(rotating_image)
}

#[cfg(test)]
mod tests {
    use crate::image::Image;

    #[test]
    fn test_rotate_image() {
        let mut image = Image::new("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        image = image.rotate(90).unwrap();
        image = image.rotate(180).unwrap();
        image = image.rotate(270).unwrap();
        assert!(image.rotate(1).is_err());
    }
}
