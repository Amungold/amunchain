use amun_resource_core::{
    ResourceId, ResourceMetadata, ResourceState,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Handle = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferAction {
    Consume { terminal_state: ResourceState },
    Produce { metadata: ResourceMetadata },
    Reference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationRecord {
    pub opcode: String,
    pub inputs: Vec<Handle>,
    pub outputs: Vec<Handle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VMEvidence {
    ExecutionFailure { reason: String },
    ConstitutionalViolation { law: String, resource_ids: Vec<ResourceId> },
    InvariantViolation { obligation_id: String },
}

#[derive(Debug, Clone, Default)]
pub struct PendingBuffer {
    resources: HashMap<Handle, (ResourceId, BufferAction)>,
    next_handle: Handle,
    operation_log: Vec<OperationRecord>,
    evidence: Vec<VMEvidence>,
    pre_state: HashMap<Handle, ResourceMetadata>,
}

impl PendingBuffer {
    pub fn new(pre_state_resources: Vec<ResourceMetadata>) -> Self {
        let mut buffer = Self::default();
        for (i, meta) in pre_state_resources.into_iter().enumerate() {
            buffer.resources.insert(
                i as Handle,
                (meta.resource_id, BufferAction::Reference),
            );
            buffer.pre_state.insert(i as Handle, meta);
        }
        buffer.next_handle = buffer.resources.len() as Handle;
        buffer
    }

    pub fn register_consumption(
        &mut self,
        handle: Handle,
        terminal_state: ResourceState,
    ) -> Result<(), String> {
        let entry = self.resources.get_mut(&handle)
            .ok_or_else(|| format!("handle {} not found", handle))?;
        entry.1 = BufferAction::Consume { terminal_state };
        Ok(())
    }

    pub fn register_production(&mut self, metadata: ResourceMetadata) -> Handle {
        let handle = self.next_handle;
        self.next_handle += 1;
        self.resources.insert(
            handle,
            (metadata.resource_id, BufferAction::Produce { metadata }),
        );
        handle
    }

    pub fn record_operation(&mut self, opcode: &str, inputs: Vec<Handle>, outputs: Vec<Handle>) {
        self.operation_log.push(OperationRecord {
            opcode: opcode.to_string(),
            inputs,
            outputs,
        });
    }

    pub fn record_evidence(&mut self, evidence: VMEvidence) {
        self.evidence.push(evidence);
    }

    pub fn get_metadata(&self, handle: Handle) -> Option<&ResourceMetadata> {
        self.resources.get(&handle).and_then(|(_, action)| match action {
            BufferAction::Produce { metadata } => Some(metadata),
            BufferAction::Reference | BufferAction::Consume { .. } => {
                self.pre_state.get(&handle)
            }
        })
    }

    pub fn produced_resources(&self) -> Vec<&ResourceMetadata> {
        self.resources.values()
            .filter_map(|(_, action)| match action {
                BufferAction::Produce { metadata } => Some(metadata),
                _ => None,
            })
            .collect()
    }

    pub fn consumed_handles(&self) -> Vec<(Handle, ResourceId, ResourceState)> {
        self.resources.iter()
            .filter_map(|(handle, (rid, action))| match action {
                BufferAction::Consume { terminal_state } => {
                    Some((*handle, *rid, terminal_state.clone()))
                }
                _ => None,
            })
            .collect()
    }

    pub fn operation_log(&self) -> Vec<OperationRecord> {
        self.operation_log.clone()
    }

    pub fn all_evidence(&self) -> Vec<VMEvidence> {
        self.evidence.clone()
    }

    pub fn has_evidence(&self) -> bool {
        !self.evidence.is_empty()
    }

    pub fn evidence_count(&self) -> usize {
        self.evidence.len()
    }

    pub fn operation_count(&self) -> usize {
        self.operation_log.len()
    }
}
