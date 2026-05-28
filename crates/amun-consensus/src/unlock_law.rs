// Merged from amun-unlock-law (Phase 48 Merge Strategy A)
// Functions only - tests remain in original crate until full migration
use amun_quorum_certificate::QuorumCertificate;
use amun_validator_attestation::ValidatorSet;

/// Check if a validator can be unlocked
pub fn can_unlock(validator_id: u64, qc: &QuorumCertificate, validator_set: &ValidatorSet) -> bool {
    qc.votes.iter().any(|v| v.message.validator_id == validator_id)
        && validator_set.get_validator(validator_id).is_some()
}

/// Verify unlock condition: QC must have quorum
pub fn verify_unlock_condition(qc: &QuorumCertificate, validator_set: &ValidatorSet) -> bool {
    let total: u64 = qc.votes.iter()
        .filter_map(|v| validator_set.get_validator(v.message.validator_id))
        .map(|vi| vi.stake)
        .sum();
    validator_set.has_quorum(total)
}
