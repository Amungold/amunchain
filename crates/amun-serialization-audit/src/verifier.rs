use amun_canonical::{CanonicalEncoder, SchemaVersion};

pub struct SerializationVerifier;

impl SerializationVerifier {
    pub fn verify_determinism(data: &[&[u8]]) -> bool {
        if data.len() < 2 {
            return true;
        }
        let hashes: Vec<[u8; 32]> = data
            .iter()
            .map(|d| {
                let mut enc = CanonicalEncoder::new(SchemaVersion::V4);
                let _ = enc.write_bytes(d);
                enc.finish()
            })
            .collect();
        hashes.windows(2).all(|w| w[0] == w[1])
    }

    pub fn verify_roundtrip(original: &[u8]) -> bool {
        let mut enc = CanonicalEncoder::new(SchemaVersion::V4);
        let _ = enc.write_bytes(original);
        let hash_before = enc.finish();
        let mut enc2 = CanonicalEncoder::new(SchemaVersion::V4);
        let _ = enc2.write_bytes(original);
        let hash_after = enc2.finish();
        hash_before == hash_after
    }
}
