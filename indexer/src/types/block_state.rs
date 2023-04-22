use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use lighthouse_types::{Epoch, EthSpec, Hash256, SignedBeaconBlock, Slot};
use types::block::BlockExtendedModel;

#[derive(Debug, Clone)]
pub enum BlockState<E: EthSpec> {
    Proposed(Arc<SignedBeaconBlock<E>>),
    Missed(Slot),
    Orphaned(Arc<SignedBeaconBlock<E>>),
}

impl<E: EthSpec> BlockState<E> {
    pub fn slot(&self) -> Slot {
        match self {
            BlockState::Proposed(block) | BlockState::Orphaned(block) => block.slot(),
            BlockState::Missed(s) => *s,
        }
    }

    pub fn epoch(&self) -> Epoch {
        self.slot().epoch(E::slots_per_epoch())
    }

    pub fn canonical_block(&self) -> Option<&Arc<SignedBeaconBlock<E>>> {
        match self {
            BlockState::Proposed(block) => Some(block),
            _ => None,
        }
    }

    pub fn root(&self) -> Option<Hash256> {
        match self {
            BlockState::Proposed(block) | BlockState::Orphaned(block) => {
                Some(block.canonical_root())
            }
            _ => None,
        }
    }
}

impl<E: EthSpec> Display for BlockState<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockState::Proposed(_) => write!(f, "Proposed"),
            BlockState::Missed(_) => write!(f, "Missed"),
            BlockState::Orphaned(_) => write!(f, "Orphaned"),
        }
    }
}

impl<E: EthSpec> From<&BlockState<E>> for Option<Arc<SignedBeaconBlock<E>>> {
    fn from(value: &BlockState<E>) -> Self {
        match value {
            BlockState::Proposed(block) => Some(block.clone()),
            BlockState::Missed(_) => None,
            BlockState::Orphaned(block) => Some(block.clone()),
        }
    }
}

impl<E: EthSpec> From<&BlockState<E>> for Option<BlockExtendedModel> {
    fn from(value: &BlockState<E>) -> Self {
        let block: Option<Arc<SignedBeaconBlock<E>>> = value.into();

        block.map(|block| BlockExtendedModel {
            block_root: block.canonical_root().as_bytes().to_vec(),
            parent_root: block.message().parent_root().as_bytes().to_vec(),
            state_root: block.message().state_root().as_bytes().to_vec(),
            randao_reveal: block
                .message()
                .body()
                .randao_reveal()
                .to_string()
                .as_bytes()
                .to_vec(),
            signature: block.signature().to_string().as_bytes().to_vec(),
            graffiti: block
                .message()
                .body()
                .graffiti()
                .to_string()
                .as_bytes()
                .to_vec(),
            graffiti_text: block.message().body().graffiti().to_string(),
            votes_count: 0,
            eth1data_deposit_root: block
                .message()
                .body()
                .eth1_data()
                .deposit_root
                .as_bytes()
                .to_vec(),
            eth1data_deposit_count: block.message().body().eth1_data().deposit_count,
            eth1data_block_hash: block
                .message()
                .body()
                .eth1_data()
                .block_hash
                .as_bytes()
                .to_vec(),
        })
    }
}
