use crate::commitment::ConstitutionalCommitment;
use crate::serializer::{serialize_v1, SERIALIZED_LENGTH};

pub const CONSTITUTIONAL_STATE_KEY: &[u8] = b"constitutional_commitment";

pub struct ConstitutionalState;

impl ConstitutionalState {
    pub fn save(commitment: &ConstitutionalCommitment) -> Vec<u8> {
        serialize_v1(commitment).to_vec()
    }

    pub fn load(data: &[u8]) -> Option<ConstitutionalCommitment> {
        if data.len() != SERIALIZED_LENGTH {
            return None;
        }
        let mut buf = [0u8; SERIALIZED_LENGTH];
        buf.copy_from_slice(data);

        let version = u16::from_be_bytes([buf[0], buf[1]]);
        let mut identity_root = [0u8; 32];
        let mut evidence_root = [0u8; 32];
        let mut governance_root = [0u8; 32];
        let mut economic_root = [0u8; 32];
        let mut constitutional_root = [0u8; 32];

        identity_root.copy_from_slice(&buf[2..34]);
        evidence_root.copy_from_slice(&buf[34..66]);
        governance_root.copy_from_slice(&buf[66..98]);
        economic_root.copy_from_slice(&buf[98..130]);
        constitutional_root.copy_from_slice(&buf[130..162]);

        Some(ConstitutionalCommitment {
            version,
            identity_root,
            evidence_root,
            governance_root,
            economic_root,
            constitutional_root,
        })
    }
}
