//! AmunChain Identity Core (AC-1.0 Article VI)
//! Shared identity data types. No crypto, no networking, no consensus.

use serde::{Deserialize, Serialize};

pub type ValidatorId = [u8; 32];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PeerId(pub [u8; 32]);

impl PeerId {
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

pub type AuthorityId = [u8; 32];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorCertificate {
    pub peer_id: PeerId,
    pub validator_id: ValidatorId,
    pub public_key: [u8; 32],
    pub authority_version: u64,
    pub authority_id: AuthorityId,
    pub valid_from: u64,
    pub valid_until: u64,
}

impl ValidatorCertificate {
    /// Structural validation only — no crypto, no authority check.
    pub fn is_structurally_valid(&self) -> bool {
        self.validator_id != [0u8; 32]
            && self.public_key != [0u8; 32]
            && self.valid_until > self.valid_from
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cert_valid() {
        let c = ValidatorCertificate {
            peer_id: PeerId([1u8; 32]),
            validator_id: [2u8; 32],
            public_key: [3u8; 32],
            authority_version: 1,
            authority_id: [4u8; 32],
            valid_from: 0,
            valid_until: 100,
        };
        assert!(c.is_structurally_valid());
    }
    #[test]
    fn test_cert_invalid_zero_id() {
        let c = ValidatorCertificate {
            peer_id: PeerId([1u8; 32]),
            validator_id: [0u8; 32],
            public_key: [3u8; 32],
            authority_version: 1,
            authority_id: [4u8; 32],
            valid_from: 0,
            valid_until: 100,
        };
        assert!(!c.is_structurally_valid());
    }
    #[test]
    fn test_peer_id_roundtrip() {
        let bytes = [0x42u8; 32];
        let id = PeerId::from_bytes(bytes);
        assert_eq!(id.as_bytes(), &bytes);
    }
}
