use std::{iter::once, sync::Arc};

use itertools::Itertools;
use lighthouse_network::{NetworkEvent as LighthouseNetworkEvent, Response};
use lighthouse_types::{EthSpec, SignedBeaconBlock, Slot};
use tokio::sync::mpsc::UnboundedSender;

use crate::{db::Stores, types::block_state::BlockState};

use super::{augmented_network_service::RequestId, event::NetworkEvent};

pub struct EventAdapter<E: EthSpec> {
    network_event_send: UnboundedSender<NetworkEvent<E>>,
    stores: Arc<Stores<E>>,
}

impl<E: EthSpec> EventAdapter<E> {
    pub fn new(
        network_event_send: UnboundedSender<NetworkEvent<E>>,
        stores: Arc<Stores<E>>,
    ) -> Self {
        Self {
            network_event_send,
            stores,
        }
    }

    pub fn handle(&mut self, network_event: LighthouseNetworkEvent<RequestId, E>) {
        match network_event {
            LighthouseNetworkEvent::PeerConnectedOutgoing(peer_id) => {
                self.network_event_send
                    .send(NetworkEvent::PeerConnected(peer_id))
                    .unwrap();
            }

            LighthouseNetworkEvent::PeerDisconnected(peer_id) => {
                self.network_event_send
                    .send(NetworkEvent::PeerDisconnected(peer_id))
                    .unwrap();
            }

            LighthouseNetworkEvent::RPCFailed {
                id: RequestId::Range(_),
                ..
            } => {
                self.network_event_send
                    .send(NetworkEvent::RangeRequestFailed)
                    .unwrap();
            }

            LighthouseNetworkEvent::RPCFailed {
                id: RequestId::Block(root),
                peer_id,
            } => {
                self.network_event_send
                    .send(NetworkEvent::BlockRequestFailed(root, peer_id))
                    .unwrap();
            }

            LighthouseNetworkEvent::ResponseReceived {
                id: RequestId::Range(start_slot),
                response: Response::BlocksByRange(block),
                ..
            } => {
                if let Some(block) = block {
                    block
                        .message()
                        .body()
                        .attestations()
                        .iter()
                        .map(|a| (a.data.slot, a.data.beacon_block_root))
                        .dedup()
                        .filter(|(_, r)| !self.stores.proposed_block_roots().contains(r))
                        .for_each(|(slot, root)| {
                            self.network_event_send
                                .send(NetworkEvent::UnknownBlockRoot(slot, root))
                                .unwrap();
                        });

                    self.new_blocks(block).for_each(|event| {
                        self.network_event_send.send(event).unwrap();
                    });

                    self.stores.latest_slot_mut().replace(slot);
                } else {
                    // A block range response has finished
                    self.network_event_send
                        .send(NetworkEvent::RangeRequestSuccedeed)
                        .unwrap();
                }
            }

            LighthouseNetworkEvent::ResponseReceived {
                peer_id,
                id: RequestId::Block(root),
                response: Response::BlocksByRoot(block),
            } => {
                if self.stores.block_by_root_requests().exists(&root) {
                    if let Some(block) = block {
                        let slot = block.slot();
                        self.network_event_send
                            .send(NetworkEvent::NewBlock(BlockState::Orphaned(block)))
                            .unwrap();

                        self.network_event_send
                            .send(NetworkEvent::BlockRootFound(root, slot, peer_id))
                            .unwrap();
                    } else {
                        self.network_event_send
                            .send(NetworkEvent::BlockRootNotFound(root))
                            .unwrap();
                    }
                }
            }

            _ => {}
        };
    }

    fn new_blocks(
        &mut self,
        block: Arc<SignedBeaconBlock<E>>,
    ) -> impl Iterator<Item = NetworkEvent<E>> {
        let previous_latest_slot = self
            .stores
            .latest_slot()
            .map(|s| s.as_u64() + 1)
            .unwrap_or_default();
        let latest_slot = block.message().slot();

        (previous_latest_slot..latest_slot.as_u64())
            .map(Slot::new)
            .map(|s| NetworkEvent::NewBlock(BlockState::Missed(s)))
            .chain(once(NetworkEvent::NewBlock(BlockState::Proposed(block))))
    }
}
