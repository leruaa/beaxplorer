
use eth2::types::GenericResponse;
use eth2::types::RootData;
use reqwest::Result;
use models::Identifier;
use types::Hash256;
use types::MainnetEthSpec;
use types::SignedBeaconBlock;

pub mod models;

pub struct NodeClient
{
    pub endpoint: String
}

impl NodeClient
{
    pub async fn get_state_root(&self, state_id: Identifier) -> Result<Hash256>
    {
        let res = reqwest::get(format!("{}/eth/v1/beacon/states/{}/root", self.endpoint, state_id.to_string())).await?;
        let json = res.json::<GenericResponse<RootData>>().await?;

        Ok(json.data.root)
    }

    pub async fn get_block(&self, block_id: Identifier) -> Result<SignedBeaconBlock<MainnetEthSpec>>
    {
        let res = reqwest::get(format!("{}/eth/v1/beacon/blocks/{}", self.endpoint, block_id.to_string())).await?;
        let json = res.json::<GenericResponse<SignedBeaconBlock<MainnetEthSpec>>>().await?;

        Ok(json.data)
    }
}