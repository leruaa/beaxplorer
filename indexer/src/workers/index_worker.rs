use std::{
    collections::{BTreeMap, HashSet},
    iter::once,
    sync::Arc,
};

use itertools::Itertools;
use lighthouse_network::{NetworkEvent, PeerId, Response};
use lighthouse_types::{EthSpec, SignedBeaconBlock, Slot};
use parking_lot::RwLock;
use task_executor::TaskExecutor;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tracing::{debug, error, info};
use types::block_root::{BlockRootModel, BlockRootModelWithId};

use crate::{
    db::Stores,
    network::{NetworkCommand, RequestId},
    types::block_state::BlockState,
    work::Work,
};

pub fn spawn_index_worker<E: EthSpec>(
    network_event_recv: UnboundedReceiver<NetworkEvent<RequestId, E>>,
    network_command_send: UnboundedSender<NetworkCommand>,
    stores: Arc<Stores<E>>,
    executor: &TaskExecutor,
) -> UnboundedReceiver<Work<E>> {
    let (work_send, work_recv) = mpsc::unbounded_channel();

    IndexWorker::new(network_event_recv, network_command_send, work_send, stores).spawn(executor);

    work_recv
}

struct IndexWorker<E: EthSpec> {
    network_event_recv: UnboundedReceiver<NetworkEvent<RequestId, E>>,
    network_command_send: UnboundedSender<NetworkCommand>,
    work_send: UnboundedSender<Work<E>>,
    block_range_requests: RwLock<BlockRangeRequests<E>>,
    stores: Arc<Stores<E>>
}

impl<E: EthSpec> IndexWorker<E> {
    pub fn new(
        network_event_recv: UnboundedReceiver<NetworkEvent<RequestId, E>>,
        network_command_send: UnboundedSender<NetworkCommand>,
        work_send: UnboundedSender<Work<E>>,
        stores: Arc<Stores<E>>
    ) -> Self {
        Self {
            network_event_recv,
            network_command_send,
            work_send,
            block_range_requests: RwLock::new(BlockRangeRequests::default()),
            stores,
        }
    }

    pub fn spawn(mut self, executor: &TaskExecutor) {
        executor.spawn(
            async move {
                loop {
                    match self.network_event_recv.recv().await {
                        Some(event) => {
                            if let Err(err) = self.handle_next_event(event) {
                                match err {
                                    IndexError::SendMessage => {
                                        info!("Shutting down index worker");
                                        return;
                                    },
                                    IndexError::BlockProcessing(err) => error!("{err}"),
                                }
                            }
                        },
                        None => {
                            info!("Shutting down index worker");
                            return;
                        },
                    }
                }
            },
            "index worker",
        );
    }

