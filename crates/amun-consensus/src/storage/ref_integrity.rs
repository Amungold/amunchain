use std::collections::{BTreeMap, BTreeSet};
use crate::state_tree::NodeHash;
pub struct RefIntegrityReport;
pub struct RefIntegrityVerifier;
impl RefIntegrityVerifier {
    pub fn verify(_reachable: &BTreeSet<NodeHash>, _refs: &BTreeMap<NodeHash, usize>) -> RefIntegrityReport { RefIntegrityReport }
}
