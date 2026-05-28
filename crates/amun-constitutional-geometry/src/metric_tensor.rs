/// A metric tensor field over constitutional space.
/// Unlike a simple distance vector, the metric tensor defines how
/// distances are computed at EACH point in the space. The metric
/// itself changes depending on where the civilization is located.
#[derive(Debug, Clone)]
pub struct MetricTensorField {
    /// The point in constitutional space where this tensor is evaluated
    pub evaluation_point: [u8; 32],
    /// The 7x7 metric tensor (physics, replay, lineage, temporal,
    /// entropy, treaty, amendment dimensions)
    pub tensor: [[f64; 7]; 7],
    /// Determinant of the metric tensor
    pub determinant: f64,
    /// Whether the metric is degenerate (singular) at this point
    pub is_degenerate: bool,
    /// Whether the metric is Riemannian (positive definite)
    pub is_riemannian: bool,
}

impl MetricTensorField {
    pub fn new(at_point: [u8; 32]) -> Self {
        // Identity metric as default (flat space)
        let mut tensor = [[0.0; 7]; 7];
        for i in 0..7 {
            tensor[i][i] = 1.0;
        }
        Self {
            evaluation_point: at_point,
            tensor,
            determinant: 1.0,
            is_degenerate: false,
            is_riemannian: true,
        }
    }

    /// Compute the squared distance between two nearby points using
    /// the metric tensor at this location.
    pub fn distance_squared(&self, delta: &[f64; 7]) -> f64 {
        let mut sum = 0.0;
        for i in 0..7 {
            for j in 0..7 {
                sum += delta[i] * self.tensor[i][j] * delta[j];
            }
        }
        sum.max(0.0)
    }

    /// Apply a "mass" that warps the metric (constitutional gravity).
    pub fn apply_gravitational_mass(&mut self, invariant_mass: f64, at_dimension: usize) {
        if at_dimension < 7 {
            // Massive invariants warp the metric more strongly
            let warp_factor = 1.0 + invariant_mass * 0.1;
            self.tensor[at_dimension][at_dimension] *= warp_factor;
            self.recompute_determinant();
        }
    }

    fn recompute_determinant(&mut self) {
        // Simplified determinant for diagonal-dominant tensors
        let mut det = 1.0;
        for i in 0..7 {
            det *= self.tensor[i][i];
        }
        self.determinant = det;
        self.is_degenerate = det.abs() < 0.001;
    }
}
