use crate::message::{MessageType, NetworkMessage};
use crate::peer::PeerTable;
use crate::transport::TransportProvider;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct HeartbeatScheduler {
    peer_table: Arc<PeerTable>,
    transport: Arc<dyn TransportProvider>,
    running: AtomicBool,
}

impl HeartbeatScheduler {
    pub fn new(peer_table: Arc<PeerTable>, transport: Arc<dyn TransportProvider>) -> Self {
        HeartbeatScheduler {
            peer_table,
            transport,
            running: AtomicBool::new(false),
        }
    }
    pub fn start(&self) {
        self.running.store(true, Ordering::SeqCst);
    }
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn tick(&self) {
        if !self.running.load(Ordering::SeqCst) {
            return;
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        for peer in self.peer_table.connected_peers() {
            let msg = NetworkMessage::new(MessageType::Ping, *peer.peer_id.as_bytes(), vec![]);
            let _ = self.transport.send(&peer.address, &msg);
            self.peer_table
                .update(&peer.peer_id, |info| info.last_ping = now);
        }
    }
}
