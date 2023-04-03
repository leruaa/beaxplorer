use std::{sync::Arc, time::Duration};

use lighthouse_network::{rpc::{BlocksByRangeRequest, BlocksByRootRequest}, Request};
use tracing::{warn, instrument, info, debug};
use store::EthSpec;
use task_executor::TaskExecutor;
use tokio::{
    sync::{
        mpsc::{self, Sender, UnboundedSender},
        watch::{Receiver},
    },
    time::{interval_at, Instant},
};
use types::{block_request::{BlockRequestModelWithId, BlockRequestsMeta}, good_peer::{GoodPeerModelWithId, GoodPeersMeta}, persistable::Persistable};

use crate::{
    beacon_chain::beacon_context::BeaconContext,
    db::{Stores, BlockByRootRequests},
    network::{
        augmented_network_service::{AugmentedNetworkService, NetworkCommand, RequestId},
        event::NetworkEvent,
        event_adapter::EventAdapter,
        peer_db::PeerDb,
        workers::Workers,
    },
    types::block_state::BlockState,
    work::Work,
};

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
                let (network_event_send, mut network_event_recv) = mpsc::unbounded_channel();

                let block_requests = BlockRequestModelWithId::iter(&base_dir).unwrap();
                let stores = Arc::new(Stores::new(block_requests.collect()));
                let peer_db = Arc::new(PeerDb::new(
                    network_globals.clone(),
                    good_peers
                        .iter()
                        .filter_map(|m| m.id.parse().ok())
                        .collect(),
                ));

                let workers = Workers::new(&executor, base_dir.clone(), beacon_context, shutdown_trigger.clone());

                let start_instant = Instant::now();
                let interval_duration = Duration::from_secs(1);
                let mut interval = interval_at(start_instant, interval_duration);
                let mut network_event_adapter = EventAdapter::new(network_event_send, stores.clone());

                loop {
                    tokio::select! {
                        Some(event) = internal_network_event_recv.recv() => {
                            network_event_adapter.handle(event);
                            //block_range_request_worker.handle_event(&event);
                            //block_by_root_requests_worker.handle_event(&event)
                        },

                        Some(event) = network_event_recv.recv() => {
                            handle_network_event(event, &network_command_send, &work_send, &peer_db, &stores);
                        },

                        Some(work) = work_recv.recv() => {
                            handle_work(&executor, base_dir.clone(), work, &workers, &peer_db, &stores, &network_command_send);
                        },

                        _ = interval.tick() => {
                            if network_globals.connected_or_dialing_peers() == 0 {
                                warn!("No connected peers");
                            }
                        },
                        
                        _ = shutdown_trigger.changed() => {
                            info!("Shutting down indexer...");
                            persist_block_requests(&base_dir, &stores.block_by_root_requests());
                            persist_good_peers(&base_dir, &peer_db);
                            return;
                        }
                    }
                }
            },
            "indexer",
        );
    }
}

#[instrument(name = "Network", skip_all)]
fn handle_network_event<E: EthSpec>(
    network_event: NetworkEvent<E>,
    network_command_send: &UnboundedSender<NetworkCommand>,
    work_send: &UnboundedSender<Work<E>>,
    peer_db: &Arc<PeerDb<E>>,
    stores: &Arc<Stores<E>>
) {
    match network_event {
        NetworkEvent::PeerConnected(peer_id) => {
            if peer_db.is_good_peer(&peer_id) {
                info!(peer = %peer_id, "Good peer connected");
            }

            if !stores.block_range_request_state().is_requesting() {
                work_send.send(Work::SendRangeRequest(Some(peer_id))).unwrap();
                stores.block_range_request_state_mut().set_to_requesting(peer_id);
            }

            stores.block_by_root_requests_mut().pending_iter_mut().for_each(|(root, req)| {
                if req.insert_peer(&peer_id) {
                    work_send
                        .send(Work::SendBlockByRootRequest(*root, peer_id))
                        .unwrap();
                }
            });
        }
        NetworkEvent::PeerDisconnected(peer_id) => {
            if stores.block_range_request_state().matches(&peer_id) {
                debug!("Range request cancelled");
                work_send.send(Work::SendRangeRequest(None)).unwrap();
            }

            stores.block_by_root_requests_mut().pending_iter_mut().for_each(|(_, req)| {
                req.remove_peer(&peer_id);
            });
        }
        NetworkEvent::RangeRequestSuccedeed  => {
            work_send.send(Work::SendRangeRequest(None)).unwrap();
        }
        NetworkEvent::RangeRequestFailed(peer_id) => {
            network_command_send.send(NetworkCommand::ReportPeer(peer_id, "Range request failed")).unwrap();
            work_send.send(Work::SendRangeRequest(None)).unwrap();
        }
        NetworkEvent::BlockRequestFailed(root, peer_id) => {
            if peer_db.is_good_peer(&peer_id) {
                warn!(peer = %peer_id, "Connection to good peer failed");
            }

            stores.block_by_root_requests_mut().update_attempt(&root, |attempt| {
                attempt.remove_peer(&peer_id);
            });
        }
        NetworkEvent::NewBlock(state) => {
            debug!(%state, slot = %state.slot(), "New block");

            if let Some(work) = stores.block_by_epoch_mut().build_epoch(state) {
                work_send.send(work).unwrap();
            }
        }
        NetworkEvent::UnknownBlockRoot(slot, root) => {
            stores.block_by_root_requests_mut().add(slot, root);
            
            peer_db
                .get_connected_good_peers()
                .into_iter()
                .for_each(|(peer_id, _)| {
                    work_send
                        .send(Work::SendBlockByRootRequest(root, peer_id))
                        .unwrap();
                });
        }
        NetworkEvent::BlockRootFound(root, slot, found_by) => {
            if stores.block_by_root_requests_mut().set_request_as_found(root, found_by) {
                info!(%found_by, %slot, %root, "An orphaned block has been found");

                if let Some(attempt) = stores.block_by_root_requests().get(&root) {
                    // Persist the found block request
                    work_send.send(Work::PersistBlockRequest(root, attempt.clone())).unwrap();
                }

                peer_db.add_good_peer(found_by);

                // Persist good peers
                work_send.send(Work::PersistAllGoodPeers).unwrap();
            }
        }
        NetworkEvent::BlockRootNotFound(root) => {
            stores.block_by_root_requests_mut().update_attempt(&root, |attempt| {
                attempt.increment_not_found();
            });
        },
    }
}

