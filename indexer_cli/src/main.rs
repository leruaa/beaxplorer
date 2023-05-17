#![recursion_limit = "256"]

use clap::StructOpt;
use cli::Commands;
use dotenv::dotenv;
use indexer::launcher;
use span_duration::SpanDurationLayer;
use tracing_subscriber::{
    filter::LevelFilter, util::SubscriberInitExt,
    EnvFilter, Layer, prelude::__tracing_subscriber_SubscriberExt,
};

use crate::cli::Cli;

mod cli;
mod span_duration;

fn main() {
    dotenv().ok();

    let stdout_log = tracing_subscriber::fmt::layer()
        .compact()
        .with_target(false)
        .with_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        );

    tracing_subscriber::registry()
        .with(stdout_log)
        .with(SpanDurationLayer)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::BuildDatabase { reset, dry } => {
            launcher::start_indexer(reset, dry, cli.base_dir, cli.execution_node_url).unwrap()
        }
        Commands::UpdateIndexes => launcher::update_indexes(cli.base_dir).unwrap(),
    }
}
