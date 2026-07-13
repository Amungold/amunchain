use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConstitutionalAuthority {
    pub authority_id: [u8; 32],
    pub authority_public_key: [u8; 32],
    pub authority_version: u64,
    pub activated_at_height: u64,
    pub revoked: bool,
    pub revoked_at_height: Option<u64>,
}

impl ConstitutionalAuthority {
    pub fn new(public_key: [u8; 32], version: u64, activated_at_height: u64) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_AUTHORITY_ID_V1");
        hasher.update(&public_key);
        let id = *hasher.finalize().as_bytes();
        Self {
            authority_id: id,
            authority_public_key: public_key,
            authority_version: version,
            activated_at_height,
            revoked: false,
            revoked_at_height: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n107_1_authority_id_deterministic() {
        let pk = [0x42u8; 32];
        let a1 = ConstitutionalAuthority::new(pk, 1, 0);
        let a2 = ConstitutionalAuthority::new(pk, 1, 0);
        assert_eq!(a1.authority_id, a2.authority_id);
    }

    #[test]
    fn n107_1_different_keys_different_ids() {
        let a1 = ConstitutionalAuthority::new([1u8; 32], 1, 0);
        let a2 = ConstitutionalAuthority::new([2u8; 32], 1, 0);
        assert_ne!(a1.authority_id, a2.authority_id);
    }
}
