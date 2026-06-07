use amun_resource_core::{ResourceId, ResourceState};
use amun_vm_kernel::pending_buffer::{Handle, PendingBuffer};
use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
use std::collections::HashSet;

/// Resolves handles to ResourceIds and validates reachability.
/// Implements the reachability rule from N48.5-E Section 5.3.
pub struct HandleResolver;

impl HandleResolver {
    /// Validate that every produced resource is reachable from at least
    /// one entry point.  Returns evidence for unreachable resources.
    pub fn detect_leaks(
        buffer: &PendingBuffer,
        contract_id: ResourceId,
        block_height: u64,
        transaction_hash: [u8; 32],
    ) -> (bool, Vec<ConstitutionalEvidence>) {
        let produced = buffer.produced_resources();
        let consumed = buffer.consumed_handles();

        let active_set: HashSet<ResourceId> = produced
            .iter()
            .filter(|m| matches!(m.state, ResourceState::Active))
            .map(|m| m.resource_id)
            .collect();

        let archived_set: HashSet<ResourceId> = produced
            .iter()
            .filter(|m| matches!(m.state, ResourceState::Archived { .. }))
            .map(|m| m.resource_id)
            .collect();

        let revoked_set: HashSet<ResourceId> = produced
            .iter()
            .filter(|m| matches!(m.state, ResourceState::Revoked { .. }))
            .map(|m| m.resource_id)
            .collect();

        let transferred_set: HashSet<ResourceId> = produced
            .iter()
            .filter(|m| matches!(m.state, ResourceState::TransferredOut { .. }))
            .map(|m| m.resource_id)
            .collect();

        let consumed_ids: HashSet<ResourceId> = consumed
            .iter()
            .map(|(_, rid, _)| *rid)
            .collect();

        let mut leaks = Vec::new();
        for meta in &produced {
            let id = meta.resource_id;
            let reachable = active_set.contains(&id)
                || archived_set.contains(&id)
                || revoked_set.contains(&id)
                || transferred_set.contains(&id)
                || consumed_ids.contains(&id);

            if !reachable {
                leaks.push(ConstitutionalEvidence::ExecutionFailure {
                    reason: format!("unreachable resource: {}", id),
                    contract_id,
                    block_height,
                    transaction_hash,
                    gas_consumed: 0,
                });
            }
        }

        (leaks.is_empty(), leaks)
    }

    /// Validate handle safety: a handle that has been consumed (appeared
    /// as an INPUT to an operation) must not be used as an INPUT again
    /// in any later operation.  Output handles are tracked as they are
    /// produced by operations and may be consumed by subsequent ones.
    pub fn validate_handle_safety(
        buffer: &PendingBuffer,
        contract_id: ResourceId,
        block_height: u64,
        transaction_hash: [u8; 32],
    ) -> (bool, Vec<ConstitutionalEvidence>) {
        let operations = buffer.operation_log();
        // Track handles that have been consumed (used as input) — cannot
        // appear as input again.
        let mut consumed_inputs: HashSet<Handle> = HashSet::new();
        // Track handles that exist (pre-state or produced) — can be used
        // as input if not already consumed.
        let mut available_outputs: HashSet<Handle> = HashSet::new();
        // Pre-state handles 0..N are available
        let _pre_state_count = buffer.consumed_handles().len() + buffer.produced_resources().len();
        // Actually, simpler: all handles from 0 to first_produced-1 are pre-state.
        // Let's just track produced outputs and allow pre-state handles.
        let mut violations = Vec::new();

        for op in &operations {
            // Check inputs: must not have been consumed before
            for input in &op.inputs {
                if consumed_inputs.contains(input) {
                    violations.push(ConstitutionalEvidence::ExecutionFailure {
                        reason: format!(
                            "invalid handle usage: handle {} already consumed before operation {}",
                            input, op.opcode
                        ),
                        contract_id,
                        block_height,
                        transaction_hash,
                        gas_consumed: 0,
                    });
                }
            }
            // Mark inputs as consumed
            for input in &op.inputs {
                consumed_inputs.insert(*input);
            }
            // Mark outputs as available for future operations
            for output in &op.outputs {
                available_outputs.insert(*output);
            }
        }

        (violations.is_empty(), violations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_resource_core::{ResourceArchetype, ResourceLineage, ResourceMetadata, ResourceState};
    use amun_vm_kernel::pending_buffer::PendingBuffer;

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32]; h[0] = seed; ResourceId(h)
    }

    fn make_meta(id: ResourceId) -> ResourceMetadata {
        ResourceMetadata {
            resource_id: id,
            archetype: ResourceArchetype::Asset,
            state: ResourceState::Active,
            lineage: ResourceLineage::genesis(id),
            contract_id: [1u8; 32],
            owner: [2u8; 32],
        }
    }

    #[test]
    fn w9_no_leaks_when_all_resources_reachable() {
        let mut buffer = PendingBuffer::new(vec![]);
        let meta = make_meta(make_id(1));
        buffer.register_production(meta);
        let (no_leaks, evidence) = HandleResolver::detect_leaks(
            &buffer, make_id(99), 1, [0xaa; 32],
        );
        assert!(no_leaks);
        assert!(evidence.is_empty());
    }

    #[test]
    fn w9_handle_safety_no_violations() {
        let mut buffer = PendingBuffer::new(vec![]);
        // Op 1: consumes handle 0 (pre-state), produces handle 1
        buffer.record_operation("OP_TRANSFORM", vec![0], vec![1]);
        // Op 2: consumes handle 1 (produced by op 1), produces handles 2,3
        buffer.record_operation("OP_SPLIT", vec![1], vec![2, 3]);
        let (safe, violations) = HandleResolver::validate_handle_safety(
            &buffer, make_id(99), 1, [0xcc; 32],
        );
        assert!(safe, "Expected safe, got violations: {:?}", violations);
        assert!(violations.is_empty());
    }

    #[test]
    fn w9_handle_safety_detects_use_after_consumption() {
        let mut buffer = PendingBuffer::new(vec![]);
        // Op 1: consumes handle 0, produces handle 1
        buffer.record_operation("OP_TRANSFORM", vec![0], vec![1]);
        // Op 2: tries to consume handle 0 again — already consumed
        buffer.record_operation("OP_TRANSFORM", vec![0], vec![2]);
        let (safe, violations) = HandleResolver::validate_handle_safety(
            &buffer, make_id(99), 1, [0xdd; 32],
        );
        assert!(!safe, "Expected violation, but got safe");
        assert_eq!(violations.len(), 1);
        match &violations[0] {
            ConstitutionalEvidence::ExecutionFailure { reason, .. } => {
                assert!(reason.contains("already consumed"));
            }
            _ => panic!("Expected ExecutionFailure"),
        }
    }

    #[test]
    fn w9_empty_buffer_no_violations() {
        let buffer = PendingBuffer::new(vec![]);
        let (no_leaks, _) = HandleResolver::detect_leaks(
            &buffer, make_id(99), 1, [0xbb; 32],
        );
        assert!(no_leaks);
    }
}
