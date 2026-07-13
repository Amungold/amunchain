use serde::{Deserialize, Serialize};

/// Types of triggers that can initiate scaling.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScalingTrigger {
    /// Based on transactions per second.
    TpsThreshold {
        above: Option<f64>,
        below: Option<f64>,
    },
    /// Based on connected peer count.
    PeerCount {
        min: Option<usize>,
        max: Option<usize>,
    },
    /// Based on block finality lag.
    FinalityLag { max_blocks: u64 },
    /// Manual trigger from operator.
    Manual { requested_count: usize },
    /// Scheduled scaling (e.g., peak hours).
    Scheduled { cron: String, target_count: usize },
}

impl ScalingTrigger {
    /// Human-readable description of the trigger.
    pub fn description(&self) -> String {
        match self {
            ScalingTrigger::TpsThreshold { above, below } => {
                format!("TPS threshold: above={:?}, below={:?}", above, below)
            }
            ScalingTrigger::PeerCount { min, max } => {
                format!("Peer count: min={:?}, max={:?}", min, max)
            }
            ScalingTrigger::FinalityLag { max_blocks } => {
                format!("Finality lag exceeds {} blocks", max_blocks)
            }
            ScalingTrigger::Manual { requested_count } => {
                format!("Manual scaling to {} validators", requested_count)
            }
            ScalingTrigger::Scheduled { cron, target_count } => {
                format!("Scheduled: {} → {} validators", cron, target_count)
            }
        }
    }
}
