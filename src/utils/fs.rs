use anyhow::{anyhow, Result};
use image::{io::Reader as ImageReader, DynamicImage, ImageFormat};
use std::env::current_dir;
use std::path::Path;
use std::path::PathBuf;
use wax::Glob;

/// Reads an image from a file.
pub fn read_image<P>(path: P) -> Result<DynamicImage>
where
    P: AsRef<Path>,
{
    let reader = ImageReader::open(path)?;
    let img = reader.decode()?;
    Ok(img)
}

/// Save an image to a file with specified format.
///
/// If format is none, the format will be inferred from the file extension.
pub fn save_image(image: DynamicImage, path: &str, format: Option<ImageFormat>) -> Result<()> {
    if let Some(format) = format {
        Ok(image.save_with_format(path, format)?)
    } else {
        Ok(image.save(path)?)
    }
}

pub fn get_file_lists(path: &str, depth: Option<usize>) -> Result<Vec<PathBuf>> {
    let depth = depth.unwrap_or(usize::MAX);

    let mut files = Vec::new();
    let glob = match Glob::new(path) {
        Ok(g) => g,
        Err(e) => {
            return Err(anyhow!("{}", e));
        }
    };

    for entry in glob.walk(current_dir()?, depth).flatten() {
        files.push(entry.path().to_owned());
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_lists() {
        let files = get_file_lists("**/*.jpg", None).unwrap();
        assert_eq!(files.len(), 1);
    }
}
