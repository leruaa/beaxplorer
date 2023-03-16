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
        Commands::BuildDatabase { reset } => launcher::start_indexer(reset, cli.base_dir).unwrap(),
        Commands::UpdateIndexes => launcher::update_indexes(cli.base_dir).unwrap(),
        Commands::SearchOrphans => launcher::search_orphans(cli.base_dir).unwrap(),
    }
}
