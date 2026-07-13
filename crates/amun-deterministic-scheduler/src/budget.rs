/// Resource budget for deterministic scheduling.
/// Ensures bounded execution per round.
#[derive(Debug, Clone)]
pub struct ResourceBudget {
    pub max_cost_per_round: u64,
    pub consumed: u64,
}

impl ResourceBudget {
    pub fn new(max_cost_per_round: u64) -> Self {
        Self {
            max_cost_per_round,
            consumed: 0,
        }
    }

    pub fn consume(&mut self, cost: u64) -> Result<(), &'static str> {
        let new_total = self.consumed.checked_add(cost).ok_or("budget overflow")?;
        if new_total > self.max_cost_per_round {
            return Err("budget exhausted");
        }
        self.consumed = new_total;
        Ok(())
    }

    pub fn remaining(&self) -> u64 {
        self.max_cost_per_round.saturating_sub(self.consumed)
    }

    pub fn reset(&mut self) {
        self.consumed = 0;
    }
}
