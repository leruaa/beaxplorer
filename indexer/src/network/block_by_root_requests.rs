use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;
use lighthouse_network::{rpc::BlocksByRootRequest, PeerId, Request};
use lighthouse_types::{EthSpec, Hash256, Slot};
use slog::{debug, Logger};
use tokio::sync::mpsc::UnboundedSender;

use super::{
    augmented_network_service::{NetworkMessage, RequestId},
    peer_db::PeerDb,
};

#[derive(Debug, Eq, PartialEq)]
enum BlockByRootRequestState {
    AwaitingPeer,
    Requesting(HashSet<PeerId>),
}

#[derive(Debug)]
pub enum BlockNotFoundResult {
    NotRequested,
    Searching,
    NotFound,
}

struct RequestAttempts {
    failed_count: u64,
    not_found_count: u64,
    current_state: BlockByRootRequestState,
}

impl RequestAttempts {
    pub fn awaiting_peer() -> Self {
        RequestAttempts {
            failed_count: 0,
            not_found_count: 0,
            current_state: BlockByRootRequestState::AwaitingPeer,
        }
    }

    pub fn requesting(peers: HashSet<PeerId>) -> Self {
        RequestAttempts {
            failed_count: 0,
            not_found_count: 0,
            current_state: BlockByRootRequestState::Requesting(peers),
        }
    }

    pub fn insert_peer(&mut self, peer_id: &PeerId) -> bool {
        match &mut self.current_state {
            BlockByRootRequestState::AwaitingPeer => {
                self.current_state = BlockByRootRequestState::Requesting(HashSet::from([*peer_id]));
                true
            }
            BlockByRootRequestState::Requesting(peers) => peers.insert(*peer_id),
        }
    }

    pub fn remove_peer(&mut self, peer_id: &PeerId) {
        if let BlockByRootRequestState::Requesting(peers) = &mut self.current_state {
            if peers.remove(peer_id) {
                self.failed_count += 1;
                if peers.is_empty() {
                    self.current_state = BlockByRootRequestState::AwaitingPeer;
                }
            }
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

    pub fn exists(&self, root: &Hash256) -> bool {
        self.requests.contains_key(root)
    }

    pub fn peer_connected(
        &mut self,
        peer_id: &PeerId,
        network_send: &UnboundedSender<NetworkMessage>,
    ) {
        self.requests.iter_mut().for_each(|(root, req)| {
            if req.insert_peer(peer_id) {
                network_send
                    .send(NetworkMessage::SendRequest {
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
        self.requests.remove(root).is_some()
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
        _slot: &Slot,
        root: &Hash256,
        network_send: &UnboundedSender<NetworkMessage>,
        peer_db: &PeerDb<E>,
    ) {
        self.requests.entry(*root).or_insert_with(|| {
            let (connected_great_peers, disconnected_great_peers) = peer_db.get_great_peers();

            for (peer_id, _) in &connected_great_peers {
                network_send
                    .send(NetworkMessage::SendRequest {
                        peer_id: *peer_id,
                        request_id: RequestId::Block(*root),
                        request: Box::new(Request::BlocksByRoot(BlocksByRootRequest {
                            block_roots: vec![*root].into(),
                        })),
                    })
                    .unwrap();
            }

            for (_, peer) in &disconnected_great_peers {
                peer.listening_addresses()
                    .iter()
                    .dedup()
                    .for_each(|a| network_send.send(NetworkMessage::Dial(a.clone())).unwrap());
            }

            if connected_great_peers.is_empty() {
                RequestAttempts::awaiting_peer()
            } else {
                RequestAttempts::requesting(HashSet::from_iter(
                    connected_great_peers
                        .into_iter()
                        .map(|(peer_id, _)| peer_id),
                ))
            }
        });
    }

    pub fn notify(&self, log: &Logger) {
        for (root, req) in &self.requests {
            debug!(log, "Block root {root} status"; "state" => ?req.current_state, "failed" => req.failed_count, "not found" => req.not_found_count);
        }
    }
}
