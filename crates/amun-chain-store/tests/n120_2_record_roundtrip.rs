use amun_chain_store::record::FinalizedChainRecord;

#[test]
fn n120_2_record_roundtrip_preserves_slashing_root() {
    let record = FinalizedChainRecord {
        height: 42,
        block_hash: [0xAA; 32],
        state_root: [0xBB; 32],
        history_root: [0xCC; 32],
        certificate_hash: [0xDD; 32],
        slashing_root: [0x42; 32],
        timestamp: 1000,
    };
    let encoded = record.encode();
    let decoded = FinalizedChainRecord::decode(&encoded).unwrap();
    assert_eq!(
        decoded.slashing_root, [0x42; 32],
        "N120.2 FAIL: slashing_root must survive encode/decode roundtrip"
    );
    assert_eq!(decoded.height, 42);
    assert_eq!(decoded.block_hash, [0xAA; 32]);
}
