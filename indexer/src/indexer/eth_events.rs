use std::{iter::once, sync::Arc};

use itertools::Itertools;
use lighthouse_network::{NetworkEvent as LighthouseNetworkEvent, Response};
use lighthouse_types::{EthSpec, SignedBeaconBlock, Slot};
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, error, info, warn};
use types::block_root::{BlockRootModel, BlockRootModelWithId};

use crate::{
    db::Stores,
    network::consensus_service::{NetworkCommand, RequestId},
    types::block_state::BlockState,
    work::Work,
};

pub fn handle<E: EthSpec>(
    network_event: LighthouseNetworkEvent<RequestId, E>,
    network_command_send: &UnboundedSender<NetworkCommand>,
    work_send: &UnboundedSender<Work<E>>,
    stores: &Arc<Stores<E>>,
) {
    match network_event {
        LighthouseNetworkEvent::PeerConnectedOutgoing(peer_id) => {
            if stores.peer_db().is_good_peer(&peer_id) {
                info!(peer = %peer_id, "Good peer connected");
            }

            if !stores.block_range_requests().is_requesting() {
                work_send
                    .send(Work::SendRangeRequest(Some(peer_id)))
                    .unwrap();
            }

            stores
                .block_by_root_requests_mut()
                .pending_iter_mut()
                .for_each(|(root, req)| {
                    if req.insert_peer(&peer_id) {
                        work_send
                            .send(Work::SendBlockByRootRequest(*root, peer_id))
                            .unwrap();
                    }
                });
        }

        LighthouseNetworkEvent::PeerDisconnected(peer_id) => {
            let mut block_range_requests = stores.block_range_requests_mut();

            if block_range_requests.request_terminated(&peer_id) {
                debug!(to = %peer_id, "Range request cancelled");
                if !block_range_requests.is_requesting() {
                    work_send.send(Work::SendRangeRequest(None)).unwrap();
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
            network_command_send
                .send(NetworkCommand::ReportPeer(peer_id, "Range request failed"))
                .unwrap();
            work_send.send(Work::SendRangeRequest(None)).unwrap();
        }

        LighthouseNetworkEvent::RPCFailed {
            id: RequestId::Block(root),
            peer_id,
        } => {
            if stores.peer_db().is_good_peer(&peer_id) {
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

                                    stores
                                        .peer_db()
                                        .good_peers_iter()
                                        .connected()
                                        .for_each(|peer_id| {
                                            work_send
                                                .send(Work::SendBlockByRootRequest(root, *peer_id))
                                                .unwrap();
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
                    work_send.send(Work::SendRangeRequest(None)).unwrap();
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

                        stores.peer_db_mut().add_good_peer(peer_id);

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

#[cfg(test)]
mod tests {
    use lighthouse_network::{NetworkEvent as LighthouseNetworkEvent, PeerId, Response};
    use lighthouse_types::{EthSpec, SignedBeaconBlock, Slot};
    use std::sync::Arc;
    use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

    use crate::{
        db::Stores,
        network::consensus_service::{NetworkCommand, RequestId},
        test_utils::{build_stores, BeaconChainHarness},
        work::Work,
    };

    use super::handle;

    fn handle_new_block<E: EthSpec>(
        peer: PeerId,
        block: SignedBeaconBlock<E>,
        network_command_send: &UnboundedSender<NetworkCommand>,
        work_send: &UnboundedSender<Work<E>>,
        stores: &Arc<Stores<E>>,
    ) {
        handle(
            LighthouseNetworkEvent::ResponseReceived {
                peer_id: peer,
                id: RequestId::Range,
                response: Response::BlocksByRange(Some(Arc::new(block))),
            },
            network_command_send,
            work_send,
            stores,
        );
    }

    #[tokio::test]
    async fn test_range_request_nonce() {
        let mut harness = BeaconChainHarness::new();

        let stores = build_stores(harness.spec());
        let (network_event_send, mut network_event_recv) = unbounded_channel();
        let (work_send, work_recv) = unbounded_channel();

        let peer1 = PeerId::random();
        let peer2 = PeerId::random();
        let block = harness.make_block(0).await;

        handle_new_block(
            peer1,
            block.clone(),
            &network_event_send,
            &work_send,
            &stores,
        );
        handle_new_block(
            peer2,
            block.clone(),
            &network_event_send,
            &work_send,
            &stores,
        );

        network_event_recv.close();

        let ev1 = network_event_recv.recv().await;
        let ev2 = network_event_recv.recv().await;

        //assert!(matches!(ev1.unwrap(), NetworkEvent::NewBlock(_, p) if p == peer1));
        assert_eq!(stores.indexing_state().latest_slot(), Some(Slot::new(0)));
        assert!(ev2.is_none());
    }

    #[tokio::test]
    async fn test_range_request_failed() {}
}
