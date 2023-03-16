use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::{Debug, Display},
    iter::FromIterator,
};

use super::{
    augmented_network_service::{NetworkCommand, RequestId},
    peer_db::PeerDb,
};
use lighthouse_network::{rpc::BlocksByRootRequest, PeerId, Request};
use lighthouse_types::{EthSpec, Hash256, Slot};
use tokio::sync::mpsc::UnboundedSender;
use types::block_request::{
    BlockRequestModel, BlockRequestModelWithId, PersistIteratorBlockRequestModel,
};

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

#[derive(Debug)]
pub enum BlockNotFoundResult {
    NotRequested,
    Searching,
    NotFound,
}

pub struct RequestAttempts {
    pub possible_slots: HashSet<Slot>,
    pub state: BlockByRootRequestState,
    pub failed_count: usize,
    pub not_found_count: usize,
    pub found_by: Option<Hash256>,
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

pub struct BlockByRootRequests {
    requests: HashMap<Hash256, RequestAttempts>,
}

impl BlockByRootRequests {
    pub fn new() -> Self {
        BlockByRootRequests {
            requests: HashMap::new(),
        }
    }

    pub fn from_block_requests(block_requests: Vec<BlockRequestModelWithId>) -> Self {
        let mut block_by_root_requests = BlockByRootRequests::new();

        for block_request in block_requests {
            block_by_root_requests.requests.insert(
                block_request.id.parse().unwrap(),
                block_request.model.into(),
            );
        }

        block_by_root_requests
    }

    pub fn count(&self) -> usize {
        self.requests.len()
    }

    pub fn exists(&self, root: &Hash256) -> bool {
        self.requests.contains_key(root)
    }

    pub fn peer_connected(
        &mut self,
        peer_id: &PeerId,
        network_send: &UnboundedSender<NetworkCommand>,
    ) {
        self.requests.iter_mut().for_each(|(root, req)| {
            if req.insert_peer(peer_id) {
                network_send
                    .send(NetworkCommand::SendRequest {
                        peer_id: *peer_id,
                        request_id: RequestId::Block(*root),
                        request: Box::new(Request::BlocksByRoot(BlocksByRootRequest {
                            block_roots: vec![*root].into(),
                        })),
                    })
                    .unwrap();
            }
        });
    }

    pub fn peer_disconnected(&mut self, peer_id: &PeerId) {
        self.requests.iter_mut().for_each(|(_, req)| {
            req.remove_peer(peer_id);
        });
    }

    pub fn failed_request(&mut self, root: &Hash256, peer_id: &PeerId) {
        if let Some(req) = self.requests.get_mut(root) {
            req.remove_peer(peer_id);
        };
    }

    pub fn block_found(&mut self, root: &Hash256) -> bool {
        if let Entry::Occupied(mut e) = self.requests.entry(*root) {
            if e.get().found_by.is_none() {
                e.get_mut().found_by = Some(*root);
            }

            e.get_mut().state = BlockByRootRequestState::Found;
            true
        } else {
            false
        }
    }

    pub fn block_not_found(&mut self, root: &Hash256) -> BlockNotFoundResult {
        if let Entry::Occupied(mut e) = self.requests.entry(*root) {
            e.get_mut().not_found_count += 1;

            if e.get().not_found_count > 9 {
                e.remove_entry();
                BlockNotFoundResult::NotFound
            } else {
                BlockNotFoundResult::Searching
            }
        } else {
            BlockNotFoundResult::NotRequested
        }
    }

    pub fn request_block_by_root<E: EthSpec>(
        &mut self,
        slot: &Slot,
        root: &Hash256,
        network_command_send: &UnboundedSender<NetworkCommand>,
        peer_db: &PeerDb<E>,
    ) {
        let attempt = self.requests.entry(*root).or_insert_with(|| {
            let (connected_great_peers, disconnected_great_peers) = peer_db.get_trusted_peers();

            for (peer_id, _) in &connected_great_peers {
                network_command_send
                    .send(NetworkCommand::SendRequest {
                        peer_id: *peer_id,
                        request_id: RequestId::Block(*root),
                        request: Box::new(Request::BlocksByRoot(BlocksByRootRequest {
                            block_roots: vec![*root].into(),
                        })),
                    })
                    .unwrap();
            }

            /*
            for (peer_id, _) in &disconnected_great_peers {
                network_send
                    .send(NetworkMessage::DialPeer(*peer_id))
                    .unwrap()
            }
             */

            if connected_great_peers.is_empty() {
                /*
                for a in peer_db.get_great_peers_known_addresses() {
                    network_send.send(NetworkMessage::Dial(a)).unwrap();
                }
                 */

                RequestAttempts::awaiting_peer()
            } else {
                RequestAttempts::requesting(HashSet::from_iter(
                    connected_great_peers
                        .into_iter()
                        .map(|(peer_id, _)| peer_id),
                ))
            }
        });

        attempt.possible_slots.insert(*slot);
    }

    pub fn persist(&self, base_dir: &str) {
        self.requests
            .iter()
            .map(|(root, attempts)| BlockRequestModelWithId {
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
                    found_by: attempts
                        .found_by
                        .map(|r| format!("{r:#?}"))
                        .unwrap_or_default(),
                },
            })
            .persist(base_dir);
    }
}
