use std::{collections::HashMap, sync::Arc};

use lighthouse_network::{Multiaddr, PeerId};
use store::EthSpec;
use task_executor::TaskExecutor;
use tokio::sync::{mpsc::Sender, watch::Receiver};
use tracing::info;
use types::{block_request::BlockRequestModelWithId, good_peer::GoodPeerModelWithId};

use crate::{
    beacon_chain::beacon_context::BeaconContext,
    db::Stores,
    workers::{spawn_index_worker, spawn_persist_block_worker},
};

mod works;

#[derive(Default)]
pub struct Indexer;

impl Indexer {
    pub fn spawn_services<E: EthSpec>(
        self,
        base_dir: String,
        executor: TaskExecutor,
        beacon_context: Arc<BeaconContext<E>>,
        _: Sender<()>,
        shutdown_trigger: Receiver<()>,
    ) {
        self.spawn_indexer(executor, base_dir, beacon_context, shutdown_trigger);
    }

    fn spawn_indexer<E: EthSpec>(
        &self,
        executor: TaskExecutor,
        base_dir: String,
        beacon_context: Arc<BeaconContext<E>>,
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
                let stores = Arc::new(Stores::new(base_dir.clone(), beacon_context.clone(), block_requests.collect()));

                let new_block_send = spawn_persist_block_worker(base_dir.clone(), stores.clone(), shutdown_trigger.clone(), &executor);

                let mut work_recv = spawn_index_worker(beacon_context, stores.clone(), good_peers, &executor);

                loop {
                    tokio::select! {
                        work = work_recv.recv() => {
                            match work {
                                Some(work) => works::handle(base_dir.clone(), work, &stores, &new_block_send, &executor),
                                None => {
                                    works::persist_block_requests(&base_dir, &stores);
                                    works::persist_good_peers(&base_dir, &stores);
                                    return
                                },
                            }
                        },

                        _ = shutdown_trigger.changed() => {
                            info!("Shutting down indexer...");
                            work_recv.close();

                        }
                    }
                }
            },
            "indexer",
        );
    }
}
