use std::{collections::HashMap, iter::once, sync::Arc};

use itertools::Itertools;
use lighthouse_network::{Multiaddr, NetworkEvent as LighthouseNetworkEvent, PeerId, Response};
use lighthouse_types::{EthSpec, SignedBeaconBlock, Slot};
use task_executor::TaskExecutor;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tracing::{debug, error, info, warn};
use types::block_root::{BlockRootModel, BlockRootModelWithId};

use crate::{
    beacon_chain::beacon_context::BeaconContext,
    db::Stores,
    network::consensus_network::{ConsensusNetwork, RequestId},
    types::block_state::BlockState,
    work::Work,
};

pub fn spawn_index_worker<E: EthSpec>(
    beacon_context: Arc<BeaconContext<E>>,
    stores: Arc<Stores<E>>,
    good_peers: HashMap<PeerId, Multiaddr>,
    executor: &TaskExecutor,
) -> UnboundedReceiver<Work<E>> {
    let (work_send, work_recv) = mpsc::unbounded_channel();

    let ex = executor.clone();

    executor.spawn(
        async move {
            let mut consensus_network = ConsensusNetwork::new(beacon_context, good_peers, &ex)
                .await
                .unwrap();

            loop {
                handle_next_event(&mut consensus_network, &stores, &work_send).await;
            }
        },
        "index worker",
    );

    work_recv
}

