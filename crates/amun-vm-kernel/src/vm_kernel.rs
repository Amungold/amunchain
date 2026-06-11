use amun_resource_core::{RegistryError, ResourceMetadata, ResourceRegistry, TransformationMatrix};

use crate::execution_context::ExecutionContext;
use crate::pending_buffer::{PendingBuffer, VMEvidence};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommitResult {
    Committed {
        post_state_root: [u8; 32],
        produced_count: usize,
        consumed_count: usize,
    },
    Rejected {
        violations: Vec<String>,
        evidence_count: usize,
    },
}

pub struct VMKernel;

impl VMKernel {
    pub fn execute(
        _ctx: &ExecutionContext,
        pre_state: Vec<ResourceMetadata>,
        _tx_data: &[u8],
    ) -> PendingBuffer {
        PendingBuffer::new(pre_state)
    }

    pub fn verify(buffer: &mut PendingBuffer, registry: &ResourceRegistry) -> bool {
        let produced: Vec<ResourceMetadata> = buffer
            .produced_resources()
            .iter()
            .map(|m| (*m).clone())
            .collect();

        let mut violations: Vec<VMEvidence> = Vec::new();
        let mut all_passed = true;

        for meta in &produced {
            if registry.contains(&meta.resource_id) {
                violations.push(VMEvidence::ConstitutionalViolation {
                    law: "R1".into(),
                    resource_ids: vec![meta.resource_id],
                });
                all_passed = false;
                continue;
            }

            if !meta.lineage.parent_resource_ids.is_empty() {
                let parent_id = &meta.lineage.parent_resource_ids[0];
                if let Some(parent) = registry.get(parent_id) {
                    if TransformationMatrix::is_terminal(parent.archetype) {
                        violations.push(VMEvidence::ConstitutionalViolation {
                            law: "T1-terminal".into(),
                            resource_ids: vec![meta.resource_id, *parent_id],
                        });
                        all_passed = false;
                        continue;
                    }
                    if !TransformationMatrix::is_legal(parent.archetype, meta.archetype) {
                        violations.push(VMEvidence::ConstitutionalViolation {
                            law: "T1".into(),
                            resource_ids: vec![meta.resource_id, *parent_id],
                        });
                        all_passed = false;
                        continue;
                    }
                    if meta.lineage.version != parent.lineage.version + 1 {
                        violations.push(VMEvidence::ConstitutionalViolation {
                            law: "R6".into(),
                            resource_ids: vec![meta.resource_id, *parent_id],
                        });
                        all_passed = false;
                        continue;
                    }
                    let actual_hash = amun_resource_core::ResourceRegistry::hash_resource(parent);
                    if meta.lineage.parent_hashes.len() == 1
                        && meta.lineage.parent_hashes[0] != actual_hash
                    {
                        violations.push(VMEvidence::ConstitutionalViolation {
                            law: "L5".into(),
                            resource_ids: vec![meta.resource_id, *parent_id],
                        });
                        all_passed = false;
                        continue;
                    }
                } else {
                    violations.push(VMEvidence::ConstitutionalViolation {
                        law: "L2".into(),
                        resource_ids: vec![meta.resource_id],
                    });
                    all_passed = false;
                    continue;
                }
            }
        }

        for ev in violations {
            buffer.record_evidence(ev);
        }

        all_passed
    }

    pub fn commit(
        buffer: &PendingBuffer,
        registry: &mut ResourceRegistry,
    ) -> Result<CommitResult, RegistryError> {
        let produced: Vec<ResourceMetadata> = buffer
            .produced_resources()
            .iter()
            .map(|m| (*m).clone())
            .collect();
        let consumed = buffer.consumed_handles();

        for meta in &produced {
            if meta.lineage.parent_resource_ids.is_empty() {
                registry.register_genesis(meta.clone())?;
            }
        }

        for meta in &produced {
            if !meta.lineage.parent_resource_ids.is_empty() {
                let parent_id = &meta.lineage.parent_resource_ids[0];
                registry.consume_and_derive(parent_id, meta.clone())?;
            }
        }

        let post_state_root = registry.compute_state_root();

        Ok(CommitResult::Committed {
            post_state_root,
            produced_count: produced.len(),
            consumed_count: consumed.len(),
        })
    }
}
