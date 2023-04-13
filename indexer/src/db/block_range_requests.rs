use std::{
    collections::{BTreeMap, HashSet},
    sync::Arc,
};

use lighthouse_network::PeerId;
use lighthouse_types::{EthSpec, SignedBeaconBlock, Slot};

#[derive(Debug, Default)]
pub struct BlockRangeRequests<E: EthSpec> {
    active_requests: HashSet<PeerId>,
    blocks_queue: BTreeMap<Slot, Arc<SignedBeaconBlock<E>>>,
}

impl<E: EthSpec> BlockRangeRequests<E> {
    pub fn next_or(&mut self, block: Arc<SignedBeaconBlock<E>>) -> Arc<SignedBeaconBlock<E>> {
        self.blocks_queue.insert(block.slot(), block);

        self.blocks_queue
            .pop_first()
            .map(|(_, b)| b)
            .expect("should never happen")
    }

    pub fn request_terminated(&mut self, peer_id: &PeerId) -> bool {
        self.active_requests.remove(peer_id)
    }

    pub fn is_requesting(&self) -> bool {
        !self.active_requests.is_empty()
    }
}
