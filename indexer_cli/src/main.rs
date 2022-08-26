#![recursion_limit = "256"]

use clap::StructOpt;
use cli::Commands;
use dotenv::dotenv;
use env_logger::{Builder, Env};
use indexer::launcher;

use crate::cli::Cli;

mod cli;
// mod node_to_files;

fn main() {
    dotenv().ok();
    Builder::from_env(Env::default()).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Discover => launcher::start_discovery().unwrap(),
        Commands::Index { reset, base_dir } => launcher::start_indexer(reset, base_dir).unwrap(),
    }
}
