use super::states::{ConstitutionalState, StateTag};
use amun_canonical_codec::CanonicalHasher;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionType {
    Genesis,
    ProposeAmendment,
    Ratify,
    Activate,
    Reject,
    Freeze,
    Unfreeze,
    Emergency,
    ResolveEmergency,
    Fork,
    Merge,
    Extinct,
}

impl TransitionType {
    pub fn canonical_tag(&self) -> u8 {
        match self {
            Self::Genesis => 0x01,
            Self::ProposeAmendment => 0x02,
            Self::Ratify => 0x03,
            Self::Activate => 0x04,
            Self::Reject => 0x05,
            Self::Freeze => 0x06,
            Self::Unfreeze => 0x07,
            Self::Emergency => 0x08,
            Self::ResolveEmergency => 0x09,
            Self::Fork => 0x0A,
            Self::Merge => 0x0B,
            Self::Extinct => 0xFF,
        }
    }
}

pub struct TransitionAlgebra;
impl TransitionAlgebra {
    pub fn resolve(from: StateTag, transition: TransitionType) -> Option<StateTag> {
        match (from, transition) {
            (StateTag::Genesis, TransitionType::Genesis) => Some(StateTag::Active),
            (StateTag::Active, TransitionType::ProposeAmendment) => Some(StateTag::UnderAmendment),
            (StateTag::Active, TransitionType::Freeze) => Some(StateTag::Frozen),
            (StateTag::Active, TransitionType::Emergency) => Some(StateTag::Emergency),
            (StateTag::Active, TransitionType::Fork) => Some(StateTag::Forked),
            (StateTag::Active, TransitionType::Merge) => Some(StateTag::Merging),
            (StateTag::Active, TransitionType::Extinct) => Some(StateTag::Extinct),
            (StateTag::UnderAmendment, TransitionType::Ratify) => Some(StateTag::Ratifying),
            (StateTag::UnderAmendment, TransitionType::Reject) => Some(StateTag::Active),
            (StateTag::UnderAmendment, TransitionType::Extinct) => Some(StateTag::Extinct),
            (StateTag::Ratifying, TransitionType::Activate) => Some(StateTag::Activating),
            (StateTag::Ratifying, TransitionType::Reject) => Some(StateTag::Active),
            (StateTag::Ratifying, TransitionType::Extinct) => Some(StateTag::Extinct),
            (StateTag::Activating, TransitionType::Activate) => Some(StateTag::Active),
            (StateTag::Activating, TransitionType::Extinct) => Some(StateTag::Extinct),
            (StateTag::Frozen, TransitionType::Unfreeze) => Some(StateTag::Active),
            (StateTag::Frozen, TransitionType::Extinct) => Some(StateTag::Extinct),
            (StateTag::Emergency, TransitionType::ResolveEmergency) => Some(StateTag::Active),
            (StateTag::Emergency, TransitionType::Extinct) => Some(StateTag::Extinct),
            (StateTag::Forked, TransitionType::Fork) => Some(StateTag::Forked),
            (StateTag::Forked, TransitionType::Extinct) => Some(StateTag::Extinct),
            (StateTag::Merging, TransitionType::Merge) => Some(StateTag::Active),
            (StateTag::Merging, TransitionType::Extinct) => Some(StateTag::Extinct),
            _ => None,
        }
    }

    pub fn legal_transitions(from: StateTag) -> Vec<TransitionType> {
        [
            TransitionType::Genesis,
            TransitionType::ProposeAmendment,
            TransitionType::Ratify,
            TransitionType::Activate,
            TransitionType::Reject,
            TransitionType::Freeze,
            TransitionType::Unfreeze,
            TransitionType::Emergency,
            TransitionType::ResolveEmergency,
            TransitionType::Fork,
            TransitionType::Merge,
            TransitionType::Extinct,
        ]
        .iter()
        .filter(|t| Self::resolve(from, **t).is_some())
        .copied()
        .collect()
    }

    pub fn forbidden_transitions(from: StateTag) -> Vec<TransitionType> {
        [
            TransitionType::Genesis,
            TransitionType::ProposeAmendment,
            TransitionType::Ratify,
            TransitionType::Activate,
            TransitionType::Reject,
            TransitionType::Freeze,
            TransitionType::Unfreeze,
            TransitionType::Emergency,
            TransitionType::ResolveEmergency,
            TransitionType::Fork,
            TransitionType::Merge,
            TransitionType::Extinct,
        ]
        .iter()
        .filter(|t| Self::resolve(from, **t).is_none())
        .copied()
        .collect()
    }
}

/// A transition with full constitutional causality binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition {
    pub transition_id: [u8; 32],
    pub transition_type: TransitionType,
    pub from_state: ConstitutionalState,
    pub to_state_tag: StateTag,
    pub epoch: u64,
    pub generation: u64,
    pub previous_transition_hash: Option<[u8; 32]>,
    pub checkpoint_root: [u8; 32],
    pub lineage_head_hash: [u8; 32],
    /// Hash representing the constitutional delta (what changed semantically)
    pub constitutional_delta_hash: [u8; 32],
    /// Causal reference: what transition(s) this one depends on
    pub causal_transition_hash: Option<[u8; 32]>,
    pub proof_hash: [u8; 32],
}

impl Transition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        transition_type: TransitionType,
        from_state: ConstitutionalState,
        to_state_tag: StateTag,
        epoch: u64,
        generation: u64,
        previous_transition_hash: Option<[u8; 32]>,
        checkpoint_root: [u8; 32],
        lineage_head_hash: [u8; 32],
        constitutional_delta_hash: [u8; 32],
        causal_transition_hash: Option<[u8; 32]>,
    ) -> Self {
        let mut t = Self {
            transition_id: [0u8; 32],
            transition_type,
            from_state,
            to_state_tag,
            epoch,
            generation,
            previous_transition_hash,
            checkpoint_root,
            lineage_head_hash,
            constitutional_delta_hash,
            causal_transition_hash,
            proof_hash: [0u8; 32],
        };
        t.transition_id = t.compute_id();
        t
    }

    fn compute_id(&self) -> [u8; 32] {
        let mut h = CanonicalHasher::with_domain(b"AMUN_TRANSITION_V1");
        h.update(&[self.transition_type.canonical_tag()]);
        h.update(&self.from_state.state_hash);
        h.update(&[self.to_state_tag.canonical_tag()]);
        h.update(&self.epoch.to_le_bytes());
        h.update(&self.generation.to_le_bytes());
        if let Some(prev) = &self.previous_transition_hash {
            h.update(prev);
        }
        h.update(&self.checkpoint_root);
        h.update(&self.lineage_head_hash);
        h.update(&self.constitutional_delta_hash);
        if let Some(causal) = &self.causal_transition_hash {
            h.update(causal);
        }
        h.finalize()
    }

    pub fn verify_monotonicity(&self) -> Result<(), String> {
        let old_epoch = self.from_state.active_since_epoch;
        let old_gen = self.from_state.active_since_generation;
        if self.epoch < old_epoch {
            return Err(format!("Epoch regression: {} -> {}", old_epoch, self.epoch));
        }
        if self.epoch == old_epoch && self.generation <= old_gen {
            return Err(format!(
                "Generation not monotonic: {} -> {} (epoch {})",
                old_gen, self.generation, self.epoch
            ));
        }
        Ok(())
    }
}

pub type TransitionId = [u8; 32];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionProof {
    pub transition_id: [u8; 32],
    pub quorum_reached: bool,
    pub validator_count: u64,
    pub required_quorum: u64,
    pub proof_data_hash: [u8; 32],
    pub proof_hash: [u8; 32],
}
