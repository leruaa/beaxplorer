use std::{sync::Arc, time::Instant};

use db::{models::BlockModel, schema::blocks, PgConnection, RunQueryDsl};
use eth2::types::{BlockId, ProposerData};
use shared::utils::convert::{IntoClampedI32, IntoClampedI64};
use tokio::sync::RwLock;
use types::{BeaconBlock, Epoch, EthSpec, Hash256, Signature, Slot};

use crate::{beacon_node_client::BeaconNodeClient, errors::IndexerError, persistable::Persistable};

#[derive(Debug)]
pub struct ConsolidatedBlock<E: EthSpec> {
    pub epoch: Epoch,
    pub slot: Slot,
    pub block: Option<BeaconBlock<E>>,
    pub block_root: Hash256,
    pub signature: Signature,
    pub status: BlockStatus,
    pub proposer: u64,
}

#[derive(Debug)]
pub enum BlockStatus {
    Scheduled = 0,
    Proposed = 1,
    Missed = 2,
    Orphaned = 3,
}

impl std::fmt::Display for BlockStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<E: EthSpec> ConsolidatedBlock<E> {
    pub async fn new(
        epoch: Epoch,
        slot: Slot,
        proposer_duties_lock: Arc<RwLock<Option<Vec<ProposerData>>>>,
        client: BeaconNodeClient,
    ) -> Result<Self, IndexerError> {
        let start = Instant::now();
        let block = BlockId::Slot(slot);
        let block_response = client.get_block::<E>(block).await?;
        let duration = start.elapsed();
        log::trace!("get_block duration: {:?}", duration);

        if let Some(block_response) = block_response {
            let start = Instant::now();
            let block_root = client.get_block_root(block).await?;
            let duration = start.elapsed();
            log::trace!("get_block_root duration: {:?}", duration);
            let consolidated_block = ConsolidatedBlock {
                epoch,
                slot: block_response.data.message.slot,
                block: Some(block_response.data.message.clone()),
                block_root: block_root.data.root,
                signature: block_response.data.signature,
                status: BlockStatus::Proposed,
                proposer: block_response.data.message.proposer_index,
            };

            return Ok(consolidated_block);
        } else {
            let mut proposer_duties = proposer_duties_lock.read().await.clone();

            if proposer_duties.is_none() {
                let mut proposer_duties_writable = proposer_duties_lock.write().await;
                proposer_duties_writable
                    .replace(client.get_validator_duties_proposer(epoch).await?);
                proposer_duties = proposer_duties_writable.clone();
            }

            if let Some(proposer_duties) = proposer_duties {
                for proposer in proposer_duties {
                    if proposer.slot == slot {
                        let consolidated_block = ConsolidatedBlock {
                            epoch,
                            slot: proposer.slot,
                            block: None,
                            block_root: Hash256::zero(),
                            signature: Signature::empty(),
                            status: BlockStatus::Missed,
                            proposer: proposer.validator_index,
                        };

                        return Ok(consolidated_block);
                    }
                }
            }
        }

        Err(IndexerError::ElementNotFound(block.to_string()))
    }

    pub fn as_model(&self) -> Result<BlockModel, IndexerError> {
        let epoch_as_i64 = self.epoch.as_u64().into_i64();
        let slot_as_i64 = self.slot.as_u64().into_i64();
        let proposer_as_i32 = self.proposer.into_i32();

        let block = match self.block.clone() {
            Some(block) => {
                let eth1data_deposit_count_as_i32 = block.body.eth1_data.deposit_count.into_i32();

                BlockModel {
                    epoch: epoch_as_i64,
                    slot: slot_as_i64,
                    block_root: self.block_root.as_bytes().to_vec(),
                    parent_root: block.parent_root.as_bytes().to_vec(),
                    state_root: block.state_root.as_bytes().to_vec(),
                    randao_reveal: Some(block.body.randao_reveal.to_string().as_bytes().to_vec()),
                    signature: self.signature.to_string().as_bytes().to_vec(),
                    graffiti: Some(block.body.graffiti.to_string().as_bytes().to_vec()),
                    graffiti_text: Some(block.body.graffiti.to_string()),
                    eth1data_deposit_root: Some(
                        block.body.eth1_data.deposit_root.as_bytes().to_vec(),
                    ),
                    eth1data_deposit_count: eth1data_deposit_count_as_i32,
                    eth1data_block_hash: Some(block.body.eth1_data.block_hash.as_bytes().to_vec()),
                    proposer_slashings_count: block.body.proposer_slashings.len() as i32,
                    attester_slashings_count: block.body.attester_slashings.len() as i32,
                    attestations_count: block.body.attestations.len() as i32,
                    deposits_count: block.body.deposits.len() as i32,
                    voluntary_exits_count: block.body.voluntary_exits.len() as i32,
                    proposer: proposer_as_i32,
                    status: self.status.to_string(),
                }
            }
            None => BlockModel {
                epoch: epoch_as_i64,
                slot: slot_as_i64,
                block_root: self.block_root.as_bytes().to_vec(),
                parent_root: vec![],
                state_root: vec![],
                randao_reveal: None,
                signature: vec![],
                graffiti: None,
                graffiti_text: None,
                eth1data_deposit_root: None,
                eth1data_deposit_count: 0,
                eth1data_block_hash: None,
                proposer_slashings_count: 0,
                attester_slashings_count: 0,
                attestations_count: 0,
                deposits_count: 0,
                voluntary_exits_count: 0,
                proposer: proposer_as_i32,
                status: self.status.to_string(),
            },
        };

        Ok(block)
    }

    pub fn get_attestations_count(&self) -> usize {
        match self.block.clone() {
            None => 0,
            Some(block) => block.body.attestations.len(),
        }
    }

    pub fn get_deposits_count(&self) -> usize {
        match self.block.clone() {
            None => 0,
            Some(block) => block.body.deposits.len(),
        }
    }

    pub fn get_voluntary_exits_count(&self) -> usize {
        match self.block.clone() {
            None => 0,
            Some(block) => block.body.voluntary_exits.len(),
        }
    }

    pub fn get_proposer_slashings_count(&self) -> usize {
        match self.block.clone() {
            None => 0,
            Some(block) => block.body.proposer_slashings.len(),
        }
    }

    pub fn get_attester_slashings_count(&self) -> usize {
        match self.block.clone() {
            None => 0,
            Some(block) => block.body.attester_slashings.len(),
        }
    }
}

impl<E: EthSpec> Persistable for ConsolidatedBlock<E> {
    fn persist(&self, db_connection: &PgConnection) -> Result<(), IndexerError> {
        let block_model = self.as_model()?;
        db::insert_into(blocks::table)
            .values(block_model)
            .execute(db_connection)?;

        Ok(())
    }
}
