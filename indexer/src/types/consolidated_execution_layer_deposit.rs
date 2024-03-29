use lighthouse_types::{typenum::U33, DepositData, FixedVector, Hash256};
use types::deposit::{ExecutionLayerDepositModel, ExecutionLayerDepositModelWithId};

#[derive(Debug, Clone)]
pub struct ConsolidatedExecutionLayerDeposit {
    pub index: u64,
    pub block_number: u64,
    pub deposit_data: DepositData,
    pub is_signature_valid: bool,
    pub proof: FixedVector<Hash256, U33>,
    pub validator_index: u64,
}

impl ConsolidatedExecutionLayerDeposit {
    pub fn new(
        index: u64,
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
                validator_index: value.validator_index,
                deposit_data: (&value.deposit_data).into(),
                is_signature_valid: value.is_signature_valid,
            },
        }
    }
}
