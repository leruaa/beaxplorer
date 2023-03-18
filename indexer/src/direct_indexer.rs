use std::{
    pin::Pin,
    sync::Arc,
    time::Duration,
};

use futures::Future;

use parking_lot::RwLock;
use slog::{info, Logger};
use store::EthSpec;
use task_executor::TaskExecutor;
use tokio::{
    sync::{
        mpsc::{unbounded_channel, Sender, UnboundedSender, UnboundedReceiver},
        watch::{Receiver, self},
    },
    time::{interval_at, Instant},
};
use types::{block_request::BlockRequestModelWithId, good_peer::GoodPeerModelWithId};

use crate::{
    beacon_chain::beacon_context::BeaconContext,
    network::{
        augmented_network_service::AugmentedNetworkService,
        block_by_root_requests::BlockByRootRequests,
        workers::block_by_root_requests_worker::BlockByRootRequestsWorker, peer_db::PeerDb, workers::{block_range_request_worker::BlockRangeRequestWorker}, persist_service::{PersistService, PersistMessage},
    },
};

// use the executor for libp2p
struct Executor(task_executor::TaskExecutor);

impl libp2p::core::Executor for Executor {
    fn exec(&self, f: Pin<Box<dyn Future<Output = ()> + Send>>) {
        self.0.spawn(f, "libp2p");
    }
}

pub struct Indexer {
    log: Logger,
}

impl Indexer {
    pub fn new(log: Logger) -> Self {
        Indexer {
            log,
        }
    }

    pub fn spawn_services<E: EthSpec>(
        self,
        base_dir: String,
        executor: TaskExecutor,
        beacon_context: Arc<BeaconContext<E>>,
        _: Sender<()>,
        shutdown_trigger: Receiver<()>,
    ) {
        let (persist_send, persist_recv) =
                unbounded_channel::<PersistMessage<E>>();

        let (shutdown_persister_request, shutdown_persister_trigger) = watch::channel(());

        self.spawn_indexer(
            executor.clone(),
            base_dir.clone(),
            beacon_context.clone(),
            persist_send,
            shutdown_persister_request,
            shutdown_trigger);

        PersistService::spawn(
            executor,
            base_dir,
            beacon_context,
            persist_recv,
            shutdown_persister_trigger,
            self.log.clone());
    }

    fn spawn_indexer<E: EthSpec>(
        &self,
        executor: TaskExecutor,
        base_dir: String,
        beacon_context: Arc<BeaconContext<E>>,
        persist_send: UnboundedSender<PersistMessage<E>>,
        shutdown_persister_request: watch::Sender<()>,
        mut shutdown_trigger: Receiver<()>
    ) {
        let log = self.log.clone();

        executor.clone().spawn(
            async move {
                let (network_command_send, mut network_event_recv, network_globals) =
                    AugmentedNetworkService::start(executor, beacon_context)
                        .await.unwrap();

                let block_requests = BlockRequestModelWithId::iter(&base_dir).unwrap();
                let block_by_root_requests = Arc::new(RwLock::new(BlockByRootRequests::from_block_requests(block_requests.collect())));
                let good_peers = GoodPeerModelWithId::iter(&base_dir).unwrap();
                let peer_db = Arc::new(PeerDb::new(network_globals.clone(), good_peers.filter_map(|m| m.id.parse().ok()).collect(), log.clone()));

                let mut block_range_request_worker = BlockRangeRequestWorker::new(
                    peer_db.clone(),
                    network_command_send.clone(),
                    persist_send.clone(),
                    block_by_root_requests.clone(),
                    log.clone(),
                );

                let mut block_by_root_requests_worker = BlockByRootRequestsWorker::new(
                    peer_db,
                    network_command_send.clone(),
                    persist_send,
                    block_by_root_requests,
                    log.clone()
                );

                let start_instant = Instant::now();
                let interval_duration = Duration::from_secs(1);
                let mut interval = interval_at(start_instant, interval_duration);

                block_by_root_requests_worker.dial_good_peers();
                
                loop {
                    tokio::select! {
                        Some(event) = network_event_recv.recv() => {
                            block_range_request_worker.handle_event(&event);
                            block_by_root_requests_worker.handle_event(&event)
                        },
                        
                        _ = interval.tick() => {
                            info!(log, "Status"; "connected peers" => network_globals.connected_peers());
                        },
                        _ = shutdown_trigger.changed() => {
                            info!(log, "Shutting down indexer...");
                            block_by_root_requests_worker.persist();
                            shutdown_persister_request.send(()).unwrap();
                            return;
                        }
                    }
                }
            },
            "indexer",
        );
    }
}
