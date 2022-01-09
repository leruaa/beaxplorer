use std::{collections::BinaryHeap, fs::File, io::BufWriter};

use crate::{
    beacon_node_client::BeaconNodeClient,
    errors::IndexerError,
    ord_epoch::OrderableEpoch,
    types::{consolidated_epoch::ConsolidatedEpoch, consolidated_validator::ConsolidatedValidator},
};
use eth2::types::StateId;
use lighthouse_types::{Epoch, MainnetEthSpec};
use rmp_serde::Serializer;
use serde::Serialize;
use types::views::{BlockView, EpochView};

pub struct Indexer {
    beacon_client: BeaconNodeClient,
    epochs_by_attestations_count: BinaryHeap<OrderableEpoch<usize>>,
}

impl Indexer {
    pub fn new(endpoint_url: String) -> Self {
        Indexer {
            beacon_client: BeaconNodeClient::new(endpoint_url),
            epochs_by_attestations_count: BinaryHeap::new(),
        }
    }

    pub async fn index_epoch(&mut self, number: u64) -> Result<(), IndexerError> {
        log::info!("Indexing epoch {}", number);

        let epoch = ConsolidatedEpoch::<MainnetEthSpec>::new(
            Epoch::new(number),
            self.beacon_client.clone(),
        )
        .await?;

        for block in &epoch.blocks {
            self.index_block(block.clone().into()).await?
        }

        let view = EpochView::from(epoch);

        self.epochs_by_attestations_count
            .push(OrderableEpoch::from((
                view.epoch.clone(),
                view.attestations_count.clone(),
            )));

        self.persist_epoch(view)?;

        Ok(())
    }

    pub fn persist_epoch(&self, view: EpochView) -> Result<(), IndexerError> {
        let mut f = BufWriter::new(
            File::create(format!(
                "../web_static/public/data/epochs/{}.msg",
                view.epoch
            ))
            .unwrap(),
        );
        view.serialize(&mut Serializer::new(&mut f)).unwrap();

        Ok(())
    }

    pub async fn index_block(&mut self, view: BlockView) -> Result<(), IndexerError> {
        self.persist_block(view)?;

        Ok(())
    }

    pub fn persist_block(&self, view: BlockView) -> Result<(), IndexerError> {
        let mut f = BufWriter::new(
            File::create(format!(
                "../web_static/public/data/blocks/{}.msg",
                view.slot
            ))
            .unwrap(),
        );
        view.serialize(&mut Serializer::new(&mut f)).unwrap();

        Ok(())
    }

    pub async fn index_validators(&self) -> Result<(), IndexerError> {
        log::info!("Indexing validators");

        let validators =
            ConsolidatedValidator::from_state(StateId::Head, self.beacon_client.clone()).await?;

        Ok(())
    }
}
