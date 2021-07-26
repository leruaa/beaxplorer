use thiserror::Error;
use types::Slot;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("Epoch cast error")]
    EpochCastingFailed { source: std::num::TryFromIntError },

    #[error("Slot cast error")]
    SlotCastingFailed { source: std::num::TryFromIntError },

    #[error(transparent)]
    QueryError(#[from] db::DieselError),

    #[error("Node error")]
    NodeError { inner_error: eth2::Error },

    #[error("Element not found")]
    ElementNotFound(Slot),

    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
}
