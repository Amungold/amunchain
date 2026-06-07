use serde::{Deserialize, Serialize};

use crate::opcodes::OpCode;

/// A compiled constitutional contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstitutionalProgram {
    /// Capability level (L0-L4).
    pub level: u8,
    /// Number of invariants declared.
    pub invariant_count: u32,
    /// Entry point offset in the instruction stream.
    pub entry_point: u32,
    /// The compiled instruction stream.
    pub code: Vec<OpCode>,
    /// Program hash (Blake3 of the code).
    pub program_hash: [u8; 32],
}

impl ConstitutionalProgram {
    pub fn new(level: u8, invariant_count: u32, entry_point: u32, code: Vec<OpCode>) -> Self {
        let mut program = Self {
            level,
            invariant_count,
            entry_point,
            code,
            program_hash: [0u8; 32],
        };
        program.program_hash = program.compute_hash();
        program
    }

    pub fn compute_hash(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AMUN_PROGRAM_V1");
        hasher.update(&[self.level]);
        hasher.update(&self.invariant_count.to_le_bytes());
        hasher.update(&self.entry_point.to_le_bytes());
        for op in &self.code {
            hasher.update(&[op.base_gas() as u8]);
            hasher.update(op.name().as_bytes());
        }
        let hash = hasher.finalize();
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(hash.as_bytes());
        bytes
    }

    pub fn verify(&self) -> bool {
        self.program_hash == self.compute_hash()
    }

    pub fn instruction_count(&self) -> usize {
        self.code.len()
    }

    pub fn total_gas_estimate(&self) -> u64 {
        self.code.iter().map(|op| op.base_gas()).sum()
    }

    pub fn resource_ops(&self) -> Vec<&OpCode> {
        self.code.iter().filter(|op| op.is_resource_op()).collect()
    }

    pub fn constitutional_ops(&self) -> Vec<&OpCode> {
        self.code.iter().filter(|op| op.is_constitutional_op()).collect()
    }
}
