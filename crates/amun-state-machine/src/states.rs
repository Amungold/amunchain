use amun_canonical_codec::CanonicalHasher;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StateTag {
    Genesis,
    Active,
    UnderAmendment,
    Ratifying,
    Activating,
    Frozen,
    Emergency,
    Forked,
    Merging,
    Extinct,
}

impl StateTag {
    pub fn canonical_tag(&self) -> u8 {
        match self {
            StateTag::Genesis => 0x01,
            StateTag::Active => 0x02,
            StateTag::UnderAmendment => 0x03,
            StateTag::Ratifying => 0x04,
            StateTag::Activating => 0x05,
            StateTag::Frozen => 0x06,
            StateTag::Emergency => 0x07,
            StateTag::Forked => 0x08,
            StateTag::Merging => 0x09,
            StateTag::Extinct => 0xFF,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstitutionalState {
    pub state_tag: StateTag,
    pub civilization_id: [u8; 32],
    pub constitution_hash: [u8; 32],
    pub active_since_epoch: u64,
    pub active_since_generation: u64,
    pub previous_state_tag: Option<StateTag>,
    pub transition_id: Option<[u8; 32]>,
    pub state_hash: [u8; 32],
}

impl ConstitutionalState {
    pub fn new(
        state_tag: StateTag,
        civilization_id: [u8; 32],
        constitution_hash: [u8; 32],
        epoch: u64,
        generation: u64,
        previous_state_tag: Option<StateTag>,
        transition_id: Option<[u8; 32]>,
    ) -> Self {
        let mut s = Self {
            state_tag,
            civilization_id,
            constitution_hash,
            active_since_epoch: epoch,
            active_since_generation: generation,
            previous_state_tag,
            transition_id,
            state_hash: [0u8; 32],
        };
        s.state_hash = s.compute_hash();
        s
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(b"AMUN_STATE_V1");
        h.update(&[self.state_tag.canonical_tag()]);
        h.update(&self.civilization_id);
        h.update(&self.constitution_hash);
        h.update(&self.active_since_epoch.to_le_bytes());
        h.update(&self.active_since_generation.to_le_bytes());
        if let Some(prev) = &self.previous_state_tag {
            h.update(&[prev.canonical_tag()]);
        }
        if let Some(tid) = &self.transition_id {
            h.update(tid);
        }
        h.finalize()
    }

    pub fn verify(&self) -> bool {
        self.compute_hash() == self.state_hash
    }
}
