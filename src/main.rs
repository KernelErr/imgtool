mod cli;
mod image;
mod process;
mod utils;

use clap::Parser;
use cli::Cli;
use log::{debug, error, info};

fn main() {
    pretty_env_logger::init();
    let args = Cli::parse();

    let file_list = match utils::fs::get_file_lists(&args.path, args.depth) {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to get image list: {}", e);
            return;
        }
    };
    info!("Found {} images", file_list.len());

    let arg_op = args.operations;
    if arg_op.is_empty() {
        error!("No operations specified");
        return;
    }

    let operations = match cli::Operations::from_file_list(arg_op) {
        Ok(p) => p,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };
    debug!("{:?}", operations);
}
