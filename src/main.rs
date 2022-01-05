mod cli;
mod image;
mod process;
mod utils;

use clap::Parser;
use cli::Cli;
use log::{debug, error, info};
use std::cmp::min;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

// type TaskQueue = deadqueue::limited::Queue<String>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let args = Cli::matches();

    // let file_list = match utils::fs::get_file_lists(&args.path, args.depth) {
    //     Ok(f) => f,
    //     Err(e) => {
    //         error!("Failed to get image list: {}", e);
    //         return;
    //     }
    // };
    // info!("Found {} images", file_list.len());

    // let queue = Arc::new(TaskQueue::new(file_list.len()));
    // let file_len = file_list.len();

    // for file in file_list {
    //     queue.try_push(file.to_str().unwrap().to_string()).unwrap();
    // }

    // let arg_op = args.operations;
    // if arg_op.is_empty() {
    //     error!("No operations specified");
    //     return;
    // }

    // let operations = match cli::Operations::from_file_list(arg_op) {
    //     Ok(p) => p,
    //     Err(e) => {
    //         error!("{}", e);
    //         return;
    //     }
    // };
    // debug!("{:?}", operations);

    // let thread_nums = min(args.threads.unwrap_or_else(num_cpus::get), file_len);
    // info!("Starting {} threads to process", thread_nums);

    // for _ in 0..thread_nums {
    //     let queue = queue.clone();
    //     let ops = operations.clone();
    //     tokio::spawn(async move {
    //         loop {
    //             let file = queue.pop().await;
    //             let mut image = match image::Image::new(&file) {
    //                 Ok(img) => img,
    //                 Err(e) => {
    //                     error!("Failed to process {}: {}", file, e);
    //                     continue;
    //                 }
    //             };
    //             for op in &ops.inner {
    //                 if let Err(e) = image.process(op.clone()) {
    //                     error!("Failed to process {}: {}", file, e);
    //                     continue;
    //                 };
    //             }
    //             if !ops.inner.last().unwrap().is_convert() {
    //                 if let Err(e) = image.save() {
    //                     error!("Failed to save {}: {}", file, e);
    //                     continue;
    //                 }
    //             }
    //         }
    //     });
    // }

    // while queue.len() > 0 {
    //     sleep(Duration::from_secs(5)).await;
    //     info!("{} images left", queue.len());
    // }
}
