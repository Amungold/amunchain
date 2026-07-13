use crate::certificate::QuorumCertificate;
use amun_validator_attestation::ValidatorSet;

/// Verify a quorum certificate against a validator set.
pub fn verify_quorum(
    qc: &QuorumCertificate,
    validator_set: &ValidatorSet,
) -> Result<(), &'static str> {
    if !qc.verify() {
        return Err("QC hash verification failed");
    }

    if !validator_set.has_quorum(qc.total_weight) {
        return Err("QC does not meet quorum threshold");
    }

    // Verify all votes are from validators in the set
    for vote in &qc.votes {
        if !vote.verify() {
            return Err("vote verification failed");
        }
        if validator_set.get_validator(vote.message.validator_id).is_none() {
            return Err("vote from unknown validator");
        }
    }

    Ok(())
}
