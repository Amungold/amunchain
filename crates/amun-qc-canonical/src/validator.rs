use amun_consensus_messages::ConsensusVote;
use amun_validator_attestation::ValidatorSet;

/// Validate that a set of votes for a QC all come from distinct, known validators.
pub fn validate_qc_validators(
    votes: &[ConsensusVote],
    validator_set: &ValidatorSet,
) -> Result<(), &'static str> {
    let mut seen = std::collections::BTreeSet::new();

    for vote in votes {
        if !seen.insert(vote.message.validator_id) {
            return Err("duplicate validator in QC votes");
        }
        if validator_set.get_validator(vote.message.validator_id).is_none() {
            return Err("vote from unknown validator");
        }
    }

    Ok(())
}
