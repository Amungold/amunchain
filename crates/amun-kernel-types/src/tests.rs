#[cfg(test)]
use crate::*;

#[test]
fn test_epoch_overflow() {
    assert!(Epoch(u64::MAX).next().is_err());
}
#[test]
fn test_epoch_next_valid() {
    assert_eq!(Epoch::new(1).next().expect("test invariant"), Epoch::new(2));
}
#[test]
fn test_epoch_previous() {
    assert_eq!(Epoch::new(5).previous(), Epoch::new(4));
}
#[test]
fn test_epoch_default_zero() {
    assert_eq!(Epoch::default(), Epoch::ZERO);
}
#[test]
fn test_round_overflow() {
    assert!(Round(u64::MAX).next().is_err());
}
#[test]
fn test_round_next_valid() {
    assert_eq!(Round::new(3).next().expect("test invariant"), Round::new(4));
}
#[test]
fn test_round_previous() {
    assert_eq!(Round::new(5).previous(), Round::new(4));
}
#[test]
fn test_hash32_default() {
    assert_eq!(PublicHash32::default().as_bytes(), &[0u8; 32]);
}
#[test]
fn test_hash32_new() {
    assert_eq!(PublicHash32::new([42u8; 32]).as_bytes(), &[42u8; 32]);
}
#[test]
fn test_secret_hash32_drop() {
    drop(SecretHash32::new([0x42u8; 32]));
}
#[test]
fn test_validator_id_default() {
    assert_eq!(ValidatorId::default(), ValidatorId::new([0u8; 32]));
}
#[test]
fn test_public_key_default() {
    assert_eq!(PublicKey::default(), PublicKey::new([0u8; 48]));
}
#[test]
fn test_signature_default() {
    assert_eq!(Signature::default(), Signature::new([0u8; 96]));
}
#[test]
fn test_newtypes() {
    assert_eq!(BlockHeight(42).0, 42);
    assert_eq!(ChainId(1).0, 1);
    assert_eq!(Gas(1000).0, 1000);
    assert_eq!(Amount(1000000).0, 1000000);
    assert_eq!(Nonce(42).0, 42);
}
#[test]
fn test_capacity_constants() {
    #[allow(clippy::assertions_on_constants)]
    {
        assert!(constitutional_capacity::MAX_SET_ITEMS > 0);
    }
    #[allow(clippy::assertions_on_constants)]
    {
        assert!(constitutional_capacity::MAX_MESSAGE_BYTES > 0);
    }
}
