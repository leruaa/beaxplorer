use lighthouse_types::{Hash256, Slot};
use parking_lot::{RwLock, RwLockReadGuard};

use self::latest_slot::LatestSlot;

pub mod blocks_by_epoch;
pub mod latest_slot;

#[derive(Default)]
pub struct Stores {
    latest_slot: RwLock<LatestSlot>,
}

impl Stores {
    pub fn latest_slot(&self) -> RwLockReadGuard<LatestSlot> {
        self.latest_slot.read()
    }

    pub fn update(&self, slot: Slot, root: Hash256) {
        self.latest_slot.write().replace(slot);
        //self.proposed_block_roots.write().insert(root);
    }
}
