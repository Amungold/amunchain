use amun_networking::crypto_identity::PeerKeyPair;
use amun_networking::validator_certificate::ValidatorCertificate;
use amun_node::peer_handshake::HandshakeMessage;

#[test]
fn test_handshake_verify_with_trust_anchors() {
    let authority = amun_networking::genesis_authority::genesis_authority_keypair();
    let validator = PeerKeyPair::generate();

    let cert = ValidatorCertificate::issue_v2(
        validator.peer_id(),
        validator.verifying_key.to_bytes(),
        0,
        [0u8; 32],
        &authority,
        0,
        0,
    );

    let genesis_hash = [0x42u8; 32];
    let handshake = HandshakeMessage::new(&validator, &cert, genesis_hash, "test-node", 4001);
    let result = handshake.verify(
        &genesis_hash,
        &amun_networking::genesis_authority::genesis_authority_public_key(),
    );
    assert!(
        result.is_ok(),
        "Handshake verification failed: {:?}",
        result.err()
    );
}
