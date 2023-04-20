pub mod attestation;
pub mod block;
pub mod block_request;
pub mod block_root;
pub mod committee;
pub mod deposit;
pub mod epoch;
pub mod good_peer;
pub mod meta;
pub mod model;
pub mod path;
pub mod persistable;
pub mod utils;
pub mod validator;
pub mod vote;

pub use serde::de::DeserializeOwned;
pub use serde::Serialize;
