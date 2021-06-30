
use eth2::types::BlockId;
use eth2::types::FinalityCheckpointsData;
use eth2::types::GenericResponse;
use eth2::types::RootData;
use eth2::types::StateId;
use reqwest::Result;
use types::EthSpec;
use types::Hash256;
use types::SignedBeaconBlock;

pub mod config;

pub struct NodeClient {
    pub endpoint: String
}

impl NodeClient {

    pub fn new(endpoint: String) -> Self {
        NodeClient {
            endpoint
        }
    }

    pub async fn get_state_root(&self, state_id: StateId) -> Result<Hash256> {
        let res = reqwest::get(format!("{}/eth/v1/beacon/states/{}/root", self.endpoint, state_id.to_string())).await?;
        let json = res.json::<GenericResponse<RootData>>().await?;

        Ok(json.data.root)
    }

    pub async fn get_state_finality_checkpoints(&self, state_id: StateId) -> Result<FinalityCheckpointsData> {
        let res = reqwest::get(format!("{}/eth/v1/beacon/states/{}/finality_checkpoints", self.endpoint, state_id.to_string())).await?;
        let json = res.json::<GenericResponse<FinalityCheckpointsData>>().await?;

        Ok(json.data)
    }
 
    pub async fn get_block<E: EthSpec>(&self, block_id: BlockId) -> Result<SignedBeaconBlock<E>> {
        let res = reqwest::get(format!("{}/eth/v1/beacon/blocks/{}", self.endpoint, block_id.to_string())).await?;
        let json = res.json::<GenericResponse<SignedBeaconBlock<E>>>().await?;

        Ok(json.data)
    }
}