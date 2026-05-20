#[cfg(test)]
mod tests {
    use amun_chain_position::ChainPosition;
    use amun_consensus_messages::{ConsensusPhase, ConsensusVote};
    use amun_safety_laws::{check_vote_uniqueness, detect_equivocation};

    #[test]
    fn test_distributed_equivocation_detection() {
        let pos = ChainPosition::new(0, 1);

        // Simulate distributed equivocation
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
    }

    #[test]
    fn test_distributed_vote_uniqueness() {
        let pos = ChainPosition::new(0, 1);

        let v1 = ConsensusVote::new(
            1,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xAA; 32]),
            [1u8; 64],
            25,
        );
        let v2 = ConsensusVote::new(
            2,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xAA; 32]),
            [2u8; 64],
            25,
        );
        let v3 = ConsensusVote::new(
            3,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xAA; 32]),
            [3u8; 64],
            25,
        );
        let v4 = ConsensusVote::new(
            4,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xAA; 32]),
            [4u8; 64],
            25,
        );

        let votes = vec![v1, v2, v3, v4];
        assert!(check_vote_uniqueness(&votes));
    }

    #[test]
    fn test_duplicate_detection() {
        let pos = ChainPosition::new(0, 1);

        let v1 = ConsensusVote::new(
            1,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xAA; 32]),
            [1u8; 64],
            25,
        );
        let v1_dup = ConsensusVote::new(
            1,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xAA; 32]),
            [2u8; 64],
            25,
        );

        let votes = vec![v1, v1_dup];
        assert!(!check_vote_uniqueness(&votes)); // Duplicate validator
    }
}
