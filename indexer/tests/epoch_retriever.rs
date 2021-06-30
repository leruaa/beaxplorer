
use indexer::epoch_retriever::EpochRetriever;
use types::{Epoch, MainnetEthSpec};

use dotenv::dotenv;

#[tokio::test]
async fn get_consolidated_epoch() {
    dotenv().ok();

    let epoch_retriever = EpochRetriever::new();

    let consolidated_epoch = epoch_retriever.get_consolidated_epoch::<MainnetEthSpec>(Epoch::new(100)).await.unwrap();

    assert!(consolidated_epoch.epoch.as_u64() == 100);
}