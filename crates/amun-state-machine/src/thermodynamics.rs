/// Constitutional entropy: a measure of disorder in the civilization system.
/// Every fork, amendment, and replay divergence increases entropy.
/// Every merge, freeze, and replay convergence decreases it.
#[derive(Debug, Clone)]
pub struct ConstitutionalEntropy {
    pub current_entropy: f64,
    pub max_entropy: f64,
    pub entropy_rate: f64,
}

impl Default for ConstitutionalEntropy {
    fn default() -> Self {
        Self::new()
    }
}

impl ConstitutionalEntropy {
    pub fn new() -> Self {
        Self {
            current_entropy: 0.0,
            max_entropy: 100.0,
            entropy_rate: 0.0,
        }
    }

    /// Each fork increases entropy.
    pub fn record_fork(&mut self) {
        self.current_entropy = (self.current_entropy + 5.0).min(self.max_entropy);
    }

    /// Each amendment increases entropy.
    pub fn record_amendment(&mut self, impact_level: u8) {
        self.current_entropy =
            (self.current_entropy + impact_level as f64 * 2.0).min(self.max_entropy);
    }

    /// Each replay divergence increases entropy.
    pub fn record_replay_divergence(&mut self, bytes: u64) {
        self.current_entropy =
            (self.current_entropy + (bytes as f64 / 100.0)).min(self.max_entropy);
    }

    /// Mergers decrease entropy.
    pub fn record_merger(&mut self) {
        self.current_entropy = (self.current_entropy - 10.0).max(0.0);
    }

    /// Freeze decreases entropy.
    pub fn record_freeze(&mut self) {
        self.current_entropy = (self.current_entropy - 3.0).max(0.0);
    }

    pub fn is_critical(&self) -> bool {
        self.current_entropy > self.max_entropy * 0.8
    }
}

/// Constitutional stability equations.
#[derive(Debug, Clone)]
pub struct StabilityEquations {
    pub entropy: ConstitutionalEntropy,
}

impl Default for StabilityEquations {
    fn default() -> Self {
        Self::new()
    }
}

impl StabilityEquations {
    pub fn new() -> Self {
        Self {
            entropy: ConstitutionalEntropy::new(),
        }
    }

    /// The civilization is stable if entropy is below threshold.
    pub fn is_stable(&self) -> bool {
        !self.entropy.is_critical()
    }

    /// The civilization is approaching chaos if entropy is high.
    pub fn chaos_warning(&self) -> Option<String> {
        if self.entropy.current_entropy > self.entropy.max_entropy * 0.9 {
            Some("CRITICAL: Constitutional entropy approaching maximum. Civilization at risk of fragmentation.".to_string())
        } else if self.entropy.is_critical() {
            Some(
                "WARNING: High constitutional entropy. Consider stabilization measures."
                    .to_string(),
            )
        } else {
            None
        }
    }
}
