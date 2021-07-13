
use std::env;

use indexer::epoch_retriever::EpochRetriever;
use types::{Epoch, EthSpec, MainnetEthSpec};

use dotenv::dotenv;

#[tokio::test]
async fn get_consolidated_epoch() {
    dotenv().ok();
    
    let endpoint = env::var("ENDPOINT_URL").unwrap();
    let epoch_retriever = EpochRetriever::new(endpoint);

    let consolidated_epoch = epoch_retriever.get_consolidated_epoch::<MainnetEthSpec>(Epoch::new(45000)).await.unwrap();

    assert!(consolidated_epoch.epoch.as_u64() == 45000);
    assert!(consolidated_epoch.blocks.len() == MainnetEthSpec::slots_per_epoch() as usize);
    assert!(consolidated_epoch.validators.len() > 0);
}