async fn handle_next_event<E: EthSpec>(
    consensus_network: &mut ConsensusNetwork<E>,
    stores: &Arc<Stores<E>>,
    work_send: &UnboundedSender<Work<E>>,
) {
    match consensus_network.next_event().await {
        LighthouseNetworkEvent::PeerConnectedOutgoing(peer_id) => {
            if consensus_network.peer_db().is_good_peer(&peer_id) {
                info!(peer = %peer_id, "Good peer connected");
            }

            if !stores.block_range_requests().is_requesting() {
                send_range_request(Some(peer_id), consensus_network, &stores);
            }

            stores
                .block_by_root_requests_mut()
                .pending_iter_mut()
                .for_each(|(root, req)| {
                    if req.insert_peer(&peer_id) {
                        consensus_network.send_block_by_root_request(peer_id, *root);
                    }
                });
        }

        LighthouseNetworkEvent::PeerDisconnected(peer_id) => {
            let mut block_range_requests = stores.block_range_requests_mut();

            if block_range_requests.request_terminated(&peer_id) {
                debug!(to = %peer_id, "Range request cancelled");
                if !block_range_requests.is_requesting() {
                    send_range_request(None, consensus_network, stores);
                }
            }

            stores
                .block_by_root_requests_mut()
                .pending_iter_mut()
                .for_each(|(_, req)| {
                    req.remove_peer(&peer_id);
                });
        }

        LighthouseNetworkEvent::RPCFailed {
            id: RequestId::Range,
            peer_id,
        } => {
            consensus_network.report_peer(peer_id, "Range request failed");
            send_range_request(None, consensus_network, stores);
        }

        LighthouseNetworkEvent::RPCFailed {
            id: RequestId::Block(root),
            peer_id,
        } => {
            if consensus_network.peer_db().is_good_peer(&peer_id) {
                warn!(peer = %peer_id, "Connection to good peer failed");
            }

            stores
                .block_by_root_requests_mut()
                .update_attempt(&root, |attempt| {
                    attempt.remove_peer(&peer_id);
                });
        }

        LighthouseNetworkEvent::ResponseReceived {
            id: RequestId::Range,
            response: Response::BlocksByRange(block),
            peer_id,
        } => {
            let mut block_range_requests = stores.block_range_requests_mut();

            if let Some(block) = block {
                stores
                    .block_roots_cache()
                    .write()
                    .put(BlockRootModelWithId {
                        id: format!("{:?}", block.canonical_root()),
                        model: BlockRootModel {
                            slot: block.slot().as_u64(),
                        },
                    });

                let block = block_range_requests.next_or(block);

                if Some(block.slot()) > stores.indexing_state().latest_slot() {
                    let processing_result =
                        new_blocks(block.clone(), stores).try_for_each(|block| {
                            match stores.indexing_state_mut().process_block(block) {
                                Ok((block, epoch)) => {
                                    work_send.send(Work::PersistBlock(block)).unwrap();

                                    if let Some(epoch) = epoch {
                                        work_send.send(Work::PersistEpoch(epoch)).unwrap();
                                    }
                                    Ok(())
                                }
                                Err(err) => Err(err),
                            }
                        });

                    match processing_result {
                        Ok(_) => {
                            block
                                .message()
                                .body()
                                .attestations()
                                .iter()
                                .map(|a| (a.data.slot, a.data.beacon_block_root))
                                .dedup()
                                .filter(|(_, r)| {
                                    !stores
                                        .block_roots_cache()
                                        .write()
                                        .contains(format!("{r:?}"))
                                })
                                .for_each(|(slot, root)| {
                                    info!(%slot, %root, "Unknown root while processing block {}", block.slot());
                                    stores.block_by_root_requests_mut().add(slot, root);


                                    let peers = consensus_network
                                        .peer_db()
                                        .good_peers_iter()
                                        .connected()
                                        .cloned()
                                        .collect::<Vec<_>>();

                                    peers.into_iter().for_each(|peer_id| {
                                        consensus_network.send_block_by_root_request(peer_id, root);
                                    });
                                });
                        }
                        Err(err) => error!("{err:?}"),
                    }
                }
            } else if block_range_requests.request_terminated(&peer_id) {
                // There is no more active range requests
                debug!("Range request succedeed");

                if !stores.block_range_requests().is_requesting() {
                    send_range_request(None, consensus_network, stores);
                }
            }
        }

        LighthouseNetworkEvent::ResponseReceived {
            peer_id,
            id: RequestId::Block(root),
            response: Response::BlocksByRoot(block),
        } => {
            if stores.block_by_root_requests().exists(&root) {
                if let Some(block) = block {
                    let slot = block.slot();

                    if stores
                        .block_by_root_requests_mut()
                        .set_request_as_found(root, peer_id)
                    {
                        info!(found_by = %peer_id, %slot, %root, "An orphaned block has been found");

                        if let Some(attempt) = stores.block_by_root_requests().get(&root) {
                            // Persist the found block request
                            work_send
                                .send(Work::PersistBlockRequest(root, attempt.clone()))
                                .unwrap();
                        }

                        consensus_network.peer_db_mut().add_good_peer(peer_id);

                        // Persist good peers
                        work_send.send(Work::PersistAllGoodPeers).unwrap();
                    }
                } else {
                    stores
                        .block_by_root_requests_mut()
                        .update_attempt(&root, |attempt| {
                            attempt.increment_not_found();
                        });
                }
            }
        }

        _ => {}
    };
}

fn new_blocks<E: EthSpec>(
    block: Arc<SignedBeaconBlock<E>>,
    stores: &Arc<Stores<E>>,
) -> impl Iterator<Item = BlockState<E>> {
    let previous_latest_slot = stores
        .indexing_state()
        .latest_slot()
        .map(|s| s.as_u64() + 1)
        .unwrap_or_default();
    let current_slot = block.message().slot();

    (previous_latest_slot..current_slot.as_u64())
        .map(Slot::new)
        .map(|s| BlockState::Missed(s))
        .chain(once(BlockState::Proposed(block)))
}

fn send_range_request<E: EthSpec>(
    to: Option<PeerId>,
    consensus_network: &mut ConsensusNetwork<E>,
    stores: &Arc<Stores<E>>,
) {
    let start_slot = stores
        .indexing_state()
        .latest_slot()
        .map(|s| s.as_u64() + 1)
        .unwrap_or_default();

    consensus_network.send_range_request(to, start_slot)
}
