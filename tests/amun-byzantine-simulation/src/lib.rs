#[cfg(test)]
mod tests {
    use amun_chain_position::ChainPosition;
    use amun_consensus_messages::{ConsensusPhase, ConsensusVote};
    use amun_safety_laws::{check_vote_uniqueness, detect_equivocation};
    use amun_validator_attestation::validator_set::ValidatorInfo;
    use amun_validator_attestation::ValidatorSet;

    #[test]
    fn test_detect_equivocation_simulation() {
        let pos = ChainPosition::new(0, 1);

        // Validator 1 votes for two different blocks - equivocation
        let vote_a = ConsensusVote::new(
            1,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xAA; 32]),
            [1u8; 64],
            25,
        );
        let vote_b = ConsensusVote::new(
            1,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xBB; 32]),
            [2u8; 64],
            25,
        );

        let votes = vec![vote_a, vote_b];
        let evidence = detect_equivocation(&votes);
        assert_eq!(evidence.len(), 1);
        assert_eq!(evidence[0].validator_id(), 1);
    }

    #[test]
    fn test_vote_uniqueness_simulation() {
        let pos = ChainPosition::new(0, 1);

        let vote1 = ConsensusVote::new(
            1,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xAA; 32]),
            [1u8; 64],
            25,
        );
        let vote2 = ConsensusVote::new(
            2,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xAA; 32]),
            [2u8; 64],
            25,
        );
        let vote3 = ConsensusVote::new(
            3,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xAA; 32]),
            [3u8; 64],
            25,
        );

        let votes = vec![vote1, vote2, vote3];
        assert!(check_vote_uniqueness(&votes));
    }

    #[test]
    fn test_duplicate_validator_fails_uniqueness() {
        let pos = ChainPosition::new(0, 1);

        let vote1 = ConsensusVote::new(
            1,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xAA; 32]),
            [1u8; 64],
            25,
        );
        let vote1_dup = ConsensusVote::new(
            1,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xAA; 32]),
            [2u8; 64],
            25,
        );

        let votes = vec![vote1, vote1_dup];
        assert!(!check_vote_uniqueness(&votes));
    }

    #[test]
    fn test_validator_set_basics() {
        let set = ValidatorSet::new(
            1,
            vec![
                ValidatorInfo {
                    id: 1,
                    public_key: [1u8; 32],
                    stake: 25,
                },
                ValidatorInfo {
                    id: 2,
                    public_key: [2u8; 32],
                    stake: 25,
                },
                ValidatorInfo {
                    id: 3,
                    public_key: [3u8; 32],
                    stake: 25,
                },
                ValidatorInfo {
                    id: 4,
                    public_key: [4u8; 32],
                    stake: 25,
                },
            ],
        )
        .unwrap();

        assert!(set.get_validator(1).is_some());
        assert_eq!(set.get_validator(1).unwrap().id, 1);
        assert_eq!(set.get_validator(3).unwrap().id, 3);
        assert!(set.get_validator(99).is_none());

        let ids = set.validator_ids();
        assert_eq!(ids.len(), 4);
        assert!(ids.contains(&1));
        assert!(ids.contains(&4));
    }
}
