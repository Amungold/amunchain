//! Temporal Federation Drift — sovereignty across time.
//!
//! Time may sequence events, but must never sequence truth.
//! The past does not govern the present. History is auditable,
//! not sovereign.
//!
//! CRITICAL: Temporal precedence ≠ constitutional precedence.
//! An old revision is not a more authoritative one.

use amun_constitutional::kernel_types::ConstitutionalHash;

/// A temporal drift boundary — prevents historical drift
/// from becoming constitutional authority.
///
/// As the system evolves across time, old artifacts, old revisions,
/// and old federations may accumulate implicit authority simply
/// through persistence. This boundary prevents that accumulation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemporalDriftBoundary {
    /// Unique boundary identifier.
    pub boundary_id: u64,
    /// The current active constitutional revision.
    pub active_revision: u32,
    /// The oldest revision still referenced in active derivations.
    pub oldest_active_revision: u32,
    /// Maximum age (in revisions) before historical artifacts
    /// are auditable only — not derivable from.
    pub max_derivable_age: u32,
    /// Whether historical inertia has been detected.
    pub historical_inertia_detected: bool,
}

impl TemporalDriftBoundary {
    pub fn new(boundary_id: u64, active_revision: u32, max_age: u32) -> Self {
        Self { boundary_id, active_revision, oldest_active_revision: active_revision, max_derivable_age: max_age, historical_inertia_detected: false }
    }

    /// Check whether a historical revision has exceeded the derivable age.
    /// Old revisions may be AUDITED, but not used for active derivation.
    pub fn is_derivable(&self, revision: u32) -> bool {
        self.active_revision.saturating_sub(revision) <= self.max_derivable_age
    }

    /// Record that a historical revision is still actively referenced.
    pub fn record_active_revision(&mut self, revision: u32) {
        if revision < self.oldest_active_revision {
            self.oldest_active_revision = revision;
        }
    }

    /// Check if historical inertia is accumulating.
    pub fn check_inertia(&mut self) {
        let age_gap = self.active_revision.saturating_sub(self.oldest_active_revision);
        if age_gap > self.max_derivable_age * 2 {
            self.historical_inertia_detected = true;
        }
    }
}

/// A historical weight neutralizer — prevents precedent gravity.
///
/// Precedent is an operational convenience: "this worked before."
/// It is NOT a constitutional signal: "this is more valid because
/// it has been used longer."
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoricalWeightNeutralizer {
    /// Artifacts whose historical weight is being tracked.
    pub tracked_artifacts: Vec<ConstitutionalHash>,
    /// Maximum historical usage count before neutrality review.
    pub max_historical_usage: u64,
    /// Whether any artifact has exceeded the neutrality threshold.
    pub neutrality_breach_detected: bool,
}

impl HistoricalWeightNeutralizer {
    pub fn new(max_usage: u64) -> Self {
        Self { tracked_artifacts: Vec::new(), max_historical_usage: max_usage, neutrality_breach_detected: false }
    }

    /// Check if an artifact's historical usage exceeds neutrality.
    pub fn check_historical_usage(&mut self, artifact_hash: ConstitutionalHash, usage_count: u64) -> bool {
        if !self.tracked_artifacts.contains(&artifact_hash) {
            self.tracked_artifacts.push(artifact_hash);
        }
        if usage_count > self.max_historical_usage {
            self.neutrality_breach_detected = true;
            false
        } else {
            true
        }
    }
}

/// A compatibility decay surface — compatibility is not eternal obligation.
///
/// Backward compatibility is a pragmatic choice, not a constitutional
/// mandate. Old formats, old bridges, and old protocols may be
/// gracefully retired without implying constitutional violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompatibilityDecaySurface {
    /// The revision whose compatibility is being evaluated.
    pub revision: u32,
    /// Whether this revision requires active compatibility support.
    pub requires_active_support: bool,
    /// How many revisions back this revision is compatible with.
    pub compatibility_depth: u32,
    /// Whether compatibility has decayed (graceful retirement).
    pub compatibility_decayed: bool,
}

impl CompatibilityDecaySurface {
    pub fn new(revision: u32, compatibility_depth: u32) -> Self {
        Self { revision, requires_active_support: true, compatibility_depth, compatibility_decayed: false }
    }

