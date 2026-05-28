use super::horizons::CausalHorizon;
use super::metrics::DistanceTensor;

/// An evolutionary trajectory is a path through constitutional phase space.
/// It represents the legitimate evolution of a civilization over time.
#[derive(Debug, Clone)]
pub struct EvolutionaryTrajectory {
    pub trajectory_id: [u8; 32],
    pub starting_point: [u8; 32],
    pub current_point: [u8; 32],
    pub path_length: f64,
    pub crossed_horizons: Vec<CausalHorizon>,
    pub is_legitimate: bool,
    pub has_collapsed: bool,
}

impl EvolutionaryTrajectory {
    pub fn new(start: [u8; 32]) -> Self {
        Self {
            trajectory_id: [0u8; 32],
            starting_point: start,
            current_point: start,
            path_length: 0.0,
            crossed_horizons: Vec::new(),
            is_legitimate: true,
            has_collapsed: false,
        }
    }

    /// Move along the trajectory by applying a constitutional delta.
    pub fn apply_delta(&mut self, delta_tensor: &DistanceTensor, new_point: [u8; 32]) {
        self.path_length += delta_tensor.total_distance();
        self.current_point = new_point;
    }

    /// Check if the trajectory has crossed any fatal horizons.
    pub fn check_horizons(&mut self, horizons: &[CausalHorizon]) {
        for horizon in horizons {
            if horizon.is_crossed {
                self.crossed_horizons.push(horizon.clone());
                if matches!(
                    horizon.horizon_type,
                    super::horizons::HorizonType::PhysicsHorizon
                        | super::horizons::HorizonType::ReplayHorizon
                ) {
                    self.is_legitimate = false;
                }
            }
        }
    }
}
