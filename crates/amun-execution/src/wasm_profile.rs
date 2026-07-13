// Deterministic WASM profile with fixed gas costs and memory limits.

#[derive(Clone, Debug)]
pub struct InstructionGasTable {
    pub i32_add: u64,
    pub i64_add: u64,
    pub i32_mul: u64,
    pub i64_mul: u64,
    pub memory_load: u64,
    pub memory_store: u64,
    pub call: u64,
    pub call_indirect: u64,
    pub branch: u64,
    pub if_else: u64,
    pub drop: u64,
    pub select: u64,
    pub local_get: u64,
    pub local_set: u64,
    pub global_get: u64,
    pub global_set: u64,
}

impl InstructionGasTable {
    pub const fn new() -> Self {
        Self {
            i32_add: 1,
            i64_add: 1,
            i32_mul: 2,
            i64_mul: 2,
            memory_load: 3,
            memory_store: 3,
            call: 5,
            call_indirect: 8,
            branch: 1,
            if_else: 2,
            drop: 1,
            select: 1,
            local_get: 1,
            local_set: 1,
            global_get: 1,
            global_set: 1,
        }
    }
}

impl Default for InstructionGasTable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct DeterministicWasmProfile {
    pub max_initial_pages: u32,
    pub max_maximum_pages: u32,
    pub instruction_gas_costs: InstructionGasTable,
    pub total_gas_limit: u64,
    pub max_call_depth: u32,
    pub max_locals: u32,
    pub max_params: u32,
    pub max_returns: u32,
}

impl DeterministicWasmProfile {
    pub const fn constitutional() -> Self {
        Self {
            max_initial_pages: 256,
            max_maximum_pages: 256,
            instruction_gas_costs: InstructionGasTable::new(),
            total_gas_limit: 10_000_000,
            max_call_depth: 64,
            max_locals: 256,
            max_params: 16,
            max_returns: 4,
        }
    }

    pub fn verify_module(&self, _module: &[u8]) -> Result<(), &'static str> {
        Ok(())
    }
}

pub const CANONICAL_NAN_32: u32 = 0x7FC00000;
pub const CANONICAL_NAN_64: u64 = 0x7FF8000000000000;
