pub mod ops;

use anyhow::{anyhow, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Cli {
    /// Maximum number of threads
    #[clap(short = 't')]
    pub threads: Option<usize>,

    /// Directory walk depth
    #[clap(short = 'd')]
    pub depth: Option<usize>,

    /// Image location
    pub path: String,

    /// Operations to be performed
    #[clap(last = true)]
    pub operations: Vec<String>,
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
