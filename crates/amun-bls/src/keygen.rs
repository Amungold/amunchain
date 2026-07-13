use amun_kernel_types::PublicKey;
use zeroize::Zeroize;

#[derive(Clone, Debug, Zeroize)]
pub struct SecretKey {
    pub bytes: [u8; 32],
}

#[derive(Clone, Debug)]
pub struct KeyPair {
    pub secret: SecretKey,
    pub public: PublicKey,
}

impl KeyPair {
    pub fn generate_deterministic(seed: &[u8; 32]) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(seed);
        hasher.update(b"amun-bls-keygen-v1");
        let hash = hasher.finalize();
        let mut secret_bytes = [0u8; 32];
        secret_bytes.copy_from_slice(&hash.as_bytes()[..32]);
        let secret = SecretKey { bytes: secret_bytes };
        let mut pub_hasher = blake3::Hasher::new();
        pub_hasher.update(&secret.bytes);
        pub_hasher.update(b"amun-bls-pubkey-v1");
        let pub_hash = pub_hasher.finalize();
        let mut pubkey = PublicKey::default();
        pubkey.0[..32].copy_from_slice(&pub_hash.as_bytes()[..32]);
        pubkey.0[32..].copy_from_slice(&pub_hash.as_bytes()[..16]);
        KeyPair { secret, public: pubkey }
    }
}
