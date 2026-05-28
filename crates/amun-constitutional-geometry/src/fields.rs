use super::metrics::DistanceTensor;

/// A constitutional field assigns a value to every point in constitutional space.
#[derive(Debug, Clone)]
pub struct ConstitutionalField {
    pub field_type: FieldType,
    pub intensity: f64,
    pub gradient: LegitimacyGradient,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldType {
    LegitimacyPotential,
    EntropyField,
    StabilityField,
    CausalTensionField,
}

/// The legitimacy gradient indicates the direction and magnitude
/// of legitimacy change across constitutional space.
#[derive(Debug, Clone)]
pub struct LegitimacyGradient {
    pub direction: [f64; 7], // 7-dimensional direction vector
    pub magnitude: f64,
    pub is_increasing: bool,
}

impl LegitimacyGradient {
    pub fn new(direction: [f64; 7], magnitude: f64) -> Self {
        Self {
            direction,
            magnitude,
            is_increasing: magnitude > 0.0,
        }
    }

    /// Compute gradient from a distance tensor.
    pub fn from_tensor(tensor: &DistanceTensor) -> Self {
        let direction = [
            tensor.physics_distance,
            tensor.replay_distance,
            tensor.lineage_distance,
            tensor.temporal_distance,
            tensor.entropy_gradient,
            tensor.treaty_separation,
            tensor.amendment_curvature,
        ];
        let magnitude = tensor.total_distance();
        Self::new(direction, magnitude)
    }
}

/// The entropy field measures constitutional disorder across space.
#[derive(Debug, Clone)]
pub struct EntropyField {
    pub base_entropy: f64,
    pub local_gradient: f64,
    pub is_increasing: bool,
    pub collapse_imminent: bool,
}

impl EntropyField {
    pub fn new(base: f64, gradient: f64) -> Self {
        Self {
            base_entropy: base,
            local_gradient: gradient,
            is_increasing: gradient > 0.0,
            collapse_imminent: base > 80.0 && gradient > 0.0,
        }
    }
}
