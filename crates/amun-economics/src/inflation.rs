pub struct InflationCurve {
    pub rate: u16,
}
impl InflationCurve {
    pub fn new() -> Self {
        Self { rate: 800 }
    }
}

impl Default for InflationCurve {
    fn default() -> Self {
        Self::new()
    }
}
