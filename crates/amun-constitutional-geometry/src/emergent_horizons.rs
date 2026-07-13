use super::metric_tensor::MetricTensorField;

/// An emergent horizon is a boundary in constitutional spacetime
/// that arises naturally from the geometry, not from predefined
/// categories. Like black hole horizons, they emerge when the
/// metric becomes singular or when geodesics terminate.
#[derive(Debug, Clone)]
pub struct EmergentHorizon {
    /// The type of singularity that created this horizon
    pub singularity_type: SingularityType,
    /// Distance from the evaluation point to the horizon
    pub distance_to_horizon: f64,
    /// Whether the horizon is approaching (distance decreasing)
    pub is_approaching: bool,
    /// Whether the horizon has been crossed (distance <= 0)
    pub is_crossed: bool,
    /// The metric at the horizon boundary
    pub boundary_metric: MetricTensorField,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingularityType {
    /// Metric determinant approaches zero (degenerate geometry)
    MetricDegeneracy,
    /// Invariant mass creates an inescapable region
    InvariantSingularity,
    /// Causal paths terminate (no forward evolution possible)
    CausalTermination,
    /// Entropy diverges (infinite disorder)
    EntropyDivergence,
    /// Replay curvature becomes infinite
    ReplaySingularity,
    /// Two constitutional spaces become disconnected
    TopologyChange,
}

impl EmergentHorizon {
    /// Detect if a horizon is emerging from the metric field.
    pub fn detect(metric: &MetricTensorField, previous_metric: &MetricTensorField) -> Option<Self> {
        // Horizon emerges when metric is becoming degenerate
        if metric.is_degenerate && !previous_metric.is_degenerate {
            return Some(Self {
                singularity_type: SingularityType::MetricDegeneracy,
                distance_to_horizon: 0.0,
                is_approaching: true,
                is_crossed: true,
                boundary_metric: metric.clone(),
            });
        }

        // Horizon emerges when determinant is rapidly decreasing
        let det_ratio = if previous_metric.determinant.abs() > 0.001 {
            metric.determinant / previous_metric.determinant
        } else {
            0.0
        };

        if det_ratio < 0.5 && det_ratio > 0.0 {
            return Some(Self {
                singularity_type: SingularityType::MetricDegeneracy,
                distance_to_horizon: (1.0 - det_ratio) * 100.0,
                is_approaching: true,
                is_crossed: false,
                boundary_metric: metric.clone(),
            });
        }

        None
    }

    /// Can legitimate evolution continue across this horizon?
    pub fn can_evolve(&self) -> bool {
        !self.is_crossed || matches!(self.singularity_type, SingularityType::TopologyChange)
    }
}
