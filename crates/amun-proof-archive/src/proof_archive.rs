use amun_transition_proof::transition_proof::TransitionProof;
use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
use std::collections::HashMap;
use crate::hot_store::HotProofStore;

#[derive(Debug, Clone, Default)]
pub struct ProofArchive {
    permanent: HashMap<[u8; 32], TransitionProof>,
    evidence: Vec<ConstitutionalEvidence>,
}

impl ProofArchive {
    pub fn new() -> Self { Self::default() }

    pub fn archive_permanent(&mut self, proof: TransitionProof) {
        self.permanent.insert(proof.proof_hash, proof);
    }

    pub fn archive_evidence(&mut self, evidence: ConstitutionalEvidence) {
        self.evidence.push(evidence);
    }

    pub fn get_permanent(&self, proof_hash: &[u8; 32]) -> Option<&TransitionProof> {
        self.permanent.get(proof_hash)
    }

    pub fn total_permanent(&self) -> usize { self.permanent.len() }
    pub fn total_evidence(&self) -> usize { self.evidence.len() }

    pub fn archive_from_hot(
        &mut self,
        hot_store: &mut HotProofStore,
        proof_hash: &[u8; 32],
    ) -> Option<()> {
        if let Some(proof) = hot_store.get(proof_hash) {
            self.archive_permanent(proof.clone());
            Some(())
        } else {
            None
        }
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
    fn w12_archive_permanent() {
        let mut archive = ProofArchive::new();
        let proof = make_proof([0xdd; 32]);
        let hash = proof.proof_hash;
        archive.archive_permanent(proof);
        assert_eq!(archive.total_permanent(), 1);
        assert!(archive.get_permanent(&hash).is_some());
    }

    #[test]
    fn w12_archive_evidence() {
        let mut archive = ProofArchive::new();
        let ev = ConstitutionalEvidence::ExecutionFailure {
            reason: "test".into(),
            contract_id: make_id(1),
            block_height: 1,
            transaction_hash: [0xee; 32],
            gas_consumed: 0,
        };
        archive.archive_evidence(ev);
        assert_eq!(archive.total_evidence(), 1);
    }

    #[test]
    fn w12_transfer_from_hot_to_permanent() {
        let mut hot = HotProofStore::new(100);
        let mut archive = ProofArchive::new();
        let proof = make_proof([0xff; 32]);
        let hash = proof.proof_hash;
        hot.store(proof, 1);
        assert!(archive.archive_from_hot(&mut hot, &hash).is_some());
        assert_eq!(archive.total_permanent(), 1);
    }
}
