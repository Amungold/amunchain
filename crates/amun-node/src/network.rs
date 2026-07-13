use amun_networking::handshake::ConstitutionInfo;
use amun_networking::tcp_transport::TcpTransport;
use amun_networking::transport_trait::Transport;
use std::net::SocketAddr;

pub struct NodeNetwork {
    pub transport: TcpTransport,
}

impl NodeNetwork {
    /// Create a NodeNetwork with real identity from bootstrap.
    /// No more fake keys, fake genesis, or fake addresses.
    pub fn new(
        addr: SocketAddr,
        signing_key: ed25519_dalek::SigningKey,
        node_id: [u8; 32],
        network_id: [u8; 32],
        genesis_hash: [u8; 32],
        constitution: ConstitutionInfo,
    ) -> Result<Self, String> {
        let mut transport = TcpTransport::new(
            addr,
            network_id,
            genesis_hash,
            node_id,
            signing_key,
            constitution,
        );
        Transport::bind(&mut transport).map_err(|e| format!("Bind error: {}", e))?;
        Ok(NodeNetwork { transport })
    }

    #[allow(dead_code)]
    pub fn connect_to(&self, peer: SocketAddr) {
        Transport::connect_to(&self.transport, peer);
    }
}
