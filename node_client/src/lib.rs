use reqwest::Result;
use reqwest::Response;
use models::Identifier;

pub mod models;
pub mod config;

pub struct NodeClient
{
    pub endpoint: String
}

impl NodeClient
{
    pub async fn get_state_root(&self, state_id: Identifier) -> Result<Response>
    {
        reqwest::get(format!("{}/eth/v1/beacon/states/{}/root", self.endpoint, state_id.to_string())).await
    }

    pub async fn get_block(&self, block_id: Identifier) -> Result<Response>
    {
        reqwest::get(format!("{}/eth/v1/beacon/blocks/{}", self.endpoint, block_id.to_string())).await
    }
}