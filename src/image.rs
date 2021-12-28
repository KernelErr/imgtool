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
    /// If delete if true, the origin image will be deleted.
    pub fn convert(self, path: &str, delete: bool) -> Result<()> {
        process::convert::execute(self.data.clone(), path, delete)
    }
}
