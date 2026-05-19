#[derive(Debug, Clone)]
pub struct NetworkConditions {
    pub mean_latency_ms: u64,
    pub packet_loss_rate: f64,
    pub partitioned: bool,
}

impl NetworkConditions {
    pub fn normal() -> Self {
        Self {
            mean_latency_ms: 100,
            packet_loss_rate: 0.001,
            partitioned: false,
        }
    }

    pub fn partitioned() -> Self {
        Self {
            mean_latency_ms: 500,
            packet_loss_rate: 0.5,
            partitioned: true,
        }
    }
}
