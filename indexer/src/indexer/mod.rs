use std::{collections::HashMap, sync::Arc};

use lighthouse_network::{Multiaddr, PeerId};
use serde::Serialize;
use store::EthSpec;
use task_executor::TaskExecutor;
use tokio::sync::{mpsc::Sender, watch::Receiver};
use tracing::info;
use types::{block_request::BlockRequestModelWithId, good_peer::GoodPeerModelWithId, DeserializeOwned};

use crate::{
    beacon_chain::beacon_context::BeaconContext,
    db::Stores,
    network::{spawn_consensus_network, spawn_execution_network, ExecutionNetworkCommand},
    workers::{spawn_index_worker, spawn_persist_block_worker, spawn_persist_validator_worker},
};

mod works;

#[derive(Default)]
pub struct Indexer;

impl Indexer {
    pub fn spawn_services<E: EthSpec + Serialize + DeserializeOwned>(
        self,
        dry: bool,
        base_dir: String,
        execution_node_url: String,
        executor: TaskExecutor,
        beacon_context: Arc<BeaconContext<E>>,
        shutdown_handle: Sender<()>,
        shutdown_trigger: Receiver<()>,
    ) {
        self.spawn_indexer(
            executor,
            dry,
            base_dir,
            execution_node_url,
            beacon_context,
            shutdown_handle,
            shutdown_trigger,
        );
    }

    fn spawn_indexer<E: EthSpec + Serialize + DeserializeOwned>(
        &self,
        executor: TaskExecutor,
        dry: bool,
        base_dir: String,
        execution_node_url: String,
        beacon_context: Arc<BeaconContext<E>>,
        shutdown_handle: Sender<()>,
        mut shutdown_trigger: Receiver<()>,
    ) {
        info!("Starting indexer");
        executor.clone().spawn(
            async move {
                let good_peers = GoodPeerModelWithId::iter(&base_dir)
                    .unwrap();

                let good_peers = good_peers
                    .filter_map(|m| m.id.parse().ok().and_then(|id| m.model.address.parse().ok().map(|a|(id, a))))
                    .collect::<HashMap<PeerId, Multiaddr>>();

                let block_requests = BlockRequestModelWithId::iter(&base_dir).unwrap();
                let stores = Arc::new(Stores::new(base_dir.clone(), beacon_context.genesis_state.clone(), beacon_context.eth2_network_config.deposit_contract_deploy_block, block_requests.collect()));

                let (execution_command_send, execution_event_recv) = spawn_execution_network(
                execution_node_url.parse().unwrap(), beacon_context.clone(), &executor)
                        .unwrap();

                let (consensus_command_send, consensus_event_recv) = spawn_consensus_network(beacon_context.clone(), good_peers, &executor)
                    .await
                    .unwrap();

                let new_block_send = spawn_persist_block_worker(base_dir.clone(), stores.clone(), shutdown_trigger.clone(), &executor);

                let validator_event_send = spawn_persist_validator_worker(base_dir.clone(), stores.clone(), &executor, shutdown_handle);

                let latest_deposit_block = *stores.get_latest_deposit_block().get_or_insert(beacon_context
                    .eth2_network_config
                    .deposit_contract_deploy_block);

                execution_command_send.send(ExecutionNetworkCommand::RetrieveDeposits(latest_deposit_block..latest_deposit_block + 1000)).unwrap();

                let mut work_recv = spawn_index_worker(
                    execution_event_recv,
                    execution_command_send,
                    consensus_event_recv,
                    consensus_command_send,
                    stores.clone(),
                    &executor);

                loop {
                    tokio::select! {
                        work = work_recv.recv(), if !dry => {
                            match work {
                                Some(work) => works::handle(base_dir.clone(), work, &stores, &new_block_send, &validator_event_send, &executor),
                                None => {
                                    works::persist_indexing_state(&base_dir, &stores);
                                    works::persist_block_requests(&base_dir, &stores);
                                    works::persist_good_peers(&base_dir, &stores);
                                    return
                                },
                            }
                        },

                        _ = shutdown_trigger.changed() => {
                            info!("Shutting down indexer...");
                            if dry {
                                return;
                            }
                            else {
                                work_recv.close();
                            }
                        }
                    }
                }
            },
            "indexer",
        );
    }
}
