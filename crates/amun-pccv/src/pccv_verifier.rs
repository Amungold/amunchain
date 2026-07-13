use crate::enhanced_proof::EnhancedTransitionProof;
use amun_resource_core::{
    ResourceArchetype, ResourceRegistry, ResourceState, TransformationMatrix,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PCCVResult {
    Verified {
        post_state_root: [u8; 32],
        evidence_count: usize,
    },
    Failed {
        reason: String,
    },
}

pub struct PCCVVerifier;

impl PCCVVerifier {
    pub fn verify(proof: &EnhancedTransitionProof, _registry: &ResourceRegistry) -> PCCVResult {
        if proof.witness.is_empty() && !proof.consumed_resources.is_empty() {
            return PCCVResult::Failed {
                reason: "Consumed resources present but witness bundle is empty".into(),
            };
        }

        for merkle_proof in &proof.witness.consumed_proofs {
            if merkle_proof.state_root != proof.pre_state_root {
                return PCCVResult::Failed {
                    reason: "Merkle proof root mismatch".into(),
                };
            }
        }

        let mut seen_ids = std::collections::HashSet::new();
        for meta in &proof.produced_resources {
            if !seen_ids.insert(meta.resource_id) {
                return PCCVResult::Failed {
                    reason: format!("R1 violation: duplicate {}", meta.resource_id),
                };
            }
        }

        for cid in &proof.consumed_resources {
            if proof
                .produced_resources
                .iter()
                .any(|m| m.resource_id == *cid)
            {
                return PCCVResult::Failed {
                    reason: format!("R2 violation: {} consumed and produced", cid),
                };
            }
        }

        for meta in &proof.produced_resources {
            for pid in &meta.lineage.parent_resource_ids {
                if !proof.consumed_resources.contains(pid) {
                    return PCCVResult::Failed {
                        reason: format!("R3 violation: parent {} not consumed", pid),
                    };
                }
            }
        }

        for meta in &proof.produced_resources {
            if meta.lineage.parent_resource_ids.is_empty() {
                continue;
            }
            let parent_id = &meta.lineage.parent_resource_ids[0];
            if let Some(parent_meta) = proof
                .witness
                .produced_metadata
                .iter()
                .find(|m| m.resource_id == *parent_id)
            {
                if meta.lineage.version != parent_meta.lineage.version + 1 {
                    return PCCVResult::Failed {
                        reason: format!(
                            "R6: child v{} != parent v{} + 1",
                            meta.lineage.version, parent_meta.lineage.version
                        ),
                    };
                }
                if !TransformationMatrix::is_legal(parent_meta.archetype, meta.archetype) {
                    return PCCVResult::Failed {
                        reason: format!(
                            "T1: {:?} -> {:?} illegal",
                            parent_meta.archetype, meta.archetype
                        ),
                    };
                }
            }
        }

        for meta in &proof.produced_resources {
            if meta.archetype == ResourceArchetype::Certificate
                && !matches!(meta.state, ResourceState::Archived { .. })
            {
                return PCCVResult::Failed {
                    reason: "R4: Certificate not Archived".into(),
                };
            }
        }

        for ev in &proof.evidence {
            if ev.evidence_id() == [0u8; 32] {
                return PCCVResult::Failed {
                    reason: "Zero evidence ID".into(),
                };
            }
        }

        if Self::compute_proof_hash(proof) != proof.proof_hash {
            return PCCVResult::Failed {
                reason: "Proof hash mismatch".into(),
            };
        }

        let min_gas: u64 = proof
            .operation_log
            .iter()
            .map(|op| match op.opcode.as_str() {
                "OP_SPLIT" => 15,
                "OP_MERGE" => 15,
                "OP_TRANSFORM" => 15,
                "OP_CONSUME" => 15,
                "OP_ARCHIVE" => 20,
                "OP_REVOKE" => 25,
                "OP_PUSH" => 1,
                "OP_POP" => 1,
                "OP_HALT" => 0,
                _ => 5,
            })
            .sum();
        if proof.gas_used < min_gas {
            return PCCVResult::Failed {
                reason: format!("Gas {} < min {}", proof.gas_used, min_gas),
            };
        }

        PCCVResult::Verified {
            post_state_root: proof.post_state_root,
            evidence_count: proof.evidence.len(),
        }
    }

    pub fn compute_proof_hash(proof: &EnhancedTransitionProof) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_ENHANCED_PROOF_V1");
        hasher.update(&proof.transaction_hash);
        hasher.update(proof.contract_id.as_bytes());
        hasher.update(&proof.block_height.to_le_bytes());
        hasher.update(&proof.block_hash);
        hasher.update(&proof.pre_state_root);
        hasher.update(&proof.post_state_root);
        hasher.update(&proof.gas_used.to_le_bytes());
        for id in &proof.consumed_resources {
            hasher.update(id.as_bytes());
        }
        for meta in &proof.produced_resources {
            hasher.update(meta.resource_id.as_bytes());
            hasher.update(&meta.lineage.version.to_le_bytes());
        }
        for ev in &proof.evidence {
            hasher.update(&ev.evidence_id());
        }
        let wb = serde_json::to_vec(&proof.witness).unwrap_or_default();
        hasher.update(&wb);
        let hash = hasher.finalize();
        let mut h = [0u8; 32];
        h.copy_from_slice(hash.as_bytes());
        h
    }
}
