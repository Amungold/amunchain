pub struct InflationLaw {
    pub initial_rate_bps: u16,
    pub min_rate_bps: u16,
    pub year: u32,
}

impl InflationLaw {
    pub fn new() -> Self {
        Self {
            initial_rate_bps: 800,
            min_rate_bps: 200,
            year: 0,
        }
    }

    pub fn current_rate_bps(&self) -> u16 {
        let decay = self.year.saturating_mul(100) as u16;
        self.initial_rate_bps
            .saturating_sub(decay)
            .max(self.min_rate_bps)
    }

    pub fn advance_year(&mut self) {
        self.year = self.year.saturating_add(1);
    }
}
