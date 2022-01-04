use anyhow::Result;
use image::DynamicImage;
use std::str::FromStr;

pub fn execute(image: &DynamicImage, orientation: &str) -> Result<DynamicImage> {
    let orientation = Orientation::from_str(orientation)
        .expect("Only horizontal/vertical orientation is supported");
    match orientation {
        Orientation::Horizontal => Ok(image.fliph()),
        Orientation::Vertical => Ok(image.flipv()),
    }
}

pub enum Orientation {
    Horizontal,
    Vertical,
}

impl FromStr for Orientation {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "horizontal" => Ok(Orientation::Horizontal),
            "vertical" => Ok(Orientation::Vertical),
            _ => Err(()),
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
