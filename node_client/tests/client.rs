use eth2::types::StateId;
use node_client::*;

use dotenv::dotenv;
use std::env;

#[tokio::test]
async fn get_state_root() {
    dotenv().ok();

    let client = NodeClient {
        endpoint: env::var("ENDPOINT_URL").unwrap()
    };

    let root = client.get_state_root(StateId::Head).await.unwrap();

    assert!(root.to_string().starts_with("0x"))
}