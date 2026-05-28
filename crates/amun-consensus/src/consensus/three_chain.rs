use std::collections::BTreeMap;
use super::qc_store::{QC, QCHash};
use super::ancestry::AncestryProofVerifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChainStage { None = 0, Prepared = 1, Precommitted = 2, Committed = 3 }

#[derive(Debug, Clone)]
pub struct ThreeChainTracker {
    stages: BTreeMap<QCHash, ChainStage>,
    store: BTreeMap<QCHash, QC>,
}
impl ThreeChainTracker {
    pub fn new() -> Self { Self { stages: BTreeMap::new(), store: BTreeMap::new() } }
    pub fn add_qc(&mut self, qc: QC) { self.store.insert(qc.hash, qc); }
    pub fn process_qc(&mut self, qc: &QC) -> (Option<QCHash>, bool) {
        self.add_qc(qc.clone());
        let parent = match qc.parent_hash { Some(p) => p, None => { self.update_stage(qc.hash, ChainStage::Prepared); return (None, false); } };
        let grandparent = match self.store.get(&parent).and_then(|p| p.parent_hash) { Some(gp) => gp, None => { self.update_stage(qc.hash, ChainStage::Precommitted); if let Some(pq) = self.store.get(&parent) { self.update_stage(pq.hash, ChainStage::Prepared); } return (None, false); } };
        let mut temp = super::qc_store::QCStore::new();
        for (_, q) in &self.store { temp.insert(q.clone()); }
        let valid = AncestryProofVerifier::verify(&temp, &qc.hash, &parent) == super::ancestry::AncestryResult::Valid && AncestryProofVerifier::verify(&temp, &parent, &grandparent) == super::ancestry::AncestryResult::Valid;
        if !valid { self.update_stage(qc.hash, ChainStage::None); return (None, false); }
        self.update_stage(qc.hash, ChainStage::Precommitted);
        if let Some(pq) = self.store.get(&parent) { self.update_stage(pq.hash, ChainStage::Prepared); }
        if let Some(gq) = self.store.get(&grandparent) { self.update_stage(gq.hash, ChainStage::Committed); }
        (Some(grandparent), true)
    }
    fn update_stage(&mut self, h: QCHash, s: ChainStage) { let cur = self.stages.get(&h).copied().unwrap_or(ChainStage::None); if s > cur { self.stages.insert(h, s); } }
    pub fn get_stage(&self, h: &QCHash) -> ChainStage { self.stages.get(h).copied().unwrap_or(ChainStage::None) }
    pub fn is_committed(&self, h: &QCHash) -> bool { self.stages.get(h) == Some(&ChainStage::Committed) }
}
impl Default for ThreeChainTracker { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_three_chain() {
        let mut t = ThreeChainTracker::new();
        let q1 = QC::new(1, [1u8; 32], 3, 1, 0, [0u8; 32]).finalize();
        let q2 = QC::new(2, [2u8; 32], 3, 1, 0, [0u8; 32]).with_parent(q1.hash).finalize();
        let q3 = QC::new(3, [3u8; 32], 3, 1, 0, [0u8; 32]).with_parent(q2.hash).finalize();
        t.add_qc(q1.clone()); t.add_qc(q2.clone());
        let (committed, is_committed) = t.process_qc(&q3);
        assert!(is_committed); assert_eq!(committed, Some(q1.hash));
    }
}
