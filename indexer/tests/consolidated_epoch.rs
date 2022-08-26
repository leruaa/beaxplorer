use std::env;

use eth2::types::BlockId;
use indexer::{beacon_node_client::BeaconNodeClient, types::consolidated_epoch::ConsolidatedEpoch};
use lighthouse_types::{Epoch, EthSpec, MainnetEthSpec};

use dotenv::dotenv;

#[tokio::test]
async fn get_committees() {
    dotenv().ok();

    let endpoint = env::var("ENDPOINT_URL").unwrap();
    let client = BeaconNodeClient::new(endpoint);

    let committees = client.get_committees(Epoch::new(100)).await.unwrap();

    println!("0: {:?}", committees[0]);
    println!("1: {:?}", committees[1]);
}
