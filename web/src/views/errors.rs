use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error(transparent)]
    IntegerCastingError(#[from] std::num::TryFromIntError),

    #[error("Slot not found")]
    SlotNotFound,
}
