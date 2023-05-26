use std::sync::Arc;

use environment::EnvironmentBuilder;
use lighthouse_types::MainnetEthSpec;

use tokio::{
    signal,
    sync::{mpsc, watch},
};
use tracing::warn;
use types::{
    attestation::AttestationModel,
    block::{BlockExtendedModel, BlockModel, BlockModelWithId, PersistIteratorBlockModel},
    block_request::{BlockRequestModel, BlockRequestModelWithId, PersistIteratorBlockRequestModel},
    block_root::BlockRootModel,
    committee::CommitteeModel,
    deposit::{ConsensusLayerDepositModel, ExecutionLayerDepositModel},
    epoch::{EpochExtendedModel, EpochModel, EpochModelWithId, PersistIteratorEpochModel},
    good_peer::{GoodPeerModel, GoodPeerModelWithId, PersistIteratorGoodPeerModel},
    path::Dirs,
    validator::{ValidatorModel, ValidatorExtendedModel},
    vote::VoteModel,
};

use crate::{
    beacon_chain::beacon_context::{build_environment, BeaconContext},
    indexer::Indexer,
};

pub fn start_indexer(
    reset: bool,
    dry: bool,
    base_dir: String,
    execution_node_url: String,
) -> Result<(), String> {
    let (environment, _) = build_environment(EnvironmentBuilder::mainnet())?;
    let context = environment.core_context();
    let beacon_context = BeaconContext::<MainnetEthSpec>::build(context.eth2_config.spec)?;
    let executor = context.executor;

    if reset {
        if let Err(err) = remove_dirs(&base_dir) {
            warn!("{err}");
        }
    }

    create_dirs(&base_dir)?;

    environment.runtime().block_on(async move {
        let (shutdown_handle, mut shutdown_complete) = mpsc::channel(1);
        let (shutdown_request, shutdown_trigger) = watch::channel(());
        let indexer = Indexer::default();

        indexer.spawn_services(
            dry,
            base_dir,
            execution_node_url,
            executor,
            Arc::new(beacon_context),
            shutdown_handle,
            shutdown_trigger,
        );

        wait_shutdown_signal().await;

        let _ = shutdown_request.send(());

        // When every sender has gone out of scope, the recv call
        // will return with an error. We ignore the error.
        let _ = shutdown_complete.recv().await;
    });

    Ok(())
}

pub fn update_indexes(base_dir: String) -> Result<(), String> {
    EpochModelWithId::iter(&base_dir)?.persist_sortables(&base_dir)?;
    BlockModelWithId::iter(&base_dir)?.persist_sortables(&base_dir)?;
    BlockRequestModelWithId::iter(&base_dir)?.persist_sortables(&base_dir)?;
    GoodPeerModelWithId::iter(&base_dir)?.persist_sortables(&base_dir)?;

    Ok(())
}

fn create_dirs(base_dir: &str) -> Result<(), String> {
    EpochModel::create_dirs(base_dir)?;
    EpochExtendedModel::create_dirs(base_dir)?;
    BlockModel::create_dirs(base_dir)?;
    BlockExtendedModel::create_dirs(base_dir)?;
    BlockRootModel::create_dirs(base_dir)?;
    AttestationModel::create_dirs(base_dir)?;
    CommitteeModel::create_dirs(base_dir)?;
    VoteModel::create_dirs(base_dir)?;
    ValidatorModel::create_dirs(base_dir)?;
    ValidatorExtendedModel::create_dirs(base_dir)?;
    ExecutionLayerDepositModel::create_dirs(base_dir)?;
    ConsensusLayerDepositModel::create_dirs(base_dir)?;
    BlockRequestModel::create_dirs(base_dir)?;
    GoodPeerModel::create_dirs(base_dir)?;

    Ok(())
}

fn remove_dirs(base_dir: &str) -> Result<(), String> {
    EpochModel::remove_dirs(base_dir)?;
    EpochExtendedModel::remove_dirs(base_dir)?;
    BlockModel::remove_dirs(base_dir)?;
    BlockExtendedModel::remove_dirs(base_dir)?;
    BlockRootModel::remove_dirs(base_dir)?;
    AttestationModel::remove_dirs(base_dir)?;
    CommitteeModel::remove_dirs(base_dir)?;
    VoteModel::remove_dirs(base_dir)?;
    ValidatorModel::remove_dirs(base_dir)?;
    ValidatorExtendedModel::remove_dirs(base_dir)?;
    ConsensusLayerDepositModel::remove_dirs(base_dir)?;
    BlockRequestModel::remove_dirs(base_dir)?;

    Ok(())
}

async fn wait_shutdown_signal() {
    match signal::ctrl_c().await {
        Ok(()) => {}
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        }
    }
}
