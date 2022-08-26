use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Discover,

    Index {
        #[clap(long)]
        reset: bool,
        #[clap(long, default_value = "../web/public/data")]
        base_dir: String,
    },
}
