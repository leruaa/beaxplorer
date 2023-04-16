#![recursion_limit = "256"]

use clap::StructOpt;
use cli::Commands;
use dotenv::dotenv;
use indexer::launcher;
use span_duration::SpanDurationLayer;
use tracing_subscriber::{
    filter::{filter_fn, LevelFilter},
    fmt::format::FmtSpan,
    prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
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
        Commands::BuildDatabase { reset } => launcher::start_indexer(reset, cli.base_dir).unwrap(),
        Commands::UpdateIndexes => launcher::update_indexes(cli.base_dir).unwrap(),
    }
}
