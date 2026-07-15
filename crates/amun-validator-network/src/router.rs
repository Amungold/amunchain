use crate::message::NetworkMessage;
use crate::peer::PeerTable;
use crate::transport::TransportProvider;
use amun_validator_api::error::PlatformResult;
use amun_validator_api::types::id::PeerId;
use std::sync::Arc;

pub struct Router {
    peer_table: Arc<PeerTable>,
    transport: Arc<dyn TransportProvider>,
}

impl Router {
    pub fn new(peer_table: Arc<PeerTable>, transport: Arc<dyn TransportProvider>) -> Self {
        Router {
            peer_table,
            transport,
        }
    }

    pub fn route_to_peer(&self, peer_id: &PeerId, message: &NetworkMessage) -> PlatformResult<()> {
        if let Some(peer) = self.peer_table.get(peer_id) {
            if peer.connected && !peer.is_banned() {
                self.transport.send(&peer.address, message)?;
            }
        }
        Ok(())
    }

    pub fn broadcast(&self, message: &NetworkMessage) -> PlatformResult<()> {
        for peer in self.peer_table.connected_peers() {
            let _ = self.transport.send(&peer.address, message);
        }
        Ok(())
    }
}
