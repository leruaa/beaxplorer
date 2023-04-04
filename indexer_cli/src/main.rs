#![recursion_limit = "256"]

use clap::StructOpt;
use cli::Commands;
use dotenv::dotenv;
use indexer::launcher;
use tracing_subscriber::EnvFilter;

use crate::cli::Cli;

mod cli;
// mod node_to_files;

fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .compact()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::BuildDatabase { reset } => launcher::start_indexer(reset, cli.base_dir).unwrap(),
        Commands::UpdateIndexes => launcher::update_indexes(cli.base_dir).unwrap(),
    }
}
