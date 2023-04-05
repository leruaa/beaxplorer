use std::ops::{Deref, DerefMut};

use lighthouse_types::Slot;

#[derive(Debug, Default)]
pub struct LatestSlot(Option<Slot>);

impl LatestSlot {
    pub fn new(slot: Option<Slot>) -> Self {
        Self(slot)
    }
}

impl Deref for LatestSlot {
    type Target = Option<Slot>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LatestSlot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PartialEq<u64> for LatestSlot {
    fn eq(&self, other: &u64) -> bool {
        if let Some(slot) = self.0 {
            slot.as_u64() == *other
        } else {
            false
        }
    }
}
