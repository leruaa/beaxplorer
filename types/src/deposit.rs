use indexer_macro::Persistable;
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use tsify::Tsify;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DepositData {
    pub public_key: String,
    pub withdrawal_credentials: String,
    pub amount: u64,
    pub signature: String,
}

#[cfg(feature = "indexing")]
impl From<&lighthouse_types::DepositData> for DepositData {
    fn from(value: &lighthouse_types::DepositData) -> Self {
        DepositData {
            public_key: value.pubkey.to_string(),
            withdrawal_credentials: format!("{:?}", value.withdrawal_credentials),
            amount: value.amount,
            signature: value.signature.to_string(),
        }
    }
}

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(model = "default")]
#[persistable(prefix = "/execution_deposits")]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct ExecutionLayerDepositModel {
    pub block_number: u64,
    #[serde(flatten)]
    pub deposit_data: DepositData,
    pub is_signature_valid: bool,
}

#[derive(Persistable, Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[persistable(model = "default")]
#[persistable(prefix = "/consensus_deposits")]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[serde(rename_all = "camelCase")]
pub struct ConsensusLayerDepositModel {
    pub slot: u64,
    #[serde(flatten)]
    pub deposit_data: DepositData,
}
