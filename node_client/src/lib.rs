use reqwest::Result;
use reqwest::Response;
use serde::Deserialize;

pub struct NodeClient
{
    pub endpoint: String
}

impl NodeClient
{
    pub async fn get_state_root(&self, state_id: &str) -> Result<Response>
    {
        reqwest::get(format!("{}/eth/v1/beacon/states/{}/root", self.endpoint, state_id)).await
    }
}

#[derive(Deserialize)]
pub struct ResponseData<T>
{
    pub data: T
}

#[derive(Deserialize)]
pub struct Root {
    pub root: String
}