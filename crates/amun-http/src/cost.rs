pub struct RpcCostMeter {
    pub used: u64,
    pub limit: u64,
    pub per_query_cost: u64,
    pub per_byte_cost: u64,
}

impl RpcCostMeter {
    pub fn new(limit: u64) -> Self {
        Self {
            used: 0,
            limit,
            per_query_cost: 10,
            per_byte_cost: 1,
        }
    }

    pub fn charge_query(&mut self) -> Result<(), &'static str> {
        self.used = self
            .used
            .checked_add(self.per_query_cost)
            .ok_or("cost overflow")?;
        if self.used > self.limit {
            return Err("RPC cost budget exceeded");
        }
        Ok(())
    }

    pub fn remaining(&self) -> u64 {
        self.limit.saturating_sub(self.used)
    }
}
