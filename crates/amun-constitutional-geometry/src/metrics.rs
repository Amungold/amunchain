use amun_civilizational_relations::relation::CivilizationalRelation;
use amun_snapshot_engine::ConstitutionalIdentity;

/// Constitutional distance tensor.
/// Distance is not a scalar - it's multi-dimensional.
/// Each dimension measures a different aspect of constitutional separation.
#[derive(Debug, Clone)]
pub struct DistanceTensor {
    /// Distance in physics space (empty root, max depth, proof version)
    pub physics_distance: f64,
    /// Distance in replay space (replay guarantee compatibility)
    pub replay_distance: f64,
    /// Distance in lineage space (ancestry divergence)
    pub lineage_distance: f64,
    /// Distance in temporal space (epoch/generation separation)
    pub temporal_distance: f64,
    /// Entropy gradient between the two civilizations
    pub entropy_gradient: f64,
    /// Treaty separation (how many treaties apart)
    pub treaty_separation: f64,
    /// Amendment curvature (how many amendments diverge)
    pub amendment_curvature: f64,
}

impl Default for DistanceTensor {
    fn default() -> Self {
        Self::new()
    }
}

impl DistanceTensor {
    pub fn new() -> Self {
        Self {
            physics_distance: 0.0,
            replay_distance: 0.0,
            lineage_distance: 0.0,
            temporal_distance: 0.0,
            entropy_gradient: 0.0,
            treaty_separation: 0.0,
            amendment_curvature: 0.0,
        }
    }

    /// Compute the total constitutional distance (Frobenius-like norm).
    pub fn total_distance(&self) -> f64 {
        (self.physics_distance.powi(2)
            + self.replay_distance.powi(2)
            + self.lineage_distance.powi(2)
            + self.temporal_distance.powi(2)
            + self.entropy_gradient.powi(2)
            + self.treaty_separation.powi(2)
            + self.amendment_curvature.powi(2))
        .sqrt()
    }

    /// Check if the distance is within a legitimate transformation bound.
    pub fn is_within_legitimate_bounds(&self, max_distance: f64) -> bool {
        self.total_distance() <= max_distance
    }
}

/// Constitutional distance between two civilizations.
/// Distance = cost of legitimate transformation from A to B.
#[derive(Debug, Clone)]
pub struct ConstitutionalDistance {
    pub source_identity_hash: [u8; 32],
    pub target_identity_hash: [u8; 32],
    pub relation: CivilizationalRelation,
    pub tensor: DistanceTensor,
    pub requires_invariant_break: bool,
    pub requires_causal_jump: bool,
}

impl ConstitutionalDistance {
    pub fn compute(local: &ConstitutionalIdentity, remote: &ConstitutionalIdentity) -> Self {
        let relation = CivilizationalRelation::classify(local, remote);
        let mut tensor = DistanceTensor::new();

        // Physics distance: how different are the constitutional physics?
        if local.canonical_empty_root != remote.canonical_empty_root {
            tensor.physics_distance = 100.0; // Different physical universe
        }
        if local.max_depth != remote.max_depth {
            tensor.physics_distance += 50.0;
        }
        if local.proof_version != remote.proof_version {
            tensor.physics_distance += 30.0;
        }

        // Replay distance: can replay be preserved?
        if local.constitutional_hash != remote.constitutional_hash {
            tensor.replay_distance = 20.0;
        }

        // Temporal distance: how far apart in constitutional time?
        tensor.temporal_distance = 5.0; // Placeholder - filled by temporal alignment

        let requires_invariant_break = tensor.physics_distance > 50.0;
        let requires_causal_jump = tensor.replay_distance > 30.0;

        Self {
            source_identity_hash: local.identity_hash,
            target_identity_hash: remote.identity_hash,
            relation,
            tensor,
            requires_invariant_break,
            requires_causal_jump,
        }
    }
}
