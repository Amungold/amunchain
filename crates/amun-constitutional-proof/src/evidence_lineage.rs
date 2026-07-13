use serde::{Deserialize, Serialize};

/// Links a derived piece of evidence back to its parent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceLineage {
    pub parent_id: String,
    pub derivation: String,
    pub parent_hash: String,
}

impl EvidenceLineage {
    pub fn new(parent_id: String, derivation: String, parent_hash: String) -> Self {
        Self {
            parent_id,
            derivation,
            parent_hash,
        }
    }
}
