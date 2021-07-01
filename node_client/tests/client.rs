use eth2::types::{BlockId, StateId};
use node_client::*;

use dotenv::dotenv;
use types::{MainnetEthSpec};
use std::env;

#[tokio::test]
async fn get_state_root() {
    dotenv().ok();

    let client = get_client();

    let root = client.get_state_root(StateId::Head).await.unwrap();

    assert!(root.to_string().starts_with("0x"))
}

#[tokio::test]
async fn get_validators_from_state() {
    dotenv().ok();

    let client = get_client();

    let validators = client.get_validators_from_state(StateId::Head).await.unwrap();

    assert!(validators.len() > 0)
}

#[tokio::test]
async fn get_state_finality_checkpoints() {
    dotenv().ok();

    let client = get_client();

    let checkpoints = client.get_state_finality_checkpoints(StateId::Head).await;

    assert!(checkpoints.is_ok())
}

#[tokio::test]
async fn get_block() {
    dotenv().ok();

    let client = get_client();

    let block = client.get_block::<MainnetEthSpec>(BlockId::Head).await;

    assert!(block.is_ok())
}

fn get_client() -> NodeClient {
    NodeClient {
        endpoint: env::var("ENDPOINT_URL").unwrap()
    }
}