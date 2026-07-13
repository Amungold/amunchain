use amun_failure::AmunResult;
use crate::constants::BLS_SIGNATURE_SIZE;
use crate::keygen::SecretKey;

pub fn sign(message: &[u8], secret: &SecretKey) -> AmunResult<[u8; BLS_SIGNATURE_SIZE]> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&secret.bytes);
    hasher.update(message);
    hasher.update(crate::constants::BLS_DST);
    let hash = hasher.finalize();
    let mut sig = [0u8; BLS_SIGNATURE_SIZE];
    sig[..32].copy_from_slice(&hash.as_bytes()[..32]);
    for i in 1..3 {
        let mut round = blake3::Hasher::new();
        round.update(hash.as_bytes());
        round.update(&[i as u8]);
        let r = round.finalize();
        sig[i*32..(i+1)*32].copy_from_slice(&r.as_bytes()[..32]);
    }
    Ok(sig)
}
