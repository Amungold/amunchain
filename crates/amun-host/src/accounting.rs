pub struct ResourceAccountant {
    pub cpu_used: u64,
    pub memory_used: u64,
    pub max_cpu: u64,
    pub max_memory: u64,
}

impl ResourceAccountant {
    pub fn new() -> Self {
        Self {
            cpu_used: 0,
            memory_used: 0,
            max_cpu: 1_000_000_000,
            max_memory: 512_000_000,
        }
    }

    pub fn charge_cpu(&mut self, units: u64) -> Result<(), &'static str> {
        self.cpu_used = self.cpu_used.checked_add(units).ok_or("cpu overflow")?;
        if self.cpu_used > self.max_cpu {
            return Err("cpu budget exceeded");
        }
        Ok(())
    }

    pub fn charge_memory(&mut self, bytes: u64) -> Result<(), &'static str> {
        self.memory_used = self.memory_used.checked_add(bytes).ok_or("memory overflow")?;
        if self.memory_used > self.max_memory {
            return Err("memory budget exceeded");
        }
        Ok(())
    }
}
