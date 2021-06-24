use reqwest::Result;
use reqwest::Response;

pub mod models;

pub struct NodeClient
{
    pub endpoint: String
}

impl NodeClient
{
    pub async fn get_state_root(&self, state_id: models::Stateidentifier) -> Result<Response>
    {
        reqwest::get(format!("{}/eth/v1/beacon/states/{}/root", self.endpoint, state_id.to_string())).await
    }
}