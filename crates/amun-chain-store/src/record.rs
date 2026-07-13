use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FinalizedChainRecord {
    pub height: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub history_root: [u8; 32],
    pub certificate_hash: [u8; 32],
    pub slashing_root: [u8; 32],
    /// N129.1: Hash of the constitutional verdict for this block
    pub verdict_hash: [u8; 32],
    /// N129.2: Hash of the constitutional evidence record
    pub evidence_record_hash: [u8; 32],
    /// N129.3: Evidence root chaining all constitutional proofs
    pub evidence_root: [u8; 32],
    pub timestamp: u64,
    /// CCA v1.0: Constitutional commitment root (hash of serialized commitment)
    pub commitment_root: [u8; 32],
    /// CCA v1.0: Constitutional root (hash of identity, evidence, governance, economic)
    pub constitutional_root: [u8; 32],
    /// CCA v1.0: Economic Merkle tree root
    pub economic_root: [u8; 32],
    /// CCA v1.0: Identity root from authority registry
    pub identity_root: [u8; 32],
    /// CCA v1.0: Governance root from governance subsystem
    pub governance_root: [u8; 32],
}

impl FinalizedChainRecord {
    pub fn encode(&self) -> Vec<u8> {
        postcard::to_stdvec(self).expect("FinalizedChainRecord encode")
    }
    pub fn decode(data: &[u8]) -> Result<Self, String> {
        postcard::from_bytes(data).map_err(|e| format!("Decode error: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn n70_record_roundtrip() {
        let record = FinalizedChainRecord {
            height: 42,
            block_hash: [0xAA; 32],
            state_root: [0xBB; 32],
            history_root: [0xCC; 32],
            certificate_hash: [0xDD; 32],
            slashing_root: [0u8; 32],
            verdict_hash: [0u8; 32],
            evidence_record_hash: [0u8; 32],
            evidence_root: [0u8; 32],
            timestamp: 1000,
            commitment_root: [0x11; 32],
            constitutional_root: [0x22; 32],
            economic_root: [0x33; 32],
            identity_root: [0x44; 32],
            governance_root: [0x55; 32],
        };
        let encoded = record.encode();
        let decoded = FinalizedChainRecord::decode(&encoded).unwrap();
        assert_eq!(decoded.height, 42);
        assert_eq!(decoded.commitment_root, record.commitment_root);
        assert_eq!(decoded.constitutional_root, record.constitutional_root);
        assert_eq!(decoded.economic_root, record.economic_root);
        assert_eq!(decoded.identity_root, record.identity_root);
        assert_eq!(decoded.governance_root, record.governance_root);
    }
}
