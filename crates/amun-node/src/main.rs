mod certificate_loader;
mod genesis;
mod peer_handshake;
mod peer_registry;

use amun_networking::node::NetworkNode;
use amun_networking::tcp_transport::TcpTransport;
use amun_networking::transport_trait::Transport;

fn main() {
    println!("AmunChain Node v0.1 (ADR-022)");
    
    let config_path = std::env::args().nth(1).unwrap_or_else(|| "config/node_unified.toml".to_string());
    let amun_config = amun_bootstrap::AmunConfig::from_file(&config_path).expect("Failed to load config");
    let ctx = amun_bootstrap::bootstrap(amun_config).expect("Bootstrap failed");
    
    let mut node = NetworkNode::new(ctx.peer_id.0);
    node.keypair = Some(ctx.keypair);
    
    let addr = ctx.config.listen_addr;
    let mut transport = TcpTransport::new(addr);
    transport.bind().expect("Failed to bind");
    println!("Listening on {}", addr);
    
    for peer_addr in &ctx.config.peer_addresses {
        if *peer_addr != addr { transport.connect_to(*peer_addr); }
    }
    
    let genesis_hash = ctx.genesis.genesis_hash();
    println!("Node ready.");
    
    let mut peer_registry = crate::peer_registry::PeerRegistry::new();
    let mut tick_count: u64 = 0;
    
    loop {
        transport.tick(100);
        tick_count += 1;
        while let Some(envelope) = transport.next_incoming() {
            if envelope.message_type == "handshake" {
                if let Ok(hs) = serde_json::from_slice::<crate::peer_handshake::HandshakeMessage>(&envelope.payload) {
                    match hs.verify(&genesis_hash) {
                        Ok(()) => { peer_registry.register(crate::peer_handshake::AuthenticatedPeer::from_handshake(&hs)); }
                        Err(e) => eprintln!("Handshake rejected: {}", e),
                    }
                }
            }
        }
        if tick_count % 50 == 0 {
            if let Some(ref kp) = node.keypair {
                if let Some(ref cert) = ctx.certificate {
                    let hs = crate::peer_handshake::HandshakeMessage::new(kp, cert, genesis_hash, "amun-node", ctx.config.listen_addr.port());
                    if let Ok(payload) = serde_json::to_vec(&hs) {
                        for peer_addr in &ctx.config.peer_addresses {
                            transport.send(amun_networking::envelope::Envelope {
                                sender: hex::encode(kp.peer_id().0), recipient: peer_addr.to_string(),
                                sequence: tick_count, timestamp: tick_count,
                                message_type: "handshake".into(), payload: payload.clone(),
                            });
                        }
                    }
                }
            }
        }
    }
}
