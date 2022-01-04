use crate::cli::ops::SupportedOps;
use crate::process;
use crate::utils::fs;
use anyhow::Result;
use image::DynamicImage;
use std::path::Path;
#[cfg(test)]
use image::GenericImageView;

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
        match op {
            SupportedOps::Blur(blur) => {
                self.blur(blur.sigma)?;
            }
            SupportedOps::Convert(convert) => {
                self.convert(&convert.format, convert.delete)?;
            }
            SupportedOps::Crop(crop) => {
                self.crop(crop.x, crop.y, crop.width, crop.height)?;
            }
            SupportedOps::Flip(flip) => {
                self.flip(&flip.orientation)?;
            }
            SupportedOps::Rotate(rotate) => {
                self.rotate(rotate.angle)?;
            }
        }

        Ok(())
    }

    #[cfg(test)]
    pub fn width(&self) -> u32 {
        self.data.width()
    }

    #[cfg(test)]
    pub fn height(&self) -> u32 {
        self.data.height()
    }

    /// Convert image's format
    ///
    /// If delete if true, the origin image will be deleted. This should be the last step of processing.
    pub fn convert(&self, format: &str, delete: bool) -> Result<()> {
        let path = Path::new(&self.path).with_extension(format);
        process::convert::execute(self.data.clone(), path.to_str().unwrap())?;
        if delete {
            std::fs::remove_file(&self.path)?;
        }
        Ok(())
    }

    /// Rotate image
    ///
    /// Only whole multiples of 90 degrees are supported.
    pub fn rotate(&mut self, angle: i32) -> Result<()> {
        let rotated_image = process::rotate::execute(self.data.clone(), angle)?;
        self.data = rotated_image;
        Ok(())
    }

    /// Flip image
    pub fn flip(&mut self, orientation: &str) -> Result<()> {
        let flipped_image = process::flip::execute(&self.data, orientation)?;
        self.data = flipped_image;
        Ok(())
    }

    /// Blur image
    pub fn blur(&mut self, sigma: f32) -> Result<()> {
        let image = process::blur::execute(&self.data, sigma)?;
        self.data = image;
        Ok(())
    }

    /// Crop image
    pub fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) -> Result<()> {
        let cropped_image = process::crop::execute(&self.data, x, y, width, height)?;
        self.data = cropped_image;
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
