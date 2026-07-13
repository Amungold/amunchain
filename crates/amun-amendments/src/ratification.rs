/// Ratification quorum requirements for constitutional amendments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RatificationQuorum {
    /// Minimum percentage of total stake required to vote
    pub participation_threshold_percent: u8,
    /// Percentage of participating stake that must approve
    pub approval_threshold_percent: u8,
    /// Minimum number of validators that must participate
    pub min_validators: u64,
    /// Maximum duration of voting period (in epochs)
    pub max_voting_duration: u64,
}

/// Proof that an amendment was properly ratified.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RatificationProof {
    pub amendment_id: [u8; 32],
    pub total_stake: u64,
    pub participating_stake: u64,
    pub approving_stake: u64,
    pub validator_count: u64,
    pub proof_hash: [u8; 32],
}
