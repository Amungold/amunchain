use amun_canonical_codec::CanonicalHasher;

/// Fork Divergence Algebra: governs constitutional bifurcation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForkDeclaration {
    pub parent_civilization: [u8; 32],
    pub fork_civilization: [u8; 32],
    pub fork_type: ForkType,
    pub fork_epoch: u64,
    pub fork_reason_hash: [u8; 32],
    pub preserves_replay: bool,
    pub preserves_governance: bool,
    pub declaration_hash: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForkType {
    /// Peaceful fork: both civilizations continue in parallel
    Parallel,
    /// Competitive fork: only one will survive
    Competitive,
    /// Experimental fork: child may merge back later
    Experimental,
    /// Hostile fork: parent and child are incompatible
    Hostile,
}

impl ForkDeclaration {
    pub fn new(
        parent: [u8; 32],
        fork: [u8; 32],
        fork_type: ForkType,
        epoch: u64,
        reason_hash: [u8; 32],
        preserves_replay: bool,
        preserves_governance: bool,
    ) -> Self {
        let mut d = Self {
            parent_civilization: parent,
            fork_civilization: fork,
            fork_type,
            fork_epoch: epoch,
            fork_reason_hash: reason_hash,
            preserves_replay,
            preserves_governance,
            declaration_hash: [0u8; 32],
        };
        d.declaration_hash = d.compute_hash();
        d
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(b"AMUN_FORK_V1");
        h.update(&self.parent_civilization);
        h.update(&self.fork_civilization);
        h.update(&[self.fork_type.canonical_tag()]);
        h.update(&self.fork_epoch.to_le_bytes());
        h.update(&self.fork_reason_hash);
        h.update(&[self.preserves_replay as u8]);
        h.update(&[self.preserves_governance as u8]);
        h.finalize()
    }
}

impl ForkType {
    pub fn canonical_tag(&self) -> u8 {
        match self {
            Self::Parallel => 0x01,
            Self::Competitive => 0x02,
            Self::Experimental => 0x03,
            Self::Hostile => 0xFF,
        }
    }
}

/// Merge Compatibility Algebra: governs constitutional reunification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeDeclaration {
    pub source_civilizations: Vec<[u8; 32]>,
    pub target_civilization: [u8; 32],
    pub merge_type: MergeType,
    pub merge_epoch: u64,
    pub replay_unified: bool,
    pub governance_unified: bool,
    pub freeze_maps_compatible: bool,
    pub identities_merged: bool,
    pub declaration_hash: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MergeType {
    /// Full union: both civilizations cease, new one emerges
    Union,
    /// Absorption: one civilization absorbs the other
    Absorption { survivor: [u8; 32] },
    /// Federation: both continue but share governance
    Federation,
}

impl MergeDeclaration {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        sources: Vec<[u8; 32]>,
        target: [u8; 32],
        merge_type: MergeType,
        epoch: u64,
        replay_unified: bool,
        governance_unified: bool,
        freeze_maps_compatible: bool,
        identities_merged: bool,
    ) -> Self {
        let mut d = Self {
            source_civilizations: sources,
            target_civilization: target,
            merge_type,
            merge_epoch: epoch,
            replay_unified,
            governance_unified,
            freeze_maps_compatible,
            identities_merged,
            declaration_hash: [0u8; 32],
        };
        d.declaration_hash = d.compute_hash();
        d
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(b"AMUN_MERGE_V1");
        h.update(&(self.source_civilizations.len() as u64).to_le_bytes());
        for s in &self.source_civilizations {
            h.update(s);
        }
        h.update(&self.target_civilization);
        h.update(&[self.merge_type.canonical_tag()]);
        h.update(&self.merge_epoch.to_le_bytes());
        h.update(&[self.replay_unified as u8]);
        h.update(&[self.governance_unified as u8]);
        h.update(&[self.freeze_maps_compatible as u8]);
        h.update(&[self.identities_merged as u8]);
        h.finalize()
    }
}

impl MergeType {
    pub fn canonical_tag(&self) -> u8 {
        match self {
            Self::Union => 0x01,
            Self::Absorption { .. } => 0x02,
            Self::Federation => 0x03,
        }
    }
}
