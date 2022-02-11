use crate::define_operation;
use anyhow::{anyhow, Result};
use image::DynamicImage;
use std::{fs, path::Path};

define_operation!(
    #[doc = "Convert image's format. If delete if true, the origin image will be deleted. This should be the last step of processing."]
    convert(image, src_path),
    format: String,
    delete: bool,
    {
        let dst_path_buf = Path::new(src_path).with_extension(format.clone());

        match format.as_str() {
            "webp" => {
                let encoder = webp::Encoder::from_image(image).map_err(|err| anyhow!("{}", err))?;
                let webp = encoder.encode_lossless();
                fs::write(dst_path_buf, webp.iter())?;
            }
            _ => image.save(dst_path_buf)?,
        };

        if delete {
            fs::remove_file(src_path)?;
        }
        Ok(None)
    }
);

#[cfg(test)]
mod tests {
    use crate::process::convert::{execute, OperationArg};
    use std::path::Path;

    #[test]
    fn test_convert_jpg_webp() {
        let image = image::open("tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg").unwrap();
        let path =
            Path::new("tests/images/ryan-yao-VURwPtZqyF4-unsplash.webp").with_extension("webp");
        execute(
            path.to_str().unwrap(),
            &image,
            OperationArg("webp".to_string(), false),
        )
        .unwrap();
        assert!(path.exists());
        std::fs::remove_file(path).unwrap();
    }
}
