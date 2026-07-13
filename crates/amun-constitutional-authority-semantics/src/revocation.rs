use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// A revocation witness proves that a capability has been revoked.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RevocationWitness {
    pub capability_id: String,
    pub revoked_by: String,
    pub timestamp: String,
}

/// A deterministic registry of revoked capabilities.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RevocationRegistry {
    revoked: BTreeSet<String>,
}

impl RevocationRegistry {
    pub fn new() -> Self {
        Self {
            revoked: BTreeSet::new(),
        }
    }
    pub fn revoke(&mut self, witness: &RevocationWitness) {
        self.revoked.insert(witness.capability_id.clone());
    }
    pub fn is_revoked(&self, capability_id: &str) -> bool {
        self.revoked.contains(capability_id)
    }
}
