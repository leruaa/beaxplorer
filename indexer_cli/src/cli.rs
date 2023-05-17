use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(long, default_value = "../web/public/data")]
    pub base_dir: String,

    #[clap(long)]
    pub execution_node_url: String,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    BuildDatabase {
        #[clap(long)]
        reset: bool,

        #[clap(long, conflicts_with("reset"))]
        dry: bool,
    },

    UpdateIndexes,
}
