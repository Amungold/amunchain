pub struct GasMeter {
    pub used: u64,
    pub limit: u64,
}

impl GasMeter {
    pub fn new(limit: u64) -> Self {
        Self { used: 0, limit }
    }

    pub fn consume(&mut self, amount: u64) -> Result<(), &'static str> {
        self.used = self.used.checked_add(amount).ok_or("gas overflow")?;
        if self.used > self.limit {
            return Err("out of gas");
        }
        Ok(())
    }

    pub fn remaining(&self) -> u64 {
        self.limit.saturating_sub(self.used)
    }
}
