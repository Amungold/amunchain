use amun_canonical::{CanonicalEncoder, SchemaVersion};

pub struct ChainIdentity {
    pub chain_id: u64,
    pub genesis_hash: [u8; 32],
    pub constitution_hash: [u8; 32],
}

impl ChainIdentity {
    pub fn new(chain_id: u64, genesis_hash: [u8; 32], constitution_hash: [u8; 32]) -> Self {
        Self {
            chain_id,
            genesis_hash,
            constitution_hash,
        }
    }

    pub fn fingerprint(&self) -> [u8; 32] {
        let mut enc = CanonicalEncoder::new(SchemaVersion::V4);
        let _ = enc.write_u64(self.chain_id);
        let _ = enc.write_bytes(&self.genesis_hash);
        let _ = enc.write_bytes(&self.constitution_hash);
        enc.finish()
    }
}
