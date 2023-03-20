use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::Debug,
    iter::FromIterator,
};

use super::augmented_network_service::{NetworkCommand, RequestId};
use lighthouse_network::{rpc::BlocksByRootRequest, PeerId, Request};
use lighthouse_types::{EthSpec, Hash256, Slot};
use tokio::sync::mpsc::UnboundedSender;
use types::{
    block_request::BlockRequestModelWithId,
    utils::{BlockByRootRequestState, RequestAttempts},
};

#[derive(Debug)]
pub enum BlockNotFoundResult {
    NotRequested,
    Searching,
    NotFound,
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
        self.requests
            .iter()
            .filter(|(_, attempt)| attempt.state != BlockByRootRequestState::Found)
            .count()
    }

    pub fn exists(&self, root: &Hash256) -> bool {
        self.requests.contains_key(root)
    }

    pub fn peer_connected(
        &mut self,
        peer_id: &PeerId,
        network_send: &UnboundedSender<NetworkCommand>,
    ) {
        self.requests
            .iter_mut()
            .filter(|(_, req)| req.state != BlockByRootRequestState::Found)
            .for_each(|(root, req)| {
                if req.insert_peer(peer_id) {
                    send_block_by_root_request(network_send, *peer_id, *root)
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

    pub fn block_found(
        &mut self,
        root: &Hash256,
        found_by: PeerId,
    ) -> Entry<Hash256, RequestAttempts> {
        self.requests.entry(*root).and_modify(|attempts| {
            if attempts.found_by.is_none() {
                attempts.found_by = Some(found_by);
            }

            attempts.state = BlockByRootRequestState::Found;
        })
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
        connected_good_peers: &Vec<PeerId>,
    ) {
        let attempt = self.requests.entry(*root).or_insert_with(|| {
            if connected_good_peers.is_empty() {
                RequestAttempts::awaiting_peer()
            } else {
                for peer_id in connected_good_peers {
                    send_block_by_root_request(network_command_send, *peer_id, *root);
                }

                RequestAttempts::requesting(HashSet::from_iter(
                    connected_good_peers.iter().copied(),
                ))
            }
        });

        attempt.possible_slots.insert(*slot);
    }
}

impl From<&BlockByRootRequests> for Vec<BlockRequestModelWithId> {
    fn from(value: &BlockByRootRequests) -> Self {
        value.requests.iter().map(Into::into).collect::<Vec<_>>()
    }
}

fn send_block_by_root_request(
    network_send: &UnboundedSender<NetworkCommand>,
    peer_id: PeerId,
    root: Hash256,
) {
    network_send
        .send(NetworkCommand::SendRequest {
            peer_id,
            request_id: RequestId::Block(root),
            request: Box::new(Request::BlocksByRoot(BlocksByRootRequest {
                block_roots: vec![root].into(),
            })),
        })
        .unwrap();
}
