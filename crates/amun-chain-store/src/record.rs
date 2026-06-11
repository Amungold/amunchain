use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FinalizedChainRecord {
    pub height: u64,
    pub block_hash: [u8; 32],
    pub state_root: [u8; 32],
    pub history_root: [u8; 32],
    pub certificate_hash: [u8; 32],
    pub timestamp: u64,
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
            timestamp: 1000,
        };
        let encoded = record.encode();
        let decoded = FinalizedChainRecord::decode(&encoded).unwrap();
        assert_eq!(decoded.height, 42);
    }
}
