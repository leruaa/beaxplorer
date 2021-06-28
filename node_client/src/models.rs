use std::fmt;

pub enum Identifier
{
    Head,
    Genesis,
    Finalized,
    Justified,
    Slot(u64),
    Root(String),
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Identifier::Head => write!(f, "head"),
            Identifier::Genesis => write!(f, "genesis"),
            Identifier::Finalized => write!(f, "finalized"),
            Identifier::Justified => write!(f, "justified"),
            Identifier::Slot(slot) => write!(f, "{}", slot),
            Identifier::Root(root) => write!(f, "{}", root),
        }
    }
}
