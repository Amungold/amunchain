use amun_resource_core::{
    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
};
use amun_vm_kernel::execution_context::ExecutionContext;
use amun_vm_kernel::pending_buffer::PendingBuffer;

use crate::opcodes::OpCode;
use crate::program::ConstitutionalProgram;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpreterResult {
    Success {
        gas_used: u64,
        resources_produced: usize,
        resources_consumed: usize,
    },
    Error {
        reason: String,
        gas_used: u64,
    },
    OutOfGas {
        gas_used: u64,
        gas_limit: u64,
    },
}

pub struct Interpreter {
    gas_limit: u64,
    gas_used: u64,
    stack: Vec<u64>,
    program_counter: usize,
    resource_counter: u64,
}

impl Interpreter {
    pub fn new(gas_limit: u64) -> Self {
        Self {
            gas_limit,
            gas_used: 0,
            stack: Vec::new(),
            program_counter: 0,
            resource_counter: 1000000,
        }
    }

    /// Generate a unique ResourceId for a produced resource within this transaction.
    fn next_resource_id(&mut self) -> ResourceId {
        self.resource_counter += 1;
        let mut h = [0u8; 32];
        h[0..8].copy_from_slice(&self.resource_counter.to_le_bytes());
        ResourceId(h)
    }

    pub fn execute(
        &mut self,
        program: &ConstitutionalProgram,
        _ctx: &ExecutionContext,
        pre_state: Vec<ResourceMetadata>,
    ) -> Result<(PendingBuffer, InterpreterResult), String> {
        if !program.verify() {
            return Err("Program hash verification failed".into());
        }

        let mut buffer = PendingBuffer::new(pre_state);
        self.program_counter = program.entry_point as usize;

        while self.program_counter < program.code.len() {
            let op = program.code[self.program_counter];

            let cost = op.base_gas();
            if self.gas_used + cost > self.gas_limit {
                return Ok((
                    buffer,
                    InterpreterResult::OutOfGas {
                        gas_used: self.gas_used,
                        gas_limit: self.gas_limit,
                    },
                ));
            }
            self.gas_used += cost;

            match op {
                OpCode::Split {
                    handle,
                    amount_count,
                } => {
                    self.check_stack(1)?;
                    let child_indices: Vec<u32> = (0..amount_count)
                        .map(|_| {
                            let id = self.next_resource_id();
                            buffer.register_production(ResourceMetadata {
                                resource_id: id,
                                archetype: ResourceArchetype::Asset,
                                state: ResourceState::Active,
                                lineage: ResourceLineage::genesis(id),
                                contract_id: [1u8; 32],
                                owner: [2u8; 32],
                            })
                        })
                        .collect();
                    buffer.record_operation(op.name(), vec![handle], child_indices.clone());
                    for h in child_indices {
                        self.stack.push(h as u64);
                    }
                }
                OpCode::Merge { handle_count } => {
                    self.check_stack(handle_count as usize)?;
                    let inputs: Vec<u32> = (0..handle_count)
                        .map(|_| self.stack.pop().unwrap() as u32)
                        .collect();
                    let id = self.next_resource_id();
                    let output = buffer.register_production(ResourceMetadata {
                        resource_id: id,
                        archetype: ResourceArchetype::Asset,
                        state: ResourceState::Active,
                        lineage: ResourceLineage::genesis(id),
                        contract_id: [1u8; 32],
                        owner: [2u8; 32],
                    });
                    buffer.record_operation(op.name(), inputs, vec![output]);
                    self.stack.push(output as u64);
                }
                OpCode::Transform { src_handle, .. } => {
                    let child_id = self.next_resource_id();
                    let output = buffer.register_production(ResourceMetadata {
                        resource_id: child_id,
                        archetype: ResourceArchetype::Asset,
                        state: ResourceState::Active,
                        lineage: ResourceLineage::genesis(child_id),
                        contract_id: [1u8; 32],
                        owner: [2u8; 32],
                    });
                    buffer
                        .register_consumption(
                            src_handle,
                            ResourceState::Consumed {
                                derived_children: vec![child_id],
                            },
                        )
                        .map_err(|e| format!("consume error: {}", e))?;
                    buffer.record_operation(op.name(), vec![src_handle], vec![output]);
                    self.stack.push(output as u64);
                }
                OpCode::Consume { src_handle, .. } => {
                    let child_id = self.next_resource_id();
                    let output = buffer.register_production(ResourceMetadata {
                        resource_id: child_id,
                        archetype: ResourceArchetype::Asset,
                        state: ResourceState::Active,
                        lineage: ResourceLineage::genesis(child_id),
                        contract_id: [1u8; 32],
                        owner: [2u8; 32],
                    });
                    buffer
                        .register_consumption(
                            src_handle,
                            ResourceState::Consumed {
                                derived_children: vec![child_id],
                            },
                        )
                        .map_err(|e| format!("consume error: {}", e))?;
                    buffer.record_operation(op.name(), vec![src_handle], vec![output]);
                    self.stack.push(output as u64);
                }
                OpCode::Archive { handle } => {
                    buffer
                        .register_consumption(handle, ResourceState::Archived { archive_height: 0 })
                        .map_err(|e| format!("archive error: {}", e))?;
                    buffer.record_operation(op.name(), vec![handle], vec![]);
                }
                OpCode::Revoke { handle, .. } => {
                    buffer
                        .register_consumption(
                            handle,
                            ResourceState::Revoked {
                                reason: "revoked".into(),
                            },
                        )
                        .map_err(|e| format!("revoke error: {}", e))?;
                    buffer.record_operation(op.name(), vec![handle], vec![]);
                }
                OpCode::Push(value) => {
                    self.stack.push(value);
                }
                OpCode::Pop => {
                    self.stack.pop();
                }
                OpCode::Dup(n) => {
                    let idx = self.stack.len().saturating_sub(1 + n as usize);
                    if let Some(&val) = self.stack.get(idx) {
                        self.stack.push(val);
                    }
                }
                OpCode::Swap(n) => {
                    let len = self.stack.len();
                    if len >= 2 {
                        let top = len - 1;
                        let other = len.saturating_sub(2 + n as usize);
                        self.stack.swap(top, other);
                    }
                }
                OpCode::Jump(offset) => {
                    self.program_counter = ((self.program_counter as i32) + offset) as usize;
                    continue;
                }
                OpCode::JumpIfZero(offset) => {
                    if self.stack.pop().unwrap_or(0) == 0 {
                        self.program_counter = ((self.program_counter as i32) + offset) as usize;
                        continue;
                    }
                }
                OpCode::JumpIfNonZero(offset) => {
                    if self.stack.pop().unwrap_or(0) != 0 {
                        self.program_counter = ((self.program_counter as i32) + offset) as usize;
                        continue;
                    }
                }
                OpCode::CheckInvariant { .. } => {
                    buffer.record_operation(op.name(), vec![], vec![]);
                }
                OpCode::EmitClaim { .. } => {
                    buffer.record_operation(op.name(), vec![], vec![]);
                }
                OpCode::Return => break,
                OpCode::Halt => break,
            }
            self.program_counter += 1;
        }

        let result = InterpreterResult::Success {
            gas_used: self.gas_used,
            resources_produced: (self.resource_counter - 1000000) as usize,
            resources_consumed: buffer.consumed_handles().len(),
        };
        Ok((buffer, result))
    }

    fn check_stack(&self, needed: usize) -> Result<(), String> {
        if self.stack.len() < needed {
            Err(format!(
                "stack underflow: need {}, have {}",
                needed,
                self.stack.len()
            ))
        } else {
            Ok(())
        }
    }
}
