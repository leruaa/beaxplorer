use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(long)]
    pub reset: bool,

    #[clap(long, env)]
    pub endpoint_url: String,
}
