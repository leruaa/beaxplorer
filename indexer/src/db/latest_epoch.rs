use std::ops::{Deref, DerefMut};

use lighthouse_types::Epoch;

#[derive(Debug, Default)]
pub struct LatestEpoch(Option<Epoch>);

impl Deref for LatestEpoch {
    type Target = Option<Epoch>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LatestEpoch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
