use amun_canonical_codec::CanonicalHasher;
use amun_lineage::compatibility::CompatibilityClass;
use std::collections::HashSet;

pub const AMENDMENT_DOMAIN: &[u8] = b"AMUN_AMENDMENT_V1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProposerIdentity {
    pub identity_hash: [u8; 32],
    pub authority: GovernanceAuthority,
    pub civilization_id: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernanceAuthority {
    Validator,
    CouncilMember,
    EmergencyResponder {
        max_amendments_per_epoch: u64,
        /// Which amendment types this responder can create
        allowed_types: HashSet<AmendmentClass>,
        /// Maximum impact level (0 = parameter only, 3 = constitutional)
        max_impact_level: u8,
    },
    ConstitutionalCourt,
    SuperMajority {
        threshold_percent: u8,
    },
}

/// Classification of amendment types for authority scoping.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AmendmentClass {
    Parameter,
    Protocol,
    Snapshot,
    Proof,
    Constitution,
    Emergency,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Amendment {
    pub amendment_id: [u8; 32],
    pub parent_constitution: [u8; 32],
    pub proposed_constitution: [u8; 32],
    pub amendment_type: AmendmentType,
    pub compatibility_class: CompatibilityClass,
    pub description_hash: [u8; 32],
    pub proposed_at_epoch: u64,
    pub proposed_by: ProposerIdentity,
    pub status: AmendmentStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmendmentType {
    ProtocolUpgrade,
    SnapshotEvolution,
    ProofEvolution,
    ParameterChange,
    ConstitutionalText,
    EmergencyFix {
        expires_after_epochs: u64,
        requires_supermajority: bool,
        auto_revoke: bool,
        /// Which constitutional layers this emergency fix affects
        affected_layers: Vec<AmendmentClass>,
    },
}

impl AmendmentType {
    pub fn amendment_class(&self) -> AmendmentClass {
        match self {
            AmendmentType::ProtocolUpgrade => AmendmentClass::Protocol,
            AmendmentType::SnapshotEvolution => AmendmentClass::Snapshot,
            AmendmentType::ProofEvolution => AmendmentClass::Proof,
            AmendmentType::ParameterChange => AmendmentClass::Parameter,
            AmendmentType::ConstitutionalText => AmendmentClass::Constitution,
            AmendmentType::EmergencyFix { .. } => AmendmentClass::Emergency,
        }
    }

    pub fn impact_level(&self) -> u8 {
        match self {
            AmendmentType::ParameterChange => 0,
            AmendmentType::SnapshotEvolution => 1,
            AmendmentType::ProtocolUpgrade => 2,
            AmendmentType::ProofEvolution => 2,
            AmendmentType::ConstitutionalText => 3,
            AmendmentType::EmergencyFix {
                affected_layers, ..
            } => affected_layers
                .iter()
                .map(|c| match c {
                    AmendmentClass::Parameter => 0,
                    AmendmentClass::Snapshot => 1,
                    AmendmentClass::Protocol => 2,
                    AmendmentClass::Proof => 2,
                    AmendmentClass::Constitution => 3,
                    AmendmentClass::Emergency => 2,
                })
                .max()
                .unwrap_or(2),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AmendmentStatus {
    Proposed,
    UnderRatification { started_at: u64, deadline: u64 },
    Ratified { at_epoch: u64, proof: [u8; 32] },
    Rejected { at_epoch: u64, reason: String },
    Activated { at_epoch: u64 },
    Expired { at_epoch: u64 },
    Revoked { at_epoch: u64, reason: String },
}

impl Amendment {
    pub fn new(
        parent_constitution: [u8; 32],
        proposed_constitution: [u8; 32],
        amendment_type: AmendmentType,
        compatibility_class: CompatibilityClass,
        description_hash: [u8; 32],
        epoch: u64,
        proposer: ProposerIdentity,
    ) -> Self {
        let mut amendment = Self {
            amendment_id: [0u8; 32],
            parent_constitution,
            proposed_constitution,
            amendment_type,
            compatibility_class,
            description_hash,
            proposed_at_epoch: epoch,
            proposed_by: proposer,
            status: AmendmentStatus::Proposed,
        };
        amendment.amendment_id = amendment.compute_id();
        amendment
    }

    fn compute_id(&self) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(AMENDMENT_DOMAIN);
        h.update(&self.parent_constitution);
        h.update(&self.proposed_constitution);
        h.update(&[self.amendment_type.amendment_class() as u8]);
        h.update(&[self.amendment_type.impact_level()]);
        h.update(&[self.compatibility_class.rank()]);
        h.update(&self.description_hash);
        h.update(&self.proposed_at_epoch.to_le_bytes());
        h.update(&self.proposed_by.identity_hash);
        h.update(&self.proposed_by.civilization_id);
        h.finalize()
    }
}
