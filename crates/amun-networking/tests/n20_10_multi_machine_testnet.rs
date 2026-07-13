use amun_networking::handshake::ConstitutionInfo;
use amun_networking::tcp_transport::TcpTransport;
use amun_networking::transport_trait::Transport;
use std::net::SocketAddr;

fn make_constitution() -> ConstitutionInfo {
    ConstitutionInfo {
        version: 1,
        hash: [0u8; 32],
        proof_system_version: 1,
        state_commitment_algorithm: "MerklePatricia".to_string(),
        accepted_features: vec![],
    }
}

fn make_signing_key() -> ed25519_dalek::SigningKey {
    ed25519_dalek::SigningKey::from_bytes(&[0u8; 32])
}

#[test]
fn test_multi_machine_connection() {
    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let mut server = TcpTransport::new(
        addr,
        [0u8; 32],
        [0u8; 32],
        [0u8; 32],
        make_signing_key(),
        make_constitution(),
    );
    Transport::bind(&mut server).unwrap();

    let client_addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let mut client = TcpTransport::new(
        client_addr,
        [0u8; 32],
        [0u8; 32],
        [0u8; 32],
        make_signing_key(),
        make_constitution(),
    );
    Transport::bind(&mut client).unwrap();
}
