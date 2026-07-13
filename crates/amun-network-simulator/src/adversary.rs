/// Adversarial network configuration.
#[derive(Debug, Clone)]
pub struct AdversaryConfig {
    pub drop_probability: f64,
    pub max_additional_delay: u64,
    pub byzantine_nodes: Vec<u64>,
    pub partition_enabled: bool,
    pub partition_boundary: usize,
}

impl AdversaryConfig {
    pub fn honest() -> Self {
        Self {
            drop_probability: 0.0,
            max_additional_delay: 0,
            byzantine_nodes: Vec::new(),
            partition_enabled: false,
            partition_boundary: 0,
        }
    }

    pub fn with_partition(boundary: usize) -> Self {
        Self {
            drop_probability: 0.1,
            max_additional_delay: 5,
            byzantine_nodes: Vec::new(),
            partition_enabled: true,
            partition_boundary: boundary,
        }
    }

    pub fn additional_delay(&self, sender: u64, _receiver: u64, _round: u64) -> u64 {
        if self.byzantine_nodes.contains(&sender) {
            return self.max_additional_delay;
        }
        0
    }

    pub fn should_drop(&self, message_count: u64) -> bool {
        if self.drop_probability <= 0.0 {
            return false;
        }
        // Deterministic drop decision based on message count
        (message_count.wrapping_mul(1103515245).wrapping_add(12345) % 100)
            < (self.drop_probability * 100.0) as u64
    }
}
