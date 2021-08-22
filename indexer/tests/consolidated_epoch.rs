use std::env;

use indexer::{beacon_node_client::BeaconNodeClient, types::consolidated_epoch::ConsolidatedEpoch};
use types::{Epoch, EthSpec, MainnetEthSpec};

use dotenv::dotenv;

#[tokio::test]
async fn get_consolidated_epoch() {
    dotenv().ok();

    let endpoint = env::var("ENDPOINT_URL").unwrap();
    let client = BeaconNodeClient::new(endpoint);

    let consolidated_epoch = ConsolidatedEpoch::<MainnetEthSpec>::new(Epoch::new(45000), client)
        .await
        .unwrap();

    assert!(consolidated_epoch.epoch.as_u64() == 45000);
    assert!(consolidated_epoch.blocks.len() == MainnetEthSpec::slots_per_epoch() as usize);
    assert!(consolidated_epoch.validator_balances.len() > 0);
}
