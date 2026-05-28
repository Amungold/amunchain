use super::qc_store::{QCStore, QCHash};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AncestryResult { Valid, InvalidParent, CycleDetected, Orphaned, SelfReference, DepthExceeded }

pub struct AncestryProofVerifier;
impl AncestryProofVerifier {
    pub const MAX_DEPTH: usize = 100;
    pub fn verify(store: &QCStore, child: &QCHash, parent: &QCHash) -> AncestryResult {
        if child == parent { return AncestryResult::SelfReference; }
        let mut cur = *child;
        let mut depth = 0;
        let mut visited = BTreeSet::new();
        while depth < Self::MAX_DEPTH {
            if visited.contains(&cur) { return AncestryResult::CycleDetected; }
            visited.insert(cur);
            if cur == *parent { return AncestryResult::Valid; }
            match store.get(&cur).and_then(|qc| qc.parent_hash) { Some(p) => cur = p, None => return AncestryResult::Orphaned, }
            depth += 1;
        }
        AncestryResult::DepthExceeded
    }
}
