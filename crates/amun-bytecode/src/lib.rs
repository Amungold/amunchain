pub mod opcodes;
pub mod program;
pub mod interpreter;

pub use opcodes::*;
pub use program::*;
pub use interpreter::*;

#[cfg(test)]
mod tests {
    use super::*;
    use amun_resource_core::ResourceId;
    use amun_vm_kernel::execution_context::ExecutionContext;

    fn make_id(seed: u8) -> ResourceId {
        let mut h = [0u8; 32]; h[0] = seed; ResourceId(h)
    }

    #[test]
    fn w6_program_hash_deterministic() {
        let code = vec![OpCode::Push(42), OpCode::Push(10), OpCode::Halt];
        let p1 = ConstitutionalProgram::new(1, 0, 0, code.clone());
        let p2 = ConstitutionalProgram::new(1, 0, 0, code);
        assert_eq!(p1.program_hash, p2.program_hash);
        assert!(p1.verify());
    }

    #[test]
    fn w6_execute_simple_program() {
        let code = vec![OpCode::Push(100), OpCode::Push(200), OpCode::Halt];
        let program = ConstitutionalProgram::new(1, 0, 0, code);
        let ctx = ExecutionContext {
            contract_id: make_id(1),
            caller: [1u8; 32],
            block_height: 1,
            block_hash: [0u8; 32],
            transaction_hash: [0xaa; 32],
            pre_state_root: [0u8; 32],
            authority: [2u8; 32],
        };
        let pre_state = vec![];
        let mut interpreter = Interpreter::new(10000);
        let (_buffer, result) = interpreter.execute(&program, &ctx, pre_state).unwrap();
        match result {
            InterpreterResult::Success { gas_used, .. } => {
                assert!(gas_used > 0);
                assert!(gas_used < 10000);
            }
            _ => panic!("Expected Success"),
        }
    }

    #[test]
    fn w6_execute_out_of_gas() {
        let code = vec![OpCode::Push(1); 1000];
        let program = ConstitutionalProgram::new(1, 0, 0, code);
        let ctx = ExecutionContext {
            contract_id: make_id(1),
            caller: [1u8; 32],
            block_height: 1,
            block_hash: [0u8; 32],
            transaction_hash: [0xbb; 32],
            pre_state_root: [0u8; 32],
            authority: [2u8; 32],
        };
        let pre_state = vec![];
        let mut interpreter = Interpreter::new(10);
        let (_, result) = interpreter.execute(&program, &ctx, pre_state).unwrap();
        assert!(matches!(result, InterpreterResult::OutOfGas { .. }));
    }

    #[test]
    fn w6_opcode_gas_costs() {
        assert_eq!(OpCode::Push(0).base_gas(), 1);
        assert_eq!(OpCode::Halt.base_gas(), 0);
        assert_eq!(OpCode::Transform { src_handle: 0, type_idx: 0 }.base_gas(), 15);
        assert_eq!(OpCode::Split { handle: 0, amount_count: 4 }.base_gas(), 10 + 20);
    }

    #[test]
    fn w6_resource_op_classification() {
        assert!(OpCode::Split { handle: 0, amount_count: 2 }.is_resource_op());
        assert!(OpCode::Transform { src_handle: 0, type_idx: 0 }.is_resource_op());
        assert!(!OpCode::Push(1).is_resource_op());
        assert!(!OpCode::Halt.is_resource_op());
    }
}