fn handle_work<E: EthSpec>(
    executor: &TaskExecutor,
    base_dir: String,
    work: Work<E>,
    workers: &Workers<E>,
    peer_db: &Arc<PeerDb<E>>,
    stores: &Arc<Stores<E>>,
    network_command_send: &UnboundedSender<NetworkCommand>,
) {
    match work {
        Work::PersistEpoch { epoch, blocks } => {
            workers.epoch_persister.work(epoch, blocks)
        }

        Work::PersistBlock(block) => {
            workers.existing_block_persister.spawn(executor, block)
        },

        Work::PersistBlockRequest(root, attempts) => {
            let block_request = BlockRequestModelWithId::from((&root, &attempts));

            block_request.persist(&base_dir);
        }

        Work::PersistAllBlockRequests => {
            persist_block_requests(&base_dir, &stores.block_by_root_requests())
        }

        Work::PersistAllGoodPeers => {
            persist_good_peers(&base_dir, peer_db)
        },

        Work::SendRangeRequest(to) => {
            match to.or_else(|| peer_db.get_best_connected_peer()) {
                Some(to) => {
                    let start_slot = stores
                        .latest_slot()
                        .map(|s| s.as_u64() + 1)
                        .unwrap_or_default();
        
                    network_command_send
                        .send(NetworkCommand::SendRequest {
                            peer_id: to,
                            request_id: RequestId::Range(start_slot),
                            request: Box::new(Request::BlocksByRange(BlocksByRangeRequest {
                                start_slot,
                                count: 32,
                            })),
                        })
                        .unwrap();
                    stores.block_range_request_state_mut().set_to_requesting(to);
                },
                None => {
                    warn!("No peer available to a new range request");
                    stores.block_range_request_state_mut().set_to_awaiting_peer();
                }
            }
        }

        Work::SendBlockByRootRequest(root, to) => {
            info!(%root, %to, "Send block by root request");
            network_command_send
                .send(NetworkCommand::SendRequest {
                    peer_id: to,
                    request_id: RequestId::Block(root),
                    request: Box::new(Request::BlocksByRoot(BlocksByRootRequest {
                        block_roots: vec![root].into(),
                    })),
                })
                .unwrap();
        }
    }
}

fn persist_block_requests(base_dir: &str, block_by_root_requests: &BlockByRootRequests) {
    let block_requests = Vec::<BlockRequestModelWithId>::from(block_by_root_requests);
    let meta = BlockRequestsMeta::new(block_requests.len());

    block_requests.persist(base_dir);
    meta.persist(base_dir);
}

fn persist_good_peers<E: EthSpec>(base_dir: &str, peer_db: &Arc<PeerDb<E>>) {
    let good_peers = Vec::<GoodPeerModelWithId>::from(&**peer_db);
    let meta = GoodPeersMeta::new(good_peers.len());

    good_peers.persist(base_dir);
    meta.persist(base_dir);
    //info_span!(self.log, "Good peers persisted");
}