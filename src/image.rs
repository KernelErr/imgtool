use crate::cli::ops::SupportedOps;
use crate::process;
use crate::utils::fs;
use anyhow::Result;
use image::DynamicImage;

/// A struct for single image and its processing method.
#[derive(Debug)]
pub struct Image {
    pub path: String,
    pub data: DynamicImage,
}

impl Image {
    /// Create a image object and read it from file
    pub fn new(path: &str) -> Result<Self> {
        Ok(Self {
            path: path.to_string(),
            data: fs::read_image(path)?,
        })
    }

    pub fn save(self) -> Result<()> {
        fs::save_image(self.data, &self.path, None)
    }

    pub fn process(&mut self, op: SupportedOps) -> Result<()> {
        let new_image: Option<DynamicImage> = match op {
            SupportedOps::Blur(blur) => process::blur::execute(&self.data, blur)?,
            SupportedOps::Convert(convert) => {
                process::convert::execute(&self.path, &self.data, convert)?
            }
            SupportedOps::Crop(crop) => process::crop::execute(&self.data, crop)?,
            SupportedOps::Flip(flip) => process::flip::execute(&self.data, flip)?,
            SupportedOps::Rotate(rotate) => process::rotate::execute(&self.data, rotate)?,
            SupportedOps::Tile(tile) => process::tile::execute(&self.data, tile)?,
        };

        if let Some(image) = new_image {
            self.data = image;
        }

        Ok(())
    }
    ///Tile image
    ///
    /// Tile the image to the specified size.
    pub fn tile(&mut self, width: u32, height: u32) -> Result<()> {
        let image = process::tile::execute(&self.data, width, height)?;
        self.data = image;
        Ok(())
    }
    /// Unsharp mask image
    ///
    /// The sigma and threshold parameters are used to determine how much to blur the image.
    /// The sigma parameter is the standard deviation of the Gaussian, in pixels.
    /// The threshold parameter is the threshold, in the range 0 to 255, for the difference between
    /// the original and the blur image.
    pub fn unsharpen(&mut self, sigma: f32, threshold: i32) -> Result<()> {
        self.data = process::unsharpen::execute(&self.data, sigma, threshold);
        Ok(())
    }
}
