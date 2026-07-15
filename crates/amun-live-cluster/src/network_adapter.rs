use amun_networking::frame::{FrameKind, NetworkFrame};
use amun_networking::tcp_transport::TcpTransport;
use amun_networking::transport_trait::Transport;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ValidatorNetworkAdapter {
    transport: Arc<Mutex<TcpTransport>>,
}

impl ValidatorNetworkAdapter {
    pub fn new(transport: Arc<Mutex<TcpTransport>>) -> Self {
        Self { transport }
    }

    pub fn poll(&self) {
        let mut t = self.transport.lock().expect("mutex poisoned");
        t.tick(10);
    }

    pub fn broadcast_vote(&self, vote: Vec<u8>) {
        let frame = NetworkFrame::new(FrameKind::Vote, vote.into());
        let bytes = postcard::to_stdvec(&frame).unwrap();
        let data: Arc<[u8]> = Arc::from(bytes.into_boxed_slice());
        let t = self.transport.lock().expect("mutex poisoned");
        t.broadcast(data);
    }

    pub fn send_to(&self, peer: SocketAddr, frame: NetworkFrame) -> Result<(), String> {
        let bytes = postcard::to_stdvec(&frame).map_err(|e| format!("encode: {}", e))?;
        let data: Arc<[u8]> = Arc::from(bytes.into_boxed_slice());
        let t = self.transport.lock().expect("mutex poisoned");
        t.send_to(peer, data)
    }

    #[allow(unused_mut)]
    pub fn recv_from(&self) -> Option<(SocketAddr, NetworkFrame)> {
        let mut t = self.transport.lock().expect("mutex poisoned");
        if let Some((addr, data)) = t.recv_from() {
            if let Ok(frame) = postcard::from_bytes::<NetworkFrame>(&data) {
                return Some((addr, frame));
            }
        }
        None
    }

    pub fn next_request_id(&self) -> u64 {
        let t = self.transport.lock().expect("mutex poisoned");
        t.next_request_id()
    }
}
