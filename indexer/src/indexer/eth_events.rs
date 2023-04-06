use std::{iter::once, sync::Arc};

use itertools::Itertools;
use lighthouse_network::{NetworkEvent as LighthouseNetworkEvent, PeerId, Response};
use lighthouse_types::{EthSpec, SignedBeaconBlock, Slot};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    db::Stores,
    network::{augmented_network_service::RequestId, event::NetworkEvent},
    types::block_state::BlockState,
};

pub fn handle<E: EthSpec>(
    network_event: LighthouseNetworkEvent<RequestId, E>,
    network_event_send: &UnboundedSender<NetworkEvent<E>>,
    stores: &Arc<Stores<E>>,
) {
    match network_event {
        LighthouseNetworkEvent::PeerConnectedOutgoing(peer_id) => {
            network_event_send
                .send(NetworkEvent::PeerConnected(peer_id))
                .unwrap();
        }

        LighthouseNetworkEvent::PeerDisconnected(peer_id) => {
            network_event_send
                .send(NetworkEvent::PeerDisconnected(peer_id))
                .unwrap();
        }

        LighthouseNetworkEvent::RPCFailed {
            id: RequestId::Range(_),
            peer_id,
        } => {
            network_event_send
                .send(NetworkEvent::RangeRequestFailed(peer_id))
                .unwrap();
        }

        LighthouseNetworkEvent::RPCFailed {
            id: RequestId::Block(root),
            peer_id,
        } => {
            network_event_send
                .send(NetworkEvent::BlockRequestFailed(root, peer_id))
                .unwrap();
        }

        LighthouseNetworkEvent::ResponseReceived {
            id: RequestId::Range(nonce),
            response: Response::BlocksByRange(block),
            peer_id,
        } => {
            if let Some(block) = block {
                if stores
                    .block_range_request_mut()
                    .request_peer_if_possible(nonce, peer_id)
                {
                    let slot = block.slot();
                    let root = block.canonical_root();

                    stores.proposed_block_roots_mut().insert(root);
                    stores.block_by_root_requests_mut().remove(&root);

                    block
                        .message()
                        .body()
                        .attestations()
                        .iter()
                        .map(|a| (a.data.slot, a.data.beacon_block_root))
                        .dedup()
                        .filter(|(_, r)| !stores.proposed_block_roots().contains(r))
                        .for_each(|(slot, root)| {
                            network_event_send
                                .send(NetworkEvent::UnknownBlockRoot(slot, root))
                                .unwrap();
                        });

                    new_blocks(block, peer_id, stores).for_each(|event| {
                        network_event_send.send(event).unwrap();
                    });

                    stores.latest_slot_mut().replace(slot);
                }
            } else if stores.block_range_request().matches_nonce(nonce) {
                // A block range response has finished
                network_event_send
                    .send(NetworkEvent::RangeRequestSuccedeed)
                    .unwrap();
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
                    network_event_send
                        .send(NetworkEvent::NewBlock(BlockState::Orphaned(block), peer_id))
                        .unwrap();

                    network_event_send
                        .send(NetworkEvent::BlockRootFound(root, slot, peer_id))
                        .unwrap();
                } else {
                    network_event_send
                        .send(NetworkEvent::BlockRootNotFound(root))
                        .unwrap();
                }
            }
        }

        _ => {}
    };
}

fn new_blocks<E: EthSpec>(
    block: Arc<SignedBeaconBlock<E>>,
    from: PeerId,
    stores: &Arc<Stores<E>>,
) -> impl Iterator<Item = NetworkEvent<E>> {
    let previous_latest_slot = stores
        .latest_slot()
        .map(|s| s.as_u64() + 1)
        .unwrap_or_default();
    let latest_slot = block.message().slot();

    (previous_latest_slot..latest_slot.as_u64())
        .map(Slot::new)
        .map(move |s| NetworkEvent::NewBlock(BlockState::Missed(s), from))
        .chain(once(NetworkEvent::NewBlock(
            BlockState::Proposed(block),
            from,
        )))
}

#[cfg(test)]
mod tests {
    use lighthouse_network::{NetworkEvent as LighthouseNetworkEvent, PeerId, Response};
    use lighthouse_types::{EthSpec, MainnetEthSpec, SignedBeaconBlock};
    use std::sync::Arc;
    use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

    use crate::{
        db::Stores,
        indexer::test_utils::{build_stores, BeaconChainHarness},
        network::{augmented_network_service::RequestId, event::NetworkEvent},
    };

    use super::handle;

    fn handle_new_block<E: EthSpec>(
        peer: PeerId,
        nonce: u64,
        block: SignedBeaconBlock<E>,
        network_event_send: &UnboundedSender<NetworkEvent<E>>,
        stores: &Arc<Stores<E>>,
    ) {
        handle(
            LighthouseNetworkEvent::ResponseReceived {
                peer_id: peer,
                id: RequestId::Range(nonce),
                response: Response::BlocksByRange(Some(Arc::new(block))),
            },
            network_event_send,
            stores,
        );
    }

    #[tokio::test]
    async fn test_range_request_nonce() {
        let stores = build_stores::<MainnetEthSpec>();
        let (network_event_send, mut network_event_recv) = unbounded_channel();

        let mut harness = BeaconChainHarness::new();

        let peer1 = PeerId::random();
        let peer2 = PeerId::random();
        let block = harness.make_block(0).await;

        handle_new_block(peer1, 1, block.clone(), &network_event_send, &stores);
        handle_new_block(peer2, 2, block.clone(), &network_event_send, &stores);

        network_event_recv.close();

        let ev1 = network_event_recv.recv().await;
        let ev2 = network_event_recv.recv().await;

        assert!(*stores.latest_slot() == 0_u64);
        assert!(stores.block_range_request().matches_nonce(1));
        assert!(stores.block_range_request().matches_peer(peer1));
        assert!(matches!(ev1.unwrap(), NetworkEvent::NewBlock(_, p) if p == peer1));
        assert!(ev2.is_none());
    }
}
