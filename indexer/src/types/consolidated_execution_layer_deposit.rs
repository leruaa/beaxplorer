use lighthouse_types::{FixedVector, DepositData, Hash256, typenum::U33};
use types::deposit::{ExecutionLayerDepositModelWithId, ExecutionLayerDepositModel};


#[derive(Debug, Clone)]
pub struct ConsolidatedExecutionLayerDeposit {
    index: u64,
    block_number: u64,
    deposit_data: DepositData,
    is_signature_valid: bool,
    proof: FixedVector<Hash256, U33>,
    validator_index: u64
}

impl ConsolidatedExecutionLayerDeposit {
    pub fn new(index: u64,
        block_number: u64,
        deposit_data: DepositData,
        is_signature_valid: bool,
        proof: FixedVector<Hash256, U33>,
        validator_index: u64,
    ) -> Self {
        Self {
            index,
            block_number,
            deposit_data,
            is_signature_valid,
            proof,
            validator_index,
        }
    }
}

impl From<&ConsolidatedExecutionLayerDeposit> for ExecutionLayerDepositModelWithId {
    fn from(value: &ConsolidatedExecutionLayerDeposit) -> Self {
        Self {
            id: value.index,
            model: ExecutionLayerDepositModel {
                block_number: value.block_number,
                deposit_data: (&value.deposit_data).into(),
                is_signature_valid: value.is_signature_valid,
            },
        }
    }
}