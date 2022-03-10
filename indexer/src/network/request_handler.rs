use std::{
    collections::HashMap,
    pin::{self, Pin},
    sync::Arc,
    task::{Context, Poll},
};

use futures::{Stream, StreamExt};
use libp2p::PeerId;
use lighthouse_network::{rpc::outbound::OutboundRequest, Request};
use pin_project::pin_project;
use store::MainnetEthSpec;
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    Mutex, MutexGuard,
};
use tokio_stream::StreamMap;

type RequestStream = Pin<Box<dyn Stream<Item = OutboundRequest<MainnetEthSpec>> + Send>>;
type RequestStreamMap = StreamMap<PeerId, RequestStream>;

#[pin_project]
pub struct RequestHandler {
    pending_channels: HashMap<PeerId, RequestStream>,
    #[pin]
    streams: RequestStreamMap,
}

impl RequestHandler {
    pub fn new() -> Self {
        RequestHandler {
            pending_channels: HashMap::new(),
            streams: StreamMap::new(),
        }
    }

    pub fn create_channel(
        &mut self,
        peer_id: PeerId,
    ) -> Result<UnboundedSender<OutboundRequest<MainnetEthSpec>>, String> {
        if self.pending_channels.contains_key(&peer_id) {
            return Err("A channel has already been created for this peer".to_string());
        }

        let (tx, mut rx) = mpsc::unbounded_channel::<OutboundRequest<MainnetEthSpec>>();

        let rx = Box::pin(async_stream::stream! {
              while let Some(item) = rx.recv().await {
                  yield item;
              }
        }) as RequestStream;

        self.pending_channels.insert(peer_id, rx);

        Ok(tx)
    }

    pub fn close_channel(&mut self, peer_id: PeerId) {
        self.pending_channels.remove(&peer_id);
    }

    pub fn activate(&mut self, peer_id: PeerId) -> Result<(), String> {
        let rx = self
            .pending_channels
            .remove(&peer_id)
            .ok_or("Trying to activate a non existant channel")?;
        self.streams.insert(peer_id, rx);

        Ok(())
    }
}

impl Stream for RequestHandler {
    type Item = (PeerId, OutboundRequest<MainnetEthSpec>);

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let next = this.streams.poll_next(cx);

        if next.is_pending() {
            Poll::Ready(None)
        } else {
            next
        }
    }
}

pub struct SafeRequestHandler(Arc<Mutex<RequestHandler>>);

impl SafeRequestHandler {
    pub fn new() -> Self {
        SafeRequestHandler(Arc::new(Mutex::new(RequestHandler::new())))
    }

    pub async fn activate(&mut self, peer_id: PeerId) -> Result<(), String> {
        self.0.lock().await.activate(peer_id)
    }

    pub async fn close_channel(&mut self, peer_id: PeerId) {
        self.0.lock().await.close_channel(peer_id)
    }

    pub async fn next(&mut self) -> Option<(PeerId, OutboundRequest<MainnetEthSpec>)> {
        self.0.lock().await.next().await
    }

    pub async fn guard(&self) -> MutexGuard<'_, RequestHandler> {
        self.0.lock().await
    }
}

impl Clone for SafeRequestHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
