use ed25519_dalek::{Signature, Verifier, VerifyingKey};

/// Single source of truth for all cryptographic operations:
/// - Ed25519 verification
/// - Vote signing payload construction
/// - Validator ID derivation
///
/// Both the new API (IdentityService) and the compatibility layer (compat.rs)
/// delegate to this module. No other module owns this logic.

/// Verify an Ed25519 signature using real ed25519-dalek verification.
pub fn verify_ed25519(public_key: &[u8; 32], message: &[u8], signature: &[u8; 64]) -> bool {
    // Reject obviously invalid inputs
    if public_key == &[0u8; 32] || message.is_empty() || signature == &[0u8; 64] {
        return false;
    }
    let vk = match VerifyingKey::from_bytes(public_key) {
        Ok(k) => k,
        Err(_) => return false,
    };
    let sig = match Signature::from_slice(signature) {
        Ok(s) => s,
        Err(_) => return false,
    };
    vk.verify(message, &sig).is_ok()
}

/// Build a vote signing payload deterministically.
/// Format: voter_id (32) + chain_id (8) + height (8) + round (8) + block_hash (32)
pub fn build_vote_signing_payload(
    voter_id: &[u8; 32],
    chain_id: u64,
    height: u64,
    round: u64,
    block_hash: &[u8; 32],
) -> Vec<u8> {
    let mut payload = Vec::with_capacity(88);
    payload.extend_from_slice(voter_id);
    payload.extend_from_slice(&chain_id.to_le_bytes());
    payload.extend_from_slice(&height.to_le_bytes());
    payload.extend_from_slice(&round.to_le_bytes());
    payload.extend_from_slice(block_hash);
    payload
}

/// Derive a validator ID from a public key.
/// Currently copies the public key directly; may use hashing in the future.
pub fn derive_validator_id(public_key: &[u8; 32]) -> [u8; 32] {
    let mut id = [0u8; 32];
    let len = public_key.len().min(32);
    id[..len].copy_from_slice(&public_key[..len]);
    id
}

/// Default chain ID constant.
pub const DEFAULT_CHAIN_ID: u64 = 1;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_ed25519_rejects_zero_key() {
        assert!(!verify_ed25519(&[0u8; 32], b"msg", &[1u8; 64]));
    }

    #[test]
    fn test_verify_ed25519_rejects_empty_message() {
        assert!(!verify_ed25519(&[1u8; 32], b"", &[1u8; 64]));
    }

    #[test]
    fn test_verify_ed25519_rejects_zero_signature() {
        assert!(!verify_ed25519(&[1u8; 32], b"msg", &[0u8; 64]));
    }

    #[test]
    fn test_verify_ed25519_rejects_invalid_signature() {
        // A random signature should not verify against an arbitrary public key
        let mut sig = [0u8; 64];
        sig[0] = 0xAA;
        assert!(!verify_ed25519(&[0xAAu8; 32], b"msg", &sig));
    }

    #[test]
    fn test_vote_signing_payload_size() {
        let payload = build_vote_signing_payload(&[1u8; 32], 1, 100, 2, &[3u8; 32]);
        assert_eq!(payload.len(), 88);
    }

    #[test]
    fn test_vote_signing_payload_deterministic() {
        let p1 = build_vote_signing_payload(&[1u8; 32], 1, 100, 2, &[3u8; 32]);
        let p2 = build_vote_signing_payload(&[1u8; 32], 1, 100, 2, &[3u8; 32]);
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_derive_validator_id_copies_bytes() {
        let pk = [0xAAu8; 32];
        let id = derive_validator_id(&pk);
        assert_eq!(id, pk);
    }
}
