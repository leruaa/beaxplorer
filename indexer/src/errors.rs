use std::num::TryFromIntError;

use thiserror::Error;
use types::Slot;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("Epoch cast error")]
    EpochCastingFailed { source: TryFromIntError },

    #[error("Slot cast error")]
    SlotCastingFailed { source: TryFromIntError },

    #[error(transparent)]
    QueryError(#[from] db::DieselError),

    #[error("Node error")]
    NodeError { inner_error: eth2::Error },

    #[error("Element not found")]
    ElementNotFound(Slot),
}