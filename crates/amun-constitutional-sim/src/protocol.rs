use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentalProtocol {
    pub num_sovereigns: usize,
    pub num_steps: usize,
    pub initial_recognition_density: f64,
    pub initial_treaty_density: f64,
    pub erosion_rate: f64,
    pub formation_base_rate: f64,
    pub formation_legitimacy_factor: f64,
    pub formation_reciprocity_bias: f64,
    pub treaty_failure_rate: f64,
    pub horizon: usize,
}
