use amun_kernel::canonical::{CanonicalEncode, CanonicalEncoder};
use amun_kernel::hashing::domain_tags;

#[derive(Clone, PartialEq, Eq)]
pub struct ChainIdentityRoot {
    pub network_id: u64,
    pub genesis_hash: [u8; 32],
    pub validator_root: [u8; 32],
    pub constitution_root: [u8; 32],
    pub protocol_root: [u8; 32],
    pub epoch_constitution_root: [u8; 32],
}

impl CanonicalEncode for ChainIdentityRoot {
    fn encode_canonical(&self, out: &mut Vec<u8>) {
        self.network_id.encode_canonical(out);
        out.extend_from_slice(&self.genesis_hash);
        out.extend_from_slice(&self.validator_root);
        out.extend_from_slice(&self.constitution_root);
        out.extend_from_slice(&self.protocol_root);
        out.extend_from_slice(&self.epoch_constitution_root);
    }
}

impl ChainIdentityRoot {
    pub fn compute(&self) -> [u8; 32] {
        CanonicalEncoder::hash_value(self, domain_tags::IDENTITY)
    }
}
