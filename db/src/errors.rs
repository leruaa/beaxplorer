use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    
    #[error(transparent)]
    ConnectionError(#[from] diesel::ConnectionError),
}