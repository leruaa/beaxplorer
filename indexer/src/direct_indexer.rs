use std::{
    pin::Pin,
    sync::Arc,
    time::Duration,
};

use futures::Future;

use parking_lot::RwLock;
use slog::{info, Logger};
use store::{EthSpec, SignedBeaconBlock, Slot};
use task_executor::TaskExecutor;
use tokio::{
    sync::{
        mpsc::{unbounded_channel, Sender, UnboundedSender, UnboundedReceiver},
        watch::Receiver,
    },
    time::{interval_at, Instant},
};

use crate::{
    beacon_chain::beacon_context::BeaconContext,
    network::{
        augmented_network_service::AugmentedNetworkService,
        block_by_root_requests::BlockByRootRequests,
        workers::block_by_root_requests_worker::BlockByRootRequestsWorker, peer_db::PeerDb, worker::Worker, workers::persist_worker::PersistWorker,
    },
};

// use the executor for libp2p
struct Executor(task_executor::TaskExecutor);

impl libp2p::core::Executor for Executor {
    fn exec(&self, f: Pin<Box<dyn Future<Output = ()> + Send>>) {
        self.0.spawn(f, "libp2p");
    }
}

#[derive(Debug, Clone)]
pub enum BlockMessage<E: EthSpec> {
    Proposed(Arc<SignedBeaconBlock<E>>),
    Orphaned(Arc<SignedBeaconBlock<E>>),
    Missed(Slot),
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
        let block_by_root_requests = Arc::new(RwLock::new(BlockByRootRequests::new()));
        let (block_send, block_recv) = unbounded_channel::<BlockMessage<E>>();

        self.spawn_indexer(
            executor.clone(),
            beacon_context.clone(),
            block_send,
            block_by_root_requests.clone(),
            shutdown_trigger.clone());

        self.spawn_persister(
            executor,
            base_dir,
            beacon_context,
            block_by_root_requests,
            block_recv,
            shutdown_trigger);
    }

    fn spawn_indexer<E: EthSpec>(
        &self,
        executor: TaskExecutor,
        beacon_context: Arc<BeaconContext<E>>,
        block_send: UnboundedSender<BlockMessage<E>>,
        block_by_root_requests: Arc<RwLock<BlockByRootRequests>>,
        mut shutdown_trigger: Receiver<()>
    ) {
        let log = self.log.clone();

        executor.clone().spawn(
            async move {
                let (network_command_send, mut network_event_recv, network_globals) =
                    AugmentedNetworkService::start(executor, beacon_context)
                        .await.unwrap();

                let mut worker = Worker::new(
                    PeerDb::new(network_globals.clone(), log.clone()),
                    network_command_send.clone(),
                    block_send.clone(),
                    block_by_root_requests.clone(),
                    log.clone(),
                );

                let mut block_by_root_requests_worker = BlockByRootRequestsWorker::new(
                    network_command_send.clone(),
                    block_send,
                    block_by_root_requests.clone(),
                    log.clone()
                );

                let start_instant = Instant::now();
                let interval_duration = Duration::from_secs(1);
                let mut interval = interval_at(start_instant, interval_duration);

                loop {
                    tokio::select! {
                        Some(event) = network_event_recv.recv() => {
                            worker.handle_event(&event);
                            block_by_root_requests_worker.handle_event(&event)
                        },
                        
                        _ = interval.tick() => {
                            info!(log, "Status"; "connected peers" => network_globals.connected_peers());
                        },
                        _ = shutdown_trigger.changed() => {
                            info!(log, "Shutting down indexer...");
                            return;
                        }
                    }
                }
            },
            "indexer",
        );
    }

    fn spawn_persister<E: EthSpec>(
        &self,
        executor: TaskExecutor,
        base_dir: String,
        beacon_context: Arc<BeaconContext<E>>,
        block_by_root_requests: Arc<RwLock<BlockByRootRequests>>,
        mut block_recv: UnboundedReceiver<BlockMessage<E>>,
        mut shutdown_trigger: Receiver<()>,
    ) {
        let log = self.log.clone();
        let mut persist_worker = PersistWorker::new(base_dir, beacon_context, block_by_root_requests, log.clone());

        executor.spawn(async move {

            loop {
                tokio::select! {
                    Some(block_message) = block_recv.recv() => persist_worker.handle_block_message(block_message),
                    _ = shutdown_trigger.changed() => {
                        info!(log, "Shutting down indexer...");
                        return;
                    }
                }
            }

        }, "persist service");
    }
}
