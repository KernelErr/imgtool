use crate::define_operation;
use crate::image::Image;
use anyhow::{anyhow, Result};
use image::DynamicImage;
use std::str::FromStr;

define_operation!(flip, source, orientation: Orientation, {
    let flipped_image = match orientation {
        Orientation::Horizontal => source.data.fliph(),
        Orientation::Vertical => source.data.flipv(),
    };
    Ok(Some(flipped_image))
});

#[derive(Debug, Clone)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl FromStr for Orientation {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "horizontal" => Ok(Orientation::Horizontal),
            "vertical" => Ok(Orientation::Vertical),
            _ => Err("Only horizontal/vertical orientation is supported"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::image::Image;
    use image::GenericImageView;

    #[test]
    fn test_flip_image() {
        let mut image = Image::new("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        let origin = image.data.get_pixel(0, 0);
        let (width, height) = image.data.dimensions();

        image.flip("horizontal").unwrap();
        assert_eq!(origin, image.data.get_pixel(width - 1, 0));

        image.flip("vertical").unwrap();
        assert_eq!(origin, image.data.get_pixel(width - 1, height - 1));
    }
}
