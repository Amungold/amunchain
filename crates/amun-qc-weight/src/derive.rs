use amun_quorum_certificate::QuorumCertificate;
use amun_validator_attestation::ValidatorSet;

/// Derive QC weight from the ValidatorSet.
/// NEVER trust the weight field in the QC itself.
/// Returns the total verified weight.
pub fn derive_qc_weight(
    qc: &QuorumCertificate,
    validator_set: &ValidatorSet,
) -> Result<u64, &'static str> {
    let mut total_weight: u64 = 0;
    let mut seen = std::collections::BTreeSet::new();

    for vote in &qc.votes {
        if !seen.insert(vote.message.validator_id) {
            return Err("duplicate validator in QC");
        }

        let validator = validator_set
            .get_validator(vote.message.validator_id)
            .ok_or("vote from unknown validator")?;

        total_weight = total_weight
            .checked_add(validator.stake)
            .ok_or("weight overflow")?;
    }

    Ok(total_weight)
}
