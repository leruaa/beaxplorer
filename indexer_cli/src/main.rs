#![recursion_limit = "256"]

use clap::StructOpt;
use dotenv::dotenv;
use env_logger::{Builder, Env};

use crate::cli::Cli;

mod cli;
mod direct;
// mod node_to_files;

fn main() {
    dotenv().ok();
    Builder::from_env(Env::default()).init();

    let cli = Cli::parse();

    let _res = direct::process(cli);
}
