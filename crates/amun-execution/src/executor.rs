use amun_kernel_types::*;
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use crate::gas::GasMeter;
use crate::wasm::VerifiedInterpreter;

pub struct ExecutionContext {
    pub block_height: u64,
    pub block_hash: PublicHash32,
    pub gas_meter: GasMeter,
    pub interpreter: VerifiedInterpreter,
}

impl ExecutionContext {
    pub fn new(block_height: u64, block_hash: PublicHash32, gas_limit: u64) -> Self {
        Self { block_height, block_hash, gas_meter: GasMeter::new(gas_limit), interpreter: VerifiedInterpreter::new() }
    }

    pub fn execute_wasm(&mut self, code: &[u8], input: &[u8]) -> AmunResult<Vec<u8>> {
        self.gas_meter.consume(100)?;
        let result = self.interpreter.execute_deterministic(code, input)?;
        self.gas_meter.consume(result.len() as u64 * 10)?;
        Ok(result)
    }

    pub fn gas_used(&self) -> u64 { self.gas_meter.total_consumed() }
    pub fn gas_remaining(&self) -> u64 { self.gas_meter.remaining() }
}
