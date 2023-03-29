use std::collections::{hash_map::Entry, HashMap};

use lighthouse_network::PeerId;
use lighthouse_types::Hash256;
use types::utils::{BlockByRootRequestState, RequestAttempts};

#[derive(Debug, Default)]
pub struct BlockByRootRequests(HashMap<Hash256, RequestAttempts>);

impl BlockByRootRequests {
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
