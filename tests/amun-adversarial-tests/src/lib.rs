#[cfg(test)]
mod tests {
    use amun_byzantine_harness::ByzantineHarness;
    use amun_chain_position::ChainPosition;
    use amun_consensus_messages::{ConsensusPhase, ConsensusVote};

    #[test]
    fn test_equivocation_attack_detection() {
        let pos = ChainPosition::new(0, 1);

        // Validator 1 equivocates: votes for two different blocks
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
        let evidence = ByzantineHarness::detect_equivocation(&votes);
        assert_eq!(evidence.len(), 1, "Must detect equivocation");
        assert!(evidence[0].verify());
    }

    #[test]
    fn test_honest_votes_pass() {
        let pos = ChainPosition::new(0, 1);

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
            2,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xAA; 32]),
            [2u8; 64],
            25,
        );

        let votes = vec![vote_a, vote_b];
        assert!(ByzantineHarness::check_uniqueness(&votes));
    }

    #[test]
    fn test_create_vote() {
        let vote = ByzantineHarness::create_vote(1, 0, ConsensusPhase::Prevote, [0xAA; 32]);
        assert_eq!(vote.message.validator_id, 1);
        assert_eq!(vote.message.round, 0);
        assert_eq!(vote.message.block_hash, Some([0xAA; 32]));
    }
}
