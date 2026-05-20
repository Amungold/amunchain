#[cfg(test)]
use crate::tx::*;
use amun_kernel_types::*;

#[test]
fn test_transfer_rejects_zero_chain() {
    let r = UnsignedTransaction::new_transfer(
        1,
        ChainId(0),
        Nonce(0),
        PublicKey::new([0xCCu8; 48]),
        PublicHash32::new([0xBBu8; 32]),
        Amount(100),
        Gas(1000),
        b"",
    );
    assert!(r.is_err());
}

#[test]
fn test_transfer_rejects_zero_gas() {
    let r = UnsignedTransaction::new_transfer(
        1,
        ChainId(42),
        Nonce(0),
        PublicKey::new([0xCCu8; 48]),
        PublicHash32::new([0xBBu8; 32]),
        Amount(100),
        Gas(0),
        b"",
    );
    assert!(r.is_err());
}

#[test]
fn test_transfer_roundtrip_ok() {
    let r = UnsignedTransaction::new_transfer(
        1,
        ChainId(42),
        Nonce(0),
        PublicKey::new([0xAAu8; 48]),
        PublicHash32::new([0xBBu8; 32]),
        Amount(100),
        Gas(1000),
        b"",
    );
    assert!(r.is_ok());
    let tx = r.expect("test invariant");
    assert!(tx.validate_basic().is_ok());
}

#[test]
fn test_stake_ok() {
    let r = UnsignedTransaction::new_stake(
        1,
        ChainId(42),
        Nonce(0),
        PublicKey::new([0xAAu8; 48]),
        PublicHash32::new([0xBBu8; 32]),
        Amount(500),
        Gas(2000),
    );
    assert!(r.is_ok());
    assert!(r.expect("test invariant").validate_basic().is_ok());
}

#[test]
fn test_unstake_ok() {
    let r = UnsignedTransaction::new_unstake(
        1,
        ChainId(42),
        Nonce(0),
        PublicKey::new([0xAAu8; 48]),
        PublicHash32::new([0xBBu8; 32]),
        Amount(300),
        Gas(2000),
    );
    assert!(r.is_ok());
    assert!(r.expect("test invariant").validate_basic().is_ok());
}

#[test]
fn test_contract_call_ok() {
    let r = UnsignedTransaction::new_contract_call(
        1,
        ChainId(42),
        Nonce(0),
        PublicKey::new([0xAAu8; 48]),
        PublicHash32::new([0xBBu8; 32]),
        Gas(5000),
        b"hello",
    );
    assert!(r.is_ok());
    assert!(r.expect("test invariant").validate_basic().is_ok());
}

#[test]
fn test_rejects_zero_pubkey() {
    let r = UnsignedTransaction::new_transfer(
        1,
        ChainId(42),
        Nonce(0),
        PublicKey::new([0u8; 48]),
        PublicHash32::new([0xBBu8; 32]),
        Amount(100),
        Gas(1000),
        b"",
    );
    assert!(r.is_ok());
    assert!(r.expect("test invariant").validate_basic().is_err());
}
