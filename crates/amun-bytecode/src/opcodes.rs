use serde::{Deserialize, Serialize};

/// Constitutional bytecode opcodes as defined in N48.5-E Section 3.2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpCode {
    // Resource operations
    Split   { handle: u32, amount_count: u32 },
    Merge   { handle_count: u32 },
    Transform { src_handle: u32, type_idx: u32 },
    Consume { src_handle: u32, type_idx: u32 },
    Archive { handle: u32 },
    Revoke  { handle: u32, reason_ptr: u32 },
    // Constitutional operations
    CheckInvariant { invariant_idx: u32 },
    EmitClaim { obligation_id_ptr: u32 },
    // Stack operations
    Push(u64),
    Pop,
    Dup(u32),
    Swap(u32),
    // Control flow
    Jump(i32),
    JumpIfZero(i32),
    JumpIfNonZero(i32),
    // VM control
    Return,
    Halt,
}

impl OpCode {
    /// Human-readable name for the opcode (used in operation log).
    pub fn name(&self) -> &str {
        match self {
            OpCode::Split { .. } => "OP_SPLIT",
            OpCode::Merge { .. } => "OP_MERGE",
            OpCode::Transform { .. } => "OP_TRANSFORM",
            OpCode::Consume { .. } => "OP_CONSUME",
            OpCode::Archive { .. } => "OP_ARCHIVE",
            OpCode::Revoke { .. } => "OP_REVOKE",
            OpCode::CheckInvariant { .. } => "OP_CHECK_INVARIANT",
            OpCode::EmitClaim { .. } => "OP_EMIT_CLAIM",
            OpCode::Push(_) => "OP_PUSH",
            OpCode::Pop => "OP_POP",
            OpCode::Dup(_) => "OP_DUP",
            OpCode::Swap(_) => "OP_SWAP",
            OpCode::Jump(_) => "OP_JUMP",
            OpCode::JumpIfZero(_) => "OP_JUMP_IF_ZERO",
            OpCode::JumpIfNonZero(_) => "OP_JUMP_IF_NON_ZERO",
            OpCode::Return => "OP_RETURN",
            OpCode::Halt => "OP_HALT",
        }
    }

    /// Base gas cost for this opcode.
    pub fn base_gas(&self) -> u64 {
        match self {
            OpCode::Split { amount_count, .. } => 10 + (*amount_count as u64 * 5),
            OpCode::Merge { handle_count } => 10 + (*handle_count as u64 * 5),
            OpCode::Transform { .. } => 15,
            OpCode::Consume { .. } => 15,
            OpCode::Archive { .. } => 20,
            OpCode::Revoke { .. } => 25,
            OpCode::CheckInvariant { .. } => 50,
            OpCode::EmitClaim { .. } => 30,
            OpCode::Push(_) => 1,
            OpCode::Pop => 1,
            OpCode::Dup(_) => 1,
            OpCode::Swap(_) => 2,
            OpCode::Jump(_) => 2,
            OpCode::JumpIfZero(_) => 3,
            OpCode::JumpIfNonZero(_) => 3,
            OpCode::Return => 1,
            OpCode::Halt => 0,
        }
    }

    /// Whether this opcode is a resource operation (produces/consumes resources).
    pub fn is_resource_op(&self) -> bool {
        matches!(
            self,
            OpCode::Split { .. }
                | OpCode::Merge { .. }
                | OpCode::Transform { .. }
                | OpCode::Consume { .. }
                | OpCode::Archive { .. }
                | OpCode::Revoke { .. }
        )
    }

    /// Whether this opcode is a constitutional operation.
    pub fn is_constitutional_op(&self) -> bool {
        matches!(
            self,
            OpCode::CheckInvariant { .. } | OpCode::EmitClaim { .. }
        )
    }
}
