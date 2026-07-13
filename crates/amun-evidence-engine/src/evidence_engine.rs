use amun_resource_core::ResourceId;
use amun_vm_kernel::pending_buffer::VMEvidence;

use crate::evidence_archive::VMEvidenceArchive;
use crate::evidence_types::ConstitutionalEvidence;

pub struct EvidenceEngine;

impl EvidenceEngine {
    pub fn convert(
        vm_evidence: &VMEvidence,
        contract_id: ResourceId,
        block_height: u64,
        transaction_hash: [u8; 32],
    ) -> ConstitutionalEvidence {
        match vm_evidence {
            VMEvidence::ExecutionFailure { reason } => ConstitutionalEvidence::ExecutionFailure {
                reason: reason.clone(),
                contract_id,
                block_height,
                transaction_hash,
                gas_consumed: 0,
            },
            VMEvidence::ConstitutionalViolation { law, resource_ids } => {
                ConstitutionalEvidence::ConstitutionalViolation {
                    law: law.clone(),
                    resource_ids: resource_ids.clone(),
                    contract_id,
                    block_height,
                    transaction_hash,
                }
            }
            VMEvidence::InvariantViolation { obligation_id } => {
                ConstitutionalEvidence::InvariantViolation {
                    obligation_id: obligation_id.clone(),
                    contract_id,
                    block_height,
                    transaction_hash,
                    state_root: [0u8; 32],
                }
            }
        }
    }

    pub fn archive(evidence: ConstitutionalEvidence, archive: &mut VMEvidenceArchive) -> [u8; 32] {
        archive.insert(evidence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_resource_core::ResourceId;

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32];
        h[0] = seed;
        ResourceId(h)
    }

    #[test]
    fn w4_convert_execution_failure() {
        let vm_ev = VMEvidence::ExecutionFailure {
            reason: "out of gas".into(),
        };
        let contract_id = make_id(1);
        let tx_hash = [0xaa; 32];
        let evidence = EvidenceEngine::convert(&vm_ev, contract_id, 42, tx_hash);
        match evidence {
            ConstitutionalEvidence::ExecutionFailure {
                reason,
                block_height,
                ..
            } => {
                assert_eq!(reason, "out of gas");
                assert_eq!(block_height, 42);
            }
            _ => panic!("Expected ExecutionFailure"),
        }
    }

    #[test]
    fn w4_convert_constitutional_violation() {
        let vm_ev = VMEvidence::ConstitutionalViolation {
            law: "R1".into(),
            resource_ids: vec![make_id(1), make_id(2)],
        };
        let evidence = EvidenceEngine::convert(&vm_ev, make_id(3), 10, [0xbb; 32]);
        match evidence {
            ConstitutionalEvidence::ConstitutionalViolation {
                law, resource_ids, ..
            } => {
                assert_eq!(law, "R1");
                assert_eq!(resource_ids.len(), 2);
            }
            _ => panic!("Expected ConstitutionalViolation"),
        }
    }

    #[test]
    fn w4_archive_and_retrieve() {
        let mut archive = VMEvidenceArchive::new();
        let evidence = ConstitutionalEvidence::ExecutionFailure {
            reason: "stack overflow".into(),
            contract_id: make_id(1),
            block_height: 5,
            transaction_hash: [0xcc; 32],
            gas_consumed: 1000,
        };
        let id = evidence.evidence_id();
        archive.insert(evidence);
        assert_eq!(archive.total(), 1);
        assert!(archive.get(&id).is_some());
        assert_eq!(archive.failures().len(), 1);
    }

    #[test]
    fn w4_evidence_id_deterministic() {
        let e1 = ConstitutionalEvidence::ExecutionFailure {
            reason: "test".into(),
            contract_id: make_id(1),
            block_height: 1,
            transaction_hash: [0xdd; 32],
            gas_consumed: 0,
        };
        let e2 = ConstitutionalEvidence::ExecutionFailure {
            reason: "test".into(),
            contract_id: make_id(1),
            block_height: 1,
            transaction_hash: [0xdd; 32],
            gas_consumed: 0,
        };
        assert_eq!(e1.evidence_id(), e2.evidence_id());
    }

    #[test]
    fn w4_violation_and_failure_categories() {
        let mut archive = VMEvidenceArchive::new();
        archive.insert(ConstitutionalEvidence::ExecutionFailure {
            reason: "oog".into(),
            contract_id: make_id(1),
            block_height: 1,
            transaction_hash: [0x11; 32],
            gas_consumed: 0,
        });
        archive.insert(ConstitutionalEvidence::ConstitutionalViolation {
            law: "R1".into(),
            resource_ids: vec![],
            contract_id: make_id(1),
            block_height: 1,
            transaction_hash: [0x22; 32],
        });
        archive.insert(ConstitutionalEvidence::InvariantViolation {
            obligation_id: "SAFETY-001".into(),
            contract_id: make_id(1),
            block_height: 1,
            transaction_hash: [0x33; 32],
            state_root: [0u8; 32],
        });
        assert_eq!(archive.total(), 3);
        assert_eq!(archive.failures().len(), 1);
        assert_eq!(archive.violations().len(), 1);
    }
}
