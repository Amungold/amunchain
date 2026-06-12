use crate::misbehavior::{MisbehaviorRegistry, OffenseType};

/// Minimum number of offenses before slashing is triggered
pub const DEFAULT_SLASHING_THRESHOLD: u64 = 2;

/// Percentage of stake to slash for each offense level
pub const FIRST_OFFENSE_SLASH_PERCENT: u8 = 5;
pub const SECOND_OFFENSE_SLASH_PERCENT: u8 = 10;
pub const THIRD_OFFENSE_SLASH_PERCENT: u8 = 25;

/// Returns true if the validator should be slashed based on their offense count
pub fn should_slash(registry: &MisbehaviorRegistry, validator_id: &[u8; 32]) -> bool {
    registry.offense_count(validator_id) >= DEFAULT_SLASHING_THRESHOLD
}

/// Returns the slash percentage for a validator based on their offense count
pub fn slash_percentage(registry: &MisbehaviorRegistry, validator_id: &[u8; 32]) -> u8 {
    let count = registry.offense_count(validator_id);
    match count {
        0 => 0,
        1 => FIRST_OFFENSE_SLASH_PERCENT,
        2 => SECOND_OFFENSE_SLASH_PERCENT,
        _ => THIRD_OFFENSE_SLASH_PERCENT,
    }
}

/// Returns the most severe offense type found for a validator
pub fn most_severe_offense(
    registry: &MisbehaviorRegistry,
    validator_id: &[u8; 32],
) -> Option<OffenseType> {
    let history = registry.validator_history(validator_id);
    if history.is_empty() {
        None
    } else {
        // Currently only DoubleVote exists, so return it
        Some(OffenseType::DoubleVote)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::{ConsensusVote, EquivocationProof, SignedVote};
    use crate::misbehavior::MisbehaviorRegistry;

    fn make_proof(
        validator_id: u8,
        height: u64,
        block_a: [u8; 32],
        block_b: [u8; 32],
    ) -> EquivocationProof {
        let mut proof = EquivocationProof {
            validator_id: [validator_id; 32],
            height,
            round: 1,
            vote_a: SignedVote {
                vote: ConsensusVote {
                    voter_id: [validator_id; 32],
                    height,
                    block_hash: block_a,
                    state_root: [0xBB; 32],
                    approve: true,
                    signature: [0u8; 64],
                    timestamp: 1,
                },
                signature: [1u8; 64],
            },
            vote_b: SignedVote {
                vote: ConsensusVote {
                    voter_id: [validator_id; 32],
                    height,
                    block_hash: block_b,
                    state_root: [0xBB; 32],
                    approve: true,
                    signature: [0u8; 64],
                    timestamp: 1,
                },
                signature: [2u8; 64],
            },
            detected_at_height: height + 1,
        };
        proof.vote_a.signature = [1u8; 64];
        proof.vote_b.signature = [2u8; 64];
        proof
    }

    #[test]
    fn n101_5_no_offenses_no_slash() {
        let registry = MisbehaviorRegistry::new();
        assert!(!should_slash(&registry, &[1u8; 32]));
        assert_eq!(slash_percentage(&registry, &[1u8; 32]), 0);
    }

    #[test]
    fn n101_5_single_offense_below_threshold() {
        let mut registry = MisbehaviorRegistry::new();
        let proof = make_proof(1, 10, [0xAA; 32], [0xBB; 32]);
        registry.add_proof(proof).unwrap();
        assert_eq!(registry.offense_count(&[1u8; 32]), 1);
        assert!(
            !should_slash(&registry, &[1u8; 32]),
            "Single offense should not trigger slash"
        );
        assert_eq!(
            slash_percentage(&registry, &[1u8; 32]),
            FIRST_OFFENSE_SLASH_PERCENT
        );
    }

    #[test]
    fn n101_5_two_offenses_triggers_slash() {
        let mut registry = MisbehaviorRegistry::new();
        let proof1 = make_proof(1, 10, [0xAA; 32], [0xBB; 32]);
        let proof2 = make_proof(1, 20, [0xCC; 32], [0xDD; 32]);
        registry.add_proof(proof1).unwrap();
        registry.add_proof(proof2).unwrap();
        assert_eq!(registry.offense_count(&[1u8; 32]), 2);
        assert!(
            should_slash(&registry, &[1u8; 32]),
            "Two offenses should trigger slash"
        );
        assert_eq!(
            slash_percentage(&registry, &[1u8; 32]),
            SECOND_OFFENSE_SLASH_PERCENT
        );
    }

    #[test]
    fn n101_5_three_offenses_higher_slash() {
        let mut registry = MisbehaviorRegistry::new();
        registry
            .add_proof(make_proof(1, 10, [0xAA; 32], [0xBB; 32]))
            .unwrap();
        registry
            .add_proof(make_proof(1, 20, [0xCC; 32], [0xDD; 32]))
            .unwrap();
        registry
            .add_proof(make_proof(1, 30, [0xEE; 32], [0xFF; 32]))
            .unwrap();
        assert_eq!(registry.offense_count(&[1u8; 32]), 3);
        assert_eq!(
            slash_percentage(&registry, &[1u8; 32]),
            THIRD_OFFENSE_SLASH_PERCENT
        );
    }

    #[test]
    fn n101_5_most_severe_offense() {
        let mut registry = MisbehaviorRegistry::new();
        registry
            .add_proof(make_proof(1, 10, [0xAA; 32], [0xBB; 32]))
            .unwrap();
        let offense = most_severe_offense(&registry, &[1u8; 32]);
        assert!(offense.is_some());
        assert_eq!(offense.unwrap(), OffenseType::DoubleVote);
    }
}
