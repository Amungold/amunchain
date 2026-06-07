use amun_header_gossip::message::GossipMessage;

#[test]
fn test_gossip_message_creation() {
    let msg = GossipMessage {
        block_height: 1,
        block_hash: "0xabc".into(),
        state_root: "0xdef".into(),
    };
    assert_eq!(msg.block_height, 1);
    assert!(!msg.block_hash.is_empty());
}
