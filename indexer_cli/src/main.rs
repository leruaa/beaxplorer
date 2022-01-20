use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use clap::StructOpt;
use simple_logger::SimpleLogger;
use tokio::sync::oneshot;

use crate::cli::Cli;

//pub mod node_to_db;
mod cli;
pub mod node_to_files;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .ok();

    let cli = Cli::parse();

    let start = Instant::now();

    let running = Arc::new(AtomicBool::new(true));

    let (sender, receiver) = oneshot::channel::<()>();
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    tokio::spawn(async move {
        node_to_files::process(cli, running).await;

        sender.send(()).unwrap();
    });

    receiver.await.unwrap();

    let duration = start.elapsed();
    log::info!("Avg epoch indexing duration: {:?}", duration.div_f32(10.));
}
