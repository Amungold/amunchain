use crate::config::Config;
use amun_networking::tcp_transport::TcpTransport;
use amun_networking::transport_trait::Transport;
use amun_networking::validator_certificate::ValidatorCertificate;
use amun_networking::NetworkNode;
use std::sync::Arc;

pub struct NodeRuntime {
    pub transport: TcpTransport,
}

#[allow(dead_code)]
impl NodeRuntime {
    pub fn new(transport: TcpTransport) -> Self {
        Self { transport }
    }

    pub fn tick(&mut self) {
        self.transport.tick(10);

        // Process incoming messages
        while let Some((_peer, data)) = self.transport.recv_from() {
            // Handle message
            let _ = data;
        }
    }

    pub fn broadcast(&self, data: Vec<u8>) {
        let arc_data: Arc<[u8]> = Arc::from(data.into_boxed_slice());
        self.transport.broadcast(arc_data);
    }
}

#[allow(dead_code)]
// Backward compatibility - delegate to transport
impl NodeRuntime {
    pub fn next_incoming(&mut self) -> Option<bytes::Bytes> {
        self.transport.recv_raw()
    }

    pub fn send(&self, data: Vec<u8>) {
        self.transport.send_raw(data.into());
    }
}

pub fn run(
    transport: TcpTransport,
    _node: NetworkNode,
    _cert: ValidatorCertificate,
    _genesis_hash: [u8; 32],
    _peer_id_bytes: [u8; 32],
    _config: Config,
) {
    let mut runtime = NodeRuntime::new(transport);
    loop {
        runtime.tick();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
