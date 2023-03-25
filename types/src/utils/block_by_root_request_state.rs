use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    str::FromStr,
};

use lighthouse_network::PeerId;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum BlockByRootRequestState {
    #[default]
    AwaitingPeer,
    Requesting(HashSet<PeerId>),
    Found,
}

impl BlockByRootRequestState {
    pub fn active_request_count(&self) -> usize {
        match &self {
            BlockByRootRequestState::Requesting(peers) => peers.len(),
            _ => 0,
        }
    }
}

impl Display for BlockByRootRequestState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl FromStr for BlockByRootRequestState {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AwaitingPeer" => Ok(BlockByRootRequestState::AwaitingPeer),
            "Requesting" => Ok(BlockByRootRequestState::Requesting(HashSet::new())),
            "Found" => Ok(BlockByRootRequestState::Found),
            s => Err(format!("Failed to parse '{s}'")),
        }
    }
}
