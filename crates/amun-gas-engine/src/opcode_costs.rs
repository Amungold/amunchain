use amun_bytecode::opcodes::OpCode;

/// Constitutional gas cost table as specified in N48.5-E Section 3.3.
pub struct OpCodeCosts;

impl OpCodeCosts {
    /// Compute the gas cost for a given opcode.
    /// Resource operations carry additional costs proportional
    /// to the complexity of the constitutional checks they trigger.
    pub fn cost(op: &OpCode) -> u64 {
        match op {
            OpCode::Split { amount_count, .. } => {
                10u64.saturating_add((*amount_count as u64).saturating_mul(5))
            }
            OpCode::Merge { handle_count } => {
                10u64.saturating_add((*handle_count as u64).saturating_mul(5))
            }
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

    /// Estimated total gas for a program.
    pub fn estimate(ops: &[OpCode]) -> u64 {
        ops.iter().map(Self::cost).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn w7_split_cost_proportional() {
        let split_2 = OpCode::Split { handle: 0, amount_count: 2 };
        let split_10 = OpCode::Split { handle: 0, amount_count: 10 };
        let cost_2 = OpCodeCosts::cost(&split_2);
        let cost_10 = OpCodeCosts::cost(&split_10);
        assert!(cost_10 > cost_2);
    }

    #[test]
    fn w7_merge_cost_proportional() {
        let merge_2 = OpCode::Merge { handle_count: 2 };
        let merge_10 = OpCode::Merge { handle_count: 10 };
        assert!(OpCodeCosts::cost(&merge_10) > OpCodeCosts::cost(&merge_2));
    }

    #[test]
    fn w7_simple_ops_cheap() {
        assert_eq!(OpCodeCosts::cost(&OpCode::Push(1)), 1);
        assert_eq!(OpCodeCosts::cost(&OpCode::Halt), 0);
    }

    #[test]
    fn w7_estimate_accumulates() {
        let ops = vec![
            OpCode::Push(1),
            OpCode::Push(2),
            OpCode::Transform { src_handle: 0, type_idx: 0 },
            OpCode::Halt,
        ];
        let total = OpCodeCosts::estimate(&ops);
        assert_eq!(total, 1 + 1 + 15);
    }
}
