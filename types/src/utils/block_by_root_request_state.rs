use std::{
    collections::HashSet,
    fmt::{Debug, Display},
};

use lighthouse_network::PeerId;

#[derive(Debug, Eq, PartialEq)]
pub enum BlockByRootRequestState {
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
