use amun_constitutional_geometry::emergent_horizons::SingularityType;

/// A constitutional phase transition is a discontinuous change
/// in the topology of constitutional space. Unlike smooth evolution,
/// phase transitions alter the structure of legitimacy itself.
#[derive(Debug, Clone)]
pub struct PhaseTransition {
    /// The type of transition
    pub transition_type: TransitionType,
    /// The constitutional state before the transition
    pub before_state: [u8; 32],
    /// The constitutional state after the transition
    pub after_state: [u8; 32],
    /// Whether the transition is reversible
    pub is_reversible: bool,
    /// Whether the transition preserves constitutional identity
    pub preserves_identity: bool,
    /// The singularity that triggered this transition
    pub triggering_singularity: Option<SingularityType>,
    /// Energy barrier that must be overcome (constitutional action)
    pub activation_energy: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransitionType {
    /// First-order: discontinuous jump in constitutional state
    FirstOrder { latent_heat: f64 },
    /// Second-order: continuous but topology-changing
    SecondOrder { order_parameter: f64 },
    /// Critical: at the boundary of constitutional collapse
    Critical { critical_exponent: f64 },
    /// Irreversible: cannot return to previous state
    Irreversible { entropy_increase: f64 },
}

impl PhaseTransition {
    pub fn new(
        transition_type: TransitionType,
        before: [u8; 32],
        after: [u8; 32],
        singularity: Option<SingularityType>,
    ) -> Self {
        let is_reversible = !matches!(transition_type, TransitionType::Irreversible { .. });
        let preserves_identity = matches!(transition_type, TransitionType::SecondOrder { .. });

        Self {
            transition_type,
            before_state: before,
            after_state: after,
            is_reversible,
            preserves_identity,
            triggering_singularity: singularity,
            activation_energy: 0.0,
        }
    }

    /// Detect if a phase transition is imminent based on metric analysis.
    pub fn detect_imminent(
        current_metric_determinant: f64,
        previous_metric_determinant: f64,
        entropy: f64,
    ) -> Option<TransitionType> {
        let det_ratio = if previous_metric_determinant.abs() > 0.001 {
            current_metric_determinant / previous_metric_determinant
        } else {
            0.0
        };

        if det_ratio < 0.1 {
            Some(TransitionType::Critical {
                critical_exponent: 1.5,
            })
        } else if det_ratio < 0.3 && entropy > 70.0 {
            Some(TransitionType::FirstOrder {
                latent_heat: entropy - 50.0,
            })
        } else if entropy > 90.0 {
            Some(TransitionType::Irreversible {
                entropy_increase: entropy - 80.0,
            })
        } else {
            None
        }
    }

    /// A phase transition is catastrophic if it destroys constitutional identity.
    pub fn is_catastrophic(&self) -> bool {
        !self.preserves_identity || !self.is_reversible
    }
}
