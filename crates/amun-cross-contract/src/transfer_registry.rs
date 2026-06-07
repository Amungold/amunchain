use std::collections::HashSet;

use amun_resource_core::ResourceId;
use amun_evidence_engine::evidence_types::ConstitutionalEvidence;

use crate::transfer_proof::CrossContractTransferProof;

#[derive(Debug, Clone, Default)]
pub struct TransferProofRegistry {
    consumed_proofs: HashSet<[u8; 32]>,
}

impl TransferProofRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_consumed(&self, proof_id: &[u8; 32]) -> bool {
        self.consumed_proofs.contains(proof_id)
    }

    pub fn consume(
        &mut self,
        proof: &CrossContractTransferProof,
        contract_id: ResourceId,
        block_height: u64,
        transaction_hash: [u8; 32],
    ) -> Result<(), Box<ConstitutionalEvidence>> {
        if self.consumed_proofs.contains(&proof.proof_id) {
            return Err(Box::new(ConstitutionalEvidence::ConstitutionalViolation {
                law: "X1".into(),
                resource_ids: vec![proof.consumed_resource_id],
                contract_id,
                block_height,
                transaction_hash,
            }));
        }
        self.consumed_proofs.insert(proof.proof_id);
        Ok(())
    }

    pub fn total_consumed(&self) -> usize {
        self.consumed_proofs.len()
    }

    pub fn mark_consumed(&mut self, proof_id: [u8; 32]) {
        self.consumed_proofs.insert(proof_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_resource_core::ResourceId;

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32]; h[0] = seed; ResourceId(h)
    }

    #[test]
    fn w11_register_and_consume_proof() {
        let mut registry = TransferProofRegistry::new();
        let proof = CrossContractTransferProof::new(
            make_id(1), [1u8; 32], [2u8; 32], 42, [0xaa; 32], [0xbb; 32],
        );
        assert!(registry.consume(&proof, make_id(99), 1, [0xcc; 32]).is_ok());
        assert_eq!(registry.total_consumed(), 1);
        assert!(registry.is_consumed(&proof.proof_id));
    }

    #[test]
    fn w11_reject_double_consumption() {
        let mut registry = TransferProofRegistry::new();
        let proof = CrossContractTransferProof::new(
            make_id(1), [1u8; 32], [2u8; 32], 42, [0xaa; 32], [0xbb; 32],
        );
        registry.consume(&proof, make_id(99), 1, [0xcc; 32]).unwrap();
        let result = registry.consume(&proof, make_id(99), 2, [0xdd; 32]);
        assert!(result.is_err());
    }

    #[test]
    fn w11_double_consumption_produces_x1_evidence() {
        let mut registry = TransferProofRegistry::new();
        let proof = CrossContractTransferProof::new(
            make_id(1), [1u8; 32], [2u8; 32], 42, [0xaa; 32], [0xbb; 32],
        );
        registry.consume(&proof, make_id(99), 1, [0xcc; 32]).unwrap();
        let err = registry.consume(&proof, make_id(99), 2, [0xdd; 32]).unwrap_err();
        match *err {
            ConstitutionalEvidence::ConstitutionalViolation { law, .. } => {
                assert_eq!(law, "X1");
            }
            _ => panic!("Expected X1 violation"),
        }
    }

    #[test]
    fn w11_proof_id_deterministic() {
        let p1 = CrossContractTransferProof::new(
            make_id(1), [1u8; 32], [2u8; 32], 42, [0xaa; 32], [0xbb; 32],
        );
        let p2 = CrossContractTransferProof::new(
            make_id(1), [1u8; 32], [2u8; 32], 42, [0xaa; 32], [0xbb; 32],
        );
        assert_eq!(p1.proof_id, p2.proof_id);
    }
}
