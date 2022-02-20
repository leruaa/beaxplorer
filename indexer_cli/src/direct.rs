use crate::cli::Cli;
use indexer::direct_indexer::Indexer;

pub fn process(_cli: Cli) -> Result<(), String> {
    Indexer::start()
}
