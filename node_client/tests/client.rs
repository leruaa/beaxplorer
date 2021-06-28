use node_client::*;
use node_client::models::*;

use dotenv::dotenv;
use std::env;

#[tokio::test]
async fn get_state_root() {
    dotenv().ok();

    let client = NodeClient {
        endpoint: env::var("ENDPOINT_URL").unwrap()
    };

    let root = client.get_state_root(Identifier::Head).await.unwrap();

    assert!(root.starts_with("0x"))
}