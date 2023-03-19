use std::collections::HashSet;

use lighthouse_network::PeerId;
use lighthouse_types::{Hash256, Slot};

use super::BlockByRootRequestState;
use crate::block_request::{BlockRequestModel, BlockRequestModelWithId};

pub struct RequestAttempts {
    pub possible_slots: HashSet<Slot>,
    pub state: BlockByRootRequestState,
    pub failed_count: usize,
    pub not_found_count: usize,
    pub found_by: Option<PeerId>,
}

impl RequestAttempts {
    pub fn awaiting_peer() -> Self {
        RequestAttempts {
            possible_slots: HashSet::new(),
            state: BlockByRootRequestState::AwaitingPeer,
            failed_count: 0,
            not_found_count: 0,
            found_by: None,
        }
    }

    pub fn requesting(peers: HashSet<PeerId>) -> Self {
        RequestAttempts {
            possible_slots: HashSet::new(),
            failed_count: 0,
            not_found_count: 0,
            state: BlockByRootRequestState::Requesting(peers),
            found_by: None,
        }
    }

    pub fn insert_peer(&mut self, peer_id: &PeerId) -> bool {
        match &mut self.state {
            BlockByRootRequestState::AwaitingPeer => {
                self.state = BlockByRootRequestState::Requesting(HashSet::from([*peer_id]));
                true
            }
            BlockByRootRequestState::Requesting(peers) => peers.insert(*peer_id),
            _ => false,
        }
    }

    pub fn remove_peer(&mut self, peer_id: &PeerId) {
        if let BlockByRootRequestState::Requesting(peers) = &mut self.state {
            if peers.remove(peer_id) {
                self.failed_count += 1;
                if peers.is_empty() {
                    self.state = BlockByRootRequestState::AwaitingPeer;
                }
            }
        }
    }
}

impl From<BlockRequestModel> for RequestAttempts {
    fn from(value: BlockRequestModel) -> Self {
        Self {
            possible_slots: HashSet::new(),
            state: BlockByRootRequestState::AwaitingPeer,
            failed_count: value.failed_count,
            not_found_count: value.not_found_count,
            found_by: value.found_by.parse().ok(),
        }
    }
}

impl From<(&Hash256, &RequestAttempts)> for BlockRequestModelWithId {
    fn from((root, attempts): (&Hash256, &RequestAttempts)) -> Self {
        BlockRequestModelWithId {
            id: format!("{root:#?}"),
            model: BlockRequestModel {
                possible_slots: attempts
                    .possible_slots
                    .iter()
                    .map(|s| s.as_u64())
                    .collect::<Vec<_>>(),
                state: attempts.state.to_string(),
                active_request_count: attempts.state.active_request_count(),
                failed_count: attempts.failed_count,
                not_found_count: attempts.not_found_count,
                found_by: attempts.found_by.map(|p| p.to_string()).unwrap_or_default(),
            },
        }
    }
}
