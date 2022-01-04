use crate::define_operation;
use anyhow::{anyhow, Result};
use image::DynamicImage;
use std::{fs, path::Path, str::FromStr};

define_operation!(
    #[doc = "Convert image's format. If delete if true, the origin image will be deleted. This should be the last step of processing."]
    convert(image, src_path),
    format: String,
    delete: bool,
    {
        let dst_path_buf = Path::new(src_path).with_extension(format);
        let dst_path = dst_path_buf.to_str().unwrap();

        if dst_path.ends_with("webp") {
            let encoder = webp::Encoder::from_image(&image).map_err(|err| anyhow!("{}", err))?;
            let webp = encoder.encode_lossless();
            fs::write(dst_path, webp.iter())?;
        } else {
            image.save(dst_path)?;
        }
        if delete {
            std::fs::remove_file(src_path)?;
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
        let original_path = "tests/images/ryan-yao-VURwPtZqyF4-unsplash.jpg";
        let image = image::open(original_path).unwrap();
        let path = Path::new(original_path).with_extension("webp");
        execute(
            original_path,
            &image,
            OperationArg("webp".to_string(), false),
        )
        .unwrap();
        assert!(path.exists());
        std::fs::remove_file(path).unwrap();
    }
}
