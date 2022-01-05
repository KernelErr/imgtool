pub mod ops;
pub mod utils;

use anyhow::{anyhow, Result};
use clap::{app_from_crate, arg, App, AppSettings, ArgMatches};

pub struct Cli {}

impl Cli {
    pub fn matches() -> ArgMatches {
        let matches = app_from_crate!()
            .global_setting(AppSettings::PropagateVersion)
            .global_setting(AppSettings::UseLongFormatForHelpSubcommand)
            .subcommand(App::new("contributers").about("List contributors"))
            .subcommand(
                App::new("blur")
                    .about("Gaussian blur")
                    .arg(arg!(-s <Sigma> "Sigma of the Gaussian blur"))
                    .arg(arg!(<Path> "Path to the image"))
                    .arg(arg!([Output] "Output path")),
            )
            .subcommand(
                App::new("convert")
                    .about("Convert image format")
                    .arg(arg!(-f <Format> "Format of the output image"))
                    .arg(arg!(Delete: -d "Delete original image after conversion"))
                    .arg(arg!(<Path> "Path to the image"))
                    .arg(arg!([Output] "Output path")),
            )
            .subcommand(
                App::new("crop")
                    .about("Crop image")
                    .arg(arg!(-x [X] "X coordinate of the top left corner"))
                    .arg(arg!(-y [Y] "Y coordinate of the top left corner"))
                    .arg(arg!(--width <Width> "Width of the output image"))
                    .arg(arg!(--height <Height> "Height of the output image"))
                    .arg(arg!(<Path> "Path to the image"))
                    .arg(arg!([Output] "Output path")),
            )
            .subcommand(
                App::new("flip")
                    .about("Flip image")
                    .arg(arg!(Horizontal: -h "Horizontal flip"))
                    .arg(arg!(Vertical: -v "Vertical flip"))
                    .arg(arg!(<Path> "Path to the image"))
                    .arg(arg!([Output] "Output path")),
            )
            .subcommand(
                App::new("resize")
                    .about("Resize image")
                    .arg(arg!(--width <Width> "Width of the output image"))
                    .arg(arg!(--height <Height> "Height of the output image"))
                    .arg(arg!(<Path> "Path to the image"))
                    .arg(arg!([Output] "Output path")),
            )
            .subcommand(
                App::new("rotate")
                    .about("Rotate image")
                    .arg(arg!(-r <Rotation> "Rotation angle in degrees"))
                    .arg(arg!(<Path> "Path to the image"))
                    .arg(arg!([Output] "Output path")),
            )
            .subcommand(
                App::new("thumb")
                    .about("Create thumbnail")
                    .arg(arg!(--width <Width> "Width of the thumbnail"))
                    .arg(arg!(--height <Height> "Height of the thumbnail"))
                    .arg(arg!(<Path> "Path to the image"))
                    .arg(arg!([Output] "Output path")),
            )
            .subcommand(
                App::new("tile")
                    .about("Tile image")
                    .arg(arg!(--width <Width> "Width of the output image"))
                    .arg(arg!(--height <Height> "Height of the output image"))
                    .arg(arg!(<Path> "Path to the image"))
                    .arg(arg!([Output] "Output path")),
            )
            .subcommand(
                App::new("unsharpen")
                    .about("Unsharp mask")
                    .arg(arg!(-s <Sigma> "Amount to blur the image"))
                    .arg(arg!(-r <Threshold> "How much to sharpen"))
                    .arg(arg!(<Path> "Path to the image"))
                    .arg(arg!([Output] "Output path")),
            )
            .get_matches();

        matches
    }
}

#[derive(Debug, Clone)]
pub struct Operations {
    pub inner: Vec<crate::cli::ops::SupportedOps>,
}

impl Operations {
    pub fn from_file_list(operations: Vec<String>) -> Result<Operations> {
        let mut ops: Vec<crate::cli::ops::SupportedOps> = Vec::new();

        let operation_str_list = operations.iter().map(|s| s.as_str()).collect::<Vec<_>>();
        let mut iter = operation_str_list.iter();
        while let Some(operation) = iter.next() {
            match *operation {
                "blur" => {
                    let op = crate::process::blur::to_operation(&mut iter)?;
                    ops.push(crate::cli::ops::SupportedOps::Blur(op));
                }
                "convert" => {
                    let op = crate::process::convert::to_operation(&mut iter)?;
                    ops.push(crate::cli::ops::SupportedOps::Convert(op));
                }
                "crop" => {
                    let op = crate::process::crop::to_operation(&mut iter)?;
                    ops.push(crate::cli::ops::SupportedOps::Crop(op));
                }
                "flip" => {
                    let op = crate::process::flip::to_operation(&mut iter)?;
                    ops.push(crate::cli::ops::SupportedOps::Flip(op));
                }
                "rotate" => {
                    let op = crate::process::rotate::to_operation(&mut iter)?;
                    ops.push(crate::cli::ops::SupportedOps::Rotate(op));
                }
                "tile" => {
                    let op = crate::process::tile::to_operation(&mut iter)?;
                    ops.push(crate::cli::ops::SupportedOps::Tile(op));
                }
                _ => {
                    return Err(anyhow!("Operation not supported: {}", operation));
                }
            }
        }

        Ok(Operations { inner: ops })
    }
}
