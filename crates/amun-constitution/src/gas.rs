pub mod gas_costs {
    pub const TX_BASE: u64 = 21_000;
    pub const MAX_GAS_PER_BLOCK: u64 = 10_000_000;
    pub const MAX_GAS_PER_TX: u64 = 1_000_000;
}

#[derive(Clone, Debug)]
pub struct GasMeter {
    pub gas_used: u64,
    pub gas_limit: u64,
}

impl GasMeter {
    pub fn new(gas_limit: u64) -> Self { Self { gas_used: 0, gas_limit } }

    pub fn consume(&mut self, amount: u64) -> Result<(), &'static str> {
        self.gas_used = self.gas_used.checked_add(amount).ok_or("Gas overflow")?;
        if self.gas_used > self.gas_limit { return Err("Out of gas"); }
        Ok(())
    }

    pub fn remaining(&self) -> u64 { self.gas_limit.saturating_sub(self.gas_used) }
}
