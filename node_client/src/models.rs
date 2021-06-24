use std::fmt;
use serde::Deserialize;

pub enum Stateidentifier
{
    Head,
    Genesis,
    Finalized,
    Justified,
    Slot(String),
}

impl fmt::Display for Stateidentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stateidentifier::Head => write!(f, "head"),
            Stateidentifier::Genesis => write!(f, "genesis"),
            Stateidentifier::Finalized => write!(f, "finalized"),
            Stateidentifier::Justified => write!(f, "justified"),
            Stateidentifier::Slot(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Deserialize)]
pub struct ResponseData<T>
{
    pub data: T
}

#[derive(Deserialize)]
pub struct Root {
    pub root: String
}