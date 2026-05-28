use super::emergent_horizons::EmergentHorizon;

/// A constitutional geodesic is the shortest legitimate path between
/// two points in constitutional space. Unlike Euclidean straight lines,
/// geodesics curve around invariant singularities and avoid horizons.
#[derive(Debug, Clone)]
pub struct ConstitutionalGeodesic {
    pub start_point: [u8; 32],
    pub end_point: [u8; 32],
    pub path_length: f64,
    pub is_legitimate: bool,
    pub crossed_horizons: Vec<EmergentHorizon>,
    pub waypoints: Vec<[f64; 7]>,
}

impl ConstitutionalGeodesic {
    pub fn new(start: [u8; 32], end: [u8; 32]) -> Self {
        Self {
            start_point: start,
            end_point: end,
            path_length: f64::INFINITY,
            is_legitimate: true,
            crossed_horizons: Vec::new(),
            waypoints: Vec::new(),
        }
    }

    /// Check if a direct geodesic exists (no horizons crossed).
    pub fn is_direct(&self) -> bool {
        self.crossed_horizons.is_empty()
    }

    /// The geodesic is legitimate if it doesn't cross fatal horizons.
    pub fn check_legitimacy(&mut self) {
        for horizon in &self.crossed_horizons {
            if !horizon.can_evolve() {
                self.is_legitimate = false;
                return;
            }
        }
        self.is_legitimate = true;
    }
}
