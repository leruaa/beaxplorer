use std::{pin::Pin, sync::Arc, time::Duration};

use futures::Future;

use lighthouse_network::{rpc::BlocksByRangeRequest, Request};
use slog::{debug, info, warn, Logger};
use store::EthSpec;
use task_executor::TaskExecutor;
use tokio::{
    sync::{
        mpsc::{self, unbounded_channel, Sender, UnboundedSender},
        watch::{self, Receiver},
    },
    time::{interval_at, Instant},
};
use types::{block_request::BlockRequestModelWithId, good_peer::GoodPeerModelWithId};

use crate::{
    beacon_chain::beacon_context::BeaconContext,
    db::blocks_by_epoch::BlocksByEpoch,
    network::{
        augmented_network_service::{AugmentedNetworkService, NetworkCommand, RequestId},
        block_by_root_requests::BlockByRootRequests,
        block_db::BlockDb,
        event::NetworkEvent,
        event_adapter::EventAdapter,
        peer_db::PeerDb,
        persist_service::{PersistMessage, PersistService},
        workers::Workers,
    },
    types::block_state::BlockState,
    work::Work,
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
        Indexer { log }
    }

    pub fn spawn_services<E: EthSpec>(
        self,
        base_dir: String,
        executor: TaskExecutor,
        beacon_context: Arc<BeaconContext<E>>,
        _: Sender<()>,
        shutdown_trigger: Receiver<()>,
    ) {
        let (persist_send, persist_recv) = unbounded_channel::<PersistMessage<E>>();

        let (shutdown_persister_request, shutdown_persister_trigger) = watch::channel(());

        self.spawn_indexer(
            executor.clone(),
            base_dir.clone(),
            beacon_context.clone(),
            persist_send,
            shutdown_persister_request,
            shutdown_trigger,
        );

        PersistService::spawn(
            executor,
            base_dir,
            beacon_context,
            persist_recv,
            shutdown_persister_trigger,
            self.log.clone(),
        );
    }

    fn spawn_indexer<E: EthSpec>(
        &self,
        executor: TaskExecutor,
        base_dir: String,
        beacon_context: Arc<BeaconContext<E>>,
        persist_send: UnboundedSender<PersistMessage<E>>,
        shutdown_persister_request: watch::Sender<()>,
        mut shutdown_trigger: Receiver<()>,
    ) {
        let log = self.log.clone();

        executor.clone().spawn(
            async move {
                let good_peers = GoodPeerModelWithId::iter(&base_dir)
                    .unwrap()
                    .collect::<Vec<_>>();
                let known_addresses = good_peers
                    .iter()
                    .filter_map(|m| m.model.address.parse().ok())
                    .collect();
                let (network_command_send, mut internal_network_event_recv, network_globals) =
                    AugmentedNetworkService::start(executor.clone(), beacon_context.clone(), known_addresses)
                        .await
                        .unwrap();

                let (work_send, mut work_recv) = mpsc::unbounded_channel();

                let block_requests = BlockRequestModelWithId::iter(&base_dir).unwrap();
                let block_by_root_requests =
                    BlockByRootRequests::from_block_requests(block_requests.collect());
                let block_db = BlockDb::new();
                let peer_db = Arc::new(PeerDb::new(
                    network_globals.clone(),
                    good_peers
                        .iter()
                        .filter_map(|m| m.id.parse().ok())
                        .collect(),
                    log.clone(),
                ));
                let mut block_by_epoch = BlocksByEpoch::new();

                let workers = Workers::new(base_dir.clone(), beacon_context);

                let start_instant = Instant::now();
                let interval_duration = Duration::from_secs(1);
                let mut interval = interval_at(start_instant, interval_duration);
                let mut network_event_adapter = EventAdapter::new(block_db.clone());
                let mut network_event_recv = network_event_adapter.receiver();

                loop {
                    tokio::select! {
                        Some(event) = internal_network_event_recv.recv() => {
                            network_event_adapter.handle(event);
                            //block_range_request_worker.handle_event(&event);
                            //block_by_root_requests_worker.handle_event(&event)
                        },

                        Ok(event) = network_event_recv.recv() => {
                            handle_network_event(event, &work_send, &block_db, &mut block_by_epoch, &peer_db, &log);
                        },

                        Some(work) = work_recv.recv() => {
                            handle_work(&executor, work, &workers, &block_db, &network_command_send);
                        },

                        _ = interval.tick() => {
                            if network_globals.connected_or_dialing_peers() == 0 {
                                warn!(log, "No connected peers");
                            }
                        },
                        
                        _ = shutdown_trigger.changed() => {
                            info!(log, "Shutting down indexer...");
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

fn handle_network_event<E: EthSpec>(
    network_event: NetworkEvent<E>,
    work_send: &UnboundedSender<Work<E>>,
    block_db: &Arc<BlockDb>,
    blocks_by_epoch: &mut BlocksByEpoch<E>,
    peer_db: &Arc<PeerDb<E>>,
    log: &Logger,
) {
    match network_event {
        NetworkEvent::PeerConnected(peer_id) => {
            if peer_db.is_good_peer(&peer_id) {
                info!(log, "Good peer connected"; "peer" => %peer_id);
            }

            if !block_db.is_requesting_block_range() {
                work_send.send(Work::SendRangeRequest(peer_id)).unwrap();
                block_db.block_range_requesting(peer_id);
            }

            block_db.for_each_pending_block_by_root_requests(|(root, req)| {
                if req.insert_peer(&peer_id) {
                    work_send
                        .send(Work::SendBlockByRootRequest(peer_id, *root))
                        .unwrap();
                }
            });
        }
        NetworkEvent::PeerDisconnected(peer_id) => {
            if block_db.block_range_matches(&peer_id) {
                debug!(log, "Range request cancelled");
                work_send.send(Work::SendRangeRequest(peer_id)).unwrap();
                block_db.block_range_requesting(peer_id);
            }

            block_db.for_each_pending_block_by_root_requests(|(_, req)| {
                req.remove_peer(&peer_id);
            });
        }
        NetworkEvent::RangeRequestSuccedeed | NetworkEvent::RangeRequestFailed => {
            if let Some(peer_id) = peer_db.get_best_connected_peer() {
                work_send.send(Work::SendRangeRequest(peer_id)).unwrap();
                block_db.block_range_requesting(peer_id);
            } else {
                block_db.block_range_awaiting_peer();
            }
        }
        NetworkEvent::BlockRequestFailed(root, peer_id) => {
            if peer_db.is_good_peer(&peer_id) {
                warn!(log, "Connection to good peer failed"; "peer" => peer_id.to_string());
            }

            block_db.for_each_pending_block_by_root_requests(|(_, req)| {
                req.remove_peer(&peer_id);
            });
        }
        NetworkEvent::NewBlock(block) => {
            if let BlockState::Proposed(block) = &block {
                block_db.update(block.slot(), block.canonical_root());
            }

            if let Some(e) = blocks_by_epoch.build_epoch(block) {
                work_send.send(Work::PersistEpoch(e)).unwrap();
            }
        }
        NetworkEvent::UnknownBlockRoot(_, root) => {
            peer_db
                .get_connected_good_peers()
                .into_iter()
                .for_each(|(peer_id, _)| {
                    work_send
                        .send(Work::SendBlockByRootRequest(peer_id, root))
                        .unwrap();
                });
        }
        NetworkEvent::BlockRootFound(root, slot, peer_id) => {
            block_db.with_found_block_root(root, peer_id, |e| {
                info!(log, "An orphaned block has been found"; "peer" => %peer_id, "slot" => slot, "root" => %root);

                peer_db.add_good_peer(peer_id);
            });
        }
        NetworkEvent::BlockRootNotFound(_) => todo!(),
    }
}

fn handle_work<E: EthSpec>(
    executor: &TaskExecutor,
    work: Work<E>,
    workers: &Workers<E>,
    block_db: &Arc<BlockDb>,
    network_command_send: &UnboundedSender<NetworkCommand>,
) {
    match work {
        Work::PersistEpoch(epoch_to_persist) => {
            workers.persist_epoch.spawn(executor, epoch_to_persist)
        }
        Work::SendRangeRequest(peer_id) => {
            let start_slot = block_db
                .latest_slot()
                .map(|s| s.as_u64() + 1)
                .unwrap_or_default();

            network_command_send
                .send(NetworkCommand::SendRequest {
                    peer_id,
                    request_id: RequestId::Range(start_slot),
                    request: Box::new(Request::BlocksByRange(BlocksByRangeRequest {
                        start_slot,
                        count: 32,
                    })),
                })
                .unwrap();
        }
        Work::SendBlockByRootRequest(_, _) => todo!(),
        Work::SendNetworkMessage(_) => todo!(),
    }
}
