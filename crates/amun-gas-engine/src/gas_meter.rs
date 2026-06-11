use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GasMeter {
    pub gas_limit: u64,
    pub gas_used: u64,
    pub exhausted: bool,
}

impl GasMeter {
    pub fn new(gas_limit: u64) -> Self {
        Self {
            gas_limit,
            gas_used: 0,
            exhausted: false,
        }
    }

    pub fn charge(&mut self, amount: u64) -> Result<u64, String> {
        if self.exhausted {
            return Err(format!("gas already exhausted at {}", self.gas_used));
        }
        let new_total = self.gas_used.saturating_add(amount);
        if new_total > self.gas_limit {
            self.gas_used = self.gas_limit;
            self.exhausted = true;
            return Err(format!(
                "gas exhausted: used={}, limit={}",
                new_total, self.gas_limit
            ));
        }
        self.gas_used = new_total;
        Ok(self.remaining())
    }

    pub fn remaining(&self) -> u64 {
        self.gas_limit.saturating_sub(self.gas_used)
    }

    pub fn is_exhausted(&self) -> bool {
        self.exhausted
    }

    pub fn can_afford(&self, amount: u64) -> bool {
        !self.exhausted && self.gas_used.saturating_add(amount) <= self.gas_limit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn w7_gas_charge_within_limit() {
        let mut meter = GasMeter::new(100);
        assert!(meter.charge(30).is_ok());
        assert!(meter.charge(50).is_ok());
        assert!(!meter.is_exhausted());
    }

    #[test]
    fn w7_gas_exhaustion_detected() {
        let mut meter = GasMeter::new(50);
        meter.charge(30).ok();
        assert!(meter.charge(30).is_err());
        assert!(meter.is_exhausted());
    }

    #[test]
    fn w7_gas_charge_after_exhaustion_fails() {
        let mut meter = GasMeter::new(50);
        meter.charge(60).ok();
        assert!(meter.charge(1).is_err());
    }

    #[test]
    fn w7_gas_can_afford() {
        let meter = GasMeter {
            gas_limit: 100,
            gas_used: 80,
            exhausted: false,
        };
        assert!(meter.can_afford(20));
        assert!(!meter.can_afford(21));
    }

    #[test]
    fn w7_gas_remaining() {
        let mut meter = GasMeter::new(100);
        meter.charge(35).ok();
        assert_eq!(meter.remaining(), 65);
    }
}
