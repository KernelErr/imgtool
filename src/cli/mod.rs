pub mod ops;

use anyhow::{anyhow, Result};
use clap::Parser;

macro_rules! extract_arg {
    ($arg:expr, $op:expr) => {
        match $arg {
            Some(s) => s,
            None => return Err(anyhow!("Missing argument for {} operation", $op)),
        }
    };
}

#[derive(Parser)]
#[clap(about, version, author)]
pub struct Cli {
    /// Maximum number of threads
    #[clap(short = 't')]
    pub thread_numbers: Option<u32>,

    /// Directory walk depth
    #[clap(short = 'd')]
    pub depth: Option<usize>,

    /// Image location
    pub path: String,

    /// Operations to be performed
    #[clap(last = true)]
    pub operations: Vec<String>,
}

#[derive(Debug)]
pub struct Operations {
    pub inner: Vec<crate::cli::ops::SupportedOps>,
}

impl Operations {
    pub fn from_file_list(operations: Vec<String>) -> Result<Operations> {
        let mut ops: Vec<crate::cli::ops::SupportedOps> = Vec::new();

        let mut iter = operations.iter();
        while let Some(operation) = iter.next() {
            match operation.as_str() {
                "blur" => {
                    let arg1 = extract_arg!(iter.next(), "blur");
                    ops.push(crate::cli::ops::SupportedOps::blur(arg1)?);
                }
                "convert" => {
                    let arg1 = extract_arg!(iter.next(), "convert");
                    let arg2 = extract_arg!(iter.next(), "convert");
                    ops.push(crate::cli::ops::SupportedOps::convert(arg1, arg2)?);
                }
                "crop" => {
                    let arg1 = extract_arg!(iter.next(), "crop");
                    let arg2 = extract_arg!(iter.next(), "crop");
                    let arg3 = extract_arg!(iter.next(), "crop");
                    let arg4 = extract_arg!(iter.next(), "crop");
                    ops.push(crate::cli::ops::SupportedOps::crop(arg1, arg2, arg3, arg4)?);
                }
                "flip" => {
                    let arg1 = extract_arg!(iter.next(), "flip");
                    ops.push(crate::cli::ops::SupportedOps::flip(arg1)?);
                }
                "rotate" => {
                    let arg1 = extract_arg!(iter.next(), "rotate");
                    ops.push(crate::cli::ops::SupportedOps::rotate(arg1)?);
                }
                _ => {
                    return Err(anyhow!("Operation not supported: {}", operation));
                }
            }
        }

        Ok(Operations { inner: ops })
    }
}
