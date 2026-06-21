use amun_resource_core::ResourceId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftCollectionMetadata {
    pub name: String,
    pub description: String,
    pub creator: [u8; 32],
    pub max_supply: Option<u64>,
    pub image_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftMetadata {
    pub name: String,
    pub description: String,
    pub image_uri: String,
    pub attributes: Vec<NftAttribute>,
    pub constitutional_role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftAttribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NftEvent {
    Mint {
        collection_id: ResourceId,
        token_id: ResourceId,
        owner: [u8; 32],
        metadata_hash: [u8; 32],
    },
    Transfer {
        token_id: ResourceId,
        from: [u8; 32],
        to: [u8; 32],
    },
    Burn {
        token_id: ResourceId,
    },
}

/// Evidence record for NFT operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NftEvidence {
    pub event: NftEvent,
    pub timestamp: u64,
    pub block_height: u64,
    pub evidence_hash: [u8; 32],
}

impl NftEvidence {
    pub fn new(event: NftEvent, timestamp: u64, block_height: u64) -> Self {
        let mut evidence = Self {
            event,
            timestamp,
            block_height,
            evidence_hash: [0u8; 32],
        };
        evidence.evidence_hash = evidence.compute_hash();
        evidence
    }

    pub fn compute_hash(&self) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_vec(&self.event).unwrap());
        hasher.update(self.timestamp.to_le_bytes());
        hasher.update(self.block_height.to_le_bytes());
        hasher.finalize().into()
    }
}
