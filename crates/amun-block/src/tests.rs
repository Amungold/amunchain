#[cfg(test)]
use crate::*;
use amun_codec::{CanonicalDecode, CanonicalEncode};
use amun_consensus_types::*;
use amun_kernel_types::*;

fn hdr() -> BlockHeader {
    BlockHeader::new(
        BlockHeight(1),
        Epoch::new(1),
        ConsensusRound::new(0),
        PublicHash32::new([0xAAu8; 32]),
        StateCommitment::new([0xBBu8; 32]),
        PublicHash32::new([0xCCu8; 32]),
        ValidatorIndex::new(0),
        ChainId(1),
        PublicHash32::new([0xDDu8; 32]),
        1000,
    )
}

#[test]
fn test_header_size_freeze() {
    assert_eq!(BlockHeader::MAX_ENCODED_SIZE, 170);
}
#[test]
fn test_limits_freeze() {
    assert_eq!(BlockLimits::constitutional().max_transactions, 500);
}
#[test]
fn test_header_roundtrip() {
    let h = hdr();
    let mut b = [0u8; 256];
    let l = h.encode(&mut b).expect("test invariant");
    assert_eq!(
        BlockHeader::decode_exact(&b[..l])
            .expect("test invariant")
            .height,
        BlockHeight(1)
    );
}
#[test]
fn test_body_add_tx_hash() {
    let l = BlockLimits::constitutional();
    let mut b = BlockBody::new();
    assert!(b.add_tx_hash(PublicHash32::new([0x01u8; 32]), &l).is_ok());
    assert_eq!(b.tx_count(), 1);
}
#[test]
fn test_body_rejects_exceeding_limit() {
    let l = BlockLimits {
        max_transactions: 2,
        max_block_bytes: 1024,
    };
    let mut b = BlockBody::new();
    b.add_tx_hash(PublicHash32::new([0x01u8; 32]), &l)
        .expect("test invariant");
    b.add_tx_hash(PublicHash32::new([0x02u8; 32]), &l)
        .expect("test invariant");
    assert!(b.add_tx_hash(PublicHash32::new([0x03u8; 32]), &l).is_err());
}
#[test]
fn test_block_roundtrip() {
    let l = BlockLimits::constitutional();
    let mut b = BlockBody::new();
    b.add_tx_hash(PublicHash32::new([0x01u8; 32]), &l)
        .expect("test invariant");
    let blk = Block::new(hdr(), b);
    let mut buf = [0u8; 4096];
    let len = blk.encode(&mut buf).expect("test invariant");
    assert_eq!(
        Block::decode_exact(&buf[..len])
            .expect("test invariant")
            .tx_count(),
        1
    );
}
#[test]
fn test_block_id_deterministic() {
    let l = BlockLimits::constitutional();
    let mut b = BlockBody::new();
    b.add_tx_hash(PublicHash32::new([0x01u8; 32]), &l)
        .expect("test invariant");
    let blk = Block::new(hdr(), b);
    assert_eq!(blk.compute_id(), blk.compute_id());
}
#[test]
fn test_header_decode_rejects_short() {
    assert!(BlockHeader::decode(&[0u8; 10]).is_err());
}
#[test]
fn test_decode_exact_rejects_trailing() {
    let h = hdr();
    let mut buf = [0u8; 256];
    let l = h.encode(&mut buf).expect("test invariant");
    buf[l] = 0xFF;
    assert!(BlockHeader::decode_exact(&buf[..l + 1]).is_err());
}
#[test]
fn test_body_decode_rejects_high_tx_count() {
    let mut b = [0u8; 16];
    b[0] = 0xFF;
    b[1] = 0xFF;
    b[2] = 0xFF;
    b[3] = 0xFF;
    assert!(BlockBody::decode(&b).is_err());
}

#[test]
fn test_block_body_wire_freeze() {
    let limits = BlockLimits::constitutional();
    let mut body = BlockBody::new();
    body.add_tx_hash(PublicHash32::new([0x11u8; 32]), &limits)
        .expect("test invariant");
    body.add_tx_hash(PublicHash32::new([0x22u8; 32]), &limits)
        .expect("test invariant");
    let mut buf = [0u8; 256];
    let _len = body.encode(&mut buf).expect("test invariant");
    assert_eq!(_len, 68);
    assert_eq!(buf[0], 0x02);
    assert_eq!(buf[4], 0x11);
    assert_eq!(buf[36], 0x22);
}

#[test]
fn test_block_body_wire_freeze_boundary() {
    let limits = BlockLimits::constitutional();
    let mut body = BlockBody::new();
    body.add_tx_hash(PublicHash32::new([0x11u8; 32]), &limits)
        .expect("test invariant");
    body.add_tx_hash(PublicHash32::new([0x22u8; 32]), &limits)
        .expect("test invariant");
    let mut buf = [0u8; 256];
    let _ = body.encode(&mut buf).expect("test invariant");
    assert_eq!(buf[35], 0x11);
    assert_eq!(buf[36], 0x22);
}
