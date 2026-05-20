#[cfg(test)]
use crate::*;
use amun_kernel_types::*;

// ========================================================================
// Primitive encoding freeze tests
// ========================================================================

#[test]
fn test_u8_encode() {
    let mut buf = [0u8; 1];
    assert_eq!(255u8.encode(&mut buf).expect("test invariant"), 1);
    assert_eq!(buf[0], 255);
}

#[test]
fn test_u16_little_endian_freeze() {
    let mut buf = [0u8; 2];
    0x1122u16.encode(&mut buf).expect("test invariant");
    assert_eq!(buf, [0x22, 0x11]);
}

#[test]
fn test_u32_little_endian_freeze() {
    let mut buf = [0u8; 4];
    0x11223344u32.encode(&mut buf).expect("test invariant");
    assert_eq!(buf, [0x44, 0x33, 0x22, 0x11]);
}

#[test]
fn test_u64_little_endian_freeze() {
    let mut buf = [0u8; 8];
    0x1122334455667788u64
        .encode(&mut buf)
        .expect("test invariant");
    assert_eq!(buf[0], 0x88);
    assert_eq!(buf[7], 0x11);
}

#[test]
fn test_u128_little_endian_freeze() {
    let mut buf = [0u8; 16];
    0x000102030405060708090A0B0C0D0E0Fu128
        .encode(&mut buf)
        .expect("test invariant");
    assert_eq!(buf[0], 0x0F);
    assert_eq!(buf[15], 0x00);
}

#[test]
fn test_u8_roundtrip() {
    let v: u8 = 0xAB;
    let mut buf = [0u8; 1];
    v.encode(&mut buf).expect("test invariant");
    let (d, len) = u8::decode(&buf).expect("test invariant");
    assert_eq!(len, 1);
    assert_eq!(d, v);
}

#[test]
fn test_u16_roundtrip() {
    let v: u16 = 0xABCD;
    let mut buf = [0u8; 2];
    v.encode(&mut buf).expect("test invariant");
    let (d, len) = u16::decode(&buf).expect("test invariant");
    assert_eq!(len, 2);
    assert_eq!(d, v);
}

#[test]
fn test_u32_roundtrip() {
    let v: u32 = 0xDEADBEEF;
    let mut buf = [0u8; 4];
    v.encode(&mut buf).expect("test invariant");
    let (d, len) = u32::decode(&buf).expect("test invariant");
    assert_eq!(len, 4);
    assert_eq!(d, v);
}

#[test]
fn test_u64_roundtrip() {
    let v: u64 = 0xDEADBEEFCAFEBABE;
    let mut buf = [0u8; 8];
    v.encode(&mut buf).expect("test invariant");
    let (d, len) = u64::decode(&buf).expect("test invariant");
    assert_eq!(len, 8);
    assert_eq!(d, v);
}

#[test]
fn test_u128_roundtrip() {
    let v: u128 = u128::MAX;
    let mut buf = [0u8; 16];
    v.encode(&mut buf).expect("test invariant");
    let (d, len) = u128::decode(&buf).expect("test invariant");
    assert_eq!(len, 16);
    assert_eq!(d, v);
}

#[test]
fn test_bytes32_roundtrip() {
    let v: [u8; 32] = [0x42u8; 32];
    let mut buf = [0u8; 32];
    v.encode(&mut buf).expect("test invariant");
    let (d, len) = <[u8; 32]>::decode(&buf).expect("test invariant");
    assert_eq!(len, 32);
    assert_eq!(d, v);
}

// ========================================================================
// Decode rejection tests
// ========================================================================

#[test]
fn test_decode_short_buffer_rejected() {
    assert!(u16::decode(&[0x00]).is_err());
    assert!(u32::decode(&[0x00, 0x00]).is_err());
    assert!(u64::decode(&[0x00, 0x00, 0x00]).is_err());
}

#[test]
fn test_decode_exact_trailing_bytes_rejected() {
    let mut buf = [0u8; 16];
    42u64.encode(&mut buf).expect("test invariant");
    buf[8] = 0xFF;
    assert!(u64::decode_exact(&buf[..9]).is_err());
}

#[test]
fn test_decode_exact_exact_fits() {
    let mut buf = [0u8; 8];
    42u64.encode(&mut buf).expect("test invariant");
    assert_eq!(u64::decode_exact(&buf).expect("test invariant"), 42);
}

#[test]
fn test_encode_buffer_too_small() {
    let mut buf = [0u8; 4];
    assert!(42u64.encode(&mut buf).is_err());
}

// ========================================================================
// Kernel type roundtrip tests
// ========================================================================

#[test]
fn test_epoch_roundtrip() {
    let e = Epoch::new(42);
    let mut buf = [0u8; 8];
    e.encode(&mut buf).expect("test invariant");
    let (d, _) = Epoch::decode(&buf).expect("test invariant");
    assert_eq!(d, e);
}

#[test]
fn test_round_roundtrip() {
    let r = Round::new(7);
    let mut buf = [0u8; 8];
    r.encode(&mut buf).expect("test invariant");
    let (d, _) = Round::decode(&buf).expect("test invariant");
    assert_eq!(d, r);
}

#[test]
fn test_hash32_roundtrip() {
    let h = PublicHash32::new([0x42u8; 32]);
    let mut buf = [0u8; 32];
    h.encode(&mut buf).expect("test invariant");
    let (d, _) = PublicHash32::decode(&buf).expect("test invariant");
    assert_eq!(d, h);
}

#[test]
fn test_validator_id_roundtrip() {
    let v = ValidatorId::new([0xABu8; 32]);
    let mut buf = [0u8; 32];
    v.encode(&mut buf).expect("test invariant");
    let (d, _) = ValidatorId::decode(&buf).expect("test invariant");
    assert_eq!(d, v);
}

// ========================================================================
// Domain separation tests
// ========================================================================

#[test]
fn test_domain_separation_all_distinct() {
    let data = b"test";
    let h1 = HashDomain::Block.hash(data);
    let h2 = HashDomain::Transaction.hash(data);
    let h3 = HashDomain::Vote.hash(data);
    assert_ne!(h1, h2);
    assert_ne!(h1, h3);
    assert_ne!(h2, h3);
}

#[test]
fn test_domain_hash_deterministic() {
    let h1 = HashDomain::Block.hash(b"data");
    let h2 = HashDomain::Block.hash(b"data");
    assert_eq!(h1, h2);
}

#[test]
fn test_domain_from_byte_valid() {
    assert_eq!(HashDomain::from_byte(0x01), Some(HashDomain::Block));
    assert_eq!(HashDomain::from_byte(0x08), Some(HashDomain::EpochBoundary));
    assert_eq!(HashDomain::from_byte(0x00), None);
    assert_eq!(HashDomain::from_byte(0xFF), None);
}
