use image::DynamicImage;

pub fn execute(image: DynamicImage, sigma: f32, threshold: i32) -> DynamicImage {
    image.unsharpen(sigma, threshold)
}
