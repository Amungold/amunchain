use amun_transition_proof::transition_proof::TransitionProof;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct HotProofStore {
    proofs: HashMap<[u8; 32], TransitionProof>,
    stored_at: HashMap<[u8; 32], u64>,
    retention_blocks: u64,
}

impl HotProofStore {
    pub fn new(retention_blocks: u64) -> Self {
        Self { proofs: HashMap::new(), stored_at: HashMap::new(), retention_blocks }
    }

    pub fn store(&mut self, proof: TransitionProof, block_height: u64) {
        let hash = proof.proof_hash;
        self.proofs.insert(hash, proof);
        self.stored_at.insert(hash, block_height);
    }

    pub fn get(&self, proof_hash: &[u8; 32]) -> Option<&TransitionProof> {
        self.proofs.get(proof_hash)
    }

    pub fn total(&self) -> usize { self.proofs.len() }

    pub fn prune(&mut self, current_block: u64) -> usize {
        let cutoff = current_block.saturating_sub(self.retention_blocks);
        let to_remove: Vec<[u8; 32]> = self.stored_at
            .iter()
            .filter(|(_, &stored)| stored < cutoff)
            .map(|(hash, _)| *hash)
            .collect();
        let count = to_remove.len();
        for hash in &to_remove {
            self.proofs.remove(hash);
            self.stored_at.remove(hash);
        }
        count
    }

    pub fn contains(&self, proof_hash: &[u8; 32]) -> bool {
        self.proofs.contains_key(proof_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_resource_core::ResourceId;

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32]; h[0] = seed; ResourceId(h)
    }

    fn make_proof(tx_hash: [u8; 32]) -> TransitionProof {
        TransitionProof::new(
            tx_hash, make_id(1), 1, [0u8; 32],
            [0x01; 32], [0x02; 32],
            vec![], vec![], vec![], vec![], 1000,
        )
    }

    #[test]
    fn w12_store_and_retrieve() {
        let mut store = HotProofStore::new(100);
        let proof = make_proof([0xaa; 32]);
        let hash = proof.proof_hash;
        store.store(proof, 50);
        assert_eq!(store.total(), 1);
        assert!(store.get(&hash).is_some());
    }

    #[test]
    fn w12_prune_expired_proofs() {
        let mut store = HotProofStore::new(10);
        let proof = make_proof([0xbb; 32]);
        store.store(proof, 5);
        let pruned = store.prune(20);
        assert_eq!(pruned, 1);
        assert_eq!(store.total(), 0);
    }

    #[test]
    fn w12_retain_recent_proofs() {
        let mut store = HotProofStore::new(10);
        let proof = make_proof([0xcc; 32]);
        store.store(proof, 15);
        let pruned = store.prune(20);
        assert_eq!(pruned, 0);
        assert_eq!(store.total(), 1);
    }
}
