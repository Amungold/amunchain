use ed25519_dalek::{Signature, Verifier, VerifyingKey};

pub const CONSENSUS_SIGNATURE_DOMAIN: &[u8] = b"AMUNCHAIN::CONSENSUS::V1";
pub const DEFAULT_CHAIN_ID: u64 = 1;

/// Build the canonical message that must be signed for a vote.
pub fn vote_signing_payload(
    voter_id: &[u8; 32],
    chain_id: u64,
    height: u64,
    block_hash: &[u8; 32],
    state_root: &[u8; 32],
    approve: bool,
    timestamp: u64,
) -> Vec<u8> {
    let mut data = Vec::from(b"AMUN_VOTE_V1");

    data.extend_from_slice(CONSENSUS_SIGNATURE_DOMAIN);
    data.extend_from_slice(&chain_id.to_le_bytes());
    data.extend_from_slice(voter_id);
    data.extend_from_slice(&height.to_le_bytes());
    data.extend_from_slice(block_hash);
    data.extend_from_slice(state_root);
    data.push(approve as u8);
    data.extend_from_slice(&timestamp.to_le_bytes());

    data
}
/// Verify an Ed25519 signature on a given payload.
/// `pk` is a 32-byte compressed public key.
/// `signature` is a 64-byte signature.
pub fn verify_ed25519(pk: &[u8; 32], payload: &[u8], signature: &[u8; 64]) -> bool {
    let Ok(vk) = VerifyingKey::from_bytes(pk) else {
        return false;
    };
    let Ok(sig) = Signature::from_slice(signature) else {
        return false;
    };
    vk.verify(payload, &sig).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use rand::rngs::OsRng;

    fn gen_key() -> (SigningKey, [u8; 32]) {
        let sk = SigningKey::generate(&mut OsRng);
        let pk = sk.verifying_key().to_bytes();
        (sk, pk)
    }

    #[test]
    fn n105_sign_and_verify_vote() {
        let (sk, pk) = gen_key();
        let voter_id = crate::validator_id::derive_validator_id(&pk);
        let payload = vote_signing_payload(
            &voter_id,
            DEFAULT_CHAIN_ID,
            42,
            &[0xAA; 32],
            &[0xBB; 32],
            true,
            1000,
        );
        let signature = sk.sign(&payload).to_bytes();
        assert!(verify_ed25519(&pk, &payload, &signature));
    }

    #[test]
    fn n105_tampered_payload_rejected() {
        let (sk, pk) = gen_key();
        let voter_id = crate::validator_id::derive_validator_id(&pk);
        let mut payload = vote_signing_payload(
            &voter_id,
            DEFAULT_CHAIN_ID,
            42,
            &[0xAA; 32],
            &[0xBB; 32],
            true,
            1000,
        );
        payload[20] ^= 1; // tamper
                          // Sign original payload (valid)
        let original_payload = vote_signing_payload(
            &voter_id,
            DEFAULT_CHAIN_ID,
            42,
            &[0xAA; 32],
            &[0xBB; 32],
            true,
            1000,
        );
        let signature = sk.sign(&original_payload).to_bytes();
        // Verification with tampered payload must fail
        assert!(!verify_ed25519(&pk, &payload, &signature));
    }
}
