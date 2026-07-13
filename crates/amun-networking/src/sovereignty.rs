use amun_snapshot_engine::ConstitutionalIdentity;
use amun_state_machine::states::StateTag;

/// Sovereignty boundary: defines the limits of acceptable interaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SovereigntyBoundary {
    pub local_identity: ConstitutionalIdentity,
    pub minimum_compatibility_rank: u8,
    pub allow_foreign_snapshots: bool,
    pub foreign_snapshot_quarantine_required: bool,
    pub allow_readonly_peers: bool,
    pub require_lineage_proof: bool,
}

/// Constitutional handshake between two civilizations.
/// Multi-phase: Identity -> Physics -> Replay -> Lineage -> Temporal -> Sync
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalHandshake {
    pub local_identity: ConstitutionalIdentity,
    pub remote_identity: Option<ConstitutionalIdentity>,
    pub phase: HandshakePhase,
    pub identity_compatible: bool,
    pub physics_compatible: bool,
    pub replay_compatible: bool,
    pub lineage_compatible: bool,
    pub temporal_compatible: bool,
    pub can_sync: bool,
    pub can_exchange_manifests: bool,
    pub sovereignty_respected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandshakePhase {
    IdentityVerification,
    PhysicsComparison,
    ReplayCompatibilityCheck,
    LineageValidation,
    TemporalAlignment,
    SyncAuthorized,
    Rejected { reason: String },
}

impl ConstitutionalHandshake {
    pub fn new(local: ConstitutionalIdentity) -> Self {
        Self {
            local_identity: local,
            remote_identity: None,
            phase: HandshakePhase::IdentityVerification,
            identity_compatible: false,
            physics_compatible: false,
            replay_compatible: false,
            lineage_compatible: false,
            temporal_compatible: false,
            can_sync: false,
            can_exchange_manifests: false,
            sovereignty_respected: true,
        }
    }

    pub fn accept_remote(&mut self, remote: ConstitutionalIdentity) {
        self.remote_identity = Some(remote.clone());
        self.identity_compatible = self.local_identity.matches(&remote);
        self.phase = if self.identity_compatible {
            HandshakePhase::PhysicsComparison
        } else {
            HandshakePhase::Rejected { reason: "Identity mismatch".to_string() }
        };
    }

    pub fn verify_physics(&mut self) {
        if let Some(ref remote) = self.remote_identity {
            let local = &self.local_identity;
            self.physics_compatible = local.canonical_empty_root == remote.canonical_empty_root
                && local.max_depth == remote.max_depth
                && local.proof_version == remote.proof_version;
            self.phase = if self.physics_compatible {
                HandshakePhase::ReplayCompatibilityCheck
            } else {
                HandshakePhase::Rejected { reason: "Physics incompatibility".to_string() }
            };
        }
    }

    pub fn authorize_sync(&mut self) {
        self.can_sync = self.identity_compatible
            && self.physics_compatible
            && self.replay_compatible
            && self.lineage_compatible
            && self.temporal_compatible;
        self.phase = if self.can_sync {
            HandshakePhase::SyncAuthorized
        } else {
            HandshakePhase::Rejected { reason: "Multi-phase sync authorization failed".to_string() }
        };
    }
}
