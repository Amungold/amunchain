#[cfg(test)]
use crate::wasm_profile::*;

#[test]
fn test_gas_table_default() {
    let table = InstructionGasTable::default();
    assert_eq!(table.i32_add, 1);
    assert_eq!(table.i64_add, 1);
    assert_eq!(table.memory_load, 3);
    assert_eq!(table.call, 5);
}

#[test]
fn test_gas_table_new_consistent() {
    let t1 = InstructionGasTable::new();
    let t2 = InstructionGasTable::default();
    assert_eq!(t1.i32_add, t2.i32_add);
    assert_eq!(t1.memory_store, t2.memory_store);
}

#[test]
fn test_wasm_profile_constitutional() {
    let profile = DeterministicWasmProfile::constitutional();
    assert_eq!(profile.max_initial_pages, 256);
    assert_eq!(profile.max_maximum_pages, 256);
    assert_eq!(profile.total_gas_limit, 10_000_000);
    assert_eq!(profile.max_call_depth, 64);
}

#[test]
fn test_wasm_profile_verify_module_ok() {
    let profile = DeterministicWasmProfile::constitutional();
    assert!(profile.verify_module(b"").is_ok());
}

#[test]
fn test_canonical_nan_constants() {
    assert_eq!(CANONICAL_NAN_32, 0x7FC00000);
    assert_eq!(CANONICAL_NAN_64, 0x7FF8000000000000);
}

#[test]
fn test_allowed_instructions_not_empty() {
    use crate::wasm_deterministic::wasm_deterministic_subset;
    assert!(!wasm_deterministic_subset::ALLOWED_INSTRUCTIONS.is_empty());
    assert!(!wasm_deterministic_subset::FORBIDDEN_INSTRUCTIONS.is_empty());
}

#[test]
fn test_verify_deterministic_wasm_ok() {
    use crate::wasm_deterministic::wasm_deterministic_subset;
    assert!(wasm_deterministic_subset::verify_deterministic_wasm(b"").is_ok());
}

#[test]
fn test_verified_interpreter_new() {
    let profile = DeterministicWasmProfile::constitutional();
    let interpreter = crate::verified_interpreter::VerifiedInterpreter::new(profile);
    assert!(interpreter.execute(b"", "", &[]).is_ok());
}
