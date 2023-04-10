use std::{sync::Arc, time::Duration};

use lighthouse_network::{ Multiaddr, PeerId};
use tracing::{warn, info};
use store::EthSpec;
use task_executor::TaskExecutor;
use tokio::{
    sync::{
        mpsc::{self, Sender},
        watch::{Receiver},
    },
    time::{interval_at, Instant},
};
use types::{block_request::BlockRequestModelWithId, good_peer::GoodPeerModelWithId};

use crate::{
    beacon_chain::beacon_context::BeaconContext,
    db::Stores,
    network::{
        augmented_network_service::AugmentedNetworkService,
    }, workers::spawn_persist_block_worker,
};

mod eth_events;
mod network_events;
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
        self.spawn_indexer(
            executor,
            base_dir,
            beacon_context,
            shutdown_trigger,
        );
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
                    .collect::<Vec<(PeerId, Multiaddr)>>();

                let (network_command_send, mut internal_network_event_recv, network_globals) =
                    AugmentedNetworkService::start(executor.clone(), beacon_context.clone(), good_peers.iter().map(|(_, a)| a.clone()).collect::<Vec<_>>())
                        .await
                        .unwrap();

                let (work_send, mut work_recv) = mpsc::unbounded_channel();
                let (network_event_send, mut network_event_recv) = mpsc::unbounded_channel();

                let block_requests = BlockRequestModelWithId::iter(&base_dir).unwrap();
                let stores = Arc::new(Stores::new(base_dir.clone(), network_globals.clone(), beacon_context, block_requests.collect(), good_peers));

                let new_block_send = spawn_persist_block_worker(base_dir.clone(), stores.clone(), shutdown_trigger.clone(), &executor);

                let start_instant = Instant::now();
                let interval_duration = Duration::from_secs(1);
                let mut interval = interval_at(start_instant, interval_duration);

                loop {
                    tokio::select! {
                        Some(event) = internal_network_event_recv.recv() => {
                            eth_events::handle(event, &network_event_send, &work_send, &stores);
                        },

                        Some(event) = network_event_recv.recv() => {
                            network_events::handle(event, &network_command_send, &work_send, &stores);
                        },

                        Some(work) = work_recv.recv() => {
                            works::handle(base_dir.clone(), work, &stores, &network_command_send, &new_block_send, &executor);
                        },

                        _ = interval.tick() => {
                            if network_globals.connected_or_dialing_peers() == 0 {
                                warn!("No connected peers");
                            }
                        },
                        
                        _ = shutdown_trigger.changed() => {
                            info!("Shutting down indexer...");
                            works::persist_block_requests(&base_dir, &stores.block_by_root_requests());
                            works::persist_good_peers(&base_dir, &stores.peer_db());
                            return;
                        }
                    }
                }
            },
            "indexer",
        );
    }
}