    fn handle_next_event(&self, event: NetworkEvent<RequestId, E>) -> Result<(), IndexError> {
        match event {
            NetworkEvent::PeerConnectedOutgoing(peer_id) => {
                if !self.block_range_requests.read().is_requesting() {
                    self.send_range_request(Some(peer_id))?;
                }
    
                self.stores.block_by_root_requests_mut()
                    .pending_iter_mut()
                    .try_for_each(|(root, req)| {
                        if req.insert_peer(&peer_id) {
                            self
                                .network_command_send
                                .send(NetworkCommand::SendBlockByRootRequest { peer_id: Some(peer_id), root: *root })
                                .map_err(|_| IndexError::SendMessage)
                        }
                        else {
                            Ok(())
                        }
                    })?;
            }
    
            NetworkEvent::PeerDisconnected(peer_id) => {
                let mut block_range_requests = self.block_range_requests.write();
                if block_range_requests.request_terminated(&peer_id) {
                    debug!(to = %peer_id, "Range request cancelled");
                    if !block_range_requests.is_requesting() {
                        self.send_range_request(None)?;
                    }
                }
    
                self.stores
                    .block_by_root_requests_mut()
                    .pending_iter_mut()
                    .for_each(|(_, req)| {
                        req.remove_peer(&peer_id);
                    });
            }
    
            NetworkEvent::RPCFailed {
                id: RequestId::Range,
                ..
            } => {
                self.send_range_request(None)?;
            }
    
            NetworkEvent::RPCFailed {
                id: RequestId::Block(root),
                peer_id,
            } => {   
                self.stores
                    .block_by_root_requests_mut()
                    .update_attempt(&root, |attempt| {
                        attempt.remove_peer(&peer_id);
                    });
            }
    
            NetworkEvent::ResponseReceived {
                id: RequestId::Range,
                response: Response::BlocksByRange(block),
                peer_id,
            } => {
                let mut block_range_requests = self.block_range_requests.write();

                if let Some(block) = block {
                    self.stores
                        .block_roots_cache()
                        .write()
                        .put(BlockRootModelWithId {
                            id: format!("{:?}", block.canonical_root()),
                            model: BlockRootModel {
                                slot: block.slot().as_u64(),
                            },
                        });
    
                    let block = block_range_requests.next_or(block);
    
                    if Some(block.slot()) > self.stores.indexing_state().latest_slot() {
                        new_blocks(block.clone(), &self.stores).try_for_each(|block| {
                            match self.stores.indexing_state_mut().process_block(block) {
                                Ok((block, epoch)) => {
                                    self.work_send
                                        .send(Work::PersistBlock(block))
                                        .map_err(|_| IndexError::SendMessage)?;
    
                                    if let Some(epoch) = epoch {
                                        self.work_send
                                            .send(Work::PersistEpoch(epoch))
                                            .map_err(|_| IndexError::SendMessage)?;
                                    }
                                    Ok(())
                                },
                                Err(err) => Err(IndexError::BlockProcessing(err)),
                            }
                        })?;
    
                        block
                            .message()
                            .body()
                            .attestations()
                            .iter()
                            .map(|a| (a.data.slot, a.data.beacon_block_root))
                            .dedup()
                            .filter(|(_, r)| {
                                !self.stores
                                    .block_roots_cache()
                                    .write()
                                    .contains(format!("{r:?}"))
                            })
                            .try_for_each(|(slot, root)| {
                                info!(%slot, %root, "Unknown root while processing block {}", block.slot());
                                self.stores.block_by_root_requests_mut().add(slot, root);
    
                                self
                                    .network_command_send
                                    .send(NetworkCommand::SendBlockByRootRequest { peer_id: None, root })
                                    .map_err(|_| IndexError::SendMessage)
                            })?;
                    }
                } else if block_range_requests.request_terminated(&peer_id) {
                    // There is no more active range requests
                    debug!("Range request succedeed");
    
                    if !block_range_requests.is_requesting() {
                        self.send_range_request(None)?;
                    }
                }
            }
    
            NetworkEvent::ResponseReceived {
                peer_id,
                id: RequestId::Block(root),
                response: Response::BlocksByRoot(block),
            } => {
                let mut block_by_root_requests = self.stores.block_by_root_requests_mut();

                if block_by_root_requests.exists(&root) {
                    if let Some(block) = block {
                        let slot = block.slot();
    
                        if block_by_root_requests
                            .set_request_as_found(root, peer_id)
                        {
                            info!(found_by = %peer_id, %slot, %root, "An orphaned block has been found");
    
                            if let Some(attempt) = block_by_root_requests.get(&root) {
                                // Persist the found block request
                                self.work_send
                                    .send(Work::PersistBlockRequest(root, attempt.clone()))
                                    .map_err(|_| IndexError::SendMessage)?;
                            }
    
                            //consensus_network.peer_db_mut().add_good_peer(peer_id);
    
                            // Persist good peers
                            self.work_send
                                .send(Work::PersistAllGoodPeers)
                                .map_err(|_| IndexError::SendMessage)?;
                        }
                    } else {
                        block_by_root_requests
                            .update_attempt(&root, |attempt| {
                                attempt.increment_not_found();
                            });
                    }
                }
            }
    
            _ => {}
        };
    
        Ok(())
    }

    fn send_range_request(
        &self,
        to: Option<PeerId>,
    ) -> Result<(), IndexError> {
        let start_slot = self.stores
            .indexing_state()
            .latest_slot()
            .map(|s| s.as_u64() + 1)
            .unwrap_or_default();
    
        self
            .network_command_send
            .send(NetworkCommand::SendRangeRequest { peer_id: to, start_slot })
            .map_err(|_| IndexError::SendMessage)
    }
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

enum IndexError {
    SendMessage,
    BlockProcessing(String),
}


#[derive(Debug, Default)]
pub struct BlockRangeRequests<E: EthSpec> {
    active_requests: HashSet<PeerId>,
    blocks_queue: BTreeMap<Slot, Arc<SignedBeaconBlock<E>>>,
}

impl<E: EthSpec> BlockRangeRequests<E> {
    pub fn next_or(&mut self, block: Arc<SignedBeaconBlock<E>>) -> Arc<SignedBeaconBlock<E>> {
        self.blocks_queue.insert(block.slot(), block);

        self.blocks_queue
            .pop_first()
            .map(|(_, b)| b)
            .expect("should never happen")
    }

    pub fn request_terminated(&mut self, peer_id: &PeerId) -> bool {
        self.active_requests.remove(peer_id)
    }

    pub fn is_requesting(&self) -> bool {
        !self.active_requests.is_empty()
    }
}