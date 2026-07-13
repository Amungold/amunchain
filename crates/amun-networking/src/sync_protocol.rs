use amun_chain_checkpoint::CheckpointCertificate;
use serde::{Deserialize, Serialize};

/// Request from a bootstrapping node to learn the current chain state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRequest {
    pub from_height: u64,
}

/// Response containing the latest height and checkpoints for catch-up.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResponse {
    pub latest_height: u64,
    pub checkpoints: Vec<CheckpointCertificate>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n18_sync_request_serialization() {
        let req = SyncRequest { from_height: 0 };
        let json = serde_json::to_string(&req).unwrap();
        let decoded: SyncRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.from_height, 0);
    }

    #[test]
    fn n18_sync_response_serialization() {
        let resp = SyncResponse {
            latest_height: 42,
            checkpoints: Vec::new(),
        };
        let json = serde_json::to_string(&resp).unwrap();
        let decoded: SyncResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.latest_height, 42);
        assert!(decoded.checkpoints.is_empty());
    }
}
