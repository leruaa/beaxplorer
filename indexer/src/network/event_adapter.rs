use std::{
    collections::HashSet,
    iter::once,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use futures::Stream;
use itertools::Itertools;
use lighthouse_network::{NetworkEvent as LighthouseNetworkEvent, Response};
use lighthouse_types::{EthSpec, Hash256, SignedBeaconBlock, Slot};
use tokio::sync::{
    mpsc::UnboundedReceiver,
    mpsc::{unbounded_channel, UnboundedSender},
};

use super::{augmented_network_service::RequestId, event::NetworkEvent};

pub struct EventAdapter<E: EthSpec> {
    network_event_send: UnboundedSender<NetworkEvent<E>>,
    network_event_recv: UnboundedReceiver<NetworkEvent<E>>,
    latest_slot: Option<Slot>,
    proposed_block_roots: HashSet<Hash256>,
}

impl<E: EthSpec> EventAdapter<E> {
    pub fn new() -> Self {
        let (network_event_send, network_event_recv) = unbounded_channel();

        Self {
            network_event_send,
            network_event_recv,
            latest_slot: None,
            proposed_block_roots: HashSet::new(),
        }
    }

    pub fn handle(&mut self, network_event: LighthouseNetworkEvent<RequestId, E>) {
        match network_event {
            LighthouseNetworkEvent::PeerConnectedOutgoing(peer_id) => self
                .network_event_send
                .send(NetworkEvent::PeerConnected(peer_id))
                .unwrap(),

            LighthouseNetworkEvent::PeerDisconnected(peer_id) => self
                .network_event_send
                .send(NetworkEvent::PeerDisconnected(peer_id))
                .unwrap(),

            LighthouseNetworkEvent::RPCFailed {
                id: RequestId::Range(id),
                ..
            } => self
                .network_event_send
                .send(NetworkEvent::RangeRequestFailed(id))
                .unwrap(),

            LighthouseNetworkEvent::RPCFailed {
                id: RequestId::Block(root),
                peer_id,
            } => self
                .network_event_send
                .send(NetworkEvent::BlockRequestFailed(root, peer_id))
                .unwrap(),

            LighthouseNetworkEvent::ResponseReceived {
                id: RequestId::Range(start_slot),
                response: Response::BlocksByRange(block),
                ..
            } => {
                if let Some(block) = block {
                    self.proposed_block_roots.insert(block.canonical_root());

                    block
                        .message()
                        .body()
                        .attestations()
                        .iter()
                        .map(|a| (a.data.slot, a.data.beacon_block_root))
                        .dedup()
                        .filter(|(_, r)| !self.proposed_block_roots.contains(r))
                        .for_each(|(slot, root)| {
                            self.network_event_send
                                .send(NetworkEvent::UnknownBlockRoot(slot, root))
                                .unwrap()
                        });

                    self.new_blocks(block)
                        .for_each(|event| self.network_event_send.send(event).unwrap())
                } else {
                    // A block range response has finished
                    self.network_event_send
                        .send(NetworkEvent::RangeRequestSuccedeed(start_slot))
                        .unwrap()
                }
            }

            _ => {}
        };
    }

    fn new_blocks(
        &mut self,
        block: Arc<SignedBeaconBlock<E>>,
    ) -> impl Iterator<Item = NetworkEvent<E>> {
        let previous_latest_slot = self.latest_slot.unwrap_or_else(|| Slot::new(0));
        self.latest_slot = Some(block.message().slot());

        (previous_latest_slot.as_u64()..self.latest_slot.unwrap_or_else(|| Slot::new(0)).as_u64())
            .map(Slot::new)
            .map(NetworkEvent::MissedBlock)
            .chain(once(NetworkEvent::ProposedBlock(block)))
    }
}

impl<E: EthSpec> Stream for EventAdapter<E> {
    type Item = NetworkEvent<E>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.get_mut().network_event_recv.poll_recv(cx)
    }
}
