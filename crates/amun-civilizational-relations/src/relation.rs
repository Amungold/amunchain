use amun_snapshot_engine::ConstitutionalIdentity;

/// The constitutional relationship between two civilizations.
/// This is richer than binary compatible/incompatible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CivilizationalRelation {
    /// Identical constitutional identity
    Identical,
    /// Same lineage, divergent constitutional branches
    DivergentBranch {
        common_ancestor_hash: [u8; 32],
        divergence_epoch: u64,
    },
    /// Branches that can be reconciled
    ReconciliableFork {
        common_ancestor_hash: [u8; 32],
        reconciliation_possible: bool,
    },
    /// Hostile fork - incompatible physics
    HostileFork { fork_evidence_hash: [u8; 32] },
    /// Different constitutional lineage entirely
    ForeignCivilization,
    /// Unknown origin - no lineage data available
    UnknownOrigin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationType {
    Identical = 5,
    DivergentBranch = 4,
    ReconciliableFork = 3,
    HostileFork = 1,
    ForeignCivilization = 0,
    UnknownOrigin = 255,
}

impl CivilizationalRelation {
    pub fn classify(local: &ConstitutionalIdentity, remote: &ConstitutionalIdentity) -> Self {
        if local.matches(remote) {
            return CivilizationalRelation::Identical;
        }

        let same_physics = local.canonical_empty_root == remote.canonical_empty_root
            && local.max_depth == remote.max_depth;

        let same_constitution = local.constitutional_hash == remote.constitutional_hash;

        if same_constitution && same_physics {
            return CivilizationalRelation::DivergentBranch {
                common_ancestor_hash: local.constitutional_hash,
                divergence_epoch: 0,
            };
        }

        if same_physics {
            return CivilizationalRelation::ReconciliableFork {
                common_ancestor_hash: local.canonical_empty_root,
                reconciliation_possible: true,
            };
        }

        CivilizationalRelation::ForeignCivilization
    }

    pub fn relation_type(&self) -> RelationType {
        match self {
            CivilizationalRelation::Identical => RelationType::Identical,
            CivilizationalRelation::DivergentBranch { .. } => RelationType::DivergentBranch,
            CivilizationalRelation::ReconciliableFork { .. } => RelationType::ReconciliableFork,
            CivilizationalRelation::HostileFork { .. } => RelationType::HostileFork,
            CivilizationalRelation::ForeignCivilization => RelationType::ForeignCivilization,
            CivilizationalRelation::UnknownOrigin => RelationType::UnknownOrigin,
        }
    }

    pub fn can_interact(&self) -> bool {
        matches!(
            self,
            CivilizationalRelation::Identical | CivilizationalRelation::DivergentBranch { .. }
        )
    }

    pub fn requires_quarantine(&self) -> bool {
        matches!(
            self,
            CivilizationalRelation::ReconciliableFork { .. }
                | CivilizationalRelation::HostileFork { .. }
        )
    }
}
