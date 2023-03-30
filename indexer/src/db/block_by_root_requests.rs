use std::collections::{hash_map::Entry, HashMap};

use lighthouse_network::PeerId;
use lighthouse_types::{Hash256, Slot};
use types::{
    block_request::BlockRequestModelWithId,
    utils::{BlockByRootRequestState, RequestAttempts},
};

#[derive(Debug, Default)]
pub struct BlockByRootRequests(HashMap<Hash256, RequestAttempts>);

impl BlockByRootRequests {
    pub fn from_block_requests(block_requests: Vec<BlockRequestModelWithId>) -> Self {
        let mut block_by_root_requests = BlockByRootRequests::default();

        for block_request in block_requests {
            block_by_root_requests.0.insert(
                block_request.id.parse().unwrap(),
                block_request.model.into(),
            );
        }

        block_by_root_requests
    }

    pub fn exists(&self, root: &Hash256) -> bool {
        self.0.contains_key(root)
    }

    pub fn get(&self, root: &Hash256) -> Option<&RequestAttempts> {
        self.0.get(root)
    }

    pub fn add(&mut self, slot: Slot, root: Hash256) {
        let attempt = self
            .0
            .entry(root)
            .or_insert(RequestAttempts::awaiting_peer());

        attempt.possible_slots.insert(slot);
    }

    pub fn update_attempt<F>(&mut self, root: &Hash256, f: F)
    where
        F: Fn(&mut RequestAttempts),
    {
        if let Some(attempt) = self.0.get_mut(root) {
            f(attempt)
        }
    }

    pub fn pending_iter_mut(&mut self) -> impl Iterator<Item = (&Hash256, &mut RequestAttempts)> {
        self.0
            .iter_mut()
            .filter(|(_, req)| req.state != BlockByRootRequestState::Found)
    }

    pub fn set_request_as_found(&mut self, root: Hash256, found_by: PeerId) -> bool {
        match self.0.entry(root).and_modify(|attempts| {
            if attempts.found_by.is_none() {
                attempts.found_by = Some(found_by);
            }

            attempts.state = BlockByRootRequestState::Found;
        }) {
            Entry::Occupied(_) => true,
            Entry::Vacant(_) => false,
        }
    }
}

impl From<&BlockByRootRequests> for Vec<BlockRequestModelWithId> {
    fn from(value: &BlockByRootRequests) -> Self {
        value.0.iter().map(Into::into).collect::<Vec<_>>()
    }
}
