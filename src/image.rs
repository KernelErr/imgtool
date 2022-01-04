use crate::process;
use crate::utils::fs;
use anyhow::Result;
use image::DynamicImage;

/// A struct for single image and its processing method.
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

    /// Convert image's format
    ///
    /// If delete if true, the origin image will be deleted. This should be the last step of processing.
    pub fn convert(self, path: &str, delete: bool) -> Result<()> {
        process::convert::execute(self.data.clone(), path, delete)
    }

    /// Rotate image
    ///
    /// Only whole multiples of 90 degrees are supported.
    pub fn rotate(mut self, angle: i32) -> Result<Self> {
        let rotated_image = process::rotate::execute(self.data.clone(), angle)?;
        self.data = rotated_image;
        Ok(self)
    }
    ///Tile image
    ///
    /// Tile the image to the specified size.
    pub fn tile(&mut self, width: u32, height: u32) -> Result<()> {
        let image = process::tile::execute(&self.data, width, height)?;
        self.data = image;
        Ok(())
    }
}
