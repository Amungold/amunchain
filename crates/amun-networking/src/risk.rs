/// Constitutional risk classification for peers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstitutionalRisk {
    /// Peer belongs to a different constitutional civilization
    ForeignCivilization { remote_identity_hash: [u8; 32] },
    /// Peer has compatible physics but different laws
    IncompatiblePhysics { reason: String },
    /// Peer's identity appears corrupted or tampered
    IdentityCorruption,
    /// Peer's replay history is unstable
    ReplayInstability { divergence_count: u64 },
    /// Peer is on a hostile fork
    HostileFork { fork_evidence_hash: [u8; 32] },
    /// Peer's temporal state has diverged beyond reconciliation
    TemporalDivergence { epoch_difference: i64 },
    /// Peer exhibits Byzantine behavior
    ByzantineBehavior { evidence_count: u64 },
}

/// Multidimensional constitutional risk profile.
#[derive(Debug, Clone)]
pub struct ConstitutionalRiskProfile {
    pub risks: Vec<ConstitutionalRisk>,
    pub overall_trust_score: f64,
    pub can_sync: bool,
    pub can_exchange_manifests: bool,
    pub requires_quarantine: bool,
}

impl ConstitutionalRiskProfile {
    pub fn new() -> Self {
        Self {
            risks: Vec::new(),
            overall_trust_score: 1.0,
            can_sync: true,
            can_exchange_manifests: true,
            requires_quarantine: false,
        }
    }

    pub fn add_risk(&mut self, risk: ConstitutionalRisk) {
        match &risk {
            ConstitutionalRisk::ForeignCivilization { .. } => {
                self.overall_trust_score *= 0.1;
                self.can_sync = false;
            }
            ConstitutionalRisk::IncompatiblePhysics { .. } => {
                self.overall_trust_score *= 0.2;
                self.can_sync = false;
            }
            ConstitutionalRisk::IdentityCorruption => {
                self.overall_trust_score *= 0.0;
                self.can_sync = false;
                self.can_exchange_manifests = false;
            }
            ConstitutionalRisk::HostileFork { .. } => {
                self.overall_trust_score *= 0.3;
                self.can_sync = false;
                self.requires_quarantine = true;
            }
            ConstitutionalRisk::ReplayInstability { .. } => {
                self.overall_trust_score *= 0.5;
            }
            ConstitutionalRisk::TemporalDivergence { .. } => {
                self.overall_trust_score *= 0.6;
            }
            ConstitutionalRisk::ByzantineBehavior { .. } => {
                self.overall_trust_score *= 0.1;
                self.can_sync = false;
                self.can_exchange_manifests = false;
                self.requires_quarantine = true;
            }
        }
        self.risks.push(risk);
    }
}
