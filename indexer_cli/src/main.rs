#![recursion_limit = "256"]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use clap::StructOpt;
use dotenv::dotenv;
use env_logger::{Builder, Env};
use tokio::sync::oneshot;

use crate::cli::Cli;

mod cli;
mod direct;
// mod node_to_files;

fn main() {
    dotenv().ok();
    Builder::from_env(Env::default()).init();

    let cli = Cli::parse();

    direct::process(cli);
    /*
    let start = Instant::now();

    let running = Arc::new(AtomicBool::new(true));

    let (sender, receiver) = oneshot::channel::<()>();
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    tokio::spawn(async move {


        sender.send(()).unwrap();
    });

    receiver.await.unwrap();

    let duration = start.elapsed();
    log::info!("Avg epoch indexing duration: {:?}", duration.div_f32(10.));
    */
}
