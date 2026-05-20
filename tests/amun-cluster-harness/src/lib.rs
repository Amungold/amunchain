#[cfg(test)]
mod tests {
    use amun_chain_position::ChainPosition;
    use amun_consensus_execution::{
        BlockDAG, BlockNode, CommitRule, ForkChoice, LeaderSchedule, Pacemaker, QCStore,
        VoteAggregator,
    };
    use amun_consensus_messages::{ConsensusPhase, ConsensusVote};
    use amun_quorum_certificate::QuorumCertificate;
    use amun_validator_attestation::validator_set::ValidatorInfo;
    use amun_validator_attestation::ValidatorSet;
    use std::time::Duration;

    fn create_test_qc(
        block_hash: [u8; 32],
        round: u64,
        position: ChainPosition,
        parent_hash: [u8; 32],
    ) -> QuorumCertificate {
        let votes = (1..=4)
            .map(|id| {
                ConsensusVote::new(
                    id,
                    position,
                    round,
                    ConsensusPhase::Prevote,
                    Some(block_hash),
                    [id as u8; 64],
                    25,
                )
            })
            .collect();
        QuorumCertificate::new(position, round, block_hash, parent_hash, votes)
    }

    fn test_validator_set() -> ValidatorSet {
        ValidatorSet::new(
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
        .unwrap()
    }

    #[test]
    fn test_vote_aggregator_partitions_by_block_hash() {
        let set = ValidatorSet::new(
            1,
            vec![
                ValidatorInfo {
                    id: 1,
                    public_key: [1u8; 32],
                    stake: 40,
                },
                ValidatorInfo {
                    id: 2,
                    public_key: [2u8; 32],
                    stake: 30,
                },
                ValidatorInfo {
                    id: 3,
                    public_key: [3u8; 32],
                    stake: 30,
                },
            ],
        )
        .unwrap();
        let mut agg = VoteAggregator::new(set);
        let pos = ChainPosition::new(0, 1);
        let va = ConsensusVote::new(
            1,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xAA; 32]),
            [1u8; 64],
            40,
        );
        assert!(agg.add_vote(va).unwrap().is_none());
        let vb = ConsensusVote::new(
            2,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xBB; 32]),
            [2u8; 64],
            30,
        );
        assert!(agg.add_vote(vb).unwrap().is_none());
        let va2 = ConsensusVote::new(
            3,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xAA; 32]),
            [3u8; 64],
            30,
        );
        let qc = agg.add_vote(va2).unwrap();
        assert!(qc.is_some());
        assert_eq!(qc.unwrap().block_hash, [0xAA; 32]);
    }

    #[test]
    fn test_equivocation_detection() {
        let set = test_validator_set();
        let mut agg = VoteAggregator::new(set);
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
        assert!(agg.add_vote(v1).is_ok());
        let v1_equiv = ConsensusVote::new(
            1,
            pos,
            0,
            ConsensusPhase::Prevote,
            Some([0xBB; 32]),
            [1u8; 64],
            25,
        );
        let result = agg.add_vote(v1_equiv);
        assert!(result.is_err());
        assert_eq!(agg.get_evidence().len(), 1);
        assert!(agg.is_slashed(1));
    }

    #[test]
    fn test_commit_rule_parent_hash_ancestry() {
        let mut commit_rule = CommitRule::new();
        let genesis = [0x00; 32];
        let block_a = [0xAA; 32];
        let block_b = [0xBB; 32];
        commit_rule.register_block(block_a, genesis, 1, 0);
        commit_rule.register_block(block_b, block_a, 2, 1);
        let qc_a = create_test_qc(block_a, 0, ChainPosition::new(0, 1), genesis);
        let qc_b = create_test_qc(block_b, 1, ChainPosition::new(0, 2), block_a);
        assert!(commit_rule.try_commit_2chain(&qc_a).is_none());
        let committed = commit_rule.try_commit_2chain(&qc_b);
        assert_eq!(committed, Some(block_a));
    }

    #[test]
    fn test_lock_rule_enforcement() {
        let mut commit_rule = CommitRule::new();
        let genesis = [0x00; 32];
        let block_a = [0xAA; 32];
        let block_b = [0xBB; 32];
        commit_rule.register_block(block_a, genesis, 1, 0);
        commit_rule.register_block(block_b, block_a, 2, 1);
        let qc_a = create_test_qc(block_a, 0, ChainPosition::new(0, 1), genesis);
        let qc_b = create_test_qc(block_b, 1, ChainPosition::new(0, 2), block_a);
        commit_rule.try_commit_2chain(&qc_a);
        commit_rule.try_commit_2chain(&qc_b);
        assert!(commit_rule.locked_qc.is_some());
    }

    #[test]
    fn test_deterministic_leader_schedule() {
        let set = test_validator_set();
        let schedule = LeaderSchedule::new(set);
        for round in 0..50 {
            let leader = schedule.leader_for_round(round);
            assert!((1..=4).contains(&leader));
        }
    }

    #[test]
    fn test_e2e_protocol_trace_proposal_to_commit() {
        let set = test_validator_set();
        let mut agg = VoteAggregator::new(set.clone());
        let mut commit_rule = CommitRule::new();
        let genesis = [0x99; 32];
        let block1 = [0x01; 32];
        let block2 = [0x02; 32];
        let pos1 = ChainPosition::new(0, 1);
        let pos2 = ChainPosition::new(0, 2);
        commit_rule.register_block(genesis, [0; 32], 0, 0);
        commit_rule.register_block(block1, genesis, 1, 0);
        commit_rule.register_block(block2, block1, 2, 1);
        let mut precommit_qc = None;
        for vid in 1..=4 {
            let vote = ConsensusVote::new(
                vid,
                pos1,
                0,
                ConsensusPhase::Precommit,
                Some(block1),
                [vid as u8; 64],
                25,
            );
            if let Ok(Some(qc)) = agg.add_vote(vote) {
                precommit_qc = Some(qc);
            }
        }
        assert!(precommit_qc.is_some());
        assert!(commit_rule
            .try_commit_2chain(&precommit_qc.unwrap())
            .is_none());
        let mut agg2 = VoteAggregator::new(set);
        let mut prevote_qc2 = None;
        for vid in 1..=4 {
            let vote = ConsensusVote::new(
                vid,
                pos2,
                1,
                ConsensusPhase::Prevote,
                Some(block2),
                [vid as u8; 64],
                25,
            );
            if let Ok(Some(qc)) = agg2.add_vote(vote) {
                prevote_qc2 = Some(qc);
            }
        }
        assert!(prevote_qc2.is_some());
        let committed = commit_rule.try_commit_2chain(&prevote_qc2.unwrap());
        assert_eq!(committed, Some(block1));
    }

    #[test]
    fn test_block_dag_ancestry() {
        let genesis = [0x00; 32];
        let mut dag = BlockDAG::new(genesis);
        let block1 = [0x01; 32];
        let block2 = [0x02; 32];
        dag.add_block(BlockNode::new(
            block1,
            Some(genesis),
            ChainPosition::new(0, 1),
            1,
            None,
            block1,
        ))
        .unwrap();
        dag.add_block(BlockNode::new(
            block2,
            Some(block1),
            ChainPosition::new(0, 2),
            2,
            None,
            block2,
        ))
        .unwrap();
        let b2 = dag.get_block(&block2).unwrap();
        assert!(b2.is_descendant_of(&genesis, &dag));
        assert!(b2.is_descendant_of(&block1, &dag));
        let chain = b2.ancestor_chain(&dag, 10);
        assert_eq!(chain.len(), 3);
    }

    #[test]
    fn test_block_dag_depth_bound() {
        let genesis = [0x00; 32];
        let mut dag = BlockDAG::new(genesis);
        let far_child = [0xFF; 32];
        dag.add_block(BlockNode::new(
            far_child,
            Some(genesis),
            ChainPosition::new(0, 1),
            1,
            None,
            far_child,
        ))
        .unwrap();
        let block = dag.get_block(&far_child).unwrap();
        assert!(block.is_descendant_of(&genesis, &dag));
    }

    #[test]
    fn test_children_index() {
        let genesis = [0x00; 32];
        let mut dag = BlockDAG::new(genesis);
        let child1 = [0x01; 32];
        let child2 = [0x02; 32];
        dag.add_block(BlockNode::new(
            child1,
            Some(genesis),
            ChainPosition::new(0, 1),
            1,
            None,
            child1,
        ))
        .unwrap();
        dag.add_block(BlockNode::new(
            child2,
            Some(genesis),
            ChainPosition::new(0, 1),
            1,
            None,
            child2,
        ))
        .unwrap();
        let children = dag.get_children(&genesis);
        assert_eq!(children.len(), 2);
        let grandchild = [0x03; 32];
        dag.add_block(BlockNode::new(
            grandchild,
            Some(child1),
            ChainPosition::new(0, 2),
            2,
            None,
            grandchild,
        ))
        .unwrap();
        let gc_children = dag.get_children(&child1);
        assert_eq!(gc_children.len(), 1);
        assert_eq!(gc_children[0].block_hash, grandchild);
    }

    #[test]
    fn test_fork_choice_canonical_tip() {
        let genesis = [0x00; 32];
        let mut dag = BlockDAG::new(genesis);
        let mut fc = ForkChoice::new();
        let pos1 = ChainPosition::new(0, 1);
        let pos2 = ChainPosition::new(0, 2);
        let block_a = [0xAA; 32];
        let a2 = [0xA2; 32];
        let qc_a1 = create_test_qc(block_a, 1, pos1, genesis);
        let qc_a2 = create_test_qc(a2, 2, pos2, block_a);
        dag.add_block(BlockNode::new(
            block_a,
            Some(genesis),
            pos1,
            1,
            Some(qc_a1.clone()),
            block_a,
        ))
        .unwrap();
        dag.add_block(BlockNode::new(
            a2,
            Some(block_a),
            pos2,
            2,
            Some(qc_a2.clone()),
            a2,
        ))
        .unwrap();
        fc.update_qc(qc_a1, &dag);
        fc.update_qc(qc_a2, &dag);
        let tip = fc.canonical_tip(&dag);
        assert_eq!(tip, Some(a2));
    }

    #[test]
    fn test_safe_node_predicate() {
        let genesis = [0x00; 32];
        let mut dag = BlockDAG::new(genesis);
        let mut fc = ForkChoice::new();
        let block_a = [0xAA; 32];
        let pos1 = ChainPosition::new(0, 1);
        let qc_a = create_test_qc(block_a, 1, pos1, genesis);
        dag.add_block(BlockNode::new(
            block_a,
            Some(genesis),
            pos1,
            1,
            Some(qc_a.clone()),
            block_a,
        ))
        .unwrap();
        fc.update_qc(qc_a.clone(), &dag);
        let pos2 = ChainPosition::new(0, 2);
        let block_b = [0xBB; 32];
        let safe_block = BlockNode::new(block_b, Some(block_a), pos2, 2, Some(qc_a), block_b);
        dag.add_block(safe_block.clone()).unwrap();
        assert!(fc.is_safe_proposal(&safe_block, &dag));
        let qc_wrong = create_test_qc([0xFF; 32], 1, pos1, genesis);
        let unsafe_block = BlockNode::new(
            [0xCC; 32],
            Some(block_a),
            pos2,
            2,
            Some(qc_wrong),
            [0xCC; 32],
        );
        dag.add_block(unsafe_block.clone()).unwrap();
        assert!(!fc.is_safe_proposal(&unsafe_block, &dag));
    }

    #[test]
    fn test_lock_monotonicity() {
        let genesis = [0x00; 32];
        let mut dag = BlockDAG::new(genesis);
        let mut fc = ForkChoice::new();
        let pos1 = ChainPosition::new(0, 1);
        let pos2 = ChainPosition::new(0, 2);
        let pos3 = ChainPosition::new(0, 3);
        let block1 = [0x01; 32];
        let block2 = [0x02; 32];
        let block3 = [0x03; 32];
        let qc1 = create_test_qc(block1, 1, pos1, genesis);
        let qc2 = create_test_qc(block2, 2, pos2, block1);
        let qc3 = create_test_qc(block3, 3, pos3, block2);
        dag.add_block(BlockNode::new(
            block1,
            Some(genesis),
            pos1,
            1,
            Some(qc1.clone()),
            block1,
        ))
        .unwrap();
        fc.update_qc(qc1, &dag);
        dag.add_block(BlockNode::new(
            block2,
            Some(block1),
            pos2,
            2,
            Some(qc2.clone()),
            block2,
        ))
        .unwrap();
        fc.update_qc(qc2, &dag);
        dag.add_block(BlockNode::new(
            block3,
            Some(block2),
            pos3,
            3,
            Some(qc3.clone()),
            block3,
        ))
        .unwrap();
        fc.update_qc(qc3, &dag);
        assert!(!fc.committed_qcs.is_empty());
        assert!(fc.verify_lock_monotonicity());
    }

    #[test]
    fn test_finalized_pruning_preserves_canonical_spine() {
        let genesis = [0x00; 32];
        let mut dag = BlockDAG::new(genesis);
        let mut prev = genesis;
        for i in 1..=5 {
            let hash = [i as u8; 32];
            dag.add_block(BlockNode::new(
                hash,
                Some(prev),
                ChainPosition::new(0, i),
                i,
                None,
                hash,
            ))
            .unwrap();
            prev = hash;
        }
        let fork = [0xF2; 32];
        dag.add_block(BlockNode::new(
            fork,
            Some([1u8; 32]),
            ChainPosition::new(0, 2),
            2,
            None,
            fork,
        ))
        .unwrap();
        dag.finalize_and_prune(3, [3u8; 32]);
        assert!(dag.get_block(&genesis).is_some());
        assert!(dag.get_block(&[1u8; 32]).is_some());
        assert!(dag.get_block(&[2u8; 32]).is_some());
        assert!(dag.get_block(&[3u8; 32]).is_some());
        assert!(dag.get_block(&fork).is_none());
        assert!(dag.get_block(&[5u8; 32]).is_some());
    }

    #[test]
    fn test_pacemaker_round_progression() {
        let mut pm = Pacemaker::new(Duration::from_millis(100));
        assert_eq!(pm.current_round, 0);
        pm.advance_round();
        assert_eq!(pm.current_round, 1);
        pm.on_timeout();
        assert_eq!(pm.current_round, 2);
        assert_eq!(pm.consecutive_timeouts, 1);
    }

    #[test]
    fn test_pacemaker_exponential_backoff() {
        let mut pm = Pacemaker::new(Duration::from_millis(1000));
        assert_eq!(pm.current_timeout(), Duration::from_millis(1000));
        pm.on_timeout();
        assert_eq!(pm.current_timeout(), Duration::from_millis(1500));
        pm.on_timeout();
        assert_eq!(pm.current_timeout(), Duration::from_millis(2250));
    }

    #[test]
    fn test_pacemaker_progress_resets_timeouts() {
        let mut pm = Pacemaker::new(Duration::from_millis(1000));
        pm.on_timeout();
        pm.on_timeout();
        assert_eq!(pm.consecutive_timeouts, 2);
        pm.on_progress(5);
        assert_eq!(pm.current_round, 6);
        assert_eq!(pm.consecutive_timeouts, 0);
    }

    #[test]
    fn test_qc_store_insert_and_retrieve() {
        let mut store = QCStore::new();
        let block = [0xAA; 32];
        let qc = create_test_qc(block, 1, ChainPosition::new(0, 1), [0x00; 32]);
        store.insert(qc.clone());
        let retrieved = store.get_by_block(&block);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().round, 1);
        assert_eq!(store.get_by_round(1).len(), 1);
    }

    #[test]
    fn test_qc_store_pruning() {
        let mut store = QCStore::new();
        for r in 0..10 {
            store.insert(create_test_qc(
                [r as u8; 32],
                r,
                ChainPosition::new(0, r + 1),
                [0x00; 32],
            ));
        }
        store.prune_below(5);
        assert_eq!(store.highest_round(), 9);
        assert!(store.get_by_round(3).is_empty());
        assert!(!store.get_by_round(5).is_empty());
    }

    #[test]
    fn test_3chain_commit_rule() {
        let mut commit_rule = CommitRule::new();
        let genesis = [0x00; 32];
        let block1 = [0x01; 32];
        let block2 = [0x02; 32];
        let block3 = [0x03; 32];
        commit_rule.register_block(block1, genesis, 1, 1);
        commit_rule.register_block(block2, block1, 2, 2);
        commit_rule.register_block(block3, block2, 3, 3);
        let qc1 = create_test_qc(block1, 1, ChainPosition::new(0, 1), genesis);
        let qc2 = create_test_qc(block2, 2, ChainPosition::new(0, 2), block1);
        let qc3 = create_test_qc(block3, 3, ChainPosition::new(0, 3), block2);
        let committed = commit_rule.try_commit_3chain(&qc3, &qc2, &qc1);
        assert_eq!(committed, Some(block1));
        assert!(commit_rule.is_committed(&block1));
    }

    #[test]
    fn test_network_partition_fork_choice_reconciliation() {
        let genesis = [0x00; 32];
        let mut merged_dag = BlockDAG::new(genesis);
        let pos1 = ChainPosition::new(0, 1);
        let pos2 = ChainPosition::new(0, 2);
        let chain_a1 = [0xA1; 32];
        let chain_a2 = [0xA2; 32];
        let chain_b1 = [0xB1; 32];
        let chain_b2 = [0xB2; 32];
        merged_dag
            .add_block(BlockNode::new(
                chain_a1,
                Some(genesis),
                pos1,
                1,
                Some(create_test_qc(chain_a1, 1, pos1, genesis)),
                chain_a1,
            ))
            .unwrap();
        merged_dag
            .add_block(BlockNode::new(
                chain_a2,
                Some(chain_a1),
                pos2,
                2,
                Some(create_test_qc(chain_a2, 2, pos2, chain_a1)),
                chain_a2,
            ))
            .unwrap();
        merged_dag
            .add_block(BlockNode::new(
                chain_b1,
                Some(genesis),
                pos1,
                1,
                Some(create_test_qc(chain_b1, 1, pos1, genesis)),
                chain_b1,
            ))
            .unwrap();
        merged_dag
            .add_block(BlockNode::new(
                chain_b2,
                Some(chain_b1),
                pos2,
                2,
                Some(create_test_qc(chain_b2, 2, pos2, chain_b1)),
                chain_b2,
            ))
            .unwrap();
        let mut merged_fc = ForkChoice::new();
        merged_fc.update_qc(create_test_qc(chain_a1, 1, pos1, genesis), &merged_dag);
        merged_fc.update_qc(create_test_qc(chain_a2, 2, pos2, chain_a1), &merged_dag);
        merged_fc.update_qc(create_test_qc(chain_b1, 1, pos1, genesis), &merged_dag);
        merged_fc.update_qc(create_test_qc(chain_b2, 2, pos2, chain_b1), &merged_dag);
        let tip = merged_fc.canonical_tip(&merged_dag);
        assert!(tip.is_some());
    }

    #[test]
    fn test_integration_dag_fork_choice_commit() {
        let genesis = [0x00; 32];
        let mut dag = BlockDAG::new(genesis);
        let mut fc = ForkChoice::new();
        let mut commit_rule = CommitRule::new();
        commit_rule.register_block(genesis, [0; 32], 0, 0);
        let block1 = [0x01; 32];
        let block2 = [0x02; 32];
        let pos1 = ChainPosition::new(0, 1);
        let pos2 = ChainPosition::new(0, 2);
        let qc1 = create_test_qc(block1, 1, pos1, genesis);
        dag.add_block(BlockNode::new(
            block1,
            Some(genesis),
            pos1,
            1,
            Some(qc1.clone()),
            block1,
        ))
        .unwrap();
        fc.update_qc(qc1.clone(), &dag);
        commit_rule.register_block(block1, genesis, 1, 1);
        let qc2 = create_test_qc(block2, 2, pos2, block1);
        dag.add_block(BlockNode::new(
            block2,
            Some(block1),
            pos2,
            2,
            Some(qc2.clone()),
            block2,
        ))
        .unwrap();
        fc.update_qc(qc2.clone(), &dag);
        commit_rule.register_block(block2, block1, 2, 2);
        assert!(commit_rule.try_commit_2chain(&qc1).is_none());
        let committed = commit_rule.try_commit_2chain(&qc2);
        assert_eq!(committed, Some(block1));
        assert!(fc.locked_qc.is_some());
    }
}
