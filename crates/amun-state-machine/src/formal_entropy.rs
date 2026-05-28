/// Conserved formal entropy for constitutional thermodynamics.
/// Entropy is not just a counter - it follows conservation laws.
#[derive(Debug, Clone)]
pub struct FormalEntropy {
    /// Total entropy in the system
    pub total_entropy: f64,
    /// Entropy that has been transferred (not created)
    pub transferred_entropy: f64,
    /// Entropy that has been irreversibly created
    pub created_entropy: f64,
    /// Entropy sinks that have absorbed entropy
    pub entropy_sinks: Vec<EntropySink>,
}

#[derive(Debug, Clone)]
pub struct EntropySink {
    pub sink_type: EntropySinkType,
    pub absorbed_entropy: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntropySinkType {
    /// A successful merger absorbs entropy
    Merger,
    /// A freeze absorbs entropy
    Freeze,
    /// Replay convergence absorbs entropy
    ReplayConvergence,
    /// Constitutional court ruling absorbs entropy
    CourtRuling,
}

/// Entropy conservation laws.
#[derive(Debug, Clone)]
pub struct EntropyConservationLaws;

impl EntropyConservationLaws {
    /// The First Law: Entropy cannot be destroyed, only transferred or created.
    pub fn first_law(entropy: &FormalEntropy) -> bool {
        let total_accounted = entropy.transferred_entropy + entropy.created_entropy;
        (total_accounted - entropy.total_entropy).abs() < 0.001
    }

    /// The Second Law: Entropy creation is always non-negative.
    pub fn second_law(entropy: &FormalEntropy) -> bool {
        entropy.created_entropy >= 0.0
    }

    /// The Third Law: Absolute zero entropy is unreachable (there is always some constitutional order).
    pub fn third_law(entropy: &FormalEntropy) -> bool {
        entropy.total_entropy > 0.0
    }
}

/// Entropy collapse threshold: when entropy exceeds this, the civilization
/// enters a critical state requiring stabilization.
#[derive(Debug, Clone)]
pub struct EntropyCollapseThreshold {
    pub warning_threshold: f64,
    pub critical_threshold: f64,
    pub collapse_threshold: f64,
}

impl Default for EntropyCollapseThreshold {
    fn default() -> Self {
        Self {
            warning_threshold: 60.0,
            critical_threshold: 80.0,
            collapse_threshold: 95.0,
        }
    }
}
