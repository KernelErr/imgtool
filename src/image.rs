use crate::process;
use crate::utils::fs;
use anyhow::Result;
use image::{DynamicImage, GenericImageView};
use std::path::PathBuf;

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

    pub fn from_pathbuf(path: PathBuf) -> Result<Self> {
        Ok(Self {
            path: path.to_str().unwrap().to_string(),
            data: fs::read_image(path)?,
        })
    }

    pub fn width(&self) -> u32 {
        self.data.width()
    }

    pub fn height(&self) -> u32 {
        self.data.height()
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

    /// Flip image
    pub fn flip(mut self, orientation: &str) -> Result<Self> {
        let flipped_image = process::flip::execute(&self.data, orientation)?;
        self.data = flipped_image;
        Ok(self)
    }

    /// Blur image
    pub fn blur(mut self, sigma: f32) -> Result<Self> {
        let image = process::blur::execute(&self.data, sigma)?;
        self.data = image;
        Ok(self)
    }

    /// Crop image
    pub fn crop(mut self, x: u32, y: u32, width: u32, height: u32) -> Result<Self> {
        let cropped_image = process::crop::execute(&self.data, x, y, width, height)?;
        self.data = cropped_image;
        Ok(self)
    }
}
