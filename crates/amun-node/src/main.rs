mod peer_registry;
mod peer_handshake;
mod certificate_loader;
mod genesis;

use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use std::str::FromStr;

use serde::Deserialize;

use amun_networking::node::NetworkNode;
use amun_networking::tcp_transport::TcpTransport;
use amun_networking::crypto_identity::PeerKeyPair;
use amun_networking::transport_trait::Transport;

#[derive(Debug, Deserialize)]
struct NodeSection {
    name: String,
    listen_host: String,
    listen_port: u16,
}

#[derive(Debug, Deserialize)]
struct PeersSection {
    seed_peers: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct IdentitySection {
    key_file: String,
}

#[derive(Debug, Deserialize)]
struct GenesisSection {
    file: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    node: NodeSection,
    peers: PeersSection,
    identity: IdentitySection,
    genesis: GenesisSection,
}

fn load_or_create_keypair(key_file: &str) -> PeerKeyPair {
    let path = Path::new(key_file);
    if path.exists() {
        let key_bytes = fs::read(path).expect("Failed to read key file");
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&key_bytes[..32]);
        println!("Loaded existing identity from {}", key_file);
        PeerKeyPair::from_seed(seed)
    } else {
        let keypair = PeerKeyPair::generate();
        let seed = keypair.to_seed();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Failed to create key directory");
        }
        fs::write(path, seed).expect("Failed to write key file");
        println!("Generated new identity and saved to {}", key_file);
        keypair
    }
}

fn main() {
    println!("AmunChain Node v0.1 - Constitutional Validator Node");
    let config_path = std::env::args().nth(1).unwrap_or_else(|| "crates/amun-node/data/config.toml".to_string());
    let config_str = fs::read_to_string(&config_path).expect("Cannot read config");
    let config_dir = std::path::Path::new(&config_path).parent().unwrap_or(std::path::Path::new("."));
    let config: Config = toml::from_str(&config_str).expect("Invalid config");
    println!("Node: {}", config.node.name);
    let key_path = config_dir.join(&config.identity.key_file);
    let keypair = load_or_create_keypair(key_path.to_str().unwrap());
    let peer_id = keypair.peer_id();
    println!("PeerID: {}", hex::encode(peer_id.0));
    let genesis_path = config_dir.join(&config.genesis.file);
    let genesis_str = fs::read_to_string(&genesis_path).unwrap_or_else(|e| panic!("Cannot read genesis at '{}': {}", genesis_path.display(), e));
    let genesis: crate::genesis::Genesis = serde_json::from_str(&genesis_str).expect("Invalid genesis JSON");
    genesis.validate().expect("Genesis validation failed");
    let genesis_hash = genesis.genesis_hash();
    println!("Genesis: {} (hash: {})", genesis_path.display(), hex::encode(genesis_hash));

    let cert_path = config_dir.join("validator.crt");
    let (cert, dev_anchor) = crate::certificate_loader::load_validator_certificate(
        cert_path.to_str().unwrap(), &keypair, &genesis,
    ).expect("Failed to load validator certificate");

    if let Some(ref anchor) = dev_anchor {
        println!("Development mode: using self-signed certificate");
        let anchor_pubkey: [u8; 32] = hex::decode(&anchor.public_key).expect("Invalid anchor public key hex").try_into().expect("Invalid anchor public key length");
        if cert.verify(&anchor_pubkey) {
            println!("Certificate: self-signed (dev mode)");
        } else {
            panic!("Self-signed certificate verification failed");
        }
    } else {
        crate::certificate_loader::verify_certificate_against_genesis(&cert, &genesis)
            .expect("Certificate verification failed");
        println!("Certificate: verified by genesis trust anchor");
    }

    let mut node = NetworkNode::new(peer_id.0);
    node.keypair = Some(keypair);
    let addr = SocketAddr::from_str(&format!("{}:{}", config.node.listen_host, config.node.listen_port)).expect("Invalid listen address");
    let mut transport = TcpTransport::new(addr);
    transport.bind().expect("Failed to bind");
    println!("Listening on {}", addr);
    for peer_addr in &config.peers.seed_peers {
        if let Ok(peer_socket) = SocketAddr::from_str(peer_addr) {
            if peer_socket != addr {
                transport.connect_to(peer_socket);
                println!("Connecting to peer: {}", peer_addr);
            }
        }
    }

    println!("Node ready. Press Ctrl+C to stop.");
    let mut peer_registry = crate::peer_registry::PeerRegistry::new();
    let mut tick_count: u64 = 0;
    println!("Waiting for peer connections...");
    println!();

    loop {
        transport.tick(100);
        tick_count += 1;


        while let Some(envelope) = transport.next_incoming() {
            if envelope.message_type == "handshake" {
                if let Ok(handshake) = serde_json::from_slice::<crate::peer_handshake::HandshakeMessage>(&envelope.payload) {
                    match handshake.verify(&genesis_hash) {
                        Ok(()) => {
                            let peer = crate::peer_handshake::AuthenticatedPeer::from_handshake(&handshake);
                            if peer_registry.register(peer) {
                                println!("✅ Authenticated peer: {} (port {})", handshake.node_name, handshake.listen_port);
                                println!("   Registry size: {}", peer_registry.len());
                            }
                        }
                        Err(e) => eprintln!("❌ Handshake rejected: {}", e),
                    }
                }
            }
        }

        if tick_count.is_multiple_of(50) {
            let handshake = crate::peer_handshake::HandshakeMessage::new(
                node.keypair.as_ref().unwrap(), &cert, genesis_hash, &config.node.name, config.node.listen_port,
            );
            if let Ok(payload) = serde_json::to_vec(&handshake) {
                for peer_addr in &config.peers.seed_peers {
                    let envelope = amun_networking::envelope::Envelope {
                        sender: hex::encode(peer_id.0),
                        recipient: peer_addr.clone(),
                        sequence: tick_count,
                        timestamp: tick_count,
                        message_type: "handshake".into(),
                        payload: payload.clone(),
                    };
                    transport.send(envelope);
                }
            }
            // write_all handled by tick()
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
