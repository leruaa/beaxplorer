use std::env;

use eth2::types::BlockId;
use indexer::{beacon_node_client::BeaconNodeClient, types::consolidated_epoch::ConsolidatedEpoch};
use lighthouse_types::{Epoch, EthSpec, MainnetEthSpec};

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
    assert!(consolidated_epoch.validator_balances.is_empty());
}

#[tokio::test]
async fn get_committees() {
    dotenv().ok();

    let endpoint = env::var("ENDPOINT_URL").unwrap();
    let client = BeaconNodeClient::new(endpoint);

    let committees = client.get_committees(Epoch::new(100)).await.unwrap();

    println!("0: {:?}", committees[0]);
    println!("1: {:?}", committees[1]);
}

#[tokio::test]
async fn votes() {
    dotenv().ok();

    let endpoint = env::var("ENDPOINT_URL").unwrap();
    let client = BeaconNodeClient::new(endpoint);

    let consolidated_epoch =
        ConsolidatedEpoch::<MainnetEthSpec>::new(Epoch::new(50000), client.clone())
            .await
            .unwrap();

    /*
    let mut attestations = consolidated_epoch
        .blocks
        .iter()
        .filter_map(|x| x.block.clone())
        .flat_map(|x| x.body().attestations().to_vec())
        .map(|x| x.data.beacon_block_root)
        .collect::<Vec<_>>();
    */
    let block = consolidated_epoch.blocks[0].block.as_ref().unwrap();

    println!("orig block: {:?}", block.slot());

    let attestations = block.clone().body().attestations().to_vec();

    let mut roots = attestations
        .iter()
        .map(|x| x.data.beacon_block_root)
        .collect::<Vec<_>>();

    roots.sort();
    roots.dedup();

    for root in &roots {
        let b = client
            .clone()
            .get_block::<MainnetEthSpec>(BlockId::Root(*root))
            .await
            .unwrap()
            .unwrap();

        println!("block: {:?}", b.data.message().block_header().slot);
    }

    println!("attestations: {:?}", attestations.len());
    println!("roots: {:?}", roots.len());
}
