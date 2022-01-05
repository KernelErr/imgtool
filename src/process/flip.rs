use crate::define_operation;
use anyhow::{anyhow, Result};
use image::DynamicImage;
use std::str::FromStr;

define_operation!(
    #[doc = "Flip image. Only both/horizontal/vertical orientation is supported."]
    flip(source),
    orientation: Orientation,
    {
        let flipped_image = match orientation {
            Orientation::Both => source.fliph().flipv(),
            Orientation::Horizontal => source.fliph(),
            Orientation::Vertical => source.flipv(),
        };
        Ok(Some(flipped_image))
    }
);

#[derive(Debug, Clone, PartialEq)]
pub enum Orientation {
    Both,
    Horizontal,
    Vertical,
}

impl FromStr for Orientation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "both" => Ok(Orientation::Both),
            "horizontal" => Ok(Orientation::Horizontal),
            "vertical" => Ok(Orientation::Vertical),
            _ => Err(anyhow!(
                "Only both/horizontal/vertical orientation is supported"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::to_operation;
    use crate::process::flip::{execute, OperationArg, Orientation};
    use image::GenericImageView;

    #[test]
    fn test_flip_operation() {
        let mut operation_str = vec!["horizontal"];
        let operation = to_operation(&mut operation_str.iter()).unwrap();
        assert_eq!(operation, OperationArg(Orientation::Horizontal));

        operation_str[0] = "invalid vertical";
        let err_operation = to_operation(&mut operation_str.iter());
        assert!(err_operation.is_err());
    }

    #[test]
    fn test_flip_image() {
        let image = image::open("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        let origin = image.get_pixel(0, 0);
        let (width, height) = image.dimensions();

        let image = execute(&image, OperationArg(Orientation::Horizontal))
            .unwrap()
            .unwrap();
        assert_eq!(origin, image.get_pixel(width - 1, 0));

        let image = execute(&image, OperationArg(Orientation::Vertical))
            .unwrap()
            .unwrap();
        assert_eq!(origin, image.get_pixel(width - 1, height - 1));

        let image = execute(&image, OperationArg(Orientation::Both))
            .unwrap()
            .unwrap();
        assert_eq!(origin, image.get_pixel(0, 0));
    }
}
