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

    let res = client.get_state_root(Identifier::Head).await.unwrap();
    let json = res.json::<ResponseData<Root>>().await.unwrap();

    assert!(json.data.root.starts_with("0x"))
}