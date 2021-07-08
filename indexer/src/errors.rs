use std::num::TryFromIntError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("Epoch cast error")]
    EpochCastingFailed { source: TryFromIntError },

    #[error("Slot cast error")]
    SlotCastingFailed { source: TryFromIntError },

    #[error(transparent)]
    QueryError(#[from] db::DieselError),
}