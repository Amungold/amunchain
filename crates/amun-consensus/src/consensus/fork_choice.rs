use super::qc_store::{QCStore, QC, QCHash};
use super::ancestry::AncestryProofVerifier;

pub struct ForkChoiceEngine;

impl ForkChoiceEngine {
    pub fn best_justified_qc(store: &QCStore) -> Option<&QC> {
        store.values().filter(|q| q.is_justified()).max_by_key(|q| q.block_height)
    }
    pub fn best_locked_qc(store: &QCStore) -> Option<&QC> {
        store.values().filter(|q| q.is_locked()).max_by_key(|q| q.block_height)
    }
    pub fn best_finalized_qc(store: &QCStore) -> Option<&QC> {
        store.values().filter(|q| q.is_finalized()).max_by_key(|q| q.block_height)
    }
    pub fn choose_chain(store: &QCStore, from: &QCHash) -> Vec<QCHash> {
        let mut chain = Vec::new();
        let mut cur = Some(*from);
        while let Some(h) = cur { chain.push(h); cur = store.get(&h).and_then(|q| q.parent_hash); }
        chain
    }
    pub fn is_ancestor(store: &QCStore, anc: &QCHash, desc: &QCHash) -> bool {
        Self::choose_chain(store, desc).contains(anc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::qc_store::QC;
    use crate::crypto::types::SignatureBytes;
    fn create_justified_qc(store: &mut QCStore, h: u64, p: Option<QCHash>) -> QCHash {
        let mut q = QC::new(h, [h as u8; 32], 3, 1, 0, [0u8; 32]);
        if let Some(par) = p { q = q.with_parent(par); }
        let mut q = q.finalize();
        q.add_vote(1, SignatureBytes::from_64([1u8; 64]));
        q.add_vote(2, SignatureBytes::from_64([2u8; 64]));
        q.add_vote(3, SignatureBytes::from_64([3u8; 64]));
        let hash = q.hash; store.insert(q); hash
    }
    #[test] fn test_best_justified() { let mut s = QCStore::new(); let h = create_justified_qc(&mut s, 1, None); assert!(ForkChoiceEngine::best_justified_qc(&s).is_some()); }
}
