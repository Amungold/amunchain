use amun_kernel_types::PublicKey;
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use crate::constants::{BLS_SIGNATURE_SIZE, BLS_MAX_SIGNERS};


pub fn aggregate_signatures(signatures: &[[u8; BLS_SIGNATURE_SIZE]]) -> AmunResult<[u8; BLS_SIGNATURE_SIZE]> {
    if signatures.is_empty() || signatures.len() > BLS_MAX_SIGNERS {
        return Err(FailureContext::new(ConstitutionalFault::InvalidInput, 0x0011, 0x0001));
    }
    let mut agg = [0u8; BLS_SIGNATURE_SIZE];
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"amun-bls-aggregate-v1");
    for sig in signatures {
        hasher.update(sig);
    }
    let hash = hasher.finalize();
    agg[..32].copy_from_slice(&hash.as_bytes()[..32]);
    Ok(agg)
}

pub fn aggregate_public_keys(public_keys: &[PublicKey]) -> AmunResult<PublicKey> {
    if public_keys.is_empty() || public_keys.len() > BLS_MAX_SIGNERS {
        return Err(FailureContext::new(ConstitutionalFault::InvalidInput, 0x0011, 0x0002));
    }
    let mut agg = PublicKey::default();
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"amun-bls-agg-pubkey-v1");
    for pk in public_keys {
        hasher.update(&pk.0);
    }
    let hash = hasher.finalize();
    agg.0[..32].copy_from_slice(&hash.as_bytes()[..32]);
    agg.0[32..].copy_from_slice(&hash.as_bytes()[..16]);
    Ok(agg)
}
