use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(long, default_value = "../web/public/data")]
    pub base_dir: String,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    BuildDatabase {
        #[clap(long)]
        reset: bool,
    },

    UpdateIndexes,

    SearchOrphans,
}
