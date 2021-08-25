use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error(transparent)]
    QueryError(#[from] db::DieselError),

    #[error("Node error")]
    NodeError { inner_error: eth2::Error },

    #[error("Element not found")]
    ElementNotFound(String),

    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
}
