use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use super::types::SignatureBytes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSignature {
    pub validator_id: u64,
    pub signature: SignatureBytes,
}

#[derive(Debug, Clone)]
pub struct SignatureAggregator {
    signatures: BTreeMap<u64, SignatureBytes>,
}
impl SignatureAggregator {
    pub fn new() -> Self { Self { signatures: BTreeMap::new() } }
    pub fn add_signature(&mut self, vid: u64, sig: SignatureBytes) -> bool {
        if self.signatures.contains_key(&vid) { return false; }
        self.signatures.insert(vid, sig);
        true
    }
    pub fn count(&self) -> usize { self.signatures.len() }
    pub fn has_quorum(&self, threshold: usize) -> bool { self.signatures.len() >= threshold }
    pub fn into_sorted_vec(self) -> Vec<(u64, SignatureBytes)> { self.signatures.into_iter().collect() }
}
impl Default for SignatureAggregator { fn default() -> Self { Self::new() } }
