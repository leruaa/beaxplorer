use std::sync::Arc;

use lighthouse_types::EthSpec;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{debug, info, instrument, warn};

use crate::{
    db::Stores,
    network::{augmented_network_service::NetworkCommand, event::NetworkEvent},
    work::Work,
};

#[instrument(name = "Network", skip_all)]
pub fn handle<E: EthSpec>(
    network_event: NetworkEvent<E>,
    network_command_send: &UnboundedSender<NetworkCommand>,
    work_send: &UnboundedSender<Work<E>>,
    stores: &Arc<Stores<E>>,
) {
    match network_event {
        NetworkEvent::PeerConnected(peer_id) => {
            if stores.peer_db().is_good_peer(&peer_id) {
                info!(peer = %peer_id, "Good peer connected");
            }

            if !stores.block_range_request_state().is_requesting() {
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
        NetworkEvent::PeerDisconnected(peer_id) => {
            if stores.block_range_request_state().matches(&peer_id) {
                debug!(to = %peer_id, "Range request cancelled");
                work_send.send(Work::SendRangeRequest(None)).unwrap();
            }

            stores
                .block_by_root_requests_mut()
                .pending_iter_mut()
                .for_each(|(_, req)| {
                    req.remove_peer(&peer_id);
                });
        }
        NetworkEvent::RangeRequestSuccedeed => {
            debug!("Range request succedeed");
            work_send.send(Work::SendRangeRequest(None)).unwrap();
        }
        NetworkEvent::RangeRequestFailed(peer_id) => {
            network_command_send
                .send(NetworkCommand::ReportPeer(peer_id, "Range request failed"))
                .unwrap();
            work_send.send(Work::SendRangeRequest(None)).unwrap();
        }
        NetworkEvent::BlockRequestFailed(root, peer_id) => {
            if stores.peer_db().is_good_peer(&peer_id) {
                warn!(peer = %peer_id, "Connection to good peer failed");
            }

            stores
                .block_by_root_requests_mut()
                .update_attempt(&root, |attempt| {
                    attempt.remove_peer(&peer_id);
                });
        }
        NetworkEvent::NewBlock(state, from) => {
            debug!(%state, slot = %state.slot(), %from, "New block");

            if let Some(work) = stores.block_by_epoch_mut().build_epoch(state) {
                work_send.send(work).unwrap();
            }
        }
        NetworkEvent::UnknownBlockRoot(slot, root) => {
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
        }
        NetworkEvent::BlockRootFound(root, slot, found_by) => {
            if stores
                .block_by_root_requests_mut()
                .set_request_as_found(root, found_by)
            {
                info!(%found_by, %slot, %root, "An orphaned block has been found");

                if let Some(attempt) = stores.block_by_root_requests().get(&root) {
                    // Persist the found block request
                    work_send
                        .send(Work::PersistBlockRequest(root, attempt.clone()))
                        .unwrap();
                }

                stores.peer_db_mut().add_good_peer(found_by);

                // Persist good peers
                work_send.send(Work::PersistAllGoodPeers).unwrap();
            }
        }
        NetworkEvent::BlockRootNotFound(root) => {
            stores
                .block_by_root_requests_mut()
                .update_attempt(&root, |attempt| {
                    attempt.increment_not_found();
                });
        }
    }
}
