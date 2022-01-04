use image::DynamicImage;

pub fn execute(image: &DynamicImage, sigma: f32, threshold: i32) -> DynamicImage {
    image.unsharpen(sigma, threshold)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsharpen() {
        let image = image::open("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        let image = execute(&image, 3.0, 3);
    }
}