    /// Decay compatibility — graceful retirement, not rejection.
    /// Decayed artifacts remain auditable but are not actively derivable.
    pub fn decay(&mut self) {
        self.compatibility_decayed = true;
        self.requires_active_support = false;
    }

    /// Returns true if this surface is still actively compatible.
    pub fn is_active(&self) -> bool { self.requires_active_support && !self.compatibility_decayed }
}

/// A temporal attribution lineage — history for traceability, not sovereignty.
///
/// Records the temporal sequence of constitutional events for AUDIT.
/// Does NOT confer authority based on age, duration, or precedence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemporalAttributionLineage {
    /// The sequence of revision activations.
    pub revision_sequence: Vec<(u32, ConstitutionalHash)>,
    /// The current active revision.
    pub active_revision: u32,
    /// Whether any revision claims authority due to age (MUST be false).
    pub age_based_authority_claimed: bool,
}

impl TemporalAttributionLineage {
    pub fn new(initial_revision: u32, initial_hash: ConstitutionalHash) -> Self {
        Self { revision_sequence: vec![(initial_revision, initial_hash)], active_revision: initial_revision, age_based_authority_claimed: false }
    }

    pub fn record_activation(&mut self, revision: u32, hash: ConstitutionalHash) {
        self.revision_sequence.push((revision, hash));
        self.active_revision = revision;
    }

    /// CRITICAL: Age never confers authority.
    /// Older revisions are auditable, not privileged.
    pub fn check_age_authority(&self) -> bool { self.age_based_authority_claimed }
}

/// An epoch containment zone — prevents temporal hegemony.
///
/// Long historical epochs may accumulate implicit authority.
/// This zone detects temporal concentration without invalidating
/// the historical record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EpochContainmentZone {
    /// Epochs that have been contained (excessive temporal dominance).
    pub contained_epochs: Vec<u32>,
    /// Maximum fraction of total lineage an epoch may occupy.
    pub max_epoch_dominance_percent: u8,
    /// Whether temporal hegemony is suspected.
    pub hegemony_suspected: bool,
}

impl EpochContainmentZone {
    pub fn new(max_percent: u8) -> Self {
        Self { contained_epochs: Vec::new(), max_epoch_dominance_percent: max_percent, hegemony_suspected: false }
    }

    pub fn check_epoch_dominance(&mut self, epoch_id: u32, lineage_share_percent: u8) {
        if lineage_share_percent > self.max_epoch_dominance_percent {
            if !self.contained_epochs.contains(&epoch_id) { self.contained_epochs.push(epoch_id); }
            self.hegemony_suspected = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derivable_age_boundary() {
        let boundary = TemporalDriftBoundary::new(1, 10, 3);
        assert!(boundary.is_derivable(8));  // age 2 ≤ 3
        assert!(boundary.is_derivable(7));  // age 3 ≤ 3
        assert!(!boundary.is_derivable(6)); // age 4 > 3
    }

    #[test]
    fn test_historical_weight_neutrality() {
        let mut neutralizer = HistoricalWeightNeutralizer::new(100);
        assert!(neutralizer.check_historical_usage([0xAA; 32], 50));  // within
        assert!(!neutralizer.check_historical_usage([0xBB; 32], 150)); // exceeds
        assert!(neutralizer.neutrality_breach_detected);
    }

    #[test]
    fn test_compatibility_decay() {
        let mut surface = CompatibilityDecaySurface::new(5, 3);
        assert!(surface.is_active());
        surface.decay();
        assert!(!surface.is_active());
        assert!(surface.compatibility_decayed);
    }

    #[test]
    fn test_lineage_never_grants_age_authority() {
        let lineage = TemporalAttributionLineage::new(1, [0xAA; 32]);
        assert!(!lineage.check_age_authority());
    }

    #[test]
    fn test_epoch_containment() {
        let mut zone = EpochContainmentZone::new(50);
        zone.check_epoch_dominance(1, 60); // exceeds 50%
        assert!(zone.hegemony_suspected);
        assert!(zone.contained_epochs.contains(&1));
    }

    #[test]
    fn test_temporal_precedence_is_not_constitutional_precedence() {
        // Old revisions are auditable, not privileged.
        let boundary = TemporalDriftBoundary::new(1, 100, 10);
        assert!(!boundary.is_derivable(80)); // too old to derive from
        // But it still exists in history — just not as authority
    }
}
