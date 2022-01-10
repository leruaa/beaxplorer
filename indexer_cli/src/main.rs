use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use dotenv::dotenv;
use simple_logger::SimpleLogger;
use tokio::sync::oneshot;

pub mod node_to_db;
pub mod node_to_files;

#[tokio::main]
async fn main() {
    dotenv().ok();
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .ok();

    let endpoint_url = env::var("LIGHTHOUSE_ENDPOINT_URL").unwrap();
    let start = Instant::now();

    let running = Arc::new(AtomicBool::new(true));

    let (sender, receiver) = oneshot::channel::<()>();
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    tokio::spawn(async move {
        node_to_files::process(endpoint_url, running).await;

        sender.send(()).unwrap();
    });

    receiver.await.unwrap();

    let duration = start.elapsed();
    log::info!("Avg epoch indexing duration: {:?}", duration.div_f32(10.));
}
