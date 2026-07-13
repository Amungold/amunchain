1034:crates/amun-constitutional-commitment/tests/state.rs:133:    assert!(ConstitutionalState::load(&[0u8; 10]).is_none());
1035:crates/amun-constitutional-commitment/tests/state.rs:134:    assert!(ConstitutionalState::load(&[0u8; 200]).is_none());
1036:crates/amun-constitutional-commitment/tests/state.rs:135:    assert!(ConstitutionalState::load(&[]).is_none());
105:crates/amun-codec/src/versioned.rs:79:                ConstitutionalFault::ReplayViolation,
1096:crates/amun-constitutional-enforcement/src/lib.rs:252:                assert_eq!(violations[0].law, ConstitutionalLaw::StateRootIntegrity);
1102:crates/amun-constitutional-enforcement/src/proof_engine.rs:276:        assert_eq!(verdict, ConstitutionalVerdict::Constitutional);
1104:crates/amun-constitutional-enforcement/src/state_transition.rs:138:        assert_eq!(verdict, ConstitutionalVerdict::Constitutional);
1175:crates/amun-constitutional-proof/src/lib.rs:1415:        assert!(md.contains("N47 Constitutional Validation Report"));
120:crates/amun-bytecode/src/opcodes.rs:3:/// Constitutional bytecode opcodes as defined in N48.5-E Section 3.2.
1369:crates/amun-constitutional/src/execution_receipt.rs:278:            ReplayOutcome::ConstitutionalFailure,
1370:crates/amun-constitutional/src/execution_receipt.rs:292:            ReplayOutcome::ConstitutionalFailure,
1402:crates/amun-constitutional/src/causality_type.rs:109:        assert!(!CausalityType::ConstitutionalDependency.is_non_causal());
1403:crates/amun-constitutional/src/causality_type.rs:113:        assert!(CausalityType::ConstitutionalDependency.is_hard_dependency());
1406:crates/amun-constitutional/src/causality_type.rs:97:        assert!(CausalityType::ConstitutionalDependency.is_constitutional_dependency());
1414:crates/amun-constitutional/src/replay_certificate.rs:32:impl ConstitutionalIdentity for ReplayCertificate {
1415:crates/amun-constitutional/src/replay_certificate.rs:47:impl ConstitutionalObject for ReplayCertificate {
1427:crates/amun-constitutional/src/replay_outcome.rs:74:        assert!(!ReplayOutcome::ConstitutionalFailure.is_admitted());
1431:crates/amun-constitutional/src/replay_outcome.rs:82:        assert!(ReplayOutcome::ConstitutionalFailure.is_failure());
144:crates/amun-certificate-network/src/distribution.rs:7:    CertificateInclusionProof, ConstitutionalStateRuntime, ReplayCertificate,
1456:crates/amun-constitutional/src/divergence_type.rs:75:        assert!(DivergenceType::ConstitutionalFork.is_admissible());
1458:crates/amun-constitutional/src/divergence_type.rs:77:        assert!(DivergenceType::ConstitutionalSupersession.is_admissible());
1461:crates/amun-constitutional/src/divergence_type.rs:83:        assert!(!DivergenceType::ConstitutionalFork.is_error());
1494:crates/amun-constitutional/src/replay_outcome.rs:74:        assert!(!ReplayOutcome::ConstitutionalFailure.is_admitted());
1498:crates/amun-constitutional/src/replay_outcome.rs:82:        assert!(ReplayOutcome::ConstitutionalFailure.is_failure());
1713:crates/amun-failure/src/tests.rs:10:    assert!(ConstitutionalFault::ConstitutionalViolation.should_halt());
1715:crates/amun-failure/src/tests.rs:11:    assert!(ConstitutionalFault::InvalidQuorum.should_halt());
1718:crates/amun-failure/src/tests.rs:12:    assert!(ConstitutionalFault::SignatureInvalid.should_halt());
1719:crates/amun-failure/src/tests.rs:13:    assert!(ConstitutionalFault::MerkleProofInvalid.should_halt());
1720:crates/amun-failure/src/tests.rs:14:    assert!(ConstitutionalFault::DurabilityViolation.should_halt());
1721:crates/amun-failure/src/tests.rs:15:    assert!(ConstitutionalFault::JournalHashMismatch.should_halt());
1722:crates/amun-failure/src/tests.rs:16:    assert!(ConstitutionalFault::ArithmeticOverflow.should_halt());
1723:crates/amun-failure/src/tests.rs:17:    assert!(ConstitutionalFault::ArithmeticUnderflow.should_halt());
1724:crates/amun-failure/src/tests.rs:18:    assert!(ConstitutionalFault::DecodeBudgetExceeded.should_halt());
1725:crates/amun-failure/src/tests.rs:19:    assert!(ConstitutionalFault::CryptoBudgetExceeded.should_halt());
1726:crates/amun-failure/src/tests.rs:24:    assert!(!ConstitutionalFault::BufferTooSmall.should_halt());
1727:crates/amun-failure/src/tests.rs:25:    assert!(!ConstitutionalFault::CapacityExceeded.should_halt());
1728:crates/amun-failure/src/tests.rs:26:    assert!(!ConstitutionalFault::TableFull.should_halt());
1729:crates/amun-failure/src/tests.rs:27:    assert!(!ConstitutionalFault::MemoryBudgetExhausted.should_halt());
1730:crates/amun-failure/src/tests.rs:32:    assert!(!ConstitutionalFault::InvalidStateTransition.should_halt());
1731:crates/amun-failure/src/tests.rs:33:    assert!(!ConstitutionalFault::UninitializedAccess.should_halt());
1732:crates/amun-failure/src/tests.rs:34:    assert!(!ConstitutionalFault::DoubleInitialization.should_halt());
1733:crates/amun-failure/src/tests.rs:35:    assert!(!ConstitutionalFault::TemporalViolation.should_halt());
1734:crates/amun-failure/src/tests.rs:36:    assert!(!ConstitutionalFault::ReplayViolation.should_halt());
1735:crates/amun-failure/src/tests.rs:37:    assert!(!ConstitutionalFault::SequenceMismatch.should_halt());
1736:crates/amun-failure/src/tests.rs:47:    assert_eq!(ctx.fault, ConstitutionalFault::CapacityExceeded);
1742:crates/amun-failure/src/tests.rs:83:        assert_eq!(fault, ConstitutionalFault::EquivocationDetected);
1744:crates/amun-failure/src/tests.rs:8:    assert!(ConstitutionalFault::EquivocationDetected.should_halt());
1749:crates/amun-failure/src/tests.rs:9:    assert!(ConstitutionalFault::UnsafeContractViolation.should_halt());
1756:crates/amun-failure/src/tests.rs:36:    assert!(!ConstitutionalFault::ReplayViolation.should_halt());
177:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:100:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
178:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:108:        let block = ConstitutionalBlock::new(
179:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:133:    let mut rt_check = ConstitutionalStateRuntime::new();
180:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:138:    let mut rt_verify = ConstitutionalStateRuntime::new();
181:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:13:    let mut rt_a = ConstitutionalStateRuntime::new();
182:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:21:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
183:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:23:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
184:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:31:        let block = ConstitutionalBlock::new(
185:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:69:    let mut rt_b = ConstitutionalStateRuntime::new();
186:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:8:use amun_constitutional_block::ConstitutionalBlock;
187:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:90:    let mut rt = ConstitutionalStateRuntime::new();
188:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:98:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
189:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:9:use amun_constitutional_state::ConstitutionalStateRuntime;
190:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:14:    let mut rt = ConstitutionalStateRuntime::new();
1910:crates/amun-live-cluster/src/validator.rs:14:        ConstitutionalEvidenceRecord, DoubleSpendEvidence, GovernanceEvidence, ReplayEvidence,
1913:crates/amun-live-cluster/src/validator.rs:568:                    // N126.3: Evidence-Based Constitutional Verification
1917:crates/amun-live-cluster/src/validator.rs:648:                        // N129.2: Build ConstitutionalEvidenceRecord from real evidence data
191:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:22:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
1924:crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:53:    assert_eq!(verdict, ConstitutionalVerdict::Constitutional);
192:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:24:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
193:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:32:        let block = ConstitutionalBlock::new(
1940:crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:39:                    .any(|v| v.law == ConstitutionalLaw::FinalitySupermajority),
194:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:7:use amun_constitutional_block::ConstitutionalBlock;
195:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:8:use amun_constitutional_state::ConstitutionalStateRuntime;
1961:crates/amun-networking/src/risk.rs:61:            ConstitutionalRisk::ReplayInstability { .. } => {
1966:crates/amun-networking/tests/n18_node_rejoin.rs:73:// N18.5 — Constitutional Invariant REJOIN-001
2474:crates/amun-replay-engine/src/byzantine_witness_filter.rs:13:use amun_constitutional::ConstitutionalWitness;
2483:crates/amun-replay-engine/src/byzantine_witness_filter.rs:42:pub fn filter_incoming_witness(witness: &ConstitutionalWitness) -> FilterResult {
2488:crates/amun-replay-engine/src/byzantine_witness_filter.rs:87:    fn make_witness(entries: Vec<WitnessEntry>) -> ConstitutionalWitness {
284:crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:141:// N127A.1 — Constitutional Evidence Builder
2850:crates/amun-snapshot-engine/tests/constitutional_tests.rs:66:        assert!(matches!(rel, ConstitutionalRelationship::Identical));
290:crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:201:        let evidence = ConstitutionalEvidenceBuilder::build(exec, replay, gov, qc, true, true);
303:crates/amun-constitutional-enforcement/src/proof_engine.rs:1:// N124 — Constitutional Proof Engine
492:crates/amun-constitutional/src/replay_certificate.rs:101:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
493:crates/amun-constitutional/src/replay_certificate.rs:114:    fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
497:crates/amun-constitutional/src/replay_certificate.rs:66:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
498:crates/amun-constitutional/src/replay_certificate.rs:88:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
620:crates/amun-constitutional-block/src/lib.rs:39:use amun_constitutional_state::{ConstitutionalStateRuntime, ReplayCertificate};
627:crates/amun-failure/src/tests.rs:32:    assert!(!ConstitutionalFault::InvalidStateTransition.should_halt());
638:crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:141:// N127A.1 — Constitutional Evidence Builder
654:crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:201:        let evidence = ConstitutionalEvidenceBuilder::build(exec, replay, gov, qc, true, true);
657:crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:2:// N127A — Constitutional Evidence Interface
689:crates/amun-constitutional-enforcement/src/evidence_providers.rs:7:    ConstitutionalEvidence, ExecutionEvidence, GovernanceEvidence, QcEvidence, ReplayEvidence,
726:crates/amun-networking/tests/n18_node_rejoin.rs:73:// N18.5 — Constitutional Invariant REJOIN-001
732:crates/amun-constitutional-enforcement/src/lib.rs:110:                ConstitutionalLaw::ReplayDeterminism,
736:crates/amun-constitutional-enforcement/src/lib.rs:169:                &ConstitutionalLaw::ReplayDeterminism,
748:crates/amun-constitutional-enforcement/src/proof_engine.rs:173:                law: ConstitutionalLaw::ReplayDeterminism,
757:crates/amun-constitutional-enforcement/src/state_transition.rs:158:                    .any(|v| v.law == ConstitutionalLaw::ReplayDeterminism));
759:crates/amun-constitutional-enforcement/src/state_transition.rs:57:                law: ConstitutionalLaw::ReplayDeterminism,
776:crates/amun-constitutional-geometry/src/flow_dynamics.rs:64:                ConstitutionalForce::ReplayForce { strength } => {
93:crates/amun-certificate-network/src/distribution.rs:7:    CertificateInclusionProof, ConstitutionalStateRuntime, ReplayCertificate,
980:crates/amun-replay-engine/src/canonical.rs:178:    pub fn finalize(&self) -> ConstitutionalHash {
crates/amun-accounts/src/lib.rs:179:    fn n25_create_account() {
crates/amun-accounts/src/lib.rs:186:    fn n25_balance_lookup() {
crates/amun-accounts/src/lib.rs:194:    fn n25_debit_success() {
crates/amun-accounts/src/lib.rs:202:    fn n25_insufficient_balance() {
crates/amun-accounts/src/lib.rs:210:    fn n25_credit_creates_account() {
crates/amun-accounts/src/lib.rs:217:    fn n25_nonce_increment() {
crates/amun-accounts/src/lib.rs:225:    fn n25_state_determinism() {
crates/amun-accounts/src/lib.rs:238:    fn n25_different_state_different_root() {
crates/amun-accounts/src/lib.rs:76:    // N132.2 — EconomicSnapshot builders
crates/amun-audit-trail/src/lib.rs:102:        assert!(trail.verify());
crates/amun-audit-trail/src/lib.rs:107:    fn n38_broken_trail_detected() {
crates/amun-audit-trail/src/lib.rs:113:        assert!(!trail.verify());
crates/amun-audit-trail/src/lib.rs:117:    fn n38_record_hash_deterministic() {
crates/amun-audit-trail/src/lib.rs:126:    fn n38_different_records_different_hash() {
crates/amun-audit-trail/src/lib.rs:12:    pub replay_certificate: [u8; 32],
crates/amun-audit-trail/src/lib.rs:25:        hasher.update(&self.replay_certificate);
crates/amun-audit-trail/src/lib.rs:48:        replay_certificate: [u8; 32],
crates/amun-audit-trail/src/lib.rs:61:            replay_certificate,
crates/amun-audit-trail/src/lib.rs:89:    fn n38_single_audit_record() {
crates/amun-audit-trail/src/lib.rs:93:        assert!(trail.verify());
crates/amun-audit-trail/src/lib.rs:97:    fn n38_audit_trail_chain() {
crates/amun-audit/tests/audit_layer06_replay.rs:100:    fn replay003_epoch_regression() {
crates/amun-audit/tests/audit_layer06_replay.rs:102:        let path = dir.path().join("replay003.wal");
crates/amun-audit/tests/audit_layer06_replay.rs:135:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer06_replay.rs:25:    // CONST-REPLAY-001: Replay produces identical state
crates/amun-audit/tests/audit_layer06_replay.rs:27:    fn replay001_equivalence() {
crates/amun-audit/tests/audit_layer06_replay.rs:29:        let path = dir.path().join("replay001.wal");
crates/amun-audit/tests/audit_layer06_replay.rs:2:mod audit_replay {
crates/amun-audit/tests/audit_layer06_replay.rs:3:    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry};
crates/amun-audit/tests/audit_layer06_replay.rs:50:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer06_replay.rs:52:            Ok((replayed_root, count)) => {
crates/amun-audit/tests/audit_layer06_replay.rs:59:                    replayed_root, root.0,
crates/amun-audit/tests/audit_layer06_replay.rs:60:                    "CONST-REPLAY-001 VIOLATION: Replayed root diverges"
crates/amun-audit/tests/audit_layer06_replay.rs:63:            Err(e) => panic!("CONST-REPLAY-001: Replay failed: {}", e),
crates/amun-audit/tests/audit_layer06_replay.rs:67:    // CONST-REPLAY-002: Replay detects state root divergence
crates/amun-audit/tests/audit_layer06_replay.rs:69:    fn replay002_divergence_detection() {
crates/amun-audit/tests/audit_layer06_replay.rs:71:        let path = dir.path().join("replay002.wal");
crates/amun-audit/tests/audit_layer06_replay.rs:91:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer06_replay.rs:98:    // CONST-REPLAY-003: Replay detects epoch regression
crates/amun-audit/tests/audit_layer06_replay.rs:9:    fn create_wal(path: &str, entries: &[WalEntry]) -> std::io::Result<()> {
crates/amun-audit/tests/audit_layer08_domains.rs:13:    fn domain001_all_domains_unique() {
crates/amun-audit/tests/audit_layer08_domains.rs:38:    fn domain002_chain_id_32_bytes() {
crates/amun-audit/tests/audit_layer08_domains.rs:52:    fn domain003_domains_are_versioned() {
crates/amun-audit/tests/audit_layer11_crash.rs:124:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer11_crash.rs:127:            "CONST-CRASH-003 VIOLATION: Corrupted frame must fail replay"
crates/amun-audit/tests/audit_layer11_crash.rs:3:    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry, WalIterator};
crates/amun-audit/tests/audit_layer15_temporal.rs:25:    // CONST-TEMP-001: Replaying same WAL twice produces identical root
crates/amun-audit/tests/audit_layer15_temporal.rs:27:    fn temp001_replay_twice_same_root() {
crates/amun-audit/tests/audit_layer15_temporal.rs:3:    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry};
crates/amun-audit/tests/audit_layer15_temporal.rs:50:        let result1 = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer15_temporal.rs:51:        let result2 = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer15_temporal.rs:58:                    "CONST-TEMP-001 VIOLATION: Replay twice produces different roots"
crates/amun-audit/tests/audit_layer15_temporal.rs:62:            (Err(e), _) | (_, Err(e)) => panic!("CONST-TEMP-001: Replay failed: {}", e),
crates/amun-authority-registry/src/authority.rs:35:    fn n107_1_authority_id_deterministic() {
crates/amun-authority-registry/src/authority.rs:43:    fn n107_1_different_keys_different_ids() {
crates/amun-authority-registry/src/executor.rs:130:    fn n107_7c_execute_add_authority() {
crates/amun-authority-registry/src/executor.rs:145:        assert!(journal.is_executed(&proposal.proposal_id));
crates/amun-authority-registry/src/executor.rs:149:    fn n107_7c_execute_transition() {
crates/amun-authority-registry/src/executor.rs:165:        assert!(reg.transition.is_some());
crates/amun-authority-registry/src/executor.rs:169:    fn n107_7c_execute_retirement() {
crates/amun-authority-registry/src/executor.rs:17:/// Tracks which proposals have been executed to prevent replay.
crates/amun-authority-registry/src/executor.rs:186:    fn n107_7c_reject_without_quorum() {
crates/amun-authority-registry/src/executor.rs:202:    fn n107_7c_reject_failed_vote() {
crates/amun-authority-registry/src/executor.rs:219:    fn n107_7c_idempotent_execution() {
crates/amun-authority-registry/src/governance.rs:100:    fn n107_7_retire_authority_proposal() {
crates/amun-authority-registry/src/governance.rs:109:    fn n107_7_different_actions_produce_different_ids() {
crates/amun-authority-registry/src/governance.rs:128:    fn n107_7_reject_invalid_governance_duplicate_proposal() {
crates/amun-authority-registry/src/governance.rs:78:    fn n107_7_add_authority_proposal() {
crates/amun-authority-registry/src/governance.rs:88:    fn n107_7_schedule_transition_proposal() {
crates/amun-authority-registry/src/recovery.rs:105:        // This entry is AFTER the snapshot height and should be replayed
crates/amun-authority-registry/src/recovery.rs:10:    /// replaying only WAL entries whose block_height is greater than
crates/amun-authority-registry/src/recovery.rs:134:    fn n107_8c_journal_recovery() {
crates/amun-authority-registry/src/recovery.rs:163:        assert!(recovered.journal.is_executed(&proposal.proposal_id));
crates/amun-authority-registry/src/recovery.rs:169:    fn n107_8c_vote_recovery() {
crates/amun-authority-registry/src/recovery.rs:210:    fn n107_8c_deterministic_recovery() {
crates/amun-authority-registry/src/recovery.rs:22:        // 2. Replay only WAL entries after the snapshot height
crates/amun-authority-registry/src/recovery.rs:56:    fn n107_8c_recover_from_snapshot_and_wal() {
crates/amun-authority-registry/src/recovery.rs:89:    fn n107_8c_partial_replay() {
crates/amun-authority-registry/src/registry.rs:173:    fn n107_2_register_and_activate() {
crates/amun-authority-registry/src/registry.rs:181:    fn n107_2_revoke() {
crates/amun-authority-registry/src/registry.rs:190:    fn n107_2_active_tracks_latest() {
crates/amun-authority-registry/src/registry.rs:198:    fn n107_3_registry_bootstrap() {
crates/amun-authority-registry/src/registry.rs:205:    fn n107_5_activate_new_authority() {
crates/amun-authority-registry/src/registry.rs:217:    fn n107_5_retire_authority() {
crates/amun-authority-registry/src/registry.rs:227:    fn n107_5_multi_version_registry() {
crates/amun-authority-registry/src/registry.rs:239:    fn n107_5_cross_epoch_verification() {
crates/amun-authority-registry/src/registry.rs:252:    fn n107_5_sunset_rejection() {
crates/amun-authority-registry/src/registry.rs:262:    fn n107_6_schedule_transition() {
crates/amun-authority-registry/src/registry.rs:275:        assert!(reg.transition.is_some());
crates/amun-authority-registry/src/registry.rs:279:    fn n107_6_pre_activation() {
crates/amun-authority-registry/src/registry.rs:297:    fn n108_3b_accept_v1_before_grace_end() {
crates/amun-authority-registry/src/registry.rs:338:    fn n108_3b_reject_v1_after_grace_end() {
crates/amun-authority-registry/src/registry.rs:378:    fn n108_3b_accept_v2_after_activation() {
crates/amun-authority-registry/src/registry.rs:417:    fn n108_3b_reject_revoked_authority() {
crates/amun-authority-registry/src/registry.rs:446:    fn n107_6_dual_validation_window() {
crates/amun-authority-registry/src/registry.rs:463:    fn n107_6_post_grace_period() {
crates/amun-authority-registry/src/registry.rs:481:    fn n107_6_old_authority_cannot_issue_after_activation() {
crates/amun-authority-registry/src/registry.rs:493:        assert!(!reg.can_issue_at(1, 150));
crates/amun-authority-registry/src/registry.rs:494:        assert!(reg.can_issue_at(2, 150));
crates/amun-authority-registry/src/registry.rs:498:    fn n107_6_historical_certificates_survive() {
crates/amun-authority-registry/src/transaction.rs:100:    fn n107_7d_submit_proposal_transaction() {
crates/amun-authority-registry/src/transaction.rs:113:    fn n107_7d_cast_vote_transaction() {
crates/amun-authority-registry/src/transaction.rs:138:    fn n107_7d_finalize_block_executes_governance() {
crates/amun-authority-registry/src/transaction.rs:160:        assert_eq!(executed.len(), 1);
crates/amun-authority-registry/src/transaction.rs:165:    fn n107_8a_snapshot_roundtrip() {
crates/amun-authority-registry/src/transaction.rs:193:    fn n107_8a_empty_snapshot() {
crates/amun-authority-registry/src/transaction.rs:201:    fn n107_8a_multiple_proposals() {
crates/amun-authority-registry/src/transaction.rs:216:    fn n107_8a_executed_proposals_survive_snapshot() {
crates/amun-authority-registry/src/transaction.rs:238:        assert!(restored.journal.is_executed(&proposal.proposal_id));
crates/amun-authority-registry/src/voting.rs:107:    fn n107_7_majority_approval() {
crates/amun-authority-registry/src/voting.rs:117:    fn n107_7_majority_rejection() {
crates/amun-authority-registry/src/voting.rs:127:    fn n107_7_abstain_counted_for_quorum() {
crates/amun-authority-registry/src/voting.rs:137:    fn n107_7_empty_vote_set() {
crates/amun-authority-registry/src/voting.rs:80:    fn n107_7_vote_submission() {
crates/amun-authority-registry/src/voting.rs:87:    fn n107_7_duplicate_vote_replaces_old_vote() {
crates/amun-authority-registry/src/voting.rs:98:    fn n107_7_quorum_required() {
crates/amun-authority-registry/src/wal.rs:109:    fn n107_8b_replay_execution() {
crates/amun-authority-registry/src/wal.rs:12:/// and provides replay functionality.
crates/amun-authority-registry/src/wal.rs:133:        let restored = wal.replay_and_finalize(4, &mut registry).unwrap();
crates/amun-authority-registry/src/wal.rs:134:        assert!(restored.journal.is_executed(&proposal.proposal_id));
crates/amun-authority-registry/src/wal.rs:139:    fn n107_8b_replay_after_crash() {
crates/amun-authority-registry/src/wal.rs:152:        // Simulate crash: create a new WAL and replay from persisted entries
crates/amun-authority-registry/src/wal.rs:155:        let state = restored_wal.replay();
crates/amun-authority-registry/src/wal.rs:160:    fn n107_8b_replay_deterministic() {
crates/amun-authority-registry/src/wal.rs:171:        let state1 = wal1.replay();
crates/amun-authority-registry/src/wal.rs:172:        let state2 = wal2.replay();
crates/amun-authority-registry/src/wal.rs:31:    /// Replay all entries in order against a fresh GovernanceState.
crates/amun-authority-registry/src/wal.rs:32:    pub fn replay(&self) -> GovernanceState {
crates/amun-authority-registry/src/wal.rs:40:    /// Replay and then finalize all approved proposals.
crates/amun-authority-registry/src/wal.rs:41:    pub fn replay_and_finalize(
crates/amun-authority-registry/src/wal.rs:46:        let mut state = self.replay();
crates/amun-authority-registry/src/wal.rs:67:    fn n107_8b_replay_single_proposal() {
crates/amun-authority-registry/src/wal.rs:76:        let restored = wal.replay();
crates/amun-authority-registry/src/wal.rs:81:    fn n107_8b_replay_votes() {
crates/amun-authority-registry/src/wal.rs:97:        let restored = wal.replay();
crates/amun-bench/tests/n161_state_root_bench.rs:10:    let mut reg = ResourceRegistry::new(20000);
crates/amun-bench/tests/n161_state_root_bench.rs:16:        lineage: ResourceLineage::genesis(col_id),
crates/amun-bench/tests/n161_state_root_bench.rs:32:                lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
crates/amun-bench/tests/n161_state_root_bench.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-bench/tests/n161_state_root_bench.rs:41:        let _root = reg.compute_state_root();
crates/amun-bench/tests/n161_state_root_bench.rs:9:fn n161_bench_state_root_10k_nft() {
crates/amun-bench/tests/n162_snapshot_bench.rs:10:    let mut reg = ResourceRegistry::new(20000);
crates/amun-bench/tests/n162_snapshot_bench.rs:16:        lineage: ResourceLineage::genesis(col_id),
crates/amun-bench/tests/n162_snapshot_bench.rs:32:                lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
crates/amun-bench/tests/n162_snapshot_bench.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-bench/tests/n162_snapshot_bench.rs:9:fn n162_bench_snapshot_build_and_restore() {
crates/amun-bench/tests/n163_wal_bench.rs:5:fn n163_bench_wal_write_and_read() {
crates/amun-benchmarks/benches/consensus_bench.rs:14:        commitment: None,
crates/amun-benchmarks/benches/consensus_bench.rs:18:fn bench_single_round(c: &mut Criterion) {
crates/amun-benchmarks/benches/consensus_bench.rs:1:use amun_consensus_network::engine::ConsensusEngine;
crates/amun-benchmarks/benches/consensus_bench.rs:21:            let mut engine = ConsensusEngine::new([0u8; 32], 4);
crates/amun-benchmarks/benches/consensus_bench.rs:34:fn bench_multi_round_10(c: &mut Criterion) {
crates/amun-benchmarks/benches/consensus_bench.rs:37:            let mut engine = ConsensusEngine::new([0u8; 32], 4);
crates/amun-benchmarks/benches/consensus_bench.rs:58:fn bench_vote_serialization(c: &mut Criterion) {
crates/amun-benchmarks/benches/consensus_bench.rs:5:fn make_vote(voter: u8, height: u64, block_hash: [u8; 32]) -> ConsensusVote {
crates/amun-benchmarks/benches/storage_bench.rs:13:        commitment_root: [0u8; 32],
crates/amun-benchmarks/benches/storage_bench.rs:25:fn bench_append_100_records(c: &mut Criterion) {
crates/amun-benchmarks/benches/storage_bench.rs:38:fn bench_read_100_records(c: &mut Criterion) {
crates/amun-benchmarks/benches/storage_bench.rs:55:fn bench_record_serialization(c: &mut Criterion) {
crates/amun-benchmarks/benches/storage_bench.rs:5:fn make_record(h: u64) -> FinalizedChainRecord {
crates/amun-benchmarks/benches/sync_bench.rs:1:use amun_resource_core::ResourceRegistry;
crates/amun-benchmarks/benches/sync_bench.rs:21:            lineage: ResourceLineage::genesis(id),
crates/amun-benchmarks/benches/sync_bench.rs:30:fn bench_snapshot_create_1k(c: &mut Criterion) {
crates/amun-benchmarks/benches/sync_bench.rs:40:fn bench_snapshot_import_1k(c: &mut Criterion) {
crates/amun-benchmarks/benches/sync_bench.rs:47:            black_box(imported.compute_state_root());
crates/amun-benchmarks/benches/sync_bench.rs:52:fn bench_snapshot_create_10k(c: &mut Criterion) {
crates/amun-benchmarks/benches/sync_bench.rs:5:fn create_test_registry(size: u64) -> ResourceRegistry {
crates/amun-benchmarks/benches/sync_bench.rs:6:    let mut reg = ResourceRegistry::new(size as usize * 2);
crates/amun-benchmarks/benches/sync_bench.rs:7:    use amun_resource_core::resource_lineage::ResourceLineage;
crates/amun-block-builder/src/lib.rs:195:    fn n27_build_block_with_transactions() {
crates/amun-block-builder/src/lib.rs:222:        assert_eq!(builder.engine.state.balance_of(&a1), 900);
crates/amun-block-builder/src/lib.rs:223:        assert_eq!(builder.engine.state.balance_of(&a2), 600);
crates/amun-block-builder/src/lib.rs:229:    fn n27_block_hash_deterministic() {
crates/amun-block-builder/src/lib.rs:245:    fn n27_different_state_different_block_hash() {
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:105:    let block = build_test_block(vec![cert]);
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:106:    let result = block.verify_slashing_certificates();
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:114:fn n110_4b_hash_mismatch_rejected() {
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:118:    let block = build_test_block(vec![cert]);
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:119:    let result = block.verify_slashing_certificates();
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:131:fn n110_4b_empty_certificates_list_passes() {
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:132:    let block = build_test_block(vec![]);
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:134:        block.verify_slashing_certificates().is_ok(),
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:14:        vec![EvidenceCount {
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:15:            evidence_type: EvidenceType::DoubleVote,
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:29:fn build_test_block(certs: Vec<SlashingCertificate>) -> Block {
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:30:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:32:    builder.build_block_with_certificates(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000, certs)
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:40:fn n110_4b_valid_certificates_pass_verification() {
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:42:    let block = build_test_block(vec![cert]);
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:44:        block.verify_slashing_certificates().is_ok(),
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:4:use amun_block_builder::{Block, BlockBuilder};
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:50:fn n110_4b_tampered_certificate_rejected() {
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:53:                             // Recompute hash to keep it consistent? No, we want hash mismatch.
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:54:                             // verify() should catch either amount_slashed=0 or hash mismatch.
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:55:    let block = build_test_block(vec![cert]);
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:56:    let result = block.verify_slashing_certificates();
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:5:use amun_consensus_network::{EvidenceCount, EvidenceType, SlashingCertificate, ValidatorStatus};
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:68:fn n110_4b_too_many_certificates_rejected() {
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:73:    let block = build_test_block(certs);
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:74:    let result = block.verify_slashing_certificates();
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:88:fn n110_4b_empty_evidence_ids_rejected() {
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:93:        vec![EvidenceCount {
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:94:            evidence_type: EvidenceType::DoubleVote,
crates/amun-block-builder/tests/n110_4b_slashing_certificate_verification.rs:9:fn make_valid_certificate(validator_id: [u8; 32]) -> SlashingCertificate {
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:10:    let sender = sk.verifying_key().to_bytes();
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:23:fn n111_cca_state_root_preserved_through_block() {
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:24:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:29:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:2:use amun_block_builder::BlockBuilder;
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:33:    builder.engine.state.create_account(a1, 1000);
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:38:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:39:    let expected_state_root = builder
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:42:        .state_root_with_ledger(&builder.engine.economic);
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:58:fn n111_cca_state_root_changes_reflected_in_block() {
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:59:    let mut builder1 = BlockBuilder::new();
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:60:    let mut builder2 = BlockBuilder::new();
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:64:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:67:    builder1.engine.state.create_account(a, 1000);
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:68:    builder2.engine.state.create_account(a, 999);
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:70:    let block1 = builder1.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:71:    let block2 = builder2.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:7:fn create_signed_transfer(seed: u8, nonce: u64, amount: u64, to: [u8; 32]) -> Transaction {
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:85:fn n111_cca_raw_state_root_differs_from_cca_state_root() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:100:        block.verify_slashing_root(&[0u8; 32]).is_ok(),
crates/amun-block-builder/tests/n120_2_slashing_root.rs:104:        block.verify_slashing_root(&[0x01; 32]).is_err(),
crates/amun-block-builder/tests/n120_2_slashing_root.rs:14:fn n120_2_block_hash_changes_with_slashing_root() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:15:    let block1 = build_block_with_root([0u8; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:16:    let mut block2 = build_block_with_root([0u8; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:28:fn n120_2_same_root_same_hash() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:29:    let block1 = build_block_with_root([0x42; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:2:use amun_block_builder::{Block, BlockBuilder};
crates/amun-block-builder/tests/n120_2_slashing_root.rs:30:    let block2 = build_block_with_root([0x42; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:41:fn n120_2_empty_root_allowed() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:42:    let block = build_block_with_root([0u8; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:51:fn n120_2_different_roots_different_hashes() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:54:    let block_a = build_block_with_root(root_a);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:55:    let block_b = build_block_with_root(root_b);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:5:fn build_block_with_root(root: [u8; 32]) -> Block {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:64:fn n120_2_zero_root_vs_nonzero_root_different_hash() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:65:    let block_zero = build_block_with_root([0u8; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:66:    let block_nonzero = build_block_with_root([0xAB; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:6:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n120_2_slashing_root.rs:76:fn n120_3_matching_root_accepted() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:78:    let block = build_block_with_root(root);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:80:        block.verify_slashing_root(&root).is_ok(),
crates/amun-block-builder/tests/n120_2_slashing_root.rs:86:fn n120_3_mismatched_root_rejected() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:87:    let block = build_block_with_root([0x42; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:88:    let result = block.verify_slashing_root(&[0xFF; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:8:    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:97:fn n120_3_zero_root_verified_correctly() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:98:    let block = build_block_with_root([0u8; 32]);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:102:fn n132_3_6_economic_change_changes_constitutional_roots() {
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:103:    let mut b1 = BlockBuilder::new();
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:104:    let mut b2 = BlockBuilder::new();
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:108:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:10:    let sender = sk.verifying_key().to_bytes();
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:119:    let block1 = b1.build_block(1, [0u8; 32], &mut mp1, 10, [0u8; 32], 1000);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:121:    let block2 = b2.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:1:use amun_block_builder::BlockBuilder;
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:25:fn n132_3_6_economic_root_consistent_with_snapshot() {
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:26:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:2:use amun_constitutional_commitment::EconomicTree;
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:31:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:36:    builder.engine.state.create_account(a1, 10000);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:42:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:44:    let recomputed_economic =
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:48:        block.economic_root, recomputed_economic,
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:49:        "economic_root must match recomputed root from snapshot"
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:54:fn n132_3_6_consistent_with_snapshot() {
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:55:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:60:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:65:    builder.engine.state.create_account(a1, 10000);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:71:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:73:    // Recompute the complete constitutional roots using the same canonical
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:74:    // pipeline as BlockBuilder.
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:75:    let expected = builder
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:78:        .constitutional_roots_with_ledger(&builder.engine.economic);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:7:fn create_signed_transfer(seed: u8, nonce: u64, amount: u64, to: [u8; 32]) -> Transaction {
crates/amun-block-builder/tests/n28_first_economic_block.rs:110:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-block-builder/tests/n28_first_economic_block.rs:113:    assert_eq!(builder.engine.state.balance_of(&alice), 100);
crates/amun-block-builder/tests/n28_first_economic_block.rs:12:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n28_first_economic_block.rs:15:    builder.engine.state.create_account(alice, 1000);
crates/amun-block-builder/tests/n28_first_economic_block.rs:1:use amun_block_builder::BlockBuilder;
crates/amun-block-builder/tests/n28_first_economic_block.rs:29:    let genesis_root = builder.engine.state.state_root();
crates/amun-block-builder/tests/n28_first_economic_block.rs:30:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 123456);
crates/amun-block-builder/tests/n28_first_economic_block.rs:31:    assert_eq!(builder.engine.state.balance_of(&alice), 700);
crates/amun-block-builder/tests/n28_first_economic_block.rs:32:    assert_eq!(builder.engine.state.balance_of(&bob), 300);
crates/amun-block-builder/tests/n28_first_economic_block.rs:41:fn n28_multiple_transfers_in_block() {
crates/amun-block-builder/tests/n28_first_economic_block.rs:42:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n28_first_economic_block.rs:47:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n28_first_economic_block.rs:51:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n28_first_economic_block.rs:54:    builder.engine.state.create_account(a1, 1000);
crates/amun-block-builder/tests/n28_first_economic_block.rs:55:    builder.engine.state.create_account(a2, 500);
crates/amun-block-builder/tests/n28_first_economic_block.rs:7:fn n28_first_economic_block() {
crates/amun-block-builder/tests/n28_first_economic_block.rs:81:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-block-builder/tests/n28_first_economic_block.rs:84:    assert_eq!(builder.engine.state.balance_of(&a1), 800);
crates/amun-block-builder/tests/n28_first_economic_block.rs:85:    assert_eq!(builder.engine.state.balance_of(&a3), 350);
crates/amun-block-builder/tests/n28_first_economic_block.rs:89:fn n28_failed_transaction_in_block() {
crates/amun-block-builder/tests/n28_first_economic_block.rs:8:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n28_first_economic_block.rs:90:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n28_first_economic_block.rs:94:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n28_first_economic_block.rs:96:    builder.engine.state.create_account(alice, 100);
crates/amun-block-builder/tests/n32_certified_block.rs:100:    let block1 = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-block-builder/tests/n32_certified_block.rs:123:    let block2 = builder.build_block(
crates/amun-block-builder/tests/n32_certified_block.rs:146:    assert_eq!(builder.engine.state.balance_of(&alice), 700);
crates/amun-block-builder/tests/n32_certified_block.rs:14:impl CertifiedBlock {
crates/amun-block-builder/tests/n32_certified_block.rs:150:fn n32_certified_block_preserves_state_root() {
crates/amun-block-builder/tests/n32_certified_block.rs:151:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n32_certified_block.rs:155:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n32_certified_block.rs:157:    builder.engine.state.create_account(alice, 1000);
crates/amun-block-builder/tests/n32_certified_block.rs:158:    let state_root_before = builder.engine.state.state_root();
crates/amun-block-builder/tests/n32_certified_block.rs:15:    pub fn new(block: Block, qc: QuorumCertificate) -> Self {
crates/amun-block-builder/tests/n32_certified_block.rs:173:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-block-builder/tests/n32_certified_block.rs:188:        builder
crates/amun-block-builder/tests/n32_certified_block.rs:191:            .state_root_with_ledger(&builder.engine.economic)
crates/amun-block-builder/tests/n32_certified_block.rs:1:use amun_block_builder::{Block, BlockBuilder};
crates/amun-block-builder/tests/n32_certified_block.rs:24:fn n32_certified_block_created() {
crates/amun-block-builder/tests/n32_certified_block.rs:25:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n32_certified_block.rs:31:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n32_certified_block.rs:34:    builder.engine.state.create_account(alice, 1000);
crates/amun-block-builder/tests/n32_certified_block.rs:52:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 123456);
crates/amun-block-builder/tests/n32_certified_block.rs:72:    assert_eq!(builder.engine.state.balance_of(&alice), 700);
crates/amun-block-builder/tests/n32_certified_block.rs:73:    assert_eq!(builder.engine.state.balance_of(&bob), 300);
crates/amun-block-builder/tests/n32_certified_block.rs:77:fn n32_multiple_certified_blocks() {
crates/amun-block-builder/tests/n32_certified_block.rs:78:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n32_certified_block.rs:82:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n32_certified_block.rs:84:    builder.engine.state.create_account(alice, 1000);
crates/amun-block-builder/tests/n32_certified_block.rs:9:pub struct CertifiedBlock {
crates/amun-block-store/src/lib.rs:107:    fn n44_height_lookup() {
crates/amun-block-store/src/lib.rs:124:    fn n44_missing_height() {
crates/amun-block-store/src/lib.rs:138:    fn n44_empty_store() {
crates/amun-block-store/src/lib.rs:75:    fn n44_append_and_load() {
crates/amun-block-store/src/lib.rs:91:    fn n44_multiple_blocks() {
crates/amun-block/src/tests.rs:85:    assert_eq!(blk.compute_id(), blk.compute_id());
crates/amun-bls/src/tests.rs:18:        assert!(verify(msg, &sig, &kp.public).expect("test invariant"));
crates/amun-bls/src/tests.rs:25:        assert!(!verify(msg, &zero_sig, &kp.public).expect("test invariant"));
crates/amun-bytecode/src/lib.rs:27:        assert!(p1.verify());
crates/amun-bytecode/src/opcodes.rs:3:/// Constitutional bytecode opcodes as defined in N48.5-E Section 3.2.
crates/amun-byzantine-tests/tests/attack_suite.rs:276:    assert!(proof.verify_integrity());
crates/amun-byzantine-tests/tests/attack_suite.rs:279:    assert!(!tampered.verify_integrity());
crates/amun-byzantine-tests/tests/attack_suite.rs:283:fn byz_010_proof_replay_attack_blocked() {
crates/amun-byzantine-tests/tests/attack_suite.rs:56:    let replay = ReplayVerifier::replay(&proof, &program, &mut fresh_reg, &[]);
crates/amun-byzantine-tests/tests/attack_suite.rs:58:        replay,
crates/amun-byzantine-tests/tests/attack_suite.rs:59:        amun_replay_verifier::replay_verifier::ReplayResult::Match { .. }
crates/amun-byzantine-tests/tests/attack_suite.rs:9:use amun_replay_verifier::replay_verifier::ReplayVerifier;
crates/amun-canonical-collections/src/lib.rs:120:impl<K: Ord + CanonicalEncode, V: CanonicalEncode> ReplaySafe for CanonicalMap<K, V> {
crates/amun-canonical-collections/src/lib.rs:160:impl<T: CanonicalEncode> ReplaySafe for CanonicalDeque<T> {
crates/amun-canonical-collections/src/lib.rs:179:    #[test] fn test_replay_root_deterministic() { let mut a = CanonicalSet::new(); let mut b = CanonicalSet::new(); a.insert(1u64).unwrap(); a.insert(2u64).unwrap(); b.insert(2u64).unwrap(); b.insert(1u64).unwrap(); assert_eq!(a.canonical_root(), b.canonical_root()); }
crates/amun-canonical-collections/src/lib.rs:18:pub trait ReplaySafe: DeterministicCollection + CanonicalEncode {
crates/amun-canonical-collections/src/lib.rs:1://! Canonical Collections — deterministic, replay-safe container types.
crates/amun-canonical-collections/src/lib.rs:21:    fn is_replay_stable(&self) -> bool { true }
crates/amun-canonical-collections/src/lib.rs:72:impl<T: Ord + CanonicalEncode> ReplaySafe for CanonicalSet<T> {
crates/amun-canonical-collections/src/lib.rs:8://!   - REPLAY-SAFE: canonical_root() produces verifiable commitments
crates/amun-certificate-network/src/distribution.rs:153:    fn n9a_certificate_request_response() {
crates/amun-certificate-network/src/distribution.rs:168:    fn n9b_inclusion_proof_request_response() {
crates/amun-certificate-network/src/distribution.rs:174:            InclusionProofMessage::InclusionProofResponse { proof } => assert!(proof.verify()),
crates/amun-certificate-network/src/distribution.rs:180:    fn n9c_light_client_bundle_creation() {
crates/amun-certificate-network/src/distribution.rs:18:    CertificateResponse { certificate: ReplayCertificate },
crates/amun-certificate-network/src/distribution.rs:190:    fn n9c_bundle_verification() {
crates/amun-certificate-network/src/distribution.rs:192:        assert!(bundle.verify().is_ok());
crates/amun-certificate-network/src/distribution.rs:196:    fn n9c_tampered_bundle_fails() {
crates/amun-certificate-network/src/distribution.rs:199:        assert!(bundle.verify().is_err());
crates/amun-certificate-network/src/distribution.rs:203:    fn n9d_bundle_response() {
crates/amun-certificate-network/src/distribution.rs:211:                assert!(received.verify().is_ok());
crates/amun-certificate-network/src/distribution.rs:219:    fn n9d_bundle_not_found() {
crates/amun-certificate-network/src/distribution.rs:224:            ProofBundleMessage::BundleNotFound { reason } => assert!(!reason.is_empty()),
crates/amun-certificate-network/src/distribution.rs:230:    fn n9e_bundle_builder() {
crates/amun-certificate-network/src/distribution.rs:250:        assert!(bundle.verify().is_ok());
crates/amun-certificate-network/src/distribution.rs:255:    fn n9_serialize_certificate_message() {
crates/amun-certificate-network/src/distribution.rs:277:    fn n9_serialize_bundle() {
crates/amun-certificate-network/src/distribution.rs:287:        assert!(deserialized.verify().is_ok());
crates/amun-certificate-network/src/distribution.rs:33:// N9B: Inclusion Proof Request/Response Protocol
crates/amun-certificate-network/src/distribution.rs:43:// N9C: Light Client Proof Bundle
crates/amun-certificate-network/src/distribution.rs:49:    pub certificate: ReplayCertificate,
crates/amun-certificate-network/src/distribution.rs:56:        certificate: ReplayCertificate,
crates/amun-certificate-network/src/distribution.rs:7:    CertificateInclusionProof, ConstitutionalStateRuntime, ReplayCertificate,
crates/amun-certificate-network/src/distribution.rs:96:// N9E: Bundle Builder
crates/amun-certificate-network/src/gossip.rs:117:    pub fn get_certificate(&self, hash: &[u8; 32]) -> Option<&ReplayCertificate> {
crates/amun-certificate-network/src/gossip.rs:163:        ReplayCertificate,
crates/amun-certificate-network/src/gossip.rs:191:    fn n10a_certificate_announcement() {
crates/amun-certificate-network/src/gossip.rs:210:    fn n10b_inventory_exchange() {
crates/amun-certificate-network/src/gossip.rs:225:    fn n10c_certificate_sync() {
crates/amun-certificate-network/src/gossip.rs:240:    fn n10d_proof_sync() {
crates/amun-certificate-network/src/gossip.rs:246:            ProofSync::ProofResponse { proof: p } => assert!(p.verify()),
crates/amun-certificate-network/src/gossip.rs:252:    fn n10e_bundle_gossip() {
crates/amun-certificate-network/src/gossip.rs:258:            BundleGossip::BundleResponse { bundle: b } => assert!(b.verify().is_ok()),
crates/amun-certificate-network/src/gossip.rs:264:    fn n10f_cache_store_and_retrieve() {
crates/amun-certificate-network/src/gossip.rs:280:    fn n10f_cache_missing_detection() {
crates/amun-certificate-network/src/gossip.rs:293:    fn n10f_cache_announcement_tracking() {
crates/amun-certificate-network/src/gossip.rs:2:use amun_constitutional_state::{CertificateInclusionProof, ReplayCertificate};
crates/amun-certificate-network/src/gossip.rs:302:    fn n10f_cache_known_hashes() {
crates/amun-certificate-network/src/gossip.rs:312:    fn n10_full_gossip_flow() {
crates/amun-certificate-network/src/gossip.rs:348:                assert!(b.verify().is_ok());
crates/amun-certificate-network/src/gossip.rs:360:    fn n10_serialize_announcement() {
crates/amun-certificate-network/src/gossip.rs:38:        certificates: Vec<ReplayCertificate>,
crates/amun-certificate-network/src/gossip.rs:43:// N10D: Inclusion Proof Distribution
crates/amun-certificate-network/src/gossip.rs:77:    certificates: BTreeMap<[u8; 32], ReplayCertificate>,
crates/amun-certificate-network/src/gossip.rs:88:    pub fn store_certificate(&mut self, cert: ReplayCertificate) {
crates/amun-chain-checkpoint/src/bootstrap.rs:110:    fn n14_rejects_empty_bundles() {
crates/amun-chain-checkpoint/src/bootstrap.rs:117:    fn n14_trusted_root_stored() {
crates/amun-chain-checkpoint/src/bootstrap.rs:87:    fn n14_accepts_valid_bundles() {
crates/amun-chain-checkpoint/src/bootstrap.rs:99:    fn n14_rejects_wrong_root() {
crates/amun-chain-checkpoint/src/chain.rs:101:    fn n13_two_checkpoint_chain() {
crates/amun-chain-checkpoint/src/chain.rs:104:        assert!(RecursiveCheckpointProof::from_checkpoints(&[c1, c2])
crates/amun-chain-checkpoint/src/chain.rs:110:    fn n13_discontinuous_chain_fails() {
crates/amun-chain-checkpoint/src/chain.rs:113:        assert!(RecursiveCheckpointProof::from_checkpoints(&[c1, c2])
crates/amun-chain-checkpoint/src/chain.rs:119:    fn n13_wrong_root_fails() {
crates/amun-chain-checkpoint/src/chain.rs:123:        assert!(proof.verify().is_err());
crates/amun-chain-checkpoint/src/chain.rs:92:    fn n13_single_checkpoint_chain() {
crates/amun-chain-checkpoint/src/inclusion.rs:146:    fn n12b_single_checkpoint_merkle_root() {
crates/amun-chain-checkpoint/src/inclusion.rs:151:    fn n12b_multiple_checkpoints_different_root() {
crates/amun-chain-checkpoint/src/inclusion.rs:160:    fn n12b_inclusion_proof_valid() {
crates/amun-chain-checkpoint/src/inclusion.rs:167:        assert!(proof.verify());
crates/amun-chain-checkpoint/src/inclusion.rs:170:    fn n12b_inclusion_proof_wrong_hash_fails() {
crates/amun-chain-checkpoint/src/inclusion.rs:176:        assert!(!proof.verify());
crates/amun-chain-checkpoint/src/inclusion.rs:179:    fn n12b_root_mismatch_fails() {
crates/amun-chain-checkpoint/src/inclusion.rs:184:        assert!(!proof.verify());
crates/amun-chain-checkpoint/src/inclusion.rs:187:    fn n12c_bundle_valid() {
crates/amun-chain-checkpoint/src/inclusion.rs:191:        assert!(CheckpointBundle::new(c, proof).verify().is_ok());
crates/amun-chain-checkpoint/src/inclusion.rs:194:    fn n12d_light_verify_sequence() {
crates/amun-chain-checkpoint/src/inclusion.rs:202:        assert!(verify_checkpoint_sequence(&bundles, &root).is_ok());
crates/amun-chain-checkpoint/src/inclusion.rs:205:    fn n12d_light_verify_wrong_root() {
crates/amun-chain-checkpoint/src/inclusion.rs:209:        assert!(verify_checkpoint_sequence(&[CheckpointBundle::new(c1, p1)], &[0x11; 32]).is_err());
crates/amun-chain-checkpoint/src/lib.rs:124:            final_replay_certificate_root: last_bundle.block_header.replay_certificate_root.clone(),
crates/amun-chain-checkpoint/src/lib.rs:136:    /// AmunChain proof layers (ReplayCertificate uses `AMUN_REPLAY_CERTIFICATE_V1`,
crates/amun-chain-checkpoint/src/lib.rs:145:        bytes.extend_from_slice(self.final_replay_certificate_root.as_bytes());
crates/amun-chain-checkpoint/src/lib.rs:263:    fn n12a_checkpoint_creation() {
crates/amun-chain-checkpoint/src/lib.rs:274:        assert!(cert.verify().is_ok());
crates/amun-chain-checkpoint/src/lib.rs:279:    fn n12a_checkpoint_single_block() {
crates/amun-chain-checkpoint/src/lib.rs:284:        assert!(cert.verify().is_ok());
crates/amun-chain-checkpoint/src/lib.rs:289:    fn n12a_checkpoint_empty_bundles_fails() {
crates/amun-chain-checkpoint/src/lib.rs:296:    fn n12a_checkpoint_discontinuity_fails() {
crates/amun-chain-checkpoint/src/lib.rs:306:    fn n12a_checkpoint_tampered_fails() {
crates/amun-chain-checkpoint/src/lib.rs:315:        assert!(cert.verify().is_err());
crates/amun-chain-checkpoint/src/lib.rs:320:    fn n12a_checkpoint_store() {
crates/amun-chain-checkpoint/src/lib.rs:335:    fn n12a_checkpoint_multiple_in_store() {
crates/amun-chain-checkpoint/src/lib.rs:354:    fn n12a_checkpoint_hash_deterministic() {
crates/amun-chain-checkpoint/src/lib.rs:365:    fn n12a_checkpoint_hash_different_for_different_range() {
crates/amun-chain-checkpoint/src/lib.rs:45:/// It commits to the final state root, evidence root, and replay certificate root
crates/amun-chain-checkpoint/src/lib.rs:47:/// of the AmunChain proof layers (ReplayCertificate, CertificateMerkleRoot, etc.).
crates/amun-chain-checkpoint/src/lib.rs:60:    /// Replay certificate merkle root at end_height.
crates/amun-chain-checkpoint/src/lib.rs:61:    pub final_replay_certificate_root: String,
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:100:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:108:        let block = ConstitutionalBlock::new(
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:121:        bundles.push(LightClientProofBundle::new(block, cert, proof));
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:127:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:12:fn n15_node_b_full_bootstrap_from_node_a_checkpoints() {
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:133:    let mut rt_check = ConstitutionalStateRuntime::new();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:135:        rt_check.apply_transition(&[height as u8; 32], &[0xBB; 32]);
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:138:    let mut rt_verify = ConstitutionalStateRuntime::new();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:13:    let mut rt_a = ConstitutionalStateRuntime::new();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:140:        rt_verify.apply_transition(&[height as u8; 32], &[0xBB; 32]);
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:142:    assert_eq!(rt_check.state_root(), rt_verify.state_root());
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:14:    let mut bundles_a: Vec<LightClientProofBundle> = Vec::new();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:18:        rt_a.apply_transition(&[height as u8; 32], &[0xAA; 32]);
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:1:use amun_certificate_network::distribution::LightClientProofBundle;
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:21:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:23:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:31:        let block = ConstitutionalBlock::new(
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:44:        bundles_a.push(LightClientProofBundle::new(block, cert, proof));
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:4:    chain::RecursiveCheckpointProof,
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:55:    let proof1 = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:56:    let proof2 = prove_checkpoint_inclusion(&checkpoints, &cp2.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:5:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:69:    let mut rt_b = ConstitutionalStateRuntime::new();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:71:        rt_b.apply_transition(&[height as u8; 32], &[0xAA; 32]);
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:76:    let recursive = RecursiveCheckpointProof::from_checkpoints(&checkpoints).unwrap();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:77:    assert!(recursive.verify().is_ok());
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:89:fn n15_bootstrap_preserves_chain_continuity() {
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:8:use amun_constitutional_block::ConstitutionalBlock;
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:90:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:91:    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:95:        rt.apply_transition(&[height as u8; 32], &[0xBB; 32]);
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:98:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:9:use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:102:fn n16_byzantine_source_mixed_valid_invalid() {
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:103:    let (_, bundle1, root) = build_checkpoint_bundle(0, 2);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:105:    let (_, mut bundle2, _) = build_checkpoint_bundle(3, 5);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:10:fn build_checkpoint_bundle(
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:113:fn n16_empty_bundle_list_rejected() {
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:120:fn n16_duplicate_checkpoint_accepted() {
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:121:    let (cp, bundle, root) = build_checkpoint_bundle(0, 4);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:124:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:14:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:15:    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:19:        rt.apply_transition(&[height as u8; 32], &[0xCC; 32]);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:1:use amun_certificate_network::distribution::LightClientProofBundle;
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:22:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:24:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:32:        let block = ConstitutionalBlock::new(
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:45:        bundles.push(LightClientProofBundle::new(block, cert, proof));
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:51:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:57:fn build_bundle(start: u64, end: u64) -> (CheckpointBundle, [u8; 32]) {
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:58:    let (_, bundle, root) = build_checkpoint_bundle(start, end);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:63:fn n16_forged_checkpoint_rejected() {
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:64:    let (cp1, _, root) = build_checkpoint_bundle(0, 2);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:65:    let (bundle2, _) = build_bundle(3, 5);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:68:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:76:fn n16_tampered_bundle_rejected() {
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:77:    let (_, mut bundle, root) = build_checkpoint_bundle(0, 4);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:7:use amun_constitutional_block::ConstitutionalBlock;
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:85:fn n16_chain_gap_detected() {
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:86:    let (_, bundle1, root) = build_checkpoint_bundle(0, 2);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:87:    let (bundle2, _) = build_bundle(4, 6);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:8:use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:94:fn n16_wrong_trusted_root_rejected() {
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:95:    let (bundle, _) = build_bundle(0, 4);
crates/amun-chain-position/src/lib.rs:15:    pub replay_root: [u8; 32],
crates/amun-chain-position/src/lib.rs:24:        replay_root: [u8; 32],
crates/amun-chain-position/src/lib.rs:31:        h.update(&replay_root);
crates/amun-chain-position/src/lib.rs:38:            replay_root,
crates/amun-chain-position/src/lib.rs:48:        h.update(&self.replay_root);
crates/amun-chain-store/examples/snapshot_builder_test.rs:45:    println!("N99.1 Snapshot Builder Test: PASS");
crates/amun-chain-store/src/record.rs:15:    /// N129.3: Evidence root chaining all constitutional proofs
crates/amun-chain-store/src/record.rs:43:    fn n70_record_roundtrip() {
crates/amun-chain-store/src/record.rs:64:        assert_eq!(decoded.commitment_root, record.commitment_root);
crates/amun-chain-store/src/store.rs:164:    fn n70_append_and_load() {
crates/amun-chain-store/src/store.rs:174:    fn n70_recover_after_restart() {
crates/amun-chain-store/src/store.rs:188:    fn n70_empty_store() {
crates/amun-chain-store/src/store.rs:196:    fn n70_skip_duplicate_heights() {
crates/amun-chain-store/src/store.rs:206:    fn n70_gap_heights_recovery() {
crates/amun-chain-store/src/sync.rs:191:    fn n72_sync_download_range() {
crates/amun-chain-store/src/sync.rs:198:    fn n72_sync_catch_up() {
crates/amun-chain-store/src/sync.rs:208:    fn n72_sync_partial_catch_up() {
crates/amun-chain-store/src/sync.rs:221:    fn n72_sync_already_synced() {
crates/amun-chain-store/src/sync.rs:233:    fn n72_sync_empty_peer() {
crates/amun-chain-store/tests/n120_2_record_roundtrip.rs:16:        commitment_root: [0u8; 32],
crates/amun-chain-store/tests/n120_2_record_roundtrip.rs:4:fn n120_2_record_roundtrip_preserves_slashing_root() {
crates/amun-cli/src/main.rs:35:fn prompt_password(prompt: &str) -> String {
crates/amun-codec/src/versioned.rs:79:                ConstitutionalFault::ReplayViolation,
crates/amun-compatibility/src/matrix.rs:16:        if !self.replay_safe && !self.snapshot_safe {
crates/amun-compatibility/src/matrix.rs:18:        } else if !self.replay_safe && self.snapshot_safe {
crates/amun-compatibility/src/matrix.rs:20:        } else if self.replay_safe && self.proof_safe && self.governance_safe {
crates/amun-compatibility/src/matrix.rs:22:        } else if self.replay_safe && self.proof_safe {
crates/amun-compatibility/src/matrix.rs:23:            CompatibilityClass::ReplayCompatible
crates/amun-compatibility/src/matrix.rs:24:        } else if self.replay_safe {
crates/amun-compatibility/src/matrix.rs:51:            replay_safe: same_empty_root && same_max_depth && same_proof,
crates/amun-compatibility/src/matrix.rs:7:    pub replay_safe: bool,
crates/amun-compatibility/src/migration.rs:18:    /// Full migration - state must be replayed
crates/amun-compatibility/src/migration.rs:6:    pub requires_replay: bool,
crates/amun-consensus-integration/src/consensus_integrator.rs:133:        assert_eq!(block.transitions.len(), 1);
crates/amun-consensus-integration/src/consensus_integrator.rs:134:        assert!(block.verify_all_proofs());
crates/amun-consensus-integration/src/consensus_integrator.rs:161:        assert!(cert.verify());
crates/amun-consensus-integration/src/consensus_integrator.rs:238:        assert_eq!(block.state_root, registry.compute_state_root());
crates/amun-consensus-integration/src/consensus_integrator.rs:239:        assert_eq!(block.proof_root, block.compute_proof_root());
crates/amun-consensus-integration/src/consensus_integrator.rs:240:        assert!(!block.transitions.is_empty());
crates/amun-consensus-network/src/certificate_evidence_validation.rs:109:    fn n111_6_certificate_rejected_when_evidence_missing() {
crates/amun-consensus-network/src/certificate_evidence_validation.rs:126:    fn n111_6_certificate_accepted_after_evidence_sync() {
crates/amun-consensus-network/src/certificate_evidence_validation.rs:140:        assert_eq!(result, EvidenceValidationResult::AllPresent);
crates/amun-consensus-network/src/certificate_evidence_validation.rs:145:    fn n111_6_partial_evidence_still_rejected() {
crates/amun-consensus-network/src/certificate_evidence_validation.rs:165:    fn n111_6_build_missing_evidence_request() {
crates/amun-consensus-network/src/certificate_evidence_validation.rs:173:    fn n111_6_process_evidence_response() {
crates/amun-consensus-network/src/certificate_evidence_validation.rs:2:// N111.6 — Certificate Evidence Validation
crates/amun-consensus-network/src/certificate_evidence_validation.rs:52:/// N111.6: Build a MissingEvidenceRequest for the given evidence IDs.
crates/amun-consensus-network/src/certificate_evidence_validation.rs:63:/// N111.6: Process a MissingEvidenceResponse by extracting the evidence
crates/amun-consensus-network/src/certificate_gossip.rs:104:    fn n110_3_duplicate_deduplicated() {
crates/amun-consensus-network/src/certificate_gossip.rs:113:    fn n110_3_broadcast_tracking() {
crates/amun-consensus-network/src/certificate_gossip.rs:124:    fn n110_3_pending_then_included() {
crates/amun-consensus-network/src/certificate_gossip.rs:135:    fn n110_3_tampered_rejected() {
crates/amun-consensus-network/src/certificate_gossip.rs:96:    fn n110_3_receive_valid_certificate() {
crates/amun-consensus-network/src/engine.rs:357:                // N109.9: Vote binding enforcement — verify ExecutionCommitment
crates/amun-consensus-network/src/engine.rs:587:    fn n68_round_propose_vote_qc_finalize() {
crates/amun-consensus-network/src/engine.rs:614:        assert!(engine.is_finalized(1));
crates/amun-consensus-network/src/engine.rs:618:    fn n68_insufficient_quorum_no_qc() {
crates/amun-consensus-network/src/engine.rs:647:    fn n68_duplicate_vote_rejected() {
crates/amun-consensus-network/src/engine.rs:670:    fn n68_byzantine_wrong_height_rejected() {
crates/amun-consensus-network/src/engine.rs:688:    fn n68_multi_round_consensus() {
crates/amun-consensus-network/src/engine.rs:718:            assert!(engine.is_finalized(height));
crates/amun-consensus-network/src/engine.rs:727:    fn n105_signature_required_when_registry_populated() {
crates/amun-consensus-network/src/engine.rs:785:    fn n130_vote_before_proposal_is_recovered() {
crates/amun-consensus-network/src/evidence_gossip.rs:117:    fn n111_2_receive_new_announcement() {
crates/amun-consensus-network/src/evidence_gossip.rs:131:    fn n111_2_duplicate_ignored() {
crates/amun-consensus-network/src/evidence_gossip.rs:146:    fn n111_2_broadcast_tracking() {
crates/amun-consensus-network/src/evidence_gossip.rs:155:    fn n111_2_has_evidence() {
crates/amun-consensus-network/src/evidence_gossip.rs:170:    fn n111_4_valid_announcement_passes() {
crates/amun-consensus-network/src/evidence_gossip.rs:178:        assert!(EvidenceGossip::verify_announcement(&ann, 50).is_ok());
crates/amun-consensus-network/src/evidence_gossip.rs:182:    fn n111_4_future_height_rejected() {
crates/amun-consensus-network/src/evidence_gossip.rs:195:    fn n111_4_zero_evidence_id_rejected() {
crates/amun-consensus-network/src/evidence_gossip.rs:203:        assert!(EvidenceGossip::verify_announcement(&ann, 50).is_err());
crates/amun-consensus-network/src/evidence_gossip.rs:207:    fn n111_4_zero_validator_id_rejected() {
crates/amun-consensus-network/src/evidence_gossip.rs:215:        assert!(EvidenceGossip::verify_announcement(&ann, 50).is_err());
crates/amun-consensus-network/src/evidence_gossip.rs:2:// N111.2 — Evidence Gossip
crates/amun-consensus-network/src/evidence_push.rs:132:    fn n112_1_select_unpushed_evidence() {
crates/amun-consensus-network/src/evidence_push.rs:146:    fn n112_1_no_duplicate_push() {
crates/amun-consensus-network/src/evidence_push.rs:159:    fn n112_1_respects_batch_size() {
crates/amun-consensus-network/src/evidence_push.rs:173:    fn n112_1_mark_pushed_individual() {
crates/amun-consensus-network/src/evidence_push.rs:192:    fn n112_1_reset_clears_tracking() {
crates/amun-consensus-network/src/evidence_push.rs:1:// N112.1 — Push-Based Evidence Propagation
crates/amun-consensus-network/src/evidence_push.rs:32:/// N112.1: Push engine that works alongside EvidenceGossip.
crates/amun-consensus-network/src/evidence_push.rs:4:// and full EvidenceRecords to known peers.  This reduces latency in
crates/amun-consensus-network/src/evidence_push_processor.rs:103:    fn n112_3_duplicate_records_counted_but_not_reimported() {
crates/amun-consensus-network/src/evidence_push_processor.rs:132:    fn n112_3_push_updates_gossip_tracking() {
crates/amun-consensus-network/src/evidence_push_processor.rs:1:// N112.3 — Automatic Peer Evidence Propagation
crates/amun-consensus-network/src/evidence_push_processor.rs:20:/// N112.3: Process an incoming EvidencePushMessage.
crates/amun-consensus-network/src/evidence_push_processor.rs:81:    fn n112_3_process_push_imports_all_records() {
crates/amun-consensus-network/src/evidence_store.rs:259:    fn n109_10_store_and_query() {
crates/amun-consensus-network/src/evidence_store.rs:273:    fn n109_10_duplicate_evidence_is_deduplicated() {
crates/amun-consensus-network/src/evidence_store.rs:288:    fn n109_10_evidence_survives_restart() {
crates/amun-consensus-network/src/evidence_store.rs:2:// N109.10/N109.11 — Evidence Store (HashMap-based)
crates/amun-consensus-network/src/evidence_store.rs:311:    fn n109_10_status_lifecycle() {
crates/amun-consensus-network/src/evidence_store.rs:316:        assert_eq!(s.get_by_id(&id).unwrap().status, EvidenceStatus::Pending);
crates/amun-consensus-network/src/evidence_store.rs:318:        assert_eq!(s.get_by_id(&id).unwrap().status, EvidenceStatus::Confirmed);
crates/amun-consensus-network/src/evidence_store.rs:320:        assert_eq!(s.get_by_id(&id).unwrap().status, EvidenceStatus::Slashed);
crates/amun-consensus-network/src/evidence_store.rs:324:    fn n109_11_get_all_for_validator() {
crates/amun-consensus-network/src/evidence_store.rs:4:// N109.11: Upgraded from Vec to HashMap<evidence_id, EvidenceRecord> for O(1) lookup.
crates/amun-consensus-network/src/execution_commitment.rs:102:    /// N109.8: Verify the commitment's signature.
crates/amun-consensus-network/src/execution_commitment.rs:13://   - Replay of commitments across heights
crates/amun-consensus-network/src/execution_commitment.rs:142:    fn n109_8_compute_execution_root_is_deterministic() {
crates/amun-consensus-network/src/execution_commitment.rs:154:    fn n109_8_different_validator_different_root() {
crates/amun-consensus-network/src/execution_commitment.rs:165:    fn n109_8_different_height_different_root() {
crates/amun-consensus-network/src/execution_commitment.rs:176:    fn n109_8_sign_and_verify() {
crates/amun-consensus-network/src/execution_commitment.rs:184:        assert!(commit.verify().is_ok(), "signature must verify");
crates/amun-consensus-network/src/execution_commitment.rs:188:    fn n109_8_tampered_state_root_rejected() {
crates/amun-consensus-network/src/execution_commitment.rs:18:// This is the foundation for N110 Slashing and N111 Evidence.
crates/amun-consensus-network/src/execution_commitment.rs:206:    fn n109_8_replayed_commitment_rejected() {
crates/amun-consensus-network/src/execution_commitment.rs:215:        // Try to replay the SAME commitment at height 2 (should fail)
crates/amun-consensus-network/src/execution_commitment.rs:220:            "replay across heights must produce different execution_root"
crates/amun-consensus-network/src/execution_commitment.rs:24:/// N109.8: A signed statement that a specific validator executed a specific
crates/amun-consensus-network/src/execution_commitment.rs:28:/// preventing any form of splitting or replay.
crates/amun-consensus-network/src/execution_commitment.rs:75:    /// N109.8: Create a new ExecutionCommitment and compute its execution_root.
crates/amun-consensus-network/src/execution_commitment.rs:95:    /// N109.8: Sign the commitment with an Ed25519 signing key.
crates/amun-consensus-network/src/finality_gate.rs:12:/// N118.1: Check if a certificate can be executed at the given finalized height.
crates/amun-consensus-network/src/finality_gate.rs:17:/// N118.1: Execute a certificate only if it has been finalized.
crates/amun-consensus-network/src/finality_gate.rs:1:// N118.1 — Finality Gate for Slashing Certificates
crates/amun-consensus-network/src/finality_gate.rs:29:            "N118.1: certificate not finalized (cert_height={}, finalized={})",
crates/amun-consensus-network/src/finality_gate.rs:67:    fn n118_1_finalized_certificate_accepted() {
crates/amun-consensus-network/src/finality_gate.rs:69:        assert!(is_certificate_finalized(&cert, 100));
crates/amun-consensus-network/src/finality_gate.rs:70:        assert!(is_certificate_finalized(&cert, 150));
crates/amun-consensus-network/src/finality_gate.rs:73:        assert_eq!(result.unwrap(), "executed");
crates/amun-consensus-network/src/finality_gate.rs:77:    fn n118_1_unfinalized_certificate_rejected() {
crates/amun-consensus-network/src/finality_gate.rs:79:        assert!(!is_certificate_finalized(&cert, 99));
crates/amun-consensus-network/src/finality_gate.rs:83:        assert!(result.unwrap_err().contains("not finalized"));
crates/amun-consensus-network/src/finality_gate.rs:87:    fn n118_1_exact_height_boundary() {
crates/amun-consensus-network/src/integrated_slashing.rs:168:    /// N109.12 GATEKEEPER: Evidence triggers the full slashing pipeline
crates/amun-consensus-network/src/integrated_slashing.rs:170:    fn n109_12_evidence_triggers_slashing_pipeline() {
crates/amun-consensus-network/src/integrated_slashing.rs:255:    fn n109_12_duplicate_evidence_ignored() {
crates/amun-consensus-network/src/integrated_slashing.rs:280:    /// N109.12: EvidenceStore records are accessible after pipeline processing
crates/amun-consensus-network/src/integrated_slashing.rs:282:    fn n109_12_evidence_store_accessible() {
crates/amun-consensus-network/src/integrated_slashing.rs:302:    fn n109_12_different_validators_isolated() {
crates/amun-consensus-network/src/integrated_slashing.rs:323:    fn n109_12_custom_thresholds_pipeline() {
crates/amun-consensus-network/src/integrated_slashing.rs:344:    fn n109_12_mark_slashed_updates_status() {
crates/amun-consensus-network/src/integrated_slashing.rs:355:        assert_eq!(updated.status, EvidenceStatus::Slashed);
crates/amun-consensus-network/src/integrated_slashing.rs:4:// Connects EvidenceStore → MisbehaviorRegistry(N109.11) → SlashingEngine → Stake Penalty
crates/amun-consensus-network/src/integrated_slashing.rs:6:// This is the bridge between N109 (Evidence + Misbehavior) and
crates/amun-consensus-network/src/messages.rs:17:    /// N109.8: Cryptographic execution commitment (optional for backward compatibility).
crates/amun-consensus-network/src/messages.rs:197:    fn n101_2_valid_equivocation_proof_accepted() {
crates/amun-consensus-network/src/messages.rs:210:        assert!(proof.verify_standalone().is_ok());
crates/amun-consensus-network/src/messages.rs:214:    fn n131_duplicate_validator_in_qc_rejected() {
crates/amun-consensus-network/src/messages.rs:231:        assert!(qc.verify_strict(&powers).is_err());
crates/amun-consensus-network/src/messages.rs:234:    fn n131_tampered_approval_power_rejected() {
crates/amun-consensus-network/src/messages.rs:257:        assert!(qc.verify_strict(&powers).is_err());
crates/amun-consensus-network/src/messages.rs:261:    fn n101_2_different_validators_rejected() {
crates/amun-consensus-network/src/messages.rs:270:        assert!(proof.verify_standalone().is_err());
crates/amun-consensus-network/src/messages.rs:274:    fn n101_2_different_heights_rejected() {
crates/amun-consensus-network/src/messages.rs:283:        assert!(proof.verify_standalone().is_err());
crates/amun-consensus-network/src/messages.rs:287:    fn n101_2_same_block_hash_rejected() {
crates/amun-consensus-network/src/messages.rs:296:        assert!(proof.verify_standalone().is_err());
crates/amun-consensus-network/src/messages.rs:300:    fn n101_2_missing_signature_rejected() {
crates/amun-consensus-network/src/messages.rs:311:        assert!(proof.verify_standalone().is_err());
crates/amun-consensus-network/src/messages.rs:315:    fn n101_2_different_rounds_rejected() {
crates/amun-consensus-network/src/messages.rs:324:        assert!(proof.verify_standalone().is_err());
crates/amun-consensus-network/src/messages.rs:328:    fn n68_qc_verifies_with_supermajority() {
crates/amun-consensus-network/src/messages.rs:343:        assert!(qc.verify());
crates/amun-consensus-network/src/messages.rs:347:    fn n68_qc_rejects_insufficient_quorum() {
crates/amun-consensus-network/src/messages.rs:362:        assert!(!qc.verify_quorum());
crates/amun-consensus-network/src/messages.rs:366:    fn n68_qc_rejects_inconsistent_votes() {
crates/amun-consensus-network/src/messages.rs:380:        assert!(!qc.verify_consistency());
crates/amun-consensus-network/src/messages.rs:384:    fn n68_roundtrip_vote_serialization() {
crates/amun-consensus-network/src/messages.rs:394:    fn n68_roundtrip_qc_serialization() {
crates/amun-consensus-network/src/messages.rs:406:        assert!(decoded.verify());
crates/amun-consensus-network/src/messages.rs:426:    fn n104_1_block_proposal_roundtrip() {
crates/amun-consensus-network/src/messages.rs:440:    fn n104_1_deterministic_hash() {
crates/amun-consensus-network/src/messages.rs:490:// N109 — Constitutional Block Propagation Types
crates/amun-consensus-network/src/messages.rs:496:pub struct N109BlockProposal {
crates/amun-consensus-network/src/messages.rs:524:// New code should use N109ConsensusVote which binds the vote to an
crates/amun-consensus-network/src/messages.rs:536:/// N109.8: A vote that carries a full execution commitment.
crates/amun-consensus-network/src/messages.rs:539:pub struct N109ConsensusVote {
crates/amun-consensus-network/src/messages.rs:561:    /// N109.8: Cryptographic execution commitment
crates/amun-consensus-network/src/misbehavior.rs:149:    fn n101_3_add_valid_proof() {
crates/amun-consensus-network/src/misbehavior.rs:168:    fn n101_3_reject_invalid_proof() {
crates/amun-consensus-network/src/misbehavior.rs:183:    fn n101_3_validator_history() {
crates/amun-consensus-network/src/misbehavior.rs:216:    fn n101_3_duplicate_proof_deduplicated() {
crates/amun-consensus-network/src/misbehavior.rs:234:    fn n101_3_offense_count() {
crates/amun-consensus-network/src/misbehavior.rs:266:    fn n101_3_persistence_roundtrip() {
crates/amun-consensus-network/src/misbehavior_registry.rs:221:    /// N109.11: Rebuild registry from EvidenceStore (e.g., after restart)
crates/amun-consensus-network/src/misbehavior_registry.rs:262:    fn n109_11_record_misbehavior() {
crates/amun-consensus-network/src/misbehavior_registry.rs:271:    fn n109_11_score_accumulates() {
crates/amun-consensus-network/src/misbehavior_registry.rs:279:    fn n109_11_different_evidence_types_weighted() {
crates/amun-consensus-network/src/misbehavior_registry.rs:293:    fn n109_11_warning_threshold_triggered() {
crates/amun-consensus-network/src/misbehavior_registry.rs:305:    fn n109_11_suspension_threshold_triggered() {
crates/amun-consensus-network/src/misbehavior_registry.rs:320:    fn n109_11_slashing_threshold_triggered() {
crates/amun-consensus-network/src/misbehavior_registry.rs:336:    fn n109_11_duplicate_evidence_not_counted_twice() {
crates/amun-consensus-network/src/misbehavior_registry.rs:340:        assert!(reg.record_misbehavior(&[1u8; 32], &ev_id, &EvidenceType::DoubleVote, 1));
crates/amun-consensus-network/src/misbehavior_registry.rs:343:        assert!(!reg.record_misbehavior(&[1u8; 32], &ev_id, &EvidenceType::DoubleVote, 2));
crates/amun-consensus-network/src/misbehavior_registry.rs:352:    fn n109_11_multiple_validators_isolated() {
crates/amun-consensus-network/src/misbehavior_registry.rs:362:    fn n109_11_registry_rebuilds_from_evidence_store() {
crates/amun-consensus-network/src/misbehavior_registry.rs:386:    fn n109_11_custom_thresholds() {
crates/amun-consensus-network/src/misbehavior_registry.rs:403:    fn n109_11_active_validator_returns_none_action() {
crates/amun-consensus-network/src/multi_signer_certificate.rs:121:    fn n116_single_approval_works() {
crates/amun-consensus-network/src/multi_signer_certificate.rs:132:    fn n116_duplicate_approval_rejected() {
crates/amun-consensus-network/src/multi_signer_certificate.rs:142:    fn n116_quorum_reached_with_enough_approvals() {
crates/amun-consensus-network/src/multi_signer_certificate.rs:158:    fn n116_verify_approvals_counts_correctly() {
crates/amun-consensus-network/src/multi_signer_certificate.rs:170:    fn n116_invalid_signature_detected() {
crates/amun-consensus-network/src/network_consensus.rs:125:    fn n68_network_consensus_single_round() {
crates/amun-consensus-network/src/re_executor.rs:19:            "N109.7 STATE_ROOT_MISMATCH: height={} proposed={} computed={}",
crates/amun-consensus-network/src/re_executor.rs:6:/// N109.7: Re-execute a block and verify the state root.
crates/amun-consensus-network/src/real_staking_adapter.rs:141:        assert!(result.is_some(), "N110.1b FAIL: slash should execute");
crates/amun-consensus-network/src/real_staking_adapter.rs:162:    fn n113_2b_unregistered_validator_rejected() {
crates/amun-consensus-network/src/real_staking_adapter.rs:179:    fn n113_2_real_executor_uses_identity_registry() {
crates/amun-consensus-network/src/real_staking_adapter.rs:96:    fn n110_1b_real_staking_slash_reduces_validator_stake() {
crates/amun-consensus-network/src/slashing.rs:108:    fn n109_13_mixed_offenses_accumulate() {
crates/amun-consensus-network/src/slashing.rs:126:    fn n109_13_all_slashable_offenses_flow_through_unified_api() {
crates/amun-consensus-network/src/slashing.rs:56:    fn n109_13_no_offenses_no_slash() {
crates/amun-consensus-network/src/slashing.rs:64:    fn n109_13_single_offense_below_threshold() {
crates/amun-consensus-network/src/slashing.rs:82:    fn n109_13_two_offenses_triggers_warning() {
crates/amun-consensus-network/src/slashing.rs:93:    fn n109_13_three_offenses_triggers_slash() {
crates/amun-consensus-network/src/slashing_certificate.rs:215:        // N114.2: If signed, verify the signature
crates/amun-consensus-network/src/slashing_certificate.rs:228:    fn n110_2_certificate_roundtrip() {
crates/amun-consensus-network/src/slashing_certificate.rs:253:        assert!(decoded.verify().is_ok());
crates/amun-consensus-network/src/slashing_certificate.rs:257:    fn n110_2_certificate_hash_is_deterministic() {
crates/amun-consensus-network/src/slashing_certificate.rs:284:        assert_eq!(cert1.compute_hash(), cert1.certificate_hash);
crates/amun-consensus-network/src/slashing_certificate.rs:285:        assert_eq!(cert2.compute_hash(), cert2.certificate_hash);
crates/amun-consensus-network/src/slashing_certificate.rs:289:    fn n110_2_different_validators_different_hash() {
crates/amun-consensus-network/src/slashing_certificate.rs:318:    fn n110_2_verify_rejects_tampered_amount() {
crates/amun-consensus-network/src/slashing_certificate.rs:332:        assert!(cert.verify().is_err());
crates/amun-consensus-network/src/slashing_fraud_proof.rs:113:    fn n121_3_valid_fraud_proof_verifies() {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:123:            "N121.3 FAIL: valid fraud proof must verify"
crates/amun-consensus-network/src/slashing_fraud_proof.rs:129:    fn n121_3_matching_roots_not_fraud() {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:143:    fn n121_3_tampered_history_rejected() {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:155:            "N121.3 FAIL: tampered slash history must invalidate proof"
crates/amun-consensus-network/src/slashing_fraud_proof.rs:160:    fn n121_3_tampered_proof_id_rejected() {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:176:    fn n121_3_roundtrip_serialization() {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:187:        assert!(decoded.verify().is_ok());
crates/amun-consensus-network/src/slashing_fraud_proof.rs:1:// N121.3 — Slashing Fraud Proof Engine
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:10:/// N121.4: A Merkle inclusion proof for an executed slash.
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:177:    fn n121_4_single_element_inclusion_proof() {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:182:            "N121.4 FAIL: single element proof must verify"
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:189:    fn n121_4_multi_element_inclusion_proof() {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:196:            "N121.4 FAIL: inclusion proof for index 2 must verify"
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:1:// N121.4 — Merkle Inclusion Proofs for Slashing Ledger
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:202:    fn n121_4_wrong_element_rejected() {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:216:    fn n121_4_out_of_bounds_index_rejected() {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:218:        assert!(build_inclusion_proof(&slashes, 5).is_err());
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:222:    fn n121_4_empty_ledger_rejected() {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:223:        assert!(build_inclusion_proof(&[], 0).is_err());
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:227:    fn n121_4_proof_matches_merkle_root() {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:240:                "N121.4 FAIL: proof must verify at index {}",
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:99:        return Err("N121.4: cannot build proof for empty ledger".into());
crates/amun-consensus-network/src/slashing_ledger.rs:100:    /// N119.5: Get all executed slashes for a validator.
crates/amun-consensus-network/src/slashing_ledger.rs:142:    fn n119_1_certificate_id_stable() {
crates/amun-consensus-network/src/slashing_ledger.rs:157:    fn n119_2_replay_rejected() {
crates/amun-consensus-network/src/slashing_ledger.rs:165:        assert!(ledger.is_executed(&id));
crates/amun-consensus-network/src/slashing_ledger.rs:167:        // Second execution: rejected (replay protection)
crates/amun-consensus-network/src/slashing_ledger.rs:169:        assert!(r2.is_err(), "N119.2 FAIL: replay must be rejected");
crates/amun-consensus-network/src/slashing_ledger.rs:170:        assert!(r2.unwrap_err().contains("already executed"));
crates/amun-consensus-network/src/slashing_ledger.rs:171:        assert_eq!(ledger.executed_count(), 1);
crates/amun-consensus-network/src/slashing_ledger.rs:175:    fn n119_3_ledger_records_execution() {
crates/amun-consensus-network/src/slashing_ledger.rs:180:        assert_eq!(ledger.executed_count(), 1);
crates/amun-consensus-network/src/slashing_ledger.rs:187:    fn n119_4_different_certificates_different_ids() {
crates/amun-consensus-network/src/slashing_ledger.rs:199:    fn n119_5_duplicate_execution_no_effect() {
crates/amun-consensus-network/src/slashing_ledger.rs:1:// N119 — Deterministic Slashing Ledger & Replay Protection
crates/amun-consensus-network/src/slashing_ledger.rs:218:    fn n119_5_audit_trail_by_validator() {
crates/amun-consensus-network/src/slashing_ledger.rs:28:/// N119.5: Record of an executed slash for auditability.
crates/amun-consensus-network/src/slashing_ledger.rs:40:/// N119.1: Persistent ledger preventing replay of slashing certificates.
crates/amun-consensus-network/src/slashing_ledger.rs:44:    /// N119.5: Audit trail of all executed slashes.
crates/amun-consensus-network/src/slashing_ledger.rs:56:    /// N119.1: Check if a certificate has already been executed.
crates/amun-consensus-network/src/slashing_ledger.rs:62:    /// Returns Err if the certificate was already executed (replay protection).
crates/amun-consensus-network/src/slashing_ledger.rs:69:        // N119.2: Replay protection
crates/amun-consensus-network/src/slashing_ledger.rs:72:                "N119: certificate already executed: {:02x?}",
crates/amun-consensus-network/src/slashing_ledger.rs:95:    /// N119.1: Get the number of executed slashes.
crates/amun-consensus-network/src/slashing_merkle.rs:109:    fn n120_1_root_changes_with_new_slash() {
crates/amun-consensus-network/src/slashing_merkle.rs:10:/// N120.1: Leaf hash for a single executed slash.
crates/amun-consensus-network/src/slashing_merkle.rs:121:    fn n120_1_larger_tree_is_deterministic() {
crates/amun-consensus-network/src/slashing_merkle.rs:132:    fn n120_1_order_affects_root() {
crates/amun-consensus-network/src/slashing_merkle.rs:22:/// N120.1: Compute the Merkle root from a slice of executed slashes.
crates/amun-consensus-network/src/slashing_merkle.rs:67:    fn n120_1_empty_ledger_gives_zero_root() {
crates/amun-consensus-network/src/slashing_merkle.rs:72:    fn n120_1_single_leaf_is_deterministic() {
crates/amun-consensus-network/src/slashing_merkle.rs:84:    fn n120_1_same_order_same_root() {
crates/amun-consensus-network/src/slashing_merkle.rs:97:    fn n120_1_different_slashes_different_root() {
crates/amun-consensus-network/src/slashing_state.rs:102:    fn n121_1_root_updates_after_execution() {
crates/amun-consensus-network/src/slashing_state.rs:117:        assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/src/slashing_state.rs:118:        assert_eq!(state.executed_count(), 1);
crates/amun-consensus-network/src/slashing_state.rs:122:    fn n121_1_multiple_executions_update_root() {
crates/amun-consensus-network/src/slashing_state.rs:140:        assert_eq!(state.executed_count(), 1);
crates/amun-consensus-network/src/slashing_state.rs:144:        assert_eq!(state.executed_count(), 2);
crates/amun-consensus-network/src/slashing_state.rs:150:        assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/src/slashing_state.rs:154:    fn n121_1_replay_protection_preserved() {
crates/amun-consensus-network/src/slashing_state.rs:163:        assert!(result.is_err(), "N121.1 FAIL: replay must be rejected");
crates/amun-consensus-network/src/slashing_state.rs:164:        assert!(result.unwrap_err().contains("already executed"));
crates/amun-consensus-network/src/slashing_state.rs:169:            "N121.1 FAIL: root must not change on replay"
crates/amun-consensus-network/src/slashing_state.rs:171:        assert_eq!(state.executed_count(), 1);
crates/amun-consensus-network/src/slashing_state.rs:94:    fn n121_1_initial_state_has_zero_root() {
crates/amun-consensus-network/src/slashing_state.rs:97:        assert_eq!(state.executed_count(), 0);
crates/amun-consensus-network/src/slashing_state.rs:98:        assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/src/staking_adapter.rs:198:    fn n110_1_slashing_reduces_real_validator_stake() {
crates/amun-consensus-network/src/staking_adapter.rs:238:            "N110.1 FAIL: should execute slash when threshold crossed"
crates/amun-consensus-network/src/staking_adapter.rs:268:    fn n110_1_validator_below_threshold_not_slashed() {
crates/amun-consensus-network/src/staking_adapter.rs:299:    fn n110_1_multiple_slashes_accumulate() {
crates/amun-consensus-network/src/staking_adapter.rs:365:    fn n110_1_different_validators_independent() {
crates/amun-consensus-network/src/staking_adapter.rs:399:    fn n115_unsigned_certificate_rejected_before_slash() {
crates/amun-consensus-network/src/staking_adapter.rs:433:    /// N115: Signed certificate executes slash successfully
crates/amun-consensus-network/src/staking_adapter.rs:435:    fn n115_signed_certificate_executes_slash() {
crates/amun-consensus-network/src/staking_adapter.rs:471:            "N115 FAIL: Slash must execute with signed certificate"
crates/amun-consensus-network/src/staking_adapter.rs:478:    fn n115_tampered_certificate_rejected_before_slash() {
crates/amun-consensus-network/src/staking_adapter.rs:517:    fn n118_2a_unfinalized_certificate_rejected() {
crates/amun-consensus-network/src/staking_adapter.rs:53:    /// N110.1: Check if a validator is slashable and execute the slash.
crates/amun-consensus-network/src/staking_adapter.rs:553:    fn n118_2b_finalized_certificate_executes() {
crates/amun-consensus-network/src/staking_adapter.rs:74:            return Err("N118.2: certificate not finalized".to_string());
crates/amun-consensus-network/src/validation.rs:4:impl N109BlockProposal {
crates/amun-consensus-network/src/validator_identity.rs:167:    fn n113_1_identity_binding_is_verifiable() {
crates/amun-consensus-network/src/validator_identity.rs:169:        assert!(id.verify_binding());
crates/amun-consensus-network/src/validator_identity.rs:173:    fn n113_1_tampered_binding_detected() {
crates/amun-consensus-network/src/validator_identity.rs:176:        assert!(!id.verify_binding());
crates/amun-consensus-network/src/validator_identity.rs:180:    fn n113_1_different_validator_different_binding() {
crates/amun-consensus-network/src/validator_identity.rs:187:    fn n113_1_different_key_different_binding() {
crates/amun-consensus-network/src/validator_identity.rs:194:    fn n113_1_registry_register_and_lookup() {
crates/amun-consensus-network/src/validator_identity.rs:204:    fn n113_1_duplicate_registration_rejected() {
crates/amun-consensus-network/src/validator_identity.rs:212:    fn n113_1_binding_must_verify_before_registration() {
crates/amun-consensus-network/src/validator_identity.rs:220:    fn n113_1_deactivation() {
crates/amun-consensus-network/src/validator_identity.rs:229:    fn n113_2_vote_identity_matches_staking_identity() {
crates/amun-consensus-network/src/validator_identity.rs:248:    fn n113_1_identity_mapping_is_bijective() {
crates/amun-consensus-network/src/validator_identity.rs:267:    fn n113_1_roundtrip_serialization() {
crates/amun-consensus-network/src/validator_identity.rs:274:        assert!(decoded.verify_binding());
crates/amun-consensus-network/src/vote_binding.rs:10:/// If the vote has NO commitment (legacy vote), the check passes
crates/amun-consensus-network/src/vote_binding.rs:132:    fn n109_9_valid_vote_with_commitment_passes() {
crates/amun-consensus-network/src/vote_binding.rs:135:        assert!(verify_vote_binding(&vote).is_ok());
crates/amun-consensus-network/src/vote_binding.rs:139:    fn n109_9_height_mismatch_rejected() {
crates/amun-consensus-network/src/vote_binding.rs:148:    fn n109_9_block_hash_mismatch_rejected() {
crates/amun-consensus-network/src/vote_binding.rs:157:    fn n109_9_tampered_execution_root_rejected() {
crates/amun-consensus-network/src/vote_binding.rs:166:    fn n109_9_unsigned_commitment_rejected() {
crates/amun-consensus-network/src/vote_binding.rs:175:    fn n109_9_wrong_signer_rejected() {
crates/amun-consensus-network/src/vote_binding.rs:196:    fn n109_9_legacy_vote_without_commitment_passes() {
crates/amun-consensus-network/src/vote_binding.rs:19:    // N109.9: If no commitment present, allow (backward compat)
crates/amun-consensus-network/src/vote_binding.rs:206:    fn n109_9_legacy_vote_still_allowed_during_migration() {
crates/amun-consensus-network/src/vote_binding.rs:207:        // Simulate: 3 votes total, 2 with commitment, 1 legacy — all pass
crates/amun-consensus-network/src/vote_binding.rs:218:            "vote with commitment must pass"
crates/amun-consensus-network/src/vote_binding.rs:222:            "vote with commitment must pass"
crates/amun-consensus-network/src/vote_binding.rs:32:            "N109.9 HEIGHT_MISMATCH: vote.height={} commitment.height={}",
crates/amun-consensus-network/src/vote_binding.rs:40:            "N109.9 BLOCK_MISMATCH: vote.block_hash={:?} commitment.block_hash={:?}",
crates/amun-consensus-network/src/vote_binding.rs:55:            "N109.9 EXEC_ROOT_MISMATCH: stated={:?} recomputed={:?}",
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:107:fn n109_8_commitment_roundtrip() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:108:    let commit = make_commitment([1u8; 32], 42, [2u8; 32], [3u8; 32]);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:110:    let encoded = postcard::to_stdvec(&commit).expect("N109.8: serialize");
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:113:    assert_eq!(decoded.validator_id, commit.validator_id);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:114:    assert_eq!(decoded.height, commit.height);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:115:    assert_eq!(decoded.block_hash, commit.block_hash);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:116:    assert_eq!(decoded.state_root, commit.state_root);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:117:    assert_eq!(decoded.execution_root, commit.execution_root);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:11://   7. GATEKEEPER: vote fields match commitment fields
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:121:// TEST 2: Same execution → same commitment (determinism)
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:124:fn n109_8_same_execution_same_commitment() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:125:    let c1 = make_commitment([1u8; 32], 5, [0xAA; 32], [0xBB; 32]);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:126:    let c2 = make_commitment([1u8; 32], 5, [0xAA; 32], [0xBB; 32]);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:137:// TEST 3: Different execution → different commitment
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:140:fn n109_8_different_execution_different_commitment() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:141:    let c1 = make_commitment([1u8; 32], 5, [0xAA; 32], [0xBB; 32]);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:142:    let c2 = make_commitment([1u8; 32], 5, [0xAA; 32], [0xCC; 32]); // Different state_root
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:152:// TEST 4: Tampered commitment detected
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:155:fn n109_8_tampered_commitment_rejected() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:156:    let mut commit = make_commitment([9u8; 32], 3, [0x11; 32], [0x22; 32]);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:159:    commit.state_root = [0xFF; 32];
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:161:    // Recompute — should not match the old execution_root
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:162:    let recomputed = compute_execution_root(
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:163:        &commit.validator_id,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:164:        commit.height,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:165:        &commit.block_hash,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:166:        &commit.state_root,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:169:        recomputed, commit.execution_root,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:178:fn n109_8_validator_cannot_repudiate() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:180:    let commit = make_commitment(pk, 7, [0xDE; 32], [0xAD; 32]);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:183:    // Any third party can recompute it and verify the signature covers it.
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:184:    let recomputed =
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:185:        compute_execution_root(&pk, commit.height, &commit.block_hash, &commit.state_root);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:187:        recomputed, commit.execution_root,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:191:    // The commitment carries the validator's identity
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:193:        commit.validator_id, pk,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:194:        "N109.8 FAIL: commitment must identify the validator"
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:199:// TEST 6: Different validator → different commitment even with same data
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:202:fn n109_8_different_validator_different_commitment() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:206:    let c1 = make_commitment([1u8; 32], 1, bh, sr);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:207:    let c2 = make_commitment([2u8; 32], 1, bh, sr);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:216:// TEST 7: GATEKEEPER — vote fields match commitment fields
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:21:struct ExecutionCommitment {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:222:// The vote and its commitment must refer to the exact same execution.
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:224:fn n109_8_vote_commitment_matches_vote_target() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:232:    // GATEKEEPER CHECK 1: vote.height == commitment.height
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:234:        vote.height, vote.commitment.height,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:235:        "N109.8 GATEKEEPER FAIL: vote.height ({}) != commitment.height ({})",
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:236:        vote.height, vote.commitment.height
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:239:    // GATEKEEPER CHECK 2: vote.block_hash == commitment.block_hash
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:241:        vote.block_hash, vote.commitment.block_hash,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:242:        "N109.8 GATEKEEPER FAIL: vote.block_hash != commitment.block_hash"
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:245:    // GATEKEEPER CHECK 3: vote.state_root == commitment.state_root
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:247:        vote.state_root, vote.commitment.state_root,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:248:        "N109.8 GATEKEEPER FAIL: vote.state_root != commitment.state_root"
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:251:    // GATEKEEPER CHECK 4: vote.voter_id == commitment.validator_id
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:253:        vote.voter_id, vote.commitment.validator_id,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:254:        "N109.8 GATEKEEPER FAIL: vote.voter_id != commitment.validator_id"
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:258:    let recomputed = compute_execution_root(&voter, height, &block_hash, &state_root);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:260:        vote.commitment.execution_root, recomputed,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:266:// TEST 8: Vote with mismatched commitment is detectable
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:269:fn n109_8_mismatched_vote_commitment_detected() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:275:    // Replace commitment with one for block B (attack attempt)
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:276:    vote.commitment = make_commitment(voter, 3, [0xFF; 32], [0xEE; 32]);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:279:    let mismatch = vote.block_hash != vote.commitment.block_hash
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:280:        || vote.state_root != vote.commitment.state_root
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:281:        || vote.height != vote.commitment.height
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:282:        || vote.voter_id != vote.commitment.validator_id;
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:286:        "N109.8 FAIL: vote with mismatched commitment must be detectable"
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:290:        vote.block_hash, vote.commitment.block_hash,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:296:// TEST 9: Vote roundtrip with commitment
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:299:fn n109_8_vote_with_commitment_roundtrip() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:310:        decoded.commitment.execution_root,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:311:        vote.commitment.execution_root
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:314:        decoded.commitment.validator_id,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:315:        vote.commitment.validator_id
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:36:struct ConsensusVote {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:46:    commitment: ExecutionCommitment,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:50:// Helper: compute execution_root = blake3(validator_id || height || block_hash || state_root)
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:52:fn compute_execution_root(
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:64:    hasher.finalize().into()
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:67:// Helper: create a commitment with computed execution_root
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:68:fn make_commitment(
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:6://   2. Sign → verify cycle
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:74:    let execution_root = compute_execution_root(&validator_id, height, &block_hash, &state_root);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:7://   3. Same execution → same commitment
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:85:// Helper: create a vote with commitment
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:86:fn make_vote(
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:8://   4. Different execution → different commitment
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:99:        commitment: make_commitment(voter_id, height, block_hash, state_root),
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:9://   5. Tampered commitment rejected
crates/amun-consensus-network/tests/n109_block_propagation.rs:100:            hex::encode(computed),
crates/amun-consensus-network/tests/n109_block_propagation.rs:110:fn n109_proposal_roundtrip() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:127:fn n109_block_hash_matches_serialized_block() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:129:    let computed: [u8; 32] = blake3::hash(&proposal.block_bytes).into();
crates/amun-consensus-network/tests/n109_block_propagation.rs:131:        proposal.block_hash, computed,
crates/amun-consensus-network/tests/n109_block_propagation.rs:140:fn n109_network_message_vote_roundtrip() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:169:fn n109_network_message_proposal_roundtrip() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:193:fn n109_proposal_cache_insert() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:19:struct BlockProposal {
crates/amun-consensus-network/tests/n109_block_propagation.rs:211:fn n109_proposal_cache_cleanup() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:229:fn n109_listener_stores_proposal() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:257:fn n109_validate_basic_accepts_valid_proposal() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:260:    let result = validate_basic_testable(&p, 0, &parent, 1050);
crates/amun-consensus-network/tests/n109_block_propagation.rs:272:fn n109_validate_basic_rejects_wrong_height() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:275:    let result = validate_basic_testable(&p, 0, &parent, 5000);
crates/amun-consensus-network/tests/n109_block_propagation.rs:287:fn n109_validate_basic_rejects_parent_mismatch() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:289:    let result = validate_basic_testable(&p, 0, &[0xBB; 32], 1000);
crates/amun-consensus-network/tests/n109_block_propagation.rs:304:fn n109_validate_basic_rejects_hash_mismatch() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:307:    let result = validate_basic_testable(&p, 0, &[0u8; 32], 1000);
crates/amun-consensus-network/tests/n109_block_propagation.rs:30:struct ConsensusVoteStub {
crates/amun-consensus-network/tests/n109_block_propagation.rs:319:fn n109_vote_before_proposal_does_not_crash() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:342:fn n109_duplicate_proposal_is_idempotent() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:42:enum NetworkMessage {
crates/amun-consensus-network/tests/n109_block_propagation.rs:4:// N109 — Constitutional Block Propagation — Test Suite
crates/amun-consensus-network/tests/n109_block_propagation.rs:51:fn make_test_proposal(height: u64, parent: [u8; 32]) -> BlockProposal {
crates/amun-consensus-network/tests/n109_block_propagation.rs:56:    let block_bytes = hasher.finalize().as_bytes().to_vec();
crates/amun-consensus-network/tests/n109_block_propagation.rs:61:    let state_root: [u8; 32] = state_hasher.finalize().into();
crates/amun-consensus-network/tests/n109_block_propagation.rs:6:// Tests N109.1 through N109.6 before building N109.7.
crates/amun-consensus-network/tests/n109_block_propagation.rs:73:fn validate_basic_testable(
crates/amun-consensus-network/tests/n109_block_propagation.rs:95:    let computed: [u8; 32] = blake3::hash(&p.block_bytes).into();
crates/amun-consensus-network/tests/n109_block_propagation.rs:96:    if computed != p.block_hash {
crates/amun-consensus-network/tests/n109_block_propagation.rs:98:            "HASH_INTEGRITY: stated={} computed={}",
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:103:        state_root: honest_root, // Proposer computed correctly
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:108:    let result = verify_block_execution(&proposal, |bytes| simulate_executor(bytes, 42));
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:119:// TEST: Validator C re-executes and MISMATCHES → vote rejected
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:122:fn n109_7_re_execution_rejects_mismatched_state_root() {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:137:    let result = verify_block_execution(&proposal, |bytes| simulate_executor(bytes, 99));
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:154:fn n109_7_three_validators_one_mismatch_no_qc() {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:15:struct BlockProposal {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:181:        let result = verify_block_execution(&proposal, |bytes| simulate_executor(bytes, seed));
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:198:    // Constitutional check: 2 approvals out of 4 total validators (not > 2/3 of 4=3)
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:233:fn n109_7b_metrics_counts_state_root_mismatches() {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:248:    let result = verify_block_execution(&proposal, |bytes| simulate_executor(bytes, 1));
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:264:fn n109_7_execution_failure_does_not_vote() {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:277:    let result = verify_block_execution(&proposal, |_bytes| {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:294:// TEST: Cache retention — proposal stays until finalized
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:297:fn n109_7c_proposal_retained_until_finalized() {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:29:struct ExecutionReceipt {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:313:    // N109.7: Re-execute using cached proposal
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:317:    let result = verify_block_execution(cached, |bytes| simulate_executor(bytes, 1));
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:341:fn n109_7_execution_is_deterministic() {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:363:fn n109_7_different_blocks_different_roots() {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:40:struct ConsensusMetrics {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:51:fn verify_block_execution<F>(
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:58:    let computed_root =
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:61:    if computed_root != proposal.state_root {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:63:            "N109.7 STATE_ROOT_MISMATCH: height={} proposed={} computed={}",
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:66:            hex::encode(computed_root),
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:71:        state_root: computed_root,
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:81:fn simulate_executor(block_bytes: &[u8], seed: u64) -> Result<[u8; 32], String> {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:86:    Ok(hasher.finalize().into())
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:8:// Constitutional requirement: state_root mismatch → no vote → no QC formed.
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:90:// TEST: Validator B re-executes and matches → vote accepted
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:93:fn n109_7_re_execution_accepts_matching_state_root() {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:103:    let mut store_b = EvidenceStore::new();
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:104:    let _gossip_b = EvidenceGossip::new();
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:106:    // Node B tries to validate — evidence is missing
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:107:    let result_b = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:109:        EvidenceValidationResult::MissingEvidence { missing_ids } => missing_ids,
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:121:    let request = build_missing_evidence_request([0xBB; 32], missing_ids);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:12:    build_missing_evidence_request, process_evidence_response, validate_certificate_evidence,
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:131:    let response = amun_consensus_network::MissingEvidenceResponse {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:13:    CertificateGossip, EvidenceAnnouncement, EvidenceCount, EvidenceGossip, EvidenceRecord,
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:14:    EvidenceStore, EvidenceType, EvidenceValidationResult, SlashingCertificate, ValidatorStatus,
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:151:    // Phase 5: Node B re-validates — all evidence now present
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:153:    let result_b_after = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:156:        EvidenceValidationResult::AllPresent,
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:167:fn n111_7_certificate_gossip_between_nodes() {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:171:    let mut store_a = EvidenceStore::new();
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:172:    let ev = make_evidence_record(validator_id, 1, EvidenceType::DoubleVote, 1);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:184:    let mut store_b = EvidenceStore::new();
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:18:fn make_certificate(validator_id: [u8; 32], evidence_ids: Vec<[u8; 32]>) -> SlashingCertificate {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:192:    let result = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:193:    assert_ne!(result, EvidenceValidationResult::AllPresent);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:197:    let response = amun_consensus_network::MissingEvidenceResponse {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:203:    let result_after = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:204:    assert_eq!(result_after, EvidenceValidationResult::AllPresent);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:23:        vec![EvidenceCount {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:24:            evidence_type: EvidenceType::DoubleVote,
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:2:// N111.7 — End-to-End Evidence Sync Across Two Simulated Nodes
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:37:/// Helper: create a realistic EvidenceRecord.
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:38:fn make_evidence_record(
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:41:    evidence_type: EvidenceType,
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:43:) -> EvidenceRecord {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:44:    EvidenceRecord::new(
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:57:fn n111_7_evidence_sync_end_to_end() {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:63:    let mut store_a = EvidenceStore::new();
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:64:    let mut gossip_a = EvidenceGossip::new();
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:67:    let ev1 = make_evidence_record(validator_id, 1, EvidenceType::DoubleVote, 1);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:68:    let ev2 = make_evidence_record(validator_id, 2, EvidenceType::DoubleVote, 2);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:69:    let ev3 = make_evidence_record(validator_id, 3, EvidenceType::DoubleVote, 3);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:80:        gossip_a.receive_announcement(EvidenceAnnouncement {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:92:    // Node A validates locally — all evidence present
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:93:    let result_a = validate_certificate_evidence(&cert, &store_a);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:96:        EvidenceValidationResult::AllPresent,
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:100:    let mut push_b = EvidencePush::default();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:103:    let result_before = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:105:        EvidenceValidationResult::MissingEvidence { missing_ids } => {
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:112:        _ => panic!("Expected MissingEvidence before push"),
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:11:    process_incoming_evidence_push, validate_certificate_evidence, EvidenceCount, EvidenceGossip,
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:128:    // Phase 4: Node B validates the certificate — ACCEPTED without pull
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:12:    EvidencePush, EvidenceRecord, EvidenceStore, EvidenceType, EvidenceValidationResult,
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:130:    let result_after = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:133:        EvidenceValidationResult::AllPresent,
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:134:        "N112.4 FAIL: After push, certificate must be accepted without MissingEvidenceRequest"
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:147:    eprintln!("N112.4 GATEKEEPER PASSED: push sync eliminated MissingEvidenceRequest cycle");
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:154:fn n112_4_push_duplicate_is_harmless() {
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:155:    let mut store = EvidenceStore::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:156:    let mut gossip = EvidenceGossip::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:157:    let mut push = EvidencePush::default();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:16:fn make_evidence(validator_id: [u8; 32], height: u64, seed: u8) -> EvidenceRecord {
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:177:// N112.4: Certificate validated immediately after push (no pull cycle)
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:17:    EvidenceRecord::new(
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:180:fn n112_4_certificate_immediately_accepted_after_push() {
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:184:    let mut store_a = EvidenceStore::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:185:    let gossip_a = EvidenceGossip::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:186:    let mut push_a = EvidencePush::default();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:197:    let mut store_b = EvidenceStore::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:198:    let mut gossip_b = EvidenceGossip::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:199:    let mut push_b = EvidencePush::default();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:204:    // Certificate accepted — NO MissingEvidenceRequest needed
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:205:    let result = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:206:    assert_eq!(result, EvidenceValidationResult::AllPresent);
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:20:        EvidenceType::DoubleVote,
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:26:fn make_certificate(validator_id: [u8; 32], evidence_ids: Vec<[u8; 32]>) -> SlashingCertificate {
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:2:// N112.4 — End-to-End Push Evidence Sync
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:31:        vec![EvidenceCount {
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:32:            evidence_type: EvidenceType::DoubleVote,
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:46:// N112.4 GATEKEEPER — Push sync eliminates the need for MissingEvidenceRequest
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:49:fn n112_4_push_sync_end_to_end() {
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:55:    let mut store_a = EvidenceStore::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:56:    let gossip_a = EvidenceGossip::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:57:    let mut push_a = EvidencePush::default();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:6:// MissingEvidenceRequest/Response cycle.  The certificate is accepted
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:98:    let mut store_b = EvidenceStore::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:99:    let mut gossip_b = EvidenceGossip::new();
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:104:fn n114_3_signature_changes_with_certificate_content() {
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:119:    assert!(cert1.verify_signature().is_ok());
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:120:    assert!(cert2.verify_signature().is_ok());
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:127:fn n114_3_signed_certificate_roundtrip() {
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:12:        vec![EvidenceCount {
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:138:        decoded.verify_signature().is_ok(),
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:139:        "N114.3 FAIL: Roundtripped certificate must verify"
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:13:            evidence_type: EvidenceType::DoubleVote,
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:30:fn n114_3_signed_certificate_verifies() {
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:38:        signing_key.verifying_key().to_bytes()
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:42:        cert.verify_signature().is_ok(),
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:43:        "N114.3 FAIL: Signed certificate must verify"
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:4:use amun_consensus_network::{EvidenceCount, EvidenceType, SlashingCertificate, ValidatorStatus};
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:51:fn n114_3_unsigned_certificate_rejected() {
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:59:        cert.verify_signature().is_err(),
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:68:fn n114_3_tampered_certificate_rejected() {
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:77:        cert.verify_signature().is_err(),
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:7:fn make_unsigned_certificate(validator_id: [u8; 32]) -> SlashingCertificate {
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:86:fn n114_3_wrong_public_key_rejected() {
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:95:        cert.verify_signature().is_err(),
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:100:    assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:103:/// N121.2: Single event replay is deterministic
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:105:fn n121_2_single_event_replay_deterministic() {
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:109:    state1.execute(&cert, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:10:fn make_cert(validator_id: [u8; 32], height: u64, amount: u64) -> SlashingCertificate {
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:112:    state2.execute(&cert, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:116:        "N121.2 FAIL: single event replay must be deterministic"
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:122:fn n121_2_consistency_verification_catches_corruption() {
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:125:    state.execute(&cert, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:131:        state.verify_consistency().is_err(),
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:15:        vec![EvidenceCount {
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:16:            evidence_type: EvidenceType::DoubleVote,
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:1:// N121.2 — Deterministic Replay of Slashing State
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:29:/// N121.2 Gatekeeper: Three nodes replay the same events → same root
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:31:fn n121_2_three_nodes_same_replay_same_root() {
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:3:// Verifies that replaying the same sequence of slashing certificates
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:41:        state_a.execute(cert, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:47:        state_b.execute(cert, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:53:        state_c.execute(cert, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:64:    assert_eq!(state_a.executed_count(), 3);
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:65:    assert_eq!(state_b.executed_count(), 3);
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:66:    assert_eq!(state_c.executed_count(), 3);
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:71:fn n121_2_different_order_different_root() {
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:77:    state_a.execute(&cert1, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:78:    state_a.execute(&cert2, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:7:    EvidenceCount, EvidenceType, SlashingCertificate, SlashingState, ValidatorStatus,
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:82:    state_b.execute(&cert2, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:83:    state_b.execute(&cert1, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:92:/// N121.2: Replay from empty always produces zero root
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:94:fn n121_2_replay_empty_produces_zero_root() {
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:11:fn make_cert(vid: [u8; 32], h: u64, amt: u64) -> SlashingCertificate {
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:130:fn n121_6_total_slash_count_auditable() {
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:132:    assert_eq!(state.executed_count(), 0);
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:135:        .execute(&make_cert([0x42; 32], 100, 15000), || Ok(()))
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:137:    assert_eq!(state.executed_count(), 1);
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:140:        .execute(&make_cert([0x99; 32], 200, 5000), || Ok(()))
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:142:    assert_eq!(state.executed_count(), 2);
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:147:fn n121_6_empty_state_empty_history() {
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:150:    assert_eq!(state.executed_count(), 0);
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:16:        vec![EvidenceCount {
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:17:            evidence_type: EvidenceType::DoubleVote,
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:32:fn n121_5_snapshot_root_matches_state() {
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:35:        .execute(&make_cert([0x42; 32], 100, 15000), || Ok(()))
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:38:        .execute(&make_cert([0x99; 32], 200, 5000), || Ok(()))
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:46:        state.verify_consistency().is_ok(),
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:57:fn n121_5_restored_state_from_snapshot() {
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:58:    // Simulate: build state, snapshot it, rebuild from scratch
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:68:        original.execute(cert, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:72:    // Rebuild from the same certificates (simulating restore from snapshot)
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:75:        restored.execute(cert, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:82:    assert!(restored.verify_consistency().is_ok());
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:87:fn n121_6_audit_trail_by_validator() {
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:8:    EvidenceCount, EvidenceType, SlashingCertificate, SlashingState, ValidatorStatus,
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:91:        .execute(&make_cert([0x42; 32], 100, 15000), || Ok(()))
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:94:        .execute(&make_cert([0x99; 32], 200, 5000), || Ok(()))
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:97:        .execute(&make_cert([0x42; 32], 300, 10000), || Ok(()))
crates/amun-consensus-signatures/src/domain.rs:3:/// Domain-separated signing contexts to prevent cross-message replay.
crates/amun-consensus/src/round_state_machine.rs:129:        // N126.1: Constitutional state transition verification
crates/amun-consensus/src/round_state_machine.rs:132:        //   - ReplayDeterminism (replay root == original root)
crates/amun-consensus/src/round_state_machine.rs:143:                true, // replay_deterministic
crates/amun-consensus/src/round_state_machine.rs:145:                true, // state_transition_valid (verified by N126)
crates/amun-constitution-builder/src/certificate.rs:39:            "replay semantics".into(),
crates/amun-constitution-builder/src/federation.rs:23:    pub replay_boundary: String,
crates/amun-constitution-builder/src/federation.rs:51:            "Replay isolation preserved".into(),
crates/amun-constitution-builder/src/federation.rs:63:            replay_boundary: "Treaty-scoped".into(),
crates/amun-constitution-builder/src/federation.rs:89:        lines.push(format!("Replay Boundary: {}", self.replay_boundary));
crates/amun-constitution-builder/src/treaty.rs:18:    pub replay_boundaries: Vec<String>,
crates/amun-constitution-builder/src/treaty.rs:38:            replay_boundaries: vec!["Treaty-scoped".into()],
crates/amun-constitution-builder/src/treaty.rs:64:        lines.push("Replay Boundaries:".to_string());
crates/amun-constitution-builder/src/treaty.rs:65:        for b in &self.replay_boundaries {
crates/amun-constitution-builder/src/verify.rs:11:    pub fn verify_replay<T: CanonicalSerialize + PartialEq>(
crates/amun-constitution-builder/tests/determinism_tests.rs:12:    VerificationEngine::verify_replay(&m1, &m2).expect("Manifests must be identical");
crates/amun-constitution-builder/tests/determinism_tests.rs:40:    VerificationEngine::verify_replay(&f1, &f2).expect("Federation artifacts must be identical");
crates/amun-constitution-builder/tests/determinism_tests.rs:58:    VerificationEngine::verify_replay(&t1, &t2).expect("Treaties must be identical");
crates/amun-constitution/src/freeze_map.rs:38:    pub requires_replay_preservation: bool,
crates/amun-constitution/src/freeze_map.rs:53:                requires_replay_preservation: true,
crates/amun-constitution/src/freeze_map.rs:55:                description: "Tree depth - changing this breaks all proofs, replays, and snapshots",
crates/amun-constitution/src/freeze_map.rs:62:                requires_replay_preservation: true,
crates/amun-constitution/src/freeze_map.rs:71:                requires_replay_preservation: true,
crates/amun-constitution/src/freeze_map.rs:80:                requires_replay_preservation: true,
crates/amun-constitution/src/freeze_map.rs:89:                requires_replay_preservation: false,
crates/amun-constitution/src/freeze_map.rs:8:    /// Amendment requires constitutional supermajority + replay proof
crates/amun-constitution/src/freeze_map.rs:98:                requires_replay_preservation: false,
crates/amun-constitution/src/freeze_validator.rs:17:    pub preserves_replay: bool,
crates/amun-constitution/src/freeze_validator.rs:48:        if boundary.requires_replay_preservation && !context.preserves_replay {
crates/amun-constitution/src/freeze_validator.rs:49:            return Err(FreezeViolation::ReplayPreservationViolation {
crates/amun-constitution/src/freeze_validator.rs:67:        } else if boundary.requires_replay_preservation || boundary.requires_snapshot_compatibility
crates/amun-constitution/src/freeze_validator.rs:8:    ReplayPreservationViolation { field: String },
crates/amun-constitution/src/replay.rs:16:impl ReplayContext {
crates/amun-constitution/src/replay.rs:2:use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
crates/amun-constitution/src/replay.rs:39:    pub fn encode_for_signing(&self) -> [u8; Self::ENCODED_SIZE] {
crates/amun-constitution/src/replay.rs:53:impl CanonicalEncode for ReplayContext {
crates/amun-constitution/src/replay.rs:55:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-constitution/src/replay.rs:61:impl CanonicalDecode for ReplayContext {
crates/amun-constitution/src/replay.rs:62:    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
crates/amun-constitution/src/replay.rs:65:                ConstitutionalFault::BufferTooSmall,
crates/amun-constitution/src/replay.rs:6:pub struct ReplayContext {
crates/amun-constitution/src/replay.rs:71:            FailureContext::new(ConstitutionalFault::MalformedEncoding, 0x0006, 0x0B01)
crates/amun-constitution/src/replay.rs:74:            FailureContext::new(ConstitutionalFault::MalformedEncoding, 0x0006, 0x0B02)
crates/amun-constitution/src/replay.rs:79:            FailureContext::new(ConstitutionalFault::MalformedEncoding, 0x0006, 0x0B03)
crates/amun-constitution/src/replay.rs:82:            FailureContext::new(ConstitutionalFault::MalformedEncoding, 0x0006, 0x0B04)
crates/amun-constitution/src/replay.rs:85:            FailureContext::new(ConstitutionalFault::MalformedEncoding, 0x0006, 0x0B05)
crates/amun-constitution/src/replay.rs:88:            FailureContext::new(ConstitutionalFault::MalformedEncoding, 0x0006, 0x0B06)
crates/amun-constitution/src/tests.rs:10:    assert!(c1.verify_compatible(&c2).is_ok());
crates/amun-constitution/src/tests.rs:20:    assert!(params.verify_safety().is_ok());
crates/amun-constitution/src/tests.rs:30:    assert!(params.verify_safety().is_err());
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:55:    assert!(chain.verify().is_ok());
crates/amun-constitutional-authority/src/lib.rs:7://! This crate provides the primitives for building replay-safe,
crates/amun-constitutional-authority/src/revocation.rs:11:/// guaranteeing replay stability.
crates/amun-constitutional-authority/tests/authority_tests.rs:112:    assert!(KeyRotationLaw::validate_rotation(&old.verifying_key_hex(), &signed, &old).is_ok());
crates/amun-constitutional-authority/tests/authority_tests.rs:122:    assert!(anchor.verify().is_ok());
crates/amun-constitutional-authority/tests/authority_tests.rs:42:fn valid_chain_passes_validation() {
crates/amun-constitutional-authority/tests/authority_tests.rs:52:    assert!(chain.validate(&RevocationRegistry::new()).is_ok());
crates/amun-constitutional-authority/tests/authority_tests.rs:65:    assert!(chain.validate(&RevocationRegistry::new()).is_err());
crates/amun-constitutional-authority/tests/authority_tests.rs:95:    assert!(chain.validate(&reg).is_err());
crates/amun-constitutional-block/src/block.rs:100:            replay_certificate_root,
crates/amun-constitutional-block/src/block.rs:17:    pub replay_certificate_root: String,
crates/amun-constitutional-block/src/block.rs:33:        replay_certificate_root: String,
crates/amun-constitutional-block/src/block.rs:46:            replay_certificate_root,
crates/amun-constitutional-block/src/block.rs:83:        replay_certificate_root: String,
crates/amun-constitutional-block/src/finalizer.rs:34:        let (state_root, replay_certificate_root) = match &ctx.state_runtime {
crates/amun-constitutional-block/src/finalizer.rs:53:            replay_certificate_root,
crates/amun-constitutional-block/src/lib.rs:107:/// the block header, a ReplayCertificate, and an inclusion proof.
crates/amun-constitutional-block/src/lib.rs:109:/// No full state, no journal replay, no full certificate set needed.
crates/amun-constitutional-block/src/lib.rs:114:///   3. Merkle root matches block.replay_certificate_root
crates/amun-constitutional-block/src/lib.rs:118:    cert: &ReplayCertificate,
crates/amun-constitutional-block/src/lib.rs:137:    if block.replay_certificate_root != proof_root {
crates/amun-constitutional-block/src/lib.rs:140:            block.replay_certificate_root, proof_root
crates/amun-constitutional-block/src/lib.rs:39:use amun_constitutional_state::{ConstitutionalStateRuntime, ReplayCertificate};
crates/amun-constitutional-block/src/lib.rs:44:/// 1. Block's replay_certificate_root matches the certificate's merkle root.
crates/amun-constitutional-block/src/lib.rs:47:/// Note: Full replay verification (N6B) will add journal-based validation.
crates/amun-constitutional-block/src/lib.rs:50:    cert: &ReplayCertificate,
crates/amun-constitutional-block/src/lib.rs:56:    if block.replay_certificate_root != computed_root {
crates/amun-constitutional-block/src/lib.rs:59:            block.replay_certificate_root, computed_root
crates/amun-constitutional-block/src/lib.rs:77:/// Full replay verification: cryptographic proof that the block's state
crates/amun-constitutional-block/src/lib.rs:81:///   Block → Certificate → Journal → Replay → StateRoot → Accept/Reject
crates/amun-constitutional-block/src/lib.rs:85:///   - ReplayCertificate::verify() for journal↔state proof
crates/amun-constitutional-block/src/lib.rs:86:pub fn verify_full_replay(
crates/amun-constitutional-block/src/lib.rs:88:    cert: &ReplayCertificate,
crates/amun-constitutional-block/src/lib.rs:97:            "Full replay verification failed: journal does not produce the claimed state".into(),
crates/amun-constitutional-block/tests/block_tests.rs:192:    assert!(verify_block_provenance(&block, &cert).is_ok());
crates/amun-constitutional-block/tests/block_tests.rs:220:    assert!(verify_block_provenance(&block, &cert).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:248:    assert!(verify_block_provenance(&block, &cert).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:252:fn n6b_full_replay_valid() {
crates/amun-constitutional-block/tests/block_tests.rs:253:    use amun_constitutional_block::verify_full_replay;
crates/amun-constitutional-block/tests/block_tests.rs:277:    assert!(verify_full_replay(&block, &cert, rt.journal()).is_ok());
crates/amun-constitutional-block/tests/block_tests.rs:281:fn n6b_full_replay_tampered_journal_fails() {
crates/amun-constitutional-block/tests/block_tests.rs:282:    use amun_constitutional_block::verify_full_replay;
crates/amun-constitutional-block/tests/block_tests.rs:309:    assert!(verify_full_replay(&block, &cert, &tampered).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:313:fn n6b_full_replay_wrong_count_fails() {
crates/amun-constitutional-block/tests/block_tests.rs:314:    use amun_constitutional_block::verify_full_replay;
crates/amun-constitutional-block/tests/block_tests.rs:339:    assert!(verify_full_replay(&block, &cert, &rt.journal()[..1]).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:343:fn n8_light_client_valid() {
crates/amun-constitutional-block/tests/block_tests.rs:369:    assert!(verify_light_client_proof(&block, &cert, &inclusion_proof).is_ok());
crates/amun-constitutional-block/tests/block_tests.rs:373:fn n8_light_client_tampered_proof_fails() {
crates/amun-constitutional-block/tests/block_tests.rs:402:    assert!(verify_light_client_proof(&block, &cert, &inclusion_proof).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:406:fn n8_light_client_wrong_certificate_fails() {
crates/amun-constitutional-block/tests/block_tests.rs:438:    assert!(verify_light_client_proof(&block, &cert2, &inclusion_proof).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:442:fn n8_light_client_wrong_block_root_fails() {
crates/amun-constitutional-block/tests/block_tests.rs:454:    // Block has a different replay_certificate_root
crates/amun-constitutional-block/tests/block_tests.rs:468:    assert!(verify_light_client_proof(&block, &cert, &inclusion_proof).is_err());
crates/amun-constitutional-block/tests/finalizer_tests.rs:29:    assert!(!block.replay_certificate_root.is_empty());
crates/amun-constitutional-block/tests/finalizer_tests.rs:60:    let b1 = BlockFinalizer::finalize(&mut chain1, &log, ctx1).unwrap();
crates/amun-constitutional-block/tests/finalizer_tests.rs:61:    let b2 = BlockFinalizer::finalize(&mut chain2, &log, ctx2).unwrap();
crates/amun-constitutional-block/tests/finalizer_tests.rs:65:    assert_ne!(b1.replay_certificate_root, b2.replay_certificate_root);
crates/amun-constitutional-block/tests/replay_tests.rs:21:    BlockFinalizer::finalize(&mut chain, &log, ctx).unwrap();
crates/amun-constitutional-block/tests/replay_tests.rs:22:    chain.verify_block_evidence(0, &log).unwrap();
crates/amun-constitutional-block/tests/replay_tests.rs:26:fn test_tampered_log_fails_verification() {
crates/amun-constitutional-block/tests/replay_tests.rs:39:    BlockFinalizer::finalize(&mut chain, &log, ctx).unwrap();
crates/amun-constitutional-block/tests/replay_tests.rs:3:    finalizer::{BlockFinalizer, FinalizationContext},
crates/amun-constitutional-block/tests/replay_tests.rs:43:    assert!(chain.verify_block_evidence(0, &tampered).is_err());
crates/amun-constitutional-block/tests/replay_tests.rs:47:fn test_full_chain_replay_audit() {
crates/amun-constitutional-block/tests/replay_tests.rs:61:        BlockFinalizer::finalize(&mut chain, &log, ctx).unwrap();
crates/amun-constitutional-block/tests/replay_tests.rs:64:    chain.verify_chain_evidence(&logs).unwrap();
crates/amun-constitutional-block/tests/replay_tests.rs:8:fn test_evidence_root_matches_block() {
crates/amun-constitutional-commitment/tests/determinism.rs:62:    assert_eq!(commitment_root(&c1), commitment_root(&c2));
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:28:    assert!(commitment.is_some());
crates/amun-constitutional-commitment/tests/state.rs:133:    assert!(ConstitutionalState::load(&[0u8; 10]).is_none());
crates/amun-constitutional-commitment/tests/state.rs:134:    assert!(ConstitutionalState::load(&[0u8; 200]).is_none());
crates/amun-constitutional-commitment/tests/state.rs:135:    assert!(ConstitutionalState::load(&[]).is_none());
crates/amun-constitutional-commitment/tests/state.rs:44:    assert_eq!(commitment.version, loaded.version);
crates/amun-constitutional-commitment/tests/state.rs:45:    assert_eq!(commitment.identity_root, loaded.identity_root);
crates/amun-constitutional-commitment/tests/state.rs:46:    assert_eq!(commitment.evidence_root, loaded.evidence_root);
crates/amun-constitutional-commitment/tests/state.rs:47:    assert_eq!(commitment.governance_root, loaded.governance_root);
crates/amun-constitutional-commitment/tests/state.rs:48:    assert_eq!(commitment.economic_root, loaded.economic_root);
crates/amun-constitutional-commitment/tests/state.rs:49:    assert_eq!(commitment.constitutional_root, loaded.constitutional_root);
crates/amun-constitutional-commitment/tests/verify.rs:123:    assert!(Verifier::verified(&result));
crates/amun-constitutional-commitment/tests/verify.rs:128:    assert_eq!(result.recomputed_commitment_root, stored_commitment);
crates/amun-constitutional-commitment/tests/verify.rs:42:    assert!(result.commitment_root_match);
crates/amun-constitutional-commitment/tests/verify.rs:43:    assert!(Verifier::verified(&result));
crates/amun-constitutional-commitment/tests/verify.rs:69:    assert!(!Verifier::verified(&result));
crates/amun-constitutional-commitment/tests/verify.rs:95:    assert!(!Verifier::verified(&result));
crates/amun-constitutional-commitments/tests/commitment_tests.rs:38:    assert!(state_tree.verify(&state_root, &proof));
crates/amun-constitutional-commitments/tests/commitment_tests.rs:43:    assert!(!gov_tree.verify(&gov_root, &proof));
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:101:// N127A.2 — Execution Evidence Adapter
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:103:/// Evidence sourced from the ExecutionEngine (N109.7, N109.8).
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:113:// N127A.3 — Replay Evidence Adapter
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:115:/// Evidence sourced from ReplayVerifier or N109.7 architectural guarantee.
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:117:pub struct ReplayEvidence {
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:118:    pub replay_deterministic: bool,
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:122:// N127A.4 — Governance Evidence Adapter
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:131:// N127A.5 — QC Evidence Adapter
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:13:    // N127A.2: From ExecutionEngine
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:141:// N127A.1 — Constitutional Evidence Builder
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:149:        replay: ReplayEvidence,
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:160:            replay_deterministic: replay.replay_deterministic,
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:175:    fn n127a_1_evidence_struct_creation() {
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:183:    fn n127a_1_builder_assembles_all_evidence() {
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:190:        let replay = ReplayEvidence {
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:191:            replay_deterministic: true,
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:19:    // N127A.3: From ReplayVerifier (N109.7 architectural guarantee)
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:201:        let evidence = ConstitutionalEvidenceBuilder::build(exec, replay, gov, qc, true, true);
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:204:        assert!(evidence.replay_deterministic);
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:20:    pub replay_deterministic: bool,
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:212:    fn n127a_1_evidence_defaults_are_explicit() {
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:2:// N127A — Constitutional Evidence Interface
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:45:        replay_deterministic: bool,
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:57:            replay_deterministic,
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:79:    /// N127A.3: Build from replay verification.
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:80:    pub fn from_replay(replay_deterministic: bool) -> ReplayEvidence {
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:81:        ReplayEvidence {
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:82:            replay_deterministic,
crates/amun-constitutional-enforcement/src/evidence_providers.rs:102:        replay.replay_deterministic,
crates/amun-constitutional-enforcement/src/evidence_providers.rs:10:/// N127D: Provides execution evidence from available commit-time data.
crates/amun-constitutional-enforcement/src/evidence_providers.rs:114:    fn n127d_execution_provider_from_cert() {
crates/amun-constitutional-enforcement/src/evidence_providers.rs:121:        assert!(evidence.transition_valid);
crates/amun-constitutional-enforcement/src/evidence_providers.rs:125:    fn n127d_replay_provider_with_qc() {
crates/amun-constitutional-enforcement/src/evidence_providers.rs:126:        let evidence = ReplayEvidenceProvider::from_qc_and_state(true, true, true);
crates/amun-constitutional-enforcement/src/evidence_providers.rs:127:        assert!(evidence.replay_deterministic);
crates/amun-constitutional-enforcement/src/evidence_providers.rs:131:    fn n127d_replay_provider_without_qc() {
crates/amun-constitutional-enforcement/src/evidence_providers.rs:132:        let evidence = ReplayEvidenceProvider::from_qc_and_state(false, true, true);
crates/amun-constitutional-enforcement/src/evidence_providers.rs:133:        assert!(!evidence.replay_deterministic);
crates/amun-constitutional-enforcement/src/evidence_providers.rs:137:    fn n127d_assemble_all_evidence() {
crates/amun-constitutional-enforcement/src/evidence_providers.rs:149:        assert!(evidence.replay_deterministic);
crates/amun-constitutional-enforcement/src/evidence_providers.rs:1:// N127D — Direct Evidence Providers
crates/amun-constitutional-enforcement/src/evidence_providers.rs:35:/// N127D: Provides replay evidence.
crates/amun-constitutional-enforcement/src/evidence_providers.rs:36:/// N109.7 guarantees that every validator re-executes the block
crates/amun-constitutional-enforcement/src/evidence_providers.rs:37:/// and compares state_root before voting. QC formation = replay verified.
crates/amun-constitutional-enforcement/src/evidence_providers.rs:38:pub struct ReplayEvidenceProvider;
crates/amun-constitutional-enforcement/src/evidence_providers.rs:40:impl ReplayEvidenceProvider {
crates/amun-constitutional-enforcement/src/evidence_providers.rs:45:    ) -> ReplayEvidence {
crates/amun-constitutional-enforcement/src/evidence_providers.rs:46:        ReplayEvidence {
crates/amun-constitutional-enforcement/src/evidence_providers.rs:48:            replay_deterministic: qc_verified && state_root_valid && transition_valid,
crates/amun-constitutional-enforcement/src/evidence_providers.rs:7:    ConstitutionalEvidence, ExecutionEvidence, GovernanceEvidence, QcEvidence, ReplayEvidence,
crates/amun-constitutional-enforcement/src/evidence_providers.rs:87:    let replay = ReplayEvidenceProvider::from_qc_and_state(
crates/amun-constitutional-enforcement/src/evidence_records.rs:106:    pub replay_evidence: ReplayEvidence,
crates/amun-constitutional-enforcement/src/evidence_records.rs:129:        replay_evidence: ReplayEvidence,
crates/amun-constitutional-enforcement/src/evidence_records.rs:143:            replay_evidence,
crates/amun-constitutional-enforcement/src/evidence_records.rs:178:    fn n128_signature_evidence_all_valid() {
crates/amun-constitutional-enforcement/src/evidence_records.rs:186:    fn n128_signature_evidence_with_failures() {
crates/amun-constitutional-enforcement/src/evidence_records.rs:192:    fn n128_double_spend_evidence_clean() {
crates/amun-constitutional-enforcement/src/evidence_records.rs:198:    fn n128_double_spend_evidence_detected() {
crates/amun-constitutional-enforcement/src/evidence_records.rs:1:// N128 — Evidence Records
crates/amun-constitutional-enforcement/src/evidence_records.rs:204:    fn n128_replay_evidence_deterministic() {
crates/amun-constitutional-enforcement/src/evidence_records.rs:206:        let ev = ReplayEvidence::new(root, root);
crates/amun-constitutional-enforcement/src/evidence_records.rs:211:    fn n128_replay_evidence_divergent() {
crates/amun-constitutional-enforcement/src/evidence_records.rs:212:        let ev = ReplayEvidence::new([0x42; 32], [0xFF; 32]);
crates/amun-constitutional-enforcement/src/evidence_records.rs:217:    fn n128_evidence_record_hash_verifiable() {
crates/amun-constitutional-enforcement/src/evidence_records.rs:224:            ReplayEvidence::new([0x42; 32], [0x42; 32]),
crates/amun-constitutional-enforcement/src/evidence_records.rs:233:        assert!(record.verify());
crates/amun-constitutional-enforcement/src/evidence_records.rs:237:    fn n128_tampered_record_detected() {
crates/amun-constitutional-enforcement/src/evidence_records.rs:244:            ReplayEvidence::new([0x42; 32], [0x42; 32]),
crates/amun-constitutional-enforcement/src/evidence_records.rs:253:        assert!(!record.verify());
crates/amun-constitutional-enforcement/src/evidence_records.rs:72:/// N128: Structured replay evidence.
crates/amun-constitutional-enforcement/src/evidence_records.rs:74:pub struct ReplayEvidence {
crates/amun-constitutional-enforcement/src/evidence_records.rs:78:    /// State root from replay execution
crates/amun-constitutional-enforcement/src/evidence_records.rs:80:    pub replay_state_root: [u8; 32],
crates/amun-constitutional-enforcement/src/evidence_records.rs:85:impl ReplayEvidence {
crates/amun-constitutional-enforcement/src/evidence_records.rs:86:    pub fn new(original: [u8; 32], replay: [u8; 32]) -> Self {
crates/amun-constitutional-enforcement/src/evidence_records.rs:89:            replay_state_root: replay,
crates/amun-constitutional-enforcement/src/evidence_records.rs:90:            deterministic: original == replay,
crates/amun-constitutional-enforcement/src/lib.rs:110:                ConstitutionalLaw::ReplayDeterminism,
crates/amun-constitutional-enforcement/src/lib.rs:131:        replay_deterministic: bool,
crates/amun-constitutional-enforcement/src/lib.rs:169:                &ConstitutionalLaw::ReplayDeterminism,
crates/amun-constitutional-enforcement/src/lib.rs:170:                replay_deterministic,
crates/amun-constitutional-enforcement/src/lib.rs:171:                "Replay divergence",
crates/amun-constitutional-enforcement/src/lib.rs:235:    fn n123_constitutional_block_accepted() {
crates/amun-constitutional-enforcement/src/lib.rs:245:    fn n123_unconstitutional_block_rejected() {
crates/amun-constitutional-enforcement/src/lib.rs:252:                assert_eq!(violations[0].law, ConstitutionalLaw::StateRootIntegrity);
crates/amun-constitutional-enforcement/src/lib.rs:259:    fn n123_multiple_violations() {
crates/amun-constitutional-enforcement/src/lib.rs:272:    fn n123_deactivated_law() {
crates/amun-constitutional-enforcement/src/lib.rs:282:    fn n123_compliance_ratio() {
crates/amun-constitutional-enforcement/src/lib.rs:300:    fn n123_verdict_history() {
crates/amun-constitutional-enforcement/src/lib.rs:63:    ReplayDeterminism,
crates/amun-constitutional-enforcement/src/lib.rs:93:            evidence.replay_deterministic,
crates/amun-constitutional-enforcement/src/proof_engine.rs:102:        replay_root: &[u8; 32],
crates/amun-constitutional-enforcement/src/proof_engine.rs:170:        // Replay Determinism
crates/amun-constitutional-enforcement/src/proof_engine.rs:171:        if !Self::verify_replay_determinism(original_root, replay_root) {
crates/amun-constitutional-enforcement/src/proof_engine.rs:173:                law: ConstitutionalLaw::ReplayDeterminism,
crates/amun-constitutional-enforcement/src/proof_engine.rs:174:                description: "Replay produced different state root".into(),
crates/amun-constitutional-enforcement/src/proof_engine.rs:1:// N124 — Constitutional Proof Engine
crates/amun-constitutional-enforcement/src/proof_engine.rs:222:    fn n124_state_root_mismatch_detected() {
crates/amun-constitutional-enforcement/src/proof_engine.rs:235:            &[0x11; 32], // replay matches
crates/amun-constitutional-enforcement/src/proof_engine.rs:254:    fn n124_constitutional_block_passes() {
crates/amun-constitutional-enforcement/src/proof_engine.rs:268:            &root, // replay matches
crates/amun-constitutional-enforcement/src/proof_engine.rs:276:        assert_eq!(verdict, ConstitutionalVerdict::Constitutional);
crates/amun-constitutional-enforcement/src/proof_engine.rs:280:    fn n124_no_supermajority_detected() {
crates/amun-constitutional-enforcement/src/proof_engine.rs:64:    /// Verify replay determinism: replaying the block produces the same state root.
crates/amun-constitutional-enforcement/src/proof_engine.rs:65:    pub fn verify_replay_determinism(original_root: &[u8; 32], replay_root: &[u8; 32]) -> bool {
crates/amun-constitutional-enforcement/src/proof_engine.rs:66:        original_root == replay_root
crates/amun-constitutional-enforcement/src/state_transition.rs:108:    let replay_root = replay_fn();
crates/amun-constitutional-enforcement/src/state_transition.rs:114:        replay_root,
crates/amun-constitutional-enforcement/src/state_transition.rs:127:    fn n126_identical_execution_and_replay_is_constitutional() {
crates/amun-constitutional-enforcement/src/state_transition.rs:135:            || post_root,          // replay matches
crates/amun-constitutional-enforcement/src/state_transition.rs:138:        assert_eq!(verdict, ConstitutionalVerdict::Constitutional);
crates/amun-constitutional-enforcement/src/state_transition.rs:142:    fn n126_replay_divergence_is_unconstitutional() {
crates/amun-constitutional-enforcement/src/state_transition.rs:145:        let wrong_replay = [0xFF; 32];
crates/amun-constitutional-enforcement/src/state_transition.rs:151:            || wrong_replay, // replay differs!
crates/amun-constitutional-enforcement/src/state_transition.rs:158:                    .any(|v| v.law == ConstitutionalLaw::ReplayDeterminism));
crates/amun-constitutional-enforcement/src/state_transition.rs:160:            _ => panic!("Expected Unconstitutional for replay divergence"),
crates/amun-constitutional-enforcement/src/state_transition.rs:165:    fn n126_supply_not_conserved_is_unconstitutional() {
crates/amun-constitutional-enforcement/src/state_transition.rs:16:    /// The replay state root (independently computed)
crates/amun-constitutional-enforcement/src/state_transition.rs:17:    pub replay_root: [u8; 32],
crates/amun-constitutional-enforcement/src/state_transition.rs:1:// N126 — Constitutional State Transition Verification
crates/amun-constitutional-enforcement/src/state_transition.rs:27:    /// N126: Create from the raw data produced by execution and replay.
crates/amun-constitutional-enforcement/src/state_transition.rs:31:        replay_root: [u8; 32],
crates/amun-constitutional-enforcement/src/state_transition.rs:38:            replay_root,
crates/amun-constitutional-enforcement/src/state_transition.rs:45:    /// N126: Verify the state transition constitutionally.
crates/amun-constitutional-enforcement/src/state_transition.rs:49:        // N126.1: State transition must change state (empty blocks are valid but rare)
crates/amun-constitutional-enforcement/src/state_transition.rs:54:        // N126.2: Replay must produce identical state root
crates/amun-constitutional-enforcement/src/state_transition.rs:55:        if self.post_state_root != self.replay_root {
crates/amun-constitutional-enforcement/src/state_transition.rs:57:                law: ConstitutionalLaw::ReplayDeterminism,
crates/amun-constitutional-enforcement/src/state_transition.rs:59:                    "Replay divergence: execution={:02x?} replay={:02x?}",
crates/amun-constitutional-enforcement/src/state_transition.rs:5:// parameter with real execution and replay verification.
crates/amun-constitutional-enforcement/src/state_transition.rs:61:                    &self.replay_root[..4]
crates/amun-constitutional-enforcement/src/state_transition.rs:87:/// N126: Execute and verify a state transition constitutionally.
crates/amun-constitutional-enforcement/src/state_transition.rs:89:/// This function takes the pre-state, executes a block, replays it,
crates/amun-constitutional-enforcement/src/state_transition.rs:93:/// `replay_fn` is the independent replay that must produce the same root.
crates/amun-constitutional-enforcement/src/state_transition.rs:99:    replay_fn: R,
crates/amun-constitutional-enforcement/src/state_transition.rs:9:/// N126: Result of verifying a state transition.
crates/amun-constitutional-evidence/src/lib.rs:107:    fn test_evidence_passes_for_valid_context() {
crates/amun-constitutional-geometry/src/curvature.rs:22:/// Replay curvature measures deformation in the causal geometry
crates/amun-constitutional-geometry/src/curvature.rs:23:/// caused by replay divergence.
crates/amun-constitutional-geometry/src/curvature.rs:25:pub struct ReplayCurvature {
crates/amun-constitutional-geometry/src/curvature.rs:31:impl ReplayCurvature {
crates/amun-constitutional-geometry/src/emergent_horizons.rs:31:    /// Replay curvature becomes infinite
crates/amun-constitutional-geometry/src/emergent_horizons.rs:32:    ReplaySingularity,
crates/amun-constitutional-geometry/src/fields.rs:41:            tensor.replay_distance,
crates/amun-constitutional-geometry/src/flow_dynamics.rs:28:    /// Attraction toward replay determinism
crates/amun-constitutional-geometry/src/flow_dynamics.rs:29:    ReplayForce { strength: f64 },
crates/amun-constitutional-geometry/src/flow_dynamics.rs:64:                ConstitutionalForce::ReplayForce { strength } => {
crates/amun-constitutional-geometry/src/flow_dynamics.rs:65:                    total[1] += strength; // Replay dimension
crates/amun-constitutional-geometry/src/horizons.rs:15:    /// Replay horizon: replay divergence has made reconciliation impossible
crates/amun-constitutional-geometry/src/horizons.rs:16:    ReplayHorizon,
crates/amun-constitutional-geometry/src/lib.rs:17:pub use curvature::{CausalCurvature, LegitimacyCurvature, ReplayCurvature};
crates/amun-constitutional-geometry/src/metric_tensor.rs:9:    /// The 7x7 metric tensor (physics, replay, lineage, temporal,
crates/amun-constitutional-geometry/src/metrics.rs:11:    /// Distance in replay space (replay guarantee compatibility)
crates/amun-constitutional-geometry/src/metrics.rs:12:    pub replay_distance: f64,
crates/amun-constitutional-geometry/src/metrics.rs:35:            replay_distance: 0.0,
crates/amun-constitutional-geometry/src/metrics.rs:47:            + self.replay_distance.powi(2)
crates/amun-constitutional-geometry/src/metrics.rs:90:        // Replay distance: can replay be preserved?
crates/amun-constitutional-geometry/src/metrics.rs:92:            tensor.replay_distance = 20.0;
crates/amun-constitutional-geometry/src/metrics.rs:99:        let requires_causal_jump = tensor.replay_distance > 30.0;
crates/amun-constitutional-geometry/src/stability.rs:49:    /// Replay determinism creates a stable attractor
crates/amun-constitutional-geometry/src/stability.rs:50:    ReplayAttractor,
crates/amun-constitutional-geometry/src/trajectories.rs:44:                        | super::horizons::HorizonType::ReplayHorizon
crates/amun-constitutional-governance/src/lib.rs:15://! model formal and replay-verifiable.
crates/amun-constitutional-governance/src/lib.rs:7://! This crate provides the building blocks for replay-safe,
crates/amun-constitutional-governance/tests/governance_tests.rs:59:    assert!(delegation::verify_delegation_chain(&chain, &root_key.verifying_key_hex()).is_ok());
crates/amun-constitutional-governance/tests/governance_tests.rs:77:    assert!(delegation::verify_delegation_chain(&chain, &root_key.verifying_key_hex()).is_err());
crates/amun-constitutional-governance/tests/governance_tests.rs:81:fn simple_majority_passes() {
crates/amun-constitutional-integration/src/bin/n47_8_production_replay.rs:132:fn load_phase_certificates(
crates/amun-constitutional-integration/src/bin/n47_8_production_replay.rs:134:) -> Result<HashMap<String, VerificationCertificate>, Box<dyn std::error::Error>> {
crates/amun-constitutional-integration/src/bin/n47_8_production_replay.rs:151:            let cert: VerificationCertificate = serde_json::from_str(&content)?;
crates/amun-constitutional-integration/src/bin/n47_8_production_replay.rs:152:            if cert.verify() {
crates/amun-constitutional-integration/src/bin/n47_8_production_replay.rs:1:use amun_constitutional_integration::ConstitutionalBridge;
crates/amun-constitutional-integration/src/bin/n47_8_production_replay.rs:24:        println!("  This replay requires real AmunChain execution data.");
crates/amun-constitutional-integration/src/bin/n47_8_production_replay.rs:33:        println!("  Skipping production replay — N47.8 deferred.");
crates/amun-constitutional-integration/src/bin/n47_8_production_replay.rs:3:use amun_verification_kernel::VerificationCertificate;
crates/amun-constitutional-integration/src/bin/n47_8_production_replay.rs:56:        ConstitutionalBridge::run_full_pipeline(phase_certificates, timestamp)
crates/amun-constitutional-integration/src/bin/n47_8_production_replay.rs:8:fn main() -> Result<(), Box<dyn std::error::Error>> {
crates/amun-constitutional-integration/src/bin/n47_8_production_replay.rs:95:        "N47-Constitutional-Authority".into(),
crates/amun-constitutional-integration/src/lib.rs:162:                "Replay results must match finalized state",
crates/amun-constitutional-integration/src/lib.rs:163:                "forall b in Finalized : state_root(Replay(b)) = state_root(b)",
crates/amun-constitutional-integration/src/lib.rs:167:            .with_dependency(ObligationId::new(ObligationNamespace::Replay, 2))
crates/amun-constitutional-integration/src/lib.rs:174:                ObligationId::new(ObligationNamespace::Replay, 3),
crates/amun-constitutional-integration/src/lib.rs:176:                "Replay certificates are unique",
crates/amun-constitutional-integration/src/lib.rs:181:            .with_dependency(ObligationId::new(ObligationNamespace::Replay, 2)),
crates/amun-constitutional-integration/src/lib.rs:187:                ObligationId::new(ObligationNamespace::Replay, 4),
crates/amun-constitutional-integration/src/lib.rs:189:                "Replay chain is continuous",
crates/amun-constitutional-integration/src/lib.rs:190:                "forall b1,b2 in ReplayChain : b1.height+1=b2.height implies linked(b1,b2)",
crates/amun-constitutional-integration/src/lib.rs:194:            .with_dependency(ObligationId::new(ObligationNamespace::Replay, 1)),
crates/amun-constitutional-integration/src/lib.rs:242:                "Finalized = Replay Verified = Evidence Certified",
crates/amun-constitutional-integration/src/lib.rs:243:                "|Finalized| = |ReplayVerified| = |EvidenceCertified|",
crates/amun-constitutional-integration/src/lib.rs:248:            .with_dependency(ObligationId::new(ObligationNamespace::Replay, 1))
crates/amun-constitutional-integration/src/lib.rs:449:    fn n47_7_build_obligation_registry() {
crates/amun-constitutional-integration/src/lib.rs:456:    fn n47_7_run_full_pipeline() {
crates/amun-constitutional-integration/src/lib.rs:48:            ObligationId::new(ObligationNamespace::Replay, 1),
crates/amun-constitutional-integration/src/lib.rs:50:            "Every finalized block has replay evidence",
crates/amun-constitutional-integration/src/lib.rs:51:            "forall b in Finalized : exists replay_certificate(b)",
crates/amun-constitutional-integration/src/lib.rs:58:            ObligationId::new(ObligationNamespace::Replay, 2),
crates/amun-constitutional-integration/src/lib.rs:60:            "Replay execution is deterministic",
crates/amun-constitutional-integration/src/lib.rs:61:            "forall b : Replay(b, t1) = Replay(b, t2)",
crates/amun-constitutional-integration/src/lib.rs:70:            "Every replay certificate maps to evidence",
crates/amun-constitutional-integration/src/lib.rs:71:            "forall rc in ReplayCertificate : exists ev in EvidenceRoot : ev.replay = rc",
crates/amun-constitutional-kernel/src/lib.rs:14://! - Replay-safe receipts: every transition leaves a verifiable proof.
crates/amun-constitutional-kernel/tests/kernel_tests.rs:67:    // Replay must produce identical result
crates/amun-constitutional-proof/src/evidence_type.rs:7:    ExperimentalEvidence,
crates/amun-constitutional-proof/src/evidence_type.rs:8:    #[serde(rename = "replay")]
crates/amun-constitutional-proof/src/evidence_type.rs:9:    ReplayEvidence,
crates/amun-constitutional-proof/src/lib.rs:1002:    fn n47_3_s0_evidence_with_reproducibility() {
crates/amun-constitutional-proof/src/lib.rs:100:    fn n47_1_s0_serialize_safety_001() {
crates/amun-constitutional-proof/src/lib.rs:1024:    fn n47_3_s0_evidence_with_lineage() {
crates/amun-constitutional-proof/src/lib.rs:1042:        assert_eq!(ev.status, EvidenceStatus::Verified);
crates/amun-constitutional-proof/src/lib.rs:1047:    fn n47_3_s0_serialization_roundtrip() {
crates/amun-constitutional-proof/src/lib.rs:1074:    // --- N47.3-S1: EvidenceArchive tests ---
crates/amun-constitutional-proof/src/lib.rs:107:    fn n47_1_s0_deserialize_safety_001() {
crates/amun-constitutional-proof/src/lib.rs:1094:    fn n47_3_s1_insert_and_retrieve() {
crates/amun-constitutional-proof/src/lib.rs:1096:        let obl = ObligationId::new(ObligationNamespace::Replay, 1);
crates/amun-constitutional-proof/src/lib.rs:1097:        let ev = make_evidence("EV-ARC-1", "N42", EvidenceType::ReplayEvidence, obl);
crates/amun-constitutional-proof/src/lib.rs:1105:    fn n47_3_s1_reject_duplicate() {
crates/amun-constitutional-proof/src/lib.rs:1108:        let ev = make_evidence("EV-DUP", "N42", EvidenceType::AuditEvidence, obl);
crates/amun-constitutional-proof/src/lib.rs:110:        assert_eq!(id.namespace(), ObligationNamespace::Safety);
crates/amun-constitutional-proof/src/lib.rs:1114:    fn n47_3_s1_verify_and_archive() {
crates/amun-constitutional-proof/src/lib.rs:1117:        let ev = make_evidence("EV-LIFECYCLE", "N41", EvidenceType::ConsensusEvidence, obl);
crates/amun-constitutional-proof/src/lib.rs:1134:    fn n47_3_s1_cannot_archive_unverified() {
crates/amun-constitutional-proof/src/lib.rs:1141:                EvidenceType::ReplayEvidence,
crates/amun-constitutional-proof/src/lib.rs:1149:    fn n47_3_s1_reject_is_permanent() {
crates/amun-constitutional-proof/src/lib.rs:115:    fn n47_1_s0_parse_safety_001() {
crates/amun-constitutional-proof/src/lib.rs:1165:        assert!(archive.verify("EV-REJ").is_err());
crates/amun-constitutional-proof/src/lib.rs:1171:    fn n47_3_s1_lineage_integrity() {
crates/amun-constitutional-proof/src/lib.rs:117:        assert_eq!(id, ObligationId::new(ObligationNamespace::Safety, 1));
crates/amun-constitutional-proof/src/lib.rs:1205:    fn n47_3_s1_reject_lineage_hash_mismatch() {
crates/amun-constitutional-proof/src/lib.rs:121:    fn n47_1_s0_namespace_extraction() {
crates/amun-constitutional-proof/src/lib.rs:122:        let id = ObligationId::new(ObligationNamespace::Replay, 4);
crates/amun-constitutional-proof/src/lib.rs:1239:    fn n47_3_s1_query_by_obligation() {
crates/amun-constitutional-proof/src/lib.rs:123:        assert_eq!(id.namespace(), ObligationNamespace::Replay);
crates/amun-constitutional-proof/src/lib.rs:1256:                EvidenceType::ReplayEvidence,
crates/amun-constitutional-proof/src/lib.rs:1274:    fn n47_3_s1_query_by_phase() {
crates/amun-constitutional-proof/src/lib.rs:128:    fn n47_1_s0_reject_invalid_namespace() {
crates/amun-constitutional-proof/src/lib.rs:1298:                EvidenceType::ReplayEvidence,
crates/amun-constitutional-proof/src/lib.rs:1309:    fn n47_3_s1_admissibility_rules() {
crates/amun-constitutional-proof/src/lib.rs:1310:        let obl = ObligationId::new(ObligationNamespace::Replay, 1);
crates/amun-constitutional-proof/src/lib.rs:1313:            make_evidence("EV-ADM-1", "N42", EvidenceType::ReplayEvidence, obl.clone());
crates/amun-constitutional-proof/src/lib.rs:1315:        assert!(!EvidenceArchive::is_admissible(&collected));
crates/amun-constitutional-proof/src/lib.rs:1318:            make_evidence("EV-ADM-2", "N42", EvidenceType::ReplayEvidence, obl.clone());
crates/amun-constitutional-proof/src/lib.rs:1320:        assert!(EvidenceArchive::is_admissible(&verified));
crates/amun-constitutional-proof/src/lib.rs:1323:            make_evidence("EV-ADM-3", "N42", EvidenceType::ReplayEvidence, obl.clone());
crates/amun-constitutional-proof/src/lib.rs:1325:        assert!(EvidenceArchive::is_admissible(&archived));
crates/amun-constitutional-proof/src/lib.rs:1327:        let mut rejected = make_evidence("EV-ADM-4", "N42", EvidenceType::ReplayEvidence, obl);
crates/amun-constitutional-proof/src/lib.rs:1329:        assert!(!EvidenceArchive::is_admissible(&rejected));
crates/amun-constitutional-proof/src/lib.rs:1335:    fn n47_3_cert_issue() {
crates/amun-constitutional-proof/src/lib.rs:1355:    fn n47_3_cert_reject_empty_archive() {
crates/amun-constitutional-proof/src/lib.rs:1357:        assert!(ArticleIIICertificate::issue(&archive, 1000).is_none());
crates/amun-constitutional-proof/src/lib.rs:1363:    fn n47_4_generate_markdown_report() {
crates/amun-constitutional-proof/src/lib.rs:138:    fn n47_1_s0_reject_invalid_format() {
crates/amun-constitutional-proof/src/lib.rs:1415:        assert!(md.contains("N47 Constitutional Validation Report"));
crates/amun-constitutional-proof/src/lib.rs:1428:    fn n47_5_create_publication_package() {
crates/amun-constitutional-proof/src/lib.rs:142:            Err(RegistryError::InvalidObligationIdFormat(s)) => assert_eq!(s, "SAFETY"),
crates/amun-constitutional-proof/src/lib.rs:1476:        assert!(pkg.verify());
crates/amun-constitutional-proof/src/lib.rs:1483:            "N47-Constitutional-Authority".into(),
crates/amun-constitutional-proof/src/lib.rs:148:    fn n47_1_s0_namespace_display_roundtrip() {
crates/amun-constitutional-proof/src/lib.rs:1493:    fn n47_5_package_verification() {
crates/amun-constitutional-proof/src/lib.rs:1496:            ObligationId::new(ObligationNamespace::Replay, 1),
crates/amun-constitutional-proof/src/lib.rs:151:            ObligationNamespace::Replay,
crates/amun-constitutional-proof/src/lib.rs:1522:        assert!(pkg.verify());
crates/amun-constitutional-proof/src/lib.rs:1526:    // --- N47.6: Constitutional Certification tests ---
crates/amun-constitutional-proof/src/lib.rs:1529:    fn n47_6_certify_pass() {
crates/amun-constitutional-proof/src/lib.rs:1631:            "N47-Constitutional-Authority".into(),
crates/amun-constitutional-proof/src/lib.rs:1640:    fn n47_6_certify_fail_missing_obligations() {
crates/amun-constitutional-proof/src/lib.rs:166:    fn n47_1_s0_id_serialization_roundtrip() {
crates/amun-constitutional-proof/src/lib.rs:1725:            "N47-Constitutional-Authority".into(),
crates/amun-constitutional-proof/src/lib.rs:1734:    fn n47_6_certify_fail_missing_phase_verdict() {
crates/amun-constitutional-proof/src/lib.rs:1768:            "N47-Constitutional-Authority".into(),
crates/amun-constitutional-proof/src/lib.rs:176:    fn n47_1_s1_create_primary_obligation() {
crates/amun-constitutional-proof/src/lib.rs:187:        assert_eq!(obl.kind, ObligationKind::Primary);
crates/amun-constitutional-proof/src/lib.rs:188:        assert_eq!(obl.severity, ObligationSeverity::Critical);
crates/amun-constitutional-proof/src/lib.rs:191:        assert_eq!(obl.status, ObligationStatus::Active);
crates/amun-constitutional-proof/src/lib.rs:196:    fn n47_1_s1_create_derived_with_deps() {
crates/amun-constitutional-proof/src/lib.rs:202:            "|Finalized| = |ReplayVerified| = |EvidenceCertified|",
crates/amun-constitutional-proof/src/lib.rs:209:        assert_eq!(obl.kind, ObligationKind::Derived);
crates/amun-constitutional-proof/src/lib.rs:215:    fn n47_1_s1_status_builder() {
crates/amun-constitutional-proof/src/lib.rs:226:        assert_eq!(obl.status, ObligationStatus::Frozen);
crates/amun-constitutional-proof/src/lib.rs:230:    fn n47_1_s1_serialization_roundtrip() {
crates/amun-constitutional-proof/src/lib.rs:232:            ObligationId::new(ObligationNamespace::Replay, 2),
crates/amun-constitutional-proof/src/lib.rs:234:            "Replay determinism",
crates/amun-constitutional-proof/src/lib.rs:235:            "forall b : Replay(b, t1) = Replay(b, t2)",
crates/amun-constitutional-proof/src/lib.rs:239:        .with_dependency(ObligationId::new(ObligationNamespace::Replay, 1));
crates/amun-constitutional-proof/src/lib.rs:247:    fn n47_1_s1_deserialize_minimal() {
crates/amun-constitutional-proof/src/lib.rs:270:    fn n47_1_s2_add_dependency() {
crates/amun-constitutional-proof/src/lib.rs:279:    fn n47_1_s2_detect_cycle() {
crates/amun-constitutional-proof/src/lib.rs:292:    fn n47_1_s2_topological_sort() {
crates/amun-constitutional-proof/src/lib.rs:308:    fn n47_1_s2_missing_dependency() {
crates/amun-constitutional-proof/src/lib.rs:315:    fn n47_1_s2_derived_terminates_in_primary() {
crates/amun-constitutional-proof/src/lib.rs:325:        assert!(graph.validate_derived_terminate_in_primary(&kinds).is_ok());
crates/amun-constitutional-proof/src/lib.rs:329:    fn n47_1_s2_reject_infinite_derivation() {
crates/amun-constitutional-proof/src/lib.rs:364:    fn n47_1_s3_register_obligation() {
crates/amun-constitutional-proof/src/lib.rs:373:    fn n47_1_s3_reject_duplicate() {
crates/amun-constitutional-proof/src/lib.rs:382:    fn n47_1_s3_freeze_and_reject_modification() {
crates/amun-constitutional-proof/src/lib.rs:393:    fn n47_1_s3_reject_missing_dependency() {
crates/amun-constitutional-proof/src/lib.rs:395:        let missing_dep = make_id(ObligationNamespace::Replay, 99);
crates/amun-constitutional-proof/src/lib.rs:413:    fn n47_1_s3_query_by_severity() {
crates/amun-constitutional-proof/src/lib.rs:435:        assert_eq!(reg.by_severity(ObligationSeverity::Critical).len(), 1);
crates/amun-constitutional-proof/src/lib.rs:436:        assert_eq!(reg.by_severity(ObligationSeverity::Minor).len(), 1);
crates/amun-constitutional-proof/src/lib.rs:437:        assert_eq!(reg.by_severity(ObligationSeverity::Major).len(), 0);
crates/amun-constitutional-proof/src/lib.rs:441:    fn n47_1_s3_query_by_phase() {
crates/amun-constitutional-proof/src/lib.rs:469:    fn n47_1_cert_issue_success() {
crates/amun-constitutional-proof/src/lib.rs:496:    fn n47_1_cert_reject_unfrozen() {
crates/amun-constitutional-proof/src/lib.rs:510:        assert!(ArticleICertificate::issue(&reg, 1000).is_none());
crates/amun-constitutional-proof/src/lib.rs:514:    fn n47_1_cert_reject_insufficient_obligations() {
crates/amun-constitutional-proof/src/lib.rs:529:        assert!(ArticleICertificate::issue(&reg, 1000).is_none());
crates/amun-constitutional-proof/src/lib.rs:535:    fn n47_2_s0_create_satisfied_result() {
crates/amun-constitutional-proof/src/lib.rs:539:        assert_eq!(result.status, ObligationResultStatus::Satisfied);
crates/amun-constitutional-proof/src/lib.rs:545:    fn n47_2_s0_create_failed_result() {
crates/amun-constitutional-proof/src/lib.rs:546:        let id = ObligationId::new(ObligationNamespace::Replay, 1);
crates/amun-constitutional-proof/src/lib.rs:547:        let reason = FailureReason::new("MISSING_EVIDENCE", "No replay data found");
crates/amun-constitutional-proof/src/lib.rs:549:        assert_eq!(result.status, ObligationResultStatus::Failed);
crates/amun-constitutional-proof/src/lib.rs:554:    fn n47_2_s0_create_inconclusive_result() {
crates/amun-constitutional-proof/src/lib.rs:557:        assert_eq!(result.status, ObligationResultStatus::Inconclusive);
crates/amun-constitutional-proof/src/lib.rs:562:    fn n47_2_s0_verdict_pass() {
crates/amun-constitutional-proof/src/lib.rs:567:    fn n47_2_s0_verdict_fail_with_reasons() {
crates/amun-constitutional-proof/src/lib.rs:577:    fn n47_2_s0_serialize_obligation_result() {
crates/amun-constitutional-proof/src/lib.rs:585:    // --- N47.2-S1: ConstitutionalVerdict tests ---
crates/amun-constitutional-proof/src/lib.rs:588:    fn n47_2_s1_create_constitutional_verdict() {
crates/amun-constitutional-proof/src/lib.rs:601:            "N41-Finality".into(),
crates/amun-constitutional-proof/src/lib.rs:612:        assert!(verdict.verify());
crates/amun-constitutional-proof/src/lib.rs:617:    fn n47_2_s1_count_satisfied_obligations() {
crates/amun-constitutional-proof/src/lib.rs:620:                ObligationId::new(ObligationNamespace::Replay, 1),
crates/amun-constitutional-proof/src/lib.rs:624:                ObligationId::new(ObligationNamespace::Replay, 2),
crates/amun-constitutional-proof/src/lib.rs:629:                ObligationId::new(ObligationNamespace::Replay, 3),
crates/amun-constitutional-proof/src/lib.rs:639:            VerdictResult::ConditionalPass(vec!["Replay-2 failed".into()]),
crates/amun-constitutional-proof/src/lib.rs:649:    fn n47_2_s1_collect_evidence_refs() {
crates/amun-constitutional-proof/src/lib.rs:676:    fn n47_2_s1_compute_verdict_hash() {
crates/amun-constitutional-proof/src/lib.rs:705:    fn n47_2_s1_serialization_roundtrip() {
crates/amun-constitutional-proof/src/lib.rs:730:        assert!(parsed.verify());
crates/amun-constitutional-proof/src/lib.rs:747:    fn n47_2_s2_fail_on_critical() {
crates/amun-constitutional-proof/src/lib.rs:771:    fn n47_2_s2_fail_on_two_major() {
crates/amun-constitutional-proof/src/lib.rs:773:            ObligationId::new(ObligationNamespace::Replay, 1),
crates/amun-constitutional-proof/src/lib.rs:777:            ObligationId::new(ObligationNamespace::Replay, 2),
crates/amun-constitutional-proof/src/lib.rs:799:    fn n47_2_s2_conditional_pass_on_one_major() {
crates/amun-constitutional-proof/src/lib.rs:826:    fn n47_2_s2_pass_with_minor_failures() {
crates/amun-constitutional-proof/src/lib.rs:853:    fn n47_2_s2_pass_with_advisory_failures() {
crates/amun-constitutional-proof/src/lib.rs:880:    fn n47_2_s2_pass_all_satisfied() {
crates/amun-constitutional-proof/src/lib.rs:903:    fn n47_2_s2_count_obligations_correctly() {
crates/amun-constitutional-proof/src/lib.rs:938:    fn n47_2_s2_waived_advisory_does_not_fail() {
crates/amun-constitutional-proof/src/lib.rs:94:    fn n47_1_s0_display_safety_001() {
crates/amun-constitutional-proof/src/lib.rs:959:    fn n47_2_s2_not_applicable_does_not_fail() {
crates/amun-constitutional-proof/src/lib.rs:979:    // --- N47.3-S0: Evidence Foundation Types tests ---
crates/amun-constitutional-proof/src/lib.rs:982:    fn n47_3_s0_create_evidence_record() {
crates/amun-constitutional-proof/src/lib.rs:983:        let id = ObligationId::new(ObligationNamespace::Replay, 1);
crates/amun-constitutional-proof/src/lib.rs:986:            EvidenceType::ReplayEvidence,
crates/amun-constitutional-proof/src/lib.rs:987:            "amun-replay-engine".into(),
crates/amun-constitutional-proof/src/lib.rs:994:        assert_eq!(ev.status, EvidenceStatus::Collected);
crates/amun-constitutional-proof/src/obligation_namespace.rs:10:    Replay,
crates/amun-constitutional-proof/src/obligation_namespace.rs:29:            Self::Replay => "REPLAY",
crates/amun-constitutional-proof/src/obligation_namespace.rs:47:            "REPLAY" => Ok(Self::Replay),
crates/amun-constitutional-proof/src/obligation_namespace.rs:9:    #[serde(rename = "replay")]
crates/amun-constitutional-proof/src/publication_package.rs:7:/// required to independently verify the N47 results.
crates/amun-constitutional-proof/src/report_generator.rs:32:            report_type: "N47 Constitutional Validation".into(),
crates/amun-constitutional-proof/src/report_generator.rs:44:        md.push_str("# N47 Constitutional Validation Report\n\n");
crates/amun-constitutional-quarantine/src/levels.rs:25:    pub replay_verified: bool,
crates/amun-constitutional-quarantine/src/pipeline.rs:34:    pub fn verify_physics(&mut self, snapshot_root: [u8; 32], passed: bool) {
crates/amun-constitutional-quarantine/src/pipeline.rs:38:            replay_verified: false,
crates/amun-constitutional-quarantine/src/pipeline.rs:47:    pub fn verify_replay(&mut self, snapshot_root: [u8; 32], passed: bool) {
crates/amun-constitutional-quarantine/src/pipeline.rs:54:            record.replay_verified = passed;
crates/amun-constitutional-quarantine/src/pipeline.rs:61:    pub fn verify_lineage(&mut self, snapshot_root: [u8; 32], passed: bool) {
crates/amun-constitutional-quarantine/src/rehabilitation.rs:15:    ReplayContinuityVerification,
crates/amun-constitutional-quarantine/src/rehabilitation.rs:28:                RehabilitationStep::ReplayContinuityVerification,
crates/amun-constitutional-quarantine/src/rehabilitation.rs:32:                RehabilitationStep::ReplayContinuityVerification,
crates/amun-constitutional-quarantine/src/rehabilitation.rs:37:                RehabilitationStep::ReplayContinuityVerification,
crates/amun-constitutional-quarantine/src/rehabilitation.rs:43:                RehabilitationStep::ReplayContinuityVerification,
crates/amun-constitutional-runtime/src/block_validator.rs:101:    fn n51_valid_block_all_transactions_pass() {
crates/amun-constitutional-runtime/src/block_validator.rs:129:        assert_eq!(result.committed, 5);
crates/amun-constitutional-runtime/src/block_validator.rs:137:    fn n51_block_invalid_if_any_transaction_rejected() {
crates/amun-constitutional-runtime/src/block_validator.rs:178:        assert_eq!(result.committed, 5);
crates/amun-constitutional-runtime/src/block_validator.rs:182:    fn n51_block_state_root_consistent() {
crates/amun-constitutional-runtime/src/block_validator.rs:212:        assert_eq!(result.state_root, registry.compute_state_root());
crates/amun-constitutional-runtime/src/block_validator.rs:22:/// Validates that every transaction in a block passes PCCV.
crates/amun-constitutional-runtime/src/block_validator.rs:28:    /// The block is valid iff ALL transactions pass PCCV.
crates/amun-constitutional-runtime/src/certificate_chain.rs:152:    fn n53_create_certificate_chain() {
crates/amun-constitutional-runtime/src/certificate_chain.rs:158:        assert!(chain.verify_chain());
crates/amun-constitutional-runtime/src/certificate_chain.rs:162:    fn n53_chain_deterministic_root() {
crates/amun-constitutional-runtime/src/certificate_chain.rs:173:    fn n53_chain_rejects_invalid_certificate() {
crates/amun-constitutional-runtime/src/certificate_chain.rs:181:    fn n53_chain_tampering_detected() {
crates/amun-constitutional-runtime/src/certificate_chain.rs:186:        assert!(!chain.verify_chain());
crates/amun-constitutional-runtime/src/certificate_chain.rs:190:    fn n53a_broken_chain_detected() {
crates/amun-constitutional-runtime/src/certificate_chain.rs:198:    fn n53a_replaced_certificate_detected() {
crates/amun-constitutional-runtime/src/certificate_chain.rs:210:        assert!(!chain.verify_chain());
crates/amun-constitutional-runtime/src/finality_certificate.rs:150:    fn n52_issue_finality_certificate() {
crates/amun-constitutional-runtime/src/finality_certificate.rs:193:        assert!(cert.verify());
crates/amun-constitutional-runtime/src/finality_certificate.rs:198:    fn n52_certificate_deterministic() {
crates/amun-constitutional-runtime/src/finality_certificate.rs:214:    fn n52_certificate_detects_tampering() {
crates/amun-constitutional-runtime/src/finality_certificate.rs:227:        assert!(!cert.verify());
crates/amun-constitutional-runtime/src/history_root.rs:144:    fn n54_compute_history_root() {
crates/amun-constitutional-runtime/src/history_root.rs:160:    fn n54_history_root_deterministic() {
crates/amun-constitutional-runtime/src/history_root.rs:167:        let root1 = ConstitutionalHistoryRoot::from_chain(&chain1);
crates/amun-constitutional-runtime/src/history_root.rs:168:        let root2 = ConstitutionalHistoryRoot::from_chain(&chain2);
crates/amun-constitutional-runtime/src/history_root.rs:174:    fn n54_verify_chain_against_root() {
crates/amun-constitutional-runtime/src/history_root.rs:181:        assert!(root.verify_chain(&chain));
crates/amun-constitutional-runtime/src/history_root.rs:185:    fn n54_reject_tampered_chain() {
crates/amun-constitutional-runtime/src/history_root.rs:196:        assert!(!root.verify_chain(&tampered_chain));
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:161:        // ── N50: Integrated PCCV verification ─────────────────
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:239:    fn n50_execute_with_pccv_integration() {
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:269:                assert!(pccv_verified, "PCCV must pass for valid execution");
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:276:    fn n50_pccv_rejects_illegal_execution() {
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:280:        // so PCCV passes trivially. Full illegal execution testing
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:315:    fn n50_pccv_rejection_preserves_state() {
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:97:        let passed = VMKernel::verify(&mut buffer, registry);
crates/amun-constitutional-semantics/src/lib.rs:113:    fn mk(seq: u64, hash: [u8; 32], parent: [u8; 32]) -> TranscriptEntry { TranscriptEntry::Consensus(ConsensusEvent { identity: EventIdentity::new(hash, parent, [0xAA; 32], seq, ReplayDomain::Consensus, [0xBB; 32]), round: seq, event_type: ConsensusEventType::Proposal, authority: EventAuthority::Authoritative }) }
crates/amun-constitutional-semantics/src/lib.rs:114:    #[test] fn test_commitment_verification() { let e = vec![mk(1,[0x01;32],[0x00;32]), mk(2,[0x02;32],[0x01;32])]; let c = TranscriptCommitment::new_sequential(&e, [0xBB;32]); assert!(c.verify_events(&e)); }
crates/amun-constitutional-semantics/src/lib.rs:115:    #[test] fn test_commitment_detects_tamper() { let e1 = mk(1,[0x01;32],[0x00;32]); let e2 = mk(2,[0x02;32],[0x01;32]); let e3 = mk(3,[0x03;32],[0x02;32]); let c = TranscriptCommitment::new_sequential(&vec![e1.clone(), e2], [0xBB;32]); assert!(!c.verify_events(&vec![e1, e3])); }
crates/amun-constitutional-semantics/src/lib.rs:116:    #[test] fn test_finality_progression() { assert!(EventFinality::Tentative < EventFinality::Finalized); assert!(EventFinality::Finalized < EventFinality::ReplayCertified); }
crates/amun-constitutional-semantics/src/lib.rs:117:    #[test] fn test_finality_replay_safety() { assert!(!EventFinality::Tentative.is_replay_safe()); assert!(EventFinality::Finalized.is_replay_safe()); }
crates/amun-constitutional-semantics/src/lib.rs:118:    #[test] fn test_witness_normalization_deterministic() { let w = vec![(ReplayDomain::Consensus,2,[0x02;32]),(ReplayDomain::Consensus,1,[0x01;32])]; assert_eq!(WitnessNormalization::normalize(&w).normalization_root, WitnessNormalization::normalize(&w).normalization_root); }
crates/amun-constitutional-semantics/src/lib.rs:120:    #[test] fn test_replay_policy() { assert!(ReplayPolicy::CONSENSUS_AUTHORITATIVE.replay_required); assert!(!ReplayPolicy::EPHEMERAL.replay_required); }
crates/amun-constitutional-semantics/src/lib.rs:121:    #[test] fn test_authority_binding() { let b = AuthorityBinding { authority: EventAuthority::Authoritative, authority_set_root: [0xAA;32], authority_epoch: [0xBB;32], authority_proof: AuthorityProof::SingleSignature { validator_id: 1, signature: [0;64] } }; assert!(b.verify_binding(EventAuthority::Authoritative)); assert!(!b.verify_binding(EventAuthority::Derived)); }
crates/amun-constitutional-semantics/src/lib.rs:1://! Constitutional Semantics — complete truth model for replay verification.
crates/amun-constitutional-semantics/src/lib.rs:23:    pub fn verify_boundary(&self, boundary: &ReplayBoundary) -> bool { self.start_sequence >= boundary.finalized_sequence }
crates/amun-constitutional-semantics/src/lib.rs:27:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)] pub enum EventFinality { Tentative, QuorumAccepted, Finalized, ReplayCertified }
crates/amun-constitutional-semantics/src/lib.rs:29:    pub fn is_replay_safe(&self) -> bool { matches!(self, EventFinality::Finalized | EventFinality::ReplayCertified) }
crates/amun-constitutional-semantics/src/lib.rs:30:    pub fn is_immutable(&self) -> bool { matches!(self, EventFinality::Finalized | EventFinality::ReplayCertified) }
crates/amun-constitutional-semantics/src/lib.rs:36:    pub fn with_replay_certification(mut self, at_sequence: u64) -> Self { self.finality = EventFinality::ReplayCertified; self.finalized_at_sequence = Some(at_sequence); self }
crates/amun-constitutional-semantics/src/lib.rs:48:#[derive(Debug, Clone, PartialEq, Eq)] pub struct NormalizedWitness { pub domain: ReplayDomain, pub sequence: u64, pub witness_hash: [u8; 32] }
crates/amun-constitutional-semantics/src/lib.rs:51:    pub fn normalize(witnesses: &[(ReplayDomain, u64, [u8; 32])]) -> Self {
crates/amun-constitutional-semantics/src/lib.rs:58:    pub fn verify_normalization(&self, witnesses: &[(ReplayDomain, u64, [u8; 32])]) -> bool { Self::normalize(witnesses).normalization_root == self.normalization_root }
crates/amun-constitutional-semantics/src/lib.rs:61:// ─── Replay Policy ─────────────────────────────────────────
crates/amun-constitutional-semantics/src/lib.rs:62:#[derive(Debug, Clone, PartialEq, Eq)] pub struct ReplayPolicy { pub authority: EventAuthority, pub replay_required: bool, pub divergence_is_violation: bool, pub contributes_to_causality: bool, pub can_be_checkpoint: bool, pub requires_certification: bool }
crates/amun-constitutional-semantics/src/lib.rs:63:impl ReplayPolicy {
crates/amun-constitutional-semantics/src/lib.rs:6:use amun_replay_semantics::{ReplayDomain, ReplayBoundary, ReplayFailure};
crates/amun-constitutional-semantics/src/lib.rs:70:            self.replay_required as u8,
crates/amun-constitutional-semantics/src/lib.rs:86:    pub const CONSENSUS_AUTHORITATIVE: Self = Self { authority: EventAuthority::Authoritative, replay_required: true, divergence_is_violation: true, contributes_to_causality: true, can_be_checkpoint: true, requires_certification: true };
crates/amun-constitutional-semantics/src/lib.rs:87:    pub const DERIVED: Self = Self { authority: EventAuthority::Derived, replay_required: false, divergence_is_violation: false, contributes_to_causality: true, can_be_checkpoint: false, requires_certification: false };
crates/amun-constitutional-semantics/src/lib.rs:88:    pub const EPHEMERAL: Self = Self { authority: EventAuthority::Ephemeral, replay_required: false, divergence_is_violation: false, contributes_to_causality: false, can_be_checkpoint: false, requires_certification: false };
crates/amun-constitutional-semantics/src/lib.rs:89:    pub const CERTIFYING: Self = Self { authority: EventAuthority::Certifying, replay_required: true, divergence_is_violation: true, contributes_to_causality: true, can_be_checkpoint: true, requires_certification: true };
crates/amun-constitutional-semantics/src/lib.rs:99:    pub fn transcript_continuity(events: &[TranscriptEntry]) -> Result<ContinuityResult, ReplayFailure> {
crates/amun-constitutional-signing/tests/signing_tests.rs:15:    assert!(signed.verify().is_ok());
crates/amun-constitutional-signing/tests/signing_tests.rs:32:    assert!(signed.verify().is_ok());
crates/amun-constitutional-signing/tests/signing_tests.rs:44:    assert!(signed.verify().is_ok());
crates/amun-constitutional-signing/tests/signing_tests.rs:65:    assert!(signed.verify().is_ok());
crates/amun-constitutional-signing/tests/signing_tests.rs:74:    assert!(tampered_signed.verify().is_err());
crates/amun-constitutional-sim/src/protocol.rs:4:pub struct ExperimentalProtocol {
crates/amun-constitutional-sim/src/runner.rs:22:    pub fn run(protocol: &ExperimentalProtocol) -> Vec<SimulationStep> {
crates/amun-constitutional-sim/tests/recalibrated_n100.rs:4:fn test_n100_validation_recalibrated() {
crates/amun-constitutional-sim/tests/recognition_erosion.rs:49:fn test_saturation_ratio_scan_n100() {
crates/amun-constitutional-sim/tests/recognition_erosion.rs:4:fn test_saturation_at_n100() {
crates/amun-constitutional-state/src/lib.rs:13:/// A record of a state transition for deterministic replay.
crates/amun-constitutional-state/src/lib.rs:148:        assert!(key.starts_with(b"transition/"));
crates/amun-constitutional-state/src/lib.rs:169:    fn test_replay_produces_same_root() {
crates/amun-constitutional-state/src/lib.rs:175:        let replayed = ConstitutionalStateRuntime::replay(original.journal());
crates/amun-constitutional-state/src/lib.rs:176:        let root2 = replayed.state_root();
crates/amun-constitutional-state/src/lib.rs:181:    fn test_replay_order_independence() {
crates/amun-constitutional-state/src/lib.rs:189:        let rt2 = ConstitutionalStateRuntime::replay(&reversed);
crates/amun-constitutional-state/src/lib.rs:197:pub struct ReplayCertificate {
crates/amun-constitutional-state/src/lib.rs:218:    /// Create a replay certificate for a block transition.
crates/amun-constitutional-state/src/lib.rs:223:    ) -> ReplayCertificate {
crates/amun-constitutional-state/src/lib.rs:224:        ReplayCertificate {
crates/amun-constitutional-state/src/lib.rs:234:impl ReplayCertificate {
crates/amun-constitutional-state/src/lib.rs:235:    /// Verify that replaying the journal produces the claimed post_state_root.
crates/amun-constitutional-state/src/lib.rs:237:        let replayed = ConstitutionalStateRuntime::replay(records);
crates/amun-constitutional-state/src/lib.rs:238:        replayed.state_root() == self.post_state_root
crates/amun-constitutional-state/src/lib.rs:239:            && replayed.journal_root() == self.journal_root
crates/amun-constitutional-state/src/lib.rs:249:    fn test_replay_certificate_valid() {
crates/amun-constitutional-state/src/lib.rs:256:        assert!(cert.verify(rt.journal()));
crates/amun-constitutional-state/src/lib.rs:260:    fn test_replay_certificate_detects_tampering() {
crates/amun-constitutional-state/src/lib.rs:269:        assert!(!cert.verify(&tampered));
crates/amun-constitutional-state/src/lib.rs:277:impl ReplayCertificate {
crates/amun-constitutional-state/src/lib.rs:306:    fn n1_hash_deterministic() {
crates/amun-constitutional-state/src/lib.rs:315:    fn n1_hash_sensitive_to_height() {
crates/amun-constitutional-state/src/lib.rs:324:    fn n1_hash_sensitive_to_state() {
crates/amun-constitutional-state/src/lib.rs:335:    fn n1_hash_sensitive_to_pre_state() {
crates/amun-constitutional-state/src/lib.rs:344:    fn n1_hash_sensitive_to_journal() {
crates/amun-constitutional-state/src/lib.rs:357:    /// Build a Merkle root from multiple ReplayCertificates.
crates/amun-constitutional-state/src/lib.rs:359:    pub fn certificate_merkle_root(certificates: &[ReplayCertificate]) -> [u8; 32] {
crates/amun-constitutional-state/src/lib.rs:374:    fn n3_single_certificate_merkle_root() {
crates/amun-constitutional-state/src/lib.rs:386:    fn n3_multiple_certificates_different_root() {
crates/amun-constitutional-state/src/lib.rs:404:/// A Merkle proof that a specific ReplayCertificate is included
crates/amun-constitutional-state/src/lib.rs:418:        certificates: &[ReplayCertificate],
crates/amun-constitutional-state/src/lib.rs:457:    fn n7_inclusion_proof_valid() {
crates/amun-constitutional-state/src/lib.rs:472:        assert!(proof.verify());
crates/amun-constitutional-state/src/lib.rs:476:    fn n7_inclusion_proof_wrong_certificate_fails() {
crates/amun-constitutional-state/src/lib.rs:492:        assert!(!proof.verify());
crates/amun-constitutional-state/src/lib.rs:496:    fn n7_inclusion_proof_missing_certificate() {
crates/amun-constitutional-state/src/lib.rs:82:    /// Replay a journal to reconstruct the state.
crates/amun-constitutional-state/src/lib.rs:83:    pub fn replay(records: &[StateTransitionRecord]) -> Self {
crates/amun-constitutional-verifier/src/verifier.rs:145:        assert!(verify_qc(&qc, &set));
crates/amun-constitutional-verifier/src/verifier.rs:146:        assert!(verify_vote_uniqueness(&qc));
crates/amun-constitutional/src/architectural_invariants.rs:103:/// INVARIANT 7: Replay-Derived State Identity
crates/amun-constitutional/src/architectural_invariants.rs:106:/// deterministic replay lineage and attested transcript scope,
crates/amun-constitutional/src/architectural_invariants.rs:109:/// CONSEQUENCE: State roots are attestations of replay outcomes,
crates/amun-constitutional/src/architectural_invariants.rs:111:/// the same replay lineage produced it — not the same database
crates/amun-constitutional/src/architectural_invariants.rs:114:    "State identity is replay-derived, not storage-derived.";
crates/amun-constitutional/src/architectural_invariants.rs:118:/// Snapshot restoration validity is derived from replay lineage,
crates/amun-constitutional/src/architectural_invariants.rs:11:/// replay locality, and admissibility are all local within a
crates/amun-constitutional/src/architectural_invariants.rs:131:/// Restoration does not create a new replay lineage. It continues
crates/amun-constitutional/src/architectural_invariants.rs:133:/// The restored execution is a CONTINUATION of the original replay,
crates/amun-constitutional/src/architectural_invariants.rs:136:/// CONSEQUENCE: After restoration, the replay journal continues
crates/amun-constitutional/src/architectural_invariants.rs:185:/// accidental replay equivalence, and operational proof leakage.
crates/amun-constitutional/src/architectural_invariants.rs:24:/// A ReplayCertificate attests replay ADMISSIBILITY, not
crates/amun-constitutional/src/architectural_invariants.rs:28:/// CONSEQUENCE: Certificates are replay witnesses, not
crates/amun-constitutional/src/architectural_invariants.rs:424:            assert!(!invariant.is_empty(), "Invariant must be documented");
crates/amun-constitutional/src/artifact_graph.rs:155:        assert!(g.verify_all_edges().is_ok());
crates/amun-constitutional/src/canonical_witness.rs:104:        assert_eq!(entries[0].witness_type, WitnessType::HardDependency);
crates/amun-constitutional/src/canonical_witness.rs:105:        assert_eq!(entries[3].witness_type, WitnessType::CompressionElidable);
crates/amun-constitutional/src/canonical_witness.rs:62:        witness.replay_revision,
crates/amun-constitutional/src/causal_edge.rs:111:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/causal_edge.rs:128:        replay_revision: u32,
crates/amun-constitutional/src/causal_edge.rs:138:            replay_revision,
crates/amun-constitutional/src/causal_edge.rs:175:        assert!(e.verify().is_ok());
crates/amun-constitutional/src/causal_edge.rs:188:        assert!(e.verify_structure().is_err());
crates/amun-constitutional/src/causal_edge.rs:28:    pub replay_revision: u32,
crates/amun-constitutional/src/causal_edge.rs:56:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/causal_edge.rs:57:        self.replay_revision
crates/amun-constitutional/src/causal_edge.rs:63:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/causal_edge.rs:66:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/causality_chain.rs:116:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/causality_chain.rs:162:        replay_revision: u32,
crates/amun-constitutional/src/causality_chain.rs:171:            replay_revision,
crates/amun-constitutional/src/causality_chain.rs:201:        assert!(c.verify().is_ok());
crates/amun-constitutional/src/causality_chain.rs:216:        assert!(c.verify().is_ok());
crates/amun-constitutional/src/causality_chain.rs:231:        assert!(c.verify_structure().is_err());
crates/amun-constitutional/src/causality_chain.rs:254:        assert!(c.verify_constitutional().is_err());
crates/amun-constitutional/src/causality_chain.rs:31:    pub replay_revision: u32,
crates/amun-constitutional/src/causality_chain.rs:56:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/causality_chain.rs:57:        self.replay_revision
crates/amun-constitutional/src/causality_chain.rs:63:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/causality_chain.rs:66:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/causality_type.rs:109:        assert!(!CausalityType::ConstitutionalDependency.is_non_causal());
crates/amun-constitutional/src/causality_type.rs:113:        assert!(CausalityType::ConstitutionalDependency.is_hard_dependency());
crates/amun-constitutional/src/causality_type.rs:32:    /// B's state is derived from A's state via replay.
crates/amun-constitutional/src/causality_type.rs:59:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/causality_type.rs:97:        assert!(CausalityType::ConstitutionalDependency.is_constitutional_dependency());
crates/amun-constitutional/src/certificate_scope.rs:113:            || self.replay_revision != other.replay_revision
crates/amun-constitutional/src/certificate_scope.rs:13://!   This prevents: replay ambiguity, overlapping admissibility conflicts,
crates/amun-constitutional/src/certificate_scope.rs:174:    fn make_scope(start: u64, end: u64, outcome: ReplayOutcome) -> CertificateScope {
crates/amun-constitutional/src/certificate_scope.rs:180:            replay_revision: 1,
crates/amun-constitutional/src/certificate_scope.rs:188:        let s1 = make_scope(0, 99, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:189:        let s2 = make_scope(0, 99, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:196:        let narrow = make_scope(0, 49, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:197:        let broad = make_scope(0, 99, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:19:use crate::replay_outcome::ReplayOutcome;
crates/amun-constitutional/src/certificate_scope.rs:1://! CertificateScope — the admissibility envelope of a replay certificate.
crates/amun-constitutional/src/certificate_scope.rs:203:        let narrow = make_scope(0, 49, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:204:        let broad = make_scope(0, 99, ReplayOutcome::Divergent);
crates/amun-constitutional/src/certificate_scope.rs:213:        let s1 = make_scope(0, 99, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:214:        let mut s2 = make_scope(0, 99, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:222:        let s1 = make_scope(0, 50, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:223:        let s2 = make_scope(25, 75, ReplayOutcome::Divergent);
crates/amun-constitutional/src/certificate_scope.rs:229:        let scope = make_scope(10, 20, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:240:            make_scope(0, 99, ReplayOutcome::Admitted).span_length(),
crates/amun-constitutional/src/certificate_scope.rs:244:            make_scope(10, 20, ReplayOutcome::Admitted).span_length(),
crates/amun-constitutional/src/certificate_scope.rs:251:        let parent = make_scope(0, 99, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:252:        let child = make_scope(0, 49, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:253:        assert!(child.verify_against_parent(&parent).is_ok());
crates/amun-constitutional/src/certificate_scope.rs:258:        let parent = make_scope(0, 99, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:259:        let mut child = make_scope(0, 49, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:261:        assert!(child.verify_against_parent(&parent).is_err());
crates/amun-constitutional/src/certificate_scope.rs:266:        let s1 = make_scope(0, 99, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:267:        let s2 = make_scope(0, 99, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:273:        let s1 = make_scope(0, 99, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:274:        let s2 = make_scope(0, 49, ReplayOutcome::Admitted);
crates/amun-constitutional/src/certificate_scope.rs:41:/// The constitutional scope of a replay certificate.
crates/amun-constitutional/src/certificate_scope.rs:59:    /// Replay revision this certificate was issued under.
crates/amun-constitutional/src/certificate_scope.rs:60:    pub replay_revision: u32,
crates/amun-constitutional/src/certificate_scope.rs:66:    pub outcome: ReplayOutcome,
crates/amun-constitutional/src/certificate_scope.rs:72:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/certificate_scope.rs:78:            .update_u32(self.replay_revision)
crates/amun-constitutional/src/constitutional_failure.rs:102:            .update_optional_u64(self.replay_domain.map(|x| x as u64))
crates/amun-constitutional/src/constitutional_failure.rs:146:            || self.replay_revision == 0
crates/amun-constitutional/src/constitutional_failure.rs:186:    pub fn with_replay_domain(mut self, d: u8) -> Self {
crates/amun-constitutional/src/constitutional_failure.rs:187:        self.replay_domain = Some(d);
crates/amun-constitutional/src/constitutional_failure.rs:233:        assert!(m().verify().is_ok());
crates/amun-constitutional/src/constitutional_failure.rs:39:    pub replay_revision: u32,
crates/amun-constitutional/src/constitutional_failure.rs:46:    pub replay_domain: Option<u8>,
crates/amun-constitutional/src/constitutional_failure.rs:70:            replay_revision: 1,
crates/amun-constitutional/src/constitutional_failure.rs:77:            replay_domain: None,
crates/amun-constitutional/src/constitutional_failure.rs:95:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/constitutional_hasher.rs:23:    pub fn update_revision(&mut self, constitutional: u32, replay: u32) -> &mut Self {
crates/amun-constitutional/src/constitutional_hasher.rs:25:        self.inner.update(replay.to_le_bytes());
crates/amun-constitutional/src/constitutional_object.rs:24:    fn replay_revision(&self) -> u32;
crates/amun-constitutional/src/constitutional_witness.rs:101:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/constitutional_witness.rs:102:        self.replay_revision
crates/amun-constitutional/src/constitutional_witness.rs:108:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/constitutional_witness.rs:111:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/constitutional_witness.rs:161:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/constitutional_witness.rs:202:        replay_revision: u32,
crates/amun-constitutional/src/constitutional_witness.rs:213:            replay_revision,
crates/amun-constitutional/src/constitutional_witness.rs:262:        assert!(w.verify().is_ok());
crates/amun-constitutional/src/constitutional_witness.rs:297:        assert!(w.verify_structure().is_err());
crates/amun-constitutional/src/constitutional_witness.rs:306:        assert!(w.verify_constitutional().is_err());
crates/amun-constitutional/src/constitutional_witness.rs:70:    pub replay_revision: u32,
crates/amun-constitutional/src/continuation_chain.rs:117:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/continuation_chain.rs:134:        replay_revision: u32,
crates/amun-constitutional/src/continuation_chain.rs:146:            replay_revision,
crates/amun-constitutional/src/continuation_chain.rs:217:        assert!(chain.verify().is_ok());
crates/amun-constitutional/src/continuation_chain.rs:225:        assert!(chain.verify_continuation(&rp).is_ok());
crates/amun-constitutional/src/continuation_chain.rs:233:        assert!(chain.verify_continuation(&rp).is_err());
crates/amun-constitutional/src/continuation_chain.rs:24:///   - The restoration point (where replay resumes)
crates/amun-constitutional/src/continuation_chain.rs:34:    pub replay_revision: u32,
crates/amun-constitutional/src/continuation_chain.rs:3://! The continuation chain verifies that the post-restoration replay
crates/amun-constitutional/src/continuation_chain.rs:69:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/continuation_chain.rs:70:        self.replay_revision
crates/amun-constitutional/src/continuation_chain.rs:76:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/continuation_chain.rs:79:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/continuation_chain.rs:7://!   Restoration does not create a new replay lineage. The chain
crates/amun-constitutional/src/divergence_point.rs:120:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/divergence_point.rs:137:        replay_revision: u32,
crates/amun-constitutional/src/divergence_point.rs:151:            replay_revision,
crates/amun-constitutional/src/divergence_point.rs:16:/// The point in the transcript where replay diverged.
crates/amun-constitutional/src/divergence_point.rs:195:        assert!(d.verify().is_ok());
crates/amun-constitutional/src/divergence_point.rs:1://! DivergencePoint — the constitutional position where replay diverged.
crates/amun-constitutional/src/divergence_point.rs:216:    fn test_replay_error() {
crates/amun-constitutional/src/divergence_point.rs:222:            DivergenceType::ReplayError,
crates/amun-constitutional/src/divergence_point.rs:26:    pub replay_revision: u32,
crates/amun-constitutional/src/divergence_point.rs:69:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/divergence_point.rs:70:        self.replay_revision
crates/amun-constitutional/src/divergence_point.rs:76:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/divergence_point.rs:79:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/divergence_resolution.rs:127:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/divergence_resolution.rs:144:        replay_revision: u32,
crates/amun-constitutional/src/divergence_resolution.rs:157:            replay_revision,
crates/amun-constitutional/src/divergence_resolution.rs:190:        assert!(r.verify().is_ok());
crates/amun-constitutional/src/divergence_resolution.rs:29:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/divergence_resolution.rs:41:    pub replay_revision: u32,
crates/amun-constitutional/src/divergence_resolution.rs:78:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/divergence_resolution.rs:79:        self.replay_revision
crates/amun-constitutional/src/divergence_resolution.rs:85:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/divergence_resolution.rs:88:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/divergence_type.rs:11:/// The constitutional classification of a replay divergence.
crates/amun-constitutional/src/divergence_type.rs:18:    /// A divergence caused by replay error — should be investigated.
crates/amun-constitutional/src/divergence_type.rs:1://! DivergenceType — constitutional classification of replay divergence.
crates/amun-constitutional/src/divergence_type.rs:20:    ReplayError = 0x02,
crates/amun-constitutional/src/divergence_type.rs:41:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/divergence_type.rs:4://! governance transitions), others are violations (replay errors, boundary
crates/amun-constitutional/src/divergence_type.rs:56:    /// Returns true if this divergence indicates a replay error.
crates/amun-constitutional/src/divergence_type.rs:60:            DivergenceType::ReplayError | DivergenceType::BoundaryViolation
crates/amun-constitutional/src/divergence_type.rs:75:        assert!(DivergenceType::ConstitutionalFork.is_admissible());
crates/amun-constitutional/src/divergence_type.rs:77:        assert!(DivergenceType::ConstitutionalSupersession.is_admissible());
crates/amun-constitutional/src/divergence_type.rs:81:        assert!(DivergenceType::ReplayError.is_error());
crates/amun-constitutional/src/divergence_type.rs:83:        assert!(!DivergenceType::ConstitutionalFork.is_error());
crates/amun-constitutional/src/divergence_type.rs:88:        assert!(!DivergenceType::ReplayError.is_ambiguous());
crates/amun-constitutional/src/execution_boundary.rs:108:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/execution_boundary.rs:135:            replay_revision: 1,
crates/amun-constitutional/src/execution_boundary.rs:13:    pub replay_revision: u32,
crates/amun-constitutional/src/execution_boundary.rs:183:        assert!(mb().verify().is_ok());
crates/amun-constitutional/src/execution_boundary.rs:39:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/execution_boundary.rs:40:        self.replay_revision
crates/amun-constitutional/src/execution_boundary.rs:48:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/execution_context.rs:102:            replay_lineage_hash: None,
crates/amun-constitutional/src/execution_context.rs:110:    pub fn with_replay_lineage(mut self, h: [u8; 32]) -> Self {
crates/amun-constitutional/src/execution_context.rs:111:        self.replay_lineage_hash = Some(h);
crates/amun-constitutional/src/execution_context.rs:12:    pub replay_revision: u32,
crates/amun-constitutional/src/execution_context.rs:132:        assert!(ExecutionContext::new(1, [0xAB; 32], 0).verify().is_ok());
crates/amun-constitutional/src/execution_context.rs:16:    pub replay_lineage_hash: Option<[u8; 32]>,
crates/amun-constitutional/src/execution_context.rs:32:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/execution_context.rs:33:        self.replay_revision
crates/amun-constitutional/src/execution_context.rs:41:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/execution_context.rs:44:            .update_optional_hash(self.replay_lineage_hash.as_ref())
crates/amun-constitutional/src/execution_context.rs:75:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/execution_context.rs:98:            replay_revision: 1,
crates/amun-constitutional/src/execution_journal.rs:106:            replay_revision: 1,
crates/amun-constitutional/src/execution_journal.rs:120:    pub fn with_revision(mut self, constitutional: u32, replay: u32) -> Self {
crates/amun-constitutional/src/execution_journal.rs:122:        self.replay_revision = replay;
crates/amun-constitutional/src/execution_journal.rs:138:    pub replay_revision: u32,
crates/amun-constitutional/src/execution_journal.rs:13:    pub replay_revision: u32,
crates/amun-constitutional/src/execution_journal.rs:157:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/execution_journal.rs:158:        self.replay_revision
crates/amun-constitutional/src/execution_journal.rs:166:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/execution_journal.rs:199:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/execution_journal.rs:218:            replay_revision: 1,
crates/amun-constitutional/src/execution_journal.rs:241:        if entry.replay_revision != self.replay_revision {
crates/amun-constitutional/src/execution_journal.rs:247:                "Replay revision mismatch",
crates/amun-constitutional/src/execution_journal.rs:300:        assert!(me(&mc(), 1, 0, None).verify().is_ok());
crates/amun-constitutional/src/execution_journal.rs:35:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/execution_journal.rs:36:        self.replay_revision
crates/amun-constitutional/src/execution_journal.rs:44:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/execution_journal.rs:79:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/execution_limits.rs:139:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/execution_limits.rs:169:        if self.replay.max_transcript_span == 0 || self.admissibility.max_events_per_context == 0 {
crates/amun-constitutional/src/execution_limits.rs:194:        assert!(ExecutionLimits::constitutional_default().verify().is_ok());
crates/amun-constitutional/src/execution_limits.rs:38:    pub replay_revision: u32,
crates/amun-constitutional/src/execution_limits.rs:44:    pub replay: ReplayLimits,
crates/amun-constitutional/src/execution_limits.rs:57:            replay_revision: 1,
crates/amun-constitutional/src/execution_limits.rs:63:            replay: ReplayLimits {
crates/amun-constitutional/src/execution_limits.rs:65:                max_replay_divergence: 1000,
crates/amun-constitutional/src/execution_limits.rs:7:pub struct ReplayLimits {
crates/amun-constitutional/src/execution_limits.rs:92:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/execution_limits.rs:97:        h.update_u64(self.replay.max_transcript_span)
crates/amun-constitutional/src/execution_limits.rs:98:            .update_u64(self.replay.max_replay_divergence)
crates/amun-constitutional/src/execution_limits.rs:99:            .update_u64(self.replay.max_journal_entries);
crates/amun-constitutional/src/execution_limits.rs:9:    pub max_replay_divergence: u64,
crates/amun-constitutional/src/execution_receipt.rs:106:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/execution_receipt.rs:115:            .update_u8(self.replay_outcome as u8);
crates/amun-constitutional/src/execution_receipt.rs:147:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/execution_receipt.rs:15://!   The certificate_hash links to a ReplayCertificate that attests
crates/amun-constitutional/src/execution_receipt.rs:16://!   replay ADMISSIBILITY, not execution finality.
crates/amun-constitutional/src/execution_receipt.rs:172:        replay_revision: u32,
crates/amun-constitutional/src/execution_receipt.rs:180:        replay_outcome: ReplayOutcome,
crates/amun-constitutional/src/execution_receipt.rs:187:            replay_revision,
crates/amun-constitutional/src/execution_receipt.rs:197:            replay_outcome,
crates/amun-constitutional/src/execution_receipt.rs:205:        self.replay_outcome.is_admitted()
crates/amun-constitutional/src/execution_receipt.rs:208:        self.replay_outcome.is_failure()
crates/amun-constitutional/src/execution_receipt.rs:236:            ReplayOutcome::Admitted,
crates/amun-constitutional/src/execution_receipt.rs:243:        assert!(make_receipt(1, 0, [0xAB; 32], None).verify().is_ok());
crates/amun-constitutional/src/execution_receipt.rs:255:        assert!(r.verify().is_ok());
crates/amun-constitutional/src/execution_receipt.rs:26:use crate::replay_outcome::ReplayOutcome;
crates/amun-constitutional/src/execution_receipt.rs:278:            ReplayOutcome::ConstitutionalFailure,
crates/amun-constitutional/src/execution_receipt.rs:28:/// A terminal constitutional witness attesting to replay admissibility.
crates/amun-constitutional/src/execution_receipt.rs:292:            ReplayOutcome::ConstitutionalFailure,
crates/amun-constitutional/src/execution_receipt.rs:42:/// `certificate_hash` links to a ReplayCertificate. The certificate model
crates/amun-constitutional/src/execution_receipt.rs:51:    pub replay_revision: u32,
crates/amun-constitutional/src/execution_receipt.rs:72:    /// Certificate attesting replay admissibility (admissibility-first model).
crates/amun-constitutional/src/execution_receipt.rs:80:    pub replay_outcome: ReplayOutcome,
crates/amun-constitutional/src/execution_receipt.rs:97:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/execution_receipt.rs:98:        self.replay_revision
crates/amun-constitutional/src/execution_receipt.rs:9://!      This preserves replay locality and prevents global sequencing bottlenecks.
crates/amun-constitutional/src/hash_domains.rs:39:/// Domain: ReplayOutcome objects
crates/amun-constitutional/src/hash_domains.rs:45:/// Domain: ReplayCertificate objects
crates/amun-constitutional/src/kernel_types.rs:14:pub struct ReplayFailure {
crates/amun-constitutional/src/kernel_types.rs:16:    pub domain: ReplayDomain,
crates/amun-constitutional/src/kernel_types.rs:21:pub struct ReplayPolicy {
crates/amun-constitutional/src/kernel_types.rs:27:impl Default for ReplayPolicy {
crates/amun-constitutional/src/kernel_types.rs:41:    pub domain: ReplayDomain,
crates/amun-constitutional/src/kernel_types.rs:6:pub enum ReplayDomain {
crates/amun-constitutional/src/lib.rs:121:pub use replay_certificate::ReplayCertificate;
crates/amun-constitutional/src/lib.rs:122:pub use replay_outcome::ReplayOutcome;
crates/amun-constitutional/src/lib.rs:59:pub mod replay_certificate;
crates/amun-constitutional/src/lib.rs:60:pub mod replay_outcome;
crates/amun-constitutional/src/replay_certificate.rs:101:    fn verify_provenance(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/replay_certificate.rs:102:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/replay_certificate.rs:103:            return Err(ConstitutionalFailure::new(
crates/amun-constitutional/src/replay_certificate.rs:114:    fn verify_constitutional(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/replay_certificate.rs:116:            || self.scope.replay_revision != self.replay_revision
crates/amun-constitutional/src/replay_certificate.rs:118:            return Err(ConstitutionalFailure::new(
crates/amun-constitutional/src/replay_certificate.rs:11:use crate::constitutional_hasher::ConstitutionalHasher;
crates/amun-constitutional/src/replay_certificate.rs:12:use crate::constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};
crates/amun-constitutional/src/replay_certificate.rs:130:impl ReplayCertificate {
crates/amun-constitutional/src/replay_certificate.rs:131:    pub fn new(
crates/amun-constitutional/src/replay_certificate.rs:134:        replay_revision: u32,
crates/amun-constitutional/src/replay_certificate.rs:137:        context_hash: ConstitutionalHash,
crates/amun-constitutional/src/replay_certificate.rs:138:        boundary_hash: ConstitutionalHash,
crates/amun-constitutional/src/replay_certificate.rs:139:        outcome: crate::replay_outcome::ReplayOutcome,
crates/amun-constitutional/src/replay_certificate.rs:140:        journal_root: ConstitutionalHash,
crates/amun-constitutional/src/replay_certificate.rs:141:        state_root: ConstitutionalHash,
crates/amun-constitutional/src/replay_certificate.rs:142:        parent_certificate_hash: Option<ConstitutionalHash>,
crates/amun-constitutional/src/replay_certificate.rs:149:            replay_revision,
crates/amun-constitutional/src/replay_certificate.rs:14:use crate::kernel_types::ConstitutionalHash;
crates/amun-constitutional/src/replay_certificate.rs:157:            replay_revision,
crates/amun-constitutional/src/replay_certificate.rs:170:    pub fn outcome(&self) -> crate::replay_outcome::ReplayOutcome {
crates/amun-constitutional/src/replay_certificate.rs:173:    pub fn is_admitted(&self) -> bool {
crates/amun-constitutional/src/replay_certificate.rs:177:    pub fn verify_scope_against_parent(
crates/amun-constitutional/src/replay_certificate.rs:179:        parent: &ReplayCertificate,
crates/amun-constitutional/src/replay_certificate.rs:180:    ) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/replay_certificate.rs:181:        match self.scope.verify_against_parent(&parent.scope) {
crates/amun-constitutional/src/replay_certificate.rs:183:            Err(_) => Err(ConstitutionalFailure::new(
crates/amun-constitutional/src/replay_certificate.rs:18:pub struct ReplayCertificate {
crates/amun-constitutional/src/replay_certificate.rs:197:    use crate::replay_outcome::ReplayOutcome;
crates/amun-constitutional/src/replay_certificate.rs:199:    fn mc(
crates/amun-constitutional/src/replay_certificate.rs:1://! ReplayCertificate — cryptographically scoped admissibility envelope.
crates/amun-constitutional/src/replay_certificate.rs:203:        outcome: ReplayOutcome,
crates/amun-constitutional/src/replay_certificate.rs:204:        parent: Option<ConstitutionalHash>,
crates/amun-constitutional/src/replay_certificate.rs:205:    ) -> ReplayCertificate {
crates/amun-constitutional/src/replay_certificate.rs:206:        ReplayCertificate::new(
crates/amun-constitutional/src/replay_certificate.rs:212:    fn test_cert_verifies() {
crates/amun-constitutional/src/replay_certificate.rs:213:        assert!(mc(1, 0, 99, ReplayOutcome::Admitted, None).verify().is_ok());
crates/amun-constitutional/src/replay_certificate.rs:216:    fn test_hash_det() {
crates/amun-constitutional/src/replay_certificate.rs:218:            mc(1, 0, 99, ReplayOutcome::Admitted, None).certificate_hash,
crates/amun-constitutional/src/replay_certificate.rs:219:            mc(1, 0, 99, ReplayOutcome::Admitted, None).certificate_hash
crates/amun-constitutional/src/replay_certificate.rs:223:    fn test_scope_affects_hash() {
crates/amun-constitutional/src/replay_certificate.rs:225:            mc(1, 0, 49, ReplayOutcome::Admitted, None).certificate_hash,
crates/amun-constitutional/src/replay_certificate.rs:226:            mc(1, 0, 99, ReplayOutcome::Admitted, None).certificate_hash
crates/amun-constitutional/src/replay_certificate.rs:22:    pub replay_revision: u32,
crates/amun-constitutional/src/replay_certificate.rs:230:    fn test_outcome_affects_hash() {
crates/amun-constitutional/src/replay_certificate.rs:232:            mc(1, 0, 99, ReplayOutcome::Admitted, None).certificate_hash,
crates/amun-constitutional/src/replay_certificate.rs:233:            mc(1, 0, 99, ReplayOutcome::Divergent, None).certificate_hash
crates/amun-constitutional/src/replay_certificate.rs:237:    fn test_monotonicity_ok() {
crates/amun-constitutional/src/replay_certificate.rs:238:        let p = mc(1, 0, 99, ReplayOutcome::Admitted, None);
crates/amun-constitutional/src/replay_certificate.rs:239:        let c = mc(2, 0, 49, ReplayOutcome::Admitted, Some(p.certificate_hash));
crates/amun-constitutional/src/replay_certificate.rs:240:        assert!(c.verify_scope_against_parent(&p).is_ok());
crates/amun-constitutional/src/replay_certificate.rs:244:    fn test_monotonicity_violated() {
crates/amun-constitutional/src/replay_certificate.rs:246:        let p = mc(1, 0, 99, ReplayOutcome::Admitted, None);
crates/amun-constitutional/src/replay_certificate.rs:247:        let c = ReplayCertificate::new(
crates/amun-constitutional/src/replay_certificate.rs:24:    pub certificate_hash: ConstitutionalHash,
crates/amun-constitutional/src/replay_certificate.rs:255:            ReplayOutcome::Admitted,
crates/amun-constitutional/src/replay_certificate.rs:260:        assert!(c.verify_scope_against_parent(&p).is_err());
crates/amun-constitutional/src/replay_certificate.rs:264:    fn test_invalid_span_rejected() {
crates/amun-constitutional/src/replay_certificate.rs:265:        let mut c = mc(1, 100, 50, ReplayOutcome::Admitted, None);
crates/amun-constitutional/src/replay_certificate.rs:267:        assert!(c.verify_structure().is_err());
crates/amun-constitutional/src/replay_certificate.rs:26:    pub journal_root: ConstitutionalHash,
crates/amun-constitutional/src/replay_certificate.rs:270:    fn test_revision_mismatch_rejected() {
crates/amun-constitutional/src/replay_certificate.rs:271:        let mut c = mc(1, 0, 99, ReplayOutcome::Admitted, None);
crates/amun-constitutional/src/replay_certificate.rs:274:        assert!(c.verify_constitutional().is_err());
crates/amun-constitutional/src/replay_certificate.rs:27:    pub state_root: ConstitutionalHash,
crates/amun-constitutional/src/replay_certificate.rs:28:    pub parent_certificate_hash: Option<ConstitutionalHash>,
crates/amun-constitutional/src/replay_certificate.rs:32:impl ConstitutionalIdentity for ReplayCertificate {
crates/amun-constitutional/src/replay_certificate.rs:33:    fn schema_id(&self) -> u16 {
crates/amun-constitutional/src/replay_certificate.rs:36:    fn schema_version(&self) -> u16 {
crates/amun-constitutional/src/replay_certificate.rs:39:    fn constitutional_revision(&self) -> u32 {
crates/amun-constitutional/src/replay_certificate.rs:42:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/replay_certificate.rs:43:        self.replay_revision
crates/amun-constitutional/src/replay_certificate.rs:47:impl ConstitutionalObject for ReplayCertificate {
crates/amun-constitutional/src/replay_certificate.rs:48:    fn constitutional_hash(&self) -> ConstitutionalHash {
crates/amun-constitutional/src/replay_certificate.rs:49:        let mut h = ConstitutionalHasher::new(DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/replay_certificate.rs:51:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/replay_certificate.rs:57:            .update_u32(self.scope.replay_revision)
crates/amun-constitutional/src/replay_certificate.rs:63:        h.finalize()
crates/amun-constitutional/src/replay_certificate.rs:66:    fn verify_structure(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/replay_certificate.rs:68:            return Err(ConstitutionalFailure::new(
crates/amun-constitutional/src/replay_certificate.rs:77:            return Err(ConstitutionalFailure::new(
crates/amun-constitutional/src/replay_certificate.rs:88:    fn verify_semantics(&self) -> Result<(), ConstitutionalFailure> {
crates/amun-constitutional/src/replay_certificate.rs:90:            return Err(ConstitutionalFailure::new(
crates/amun-constitutional/src/replay_certificate.rs:9:    failure_domain, failure_type, severity, ConstitutionalFailure,
crates/amun-constitutional/src/replay_outcome.rs:10:/// The constitutional outcome of a replay.
crates/amun-constitutional/src/replay_outcome.rs:15:pub enum ReplayOutcome {
crates/amun-constitutional/src/replay_outcome.rs:1://! ReplayOutcome — constitutional admissibility result.
crates/amun-constitutional/src/replay_outcome.rs:26:    ConstitutionalFailure = 0x04,
crates/amun-constitutional/src/replay_outcome.rs:29:impl ReplayOutcome {
crates/amun-constitutional/src/replay_outcome.rs:32:    pub fn outcome_hash(&self) -> ConstitutionalHash {
crates/amun-constitutional/src/replay_outcome.rs:33:        let mut h = ConstitutionalHasher::new(crate::hash_domains::DOMAIN_REPLAY_OUTCOME);
crates/amun-constitutional/src/replay_outcome.rs:35:        h.finalize()
crates/amun-constitutional/src/replay_outcome.rs:38:    /// Returns true if this outcome represents a successful replay.
crates/amun-constitutional/src/replay_outcome.rs:39:    pub fn is_admitted(&self) -> bool {
crates/amun-constitutional/src/replay_outcome.rs:40:        matches!(self, ReplayOutcome::Admitted)
crates/amun-constitutional/src/replay_outcome.rs:44:    pub fn is_failure(&self) -> bool {
crates/amun-constitutional/src/replay_outcome.rs:54:    fn test_outcome_hash_deterministic() {
crates/amun-constitutional/src/replay_outcome.rs:56:            ReplayOutcome::Admitted.outcome_hash(),
crates/amun-constitutional/src/replay_outcome.rs:57:            ReplayOutcome::Admitted.outcome_hash()
crates/amun-constitutional/src/replay_outcome.rs:62:    fn test_different_outcomes_different_hashes() {
crates/amun-constitutional/src/replay_outcome.rs:64:            ReplayOutcome::Admitted.outcome_hash(),
crates/amun-constitutional/src/replay_outcome.rs:65:            ReplayOutcome::Divergent.outcome_hash()
crates/amun-constitutional/src/replay_outcome.rs:70:    fn test_is_admitted() {
crates/amun-constitutional/src/replay_outcome.rs:71:        assert!(ReplayOutcome::Admitted.is_admitted());
crates/amun-constitutional/src/replay_outcome.rs:72:        assert!(!ReplayOutcome::Divergent.is_admitted());
crates/amun-constitutional/src/replay_outcome.rs:73:        assert!(!ReplayOutcome::BoundaryViolation.is_admitted());
crates/amun-constitutional/src/replay_outcome.rs:74:        assert!(!ReplayOutcome::ConstitutionalFailure.is_admitted());
crates/amun-constitutional/src/replay_outcome.rs:78:    fn test_is_failure() {
crates/amun-constitutional/src/replay_outcome.rs:79:        assert!(!ReplayOutcome::Admitted.is_failure());
crates/amun-constitutional/src/replay_outcome.rs:7:use crate::constitutional_hasher::ConstitutionalHasher;
crates/amun-constitutional/src/replay_outcome.rs:80:        assert!(ReplayOutcome::Divergent.is_failure());
crates/amun-constitutional/src/replay_outcome.rs:81:        assert!(ReplayOutcome::BoundaryViolation.is_failure());
crates/amun-constitutional/src/replay_outcome.rs:82:        assert!(ReplayOutcome::ConstitutionalFailure.is_failure());
crates/amun-constitutional/src/replay_outcome.rs:8:use crate::kernel_types::ConstitutionalHash;
crates/amun-constitutional/src/restoration_point.rs:117:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/restoration_point.rs:134:        replay_revision: u32,
crates/amun-constitutional/src/restoration_point.rs:146:            replay_revision,
crates/amun-constitutional/src/restoration_point.rs:18:/// Where replay resumes after restoration.
crates/amun-constitutional/src/restoration_point.rs:194:        assert!(rp.verify().is_ok());
crates/amun-constitutional/src/restoration_point.rs:1://! RestorationPoint — the constitutional position where replay resumes.
crates/amun-constitutional/src/restoration_point.rs:29:    pub replay_revision: u32,
crates/amun-constitutional/src/restoration_point.rs:3://! A restoration point defines exactly where in the replay lineage
crates/amun-constitutional/src/restoration_point.rs:40:    /// The transcript position where replay resumes.
crates/amun-constitutional/src/restoration_point.rs:68:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/restoration_point.rs:69:        self.replay_revision
crates/amun-constitutional/src/restoration_point.rs:75:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/restoration_point.rs:78:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/schema_registry.rs:163:        assert!(core_schemas::verify_uniqueness());
crates/amun-constitutional/src/schema_registry.rs:168:        assert!(core_schemas::verify_range());
crates/amun-constitutional/src/schema_registry.rs:69:    /// ReplayCertificate
crates/amun-constitutional/src/snapshot.rs:11://!   Replay → StateAnchor → Snapshot
crates/amun-constitutional/src/snapshot.rs:140:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/snapshot.rs:154:            || self.scope.replay_revision != self.replay_revision
crates/amun-constitutional/src/snapshot.rs:182:        replay_revision: u32,
crates/amun-constitutional/src/snapshot.rs:198:            replay_revision,
crates/amun-constitutional/src/snapshot.rs:1://! ConstitutionalSnapshot — constitutionally restorable replay surface.
crates/amun-constitutional/src/snapshot.rs:206:            replay_revision,
crates/amun-constitutional/src/snapshot.rs:24:/// A constitutionally restorable replay surface.
crates/amun-constitutional/src/snapshot.rs:27:/// replay lineage, is admissible for restoration under specific constitutional
crates/amun-constitutional/src/snapshot.rs:320:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/snapshot.rs:338:        assert!(c.verify_against_parent(&p).is_err());
crates/amun-constitutional/src/snapshot.rs:345:        assert!(s.verify_constitutional().is_err());
crates/amun-constitutional/src/snapshot.rs:43:    pub replay_revision: u32,
crates/amun-constitutional/src/snapshot.rs:4://!   "This replay lineage, within this scope, produced this state anchor,
crates/amun-constitutional/src/snapshot.rs:74:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/snapshot.rs:75:        self.replay_revision
crates/amun-constitutional/src/snapshot.rs:81:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/snapshot.rs:84:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/snapshot.rs:92:            .update_u32(self.scope.replay_revision)
crates/amun-constitutional/src/snapshot_scope.rs:124:            || self.replay_revision != other.replay_revision
crates/amun-constitutional/src/snapshot_scope.rs:180:            replay_revision: 1,
crates/amun-constitutional/src/snapshot_scope.rs:217:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/snapshot_scope.rs:224:        assert!(c.verify_against_parent(&p).is_err());
crates/amun-constitutional/src/snapshot_scope.rs:42:    /// Snapshot is from a divergent replay lineage.
crates/amun-constitutional/src/snapshot_scope.rs:55:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/snapshot_scope.rs:64:/// The scope defines the restoration surface — what replay window
crates/amun-constitutional/src/snapshot_scope.rs:79:    /// Replay revision active for this scope.
crates/amun-constitutional/src/snapshot_scope.rs:80:    pub replay_revision: u32,
crates/amun-constitutional/src/snapshot_scope.rs:89:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/snapshot_scope.rs:8://! replay surface, not a serialized storage image.
crates/amun-constitutional/src/snapshot_scope.rs:96:            .update_u32(self.replay_revision)
crates/amun-constitutional/src/state_anchor.rs:11://! It is REPLAY-DERIVED: the state root is an output of replay, not storage.
crates/amun-constitutional/src/state_anchor.rs:134:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/state_anchor.rs:13://! INVARIANT (Replay-Derived State Identity):
crates/amun-constitutional/src/state_anchor.rs:148:            || self.scope.replay_revision != self.replay_revision
crates/amun-constitutional/src/state_anchor.rs:15://!   replay lineage and attested transcript scope, never from mutable storage
crates/amun-constitutional/src/state_anchor.rs:166:        replay_revision: u32,
crates/amun-constitutional/src/state_anchor.rs:182:            replay_revision,
crates/amun-constitutional/src/state_anchor.rs:189:            replay_revision,
crates/amun-constitutional/src/state_anchor.rs:1://! ConstitutionalStateAnchor — immutable, replay-derived state attestation.
crates/amun-constitutional/src/state_anchor.rs:243:        assert!(ma(1, 0, 99, [0x11; 32], None).verify().is_ok());
crates/amun-constitutional/src/state_anchor.rs:270:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/state_anchor.rs:276:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/state_anchor.rs:28:/// An immutable constitutional attestation of replay-derived state.
crates/amun-constitutional/src/state_anchor.rs:294:        assert!(c.verify_against_parent(&p).is_err());
crates/amun-constitutional/src/state_anchor.rs:300:        assert!(a.verify_structure().is_err());
crates/amun-constitutional/src/state_anchor.rs:307:        assert!(a.verify_constitutional().is_err());
crates/amun-constitutional/src/state_anchor.rs:30:/// The anchor says: "Within this scope, replay produced this state root."
crates/amun-constitutional/src/state_anchor.rs:38:    pub replay_revision: u32,
crates/amun-constitutional/src/state_anchor.rs:69:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/state_anchor.rs:6://! A StateAnchor attests that a specific replay lineage, within a specific
crates/amun-constitutional/src/state_anchor.rs:70:        self.replay_revision
crates/amun-constitutional/src/state_anchor.rs:76:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/state_anchor.rs:79:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/state_anchor.rs:87:            .update_u32(self.scope.replay_revision)
crates/amun-constitutional/src/state_anchor_scope.rs:148:            replay_revision: 1,
crates/amun-constitutional/src/state_anchor_scope.rs:193:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/state_anchor_scope.rs:199:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/state_anchor_scope.rs:206:        assert!(c.verify_against_parent(&p).is_err());
crates/amun-constitutional/src/state_anchor_scope.rs:50:    /// Replay revision active for this scope.
crates/amun-constitutional/src/state_anchor_scope.rs:51:    pub replay_revision: u32,
crates/amun-constitutional/src/state_anchor_scope.rs:58:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/state_anchor_scope.rs:65:            .update_u32(self.replay_revision)
crates/amun-constitutional/src/state_anchor_scope.rs:89:            || self.replay_revision != other.replay_revision
crates/amun-constitutional/src/transition_commitment.rs:104:            replay_revision: 1,
crates/amun-constitutional/src/transition_commitment.rs:126:        assert!(c.verify().is_ok());
crates/amun-constitutional/src/transition_commitment.rs:12:    pub replay_revision: u32,
crates/amun-constitutional/src/transition_commitment.rs:33:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/transition_commitment.rs:34:        self.replay_revision
crates/amun-constitutional/src/transition_commitment.rs:42:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/transition_commitment.rs:77:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/transition_evidence.rs:107:            replay_revision: 1,
crates/amun-constitutional/src/transition_evidence.rs:130:        assert!(e.verify().is_ok());
crates/amun-constitutional/src/transition_evidence.rs:13:    pub replay_revision: u32,
crates/amun-constitutional/src/transition_evidence.rs:36:    fn replay_revision(&self) -> u32 {
crates/amun-constitutional/src/transition_evidence.rs:37:        self.replay_revision
crates/amun-constitutional/src/transition_evidence.rs:45:            .update_revision(self.constitutional_revision, self.replay_revision)
crates/amun-constitutional/src/transition_evidence.rs:80:        if self.constitutional_revision == 0 || self.replay_revision == 0 {
crates/amun-constitutional/src/witness_type.rs:36:        let mut h = ConstitutionalHasher::new(hash_domains::DOMAIN_REPLAY_CERTIFICATE);
crates/amun-constitutional/src/witness_type.rs:66:        assert!(WitnessType::HardDependency.is_required());
crates/amun-constitutional/src/witness_type.rs:67:        assert!(!WitnessType::SupportingDependency.is_required());
crates/amun-constitutional/src/witness_type.rs:68:        assert!(!WitnessType::AuditDependency.is_required());
crates/amun-constitutional/src/witness_type.rs:69:        assert!(!WitnessType::CompressionElidable.is_required());
crates/amun-constitutional/src/witness_type.rs:73:        assert!(WitnessType::CompressionElidable.is_elidable());
crates/amun-constitutional/src/witness_type.rs:74:        assert!(!WitnessType::HardDependency.is_elidable());
crates/amun-constitutional/src/witness_type.rs:78:        assert!(WitnessType::AuditDependency.is_non_essential());
crates/amun-constitutional/src/witness_type.rs:79:        assert!(WitnessType::CompressionElidable.is_non_essential());
crates/amun-constitutional/src/witness_type.rs:80:        assert!(!WitnessType::HardDependency.is_non_essential());
crates/amun-contract-events/tests/n173_events_storage_tests.rs:14:fn n173_emit_and_query_events() {
crates/amun-contract-events/tests/n173_events_storage_tests.rs:24:fn n173_events_root_deterministic() {
crates/amun-contract-events/tests/n173_events_storage_tests.rs:31:        storage1.compute_events_root(),
crates/amun-contract-events/tests/n173_events_storage_tests.rs:32:        storage2.compute_events_root()
crates/amun-contract-events/tests/n173_events_storage_tests.rs:5:fn n173_store_and_retrieve() {
crates/amun-contract-fuzzing/src/lib.rs:30:    pub fn passed(&self) -> bool {
crates/amun-contract-fuzzing/tests/n171_fuzzing_tests.rs:12:        "Evidence mismatches: {}",
crates/amun-contract-fuzzing/tests/n171_fuzzing_tests.rs:19:fn n171_fuzz_call_5000() {
crates/amun-contract-fuzzing/tests/n171_fuzzing_tests.rs:30:        "Evidence mismatches: {}",
crates/amun-contract-fuzzing/tests/n171_fuzzing_tests.rs:36:fn n171_fuzz_gas_limits_5000() {
crates/amun-contract-fuzzing/tests/n171_fuzzing_tests.rs:47:        "Evidence mismatches: {}",
crates/amun-contract-fuzzing/tests/n171_fuzzing_tests.rs:4:fn n171_fuzz_deploy_10000() {
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:19:    let program = ConstitutionalProgram::new(1, 0, 0, code.clone());
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:1:use amun_bytecode::{ConstitutionalProgram, OpCode};
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:20:    assert!(program.verify());
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:38:fn n167_contract_evidence_root_deterministic() {
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:39:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:3:use amun_resource_core::{ResourceId, ResourceRegistry};
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:40:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:46:    let root1 = ContractExecutor::compute_contract_evidence_root(&reg1);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:47:    let root2 = ContractExecutor::compute_contract_evidence_root(&reg2);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:52:fn n167_invalid_program_rejected() {
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:53:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:60:fn n167_contract_state_persistence() {
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:61:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:6:fn n167_deploy_and_call_contract() {
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:7:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:10:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:25:        lineage: ResourceLineage::genesis(col_id),
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:40:            lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:53:fn n169_multiple_contracts_independent() {
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:54:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:73:fn n169_contract_registry_root_deterministic() {
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:74:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:75:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:91:    assert_eq!(cr1.compute_registry_root(), cr2.compute_registry_root());
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:9:fn n169_contract_interacts_with_nft() {
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:32:fn n172_contract_a_cannot_modify_contract_b_state() {
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:33:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:3:use amun_resource_core::{ResourceId, ResourceRegistry};
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:44:    let root_before = cr.compute_registry_root();
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:46:    let root_after = cr.compute_registry_root();
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:6:fn n172_cross_contract_call_succeeds() {
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:7:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-security/tests/n170_security_audit_tests.rs:10:fn n170_audit_gas_exhaustion_pass() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:16:fn n170_audit_state_isolation_pass() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:22:fn n170_audit_determinism_pass() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:28:fn n170_audit_malicious_bytecode_pass() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:34:fn n170_audit_evidence_consistency_pass() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:40:fn n170_full_security_suite() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:4:fn n170_audit_reentrancy_pass() {
crates/amun-contract-upgrade/tests/n174_upgrade_tests.rs:5:use amun_resource_core::ResourceRegistry;
crates/amun-contract-upgrade/tests/n174_upgrade_tests.rs:8:fn n174_upgrade_contract_success() {
crates/amun-contract-upgrade/tests/n174_upgrade_tests.rs:9:    let mut reg = ResourceRegistry::new(100);
crates/amun-core-optimization/tests/n161_optimization_tests.rs:10:fn n161_compare_cached_vs_uncached() {
crates/amun-core-optimization/tests/n161_optimization_tests.rs:11:    let mut reg = ResourceRegistry::new(20000);
crates/amun-core-optimization/tests/n161_optimization_tests.rs:21:                lineage: ResourceLineage::genesis(col_id),
crates/amun-core-optimization/tests/n161_optimization_tests.rs:36:            lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
crates/amun-core-optimization/tests/n161_optimization_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-core-optimization/tests/n161_optimization_tests.rs:48:    let _root1 = reg.compute_state_root();
crates/amun-core-optimization/tests/n161_optimization_tests.rs:52:    let _root2 = opt_reg.compute_state_root();
crates/amun-cross-contract/src/transfer_proof.rs:9:    /// Unique identifier for this proof — prevents replay.
crates/amun-crypto-hardening/src/anti_replay.rs:11:impl AntiReplayGuard {
crates/amun-crypto-hardening/src/anti_replay.rs:12:    pub fn new(max_capacity: usize) -> Self {
crates/amun-crypto-hardening/src/anti_replay.rs:21:    /// If seen, return false (replay detected).
crates/amun-crypto-hardening/src/anti_replay.rs:22:    pub fn check_and_record(&mut self, hash: &[u8; 32]) -> bool {
crates/amun-crypto-hardening/src/anti_replay.rs:24:            return false; // replay detected
crates/amun-crypto-hardening/src/anti_replay.rs:3:/// Anti-replay protection using a sliding window of seen message hashes.
crates/amun-crypto-hardening/src/anti_replay.rs:43:    pub fn tracked_count(&self) -> usize {
crates/amun-crypto-hardening/src/anti_replay.rs:53:    fn n58_allow_first_use() {
crates/amun-crypto-hardening/src/anti_replay.rs:54:        let mut guard = AntiReplayGuard::new(100);
crates/amun-crypto-hardening/src/anti_replay.rs:59:    fn n58_detect_replay() {
crates/amun-crypto-hardening/src/anti_replay.rs:60:        let mut guard = AntiReplayGuard::new(100);
crates/amun-crypto-hardening/src/anti_replay.rs:63:        assert!(!guard.check_and_record(&hash)); // replay
crates/amun-crypto-hardening/src/anti_replay.rs:67:    fn n58_capacity_management() {
crates/amun-crypto-hardening/src/anti_replay.rs:68:        let mut guard = AntiReplayGuard::new(10);
crates/amun-crypto-hardening/src/anti_replay.rs:6:pub struct AntiReplayGuard {
crates/amun-crypto-hardening/src/key_rotation.rs:119:    fn n58_single_key_rotation() {
crates/amun-crypto-hardening/src/key_rotation.rs:130:        assert!(cert.verify());
crates/amun-crypto-hardening/src/key_rotation.rs:134:    fn n58_rotation_chain() {
crates/amun-crypto-hardening/src/key_rotation.rs:160:        assert!(chain.verify_chain());
crates/amun-crypto-hardening/src/key_rotation.rs:164:    fn n58_reject_wrong_sequence() {
crates/amun-crypto-hardening/src/lib.rs:32:pub mod anti_replay;
crates/amun-crypto-hardening/src/lib.rs:36:pub use anti_replay::*;
crates/amun-crypto-hardening/src/production_keys.rs:60:    fn n58_generate_and_sign() {
crates/amun-crypto-hardening/src/production_keys.rs:65:        assert!(pubkey.verify(msg, &sig));
crates/amun-crypto-hardening/src/production_keys.rs:69:    fn n58_tampered_signature_rejected() {
crates/amun-crypto-hardening/src/production_keys.rs:75:        assert!(!pubkey.verify(msg, &sig));
crates/amun-crypto-hardening/src/production_keys.rs:79:    fn n58_different_message_rejected() {
crates/amun-crypto-hardening/src/production_keys.rs:83:        assert!(!pubkey.verify(b"message B", &sig));
crates/amun-defi-amm/tests/n153_amm_tests.rs:19:fn n153_pool_evidence_root_deterministic() {
crates/amun-defi-amm/tests/n153_amm_tests.rs:1:use amun_defi_amm::AmmEngine;
crates/amun-defi-amm/tests/n153_amm_tests.rs:20:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-defi-amm/tests/n153_amm_tests.rs:21:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-defi-amm/tests/n153_amm_tests.rs:22:    let mut amm1 = AmmEngine::new();
crates/amun-defi-amm/tests/n153_amm_tests.rs:23:    let mut amm2 = AmmEngine::new();
crates/amun-defi-amm/tests/n153_amm_tests.rs:2:use amun_resource_core::ResourceRegistry;
crates/amun-defi-amm/tests/n153_amm_tests.rs:32:    assert_eq!(amm1.compute_evidence_root(), amm2.compute_evidence_root());
crates/amun-defi-amm/tests/n153_amm_tests.rs:36:fn n153_swap_changes_evidence_root() {
crates/amun-defi-amm/tests/n153_amm_tests.rs:37:    let mut reg = ResourceRegistry::new(100);
crates/amun-defi-amm/tests/n153_amm_tests.rs:38:    let mut amm = AmmEngine::new();
crates/amun-defi-amm/tests/n153_amm_tests.rs:43:    let root_before = amm.compute_evidence_root();
crates/amun-defi-amm/tests/n153_amm_tests.rs:45:    let root_after = amm.compute_evidence_root();
crates/amun-defi-amm/tests/n153_amm_tests.rs:5:fn n153_create_pool_and_swap() {
crates/amun-defi-amm/tests/n153_amm_tests.rs:6:    let mut reg = ResourceRegistry::new(100);
crates/amun-defi-amm/tests/n153_amm_tests.rs:7:    let mut amm = AmmEngine::new();
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:11:fn n153_swap_evidence_differs_by_amount() {
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:12:    let e1 = DefiEvidence::generate_swap_evidence([1u8; 32], [10u8; 32], 100, 90, 42);
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:13:    let e2 = DefiEvidence::generate_swap_evidence([1u8; 32], [10u8; 32], 200, 180, 42);
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:18:fn n153_liquidity_evidence_deterministic() {
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:19:    let e1 = DefiEvidence::generate_liquidity_evidence([2u8; 32], [20u8; 32], 500, 500, 1000, 10);
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:1:use amun_defi_evidence::DefiEvidence;
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:20:    let e2 = DefiEvidence::generate_liquidity_evidence([2u8; 32], [20u8; 32], 500, 500, 1000, 10);
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:4:fn n153_swap_evidence_deterministic() {
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:5:    let e1 = DefiEvidence::generate_swap_evidence([1u8; 32], [10u8; 32], 100, 90, 42);
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:6:    let e2 = DefiEvidence::generate_swap_evidence([1u8; 32], [10u8; 32], 100, 90, 42);
crates/amun-defi-governance/tests/n157_governance_tests.rs:10:    assert!(engine.execute(&prop_id));
crates/amun-defi-governance/tests/n157_governance_tests.rs:15:fn n157_governance_root_deterministic() {
crates/amun-defi-governance/tests/n157_governance_tests.rs:16:    let mut engine1 = GovernanceEngine::new();
crates/amun-defi-governance/tests/n157_governance_tests.rs:17:    let mut engine2 = GovernanceEngine::new();
crates/amun-defi-governance/tests/n157_governance_tests.rs:1:use amun_defi_governance::GovernanceEngine;
crates/amun-defi-governance/tests/n157_governance_tests.rs:22:    engine1.execute(&id1);
crates/amun-defi-governance/tests/n157_governance_tests.rs:23:    engine2.execute(&id2);
crates/amun-defi-governance/tests/n157_governance_tests.rs:25:        engine1.compute_governance_root(),
crates/amun-defi-governance/tests/n157_governance_tests.rs:26:        engine2.compute_governance_root()
crates/amun-defi-governance/tests/n157_governance_tests.rs:31:fn n157_parameter_change_detected() {
crates/amun-defi-governance/tests/n157_governance_tests.rs:32:    let mut engine = GovernanceEngine::new();
crates/amun-defi-governance/tests/n157_governance_tests.rs:33:    let root_before = engine.compute_governance_root();
crates/amun-defi-governance/tests/n157_governance_tests.rs:36:    engine.execute(&prop_id);
crates/amun-defi-governance/tests/n157_governance_tests.rs:37:    let root_after = engine.compute_governance_root();
crates/amun-defi-governance/tests/n157_governance_tests.rs:4:fn n157_propose_vote_and_execute() {
crates/amun-defi-governance/tests/n157_governance_tests.rs:5:    let mut engine = GovernanceEngine::new();
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:109:        engine1.compute_lending_root(),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:10:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:110:        engine2.compute_lending_root()
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:25:fn n154_interest_accrual_increases_debt() {
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:26:    let mut engine = LendingEngine::new();
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:2:use amun_defi_lending_engine::LendingEngine;
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:30:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:44:fn n154_liquidation_triggered_when_health_factor_low() {
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:45:    let mut engine = LendingEngine::new();
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:49:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:5:fn n154_loan_creation_and_repayment() {
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:63:fn n154_full_repayment_closes_loan() {
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:64:    let mut engine = LendingEngine::new();
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:68:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:6:    let mut engine = LendingEngine::new();
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:83:fn n154_lending_root_deterministic() {
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:84:    let mut engine1 = LendingEngine::new();
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:85:    let mut engine2 = LendingEngine::new();
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:88:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:99:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:16:fn n155_cannot_mint_above_collateral_ratio() {
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:17:    let mut reg = ResourceRegistry::new(10);
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:18:    let mut engine = StablecoinEngine::new();
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:1:use amun_defi_stablecoin::StablecoinEngine;
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:24:fn n155_stablecoin_root_deterministic() {
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:25:    let mut reg1 = ResourceRegistry::new(10);
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:26:    let mut reg2 = ResourceRegistry::new(10);
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:27:    let mut engine1 = StablecoinEngine::new();
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:28:    let mut engine2 = StablecoinEngine::new();
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:2:use amun_resource_core::ResourceRegistry;
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:32:        engine1.compute_stablecoin_root(),
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:33:        engine2.compute_stablecoin_root()
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:5:fn n155_mint_and_burn() {
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:6:    let mut reg = ResourceRegistry::new(10);
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:7:    let mut engine = StablecoinEngine::new();
crates/amun-defi-stress/src/lib.rs:102:pub fn stress_stablecoin_mint_burn(iterations: u64) -> DefiStressResult {
crates/amun-defi-stress/src/lib.rs:106:        let mut reg = ResourceRegistry::new(100);
crates/amun-defi-stress/src/lib.rs:107:        let mut engine = StablecoinEngine::new();
crates/amun-defi-stress/src/lib.rs:112:            let root_before = engine.compute_stablecoin_root();
crates/amun-defi-stress/src/lib.rs:115:            let root_after = engine.compute_stablecoin_root();
crates/amun-defi-stress/src/lib.rs:11:pub struct DefiStressResult {
crates/amun-defi-stress/src/lib.rs:126:pub fn stress_nft_collateral_flow(iterations: u64) -> DefiStressResult {
crates/amun-defi-stress/src/lib.rs:130:        let mut reg = ResourceRegistry::new(100);
crates/amun-defi-stress/src/lib.rs:137:            lineage: ResourceLineage::genesis(token_id),
crates/amun-defi-stress/src/lib.rs:142:        let mut engine = NftCollateralEngine::new();
crates/amun-defi-stress/src/lib.rs:18:impl Default for DefiStressResult {
crates/amun-defi-stress/src/lib.rs:19:    fn default() -> Self {
crates/amun-defi-stress/src/lib.rs:1:use amun_defi_amm::AmmEngine;
crates/amun-defi-stress/src/lib.rs:24:impl DefiStressResult {
crates/amun-defi-stress/src/lib.rs:25:    pub fn new() -> Self {
crates/amun-defi-stress/src/lib.rs:2:use amun_defi_lending_engine::LendingEngine;
crates/amun-defi-stress/src/lib.rs:34:    pub fn passed(&self) -> bool {
crates/amun-defi-stress/src/lib.rs:39:pub fn stress_amm_swaps(iterations: u64) -> DefiStressResult {
crates/amun-defi-stress/src/lib.rs:3:use amun_defi_stablecoin::StablecoinEngine;
crates/amun-defi-stress/src/lib.rs:43:        let mut reg = ResourceRegistry::new(1000);
crates/amun-defi-stress/src/lib.rs:44:        let mut amm = AmmEngine::new();
crates/amun-defi-stress/src/lib.rs:4:use amun_nft_collateral::NftCollateralEngine;
crates/amun-defi-stress/src/lib.rs:52:            let root_before = amm.compute_evidence_root();
crates/amun-defi-stress/src/lib.rs:54:            let root_after = amm.compute_evidence_root();
crates/amun-defi-stress/src/lib.rs:69:pub fn stress_lending_liquidations(iterations: u64) -> DefiStressResult {
crates/amun-defi-stress/src/lib.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-defi-stress/src/lib.rs:73:        let mut reg = ResourceRegistry::new(1000);
crates/amun-defi-stress/src/lib.rs:74:        let mut engine = LendingEngine::new();
crates/amun-defi-stress/tests/n158_stress_tests.rs:15:fn n158_stress_lending_liquidations_500() {
crates/amun-defi-stress/tests/n158_stress_tests.rs:26:fn n158_stress_stablecoin_mint_burn_1000() {
crates/amun-defi-stress/tests/n158_stress_tests.rs:37:fn n158_stress_nft_collateral_flow_100() {
crates/amun-defi-stress/tests/n158_stress_tests.rs:48:fn n158_full_defi_integration_stress() {
crates/amun-defi-stress/tests/n158_stress_tests.rs:4:fn n158_stress_amm_swaps_1000() {
crates/amun-entropy-transcript/src/source.rs:10:    Replay,
crates/amun-entropy-transcript/src/transcript.rs:33:    pub fn verify_replay(&self, other: &EntropyTranscript) -> bool {
crates/amun-evidence-engine/src/evidence_archive.rs:6:/// In production, this integrates with the full N47 EvidenceArchive.
crates/amun-evidence-finality/src/evidence_finality.rs:101:            replay_root: block.replay_root,
crates/amun-evidence-finality/src/evidence_finality.rs:117:        hasher.update(&self.replay_root);
crates/amun-evidence-finality/src/evidence_finality.rs:12:/// A fully verified block with execution, proof, replay, and evidence roots.
crates/amun-evidence-finality/src/evidence_finality.rs:135:            &self.replay_root,
crates/amun-evidence-finality/src/evidence_finality.rs:158:        let mut replay_hashes = Vec::new();
crates/amun-evidence-finality/src/evidence_finality.rs:184:            let replay = ReplayVerifier::replay(&proof, program, &mut fresh_reg, &[]);
crates/amun-evidence-finality/src/evidence_finality.rs:186:            let verified = matches!(replay, ReplayResult::Match { .. });
crates/amun-evidence-finality/src/evidence_finality.rs:187:            replay_hashes.push(if verified {
crates/amun-evidence-finality/src/evidence_finality.rs:19:    pub replay_root: [u8; 32],
crates/amun-evidence-finality/src/evidence_finality.rs:204:        let replay_root = Self::hash_list(b"AMUN_REPLAY_ROOT_V1", &replay_hashes);
crates/amun-evidence-finality/src/evidence_finality.rs:211:            &replay_root,
crates/amun-evidence-finality/src/evidence_finality.rs:220:            replay_root,
crates/amun-evidence-finality/src/evidence_finality.rs:233:            return Err("Not all transitions passed replay verification".into());
crates/amun-evidence-finality/src/evidence_finality.rs:240:            replay_root: block.replay_root,
crates/amun-evidence-finality/src/evidence_finality.rs:307:        assert!(cert.verify());
crates/amun-evidence-finality/src/evidence_finality.rs:337:        assert_eq!(rr, &cert.replay_root);
crates/amun-evidence-finality/src/evidence_finality.rs:43:        replay_root: &[u8; 32],
crates/amun-evidence-finality/src/evidence_finality.rs:52:        hasher.update(replay_root);
crates/amun-evidence-finality/src/evidence_finality.rs:68:    pub replay_root: [u8; 32],
crates/amun-evidence-finality/src/evidence_finality.rs:7:use amun_replay_verifier::replay_verifier::{ReplayResult, ReplayVerifier};
crates/amun-evidence-finality/src/evidence_finality.rs:88:    pub replay_root: [u8; 32],
crates/amun-evidence-root/src/lib.rs:125:    fn n39_genesis_evidence_root() {
crates/amun-evidence-root/src/lib.rs:127:        assert!(genesis.verify());
crates/amun-evidence-root/src/lib.rs:132:    fn n39_evidence_root_deterministic() {
crates/amun-evidence-root/src/lib.rs:139:    fn n39_different_state_different_root() {
crates/amun-evidence-root/src/lib.rs:146:    fn n39_evidence_chain_continuity() {
crates/amun-evidence-root/src/lib.rs:151:        assert!(chain.verify());
crates/amun-evidence-root/src/lib.rs:156:    fn n39_broken_chain_detected() {
crates/amun-evidence-root/src/lib.rs:162:        assert!(!chain.verify());
crates/amun-evidence-root/src/lib.rs:16:    pub replay_certificate: [u8; 32],
crates/amun-evidence-root/src/lib.rs:26:        replay_certificate: [u8; 32],
crates/amun-evidence-root/src/lib.rs:35:        hasher.update(&replay_certificate);
crates/amun-evidence-root/src/lib.rs:44:            replay_certificate,
crates/amun-evidence-root/src/lib.rs:60:            self.replay_certificate,
crates/amun-evidence-root/src/lib.rs:84:        replay_certificate: [u8; 32],
crates/amun-evidence-root/src/lib.rs:8:/// - Replay certificate (replay verification proof)
crates/amun-evidence-root/src/lib.rs:92:            replay_certificate,
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:100:    let e1 = EvidenceRoot::compute(before, [0u8; 32], [0u8; 32], [0u8; 32], [0u8; 32], 1);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:101:    let e2 = EvidenceRoot::compute(after, [0u8; 32], [0u8; 32], [0u8; 32], [0u8; 32], 1);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:13:        sk.verifying_key().to_bytes()
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:15:    builder.engine.state.create_account(alice, 1000);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:1:use amun_block_builder::BlockBuilder;
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:29:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:2:use amun_evidence_root::{EvidenceChain, EvidenceRoot};
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:30:    let evidence = EvidenceRoot::compute(
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:38:    assert!(evidence.verify());
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:44:fn n40_evidence_chain_across_blocks() {
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:45:    let mut builder = BlockBuilder::new();
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:49:        sk.verifying_key().to_bytes()
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:51:    builder.engine.state.create_account(alice, 1000);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:52:    let mut chain = EvidenceChain::new();
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:68:        let b = builder.build_block(i, parent, &mut mempool, 100, [9u8; 32], i * 1000);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:71:    assert!(chain.verify());
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:76:fn n40_evidence_root_changes_with_state() {
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:77:    let mut builder = BlockBuilder::new();
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:81:        sk.verifying_key().to_bytes()
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:83:    builder.engine.state.create_account(alice, 1000);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:84:    let before = builder.engine.state.state_root();
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:8:fn n40_evidence_backed_block_created() {
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:98:    builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:99:    let after = builder.engine.state.state_root();
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:9:    let mut builder = BlockBuilder::new();
crates/amun-evidence/src/tests.rs:61:        assert!(evidence.verify().is_err());
crates/amun-evolution/src/certificate.rs:111:            replay_guarantee: replay,
crates/amun-evolution/src/certificate.rs:137:        h.update(&[self.replay_guarantee.canonical_tag()]);
crates/amun-evolution/src/certificate.rs:13:    pub preserves_replay_determinism: bool,
crates/amun-evolution/src/certificate.rs:23:        preserves_replay_determinism: bool,
crates/amun-evolution/src/certificate.rs:30:            preserves_replay_determinism,
crates/amun-evolution/src/certificate.rs:3:    ContinuityClass, GovernanceGuarantee, ProofGuarantee, ReplayGuarantee, SnapshotGuarantee,
crates/amun-evolution/src/certificate.rs:43:        h.update(&[self.preserves_replay_determinism as u8]);
crates/amun-evolution/src/certificate.rs:67:    pub replay_guarantee: ReplayGuarantee,
crates/amun-evolution/src/certificate.rs:91:        replay: ReplayGuarantee,
crates/amun-evolution/src/executor.rs:28:    ReplayBreak,
crates/amun-evolution/src/signatures.rs:2:// Defines signing domains, aggregation rules, and anti-replay protection.
crates/amun-evolution/src/validator.rs:13:        proof.replay_guarantee == certificate.replay_guarantee
crates/amun-evolution/src/validator.rs:22:        proof.replay_guarantee.rank() >= ReplayGuarantee::Deterministic.rank()
crates/amun-evolution/src/validator.rs:3:    EvolutionProof, GovernanceGuarantee, ReplayGuarantee, SnapshotGuarantee,
crates/amun-execution/src/lib.rs:154:    fn n26_transfer_success() {
crates/amun-execution/src/lib.rs:171:    fn n26_insufficient_balance() {
crates/amun-execution/src/lib.rs:187:    fn n26_wrong_nonce() {
crates/amun-execution/src/lib.rs:202:    fn n26_invalid_signature() {
crates/amun-execution/src/lib.rs:218:    fn n26_state_root_changes() {
crates/amun-execution/src/lib.rs:234:    fn n26_execute_block() {
crates/amun-execution/src/tests.rs:33:    assert!(profile.verify_module(b"").is_ok());
crates/amun-execution/src/tests.rs:52:    assert!(wasm_deterministic_subset::verify_deterministic_wasm(b"").is_ok());
crates/amun-execution/src/tests.rs:59:    assert!(interpreter.execute(b"", "", &[]).is_ok());
crates/amun-experimental-framework/src/main.rs:104:                lineage: ResourceLineage::single_ancestor(child, parent, hash, version),
crates/amun-experimental-framework/src/main.rs:116:fn workload_halt() -> ConstitutionalProgram {
crates/amun-experimental-framework/src/main.rs:117:    ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt])
crates/amun-experimental-framework/src/main.rs:120:/// Workload B: Push10 — compute overhead.
crates/amun-experimental-framework/src/main.rs:121:fn workload_push10() -> ConstitutionalProgram {
crates/amun-experimental-framework/src/main.rs:122:    ConstitutionalProgram::new(
crates/amun-experimental-framework/src/main.rs:143:fn workload_transform() -> ConstitutionalProgram {
crates/amun-experimental-framework/src/main.rs:144:    ConstitutionalProgram::new(
crates/amun-experimental-framework/src/main.rs:14:fn make_id(seed: u64) -> ResourceId {
crates/amun-experimental-framework/src/main.rs:159:fn workload_split() -> ConstitutionalProgram {
crates/amun-experimental-framework/src/main.rs:160:    ConstitutionalProgram::new(
crates/amun-experimental-framework/src/main.rs:176:fn exp1_state_scale() {
crates/amun-experimental-framework/src/main.rs:183:        let mut reg = build_registry(size);
crates/amun-experimental-framework/src/main.rs:190:            pre_state_root: reg.compute_state_root(),
crates/amun-experimental-framework/src/main.rs:193:        let mut hot = HotProofStore::new(1000);
crates/amun-experimental-framework/src/main.rs:194:        let mut archive = ProofArchive::new();
crates/amun-experimental-framework/src/main.rs:195:        let result = ConstitutionalRuntime::execute(
crates/amun-experimental-framework/src/main.rs:207:                transition_proof, ..
crates/amun-experimental-framework/src/main.rs:208:            } => transition_proof,
crates/amun-experimental-framework/src/main.rs:20:struct Stats {
crates/amun-experimental-framework/src/main.rs:212:        let stats = measure_us(&format!("replay_{}_active", size), 5, 30, || {
crates/amun-experimental-framework/src/main.rs:213:            let mut fresh = ResourceRegistry::new((size * 2) as usize);
crates/amun-experimental-framework/src/main.rs:214:            ReplayVerifier::replay(&proof, &program, &mut fresh, &[]);
crates/amun-experimental-framework/src/main.rs:224:        &["active_resources", "replay_time_us", "ci95_us"],
crates/amun-experimental-framework/src/main.rs:229:// ── Experiment 2: Replay vs Execution (all workloads) ───────
crates/amun-experimental-framework/src/main.rs:230:fn exp2_replay_vs_execute() {
crates/amun-experimental-framework/src/main.rs:231:    println!("\n=== Experiment 2: Replay vs Execution ===");
crates/amun-experimental-framework/src/main.rs:232:    let workloads: Vec<(&str, ConstitutionalProgram)> = vec![
crates/amun-experimental-framework/src/main.rs:242:        let mut reg = build_registry(size);
crates/amun-experimental-framework/src/main.rs:249:            pre_state_root: reg.compute_state_root(),
crates/amun-experimental-framework/src/main.rs:254:        let mut hot = HotProofStore::new(1000);
crates/amun-experimental-framework/src/main.rs:255:        let mut archive = ProofArchive::new();
crates/amun-experimental-framework/src/main.rs:256:        let exec_stats = measure_us(&format!("execute_{}", name), 5, 30, || {
crates/amun-experimental-framework/src/main.rs:260:            match ConstitutionalRuntime::execute(
crates/amun-experimental-framework/src/main.rs:26:fn compute_stats(times: &[f64]) -> Stats {
crates/amun-experimental-framework/src/main.rs:277:        let result = ConstitutionalRuntime::execute(
crates/amun-experimental-framework/src/main.rs:288:                transition_proof, ..
crates/amun-experimental-framework/src/main.rs:289:            }) => transition_proof,
crates/amun-experimental-framework/src/main.rs:291:                transition_proof, ..
crates/amun-experimental-framework/src/main.rs:294:                transition_proof
crates/amun-experimental-framework/src/main.rs:2:use amun_bytecode::program::ConstitutionalProgram;
crates/amun-experimental-framework/src/main.rs:308:        // Measure replay
crates/amun-experimental-framework/src/main.rs:309:        let replay_stats = measure_us(&format!("replay_{}", name), 5, 30, || {
crates/amun-experimental-framework/src/main.rs:310:            let mut fresh = ResourceRegistry::new((size * 2) as usize);
crates/amun-experimental-framework/src/main.rs:311:            ReplayVerifier::replay(&proof, program, &mut fresh, &[]);
crates/amun-experimental-framework/src/main.rs:314:        let speedup = exec_stats.mean / replay_stats.mean;
crates/amun-experimental-framework/src/main.rs:319:            format!("{:.4}", replay_stats.mean),
crates/amun-experimental-framework/src/main.rs:325:        &["workload", "execution_us", "replay_us", "speedup"],
crates/amun-experimental-framework/src/main.rs:331:fn exp3_full_pipeline() {
crates/amun-experimental-framework/src/main.rs:339:            let mut reg = ResourceRegistry::new((n * 10) as usize);
crates/amun-experimental-framework/src/main.rs:340:            let mut hot = HotProofStore::new(10000);
crates/amun-experimental-framework/src/main.rs:341:            let mut archive = ProofArchive::new();
crates/amun-experimental-framework/src/main.rs:353:                    pre_state_root: reg.compute_state_root(),
crates/amun-experimental-framework/src/main.rs:356:                let result = ConstitutionalRuntime::execute(
crates/amun-experimental-framework/src/main.rs:35:fn measure_us(name: &str, warmup: u32, iterations: u32, mut f: impl FnMut()) -> Stats {
crates/amun-experimental-framework/src/main.rs:367:                    transition_proof, ..
crates/amun-experimental-framework/src/main.rs:370:                    let mut fresh = ResourceRegistry::new(10000);
crates/amun-experimental-framework/src/main.rs:371:                    ReplayVerifier::replay(&transition_proof, &program, &mut fresh, &[]);
crates/amun-experimental-framework/src/main.rs:389:fn exp4_cycle_detection() {
crates/amun-experimental-framework/src/main.rs:396:            let (mut reg, tip) = build_deep_chain(depth);
crates/amun-experimental-framework/src/main.rs:398:            let tip_hash = ResourceRegistry::hash_resource(tip_meta);
crates/amun-experimental-framework/src/main.rs:3:use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
crates/amun-experimental-framework/src/main.rs:407:                    lineage: ResourceLineage::single_ancestor(new_id, tip, tip_hash, version),
crates/amun-experimental-framework/src/main.rs:426:// ── Experiment 5: Witness Bundle Size ───────────────────────
crates/amun-experimental-framework/src/main.rs:427:fn exp5_witness_size() {
crates/amun-experimental-framework/src/main.rs:428:    println!("\n=== Experiment 5: Witness Bundle Size ===");
crates/amun-experimental-framework/src/main.rs:430:    let workloads: Vec<(&str, ConstitutionalProgram)> = vec![
crates/amun-experimental-framework/src/main.rs:438:            let mut reg = build_registry(size);
crates/amun-experimental-framework/src/main.rs:445:                pre_state_root: reg.compute_state_root(),
crates/amun-experimental-framework/src/main.rs:448:            let mut hot = HotProofStore::new(1000);
crates/amun-experimental-framework/src/main.rs:449:            let mut archive = ProofArchive::new();
crates/amun-experimental-framework/src/main.rs:450:            let result = ConstitutionalRuntime::execute(
crates/amun-experimental-framework/src/main.rs:462:                    transition_proof, ..
crates/amun-experimental-framework/src/main.rs:463:                } => transition_proof,
crates/amun-experimental-framework/src/main.rs:46:    let s = compute_stats(&times);
crates/amun-experimental-framework/src/main.rs:483:fn exp6_law_verification() {
crates/amun-experimental-framework/src/main.rs:489:        let stats = measure_us(&format!("verify_{}_resources", size), 5, 30, || {
crates/amun-experimental-framework/src/main.rs:490:            let reg = build_registry(size);
crates/amun-experimental-framework/src/main.rs:491:            let _root = reg.compute_state_root();
crates/amun-experimental-framework/src/main.rs:4:use amun_proof_archive::hot_store::HotProofStore;
crates/amun-experimental-framework/src/main.rs:507:fn main() {
crates/amun-experimental-framework/src/main.rs:511:    exp2_replay_vs_execute();
crates/amun-experimental-framework/src/main.rs:54:fn write_csv(filename: &str, header: &[&str], rows: &[Vec<String>]) {
crates/amun-experimental-framework/src/main.rs:5:use amun_proof_archive::proof_archive::ProofArchive;
crates/amun-experimental-framework/src/main.rs:65:fn build_registry(size: u64) -> ResourceRegistry {
crates/amun-experimental-framework/src/main.rs:66:    let mut reg = ResourceRegistry::new((size * 2) as usize);
crates/amun-experimental-framework/src/main.rs:6:use amun_replay_verifier::replay_verifier::ReplayVerifier;
crates/amun-experimental-framework/src/main.rs:72:            lineage: ResourceLineage::genesis(make_id(i)),
crates/amun-experimental-framework/src/main.rs:81:fn build_deep_chain(depth: u64) -> (ResourceRegistry, ResourceId) {
crates/amun-experimental-framework/src/main.rs:82:    let mut reg = ResourceRegistry::new((depth * 2) as usize);
crates/amun-experimental-framework/src/main.rs:88:        lineage: ResourceLineage::genesis(root),
crates/amun-experimental-framework/src/main.rs:8:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-experimental-framework/src/main.rs:96:        let hash = ResourceRegistry::hash_resource(reg.get(&parent).unwrap());
crates/amun-explorer-api/src/services/chain_service.rs:40:            has_replay_evidence: true,
crates/amun-explorer-api/src/services/chain_service.rs:56:            has_replay_evidence: true,
crates/amun-explorer-api/src/services/constitutional_service.rs:68:                evidence_type: "ReplayEvidence".into(),
crates/amun-explorer-api/src/types.rs:12:    pub has_replay_evidence: bool,
crates/amun-failure/src/taxonomy.rs:17:    ReplayViolation = 0x3002,
crates/amun-failure/src/taxonomy.rs:55:            | Self::ReplayViolation
crates/amun-failure/src/tests.rs:10:    assert!(ConstitutionalFault::ConstitutionalViolation.should_halt());
crates/amun-failure/src/tests.rs:11:    assert!(ConstitutionalFault::InvalidQuorum.should_halt());
crates/amun-failure/src/tests.rs:12:    assert!(ConstitutionalFault::SignatureInvalid.should_halt());
crates/amun-failure/src/tests.rs:13:    assert!(ConstitutionalFault::MerkleProofInvalid.should_halt());
crates/amun-failure/src/tests.rs:14:    assert!(ConstitutionalFault::DurabilityViolation.should_halt());
crates/amun-failure/src/tests.rs:15:    assert!(ConstitutionalFault::JournalHashMismatch.should_halt());
crates/amun-failure/src/tests.rs:16:    assert!(ConstitutionalFault::ArithmeticOverflow.should_halt());
crates/amun-failure/src/tests.rs:17:    assert!(ConstitutionalFault::ArithmeticUnderflow.should_halt());
crates/amun-failure/src/tests.rs:18:    assert!(ConstitutionalFault::DecodeBudgetExceeded.should_halt());
crates/amun-failure/src/tests.rs:19:    assert!(ConstitutionalFault::CryptoBudgetExceeded.should_halt());
crates/amun-failure/src/tests.rs:24:    assert!(!ConstitutionalFault::BufferTooSmall.should_halt());
crates/amun-failure/src/tests.rs:25:    assert!(!ConstitutionalFault::CapacityExceeded.should_halt());
crates/amun-failure/src/tests.rs:26:    assert!(!ConstitutionalFault::TableFull.should_halt());
crates/amun-failure/src/tests.rs:27:    assert!(!ConstitutionalFault::MemoryBudgetExhausted.should_halt());
crates/amun-failure/src/tests.rs:32:    assert!(!ConstitutionalFault::InvalidStateTransition.should_halt());
crates/amun-failure/src/tests.rs:33:    assert!(!ConstitutionalFault::UninitializedAccess.should_halt());
crates/amun-failure/src/tests.rs:34:    assert!(!ConstitutionalFault::DoubleInitialization.should_halt());
crates/amun-failure/src/tests.rs:35:    assert!(!ConstitutionalFault::TemporalViolation.should_halt());
crates/amun-failure/src/tests.rs:36:    assert!(!ConstitutionalFault::ReplayViolation.should_halt());
crates/amun-failure/src/tests.rs:37:    assert!(!ConstitutionalFault::SequenceMismatch.should_halt());
crates/amun-failure/src/tests.rs:47:    assert_eq!(ctx.fault, ConstitutionalFault::CapacityExceeded);
crates/amun-failure/src/tests.rs:83:        assert_eq!(fault, ConstitutionalFault::EquivocationDetected);
crates/amun-failure/src/tests.rs:8:    assert!(ConstitutionalFault::EquivocationDetected.should_halt());
crates/amun-failure/src/tests.rs:96:    assert!(actions.invalidate_snapshots);
crates/amun-failure/src/tests.rs:9:    assert!(ConstitutionalFault::UnsafeContractViolation.should_halt());
crates/amun-finality-certificate/src/lib.rs:125:    fn n41_single_finality_certificate() {
crates/amun-finality-certificate/src/lib.rs:127:        assert!(cert.verify());
crates/amun-finality-certificate/src/lib.rs:132:    fn n41_hash_deterministic() {
crates/amun-finality-certificate/src/lib.rs:139:    fn n41_tampered_evidence_rejected() {
crates/amun-finality-certificate/src/lib.rs:142:        assert!(!cert.verify());
crates/amun-finality-certificate/src/lib.rs:146:    fn n41_zero_block_hash_rejected() {
crates/amun-finality-certificate/src/lib.rs:149:        assert!(!cert.verify());
crates/amun-finality-certificate/src/lib.rs:153:    fn n41_finality_chain_continuity() {
crates/amun-finality-certificate/src/lib.rs:15:    pub replay_certificate_head: [u8; 32],
crates/amun-finality-certificate/src/lib.rs:28:        replay_head: [u8; 32],
crates/amun-finality-certificate/src/lib.rs:39:            replay_certificate_head: replay_head,
crates/amun-finality-certificate/src/lib.rs:55:        hasher.update(&self.replay_certificate_head);
crates/amun-finality-certificate/src/lib.rs:8:/// with the complete evidence chain (replay, audit, evidence root).
crates/amun-finality-law/src/finality.rs:65:        assert!(is_finalized_simple(&qc_a, &qc_b));
crates/amun-finality-law/src/finality.rs:94:        assert!(!is_finalized_simple(&qc_a, &qc_b));
crates/amun-gas-engine/src/gas_engine.rs:118:        assert!(GasEngine::can_execute(1000, 500));
crates/amun-gas-engine/src/gas_engine.rs:119:        assert!(!GasEngine::can_execute(100, 500));
crates/amun-gas-engine/src/gas_engine.rs:129:        assert!(matches!(result, GasEngineResult::Success { .. }));
crates/amun-gas-engine/src/gas_engine.rs:94:        assert!(matches!(result, GasEngineResult::Success { gas_used: 300 }));
crates/amun-gas-engine/src/opcode_costs.rs:3:/// Constitutional gas cost table as specified in N48.5-E Section 3.3.
crates/amun-governance/src/tests.rs:9:fn test_proposal_passing() {
crates/amun-host/src/lib.rs:4:pub mod replay;
crates/amun-host/src/lib.rs:9:pub use replay::ReplayGuard;
crates/amun-host/src/replay.rs:14:    pub fn check_chain_id(&self, chain_id: u64) -> Result<(), &'static str> {
crates/amun-host/src/replay.rs:19:            return Err("chain ID mismatch - cross-chain replay rejected");
crates/amun-host/src/replay.rs:1:pub struct ReplayGuard {
crates/amun-host/src/replay.rs:6:impl ReplayGuard {
crates/amun-host/src/replay.rs:7:    pub fn new(chain_id: u64) -> Self {
crates/amun-invariant-engine/src/invariant_engine.rs:105:    fn w8_all_invariants_pass() {
crates/amun-invariant-engine/src/invariant_engine.rs:115:        assert!(InvariantEngine::all_passed(&results));
crates/amun-invariant-engine/src/invariant_engine.rs:116:        assert!(!InvariantEngine::has_critical_failure(&results));
crates/amun-invariant-engine/src/invariant_engine.rs:131:        assert!(!InvariantEngine::all_passed(&results));
crates/amun-invariant-engine/src/invariant_engine.rs:132:        assert!(InvariantEngine::has_critical_failure(&results));
crates/amun-invariant-engine/src/invariant_engine.rs:153:        assert!(!InvariantEngine::all_passed(&results));
crates/amun-invariant-engine/src/invariant_engine.rs:154:        assert!(!InvariantEngine::has_critical_failure(&results)); // No Critical failed
crates/amun-invariant-engine/src/invariant_engine.rs:60:            .any(|r| r.severity == InvariantSeverity::Critical && !r.passed)
crates/amun-invariant-engine/src/invariant_engine.rs:64:    pub fn all_passed(results: &[InvariantResult]) -> bool {
crates/amun-invariant-engine/src/invariant_engine.rs:6:/// Evaluates contract invariants after commit (Phase 5 in N48.5-D).
crates/amun-invariants/src/kernel.rs:16:    pub requires_replay_verification: bool,
crates/amun-invariants/src/kernel.rs:30:        requires_replay_verification: true,
crates/amun-invariants/src/kernel.rs:41:        requires_replay_verification: false,
crates/amun-invariants/src/kernel.rs:52:        requires_replay_verification: false,
crates/amun-invariants/src/kernel.rs:63:        requires_replay_verification: false,
crates/amun-keystore/src/store.rs:41:    pub fn decrypt(&self, password: &str) -> Result<Vec<u8>, &'static str> {
crates/amun-light-client/src/constitutional_client.rs:158:    fn n55_bootstrap_client() {
crates/amun-light-client/src/constitutional_client.rs:178:    fn n55_verify_valid_certificate() {
crates/amun-light-client/src/constitutional_client.rs:181:        assert!(client.verify_certificate(&cert));
crates/amun-light-client/src/constitutional_client.rs:185:    fn n55_reject_tampered_certificate() {
crates/amun-light-client/src/constitutional_client.rs:189:        assert!(!client.verify_certificate(&cert));
crates/amun-light-client/src/constitutional_client.rs:193:    fn n55_verify_chain_extension() {
crates/amun-light-client/src/constitutional_client.rs:210:        assert!(client.verify_chain_extension(&chain));
crates/amun-light-client/src/constitutional_client.rs:214:    fn n55_advance_client_state() {
crates/amun-light-client/tests/light_client_tests.rs:107:    assert!(!client.verify_chain_extension(&chain));
crates/amun-light-client/tests/light_client_tests.rs:58:fn n55_light_client_full_workflow() {
crates/amun-light-client/tests/light_client_tests.rs:61:    let history_root = ConstitutionalHistoryRoot::from_chain(&chain1);
crates/amun-light-client/tests/light_client_tests.rs:87:fn n55_light_client_rejects_broken_chain() {
crates/amun-lineage-law/src/compatibility.rs:13:    pub is_replay_compatible: bool,
crates/amun-lineage-law/src/compatibility.rs:29:        let is_replay = migration_witness
crates/amun-lineage-law/src/compatibility.rs:31:            .map(|w| w.replay_preserved)
crates/amun-lineage-law/src/compatibility.rs:44:            is_replay_compatible: is_replay,
crates/amun-lineage-law/src/compatibility.rs:59:        h.update([self.is_replay_compatible as u8]);
crates/amun-lineage-law/src/compatibility.rs:78:    /// Compatible but replay not preserved
crates/amun-lineage-law/src/compatibility.rs:90:            && theorem.is_replay_compatible
crates/amun-lineage-law/src/migration.rs:107:        requires_replay: bool,
crates/amun-lineage-law/src/migration.rs:114:            requires_replay,
crates/amun-lineage-law/src/migration.rs:128:        h.update([self.requires_replay as u8]);
crates/amun-lineage-law/src/migration.rs:12:    /// Whether replay determinism was preserved during migration
crates/amun-lineage-law/src/migration.rs:13:    pub replay_preserved: bool,
crates/amun-lineage-law/src/migration.rs:26:        replay_preserved: bool,
crates/amun-lineage-law/src/migration.rs:33:            replay_preserved,
crates/amun-lineage-law/src/migration.rs:47:        h.update([self.replay_preserved as u8]);
crates/amun-lineage-law/src/migration.rs:97:    pub requires_replay: bool,
crates/amun-lineage/src/compatibility.rs:19:            CompatibilityClass::ReplayCompatible => 0x04,
crates/amun-lineage/src/compatibility.rs:32:            CompatibilityClass::ReplayCompatible => 4,
crates/amun-lineage/src/compatibility.rs:7:    ReplayCompatible,
crates/amun-lineage/src/lib.rs:13:    ReplayGuarantee, SnapshotGuarantee,
crates/amun-lineage/src/record.rs:14:impl ReplayGuarantee {
crates/amun-lineage/src/record.rs:169:    pub replay_guarantee: ReplayGuarantee,
crates/amun-lineage/src/record.rs:17:            ReplayGuarantee::Exact => 0x03,
crates/amun-lineage/src/record.rs:181:        replay: ReplayGuarantee,
crates/amun-lineage/src/record.rs:18:            ReplayGuarantee::Deterministic => 0x02,
crates/amun-lineage/src/record.rs:190:            replay_guarantee: replay,
crates/amun-lineage/src/record.rs:19:            ReplayGuarantee::Partial => 0x01,
crates/amun-lineage/src/record.rs:205:        h.update(&[self.replay_guarantee.canonical_tag()]);
crates/amun-lineage/src/record.rs:20:            ReplayGuarantee::Unsupported => 0x00,
crates/amun-lineage/src/record.rs:7:pub enum ReplayGuarantee {
crates/amun-live-cluster/src/bin/benchmark.rs:6:fn main() {
crates/amun-live-cluster/src/bin/benchmark_10.rs:6:fn main() {
crates/amun-live-cluster/src/bin/benchmark_10_event.rs:6:fn main() {
crates/amun-live-cluster/src/bin/latency_audit.rs:6:fn main() {
crates/amun-live-cluster/src/bin/rejoin_stress.rs:52:fn main() {
crates/amun-live-cluster/src/bin/rejoin_stress.rs:8:fn sync_to_tip(v: &LiveValidator, peer_addr: SocketAddr) {
crates/amun-live-cluster/src/bin/replay_audit.rs:6:fn main() {
crates/amun-live-cluster/src/validator.rs:1034:            assert_eq!(executed.len(), 1, "RetireAuthority should execute");
crates/amun-live-cluster/src/validator.rs:1046:    fn n108_3e_authority_rotation_certificate_validation() {
crates/amun-live-cluster/src/validator.rs:1200:    fn n71_persist_finalized_blocks() {
crates/amun-live-cluster/src/validator.rs:1214:    fn n71_recover_after_restart() {
crates/amun-live-cluster/src/validator.rs:1239:    fn n69_single_validator_self_finalizes() {
crates/amun-live-cluster/src/validator.rs:1252:    fn n69_two_validators_reach_consensus() {
crates/amun-live-cluster/src/validator.rs:1267:    fn n69_three_of_four_reach_quorum() {
crates/amun-live-cluster/src/validator.rs:1285:    fn n69_duplicate_vote_ignored() {
crates/amun-live-cluster/src/validator.rs:1298:    fn n69_four_validators_full_cluster() {
crates/amun-live-cluster/src/validator.rs:14:        ConstitutionalEvidenceRecord, DoubleSpendEvidence, GovernanceEvidence, ReplayEvidence,
crates/amun-live-cluster/src/validator.rs:418:                    // Recompute hash after setting slashing_root (N120.2 requires it in hash)
crates/amun-live-cluster/src/validator.rs:568:                    // N126.3: Evidence-Based Constitutional Verification
crates/amun-live-cluster/src/validator.rs:58:    /// N110.4c: Staking adapter for applying slashes after finality
crates/amun-live-cluster/src/validator.rs:594:                        // N126.3: Replay determinism from ExecutionEngine
crates/amun-live-cluster/src/validator.rs:596:                        let replay_deterministic = cert.state_root == history_root;
crates/amun-live-cluster/src/validator.rs:598:                        // N126.3: Finality supermajority from QC voting power
crates/amun-live-cluster/src/validator.rs:604:                        // Evidence validity: all certificates pass .verify()
crates/amun-live-cluster/src/validator.rs:618:                            replay_deterministic,
crates/amun-live-cluster/src/validator.rs:63:    /// N123.1: Constitutional enforcement kernel
crates/amun-live-cluster/src/validator.rs:648:                        // N129.2: Build ConstitutionalEvidenceRecord from real evidence data
crates/amun-live-cluster/src/validator.rs:662:                        let rep_ev = ReplayEvidence::new(cert.state_root, history_root);
crates/amun-live-cluster/src/validator.rs:679:                        // N129.3: Compute EvidenceRoot with constitutional continuity
crates/amun-live-cluster/src/validator.rs:684:                            [0u8; 32], // replay_certificate: placeholder until N126.4
crates/amun-live-cluster/src/validator.rs:747:                            // N110.4c.1: Replay protection
crates/amun-live-cluster/src/validator.rs:827:    fn n108_1_governance_updates_live_authority_registry() {
crates/amun-live-cluster/src/validator.rs:85:        // N105.4A: Deterministic key matching committed test certificates
crates/amun-live-cluster/src/validator.rs:875:            assert_eq!(executed.len(), 1, "Proposal should be executed");
crates/amun-live-cluster/src/validator.rs:909:    fn n108_2_runtime_authority_rotation() {
crates/amun-live-cluster/src/validator.rs:947:            assert_eq!(executed.len(), 1, "AddAuthority should execute");
crates/amun-live-cluster/src/validator.rs:979:            assert_eq!(executed.len(), 1, "ScheduleTransition should execute");
crates/amun-live-cluster/tests/n102_catchup_test.rs:8:fn n102_3_catchup_after_50_block_gap() {
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:10:fn make_pk(id: u8) -> PublicKey {
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:122:fn n110_4c_applied_hashes_prevent_replay() {
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:16:fn pk_to_id(pk: &PublicKey) -> [u8; 32] {
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:22:fn make_valid_certificate(validator_id: [u8; 32]) -> SlashingCertificate {
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:27:        vec![EvidenceCount {
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:28:            evidence_type: EvidenceType::DoubleVote,
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:2:    EvidenceCount, EvidenceType, MisbehaviorRegistry, MisbehaviorThresholds, RealStakingExecutor,
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:42:fn n110_4c_slash_applied_after_finality() {
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:50:    misbehavior.record_misbehavior(&vid, &[0x01; 32], &EvidenceType::DoubleVote, 1);
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:51:    misbehavior.record_misbehavior(&vid, &[0x02; 32], &EvidenceType::DoubleVote, 2);
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:52:    misbehavior.record_misbehavior(&vid, &[0x03; 32], &EvidenceType::DoubleVote, 3);
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:70:fn n110_4c_duplicate_certificate_not_reapplied() {
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:78:    misbehavior.record_misbehavior(&vid, &[0x01; 32], &EvidenceType::DoubleVote, 1);
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:79:    misbehavior.record_misbehavior(&vid, &[0x02; 32], &EvidenceType::DoubleVote, 2);
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:80:    misbehavior.record_misbehavior(&vid, &[0x03; 32], &EvidenceType::DoubleVote, 3);
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:98:    // execute another slash. The idempotency protection is at the
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:12:fn n118_slashing_executes_after_finality() {
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:1:// N118 — Finality-Gated Slashing Verification
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:20:    misbehavior.record_misbehavior(&validator_id, &[0xA1; 32], &EvidenceType::DoubleVote, 1);
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:21:    misbehavior.record_misbehavior(&validator_id, &[0xA2; 32], &EvidenceType::DoubleVote, 2);
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:22:    misbehavior.record_misbehavior(&validator_id, &[0xA3; 32], &EvidenceType::DoubleVote, 3);
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:2:// Verifies that slashing executes in the finality path and reduces stake.
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:34:        "N118: Slashing must execute in finality path"
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:41:    eprintln!("N118 PASSED: Slashing executes in finality-gated path");
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:5:    EvidenceType, MisbehaviorRegistry, MisbehaviorThresholds, RealStakingExecutor, StakingAdapter,
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:10:    let mut builder = BlockBuilder::new();
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:12:    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:17:        block.verify_slashing_root(&root).is_ok(),
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:23:fn n120_4b_validator_rejects_mismatched_slashing_root() {
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:24:    let mut builder = BlockBuilder::new();
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:26:    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:2:use amun_block_builder::BlockBuilder;
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:30:    let result = block.verify_slashing_root(&[0xFF; 32]);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:38:fn n120_4c_block_with_tampered_root_rejected() {
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:39:    let mut builder = BlockBuilder::new();
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:41:    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:50:    let result = block.verify_slashing_root(&[0x42; 32]);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:58:fn n120_4d_mismatched_root_prevents_voting() {
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:59:    // Simulates: proposer builds block with root X, validator computes root Y
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:64:    let mut builder = BlockBuilder::new();
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:66:    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:78:        block.verify_slashing_root(&validator_root).is_err(),
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:7:fn n120_4a_validator_accepts_matching_slashing_root() {
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:8:    // Setup: build a block with a known slashing_root
crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:15:        ConstitutionalVerdict::Unconstitutional { violations } => {
crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:19:                    .any(|v| v.law == ConstitutionalLaw::StateRootIntegrity),
crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:1:// N126.3 — Unconstitutional block must be rejected at commit time
crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:28:fn n126_3_finality_missing_supermajority_rejected() {
crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:29:    let mut kernel = ConstitutionalEnforcementKernel::new();
crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:35:        ConstitutionalVerdict::Unconstitutional { violations } => {
crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:39:                    .any(|v| v.law == ConstitutionalLaw::FinalitySupermajority),
crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:3:    ConstitutionalEnforcementKernel, ConstitutionalLaw, ConstitutionalVerdict,
crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:40:                "Must detect FinalitySupermajority violation"
crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:48:fn n126_3_constitutional_block_accepted() {
crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:49:    let mut kernel = ConstitutionalEnforcementKernel::new();
crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:53:    assert_eq!(verdict, ConstitutionalVerdict::Constitutional);
crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:7:fn n126_3_state_root_mismatch_rejected() {
crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:8:    let mut kernel = ConstitutionalEnforcementKernel::new();
crates/amun-live-cluster/tests/n129_evidence_continuity_audit.rs:103:        root.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
crates/amun-live-cluster/tests/n129_evidence_continuity_audit.rs:124:        root.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
crates/amun-live-cluster/tests/n129_evidence_continuity_audit.rs:8:fn n129_4_evidence_continuity_audit() {
crates/amun-mempool-gossip/src/mempool.rs:228:    fn n74_mempool_insert_and_retrieve() {
crates/amun-mempool-gossip/src/mempool.rs:238:    fn n74_mempool_reject_duplicate() {
crates/amun-mempool-gossip/src/mempool.rs:247:    fn n74_mempool_reject_tampered() {
crates/amun-mempool-gossip/src/mempool.rs:255:    fn n74_gossip_announce() {
crates/amun-mempool-gossip/src/mempool.rs:263:    fn n74_gossip_request_and_response() {
crates/amun-mempool-gossip/src/mempool.rs:274:    fn n74_gossip_broadcast() {
crates/amun-mempool-gossip/src/mempool.rs:286:    fn n74_mempool_pending_transactions() {
crates/amun-mempool-gossip/src/messages.rs:102:    fn n74_gossip_message_serialization() {
crates/amun-mempool-gossip/src/messages.rs:75:    fn n74_tx_hash_deterministic() {
crates/amun-mempool-gossip/src/messages.rs:82:    fn n74_tx_hash_changes_with_nonce() {
crates/amun-mempool-gossip/src/messages.rs:89:    fn n74_tx_hash_verification() {
crates/amun-mempool-gossip/src/messages.rs:91:        assert!(tx.verify_hash());
crates/amun-mempool-gossip/src/messages.rs:95:    fn n74_tx_tampered_rejected() {
crates/amun-mempool-gossip/src/messages.rs:98:        assert!(!tx.verify_hash());
crates/amun-mempool/src/lib.rs:105:    fn n24_remove_committed() {
crates/amun-mempool/src/lib.rs:117:    fn n24_order_preserved() {
crates/amun-mempool/src/lib.rs:129:    fn n24_empty_mempool() {
crates/amun-mempool/src/lib.rs:77:    fn n24_add_transaction() {
crates/amun-mempool/src/lib.rs:85:    fn n24_reject_duplicate_nonce() {
crates/amun-mempool/src/lib.rs:94:    fn n24_take_for_block() {
crates/amun-merkle/src/proof.rs:159:        assert!(proof.verify(&l1, &root));
crates/amun-merkle/src/proof.rs:171:        assert!(decoded.verify(&l1, &MerkleTree::compute_root(&[l1, l2])));
crates/amun-merkle/src/proof.rs:181:        assert!(MerkleProof::decode_exact(&buf[..len + 1]).is_err());
crates/amun-merkle/src/proof.rs:188:        assert!(MerkleProof::decode(&buf).is_err());
crates/amun-merkle/src/proof.rs:199:        assert!(MerkleProof::decode(&buf).is_err());
crates/amun-merkle/src/tests.rs:100:    assert!(!proof.verify(&l3, &root));
crates/amun-merkle/src/tests.rs:87:    assert!(proof.verify(&l1, &root));
crates/amun-network-constitution/src/anti_replay.rs:10:    pub fn new() -> Self {
crates/amun-network-constitution/src/anti_replay.rs:17:    pub fn check_and_record(&mut self, sender_id: u64, nonce: u64) -> Result<(), &'static str> {
crates/amun-network-constitution/src/anti_replay.rs:21:                return Err("replay detected: nonce too low");
crates/amun-network-constitution/src/anti_replay.rs:33:    fn prune_oldest(&mut self) {
crates/amun-network-constitution/src/anti_replay.rs:42:    pub fn highest_nonce(&self, sender_id: u64) -> Option<u64> {
crates/amun-network-constitution/src/anti_replay.rs:47:impl Default for AntiReplayGuard {
crates/amun-network-constitution/src/anti_replay.rs:48:    fn default() -> Self {
crates/amun-network-constitution/src/anti_replay.rs:4:pub struct AntiReplayGuard {
crates/amun-network-constitution/src/anti_replay.rs:9:impl AntiReplayGuard {
crates/amun-network-constitution/src/lib.rs:1:pub mod anti_replay;
crates/amun-network-constitution/src/lib.rs:4:pub use anti_replay::AntiReplayGuard;
crates/amun-network-fastpath/src/lib.rs:61:pub fn benchmark_batching(message_count: u64, batch_size: usize) -> FastPathResult {
crates/amun-network-fastpath/tests/n164_fastpath_tests.rs:29:fn n164_large_message_throughput() {
crates/amun-network-fastpath/tests/n164_fastpath_tests.rs:44:fn n164_batch_hash_deterministic() {
crates/amun-network-fastpath/tests/n164_fastpath_tests.rs:4:fn n164_message_batching_reduces_overhead() {
crates/amun-network-fastpath/tests/n164_fastpath_tests.rs:54:    let hash1 = batch1.finalize();
crates/amun-network-fastpath/tests/n164_fastpath_tests.rs:55:    let hash2 = batch2.finalize();
crates/amun-network-simulator/src/delivery.rs:16:    pub fn latency_for(&self, sender: u64, receiver: u64) -> u64 {
crates/amun-network-simulator/src/delivery.rs:9:    pub fn new(base_latency_rounds: u64, jitter_rounds: u64) -> Self {
crates/amun-network-transport/src/message.rs:13:    /// N111.3: Evidence announcement for network propagation
crates/amun-network-transport/src/message.rs:148:    fn n66_message_roundtrip_block_announce() {
crates/amun-network-transport/src/message.rs:169:    fn n66_message_roundtrip_certificate() {
crates/amun-network-transport/src/message.rs:190:    fn n66_message_roundtrip_ping_pong() {
crates/amun-network-transport/src/message.rs:208:    fn n66_reject_malformed_frame() {
crates/amun-network-transport/src/message.rs:214:    fn n66_reject_truncated_frame() {
crates/amun-network-transport/src/message.rs:226:    fn n110_3b_roundtrip_slashing_certificate() {
crates/amun-network-transport/src/message.rs:249:    fn n111_3_roundtrip_evidence_announcement() {
crates/amun-network-transport/src/message.rs:60:/// N111.3: Evidence announcement for network propagation.
crates/amun-network-transport/src/peer.rs:35:    fn n66_peer_identity_creation() {
crates/amun-network-transport/src/sync_over_network.rs:154:    fn n67_network_sync_snapshot_request_response() {
crates/amun-network-transport/src/sync_over_network.rs:211:    fn n67_network_sync_already_synced() {
crates/amun-network-transport/src/sync_over_network.rs:254:    fn n67_network_sync_delta() {
crates/amun-network-transport/src/sync_over_network.rs:308:    fn n67_network_sync_malformed_request_no_crash() {
crates/amun-network-transport/src/transport.rs:221:    fn n66_transport_create_and_add_peer() {
crates/amun-network-transport/src/transport.rs:240:    fn n66_message_encode_decode_preserves_content() {
crates/amun-network-transport/src/transport.rs:272:    fn n66_real_socket_ping_pong() {
crates/amun-network-transport/src/transport.rs:350:    fn n66_real_socket_block_announce() {
crates/amun-network-transport/src/transport.rs:395:    fn n66_real_socket_certificate_announce() {
crates/amun-network-transport/src/transport.rs:440:    fn n66_real_socket_two_way_exchange() {
crates/amun-networking/src/crypto_identity.rs:102:        assert_eq!(peer_id.0, keypair.verifying_key.to_bytes());
crates/amun-networking/src/crypto_identity.rs:106:    fn n20_5_sign_and_verify() {
crates/amun-networking/src/crypto_identity.rs:110:        assert!(PeerKeyPair::verify(
crates/amun-networking/src/crypto_identity.rs:118:    fn n20_5_tampered_message_rejected() {
crates/amun-networking/src/crypto_identity.rs:122:        assert!(!PeerKeyPair::verify(
crates/amun-networking/src/crypto_identity.rs:130:    fn n20_5_wrong_signer_rejected() {
crates/amun-networking/src/crypto_identity.rs:135:        assert!(!PeerKeyPair::verify(
crates/amun-networking/src/crypto_identity.rs:143:    fn n20_5_signed_message_roundtrip() {
crates/amun-networking/src/crypto_identity.rs:147:        assert!(signed.verify());
crates/amun-networking/src/crypto_identity.rs:148:        assert_eq!(signed.sender_peer_id().0, keypair.verifying_key.to_bytes());
crates/amun-networking/src/crypto_identity.rs:152:    fn n20_5_serialize_deserialize_signed_message() {
crates/amun-networking/src/crypto_identity.rs:157:        assert!(decoded.verify());
crates/amun-networking/src/crypto_identity.rs:99:    fn n20_5_keypair_generation() {
crates/amun-networking/src/peer_discovery.rs:100:        assert!(PeerAnnouncement::verify(&announcement, &signed));
crates/amun-networking/src/peer_discovery.rs:104:    fn n20_7_tampered_announcement_rejected() {
crates/amun-networking/src/peer_discovery.rs:110:        assert!(!PeerAnnouncement::verify(&announcement, &signed));
crates/amun-networking/src/peer_discovery.rs:114:    fn n20_7_peer_registry_register_and_lookup() {
crates/amun-networking/src/peer_discovery.rs:130:    fn n20_7_peer_expiration() {
crates/amun-networking/src/peer_discovery.rs:149:    fn n20_7_duplicate_peer_updates_timestamp() {
crates/amun-networking/src/peer_discovery.rs:169:    fn n20_7_registry_deterministic() {
crates/amun-networking/src/peer_discovery.rs:96:    fn n20_7_peer_announcement_sign_and_verify() {
crates/amun-networking/src/quarantine.rs:32:    ReplayVerified,
crates/amun-networking/src/risk.rs:10:    /// Peer's replay history is unstable
crates/amun-networking/src/risk.rs:11:    ReplayInstability { divergence_count: u64 },
crates/amun-networking/src/risk.rs:61:            ConstitutionalRisk::ReplayInstability { .. } => {
crates/amun-networking/src/signed_envelope.rs:104:        assert!(directed.signed_envelope.verify());
crates/amun-networking/src/signed_envelope.rs:56:    fn n20_6_signed_envelope_verification() {
crates/amun-networking/src/signed_envelope.rs:67:        assert!(signed.verify());
crates/amun-networking/src/signed_envelope.rs:71:    fn n20_6_tampered_envelope_rejected() {
crates/amun-networking/src/signed_envelope.rs:83:        assert!(!signed.verify());
crates/amun-networking/src/signed_envelope.rs:87:    fn n20_6_directed_message_roundtrip() {
crates/amun-networking/src/sovereignty.rs:16:/// Multi-phase: Identity -> Physics -> Replay -> Lineage -> Temporal -> Sync
crates/amun-networking/src/sovereignty.rs:24:    pub replay_compatible: bool,
crates/amun-networking/src/sovereignty.rs:36:    ReplayCompatibilityCheck,
crates/amun-networking/src/sovereignty.rs:51:            replay_compatible: false,
crates/amun-networking/src/sovereignty.rs:77:                HandshakePhase::ReplayCompatibilityCheck
crates/amun-networking/src/sovereignty.rs:87:            && self.replay_compatible
crates/amun-networking/src/sync_protocol.rs:22:    fn n18_sync_request_serialization() {
crates/amun-networking/src/sync_protocol.rs:30:    fn n18_sync_response_serialization() {
crates/amun-networking/src/trust_anchor.rs:55:    fn n21_register_and_lookup_trust_anchor() {
crates/amun-networking/src/trust_anchor.rs:68:    fn n21_revoke_trust_anchor() {
crates/amun-networking/src/trust_anchor.rs:82:    fn n21_untrusted_peer_rejected() {
crates/amun-networking/src/validator_certificate.rs:106:    fn n21_validator_certificate_issue_and_verify() {
crates/amun-networking/src/validator_certificate.rs:118:        assert!(cert.verify(&authority.verifying_key.to_bytes()));
crates/amun-networking/src/validator_certificate.rs:120:        assert_eq!(cert.issuer, authority.peer_id());
crates/amun-networking/src/validator_certificate.rs:124:    fn n21_certificate_rejects_wrong_authority() {
crates/amun-networking/src/validator_certificate.rs:138:        assert!(!cert.verify(&impostor.verifying_key.to_bytes()));
crates/amun-networking/src/validator_certificate.rs:142:    fn n21_certificate_tampered_fields_rejected() {
crates/amun-networking/src/validator_certificate.rs:156:        assert!(!cert.verify(&authority.verifying_key.to_bytes()));
crates/amun-networking/src/validator_certificate.rs:160:    fn n21_certificate_validity_window() {
crates/amun-networking/src/validator_certificate.rs:180:    fn n21_certificate_serialization_roundtrip() {
crates/amun-networking/src/validator_certificate.rs:201:        assert!(decoded.verify(&authority.verifying_key.to_bytes()));
crates/amun-networking/src/validator_registry.rs:105:    fn n21_validator_rejected_with_forged_certificate() {
crates/amun-networking/src/validator_registry.rs:64:    fn n21_validator_registration_with_valid_certificate() {
crates/amun-networking/src/validator_registry.rs:86:    fn n21_validator_rejected_without_trust_anchor() {
crates/amun-networking/tests/n17_multi_node_network.rs:101:fn n17_four_node_network_reaches_first_commit() {
crates/amun-networking/tests/n17_multi_node_network.rs:103:    assert!(net.run_until_commits(500, 1));
crates/amun-networking/tests/n17_multi_node_network.rs:107:fn n17_seven_node_network_reaches_first_commit() {
crates/amun-networking/tests/n17_multi_node_network.rs:109:    assert!(net.run_until_commits(800, 1));
crates/amun-networking/tests/n17_multi_node_network.rs:113:fn n17_node_crash_and_recovery() {
crates/amun-networking/tests/n17_multi_node_network.rs:116:    net.run_until_commits(500, 1);
crates/amun-networking/tests/n17_multi_node_network.rs:121:    // Rebuild validator set from remaining live nodes
crates/amun-networking/tests/n17_multi_node_network.rs:134:    let _ = net.run_until_commits(2000, 2);
crates/amun-networking/tests/n17_multi_node_network.rs:135:    // If it doesn't reach 2 more commits, that's expected with tight quorum
crates/amun-networking/tests/n17_multi_node_network.rs:138:    // At minimum, we should have at least the original commit
crates/amun-networking/tests/n17_multi_node_network.rs:139:    let total_commits: usize = net.nodes.values().map(|n| n.committed_blocks.len()).sum();
crates/amun-networking/tests/n17_multi_node_network.rs:141:        total_commits >= 3,
crates/amun-networking/tests/n17_multi_node_network.rs:142:        "All nodes should have at least 1 commit each"
crates/amun-networking/tests/n17_multi_node_network.rs:147:fn n17_bootstrap_trusted_root_persists() {
crates/amun-networking/tests/n17_multi_node_network.rs:154:fn n17_all_nodes_eventually_commit_multiple_blocks() {
crates/amun-networking/tests/n17_multi_node_network.rs:156:    assert!(net.run_until_commits(1000, 3));
crates/amun-networking/tests/n17_multi_node_network.rs:15:///   5. When a QC forms, consensus emits Commit → block finalized
crates/amun-networking/tests/n17_multi_node_network.rs:16:struct Network {
crates/amun-networking/tests/n17_multi_node_network.rs:22:impl Network {
crates/amun-networking/tests/n17_multi_node_network.rs:23:    fn new(count: usize) -> Self {
crates/amun-networking/tests/n17_multi_node_network.rs:42:    fn tick(&mut self) {
crates/amun-networking/tests/n17_multi_node_network.rs:85:    fn run_until_commits(&mut self, max_ticks: usize, target_commits: usize) -> bool {
crates/amun-networking/tests/n17_multi_node_network.rs:91:                .all(|n| n.committed_blocks.len() >= target_commits);
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:22:fn n18_full_lifecycle_with_catchup() {
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:50:fn n18_catchup_preserves_height_after_activation() {
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:5:// N18.5 — Constitutional Catch-up (no network)
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:65:fn n18_sync_request_roundtrip() {
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:73:fn n18_sync_response_serialization_roundtrip() {
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:89:fn n18_full_rejoin_matches_network_state() {
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:90:    // 4 nodes commit 10 blocks
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:92:    // Network commits 20 more
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:9:fn n18_catchup_import_checkpoint_height() {
crates/amun-networking/tests/n18_checkpoint_sync.rs:100:fn n18_bootstrapping_node_rejects_proposal_before_activation() {
crates/amun-networking/tests/n18_checkpoint_sync.rs:115:fn n18_sync_response_with_multiple_checkpoints() {
crates/amun-networking/tests/n18_checkpoint_sync.rs:116:    let cp1 = build_checkpoint(0, 9);
crates/amun-networking/tests/n18_checkpoint_sync.rs:117:    let cp2 = build_checkpoint(10, 19);
crates/amun-networking/tests/n18_checkpoint_sync.rs:12:/// Helper: build a checkpoint covering blocks [start, end].
crates/amun-networking/tests/n18_checkpoint_sync.rs:13:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n18_checkpoint_sync.rs:14:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-networking/tests/n18_checkpoint_sync.rs:15:    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
crates/amun-networking/tests/n18_checkpoint_sync.rs:19:        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
crates/amun-networking/tests/n18_checkpoint_sync.rs:1:use amun_certificate_network::distribution::LightClientProofBundle;
crates/amun-networking/tests/n18_checkpoint_sync.rs:22:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-networking/tests/n18_checkpoint_sync.rs:24:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n18_checkpoint_sync.rs:32:        let block = ConstitutionalBlock::new(
crates/amun-networking/tests/n18_checkpoint_sync.rs:45:        bundles.push(LightClientProofBundle::new(block, cert, proof));
crates/amun-networking/tests/n18_checkpoint_sync.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n18_checkpoint_sync.rs:52:fn n18_checkpoint_sync_between_nodes() {
crates/amun-networking/tests/n18_checkpoint_sync.rs:54:    let cp_a = build_checkpoint(0, 49);
crates/amun-networking/tests/n18_checkpoint_sync.rs:57:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp_a.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n18_checkpoint_sync.rs:7:use amun_constitutional_block::ConstitutionalBlock;
crates/amun-networking/tests/n18_checkpoint_sync.rs:86:    // Node B transitions through lifecycle
crates/amun-networking/tests/n18_checkpoint_sync.rs:8:use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-networking/tests/n18_full_rejoin.rs:115:fn n18_rejoin_preserves_network_height_across_multiple_checkpoints() {
crates/amun-networking/tests/n18_full_rejoin.rs:116:    let cp1 = build_checkpoint(0, 19);
crates/amun-networking/tests/n18_full_rejoin.rs:117:    let cp2 = build_checkpoint(20, 39);
crates/amun-networking/tests/n18_full_rejoin.rs:118:    let cp3 = build_checkpoint(40, 59);
crates/amun-networking/tests/n18_full_rejoin.rs:123:    let proof1 = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n18_full_rejoin.rs:124:    let proof2 = prove_checkpoint_inclusion(&checkpoints, &cp2.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n18_full_rejoin.rs:125:    let proof3 = prove_checkpoint_inclusion(&checkpoints, &cp3.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n18_full_rejoin.rs:13:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n18_full_rejoin.rs:146:fn n18_bootstrapping_node_cannot_activate_directly() {
crates/amun-networking/tests/n18_full_rejoin.rs:14:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-networking/tests/n18_full_rejoin.rs:156:    // Only after proper lifecycle transition
crates/amun-networking/tests/n18_full_rejoin.rs:15:    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
crates/amun-networking/tests/n18_full_rejoin.rs:19:        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
crates/amun-networking/tests/n18_full_rejoin.rs:1:use amun_certificate_network::distribution::LightClientProofBundle;
crates/amun-networking/tests/n18_full_rejoin.rs:22:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-networking/tests/n18_full_rejoin.rs:24:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n18_full_rejoin.rs:32:        let block = ConstitutionalBlock::new(
crates/amun-networking/tests/n18_full_rejoin.rs:45:        bundles.push(LightClientProofBundle::new(block, cert, proof));
crates/amun-networking/tests/n18_full_rejoin.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n18_full_rejoin.rs:56:fn n18_rejoin_after_network_progress() {
crates/amun-networking/tests/n18_full_rejoin.rs:57:    // Phase 1: Network commits 10 blocks, then node 3 is removed
crates/amun-networking/tests/n18_full_rejoin.rs:58:    let _cp_early = build_checkpoint(0, 9);
crates/amun-networking/tests/n18_full_rejoin.rs:61:    // Phase 2: Network commits 40 more blocks (50 total)
crates/amun-networking/tests/n18_full_rejoin.rs:62:    let cp_late = build_checkpoint(10, 49);
crates/amun-networking/tests/n18_full_rejoin.rs:66:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp_late.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n18_full_rejoin.rs:7:use amun_constitutional_block::ConstitutionalBlock;
crates/amun-networking/tests/n18_full_rejoin.rs:8:use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-networking/tests/n18_node_rejoin.rs:19:fn n18_active_node_can_propose() {
crates/amun-networking/tests/n18_node_rejoin.rs:30:fn n18_lifecycle_transitions() {
crates/amun-networking/tests/n18_node_rejoin.rs:48:fn n18_bootstrapping_node_stores_trusted_root() {
crates/amun-networking/tests/n18_node_rejoin.rs:59:fn n18_full_rejoin_after_crash() {
crates/amun-networking/tests/n18_node_rejoin.rs:5:// N18.2 — Lifecycle Invariants
crates/amun-networking/tests/n18_node_rejoin.rs:61:    // 1. 4-node network commits block 1
crates/amun-networking/tests/n18_node_rejoin.rs:63:    // 3. Network commits 20 more blocks
crates/amun-networking/tests/n18_node_rejoin.rs:73:// N18.5 — Constitutional Invariant REJOIN-001
crates/amun-networking/tests/n18_node_rejoin.rs:77:fn n18_rejoin001_bootstrapping_node_must_verify_before_active() {
crates/amun-networking/tests/n18_node_rejoin.rs:9:fn n18_bootstrapping_node_cannot_propose() {
crates/amun-networking/tests/n19_adversarial_rejoin.rs:100:fn n19_checkpoint_gap_detected() {
crates/amun-networking/tests/n19_adversarial_rejoin.rs:101:    let cp1 = build_checkpoint(0, 19);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:102:    let cp2 = build_checkpoint(40, 59);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:106:    let proof1 = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:107:    let proof2 = prove_checkpoint_inclusion(&checkpoints, &cp2.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:11:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n19_adversarial_rejoin.rs:128:fn n19_mixed_checkpoint_stream_rejected() {
crates/amun-networking/tests/n19_adversarial_rejoin.rs:129:    let cp1 = build_checkpoint(0, 9);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:12:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:130:    let cp2 = build_checkpoint(10, 19);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:134:    let proof1 = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:138:        prove_checkpoint_inclusion(&checkpoints, &cp2.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:13:    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:155:fn n19_byzantine_rejoin_source_rejected() {
crates/amun-networking/tests/n19_adversarial_rejoin.rs:156:    let cp = build_checkpoint(0, 49);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:159:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:17:        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:1:use amun_certificate_network::distribution::LightClientProofBundle;
crates/amun-networking/tests/n19_adversarial_rejoin.rs:20:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-networking/tests/n19_adversarial_rejoin.rs:22:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:30:        let block = ConstitutionalBlock::new(
crates/amun-networking/tests/n19_adversarial_rejoin.rs:43:        bundles.push(LightClientProofBundle::new(block, cert, proof));
crates/amun-networking/tests/n19_adversarial_rejoin.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n19_adversarial_rejoin.rs:54:fn n19_wrong_trusted_root_rejected() {
crates/amun-networking/tests/n19_adversarial_rejoin.rs:55:    let cp = build_checkpoint(0, 9);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:58:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:72:fn n19_checkpoint_rollback_rejected() {
crates/amun-networking/tests/n19_adversarial_rejoin.rs:73:    let cp_high = build_checkpoint(50, 59);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:7:use amun_constitutional_block::ConstitutionalBlock;
crates/amun-networking/tests/n19_adversarial_rejoin.rs:81:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp_high.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:8:use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:104:        let block = ConstitutionalBlock::new(
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:117:        bundles.push(LightClientProofBundle::new(block, cert, proof));
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:11:use amun_constitutional_block::ConstitutionalBlock;
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:124:fn n20_10_new_node_bootstraps_from_existing_peer() {
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:126:    let cp = build_checkpoint(0, 49);
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:129:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:12:use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:132:    // Phase 2: New node starts bootstrapping
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:137:    // Phase 3: New node receives checkpoint from existing peer
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:164:fn n20_10_four_nodes_start_together() {
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:171:            keypair.verifying_key.to_bytes(),
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:197:fn n20_10_network_survives_disconnect() {
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:198:    let cp = build_checkpoint(50, 99);
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:201:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:23:fn find_available_port() -> u16 {
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:32:fn n20_10_tcp_bind_and_connect() {
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:59:fn n20_10_peer_identity_exchange() {
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:5:use amun_certificate_network::distribution::LightClientProofBundle;
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:68:        alice_keypair.verifying_key.to_bytes(),
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:73:        bob_keypair.verifying_key.to_bytes(),
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:85:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:86:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:87:    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:8:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:91:        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:94:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:96:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:111:    // Node B transitions to active
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:119:fn n20_8_wrong_trusted_root_rejected_over_tcp() {
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:120:    let cp = build_checkpoint(0, 9);
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:122:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:132:fn n20_8_empty_bootstrap_rejected() {
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:13:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:146:fn n20_8_activation_after_successful_bootstrap() {
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:147:    let cp = build_checkpoint(0, 49);
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:14:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:150:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:15:    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:19:        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:1:use amun_certificate_network::distribution::LightClientProofBundle;
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:22:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:24:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:32:        let block = ConstitutionalBlock::new(
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:45:        bundles.push(LightClientProofBundle::new(block, cert, proof));
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:56:fn n20_8_bootstrap_request_roundtrip() {
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:64:fn n20_8_bootstrap_response_roundtrip() {
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:65:    let cp = build_checkpoint(0, 9);
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:7:use amun_constitutional_block::ConstitutionalBlock;
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:81:fn n20_8_node_bootstraps_over_tcp() {
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:83:    let cp = build_checkpoint(0, 49);
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:86:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:8:use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:106:fn n20_9_rejoin_preserves_height_after_long_absence() {
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:107:    let cp = build_checkpoint(100, 199);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:110:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:12:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:130:fn n20_9_rejoin_rejects_outdated_checkpoint() {
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:131:    let cp_old = build_checkpoint(0, 9);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:135:        prove_checkpoint_inclusion(&checkpoints_old, &cp_old.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:13:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:141:    let cp_new = build_checkpoint(50, 99);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:145:        prove_checkpoint_inclusion(&checkpoints_new, &cp_new.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:14:    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:152:    // Trying to verify old checkpoint against new root must fail
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:161:fn n20_9_rejoin_requires_full_lifecycle() {
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:162:    let cp = build_checkpoint(0, 49);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:165:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:18:        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:1:use amun_certificate_network::distribution::LightClientProofBundle;
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:21:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:23:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:31:        let block = ConstitutionalBlock::new(
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:44:        bundles.push(LightClientProofBundle::new(block, cert, proof));
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:55:fn n20_9_rejoin_after_crash_over_tcp() {
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:57:    let cp_late = build_checkpoint(50, 99);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:61:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp_late.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:7:use amun_constitutional_block::ConstitutionalBlock;
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:84:    // Phase 6: Full lifecycle transition
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:8:use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:102:fn n149_unauthorized_transfer_rejected() {
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:103:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:112:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:118:    let result = NftEvidenceKernel::verify_ownership(&reg, &token_id, &thief);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:11:fn n149_double_mint_rejected() {
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:123:fn n149_bridge_locked_sale_rejected() {
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:124:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:126:    let mut constitutional = ConstitutionalRegistry::new();
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:12:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:135:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:151:    constitutional.register(NftConstitutionalRecord {
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:162:    let can_sell = amun_nft_constitutional_enforcement::EnforcementEngine::can_be_sold(
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:21:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:2:use amun_nft_constitutional_registry::{ConstitutionalRegistry, NftConstitutionalRecord};
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:35:            lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:3:use amun_nft_evidence::NftEvidenceKernel;
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:48:            lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:4:use amun_nft_marketplace::MarketplaceEngine;
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:57:fn n149_double_spend_prevented() {
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:58:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:59:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:69:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:83:fn n149_invalid_evidence_rejected() {
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:84:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:91:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:97:    let result = NftEvidenceKernel::verify_metadata_hash(&[1u8; 32], &[2u8; 32]);
crates/amun-nft-benchmark/src/lib.rs:10:    fn it_works() {
crates/amun-nft-benchmark/src/lib.rs:1:pub fn add(left: u64, right: u64) -> u64 {
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:101:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:122:fn n152_benchmark_state_root_10k() {
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:123:    let mut reg = ResourceRegistry::new(20000);
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:129:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:12:    hasher.finalize().into()
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:145:                lineage: ResourceLineage::single_ancestor(
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:159:    let root = reg.compute_state_root();
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:16:fn n152_benchmark_mint_10k_nfts() {
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:17:    let mut reg = ResourceRegistry::new(20000);
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:1:use amun_nft_marketplace::MarketplaceEngine;
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:23:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:40:                lineage: ResourceLineage::single_ancestor(
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:63:fn n152_benchmark_rapid_trades() {
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:64:    let mut reg = ResourceRegistry::new(2000);
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:70:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:87:                lineage: ResourceLineage::single_ancestor(
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:9:fn unique_id(seed: u64) -> [u8; 32] {
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:105:    assert_ne!(l1.compute_bridge_root(), l2.compute_bridge_root());
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:30:fn n139_unlock_without_lock_fails() {
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:42:fn n139_double_lock_differs() {
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:4:fn n139_lock_and_unlock_flow() {
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:68:fn n139_deterministic_bridge_root() {
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:81:    assert_eq!(l1.compute_bridge_root(), l2.compute_bridge_root());
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:85:fn n139_bridge_root_changes_after_unlock() {
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:98:    assert_eq!(l1.compute_bridge_root(), l2.compute_bridge_root());
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:108:fn n156_evidence_root_deterministic() {
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:109:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:10:    let mut reg = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:110:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:118:        lineage: ResourceLineage::genesis(token_id1),
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:127:        lineage: ResourceLineage::genesis(token_id2),
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:133:    let mut engine1 = NftCollateralEngine::new();
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:134:    let mut engine2 = NftCollateralEngine::new();
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:144:        engine1.compute_evidence_root(),
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:145:        engine2.compute_evidence_root()
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:17:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:23:    let mut engine = NftCollateralEngine::new();
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:2:use amun_nft_collateral::NftCollateralEngine;
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:34:fn n156_repay_and_unlock() {
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:35:    let mut reg = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:42:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:48:    let mut engine = NftCollateralEngine::new();
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:60:fn n156_cannot_transfer_locked_nft() {
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:61:    let mut reg = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:68:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:74:    let mut engine = NftCollateralEngine::new();
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:80:fn n156_liquidation_removes_lock() {
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:81:    let mut reg = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:88:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:94:    let mut engine = NftCollateralEngine::new();
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:9:fn n156_lock_nft_and_borrow() {
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:101:fn n140_1_unified_constitutional_proof() {
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:102:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:103:    reg.register(NftConstitutionalRecord {
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:122:    let proof = EnforcementEngine::produce_constitutional_proof(
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:126:    let proof2 = EnforcementEngine::produce_constitutional_proof(
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:133:fn n140_1_full_integration_flow() {
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:134:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:142:    reg.register(NftConstitutionalRecord {
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:163:    assert!(EnforcementEngine::can_be_sold(&reg, &bridge, &token));
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:164:    let royalty = EnforcementEngine::enforce_royalty(&reg, &token, 5000);
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:168:    EnforcementEngine::transfer_governance(&mut reg, &token, &buyer);
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:174:    let proof = EnforcementEngine::produce_constitutional_proof(
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:21:    let record = NftConstitutionalRecord {
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:2:use amun_nft_constitutional_enforcement::EnforcementEngine;
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:32:    assert!(!EnforcementEngine::can_be_sold(&reg, &bridge, &token));
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:36:fn n140_1_can_sell_unlocked_nft() {
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:37:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:3:use amun_nft_constitutional_registry::{ConstitutionalRegistry, NftConstitutionalRecord};
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:40:    reg.register(NftConstitutionalRecord {
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:50:    assert!(EnforcementEngine::can_be_sold(&reg, &bridge, &token));
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:54:fn n140_1_governance_transfers_with_ownership() {
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:55:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:57:    reg.register(NftConstitutionalRecord {
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:73:    EnforcementEngine::transfer_governance(&mut reg, &token, &[20u8; 32]);
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:80:fn n140_1_royalty_enforced_on_sale() {
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:81:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:83:    reg.register(NftConstitutionalRecord {
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:8:fn n140_1_cannot_sell_bridge_locked_nft() {
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:96:    let royalty = EnforcementEngine::enforce_royalty(&reg, &token, 1000);
crates/amun-nft-constitutional-enforcement/tests/n140_1_enforcement_tests.rs:9:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:100:    let base = NftConstitutionalRecord {
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:10:    let record = NftConstitutionalRecord {
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:124:        r1.compute_constitutional_root(),
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:125:        r2.compute_constitutional_root()
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:130:fn n140_root_changes_with_different_governance() {
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:131:    let mut r1 = ConstitutionalRegistry::new();
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:132:    let mut r2 = ConstitutionalRegistry::new();
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:133:    let base = NftConstitutionalRecord {
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:155:        r1.compute_constitutional_root(),
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:156:        r2.compute_constitutional_root()
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:2:use amun_nft_constitutional_registry::{ConstitutionalRegistry, NftConstitutionalRecord};
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:37:fn n140_multiple_tokens_independent() {
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:38:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:41:    reg.register(NftConstitutionalRecord {
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:51:    reg.register(NftConstitutionalRecord {
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:66:fn n140_deterministic_constitutional_root() {
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:67:    let mut r1 = ConstitutionalRegistry::new();
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:68:    let mut r2 = ConstitutionalRegistry::new();
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:69:    let record = NftConstitutionalRecord {
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:7:fn n140_register_and_query_record() {
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:8:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:91:        r1.compute_constitutional_root(),
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:92:        r2.compute_constitutional_root()
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:97:fn n140_root_changes_with_different_bridge_lock() {
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:98:    let mut r1 = ConstitutionalRegistry::new();
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:99:    let mut r2 = ConstitutionalRegistry::new();
crates/amun-nft-evidence/src/lib.rs:106:        // Law 4: replay protection
crates/amun-nft-evidence/src/lib.rs:107:        Self::verify_replay_protection(ctx.last_event_time, ctx.timestamp)?;
crates/amun-nft-evidence/src/lib.rs:123:        // Law 4: replay protection
crates/amun-nft-evidence/src/lib.rs:124:        Self::verify_replay_protection(last_event_time, timestamp)?;
crates/amun-nft-evidence/src/lib.rs:140:        // Law 4: replay protection
crates/amun-nft-evidence/src/lib.rs:141:        Self::verify_replay_protection(last_event_time, timestamp)?;
crates/amun-nft-evidence/src/lib.rs:24:    Law4ReplayDetected,
crates/amun-nft-evidence/src/lib.rs:67:    /// Law 4: Replay Protection — check nonce or timestamp ordering
crates/amun-nft-evidence/src/lib.rs:68:    pub fn verify_replay_protection(
crates/amun-nft-evidence/src/lib.rs:73:            return Err(CekError::Law4ReplayDetected);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:102:fn n131_law4_replay_protection_rejects_old_timestamp() {
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:103:    let result = NftEvidenceKernel::verify_replay_protection(1000, 500);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:104:    assert_eq!(result, Err(CekError::Law4ReplayDetected));
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:108:fn n131_evidence_root_matches() {
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:11:fn n131_mint_produces_valid_evidence() {
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:121:    let ev1 = NftEvidence::new(event1, 1000, 1);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:122:    let ev2 = NftEvidence::new(event2, 2000, 1);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:12:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:130:fn n131_full_mint_flow_with_evidence() {
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:131:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:138:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:159:    assert!(NftEvidenceKernel::verify_mint(ctx).is_ok());
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:167:    let evidence = NftEvidenceKernel::generate_evidence(event.clone(), timestamp, 42).unwrap();
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:19:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:1:use amun_nft_core::{NftEvent, NftEvidence};
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:3:    accumulate_nft_evidence_root, CekError, MintVerificationContext, NftEvidenceKernel,
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:40:    assert!(NftEvidenceKernel::verify_mint(ctx).is_ok());
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:48:    let evidence = NftEvidenceKernel::generate_evidence(event, timestamp, 1).unwrap();
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:56:fn n131_law1_prevents_unauthorized_transfer() {
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:57:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:66:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:72:    let result = NftEvidenceKernel::verify_transfer(&reg, &token_id, &thief, 2000, 1000);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:77:fn n131_law2_prevents_duplicate_mint() {
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:78:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:85:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:91:    let result = NftEvidenceKernel::verify_non_duplicate(&reg, &token_id);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:96:fn n131_law3_rejects_invalid_metadata_hash() {
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:97:    let result = NftEvidenceKernel::verify_metadata_hash(&[1u8; 32], &[2u8; 32]);
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:15:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:1:use amun_nft_explorer::ExplorerEngine;
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:21:    let collections = ExplorerEngine::get_collections(&reg);
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:27:fn n135_query_nft_by_id() {
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:28:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:34:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:40:    let nft = ExplorerEngine::get_nft(&reg, &token_id).unwrap();
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:45:fn n135_query_owner_nfts() {
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:46:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:54:        lineage: ResourceLineage::genesis(id1),
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:63:        lineage: ResourceLineage::genesis(id2),
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:69:    let owner_data = ExplorerEngine::get_owner_nfts(&reg, &owner);
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:8:fn n135_query_collections() {
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:9:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-fuzz/src/lib.rs:24:    pub fn passed(&self) -> bool {
crates/amun-nft-fuzz/tests/n148_fuzz_tests.rs:15:fn n148_fuzz_marketplace_500_iterations() {
crates/amun-nft-fuzz/tests/n148_fuzz_tests.rs:26:fn n148_fuzz_royalty_10000_iterations() {
crates/amun-nft-fuzz/tests/n148_fuzz_tests.rs:37:fn n148_fuzz_governance_1000_iterations() {
crates/amun-nft-fuzz/tests/n148_fuzz_tests.rs:48:fn n148_fuzz_bridge_1000_iterations() {
crates/amun-nft-fuzz/tests/n148_fuzz_tests.rs:4:fn n148_fuzz_mint_1000_iterations() {
crates/amun-nft-governance-execution/tests/n143_governance_execution_tests.rs:37:fn n143_execute_passing_proposal() {
crates/amun-nft-governance-execution/tests/n143_governance_execution_tests.rs:5:fn n143_create_proposal_and_vote() {
crates/amun-nft-governance-execution/tests/n143_governance_execution_tests.rs:61:    assert!(exec.execute(&id, 50));
crates/amun-nft-governance-execution/tests/n143_governance_execution_tests.rs:65:fn n143_proposal_fails_without_rights() {
crates/amun-nft-governance-execution/tests/n143_governance_execution_tests.rs:75:fn n143_execution_root_deterministic() {
crates/amun-nft-governance-execution/tests/n143_governance_execution_tests.rs:98:    assert_eq!(e1.compute_execution_root(), e2.compute_execution_root());
crates/amun-nft-governance/tests/n138_governance_tests.rs:23:fn n138_revoke_rights() {
crates/amun-nft-governance/tests/n138_governance_tests.rs:41:fn n138_multiple_tokens_independent_rights() {
crates/amun-nft-governance/tests/n138_governance_tests.rs:4:fn n138_grant_and_check_rights() {
crates/amun-nft-governance/tests/n138_governance_tests.rs:67:fn n138_deterministic_governance_root() {
crates/amun-nft-governance/tests/n138_governance_tests.rs:79:    assert_eq!(l1.compute_governance_root(), l2.compute_governance_root());
crates/amun-nft-governance/tests/n138_governance_tests.rs:83:fn n138_revoked_changes_root() {
crates/amun-nft-governance/tests/n138_governance_tests.rs:97:    assert_ne!(l1.compute_governance_root(), l2.compute_governance_root());
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:10:    reg.register(NftConstitutionalRecord {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:112:    let old_root = indexer.compute_index_root();
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:119:    let new_root = indexer.compute_index_root();
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:1:use amun_nft_constitutional_registry::{ConstitutionalRegistry, NftConstitutionalRecord};
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:30:fn n144_query_by_owner() {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:31:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:34:        reg.register(NftConstitutionalRecord {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:52:fn n144_index_events_and_query() {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:75:fn n144_deterministic_index_root() {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:76:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:77:    reg.register(NftConstitutionalRecord {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:7:fn n144_index_and_query_nft() {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:8:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:92:    assert_eq!(i1.compute_index_root(), i2.compute_index_root());
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:96:fn n144_index_updates_after_registry_change() {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:97:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:99:    reg.register(NftConstitutionalRecord {
crates/amun-nft-integration/src/lib.rs:21:        replay_certificate: original.replay_certificate,
crates/amun-nft-integration/tests/n132_integration_tests.rs:15:fn n132_different_nft_changes_root() {
crates/amun-nft-integration/tests/n132_integration_tests.rs:16:    let original = EvidenceRoot::genesis();
crates/amun-nft-integration/tests/n132_integration_tests.rs:1:use amun_evidence_root::EvidenceRoot;
crates/amun-nft-integration/tests/n132_integration_tests.rs:26:fn n132_height_and_metadata_preserved() {
crates/amun-nft-integration/tests/n132_integration_tests.rs:27:    let original = EvidenceRoot::compute(
crates/amun-nft-integration/tests/n132_integration_tests.rs:29:        [8u8; 32], // commit_hash
crates/amun-nft-integration/tests/n132_integration_tests.rs:30:        [7u8; 32], // replay_certificate
crates/amun-nft-integration/tests/n132_integration_tests.rs:40:    assert_eq!(extended.commit_hash, [8u8; 32]);
crates/amun-nft-integration/tests/n132_integration_tests.rs:41:    assert_eq!(extended.replay_certificate, [7u8; 32]);
crates/amun-nft-integration/tests/n132_integration_tests.rs:5:fn n132_extended_root_deterministic() {
crates/amun-nft-integration/tests/n132_integration_tests.rs:6:    let original = EvidenceRoot::genesis();
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:101:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:107:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:117:fn n134_1_prevent_self_purchase() {
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:118:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:126:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:132:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:140:fn n134_1_prevent_bid_below_highest() {
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:141:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:149:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:155:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:164:fn n134_1_marketplace_evidence_root() {
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:165:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:173:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:179:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:182:    let root = mp.compute_evidence_root();
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:18:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:1:use amun_nft_marketplace::MarketplaceEngine;
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:24:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:37:fn n134_cancel_listing() {
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:38:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:46:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:52:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:61:fn n134_auction_flow() {
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:62:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:72:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:78:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:8:fn n134_list_and_buy_nft() {
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:90:fn n134_1_prevent_double_buy() {
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:91:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:9:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-mining/tests/n133_mining_tests.rs:17:fn n133_mining_reward_creates_nft() {
crates/amun-nft-mining/tests/n133_mining_tests.rs:18:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-mining/tests/n133_mining_tests.rs:1:use amun_nft_mining::{evaluate_contribution, issue_mining_reward, ContributionType};
crates/amun-nft-mining/tests/n133_mining_tests.rs:26:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-mining/tests/n133_mining_tests.rs:35:    let nft_id = issue_mining_reward(
crates/amun-nft-mining/tests/n133_mining_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-mining/tests/n133_mining_tests.rs:54:fn n133_multiple_contributions_get_different_nfts() {
crates/amun-nft-mining/tests/n133_mining_tests.rs:55:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-mining/tests/n133_mining_tests.rs:62:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-mining/tests/n133_mining_tests.rs:71:    let id1 = issue_mining_reward(
crates/amun-nft-mining/tests/n133_mining_tests.rs:80:    let id2 = issue_mining_reward(
crates/amun-nft-mining/tests/n133_mining_tests.rs:8:fn n133_validator_contribution_evaluates() {
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:100:    let (reg2, royalty2, gov2, bridge2) = build_full_state();
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:102:    assert_eq!(state_root_before, reg2.compute_state_root());
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:103:    assert_eq!(royalty_root_before, royalty2.compute_accounting_root());
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:104:    assert_eq!(gov_root_before, gov2.compute_governance_root());
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:105:    assert_eq!(bridge_root_before, bridge2.compute_bridge_root());
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:10:fn unique_id(seed: u8) -> [u8; 32] {
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:13:    hasher.finalize().into()
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:16:fn build_full_state() -> (
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:17:    ResourceRegistry,
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:22:    let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:32:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:49:                lineage: ResourceLineage::single_ancestor(
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:5:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:92:fn n150_state_roots_survive_rebuild() {
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:93:    let (reg1, royalty1, gov1, bridge1) = build_full_state();
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:95:    let state_root_before = reg1.compute_state_root();
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:96:    let royalty_root_before = royalty1.compute_accounting_root();
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:97:    let gov_root_before = gov1.compute_governance_root();
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:98:    let bridge_root_before = bridge1.compute_bridge_root();
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:100:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:112:    reg.register(NftConstitutionalRecord {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:12:    let result = RightsEnforcementEngine::validate_transfer(
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:131:    let result = RightsEnforcementEngine::validate_transfer(
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:145:fn n141_produce_enforcement_proof() {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:146:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:147:    reg.register(NftConstitutionalRecord {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:160:    let proof = RightsEnforcementEngine::produce_enforcement_proof(
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:164:    let proof2 = RightsEnforcementEngine::produce_enforcement_proof(
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:26:fn n141_reject_non_owner_seller() {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:27:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:29:    reg.register(NftConstitutionalRecord {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:2:use amun_nft_constitutional_registry::{ConstitutionalRegistry, NftConstitutionalRecord};
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:41:    let result = RightsEnforcementEngine::validate_transfer(
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:4:use amun_nft_rights_enforcement::RightsEnforcementEngine;
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:58:fn n141_reject_bridge_locked_transfer() {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:59:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:72:    reg.register(NftConstitutionalRecord {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:82:    let result = RightsEnforcementEngine::validate_transfer(
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:8:fn n141_reject_unregistered_token() {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:99:fn n141_allow_valid_transfer_with_royalty() {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:9:    let reg = ConstitutionalRegistry::new();
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:20:fn n137_multiple_sales_accumulation() {
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:37:fn n137_multiple_creators_independent_balances() {
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:5:fn n137_single_creator_accrual() {
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:60:fn n137_deterministic_accounting_root() {
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:74:        ledger1.compute_accounting_root(),
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:75:        ledger2.compute_accounting_root()
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:80:fn n137_overflow_safety() {
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:26:    let mut engine = SettlementEngine::new();
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:33:fn n142_no_settlement_for_zero_balance() {
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:35:    let mut engine = SettlementEngine::new();
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:3:use amun_nft_royalty_settlement::SettlementEngine;
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:41:fn n142_deterministic_settlement_root() {
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:53:    let mut e1 = SettlementEngine::new();
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:54:    let mut e2 = SettlementEngine::new();
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:58:    assert_eq!(e1.compute_settlement_root(), e2.compute_settlement_root());
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:62:fn n142_multiple_settlements_differ() {
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:6:fn n142_settle_accumulated_royalties() {
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:83:    let mut engine = SettlementEngine::new();
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:14:fn n136_auction_royalty_10_percent() {
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:19:    let record = RoyaltyEngine::generate_royalty_record([10u8; 32], &policy, [3u8; 32], 5000, 42);
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:26:fn n136_zero_royalty() {
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:31:    let amount = RoyaltyEngine::compute_royalty(1000, policy.royalty_bps);
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:36:fn n136_overflow_safety() {
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:41:    let amount = RoyaltyEngine::compute_royalty(u64::MAX, policy.royalty_bps);
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:47:fn n136_deterministic_evidence_root() {
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:4:fn n136_direct_sale_royalty_5_percent() {
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:52:    let record1 = RoyaltyEngine::generate_royalty_record([10u8; 32], &policy, [3u8; 32], 1000, 1);
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:53:    let record2 = RoyaltyEngine::generate_royalty_record([11u8; 32], &policy, [4u8; 32], 2000, 2);
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:61:fn n136_royalty_record_serialization_roundtrip() {
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:62:    let record = RoyaltyEngine::generate_royalty_record(
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:9:    let amount = RoyaltyEngine::compute_royalty(1000, policy.royalty_bps);
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:103:            lineage: amun_resource_core::ResourceLineage::genesis(ResourceId(token)),
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:116:fn n145_full_sdk_integration_flow() {
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:148:        amun_nft_constitutional_registry::NftConstitutionalRecord {
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:26:        amun_nft_constitutional_registry::NftConstitutionalRecord {
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:44:fn n145_transfer_via_sdk() {
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:56:            lineage: amun_resource_core::ResourceLineage::genesis(ResourceId(token)),
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:67:fn n145_list_and_buy_via_sdk() {
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:6:fn n145_mint_and_query_via_sdk() {
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:79:            lineage: amun_resource_core::ResourceLineage::genesis(ResourceId(token)),
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:91:fn n145_auction_flow_via_sdk() {
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:109:    let state_root = reg.compute_state_root();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:110:    let royalty_root = royalty.compute_accounting_root();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:111:    let gov_root = gov.compute_governance_root();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:112:    let bridge_root = bridge.compute_bridge_root();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:113:    let const_root = const_reg.compute_constitutional_root();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:11:fn unique_id(seed: u8) -> [u8; 32] {
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:130:fn n151_all_roots_deterministic_after_rebuild() {
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:131:    let (_, _, _, _, _, sr1, rr1, gr1, br1, cr1) = build_state();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:132:    let (_, _, _, _, _, sr2, rr2, gr2, br2, cr2) = build_state();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:142:fn n151_snapshot_roots_are_nonzero() {
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:143:    let (_, _, _, _, _, sr, rr, gr, br, cr) = build_state();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:14:    hasher.finalize().into()
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:153:fn n151_root_changes_after_mutation() {
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:165:    ) = build_state();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:178:            lineage: ResourceLineage::single_ancestor(
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:17:type SnapshotTestState = (
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:18:    ResourceRegistry,
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:207:    const_reg.register(NftConstitutionalRecord {
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:218:    assert_ne!(sr_before, reg.compute_state_root());
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:219:    assert_ne!(rr_before, royalty.compute_accounting_root());
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:220:    assert_ne!(gr_before, gov.compute_governance_root());
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:221:    assert_ne!(cr_before, const_reg.compute_constitutional_root());
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:22:    ConstitutionalRegistry,
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:2:use amun_nft_constitutional_registry::{ConstitutionalRegistry, NftConstitutionalRecord};
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:30:fn build_state() -> SnapshotTestState {
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:31:    let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:35:    let mut const_reg = ConstitutionalRegistry::new();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:42:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:59:                lineage: ResourceLineage::single_ancestor(
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:88:        const_reg.register(NftConstitutionalRecord {
crates/amun-nft-stress/src/lib.rs:15:pub fn run_stress_mint(
crates/amun-nft-stress/src/lib.rs:16:    registry: &mut ResourceRegistry,
crates/amun-nft-stress/src/lib.rs:1:use amun_nft_marketplace::MarketplaceEngine;
crates/amun-nft-stress/src/lib.rs:29:            lineage: ResourceLineage::single_ancestor(
crates/amun-nft-stress/src/lib.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-stress/src/lib.rs:55:pub fn run_stress_marketplace(
crates/amun-nft-stress/src/lib.rs:56:    registry: &mut ResourceRegistry,
crates/amun-nft-stress/src/lib.rs:57:    marketplace: &mut MarketplaceEngine,
crates/amun-nft-stress/src/lib.rs:7:pub struct StressTestResult {
crates/amun-nft-stress/src/lib.rs:90:fn stress_token_id(index: u64) -> [u8; 32] {
crates/amun-nft-stress/src/lib.rs:94:    hasher.finalize().into()
crates/amun-nft-stress/tests/n146_stress_tests.rs:100:    use amun_nft_constitutional_enforcement::EnforcementEngine;
crates/amun-nft-stress/tests/n146_stress_tests.rs:101:    use amun_nft_constitutional_registry::{ConstitutionalRegistry, NftConstitutionalRecord};
crates/amun-nft-stress/tests/n146_stress_tests.rs:103:    use amun_nft_marketplace::MarketplaceEngine;
crates/amun-nft-stress/tests/n146_stress_tests.rs:107:        ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-stress/tests/n146_stress_tests.rs:111:    let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:112:    let mut marketplace = MarketplaceEngine::new();
crates/amun-nft-stress/tests/n146_stress_tests.rs:113:    let mut constitutional = ConstitutionalRegistry::new();
crates/amun-nft-stress/tests/n146_stress_tests.rs:129:        lineage: ResourceLineage::genesis(ResourceId(col_id)),
crates/amun-nft-stress/tests/n146_stress_tests.rs:13:    hasher.finalize().into()
crates/amun-nft-stress/tests/n146_stress_tests.rs:143:            lineage: ResourceLineage::single_ancestor(
crates/amun-nft-stress/tests/n146_stress_tests.rs:156:    constitutional.register(NftConstitutionalRecord {
crates/amun-nft-stress/tests/n146_stress_tests.rs:17:fn n146_stress_mint_1000_nfts() {
crates/amun-nft-stress/tests/n146_stress_tests.rs:183:    let root_before_sale = constitutional.compute_constitutional_root();
crates/amun-nft-stress/tests/n146_stress_tests.rs:18:    let mut reg = ResourceRegistry::new(10000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:195:        EnforcementEngine::enforce_royalty(&constitutional, &token_id, 1000).unwrap();
crates/amun-nft-stress/tests/n146_stress_tests.rs:1:use amun_nft_marketplace::MarketplaceEngine;
crates/amun-nft-stress/tests/n146_stress_tests.rs:208:    EnforcementEngine::transfer_governance(&mut constitutional, &token_id, &buyer);
crates/amun-nft-stress/tests/n146_stress_tests.rs:227:    assert!(!EnforcementEngine::can_be_sold(
crates/amun-nft-stress/tests/n146_stress_tests.rs:243:    assert!(EnforcementEngine::can_be_sold(
crates/amun-nft-stress/tests/n146_stress_tests.rs:24:        lineage: ResourceLineage::genesis(ResourceId(col_id)),
crates/amun-nft-stress/tests/n146_stress_tests.rs:250:    let root_after_all = constitutional.compute_constitutional_root();
crates/amun-nft-stress/tests/n146_stress_tests.rs:254:    let state_root = reg.compute_state_root();
crates/amun-nft-stress/tests/n146_stress_tests.rs:36:fn n146_stress_marketplace_rapid_trades() {
crates/amun-nft-stress/tests/n146_stress_tests.rs:37:    let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:43:        lineage: ResourceLineage::genesis(ResourceId(col_id)),
crates/amun-nft-stress/tests/n146_stress_tests.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-stress/tests/n146_stress_tests.rs:55:                lineage: ResourceLineage::single_ancestor(
crates/amun-nft-stress/tests/n146_stress_tests.rs:69:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-stress/tests/n146_stress_tests.rs:76:fn n146_stress_state_root_consistent_under_load() {
crates/amun-nft-stress/tests/n146_stress_tests.rs:77:    let mut reg1 = ResourceRegistry::new(1000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:78:    let mut reg2 = ResourceRegistry::new(1000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:85:            lineage: ResourceLineage::genesis(ResourceId(col_id)),
crates/amun-nft-stress/tests/n146_stress_tests.rs:94:    assert_eq!(reg1.compute_state_root(), reg2.compute_state_root());
crates/amun-nft-stress/tests/n146_stress_tests.rs:98:fn n147_full_constitutional_flow() {
crates/amun-nft-stress/tests/n146_stress_tests.rs:9:fn token_id(seed: u8, salt: u64) -> [u8; 32] {
crates/amun-node/src/bin/test_constitutional_determinism.rs:66:        println!("\nPASS: Constitutional runtime execution is deterministic");
crates/amun-node/src/bin/test_constitutional_multi_block.rs:125:        println!("\nPASS: Constitutional multi-block state evolution is deterministic");
crates/amun-node/src/bin/test_constitutional_mutation.rs:90:        println!("\nPASS: Constitutional state mutation is deterministic");
crates/amun-node/src/bin/test_constitutional_stress.rs:11:fn main() {
crates/amun-node/src/bin/test_constitutional_stress.rs:127:            "\nPASS: Constitutional stress test passed ({} blocks)",
crates/amun-node/src/bin/test_constitutional_stress.rs:2:use amun_bytecode::program::ConstitutionalProgram;
crates/amun-node/src/bin/test_constitutional_stress.rs:39:                lineage: ResourceLineage::genesis(resource_id),
crates/amun-node/src/bin/test_constitutional_stress.rs:3:use amun_constitutional_runtime::runtime_pipeline::ConstitutionalRuntime;
crates/amun-node/src/bin/test_constitutional_stress.rs:48:            let program = ConstitutionalProgram::new(
crates/amun-node/src/bin/test_constitutional_stress.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
crates/amun-node/src/bin/test_constitutional_stress.rs:72:            let mut hot = amun_proof_archive::hot_store::HotProofStore::new(1000);
crates/amun-node/src/bin/test_constitutional_stress.rs:73:            let mut archive = amun_proof_archive::proof_archive::ProofArchive::new();
crates/amun-node/src/bin/test_constitutional_stress.rs:75:            let result = ConstitutionalRuntime::execute(
crates/amun-node/src/bin/test_crash_recovery_7.rs:152:        println!("\nPASS: Network survived but did not reach target commits");
crates/amun-node/src/bin/test_replay_determinism.rs:10:        eprintln!("Usage: test_replay_determinism <temp_dir_prefix>");
crates/amun-node/src/bin/test_replay_determinism.rs:33:                lineage: ResourceLineage::genesis(id),
crates/amun-node/src/bin/test_replay_determinism.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
crates/amun-node/src/bin/test_replay_determinism.rs:52:    // Phase 2: Replay on 4 independent stores
crates/amun-node/src/bin/test_replay_determinism.rs:53:    let mut replay_roots = Vec::new();
crates/amun-node/src/bin/test_replay_determinism.rs:55:        let replay_dir = format!("{}/replay{}", prefix, v);
crates/amun-node/src/bin/test_replay_determinism.rs:56:        std::fs::create_dir_all(&replay_dir).expect("Failed to create dir");
crates/amun-node/src/bin/test_replay_determinism.rs:57:        let mut replay_store =
crates/amun-node/src/bin/test_replay_determinism.rs:58:            PersistentValidatorStore::open(&replay_dir).expect("Failed to open store");
crates/amun-node/src/bin/test_replay_determinism.rs:62:                replay_store
crates/amun-node/src/bin/test_replay_determinism.rs:67:            replay_store
crates/amun-node/src/bin/test_replay_determinism.rs:71:        let replay_root = replay_store.state_root();
crates/amun-node/src/bin/test_replay_determinism.rs:72:        println!("Replay {} root: {}", v, hex::encode(replay_root));
crates/amun-node/src/bin/test_replay_determinism.rs:73:        replay_roots.push(replay_root);
crates/amun-node/src/bin/test_replay_determinism.rs:76:    // Phase 3: Verify all replay roots match reference
crates/amun-node/src/bin/test_replay_determinism.rs:77:    let all_match = replay_roots.iter().all(|r| *r == ref_root);
crates/amun-node/src/bin/test_replay_determinism.rs:79:        println!("\nPASS: Replay determinism verified across all validators");
crates/amun-node/src/bin/test_replay_determinism.rs:7:fn main() {
crates/amun-node/src/bin/test_replay_determinism.rs:82:        println!("\nFAIL: Replay determinism violation");
crates/amun-node/src/bin/test_replay_determinism.rs:83:        for (i, r) in replay_roots.iter().enumerate() {
crates/amun-node/src/bin/test_replay_determinism.rs:85:                "Replay {} root: {} (match: {})",
crates/amun-node/src/bin/test_replay_stress.rs:103:                lineage: ResourceLineage::genesis(*resource_id),
crates/amun-node/src/bin/test_replay_stress.rs:107:            replay_store
crates/amun-node/src/bin/test_replay_stress.rs:118:                pre_state_root: replay_store.state_root(),
crates/amun-node/src/bin/test_replay_stress.rs:11:fn main() {
crates/amun-node/src/bin/test_replay_stress.rs:121:            let mut hot = amun_proof_archive::hot_store::HotProofStore::new(1000);
crates/amun-node/src/bin/test_replay_stress.rs:122:            let mut archive = amun_proof_archive::proof_archive::ProofArchive::new();
crates/amun-node/src/bin/test_replay_stress.rs:123:            ConstitutionalRuntime::execute(
crates/amun-node/src/bin/test_replay_stress.rs:126:                replay_store.registry_mut(),
crates/amun-node/src/bin/test_replay_stress.rs:132:            .expect("Replay execution failed");
crates/amun-node/src/bin/test_replay_stress.rs:133:            replay_store
crates/amun-node/src/bin/test_replay_stress.rs:135:                .expect("Replay advance failed");
crates/amun-node/src/bin/test_replay_stress.rs:137:        let replay_root = replay_store.state_root();
crates/amun-node/src/bin/test_replay_stress.rs:138:        println!("Replay {} final root: {}", v, hex::encode(replay_root));
crates/amun-node/src/bin/test_replay_stress.rs:139:        replay_roots.push(replay_root);
crates/amun-node/src/bin/test_replay_stress.rs:143:    let all_match = replay_roots.iter().all(|r| *r == ref_root);
crates/amun-node/src/bin/test_replay_stress.rs:145:        println!("\nPASS: Replay stress test passed ({} blocks)", block_count);
crates/amun-node/src/bin/test_replay_stress.rs:148:        println!("\nFAIL: Replay divergence detected");
crates/amun-node/src/bin/test_replay_stress.rs:149:        for (i, r) in replay_roots.iter().enumerate() {
crates/amun-node/src/bin/test_replay_stress.rs:14:        eprintln!("Usage: test_replay_stress <temp_dir_prefix>");
crates/amun-node/src/bin/test_replay_stress.rs:151:                "Replay {} root: {} (match: {})",
crates/amun-node/src/bin/test_replay_stress.rs:19:    let mut mutation_log: Vec<(ResourceId, ConstitutionalProgram)> = Vec::new();
crates/amun-node/src/bin/test_replay_stress.rs:2:use amun_bytecode::program::ConstitutionalProgram;
crates/amun-node/src/bin/test_replay_stress.rs:32:            lineage: ResourceLineage::genesis(resource_id),
crates/amun-node/src/bin/test_replay_stress.rs:3:use amun_constitutional_runtime::runtime_pipeline::ConstitutionalRuntime;
crates/amun-node/src/bin/test_replay_stress.rs:41:        let program = ConstitutionalProgram::new(
crates/amun-node/src/bin/test_replay_stress.rs:63:        let mut hot = amun_proof_archive::hot_store::HotProofStore::new(1000);
crates/amun-node/src/bin/test_replay_stress.rs:64:        let mut archive = amun_proof_archive::proof_archive::ProofArchive::new();
crates/amun-node/src/bin/test_replay_stress.rs:65:        ConstitutionalRuntime::execute(
crates/amun-node/src/bin/test_replay_stress.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
crates/amun-node/src/bin/test_replay_stress.rs:89:    // Phase 2: Replay on 4 independent stores from scratch
crates/amun-node/src/bin/test_replay_stress.rs:90:    let mut replay_roots = Vec::new();
crates/amun-node/src/bin/test_replay_stress.rs:92:        let replay_dir = format!("{}/replay{}", prefix, v);
crates/amun-node/src/bin/test_replay_stress.rs:93:        std::fs::create_dir_all(&replay_dir).expect("Failed to create dir");
crates/amun-node/src/bin/test_replay_stress.rs:94:        let mut replay_store =
crates/amun-node/src/bin/test_replay_stress.rs:95:            PersistentValidatorStore::open(&replay_dir).expect("Failed to open store");
crates/amun-node/src/certificate_loader.rs:107:    fn n106_0_missing_certificate_rejected() {
crates/amun-node/src/certificate_loader.rs:120:    fn n106_2_certificate_with_matching_genesis_passes() {
crates/amun-node/src/certificate_loader.rs:137:    fn n106_2_public_key_mismatch_rejected() {
crates/amun-node/src/certificate_loader.rs:160:    fn n106_0_tampered_certificate_rejected() {
crates/amun-node/src/certificate_loader.rs:184:    fn n106_1_validator_not_in_genesis_rejected() {
crates/amun-node/src/genesis.rs:135:    fn n22_3_valid_genesis_accepted() {
crates/amun-node/src/genesis.rs:137:        assert!(genesis.validate().is_ok());
crates/amun-node/src/genesis.rs:141:    fn n22_3_duplicate_validator_rejected() {
crates/amun-node/src/genesis.rs:144:        assert!(genesis.validate().is_err());
crates/amun-node/src/genesis.rs:148:    fn n22_3_empty_validator_set_rejected() {
crates/amun-node/src/genesis.rs:151:        assert!(genesis.validate().is_err());
crates/amun-node/src/genesis.rs:155:    fn n22_3_duplicate_trust_anchor_rejected() {
crates/amun-node/src/genesis.rs:161:        assert!(genesis.validate().is_err());
crates/amun-node/src/genesis.rs:165:    fn n22_3_genesis_hash_deterministic() {
crates/amun-node/src/genesis.rs:173:    fn n22_3_different_validator_set_changes_hash() {
crates/amun-node/src/genesis.rs:181:    fn n22_3_empty_chain_id_rejected() {
crates/amun-node/src/genesis.rs:184:        assert!(genesis.validate().is_err());
crates/amun-node/src/genesis.rs:188:    fn n22_3_zero_voting_power_rejected() {
crates/amun-node/src/genesis.rs:191:        assert!(genesis.validate().is_err());
crates/amun-node/src/peer_handshake.rs:109:    fn n22_5_valid_handshake_accepted() {
crates/amun-node/src/peer_handshake.rs:111:        assert!(handshake.verify(&genesis_hash).is_ok());
crates/amun-node/src/peer_handshake.rs:115:    fn n22_5_genesis_mismatch_rejected() {
crates/amun-node/src/peer_handshake.rs:118:        assert!(handshake.verify(&wrong_genesis).is_err());
crates/amun-node/src/peer_handshake.rs:122:    fn n22_5_tampered_certificate_rejected() {
crates/amun-node/src/peer_handshake.rs:125:        assert!(handshake.verify(&genesis_hash).is_err());
crates/amun-node/src/peer_handshake.rs:129:    fn n22_5_authenticated_peer_creation() {
crates/amun-node/src/peer_registry.rs:69:    fn n22_6_register_and_lookup_peer() {
crates/amun-node/src/peer_registry.rs:79:    fn n22_6_duplicate_peer_rejected() {
crates/amun-node/src/peer_registry.rs:88:    fn n22_6_remove_peer() {
crates/amun-node/src/peer_registry.rs:98:    fn n22_6_multiple_peers() {
crates/amun-operations/src/backup_recovery.rs:113:        assert!(backup.verify());
crates/amun-operations/src/backup_recovery.rs:117:    fn n62_backup_restore() {
crates/amun-operations/src/backup_recovery.rs:141:        assert_eq!(restored.compute_state_root(), state_root);
crates/amun-operations/src/backup_recovery.rs:91:    fn n62_backup_create_and_verify() {
crates/amun-operations/src/health_check.rs:54:    fn n62_health_syncing_when_not_synced() {
crates/amun-operations/src/health_check.rs:61:    fn n62_health_healthy_when_synced_with_peers() {
crates/amun-operations/src/metrics.rs:11:    pub replays_performed: u64,
crates/amun-operations/src/metrics.rs:39:    pub fn record_replay(&mut self) {
crates/amun-operations/src/metrics.rs:40:        self.replays_performed += 1;
crates/amun-operations/src/metrics.rs:54:            ("replays_performed", self.replays_performed),
crates/amun-operations/src/metrics.rs:65:    fn n62_metrics_record_and_summarize() {
crates/amun-operations/src/metrics.rs:70:        metrics.record_replay();
crates/amun-pccv/src/lib.rs:106:        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("R1")));
crates/amun-pccv/src/lib.rs:110:    fn n49_detect_r6_version_regression() {
crates/amun-pccv/src/lib.rs:158:        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("R6")));
crates/amun-pccv/src/lib.rs:162:    fn n49_detect_t1_illegal_transformation() {
crates/amun-pccv/src/lib.rs:210:        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("T1")));
crates/amun-pccv/src/lib.rs:214:    fn n49_full_semantic_verification_passes() {
crates/amun-pccv/src/lib.rs:268:        assert!(matches!(result, PCCVResult::Verified { .. }));
crates/amun-pccv/src/lib.rs:27:    fn n49_structural_verify_empty() {
crates/amun-pccv/src/lib.rs:51:        assert!(matches!(result, PCCVResult::Verified { .. }));
crates/amun-pccv/src/lib.rs:55:    fn n49_detect_r1_duplicate_id() {
crates/amun-pccv/src/transition_proof_engine.rs:119:    fn n49b_build_and_verify_simple_transition() {
crates/amun-pccv/src/transition_proof_engine.rs:166:        assert!(matches!(result, PCCVResult::Verified { .. }));
crates/amun-pccv/src/transition_proof_engine.rs:170:    fn n49b_detect_illegal_transition_in_proof() {
crates/amun-pccv/src/transition_proof_engine.rs:235:        assert!(matches!(result, PCCVResult::Failed { ref reason } if reason.contains("T1")));
crates/amun-pccv/src/transition_proof_engine.rs:239:    fn n49b_proof_hash_deterministic() {
crates/amun-pccv/tests/replay_equivalence.rs:100:    let result1 = amun_pccv::pccv_verifier::PCCVVerifier::verify(&proof1, &reg1);
crates/amun-pccv/tests/replay_equivalence.rs:101:    let result2 = amun_pccv::pccv_verifier::PCCVVerifier::verify(&proof2, &reg2);
crates/amun-pccv/tests/replay_equivalence.rs:102:    assert!(matches!(result1, PCCVResult::Verified { .. }));
crates/amun-pccv/tests/replay_equivalence.rs:103:    assert!(matches!(result2, PCCVResult::Verified { .. }));
crates/amun-pccv/tests/replay_equivalence.rs:107:fn n49c_replay_consistent_across_iterations() {
crates/amun-pccv/tests/replay_equivalence.rs:108:    let mut reg = ResourceRegistry::new(1000);
crates/amun-pccv/tests/replay_equivalence.rs:113:        lineage: ResourceLineage::genesis(make_id(1)),
crates/amun-pccv/tests/replay_equivalence.rs:118:    let pre_root = reg.compute_state_root();
crates/amun-pccv/tests/replay_equivalence.rs:119:    let parent_hash = ResourceRegistry::hash_resource(reg.get(&make_id(1)).unwrap());
crates/amun-pccv/tests/replay_equivalence.rs:126:        archetype: ResourceArchetype::ConstitutionalAsset,
crates/amun-pccv/tests/replay_equivalence.rs:128:        lineage: ResourceLineage::transformation(child_id, make_id(1), parent_hash, 2),
crates/amun-pccv/tests/replay_equivalence.rs:144:            TransitionProofEngine::build_proof(
crates/amun-pccv/tests/replay_equivalence.rs:16:fn n49c_replay_produces_identical_proof() {
crates/amun-pccv/tests/replay_equivalence.rs:17:    let mut reg1 = ResourceRegistry::new(1000);
crates/amun-pccv/tests/replay_equivalence.rs:18:    let mut reg2 = ResourceRegistry::new(1000);
crates/amun-pccv/tests/replay_equivalence.rs:1:use amun_pccv::pccv_verifier::PCCVResult;
crates/amun-pccv/tests/replay_equivalence.rs:24:        lineage: ResourceLineage::genesis(make_id(1)),
crates/amun-pccv/tests/replay_equivalence.rs:2:use amun_pccv::transition_proof_engine::TransitionProofEngine;
crates/amun-pccv/tests/replay_equivalence.rs:33:    let pre_root1 = reg1.compute_state_root();
crates/amun-pccv/tests/replay_equivalence.rs:34:    let pre_root2 = reg2.compute_state_root();
crates/amun-pccv/tests/replay_equivalence.rs:37:    let parent_hash = ResourceRegistry::hash_resource(reg1.get(&make_id(1)).unwrap());
crates/amun-pccv/tests/replay_equivalence.rs:40:        archetype: ResourceArchetype::ConstitutionalAsset,
crates/amun-pccv/tests/replay_equivalence.rs:42:        lineage: ResourceLineage::transformation(child_id, make_id(1), parent_hash, 2),
crates/amun-pccv/tests/replay_equivalence.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-pccv/tests/replay_equivalence.rs:73:    let proof1 = TransitionProofEngine::build_proof(
crates/amun-pccv/tests/replay_equivalence.rs:84:    let proof2 = TransitionProofEngine::build_proof(
crates/amun-pccv/tests/replay_equivalence.rs:9:fn make_id(seed: u8) -> ResourceId {
crates/amun-peer-discovery/src/discovery.rs:225:    fn n73_discover_single_peer() {
crates/amun-peer-discovery/src/discovery.rs:247:    fn n73_self_not_added() {
crates/amun-peer-discovery/src/discovery.rs:262:    fn n73_merge_peer_lists() {
crates/amun-peer-discovery/src/discovery.rs:284:    fn n73_peer_announce() {
crates/amun-peer-discovery/src/discovery.rs:299:    fn n73_duplicate_peer_not_duplicated() {
crates/amun-peer-discovery/src/peer_table.rs:136:    fn n73_add_and_lookup_peer() {
crates/amun-peer-discovery/src/peer_table.rs:147:    fn n73_duplicate_peer_updated() {
crates/amun-peer-discovery/src/peer_table.rs:161:    fn n73_remove_peer() {
crates/amun-peer-discovery/src/peer_table.rs:172:    fn n73_max_peers_enforced() {
crates/amun-peer-discovery/src/peer_table.rs:185:    fn n73_peer_expiry() {
crates/amun-peer-identity/tests/peer_identity_tests.rs:25:    assert!(IdentityVerifier::verify(&cert, GENESIS).is_ok());
crates/amun-peer-identity/tests/peer_identity_tests.rs:33:    assert!(IdentityVerifier::verify(&cert, GENESIS).is_err());
crates/amun-persistence/src/lib.rs:108:    fn n42_persistence_survives_reload() {
crates/amun-persistence/src/lib.rs:70:    fn n42_persistence_roundtrip() {
crates/amun-persistence/src/lib.rs:91:        assert_eq!(restored.last_commit_hash, "jkl012");
crates/amun-persistence/src/lib.rs:97:    fn n42_genesis_on_missing_file() {
crates/amun-persistent-node/src/persistent_store.rs:140:    fn n63_open_new_store() {
crates/amun-persistent-node/src/persistent_store.rs:147:    fn n63_save_and_restore() {
crates/amun-persistent-node/src/persistent_store.rs:175:    fn n63_crash_recovery_full_cycle() {
crates/amun-persistent-node/src/persistent_store.rs:203:    fn n63_backup_tampering_detected() {
crates/amun-persistent-node/src/persistent_store.rs:212:        assert!(backup.verify());
crates/amun-persistent-node/src/persistent_store.rs:214:        assert!(!backup.verify(), "Tampered backup should fail verification");
crates/amun-persistent-node/src/persistent_store.rs:218:    fn n63_open_rejects_corrupted_file() {
crates/amun-proof-carrying/src/lib.rs:9://! without a trusted node or full chain replay.
crates/amun-recovery/src/lib.rs:116:        let record = ReplayRecord {
crates/amun-recovery/src/lib.rs:131:        assert_eq!(recovered.replay_count, 1);
crates/amun-recovery/src/lib.rs:137:    fn n46_recover_from_genesis() {
crates/amun-recovery/src/lib.rs:145:        assert_eq!(recovered.replay_count, 0);
crates/amun-recovery/src/lib.rs:151:    fn n46_full_persistence_pipeline() {
crates/amun-recovery/src/lib.rs:15:    pub replay_count: usize,
crates/amun-recovery/src/lib.rs:173:            let record = ReplayRecord {
crates/amun-recovery/src/lib.rs:186:        assert_eq!(recovered.replay_count, 3);
crates/amun-recovery/src/lib.rs:23:    replay: ReplayStore,
crates/amun-recovery/src/lib.rs:32:            replay: ReplayStore::new(&format!("{}/replay.json", data_dir)),
crates/amun-recovery/src/lib.rs:3:use amun_replay_store::{ReplayRecord, ReplayStore};
crates/amun-recovery/src/lib.rs:41:        record: &ReplayRecord,
crates/amun-recovery/src/lib.rs:46:        self.replay.append(record)?;
crates/amun-recovery/src/lib.rs:55:        let replay_records = self.replay.load_all().unwrap_or_default();
crates/amun-recovery/src/lib.rs:67:        // Verify replay chain integrity
crates/amun-recovery/src/lib.rs:68:        if !self.replay.verify_chain().unwrap_or(false) {
crates/amun-recovery/src/lib.rs:69:            return Err("Replay chain verification failed".into());
crates/amun-recovery/src/lib.rs:79:            replay_count: replay_records.len(),
crates/amun-recovery/src/lib.rs:94:    fn n46_save_and_recover() {
crates/amun-replay-cert/build.rs:18:    let commit_hash = rustc_verbose
crates/amun-replay-cert/build.rs:20:        .find(|l| l.starts_with("commit-hash:"))
crates/amun-replay-cert/build.rs:21:        .map(|l| l.replace("commit-hash: ", ""))
crates/amun-replay-cert/build.rs:31:    println!("cargo:rustc-env=RUSTC_COMMIT={}", commit_hash);
crates/amun-replay-cert/build.rs:3:fn main() {
crates/amun-replay-cert/src/certificate.rs:19:impl ReplayCertificate {
crates/amun-replay-cert/src/certificate.rs:20:    pub fn new(
crates/amun-replay-cert/src/certificate.rs:43:        certificate_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);
crates/amun-replay-cert/src/certificate.rs:59:    pub fn verify(&self) -> bool {
crates/amun-replay-cert/src/certificate.rs:6:pub struct ReplayCertificate {
crates/amun-replay-cert/src/certificate.rs:71:        let mut computed = [0u8; 32];
crates/amun-replay-cert/src/certificate.rs:72:        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
crates/amun-replay-cert/src/certificate.rs:73:        computed == self.certificate_hash
crates/amun-replay-cert/src/certifier.rs:107:pub struct PlatformResult {
crates/amun-replay-cert/src/certifier.rs:14:impl ReplayCertifier {
crates/amun-replay-cert/src/certifier.rs:15:    pub fn new(engine: TruthEngine) -> Self {
crates/amun-replay-cert/src/certifier.rs:18:            transcript: ReplayTranscript::new(),
crates/amun-replay-cert/src/certifier.rs:24:    pub fn record_platform(&mut self, platform: PlatformFingerprint) {
crates/amun-replay-cert/src/certifier.rs:28:    pub fn certify(&mut self, target_height: u64) -> Result<CertificationResult, DivergenceReport> {
crates/amun-replay-cert/src/certifier.rs:33:                .compute_chain_root(target_height)
crates/amun-replay-cert/src/certifier.rs:3:use crate::transcript::ReplayTranscript;
crates/amun-replay-cert/src/certifier.rs:47:                    .compute_chain_root(target_height)
crates/amun-replay-cert/src/certifier.rs:4:use amun_truth_engine::TruthEngine;
crates/amun-replay-cert/src/certifier.rs:7:pub struct ReplayCertifier {
crates/amun-replay-cert/src/certifier.rs:82:            hash.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
crates/amun-replay-cert/src/certifier.rs:87:        self.transcript.finalize(reference_root);
crates/amun-replay-cert/src/certifier.rs:8:    engine: TruthEngine,
crates/amun-replay-cert/src/certifier.rs:99:pub struct CertificationResult {
crates/amun-replay-cert/src/certifier.rs:9:    transcript: ReplayTranscript,
crates/amun-replay-cert/src/divergence.rs:4:pub struct DivergenceReport {
crates/amun-replay-cert/src/lib.rs:10:pub use transcript::ReplayTranscript;
crates/amun-replay-cert/src/lib.rs:12:impl Default for ReplayTranscript {
crates/amun-replay-cert/src/lib.rs:13:    fn default() -> Self {
crates/amun-replay-cert/src/lib.rs:7:pub use certifier::ReplayCertifier;
crates/amun-replay-cert/src/platform.rs:12:impl PlatformFingerprint {
crates/amun-replay-cert/src/platform.rs:13:    pub fn current() -> Self {
crates/amun-replay-cert/src/platform.rs:24:            rustc_commit: env!("RUSTC_COMMIT").to_string(),
crates/amun-replay-cert/src/platform.rs:2:pub struct PlatformFingerprint {
crates/amun-replay-cert/src/platform.rs:35:    pub fn name(&self) -> String {
crates/amun-replay-cert/src/platform.rs:41:            &self.rustc_commit[..8.min(self.rustc_commit.len())]
crates/amun-replay-cert/src/platform.rs:45:    pub fn tag(&self) -> Vec<u8> {
crates/amun-replay-cert/src/platform.rs:7:    pub rustc_commit: String,
crates/amun-replay-cert/src/transcript.rs:10:pub struct TranscriptEntry {
crates/amun-replay-cert/src/transcript.rs:15:    pub transition_hash: [u8; 32],
crates/amun-replay-cert/src/transcript.rs:18:impl ReplayTranscript {
crates/amun-replay-cert/src/transcript.rs:19:    pub fn new() -> Self {
crates/amun-replay-cert/src/transcript.rs:26:    pub fn record(
crates/amun-replay-cert/src/transcript.rs:36:        payload_hash.copy_from_slice(&payload_hasher.finalize().as_bytes()[..32]);
crates/amun-replay-cert/src/transcript.rs:38:        let mut transition_hasher = Hasher::new();
crates/amun-replay-cert/src/transcript.rs:39:        transition_hasher.update(b"AMUN_TRANSITION_V1");
crates/amun-replay-cert/src/transcript.rs:40:        transition_hasher.update(&sequence.to_le_bytes());
crates/amun-replay-cert/src/transcript.rs:41:        transition_hasher.update(&from_root);
crates/amun-replay-cert/src/transcript.rs:42:        transition_hasher.update(&to_root);
crates/amun-replay-cert/src/transcript.rs:43:        let mut transition_hash = [0u8; 32];
crates/amun-replay-cert/src/transcript.rs:44:        transition_hash.copy_from_slice(&transition_hasher.finalize().as_bytes()[..32]);
crates/amun-replay-cert/src/transcript.rs:4:pub struct ReplayTranscript {
crates/amun-replay-cert/src/transcript.rs:51:            transition_hash,
crates/amun-replay-cert/src/transcript.rs:55:    pub fn finalize(&mut self, root: [u8; 32]) {
crates/amun-replay-cert/src/transcript.rs:59:    pub fn verify_continuity(&self) -> bool {
crates/amun-replay-cert/src/transcript.rs:72:    pub fn reset(&mut self) {
crates/amun-replay-cert/src/verifier.rs:1:use crate::certificate::ReplayCertificate;
crates/amun-replay-cert/src/verifier.rs:3:/// Verify a replay certificate. Returns Ok if the certificate
crates/amun-replay-cert/src/verifier.rs:5:pub fn verify_certificate(cert: &ReplayCertificate) -> Result<(), &'static str> {
crates/amun-replay-cert/src/verifier.rs:6:    if !cert.verify() {
crates/amun-replay-consensus/src/lib.rs:1:pub mod replay_backed_consensus;
crates/amun-replay-consensus/src/lib.rs:2:pub mod replay_backed_types;
crates/amun-replay-consensus/src/lib.rs:4:pub use replay_backed_consensus::*;
crates/amun-replay-consensus/src/lib.rs:5:pub use replay_backed_types::*;
crates/amun-replay-consensus/src/replay_backed_consensus.rs:105:        hasher.update(&block.replay_root);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:106:        let hash = hasher.finalize();
crates/amun-replay-consensus/src/replay_backed_consensus.rs:10:use crate::replay_backed_types::{
crates/amun-replay-consensus/src/replay_backed_consensus.rs:114:    /// Form consensus on a replay-verified block.
crates/amun-replay-consensus/src/replay_backed_consensus.rs:115:    pub fn form_consensus(
crates/amun-replay-consensus/src/replay_backed_consensus.rs:116:        block: &ReplayVerifiedBlock,
crates/amun-replay-consensus/src/replay_backed_consensus.rs:119:    ) -> Result<ReplayBackedFinalityCertificate, String> {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:11:    ReplayBackedFinalityCertificate, ReplayBackedQC, ReplayVerificationRecord, ReplayVerifiedBlock,
crates/amun-replay-consensus/src/replay_backed_consensus.rs:121:            return Err("Not all transition proofs passed replay verification".into());
crates/amun-replay-consensus/src/replay_backed_consensus.rs:124:        let mut qc = ReplayBackedQC::for_block(block, quorum_size);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:137:        Ok(ReplayBackedFinalityCertificate::issue(block, qc))
crates/amun-replay-consensus/src/replay_backed_consensus.rs:140:    fn compute_proof_root(transitions: &[TransitionProof]) -> [u8; 32] {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:143:        for proof in transitions {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:146:        let hash = hasher.finalize();
crates/amun-replay-consensus/src/replay_backed_consensus.rs:14:/// Replay-Backed Consensus Engine.
crates/amun-replay-consensus/src/replay_backed_consensus.rs:159:    fn make_id(seed: u8) -> ResourceId {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:15:/// Requires every validator to replay and verify all transition proofs
crates/amun-replay-consensus/src/replay_backed_consensus.rs:166:    fn w18_execute_and_replay_block() {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:167:        let mut registry = ResourceRegistry::new(10000);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:168:        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:175:            pre_state_root: registry.compute_state_root(),
crates/amun-replay-consensus/src/replay_backed_consensus.rs:178:        let block = ReplayBackedConsensus::execute_and_replay(
crates/amun-replay-consensus/src/replay_backed_consensus.rs:17:pub struct ReplayBackedConsensus;
crates/amun-replay-consensus/src/replay_backed_consensus.rs:187:        assert_eq!(block.replay_verifications.len(), 1);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:188:        assert!(block.replay_verifications[0].is_verified());
crates/amun-replay-consensus/src/replay_backed_consensus.rs:192:    fn w18_form_replay_backed_consensus() {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:193:        let mut registry = ResourceRegistry::new(10000);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:194:        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:19:impl ReplayBackedConsensus {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:1:use amun_bytecode::program::ConstitutionalProgram;
crates/amun-replay-consensus/src/replay_backed_consensus.rs:201:            pre_state_root: registry.compute_state_root(),
crates/amun-replay-consensus/src/replay_backed_consensus.rs:204:        let block = ReplayBackedConsensus::execute_and_replay(
crates/amun-replay-consensus/src/replay_backed_consensus.rs:20:    /// Execute a block and produce replay verifications for all transitions.
crates/amun-replay-consensus/src/replay_backed_consensus.rs:213:        let cert = ReplayBackedConsensus::form_consensus(&block, 5, sigs).unwrap();
crates/amun-replay-consensus/src/replay_backed_consensus.rs:214:        assert!(cert.verify());
crates/amun-replay-consensus/src/replay_backed_consensus.rs:218:    fn w18_reject_if_replay_fails() {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:219:        let mut registry = ResourceRegistry::new(10000);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:21:    pub fn execute_and_replay(
crates/amun-replay-consensus/src/replay_backed_consensus.rs:220:        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:227:            pre_state_root: registry.compute_state_root(),
crates/amun-replay-consensus/src/replay_backed_consensus.rs:22:        programs: &[(ConstitutionalProgram, ExecutionContext)],
crates/amun-replay-consensus/src/replay_backed_consensus.rs:230:        let mut block = ReplayBackedConsensus::execute_and_replay(
crates/amun-replay-consensus/src/replay_backed_consensus.rs:239:        block.replay_verifications[0].replay_success = false;
crates/amun-replay-consensus/src/replay_backed_consensus.rs:23:        registry: &mut ResourceRegistry,
crates/amun-replay-consensus/src/replay_backed_consensus.rs:242:        assert!(ReplayBackedConsensus::form_consensus(&block, 5, sigs).is_err());
crates/amun-replay-consensus/src/replay_backed_consensus.rs:246:    fn w18_replay_finality_certificate_deterministic() {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:247:        let mut registry = ResourceRegistry::new(10000);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:248:        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:255:            pre_state_root: registry.compute_state_root(),
crates/amun-replay-consensus/src/replay_backed_consensus.rs:258:        let block = ReplayBackedConsensus::execute_and_replay(
crates/amun-replay-consensus/src/replay_backed_consensus.rs:267:        let cert1 = ReplayBackedConsensus::form_consensus(&block, 5, sigs.clone()).unwrap();
crates/amun-replay-consensus/src/replay_backed_consensus.rs:268:        let cert2 = ReplayBackedConsensus::form_consensus(&block, 5, sigs).unwrap();
crates/amun-replay-consensus/src/replay_backed_consensus.rs:273:    fn w18_replay_root_included_in_block() {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:274:        let mut registry = ResourceRegistry::new(10000);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:275:        let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:27:    ) -> Result<ReplayVerifiedBlock, String> {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:282:            pre_state_root: registry.compute_state_root(),
crates/amun-replay-consensus/src/replay_backed_consensus.rs:285:        let block = ReplayBackedConsensus::execute_and_replay(
crates/amun-replay-consensus/src/replay_backed_consensus.rs:28:        let mut hot = HotProofStore::new(10000);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:293:        assert_eq!(block.replay_root, block.compute_replay_root());
crates/amun-replay-consensus/src/replay_backed_consensus.rs:294:        assert_ne!(block.replay_root, [0u8; 32]);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:29:        let mut archive = ProofArchive::new();
crates/amun-replay-consensus/src/replay_backed_consensus.rs:2:use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
crates/amun-replay-consensus/src/replay_backed_consensus.rs:30:        let mut transitions = Vec::new();
crates/amun-replay-consensus/src/replay_backed_consensus.rs:31:        let mut replay_records = Vec::new();
crates/amun-replay-consensus/src/replay_backed_consensus.rs:34:            let result = ConstitutionalRuntime::execute(
crates/amun-replay-consensus/src/replay_backed_consensus.rs:3:use amun_proof_archive::hot_store::HotProofStore;
crates/amun-replay-consensus/src/replay_backed_consensus.rs:47:                    transition_proof, ..
crates/amun-replay-consensus/src/replay_backed_consensus.rs:4:use amun_proof_archive::proof_archive::ProofArchive;
crates/amun-replay-consensus/src/replay_backed_consensus.rs:50:                    transition_proof, ..
crates/amun-replay-consensus/src/replay_backed_consensus.rs:51:                } => transition_proof,
crates/amun-replay-consensus/src/replay_backed_consensus.rs:54:            // Replay verification
crates/amun-replay-consensus/src/replay_backed_consensus.rs:55:            let mut fresh_reg = ResourceRegistry::new(10000);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:56:            let replay = ReplayVerifier::replay(&proof, program, &mut fresh_reg, &[]);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:58:            let record = match replay {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:59:                ReplayResult::Match {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:5:use amun_replay_verifier::replay_verifier::{ReplayResult, ReplayVerifier};
crates/amun-replay-consensus/src/replay_backed_consensus.rs:62:                } => ReplayVerificationRecord {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:67:                    replay_success: true,
crates/amun-replay-consensus/src/replay_backed_consensus.rs:69:                _ => ReplayVerificationRecord {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:6:use amun_resource_core::{ResourceId, ResourceRegistry};
crates/amun-replay-consensus/src/replay_backed_consensus.rs:74:                    replay_success: false,
crates/amun-replay-consensus/src/replay_backed_consensus.rs:78:            replay_records.push(record);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:79:            transitions.push(proof);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:7:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-replay-consensus/src/replay_backed_consensus.rs:82:        let state_root = registry.compute_state_root();
crates/amun-replay-consensus/src/replay_backed_consensus.rs:83:        let proof_root = Self::compute_proof_root(&transitions);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:84:        let all_verified = replay_records.iter().all(|r| r.is_verified());
crates/amun-replay-consensus/src/replay_backed_consensus.rs:86:        let mut block = ReplayVerifiedBlock {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:91:            replay_root: [0u8; 32],
crates/amun-replay-consensus/src/replay_backed_consensus.rs:92:            transitions,
crates/amun-replay-consensus/src/replay_backed_consensus.rs:93:            replay_verifications: replay_records,
crates/amun-replay-consensus/src/replay_backed_consensus.rs:96:        block.replay_root = block.compute_replay_root();
crates/amun-replay-consensus/src/replay_backed_types.rs:103:            replay_root: block.replay_root,
crates/amun-replay-consensus/src/replay_backed_types.rs:107:        cert.certificate_hash = cert.compute_hash();
crates/amun-replay-consensus/src/replay_backed_types.rs:111:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-replay-consensus/src/replay_backed_types.rs:118:        hasher.update(&self.replay_root);
crates/amun-replay-consensus/src/replay_backed_types.rs:119:        let hash = hasher.finalize();
crates/amun-replay-consensus/src/replay_backed_types.rs:11:    pub replay_success: bool,
crates/amun-replay-consensus/src/replay_backed_types.rs:125:    pub fn verify(&self) -> bool {
crates/amun-replay-consensus/src/replay_backed_types.rs:126:        self.certificate_hash == self.compute_hash() && self.qc.is_valid()
crates/amun-replay-consensus/src/replay_backed_types.rs:14:impl ReplayVerificationRecord {
crates/amun-replay-consensus/src/replay_backed_types.rs:16:    pub fn is_verified(&self) -> bool {
crates/amun-replay-consensus/src/replay_backed_types.rs:17:        self.replay_success && self.state_root_match && self.proof_hash_match && self.gas_used_match
crates/amun-replay-consensus/src/replay_backed_types.rs:1:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-replay-consensus/src/replay_backed_types.rs:21:/// A block that has been verified through deterministic replay.
crates/amun-replay-consensus/src/replay_backed_types.rs:23:pub struct ReplayVerifiedBlock {
crates/amun-replay-consensus/src/replay_backed_types.rs:28:    pub replay_root: [u8; 32],
crates/amun-replay-consensus/src/replay_backed_types.rs:29:    pub transitions: Vec<TransitionProof>,
crates/amun-replay-consensus/src/replay_backed_types.rs:30:    pub replay_verifications: Vec<ReplayVerificationRecord>,
crates/amun-replay-consensus/src/replay_backed_types.rs:34:impl ReplayVerifiedBlock {
crates/amun-replay-consensus/src/replay_backed_types.rs:35:    /// Compute the replay root — Merkle root of all replay verification hashes.
crates/amun-replay-consensus/src/replay_backed_types.rs:36:    pub fn compute_replay_root(&self) -> [u8; 32] {
crates/amun-replay-consensus/src/replay_backed_types.rs:39:        for rv in &self.replay_verifications {
crates/amun-replay-consensus/src/replay_backed_types.rs:41:            hasher.update(&[rv.replay_success as u8]);
crates/amun-replay-consensus/src/replay_backed_types.rs:43:        let hash = hasher.finalize();
crates/amun-replay-consensus/src/replay_backed_types.rs:4:/// A replay verification result for a single transition proof.
crates/amun-replay-consensus/src/replay_backed_types.rs:50:/// A QC that includes replay verification.
crates/amun-replay-consensus/src/replay_backed_types.rs:52:pub struct ReplayBackedQC {
crates/amun-replay-consensus/src/replay_backed_types.rs:57:    pub replay_root: [u8; 32],
crates/amun-replay-consensus/src/replay_backed_types.rs:61:    pub all_replays_verified: bool,
crates/amun-replay-consensus/src/replay_backed_types.rs:64:impl ReplayBackedQC {
crates/amun-replay-consensus/src/replay_backed_types.rs:65:    pub fn is_valid(&self) -> bool {
crates/amun-replay-consensus/src/replay_backed_types.rs:66:        self.signer_count >= self.quorum_threshold && self.all_replays_verified
crates/amun-replay-consensus/src/replay_backed_types.rs:69:    pub fn for_block(block: &ReplayVerifiedBlock, threshold: usize) -> Self {
crates/amun-replay-consensus/src/replay_backed_types.rs:6:pub struct ReplayVerificationRecord {
crates/amun-replay-consensus/src/replay_backed_types.rs:75:            replay_root: block.replay_root,
crates/amun-replay-consensus/src/replay_backed_types.rs:79:            all_replays_verified: block.all_verified,
crates/amun-replay-consensus/src/replay_backed_types.rs:84:/// A constitutional finality certificate backed by replay verification.
crates/amun-replay-consensus/src/replay_backed_types.rs:86:pub struct ReplayBackedFinalityCertificate {
crates/amun-replay-consensus/src/replay_backed_types.rs:91:    pub replay_root: [u8; 32],
crates/amun-replay-consensus/src/replay_backed_types.rs:92:    pub qc: ReplayBackedQC,
crates/amun-replay-consensus/src/replay_backed_types.rs:96:impl ReplayBackedFinalityCertificate {
crates/amun-replay-consensus/src/replay_backed_types.rs:97:    pub fn issue(block: &ReplayVerifiedBlock, qc: ReplayBackedQC) -> Self {
crates/amun-replay-engine/src/adaptive_relay.rs:101:        RoutedProofType::EquivalenceFingerprint => CompressionDecision::SendFingerprint,
crates/amun-replay-engine/src/adaptive_relay.rs:102:        RoutedProofType::FrontierDescription => {
crates/amun-replay-engine/src/adaptive_relay.rs:109:        RoutedProofType::ClosureDelta => CompressionDecision::SendDelta,
crates/amun-replay-engine/src/adaptive_relay.rs:10:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/adaptive_relay.rs:110:        RoutedProofType::FullWitness => {
crates/amun-replay-engine/src/adaptive_relay.rs:122:pub struct RelayBackpressure {
crates/amun-replay-engine/src/adaptive_relay.rs:12:use crate::proof_routing::RoutedProofType;
crates/amun-replay-engine/src/adaptive_relay.rs:131:impl RelayBackpressure {
crates/amun-replay-engine/src/adaptive_relay.rs:132:    pub fn new(max_concurrent: usize) -> Self {
crates/amun-replay-engine/src/adaptive_relay.rs:137:    pub fn can_accept(&self) -> bool {
crates/amun-replay-engine/src/adaptive_relay.rs:142:    pub fn update(&mut self) {
crates/amun-replay-engine/src/adaptive_relay.rs:149:pub struct EquivalenceRelayCache {
crates/amun-replay-engine/src/adaptive_relay.rs:151:    pub fingerprints: Vec<ConstitutionalHash>,
crates/amun-replay-engine/src/adaptive_relay.rs:154:impl EquivalenceRelayCache {
crates/amun-replay-engine/src/adaptive_relay.rs:155:    pub fn new() -> Self { Self { fingerprints: Vec::new() } }
crates/amun-replay-engine/src/adaptive_relay.rs:157:    pub fn has(&self, fp: &ConstitutionalHash) -> bool {
crates/amun-replay-engine/src/adaptive_relay.rs:161:    pub fn insert(&mut self, fp: ConstitutionalHash) {
crates/amun-replay-engine/src/adaptive_relay.rs:16:pub struct RelayTopologyView {
crates/amun-replay-engine/src/adaptive_relay.rs:173:pub struct AntiCentralityGuard {
crates/amun-replay-engine/src/adaptive_relay.rs:180:impl AntiCentralityGuard {
crates/amun-replay-engine/src/adaptive_relay.rs:181:    pub fn new(threshold: f64) -> Self {
crates/amun-replay-engine/src/adaptive_relay.rs:186:    pub fn record_route(&mut self, relay_id: u64) {
crates/amun-replay-engine/src/adaptive_relay.rs:195:    pub fn is_centralized(&self) -> bool {
crates/amun-replay-engine/src/adaptive_relay.rs:1://! Adaptive Proof Relay — topology-aware delivery without semantic preference.
crates/amun-replay-engine/src/adaptive_relay.rs:203:    pub fn dominant_relay(&self) -> Option<u64> {
crates/amun-replay-engine/src/adaptive_relay.rs:215:    fn make_frontier(worker: u64, unresolved: Vec<([u8; 32], FrontierDependencyReason, bool)>) -> DerivationalFrontier {
crates/amun-replay-engine/src/adaptive_relay.rs:220:        f.recompute_completeness();
crates/amun-replay-engine/src/adaptive_relay.rs:225:    fn test_relay_selection_avoids_congested() {
crates/amun-replay-engine/src/adaptive_relay.rs:234:    fn test_relay_avoids_avoid_list() {
crates/amun-replay-engine/src/adaptive_relay.rs:23:pub struct RelayMetrics {
crates/amun-replay-engine/src/adaptive_relay.rs:243:    fn test_frontier_pressure() {
crates/amun-replay-engine/src/adaptive_relay.rs:244:        let f1 = make_frontier(100, vec![([0x01; 32], FrontierDependencyReason::HardWitnessRequired, true)]);
crates/amun-replay-engine/src/adaptive_relay.rs:246:        let pressure = FrontierPressureMetrics::compute(&[(100, &f1), (200, &f2)]);
crates/amun-replay-engine/src/adaptive_relay.rs:253:    fn test_adaptive_compression_under_pressure() {
crates/amun-replay-engine/src/adaptive_relay.rs:255:        let decision = adaptive_compression_policy(&pressure, RoutedProofType::FullWitness);
crates/amun-replay-engine/src/adaptive_relay.rs:260:    fn test_backpressure() {
crates/amun-replay-engine/src/adaptive_relay.rs:269:    fn test_equivalence_cache() {
crates/amun-replay-engine/src/adaptive_relay.rs:278:    fn test_anti_centrality_guard() {
crates/amun-replay-engine/src/adaptive_relay.rs:290:    fn test_no_centrality_when_balanced() {
crates/amun-replay-engine/src/adaptive_relay.rs:33:impl RelayTopologyView {
crates/amun-replay-engine/src/adaptive_relay.rs:34:    pub fn new() -> Self { Self { relays: Vec::new() } }
crates/amun-replay-engine/src/adaptive_relay.rs:36:    pub fn add_relay(&mut self, metrics: RelayMetrics) {
crates/amun-replay-engine/src/adaptive_relay.rs:42:    pub fn select_relay(&self, avoid_relays: &[u64]) -> Option<u64> {
crates/amun-replay-engine/src/adaptive_relay.rs:52:pub struct FrontierPressureMetrics {
crates/amun-replay-engine/src/adaptive_relay.rs:59:impl FrontierPressureMetrics {
crates/amun-replay-engine/src/adaptive_relay.rs:60:    pub fn new() -> Self { Self { blocked_workers: Vec::new(), frontier_density: 0 } }
crates/amun-replay-engine/src/adaptive_relay.rs:63:    pub fn compute(frontiers: &[(u64, &DerivationalFrontier)]) -> Self {
crates/amun-replay-engine/src/adaptive_relay.rs:75:    pub fn is_under_pressure(&self) -> bool {
crates/amun-replay-engine/src/adaptive_relay.rs:82:pub enum CompressionDecision {
crates/amun-replay-engine/src/adaptive_relay.rs:96:pub fn adaptive_compression_policy(
crates/amun-replay-engine/src/adaptive_relay.rs:98:    route_type: RoutedProofType,
crates/amun-replay-engine/src/byzantine_witness_filter.rs:100:            WitnessEntry::new([0x01; 32], WitnessType::HardDependency),
crates/amun-replay-engine/src/byzantine_witness_filter.rs:101:            WitnessEntry::new([0x02; 32], WitnessType::SupportingDependency),
crates/amun-replay-engine/src/byzantine_witness_filter.rs:108:    fn test_inflation_suspected() {
crates/amun-replay-engine/src/byzantine_witness_filter.rs:110:        entries.push(WitnessEntry::new([0x01; 32], WitnessType::HardDependency));
crates/amun-replay-engine/src/byzantine_witness_filter.rs:112:            entries.push(WitnessEntry::new([i as u8; 32], WitnessType::AuditDependency));
crates/amun-replay-engine/src/byzantine_witness_filter.rs:120:    fn test_self_reference_rejected() {
crates/amun-replay-engine/src/byzantine_witness_filter.rs:122:            WitnessEntry::new([0xAA; 32], WitnessType::HardDependency), // same as target
crates/amun-replay-engine/src/byzantine_witness_filter.rs:13:use amun_constitutional::ConstitutionalWitness;
crates/amun-replay-engine/src/byzantine_witness_filter.rs:14:use amun_constitutional::WitnessType;
crates/amun-replay-engine/src/byzantine_witness_filter.rs:18:pub enum FilterResult {
crates/amun-replay-engine/src/byzantine_witness_filter.rs:19:    /// Witness passes all structural checks.
crates/amun-replay-engine/src/byzantine_witness_filter.rs:1://! ByzantineWitnessFilter — prevent malicious witness propagation.
crates/amun-replay-engine/src/byzantine_witness_filter.rs:21:    /// Witness is structurally incomplete (missing hard dependencies).
crates/amun-replay-engine/src/byzantine_witness_filter.rs:25:    /// Witness appears to be part of a dependency inflation attack.
crates/amun-replay-engine/src/byzantine_witness_filter.rs:31:    /// Witness contains self-referential entries.
crates/amun-replay-engine/src/byzantine_witness_filter.rs:33:    /// Witness is empty (no entries).
crates/amun-replay-engine/src/byzantine_witness_filter.rs:34:    EmptyWitness,
crates/amun-replay-engine/src/byzantine_witness_filter.rs:42:pub fn filter_incoming_witness(witness: &ConstitutionalWitness) -> FilterResult {
crates/amun-replay-engine/src/byzantine_witness_filter.rs:45:        return FilterResult::EmptyWitness;
crates/amun-replay-engine/src/byzantine_witness_filter.rs:57:        .filter(|e| matches!(e.witness_type, WitnessType::HardDependency))
crates/amun-replay-engine/src/byzantine_witness_filter.rs:61:        // Witness has entries but no hard dependencies — suspicious
crates/amun-replay-engine/src/byzantine_witness_filter.rs:85:    use amun_constitutional::{WitnessEntry, WitnessType};
crates/amun-replay-engine/src/byzantine_witness_filter.rs:87:    fn make_witness(entries: Vec<WitnessEntry>) -> ConstitutionalWitness {
crates/amun-replay-engine/src/byzantine_witness_filter.rs:88:        ConstitutionalWitness::new(1, 1, 1, [0xAA; 32], [0xAB; 32], entries)
crates/amun-replay-engine/src/byzantine_witness_filter.rs:92:    fn test_empty_witness_rejected() {
crates/amun-replay-engine/src/byzantine_witness_filter.rs:94:        assert!(matches!(filter_incoming_witness(&w), FilterResult::EmptyWitness));
crates/amun-replay-engine/src/byzantine_witness_filter.rs:98:    fn test_valid_witness_accepted() {
crates/amun-replay-engine/src/byzantine_witness_filter.rs:9://! This filter validates structural properties of incoming witnesses
crates/amun-replay-engine/src/canonical.rs:101:    pub fn new(buffer: &'a [u8]) -> Self {
crates/amun-replay-engine/src/canonical.rs:108:    pub fn remaining(&self) -> usize {
crates/amun-replay-engine/src/canonical.rs:112:    pub fn read_u64(&mut self) -> Result<u64, CanonicalError> {
crates/amun-replay-engine/src/canonical.rs:124:    pub fn read_hash(&mut self) -> Result<ConstitutionalHash, CanonicalError> {
crates/amun-replay-engine/src/canonical.rs:129:        let hash: ConstitutionalHash = slice
crates/amun-replay-engine/src/canonical.rs:136:    pub fn read_bytes(&mut self) -> Result<&'a [u8], CanonicalError> {
crates/amun-replay-engine/src/canonical.rs:148:pub enum CanonicalError {
crates/amun-replay-engine/src/canonical.rs:157:pub struct CanonicalHasher {
crates/amun-replay-engine/src/canonical.rs:161:impl Default for CanonicalHasher {
crates/amun-replay-engine/src/canonical.rs:162:    fn default() -> Self {
crates/amun-replay-engine/src/canonical.rs:167:impl CanonicalHasher {
crates/amun-replay-engine/src/canonical.rs:168:    pub fn new() -> Self {
crates/amun-replay-engine/src/canonical.rs:172:    pub fn update<T: CanonicalEncode>(&mut self, item: &T) {
crates/amun-replay-engine/src/canonical.rs:178:    pub fn finalize(&self) -> ConstitutionalHash {
crates/amun-replay-engine/src/canonical.rs:182:        let result: [u8; 32] = hasher.finalize().into();
crates/amun-replay-engine/src/canonical.rs:191:// NOTE: ConstitutionalHash IS [u8; 32], so we implement ONLY for [u8; 32].
crates/amun-replay-engine/src/canonical.rs:194:impl CanonicalEncode for u64 {
crates/amun-replay-engine/src/canonical.rs:195:    fn canonical_encode(&self) -> Vec<u8> {
crates/amun-replay-engine/src/canonical.rs:198:    fn canonical_encode_into(&self, buf: &mut CanonicalWriter) {
crates/amun-replay-engine/src/canonical.rs:203:impl CanonicalEncode for u32 {
crates/amun-replay-engine/src/canonical.rs:204:    fn canonical_encode(&self) -> Vec<u8> {
crates/amun-replay-engine/src/canonical.rs:207:    fn canonical_encode_into(&self, buf: &mut CanonicalWriter) {
crates/amun-replay-engine/src/canonical.rs:212:impl CanonicalEncode for u8 {
crates/amun-replay-engine/src/canonical.rs:213:    fn canonical_encode(&self) -> Vec<u8> {
crates/amun-replay-engine/src/canonical.rs:216:    fn canonical_encode_into(&self, buf: &mut CanonicalWriter) {
crates/amun-replay-engine/src/canonical.rs:221:impl CanonicalEncode for bool {
crates/amun-replay-engine/src/canonical.rs:222:    fn canonical_encode(&self) -> Vec<u8> {
crates/amun-replay-engine/src/canonical.rs:225:    fn canonical_encode_into(&self, buf: &mut CanonicalWriter) {
crates/amun-replay-engine/src/canonical.rs:230:// SINGLE implementation for [u8; 32] (which IS ConstitutionalHash)
crates/amun-replay-engine/src/canonical.rs:231:impl CanonicalEncode for [u8; 32] {
crates/amun-replay-engine/src/canonical.rs:232:    fn canonical_encode(&self) -> Vec<u8> {
crates/amun-replay-engine/src/canonical.rs:235:    fn canonical_encode_into(&self, buf: &mut CanonicalWriter) {
crates/amun-replay-engine/src/canonical.rs:23:use amun_constitutional::ConstitutionalHash;
crates/amun-replay-engine/src/canonical.rs:249:    fn u64_roundtrip() {
crates/amun-replay-engine/src/canonical.rs:257:    fn hash_roundtrip() {
crates/amun-replay-engine/src/canonical.rs:258:        let hash: ConstitutionalHash = [0xAB; 32];
crates/amun-replay-engine/src/canonical.rs:265:    fn hasher_is_deterministic() {
crates/amun-replay-engine/src/canonical.rs:269:        let r1 = h1.finalize();
crates/amun-replay-engine/src/canonical.rs:274:        let r2 = h2.finalize();
crates/amun-replay-engine/src/canonical.rs:280:    fn hasher_is_order_sensitive() {
crates/amun-replay-engine/src/canonical.rs:29:pub trait CanonicalEncode {
crates/amun-replay-engine/src/canonical.rs:30:    fn canonical_encode(&self) -> Vec<u8>;
crates/amun-replay-engine/src/canonical.rs:31:    fn canonical_encode_into(&self, buf: &mut CanonicalWriter);
crates/amun-replay-engine/src/canonical.rs:38:pub struct CanonicalWriter {
crates/amun-replay-engine/src/canonical.rs:42:impl Default for CanonicalWriter {
crates/amun-replay-engine/src/canonical.rs:43:    fn default() -> Self {
crates/amun-replay-engine/src/canonical.rs:48:impl CanonicalWriter {
crates/amun-replay-engine/src/canonical.rs:49:    pub fn new() -> Self {
crates/amun-replay-engine/src/canonical.rs:53:    pub fn write_u64(&mut self, v: u64) {
crates/amun-replay-engine/src/canonical.rs:57:    pub fn write_u32(&mut self, v: u32) {
crates/amun-replay-engine/src/canonical.rs:5:// This file is the constitutional foundation of the entire replay system.
crates/amun-replay-engine/src/canonical.rs:61:    pub fn write_u8(&mut self, v: u8) {
crates/amun-replay-engine/src/canonical.rs:65:    pub fn write_bool(&mut self, v: bool) {
crates/amun-replay-engine/src/canonical.rs:69:    pub fn write_bytes(&mut self, bytes: &[u8]) {
crates/amun-replay-engine/src/canonical.rs:6:// Without a stable, deterministic byte representation, replay equivalence
crates/amun-replay-engine/src/canonical.rs:74:    pub fn write_fixed_bytes(&mut self, bytes: &[u8]) {
crates/amun-replay-engine/src/canonical.rs:78:    pub fn write_hash(&mut self, hash: &ConstitutionalHash) {
crates/amun-replay-engine/src/canonical.rs:82:    pub fn into_bytes(self) -> Vec<u8> {
crates/amun-replay-engine/src/canonical.rs:86:    pub fn as_bytes(&self) -> &[u8] {
crates/amun-replay-engine/src/canonical.rs:95:pub struct CanonicalReader<'a> {
crates/amun-replay-engine/src/closure_completeness.rs:11:impl ClosureCompleteness {
crates/amun-replay-engine/src/closure_completeness.rs:12:    pub fn can_derive(&self) -> bool { matches!(self, ClosureCompleteness::Sufficient | ClosureCompleteness::Exhaustive) }
crates/amun-replay-engine/src/closure_completeness.rs:13:    pub fn should_continue_seeking(&self) -> bool { matches!(self, ClosureCompleteness::Partial) }
crates/amun-replay-engine/src/closure_completeness.rs:14:    pub fn propagation_complete(&self) -> bool { matches!(self, ClosureCompleteness::Sufficient | ClosureCompleteness::Exhaustive) }
crates/amun-replay-engine/src/closure_completeness.rs:15:    pub fn level(&self) -> u8 {
crates/amun-replay-engine/src/closure_completeness.rs:4:pub enum ClosureCompleteness {
crates/amun-replay-engine/src/constitutional_economics.rs:103:    pub context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/constitutional_economics.rs:114:impl WitnessIncentiveSurface {
crates/amun-replay-engine/src/constitutional_economics.rs:115:    pub fn new(program_id: u64, context_hash: ConstitutionalHash) -> Self {
crates/amun-replay-engine/src/constitutional_economics.rs:11:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/constitutional_economics.rs:120:    /// Proofs are not "better" because they cost more.
crates/amun-replay-engine/src/constitutional_economics.rs:121:    pub fn verify_no_semantic_reward(&self) -> bool {
crates/amun-replay-engine/src/constitutional_economics.rs:133:pub struct EconomicContainmentZone {
crates/amun-replay-engine/src/constitutional_economics.rs:142:impl EconomicContainmentZone {
crates/amun-replay-engine/src/constitutional_economics.rs:143:    pub fn new(max_share: u8) -> Self {
crates/amun-replay-engine/src/constitutional_economics.rs:148:    pub fn check_market_share(&mut self, participant_id: u64, share_percent: u8) {
crates/amun-replay-engine/src/constitutional_economics.rs:159:    pub fn is_capture_suspected(&self) -> bool { self.capture_suspected }
crates/amun-replay-engine/src/constitutional_economics.rs:169:pub struct ScarcityNeutralityBoundary {
crates/amun-replay-engine/src/constitutional_economics.rs:177:impl ScarcityNeutralityBoundary {
crates/amun-replay-engine/src/constitutional_economics.rs:178:    pub fn new(max_bound: u64) -> Self {
crates/amun-replay-engine/src/constitutional_economics.rs:185:    pub fn check_neutrality(&mut self, resource_expenditure: u64) -> bool {
crates/amun-replay-engine/src/constitutional_economics.rs:19:pub struct ProofMarketSurface {
crates/amun-replay-engine/src/constitutional_economics.rs:1://! Constitutional Economics — markets that sustain, not govern.
crates/amun-replay-engine/src/constitutional_economics.rs:200:    fn test_market_is_economic_not_constitutional() {
crates/amun-replay-engine/src/constitutional_economics.rs:201:        let market = ProofMarketSurface::new(1, [0xAB; 32]);
crates/amun-replay-engine/src/constitutional_economics.rs:207:    fn test_lease_never_grants_authority() {
crates/amun-replay-engine/src/constitutional_economics.rs:214:    fn test_lease_capacity_exhaustion() {
crates/amun-replay-engine/src/constitutional_economics.rs:216:        assert!(lease.execute());
crates/amun-replay-engine/src/constitutional_economics.rs:217:        assert!(lease.execute());
crates/amun-replay-engine/src/constitutional_economics.rs:218:        assert!(!lease.execute()); // exhausted
crates/amun-replay-engine/src/constitutional_economics.rs:222:    fn test_semantic_quality_reward_is_zero() {
crates/amun-replay-engine/src/constitutional_economics.rs:223:        let incentives = WitnessIncentiveSurface::new(1, [0xAB; 32]);
crates/amun-replay-engine/src/constitutional_economics.rs:224:        assert!(incentives.verify_no_semantic_reward());
crates/amun-replay-engine/src/constitutional_economics.rs:229:    fn test_economic_containment() {
crates/amun-replay-engine/src/constitutional_economics.rs:237:    fn test_scarcity_neutrality() {
crates/amun-replay-engine/src/constitutional_economics.rs:23:    pub context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/constitutional_economics.rs:245:    fn test_economics_does_not_affect_truth() {
crates/amun-replay-engine/src/constitutional_economics.rs:31:impl ProofMarketSurface {
crates/amun-replay-engine/src/constitutional_economics.rs:32:    pub fn new(market_id: u64, context_hash: ConstitutionalHash) -> Self {
crates/amun-replay-engine/src/constitutional_economics.rs:38:    pub fn is_affordable(&self, price: u64) -> bool {
crates/amun-replay-engine/src/constitutional_economics.rs:43:/// An execution lease boundary — lease compute, NOT constitutional authority.
crates/amun-replay-engine/src/constitutional_economics.rs:48:pub struct ExecutionLeaseBoundary {
crates/amun-replay-engine/src/constitutional_economics.rs:54:    pub context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/constitutional_economics.rs:63:impl ExecutionLeaseBoundary {
crates/amun-replay-engine/src/constitutional_economics.rs:64:    pub fn new(lease_id: u64, worker_id: u64, context_hash: ConstitutionalHash, max_executions: u64) -> Self {
crates/amun-replay-engine/src/constitutional_economics.rs:69:    pub fn has_capacity(&self) -> bool {
crates/amun-replay-engine/src/constitutional_economics.rs:74:    pub fn execute(&mut self) -> bool {
crates/amun-replay-engine/src/constitutional_economics.rs:81:    pub fn check_constitutional_authority(&self) -> bool {
crates/amun-replay-engine/src/constitutional_economics.rs:99:pub struct WitnessIncentiveSurface {
crates/amun-replay-engine/src/constitutional_governance.rs:111:pub struct ConstitutionalEvolutionBoundary {
crates/amun-replay-engine/src/constitutional_governance.rs:11:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/constitutional_governance.rs:123:impl ConstitutionalEvolutionBoundary {
crates/amun-replay-engine/src/constitutional_governance.rs:124:    pub fn new() -> Self {
crates/amun-replay-engine/src/constitutional_governance.rs:125:        // Invariants 1-25 are all constitutional law.
crates/amun-replay-engine/src/constitutional_governance.rs:12:use amun_constitutional::ConstitutionalHasher;
crates/amun-replay-engine/src/constitutional_governance.rs:136:    pub fn can_modify_invariant(&self, invariant_index: u8) -> bool {
crates/amun-replay-engine/src/constitutional_governance.rs:141:    pub fn can_recurse(&self) -> bool {
crates/amun-replay-engine/src/constitutional_governance.rs:146:    pub fn recurse(&mut self) -> bool {
crates/amun-replay-engine/src/constitutional_governance.rs:162:pub struct GovernanceContainmentZone {
crates/amun-replay-engine/src/constitutional_governance.rs:171:impl GovernanceContainmentZone {
crates/amun-replay-engine/src/constitutional_governance.rs:172:    pub fn new(max_proposals: u64) -> Self {
crates/amun-replay-engine/src/constitutional_governance.rs:177:    pub fn can_accept_proposal(&self) -> bool {
crates/amun-replay-engine/src/constitutional_governance.rs:183:    pub fn contain_proposal(&mut self, proposal_id: u64) {
crates/amun-replay-engine/src/constitutional_governance.rs:190:    pub fn record_proposal(&mut self) {
crates/amun-replay-engine/src/constitutional_governance.rs:1://! Constitutional Governance — constrained constitutional evolution.
crates/amun-replay-engine/src/constitutional_governance.rs:201:pub struct TemporalConstitutionLineage {
crates/amun-replay-engine/src/constitutional_governance.rs:205:    pub active_constitution_hash: ConstitutionalHash,
crates/amun-replay-engine/src/constitutional_governance.rs:207:    pub revision_history: Vec<(u32, ConstitutionalHash)>,
crates/amun-replay-engine/src/constitutional_governance.rs:20:pub struct RatificationSurface {
crates/amun-replay-engine/src/constitutional_governance.rs:212:impl TemporalConstitutionLineage {
crates/amun-replay-engine/src/constitutional_governance.rs:213:    pub fn new(initial_revision: u32, constitution_hash: ConstitutionalHash) -> Self {
crates/amun-replay-engine/src/constitutional_governance.rs:224:    pub fn activate_revision(&mut self, new_revision: u32, new_hash: ConstitutionalHash) {
crates/amun-replay-engine/src/constitutional_governance.rs:231:    pub fn is_active(&self, revision: u32) -> bool {
crates/amun-replay-engine/src/constitutional_governance.rs:236:    pub fn is_in_lineage(&self, revision: u32) -> bool {
crates/amun-replay-engine/src/constitutional_governance.rs:248:    fn test_ratification_surface() {
crates/amun-replay-engine/src/constitutional_governance.rs:256:    fn test_amendment_admissible() {
crates/amun-replay-engine/src/constitutional_governance.rs:262:    fn test_self_referential_amendment_flagged() {
crates/amun-replay-engine/src/constitutional_governance.rs:268:    fn test_revision_regression_rejected() {
crates/amun-replay-engine/src/constitutional_governance.rs:274:    fn test_immutable_invariants() {
crates/amun-replay-engine/src/constitutional_governance.rs:275:        let boundary = ConstitutionalEvolutionBoundary::new();
crates/amun-replay-engine/src/constitutional_governance.rs:282:    fn test_amendment_depth_boundary() {
crates/amun-replay-engine/src/constitutional_governance.rs:283:        let mut boundary = ConstitutionalEvolutionBoundary::new();
crates/amun-replay-engine/src/constitutional_governance.rs:28:    pub amendment_hash: ConstitutionalHash,
crates/amun-replay-engine/src/constitutional_governance.rs:290:    fn test_governance_containment() {
crates/amun-replay-engine/src/constitutional_governance.rs:300:    fn test_temporal_lineage() {
crates/amun-replay-engine/src/constitutional_governance.rs:301:        let mut lineage = TemporalConstitutionLineage::new(1, [0xAA; 32]);
crates/amun-replay-engine/src/constitutional_governance.rs:30:    pub context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/constitutional_governance.rs:312:    fn test_governance_does_not_manufacture_truth() {
crates/amun-replay-engine/src/constitutional_governance.rs:34:    pub surface_hash: ConstitutionalHash,
crates/amun-replay-engine/src/constitutional_governance.rs:37:impl RatificationSurface {
crates/amun-replay-engine/src/constitutional_governance.rs:38:    pub fn new(
crates/amun-replay-engine/src/constitutional_governance.rs:40:        amendment_hash: ConstitutionalHash, context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/constitutional_governance.rs:47:        s.surface_hash = s.compute_hash();
crates/amun-replay-engine/src/constitutional_governance.rs:51:    fn compute_hash(&self) -> ConstitutionalHash {
crates/amun-replay-engine/src/constitutional_governance.rs:52:        let mut h = ConstitutionalHasher::new(b"RATIFICATION_SURFACE");
crates/amun-replay-engine/src/constitutional_governance.rs:58:        h.finalize()
crates/amun-replay-engine/src/constitutional_governance.rs:64:pub enum AmendmentDerivabilityResult {
crates/amun-replay-engine/src/constitutional_governance.rs:68:    InvariantViolation {
crates/amun-replay-engine/src/constitutional_governance.rs:82:pub fn check_amendment_derivability(
crates/amun-replay-engine/src/constitutional_governance.rs:85:    _amendment_hash: ConstitutionalHash,
crates/amun-replay-engine/src/constitutional_governance.rs:8://! Constitutional evolution must remain derivationally constrained.
crates/amun-replay-engine/src/constitutional_identity.rs:105:impl ReputationNeutralityGuard {
crates/amun-replay-engine/src/constitutional_identity.rs:106:    pub fn new(max_reputation: u64) -> Self {
crates/amun-replay-engine/src/constitutional_identity.rs:113:    pub fn check_reputation(&mut self, identity_id: u64, reputation_score: u64) -> bool {
crates/amun-replay-engine/src/constitutional_identity.rs:11:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/constitutional_identity.rs:12:use amun_constitutional::ConstitutionalHasher;
crates/amun-replay-engine/src/constitutional_identity.rs:132:pub struct IdentityContainmentZone {
crates/amun-replay-engine/src/constitutional_identity.rs:142:impl IdentityContainmentZone {
crates/amun-replay-engine/src/constitutional_identity.rs:143:    pub fn new(max_share: u8) -> Self {
crates/amun-replay-engine/src/constitutional_identity.rs:148:    pub fn check_production_share(&mut self, identity_id: u64, share_percent: u8) {
crates/amun-replay-engine/src/constitutional_identity.rs:159:    pub fn is_capture_suspected(&self) -> bool { self.capture_suspected }
crates/amun-replay-engine/src/constitutional_identity.rs:167:    fn test_identity_never_has_authority() {
crates/amun-replay-engine/src/constitutional_identity.rs:173:    fn test_identity_fingerprint_deterministic() {
crates/amun-replay-engine/src/constitutional_identity.rs:180:    fn test_attribution_never_has_semantic_weight() {
crates/amun-replay-engine/src/constitutional_identity.rs:186:    fn test_reputation_neutrality() {
crates/amun-replay-engine/src/constitutional_identity.rs:194:    fn test_identity_containment() {
crates/amun-replay-engine/src/constitutional_identity.rs:1://! Constitutional Identity & Attribution — identity without privilege.
crates/amun-replay-engine/src/constitutional_identity.rs:202:    fn test_identity_is_not_authority() {
crates/amun-replay-engine/src/constitutional_identity.rs:20:pub struct IdentitySurface {
crates/amun-replay-engine/src/constitutional_identity.rs:24:    pub public_key_hash: ConstitutionalHash,
crates/amun-replay-engine/src/constitutional_identity.rs:36:impl IdentitySurface {
crates/amun-replay-engine/src/constitutional_identity.rs:37:    pub fn new(identity_id: u64, public_key_hash: ConstitutionalHash) -> Self {
crates/amun-replay-engine/src/constitutional_identity.rs:42:    pub fn record_artifact(&mut self) {
crates/amun-replay-engine/src/constitutional_identity.rs:49:    pub fn check_constitutional_authority(&self) -> bool {
crates/amun-replay-engine/src/constitutional_identity.rs:54:    pub fn fingerprint(&self) -> ConstitutionalHash {
crates/amun-replay-engine/src/constitutional_identity.rs:55:        let mut h = ConstitutionalHasher::new(b"IDENTITY_SURFACE");
crates/amun-replay-engine/src/constitutional_identity.rs:57:        h.finalize()
crates/amun-replay-engine/src/constitutional_identity.rs:66:pub struct AttributionBoundary {
crates/amun-replay-engine/src/constitutional_identity.rs:68:    pub artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/constitutional_identity.rs:77:impl AttributionBoundary {
crates/amun-replay-engine/src/constitutional_identity.rs:78:    pub fn new(artifact_hash: ConstitutionalHash, producer_id: u64) -> Self {
crates/amun-replay-engine/src/constitutional_identity.rs:84:    pub fn has_semantic_weight(&self) -> bool {
crates/amun-replay-engine/src/constitutional_identity.rs:95:pub struct ReputationNeutralityGuard {
crates/amun-replay-engine/src/containment_boundary.rs:105:    fn test_clean_worker_is_operational() {
crates/amun-replay-engine/src/containment_boundary.rs:113:    fn test_hostile_worker_is_quarantined() {
crates/amun-replay-engine/src/containment_boundary.rs:11:pub enum ContainmentStatus {
crates/amun-replay-engine/src/containment_boundary.rs:124:    fn test_containment_is_not_invalidation() {
crates/amun-replay-engine/src/containment_boundary.rs:130:        // But this does NOT invalidate its artifacts
crates/amun-replay-engine/src/containment_boundary.rs:136:    fn test_containment_zone() {
crates/amun-replay-engine/src/containment_boundary.rs:30:pub struct SemanticContaminationBoundary {
crates/amun-replay-engine/src/containment_boundary.rs:42:impl SemanticContaminationBoundary {
crates/amun-replay-engine/src/containment_boundary.rs:43:    pub fn new(worker_id: u64, anomaly_surface: RuntimeAnomalySurface) -> Self {
crates/amun-replay-engine/src/containment_boundary.rs:59:    pub fn is_contained(&self) -> bool {
crates/amun-replay-engine/src/containment_boundary.rs:63:    /// CRITICAL: This method does NOT invalidate artifacts.
crates/amun-replay-engine/src/containment_boundary.rs:66:    pub fn is_operationally_restricted(&self) -> bool {
crates/amun-replay-engine/src/containment_boundary.rs:73:pub struct ByzantineContainmentZone {
crates/amun-replay-engine/src/containment_boundary.rs:77:impl ByzantineContainmentZone {
crates/amun-replay-engine/src/containment_boundary.rs:78:    pub fn new() -> Self { Self { boundaries: Vec::new() } }
crates/amun-replay-engine/src/containment_boundary.rs:80:    pub fn add_boundary(&mut self, boundary: SemanticContaminationBoundary) {
crates/amun-replay-engine/src/containment_boundary.rs:84:    pub fn get_status(&self, worker_id: u64) -> ContainmentStatus {
crates/amun-replay-engine/src/containment_boundary.rs:91:    pub fn contained_workers(&self) -> Vec<u64> {
crates/amun-replay-engine/src/cross_constitution_federation.rs:104:    pub fn verify_sovereignty(&self) -> bool {
crates/amun-replay-engine/src/cross_constitution_federation.rs:109:    pub fn fingerprint(&self) -> ConstitutionalHash {
crates/amun-replay-engine/src/cross_constitution_federation.rs:10:use amun_constitutional::ConstitutionalHasher;
crates/amun-replay-engine/src/cross_constitution_federation.rs:110:        let mut h = ConstitutionalHasher::new(b"TRANSLATION_SURFACE");
crates/amun-replay-engine/src/cross_constitution_federation.rs:116:        h.finalize()
crates/amun-replay-engine/src/cross_constitution_federation.rs:126:pub struct SovereigntyPreservingBridge {
crates/amun-replay-engine/src/cross_constitution_federation.rs:139:impl SovereigntyPreservingBridge {
crates/amun-replay-engine/src/cross_constitution_federation.rs:140:    pub fn new(bridge_id: u64, boundary: FederationBoundary) -> Self {
crates/amun-replay-engine/src/cross_constitution_federation.rs:149:    pub fn check_equivalence_assumed(&self) -> bool {
crates/amun-replay-engine/src/cross_constitution_federation.rs:154:    pub fn record_translation(&mut self) {
crates/amun-replay-engine/src/cross_constitution_federation.rs:161:pub enum FederationStatus {
crates/amun-replay-engine/src/cross_constitution_federation.rs:175:    fn test_boundary_never_allows_override() {
crates/amun-replay-engine/src/cross_constitution_federation.rs:182:    fn test_translation_preserves_sovereignty() {
crates/amun-replay-engine/src/cross_constitution_federation.rs:183:        let translation = ConstitutionalTranslationSurface::new(
crates/amun-replay-engine/src/cross_constitution_federation.rs:186:        assert!(translation.verify_sovereignty());
crates/amun-replay-engine/src/cross_constitution_federation.rs:191:    fn test_bridge_never_assumes_equivalence() {
crates/amun-replay-engine/src/cross_constitution_federation.rs:199:    fn test_federation_is_not_unification() {
crates/amun-replay-engine/src/cross_constitution_federation.rs:19:pub struct FederationBoundary {
crates/amun-replay-engine/src/cross_constitution_federation.rs:1://! Cross-Constitution Federation — interoperability without imperialism.
crates/amun-replay-engine/src/cross_constitution_federation.rs:207:    fn test_translation_fingerprint_deterministic() {
crates/amun-replay-engine/src/cross_constitution_federation.rs:208:        let t1 = ConstitutionalTranslationSurface::new(1, [0xAA;32], [0xBB;32], [0xCC;32], [0xDD;32]);
crates/amun-replay-engine/src/cross_constitution_federation.rs:209:        let t2 = ConstitutionalTranslationSurface::new(1, [0xAA;32], [0xBB;32], [0xCC;32], [0xDD;32]);
crates/amun-replay-engine/src/cross_constitution_federation.rs:214:    fn test_bridge_records_translations() {
crates/amun-replay-engine/src/cross_constitution_federation.rs:23:    pub local_constitution_hash: ConstitutionalHash,
crates/amun-replay-engine/src/cross_constitution_federation.rs:25:    pub remote_constitution_hash: ConstitutionalHash,
crates/amun-replay-engine/src/cross_constitution_federation.rs:36:impl FederationBoundary {
crates/amun-replay-engine/src/cross_constitution_federation.rs:37:    pub fn new(
crates/amun-replay-engine/src/cross_constitution_federation.rs:38:        boundary_id: u64, local_hash: ConstitutionalHash, remote_hash: ConstitutionalHash,
crates/amun-replay-engine/src/cross_constitution_federation.rs:50:    pub fn check_remote_override_allowed(&self) -> bool {
crates/amun-replay-engine/src/cross_constitution_federation.rs:56:    pub fn check_remote_mutation_allowed(&self) -> bool {
crates/amun-replay-engine/src/cross_constitution_federation.rs:69:pub struct ConstitutionalTranslationSurface {
crates/amun-replay-engine/src/cross_constitution_federation.rs:73:    pub source_constitution_hash: ConstitutionalHash,
crates/amun-replay-engine/src/cross_constitution_federation.rs:75:    pub target_constitution_hash: ConstitutionalHash,
crates/amun-replay-engine/src/cross_constitution_federation.rs:77:    pub source_derivation_hash: ConstitutionalHash,
crates/amun-replay-engine/src/cross_constitution_federation.rs:79:    pub interpreted_derivation_hash: ConstitutionalHash,
crates/amun-replay-engine/src/cross_constitution_federation.rs:86:impl ConstitutionalTranslationSurface {
crates/amun-replay-engine/src/cross_constitution_federation.rs:87:    pub fn new(
crates/amun-replay-engine/src/cross_constitution_federation.rs:88:        translation_id: u64, source_hash: ConstitutionalHash,
crates/amun-replay-engine/src/cross_constitution_federation.rs:89:        target_hash: ConstitutionalHash, source_derivation: ConstitutionalHash,
crates/amun-replay-engine/src/cross_constitution_federation.rs:90:        interpreted_derivation: ConstitutionalHash,
crates/amun-replay-engine/src/cross_constitution_federation.rs:9:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/derivational_equivalence.rs:103:        self.class_hash = self.compute_class_hash();
crates/amun-replay-engine/src/derivational_equivalence.rs:107:    pub fn is_equivalent_to(&self, other: &DerivationalEquivalenceClass) -> bool {
crates/amun-replay-engine/src/derivational_equivalence.rs:116:pub fn canonical_order_for_transport(hashes: &mut [ConstitutionalHash]) {
crates/amun-replay-engine/src/derivational_equivalence.rs:11:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/derivational_equivalence.rs:122:pub fn reduce_to_minimal_core(
crates/amun-replay-engine/src/derivational_equivalence.rs:123:    all_hashes: &[ConstitutionalHash],
crates/amun-replay-engine/src/derivational_equivalence.rs:124:    hard_dependency_hashes: &[ConstitutionalHash],
crates/amun-replay-engine/src/derivational_equivalence.rs:125:) -> Vec<ConstitutionalHash> {
crates/amun-replay-engine/src/derivational_equivalence.rs:126:    let mut core: Vec<ConstitutionalHash> = Vec::new();
crates/amun-replay-engine/src/derivational_equivalence.rs:12:use amun_constitutional::ConstitutionalHasher;
crates/amun-replay-engine/src/derivational_equivalence.rs:141:    fn test_fingerprint_deterministic() {
crates/amun-replay-engine/src/derivational_equivalence.rs:142:        let fp1 = CanonicalDerivationFingerprint::compute([0xAA; 32], true, [0xAB; 32]);
crates/amun-replay-engine/src/derivational_equivalence.rs:143:        let fp2 = CanonicalDerivationFingerprint::compute([0xAA; 32], true, [0xAB; 32]);
crates/amun-replay-engine/src/derivational_equivalence.rs:148:    fn test_different_outcome_different_fingerprint() {
crates/amun-replay-engine/src/derivational_equivalence.rs:149:        let fp1 = CanonicalDerivationFingerprint::compute([0xAA; 32], true, [0xAB; 32]);
crates/amun-replay-engine/src/derivational_equivalence.rs:150:        let fp2 = CanonicalDerivationFingerprint::compute([0xAA; 32], false, [0xAB; 32]);
crates/amun-replay-engine/src/derivational_equivalence.rs:155:    fn test_equivalence_class_creation() {
crates/amun-replay-engine/src/derivational_equivalence.rs:166:    fn test_add_surface_keeps_smaller_core() {
crates/amun-replay-engine/src/derivational_equivalence.rs:177:    fn test_reduce_to_minimal_core() {
crates/amun-replay-engine/src/derivational_equivalence.rs:189:    fn test_equivalence_detection() {
crates/amun-replay-engine/src/derivational_equivalence.rs:196:    fn test_non_equivalence() {
crates/amun-replay-engine/src/derivational_equivalence.rs:20:pub struct CanonicalDerivationFingerprint(pub [u8; 32]);
crates/amun-replay-engine/src/derivational_equivalence.rs:22:impl CanonicalDerivationFingerprint {
crates/amun-replay-engine/src/derivational_equivalence.rs:24:    pub fn compute(target_artifact: ConstitutionalHash, can_derive: bool, context_hash: ConstitutionalHash) -> Self {
crates/amun-replay-engine/src/derivational_equivalence.rs:25:        let mut h = ConstitutionalHasher::new(b"DERIVATION_FINGERPRINT");
crates/amun-replay-engine/src/derivational_equivalence.rs:29:        Self(h.finalize())
crates/amun-replay-engine/src/derivational_equivalence.rs:36:pub struct DerivationalEquivalenceClass {
crates/amun-replay-engine/src/derivational_equivalence.rs:41:    pub target_artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/derivational_equivalence.rs:44:    pub context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/derivational_equivalence.rs:54:    pub minimal_core_hashes: Vec<ConstitutionalHash>,
crates/amun-replay-engine/src/derivational_equivalence.rs:60:impl DerivationalEquivalenceClass {
crates/amun-replay-engine/src/derivational_equivalence.rs:61:    pub fn new(
crates/amun-replay-engine/src/derivational_equivalence.rs:62:        target_artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/derivational_equivalence.rs:63:        context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/derivational_equivalence.rs:65:        minimal_core_hashes: Vec<ConstitutionalHash>,
crates/amun-replay-engine/src/derivational_equivalence.rs:67:        let fingerprint = CanonicalDerivationFingerprint::compute(
crates/amun-replay-engine/src/derivational_equivalence.rs:79:        c.class_hash = c.compute_class_hash();
crates/amun-replay-engine/src/derivational_equivalence.rs:83:    fn compute_class_hash(&self) -> [u8; 32] {
crates/amun-replay-engine/src/derivational_equivalence.rs:84:        let mut h = ConstitutionalHasher::new(b"EQUIVALENCE_CLASS");
crates/amun-replay-engine/src/derivational_equivalence.rs:93:        h.finalize()
crates/amun-replay-engine/src/derivational_equivalence.rs:97:    pub fn add_surface(&mut self, core_hashes: Vec<ConstitutionalHash>) {
crates/amun-replay-engine/src/derivational_frontier.rs:15:pub enum FrontierDependencyReason {
crates/amun-replay-engine/src/derivational_frontier.rs:16:    HardWitnessRequired,
crates/amun-replay-engine/src/derivational_frontier.rs:24:pub struct DerivationalFrontier {
crates/amun-replay-engine/src/derivational_frontier.rs:26:    pub target_artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/derivational_frontier.rs:28:    pub resolved: Vec<ConstitutionalHash>,
crates/amun-replay-engine/src/derivational_frontier.rs:32:impl DerivationalFrontier {
crates/amun-replay-engine/src/derivational_frontier.rs:33:    pub fn new(worker_id: u64, target_artifact_hash: ConstitutionalHash) -> Self {
crates/amun-replay-engine/src/derivational_frontier.rs:36:    pub fn with_unresolved(mut self, hash: ConstitutionalHash, reason: FrontierDependencyReason, blocking: bool) -> Self {
crates/amun-replay-engine/src/derivational_frontier.rs:40:    pub fn resolve(&mut self, hash: ConstitutionalHash) {
crates/amun-replay-engine/src/derivational_frontier.rs:43:        self.recompute_completeness();
crates/amun-replay-engine/src/derivational_frontier.rs:45:    pub fn recompute_completeness(&mut self) {
crates/amun-replay-engine/src/derivational_frontier.rs:4:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/derivational_frontier.rs:51:    pub fn is_clear(&self) -> bool { self.unresolved.is_empty() }
crates/amun-replay-engine/src/derivational_frontier.rs:52:    pub fn blocking_dependencies(&self) -> Vec<&FrontierDependency> { self.unresolved.iter().filter(|d| d.is_blocking).collect() }
crates/amun-replay-engine/src/derivational_frontier.rs:8:pub struct FrontierDependency {
crates/amun-replay-engine/src/derivational_frontier.rs:9:    pub artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/deterministic.rs:104:pub struct ExecutionResult {
crates/amun-replay-engine/src/deterministic.rs:108:    pub steps: Vec<ConstitutionalStep>,
crates/amun-replay-engine/src/deterministic.rs:109:    pub final_state: ConstitutionalHash,
crates/amun-replay-engine/src/deterministic.rs:112:impl ExecutionResult {
crates/amun-replay-engine/src/deterministic.rs:113:    pub fn verify_integrity(&self) -> bool {
crates/amun-replay-engine/src/deterministic.rs:11:    pub state_hash: ConstitutionalHash,
crates/amun-replay-engine/src/deterministic.rs:12:    pub step_hash: ConstitutionalHash,
crates/amun-replay-engine/src/deterministic.rs:16:pub struct ExecutionTrace {
crates/amun-replay-engine/src/deterministic.rs:17:    pub steps: Vec<ConstitutionalStep>,
crates/amun-replay-engine/src/deterministic.rs:21:impl Default for ExecutionTrace {
crates/amun-replay-engine/src/deterministic.rs:22:    fn default() -> Self {
crates/amun-replay-engine/src/deterministic.rs:27:impl ExecutionTrace {
crates/amun-replay-engine/src/deterministic.rs:28:    pub fn new() -> Self {
crates/amun-replay-engine/src/deterministic.rs:34:    pub fn add_step(&mut self, step: ConstitutionalStep) {
crates/amun-replay-engine/src/deterministic.rs:38:    pub fn final_state_hash(&self) -> ConstitutionalHash {
crates/amun-replay-engine/src/deterministic.rs:3:use crate::errors::ReplayFailure;
crates/amun-replay-engine/src/deterministic.rs:41:    pub fn is_fully_constitutional(&self) -> bool {
crates/amun-replay-engine/src/deterministic.rs:44:    pub fn is_transformative(&self) -> bool {
crates/amun-replay-engine/src/deterministic.rs:49:pub struct DeterministicExecutor;
crates/amun-replay-engine/src/deterministic.rs:4:use amun_constitutional::{ConstitutionalHash, TranscriptEntry};
crates/amun-replay-engine/src/deterministic.rs:51:impl DeterministicExecutor {
crates/amun-replay-engine/src/deterministic.rs:52:    pub fn execute_step(
crates/amun-replay-engine/src/deterministic.rs:54:        current_hash: ConstitutionalHash,
crates/amun-replay-engine/src/deterministic.rs:56:    ) -> Result<ConstitutionalStep, ReplayFailure> {
crates/amun-replay-engine/src/deterministic.rs:58:            return Err(ReplayFailure::OrderingViolation {
crates/amun-replay-engine/src/deterministic.rs:68:        let state_hash: [u8; 32] = hasher.finalize().into();
crates/amun-replay-engine/src/deterministic.rs:69:        Ok(ConstitutionalStep {
crates/amun-replay-engine/src/deterministic.rs:77:    pub fn execute_with_trace(
crates/amun-replay-engine/src/deterministic.rs:79:        initial_hash: ConstitutionalHash,
crates/amun-replay-engine/src/deterministic.rs:81:    ) -> Result<ExecutionTrace, ReplayFailure> {
crates/amun-replay-engine/src/deterministic.rs:86:            let step = Self::execute_step(entry, current_hash, expected)?;
crates/amun-replay-engine/src/deterministic.rs:8:pub struct ConstitutionalStep {
crates/amun-replay-engine/src/deterministic.rs:94:    pub fn compute_transcript_hash(entries: &[TranscriptEntry]) -> ConstitutionalHash {
crates/amun-replay-engine/src/deterministic.rs:99:        hasher.finalize().into()
crates/amun-replay-engine/src/equivalence.rs:105:    use amun_constitutional::ReplayDomain;
crates/amun-replay-engine/src/equivalence.rs:107:    fn mk_entry(seq: u64, hash: [u8; 32]) -> TranscriptEntry {
crates/amun-replay-engine/src/equivalence.rs:10:    pub expected_result_hash: ConstitutionalHash,
crates/amun-replay-engine/src/equivalence.rs:111:            domain: ReplayDomain::Canonical,
crates/amun-replay-engine/src/equivalence.rs:116:    fn execute_and_self_verify_produces_valid_proof() {
crates/amun-replay-engine/src/equivalence.rs:117:        let state = ReplayState::new([0; 32]);
crates/amun-replay-engine/src/equivalence.rs:119:        let proof = EquivalenceProver::execute_and_self_verify(&state, &entries, 1).unwrap();
crates/amun-replay-engine/src/equivalence.rs:11:    pub computed_result_hash: ConstitutionalHash,
crates/amun-replay-engine/src/equivalence.rs:125:    fn prove_over_result_accepts_valid_execution() {
crates/amun-replay-engine/src/equivalence.rs:126:        let state = ReplayState::new([0; 32]);
crates/amun-replay-engine/src/equivalence.rs:132:            expected_state = expected_state.apply_entry(entry).unwrap();
crates/amun-replay-engine/src/equivalence.rs:137:            DeterministicExecutor::execute_with_trace(&entries, state.state_root, 1).unwrap();
crates/amun-replay-engine/src/equivalence.rs:148:        let proof = EquivalenceProver::prove_over_result(&result, &expected_state).unwrap();
crates/amun-replay-engine/src/equivalence.rs:15:pub struct ConstitutionalReplayResult {
crates/amun-replay-engine/src/equivalence.rs:16:    pub replay_root: ConstitutionalHash,
crates/amun-replay-engine/src/equivalence.rs:17:    pub continuity_proof: ContinuityProof,
crates/amun-replay-engine/src/equivalence.rs:19:    pub equivalence_proof: EquivalenceProof,
crates/amun-replay-engine/src/equivalence.rs:20:    pub authority_proof: AuthorityProof,
crates/amun-replay-engine/src/equivalence.rs:21:    pub deterministic_transcript_hash: ConstitutionalHash,
crates/amun-replay-engine/src/equivalence.rs:25:pub struct ContinuityProof {
crates/amun-replay-engine/src/equivalence.rs:27:    pub chain_hash: ConstitutionalHash,
crates/amun-replay-engine/src/equivalence.rs:2:use crate::errors::ReplayFailure;
crates/amun-replay-engine/src/equivalence.rs:31:pub struct CheckpointResult {
crates/amun-replay-engine/src/equivalence.rs:32:    pub checkpoint_hash: ConstitutionalHash,
crates/amun-replay-engine/src/equivalence.rs:37:pub struct AuthorityProof {
crates/amun-replay-engine/src/equivalence.rs:38:    pub authority_root: ConstitutionalHash,
crates/amun-replay-engine/src/equivalence.rs:3:use crate::state::ReplayState;
crates/amun-replay-engine/src/equivalence.rs:42:impl ConstitutionalReplayResult {
crates/amun-replay-engine/src/equivalence.rs:43:    pub fn verify(&self) -> bool {
crates/amun-replay-engine/src/equivalence.rs:4:use amun_constitutional::{ConstitutionalHash, TranscriptEntry};
crates/amun-replay-engine/src/equivalence.rs:51:pub struct EquivalenceProver;
crates/amun-replay-engine/src/equivalence.rs:53:impl EquivalenceProver {
crates/amun-replay-engine/src/equivalence.rs:54:    pub fn prove_over_result(
crates/amun-replay-engine/src/equivalence.rs:56:        expected_state: &ReplayState,
crates/amun-replay-engine/src/equivalence.rs:57:    ) -> Result<EquivalenceProof, ReplayFailure> {
crates/amun-replay-engine/src/equivalence.rs:59:        Ok(EquivalenceProof {
crates/amun-replay-engine/src/equivalence.rs:63:            computed_result_hash: result.final_state,
crates/amun-replay-engine/src/equivalence.rs:67:    pub fn execute_and_self_verify(
crates/amun-replay-engine/src/equivalence.rs:68:        initial_state: &ReplayState,
crates/amun-replay-engine/src/equivalence.rs:71:    ) -> Result<EquivalenceProof, ReplayFailure> {
crates/amun-replay-engine/src/equivalence.rs:73:        let trace = DeterministicExecutor::execute_with_trace(
crates/amun-replay-engine/src/equivalence.rs:7:pub struct EquivalenceProof {
crates/amun-replay-engine/src/equivalence.rs:80:        // if we had applied the same entries to the ReplayState
crates/amun-replay-engine/src/equivalence.rs:91:        // For self-verification: replay the entries through ReplayState
crates/amun-replay-engine/src/equivalence.rs:95:            state = state.apply_entry(entry)?;
crates/amun-replay-engine/src/equivalence.rs:98:        Self::prove_over_result(&result, &state)
crates/amun-replay-engine/src/errors.rs:14:        expected_root: ConstitutionalHash,
crates/amun-replay-engine/src/errors.rs:15:        actual_root: ConstitutionalHash,
crates/amun-replay-engine/src/errors.rs:18:        transcript_hash: ConstitutionalHash,
crates/amun-replay-engine/src/errors.rs:19:        expected_root: ConstitutionalHash,
crates/amun-replay-engine/src/errors.rs:20:        actual_root: ConstitutionalHash,
crates/amun-replay-engine/src/errors.rs:26:        expected: ConstitutionalHash,
crates/amun-replay-engine/src/errors.rs:27:        actual: ConstitutionalHash,
crates/amun-replay-engine/src/errors.rs:30:        authority_id: ConstitutionalHash,
crates/amun-replay-engine/src/errors.rs:5:use amun_constitutional::ConstitutionalHash;
crates/amun-replay-engine/src/errors.rs:8:pub enum ReplayFailure {
crates/amun-replay-engine/src/execution_dag.rs:108:    pub fn vertex_count(&self) -> usize { self.vertices.len() }
crates/amun-replay-engine/src/execution_dag.rs:109:    pub fn edge_count(&self) -> usize { self.edges.len() }
crates/amun-replay-engine/src/execution_dag.rs:118:    fn test_empty_dag() {
crates/amun-replay-engine/src/execution_dag.rs:124:    fn test_add_vertex() {
crates/amun-replay-engine/src/execution_dag.rs:126:        dag.add_vertex(ExecutionVertex::new(1, VertexType::StateTransition, 100, [0xAB; 32], [0xBC; 32]));
crates/amun-replay-engine/src/execution_dag.rs:131:    fn test_topological_order() {
crates/amun-replay-engine/src/execution_dag.rs:133:        let v1 = ExecutionVertex::new(1, VertexType::StateTransition, 100, [0xAB; 32], [0xBC; 32])
crates/amun-replay-engine/src/execution_dag.rs:135:        let v2 = ExecutionVertex::new(2, VertexType::StateTransition, 100, [0xAB; 32], [0xBC; 32])
crates/amun-replay-engine/src/execution_dag.rs:157:    fn test_dag_is_scheduler_oblivious() {
crates/amun-replay-engine/src/execution_dag.rs:159:        // Constitutional truth must be identical regardless.
crates/amun-replay-engine/src/execution_dag.rs:161:        let v1 = ExecutionVertex::new(1, VertexType::StateTransition, 100, [0xAB; 32], [0xBC; 32])
crates/amun-replay-engine/src/execution_dag.rs:163:        let v2 = ExecutionVertex::new(2, VertexType::StateTransition, 200, [0xAB; 32], [0xBC; 32])
crates/amun-replay-engine/src/execution_dag.rs:165:        // v1 and v2 have no dependencies on each other — they can execute in any order
crates/amun-replay-engine/src/execution_dag.rs:17:pub struct ExecutionEdge {
crates/amun-replay-engine/src/execution_dag.rs:28:/// This DAG represents WHAT must execute before WHAT.
crates/amun-replay-engine/src/execution_dag.rs:32:pub struct ExecutionDAG {
crates/amun-replay-engine/src/execution_dag.rs:39:impl ExecutionDAG {
crates/amun-replay-engine/src/execution_dag.rs:40:    pub fn new() -> Self {
crates/amun-replay-engine/src/execution_dag.rs:45:    pub fn add_vertex(&mut self, vertex: ExecutionVertex) {
crates/amun-replay-engine/src/execution_dag.rs:50:    pub fn add_edge(&mut self, from: u64, to: u64, dep_type: ExecutionDependencyType) {
crates/amun-replay-engine/src/execution_dag.rs:56:    pub fn ready_vertices(&self) -> Vec<&ExecutionVertex> {
crates/amun-replay-engine/src/execution_dag.rs:75:    /// Constitutional truth MUST be identical regardless of the order chosen.
crates/amun-replay-engine/src/execution_dag.rs:76:    pub fn topological_order(&self) -> Vec<&ExecutionVertex> {
crates/amun-replay-engine/src/execution_dependency.rs:14:pub enum ExecutionDependencyType {
crates/amun-replay-engine/src/execution_dependency.rs:19:    RequiresWitness,
crates/amun-replay-engine/src/execution_dependency.rs:36:impl ExecutionDependencyType {
crates/amun-replay-engine/src/execution_dependency.rs:38:    pub fn is_mandatory(&self) -> bool {
crates/amun-replay-engine/src/execution_dependency.rs:42:                | ExecutionDependencyType::RequiresWitness
crates/amun-replay-engine/src/execution_dependency.rs:49:    pub fn is_optional(&self) -> bool {
crates/amun-replay-engine/src/execution_dependency.rs:4://! They describe what a vertex needs before it can execute, not why
crates/amun-replay-engine/src/execution_dependency.rs:62:    fn test_mandatory_dependencies() {
crates/amun-replay-engine/src/execution_dependency.rs:64:        assert!(ExecutionDependencyType::RequiresWitness.is_mandatory());
crates/amun-replay-engine/src/execution_dependency.rs:69:    fn test_optional_dependencies() {
crates/amun-replay-engine/src/execution_dependency.rs:9://!   - ExecutionDependency: what must execute first (runtime layer)
crates/amun-replay-engine/src/execution_scheduler.rs:107:    fn test_block_and_unblock() {
crates/amun-replay-engine/src/execution_scheduler.rs:118:    fn test_pending_count() {
crates/amun-replay-engine/src/execution_scheduler.rs:13:pub enum ScheduleDecision {
crates/amun-replay-engine/src/execution_scheduler.rs:21:pub struct ExecutionScheduler {
crates/amun-replay-engine/src/execution_scheduler.rs:27:    context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/execution_scheduler.rs:30:impl ExecutionScheduler {
crates/amun-replay-engine/src/execution_scheduler.rs:31:    pub fn new(context_hash: ConstitutionalHash) -> Self {
crates/amun-replay-engine/src/execution_scheduler.rs:35:    pub fn add_task(&mut self, task: ExecutionTask) {
crates/amun-replay-engine/src/execution_scheduler.rs:3://! The scheduler determines WHICH task executes NEXT.
crates/amun-replay-engine/src/execution_scheduler.rs:42:    pub fn next_task(&mut self) -> Option<&ExecutionTask> {
crates/amun-replay-engine/src/execution_scheduler.rs:60:    pub fn block_task(&mut self, task_id: u64) {
crates/amun-replay-engine/src/execution_scheduler.rs:69:    pub fn unblock_task(&mut self, task_id: u64) {
crates/amun-replay-engine/src/execution_scheduler.rs:78:    pub fn task_count(&self) -> usize { self.tasks.len() }
crates/amun-replay-engine/src/execution_scheduler.rs:79:    pub fn pending_count(&self) -> usize {
crates/amun-replay-engine/src/execution_scheduler.rs:89:    fn make_task(id: u64, seq: u64) -> ExecutionTask {
crates/amun-replay-engine/src/execution_scheduler.rs:90:        ExecutionTask::new(id, TaskType::StateTransition, [0xAB; 32], [0xBC; 32], [0xCD; 32], seq)
crates/amun-replay-engine/src/execution_scheduler.rs:94:    fn test_scheduler_fifo_order() {
crates/amun-replay-engine/src/execution_scheduler.rs:9:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/execution_task.rs:12:pub enum TaskType {
crates/amun-replay-engine/src/execution_task.rs:13:    /// Execute a state transition within a boundary.
crates/amun-replay-engine/src/execution_task.rs:14:    StateTransition = 0x01,
crates/amun-replay-engine/src/execution_task.rs:18:    ProduceWitness = 0x03,
crates/amun-replay-engine/src/execution_task.rs:26:/// Constitutional judgment happens AFTER the task produces artifacts.
crates/amun-replay-engine/src/execution_task.rs:28:pub struct ExecutionTask {
crates/amun-replay-engine/src/execution_task.rs:35:    /// The constitutional context this task executes within.
crates/amun-replay-engine/src/execution_task.rs:36:    pub context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/execution_task.rs:39:    pub boundary_hash: ConstitutionalHash,
crates/amun-replay-engine/src/execution_task.rs:3://! A task describes WHAT to execute, not HOW to schedule it.
crates/amun-replay-engine/src/execution_task.rs:42:    /// For StateTransition: the preceding journal entry.
crates/amun-replay-engine/src/execution_task.rs:43:    /// For VerifyArtifact: the artifact to verify.
crates/amun-replay-engine/src/execution_task.rs:44:    /// For ProduceWitness: the target artifact.
crates/amun-replay-engine/src/execution_task.rs:46:    pub target_artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/execution_task.rs:55:impl ExecutionTask {
crates/amun-replay-engine/src/execution_task.rs:56:    pub fn new(
crates/amun-replay-engine/src/execution_task.rs:59:        context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/execution_task.rs:60:        boundary_hash: ConstitutionalHash,
crates/amun-replay-engine/src/execution_task.rs:61:        target_artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/execution_task.rs:76:    fn test_task_creation() {
crates/amun-replay-engine/src/execution_task.rs:79:            TaskType::StateTransition,
crates/amun-replay-engine/src/execution_task.rs:86:        assert_eq!(task.task_type, TaskType::StateTransition);
crates/amun-replay-engine/src/execution_task.rs:8:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/execution_vertex.rs:102:    fn test_vertex_creation() {
crates/amun-replay-engine/src/execution_vertex.rs:103:        let v = ExecutionVertex::new(1, VertexType::StateTransition, 100, [0xAB; 32], [0xBC; 32])
crates/amun-replay-engine/src/execution_vertex.rs:10://! The vertex records WHAT was executed, not WHETHER it was valid.
crates/amun-replay-engine/src/execution_vertex.rs:113:    fn test_vertex_is_not_truth() {
crates/amun-replay-engine/src/execution_vertex.rs:13:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/execution_vertex.rs:17:pub enum VertexType {
crates/amun-replay-engine/src/execution_vertex.rs:18:    /// A state transition was executed.
crates/amun-replay-engine/src/execution_vertex.rs:19:    StateTransition,
crates/amun-replay-engine/src/execution_vertex.rs:21:    WitnessExtraction,
crates/amun-replay-engine/src/execution_vertex.rs:36:pub struct ExecutionVertex {
crates/amun-replay-engine/src/execution_vertex.rs:43:    /// The worker that executed this vertex (operational provenance).
crates/amun-replay-engine/src/execution_vertex.rs:47:    pub context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/execution_vertex.rs:4://!   - Constitutional validity
crates/amun-replay-engine/src/execution_vertex.rs:50:    pub boundary_hash: ConstitutionalHash,
crates/amun-replay-engine/src/execution_vertex.rs:54:    pub produced_artifacts: Vec<ConstitutionalHash>,
crates/amun-replay-engine/src/execution_vertex.rs:64:impl ExecutionVertex {
crates/amun-replay-engine/src/execution_vertex.rs:65:    pub fn new(
crates/amun-replay-engine/src/execution_vertex.rs:69:        context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/execution_vertex.rs:70:        boundary_hash: ConstitutionalHash,
crates/amun-replay-engine/src/execution_vertex.rs:85:    pub fn with_artifact(mut self, hash: ConstitutionalHash) -> Self {
crates/amun-replay-engine/src/execution_vertex.rs:91:    pub fn with_dependency(mut self, vertex_id: u64) -> Self {
crates/amun-replay-engine/src/frontier_reconciliation.rs:100:        f.recompute_completeness();
crates/amun-replay-engine/src/frontier_reconciliation.rs:105:        let a = make_frontier(100, vec![[0x01; 32]], vec![([0x02; 32], FrontierDependencyReason::HardWitnessRequired, true)]);
crates/amun-replay-engine/src/frontier_reconciliation.rs:117:        let target = make_frontier(200, vec![[0x01; 32]], vec![([0x02; 32], FrontierDependencyReason::HardWitnessRequired, true), ([0x03; 32], FrontierDependencyReason::CausalChainIncomplete, false)]);
crates/amun-replay-engine/src/frontier_reconciliation.rs:118:        let d = compute_closure_delta(&source, &target);
crates/amun-replay-engine/src/frontier_reconciliation.rs:11:    pub newly_resolved: Vec<ConstitutionalHash>,
crates/amun-replay-engine/src/frontier_reconciliation.rs:16:pub fn merge_frontiers(a: &DerivationalFrontier, b: &DerivationalFrontier) -> FrontierMergeResult {
crates/amun-replay-engine/src/frontier_reconciliation.rs:18:    let mut newly_resolved: Vec<ConstitutionalHash> = Vec::new();
crates/amun-replay-engine/src/frontier_reconciliation.rs:42:    merged.recompute_completeness();
crates/amun-replay-engine/src/frontier_reconciliation.rs:4:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/frontier_reconciliation.rs:55:pub enum FrontierReduction { AMoreComplete, BMoreComplete, EquallyComplete, Incomparable }
crates/amun-replay-engine/src/frontier_reconciliation.rs:57:pub fn compare_frontier_reduction(a: &DerivationalFrontier, b: &DerivationalFrontier) -> FrontierReduction {
crates/amun-replay-engine/src/frontier_reconciliation.rs:75:pub fn are_derivationally_equivalent(a: &DerivationalFrontier, b: &DerivationalFrontier) -> bool {
crates/amun-replay-engine/src/frontier_reconciliation.rs:79:pub fn compute_closure_delta(source: &DerivationalFrontier, target: &DerivationalFrontier) -> Vec<ConstitutionalHash> {
crates/amun-replay-engine/src/frontier_reconciliation.rs:80:    let mut delta: Vec<ConstitutionalHash> = Vec::new();
crates/amun-replay-engine/src/frontier_reconciliation.rs:94:    fn make_frontier(worker: u64, resolved: Vec<[u8; 32]>, unresolved: Vec<([u8; 32], FrontierDependencyReason, bool)>) -> DerivationalFrontier {
crates/amun-replay-engine/src/frontier_reconciliation.rs:9:pub struct FrontierMergeResult {
crates/amun-replay-engine/src/isolation_boundary.rs:10:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/isolation_boundary.rs:112:    pub fn check_context(&self, context_hash: ConstitutionalHash) -> Result<(), IsolationViolation> {
crates/amun-replay-engine/src/isolation_boundary.rs:125:    pub fn strict(
crates/amun-replay-engine/src/isolation_boundary.rs:129:        context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/isolation_boundary.rs:147:    fn test_capability_check_allowed() {
crates/amun-replay-engine/src/isolation_boundary.rs:154:    fn test_capability_check_denied() {
crates/amun-replay-engine/src/isolation_boundary.rs:15:pub enum IsolationViolation {
crates/amun-replay-engine/src/isolation_boundary.rs:161:    fn test_context_check_allowed() {
crates/amun-replay-engine/src/isolation_boundary.rs:168:    fn test_context_check_denied() {
crates/amun-replay-engine/src/isolation_boundary.rs:24:    ConstitutionalStateModification {
crates/amun-replay-engine/src/isolation_boundary.rs:26:        artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/isolation_boundary.rs:30:    WitnessTampering {
crates/amun-replay-engine/src/isolation_boundary.rs:38:        target_context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/isolation_boundary.rs:42:impl IsolationViolation {
crates/amun-replay-engine/src/isolation_boundary.rs:43:    pub fn describe(&self) -> Vec<u8> {
crates/amun-replay-engine/src/isolation_boundary.rs:46:            IsolationViolation::ConstitutionalStateModification { .. } => b"Constitutional state modification attempted".to_vec(),
crates/amun-replay-engine/src/isolation_boundary.rs:47:            IsolationViolation::WitnessTampering { .. } => b"Witness tampering attempted".to_vec(),
crates/amun-replay-engine/src/isolation_boundary.rs:60:pub struct ExecutionIsolationBoundary {
crates/amun-replay-engine/src/isolation_boundary.rs:71:    pub context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/isolation_boundary.rs:7://! is a constitutional requirement (Invariant 15).
crates/amun-replay-engine/src/isolation_boundary.rs:80:impl ExecutionIsolationBoundary {
crates/amun-replay-engine/src/isolation_boundary.rs:81:    pub fn new(
crates/amun-replay-engine/src/isolation_boundary.rs:85:        context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/isolation_boundary.rs:99:    pub fn check_capability(&self, requested: RuntimeCapability) -> Result<(), IsolationViolation> {
crates/amun-replay-engine/src/lib.rs:102:pub struct ReplayCursor {
crates/amun-replay-engine/src/lib.rs:104:    pub chain_hash: ConstitutionalHash,
crates/amun-replay-engine/src/lib.rs:108:impl ReplayCursor {
crates/amun-replay-engine/src/lib.rs:109:    pub fn new(start_sequence: u64) -> Self {
crates/amun-replay-engine/src/lib.rs:121:    pub fn advance_to(
crates/amun-replay-engine/src/lib.rs:125:    ) -> Result<(), ReplayFailure> {
crates/amun-replay-engine/src/lib.rs:131:        self.chain_hash = h.finalize().into();
crates/amun-replay-engine/src/lib.rs:143:    use amun_constitutional::ReplayDomain;
crates/amun-replay-engine/src/lib.rs:145:    fn mk_entry(seq: u64, hash: [u8; 32]) -> TranscriptEntry {
crates/amun-replay-engine/src/lib.rs:149:            domain: ReplayDomain::Canonical,
crates/amun-replay-engine/src/lib.rs:154:    fn session_replay_produces_self_verified_certificate() {
crates/amun-replay-engine/src/lib.rs:155:        let mut session = ReplaySession::new([0; 32], 0);
crates/amun-replay-engine/src/lib.rs:157:        let cert = session.replay(&entries).unwrap();
crates/amun-replay-engine/src/lib.rs:158:        assert!(cert.verify());
crates/amun-replay-engine/src/lib.rs:163:    fn session_detects_sequence_gap() {
crates/amun-replay-engine/src/lib.rs:164:        let mut session = ReplaySession::new([0; 32], 0);
crates/amun-replay-engine/src/lib.rs:166:        assert!(session.replay(&entries).is_err());
crates/amun-replay-engine/src/lib.rs:21:use amun_constitutional::{ConstitutionalHash, TranscriptEntry};
crates/amun-replay-engine/src/lib.rs:26:    AuthorityProof, CheckpointResult, ConstitutionalReplayResult, ContinuityProof,
crates/amun-replay-engine/src/lib.rs:29:use errors::ReplayFailure;
crates/amun-replay-engine/src/lib.rs:30:use state::ReplayState;
crates/amun-replay-engine/src/lib.rs:37:pub use deterministic::{ConstitutionalStep, ExecutionResult, ExecutionTrace};
crates/amun-replay-engine/src/lib.rs:39:// NOTE: ReplayState and ReplayFailure are imported via `use` above.
crates/amun-replay-engine/src/lib.rs:42:// STRUCT: ReplaySession
crates/amun-replay-engine/src/lib.rs:45:pub struct ReplaySession {
crates/amun-replay-engine/src/lib.rs:46:    pub state: ReplayState,
crates/amun-replay-engine/src/lib.rs:47:    pub cursor: ReplayCursor,
crates/amun-replay-engine/src/lib.rs:50:impl ReplaySession {
crates/amun-replay-engine/src/lib.rs:51:    pub fn new(initial_state_root: ConstitutionalHash, start_sequence: u64) -> Self {
crates/amun-replay-engine/src/lib.rs:53:            state: ReplayState::new(initial_state_root),
crates/amun-replay-engine/src/lib.rs:54:            cursor: ReplayCursor::new(start_sequence),
crates/amun-replay-engine/src/lib.rs:58:    pub fn replay(
crates/amun-replay-engine/src/lib.rs:61:    ) -> Result<ConstitutionalReplayResult, ReplayFailure> {
crates/amun-replay-engine/src/lib.rs:62:        let transcript_hash = DeterministicExecutor::compute_transcript_hash(entries);
crates/amun-replay-engine/src/lib.rs:67:            EquivalenceProver::execute_and_self_verify(&saved_state, entries, start_sequence)?;
crates/amun-replay-engine/src/lib.rs:69:        self.state = ReplayState::new(proof.trace.final_state_hash());
crates/amun-replay-engine/src/lib.rs:77:        Ok(ConstitutionalReplayResult {
crates/amun-replay-engine/src/lib.rs:78:            replay_root: self.state.state_root,
crates/amun-replay-engine/src/lib.rs:79:            continuity_proof: ContinuityProof {
crates/amun-replay-engine/src/lib.rs:88:            authority_proof: AuthorityProof {
crates/amun-replay-engine/src/lib.rs:98:// STRUCT: ReplayCursor
crates/amun-replay-engine/src/missing_closure.rs:101:    pub fn is_satisfied(&self) -> bool {
crates/amun-replay-engine/src/missing_closure.rs:111:    fn test_closure_request() {
crates/amun-replay-engine/src/missing_closure.rs:11:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/missing_closure.rs:125:    fn test_hop_exhaustion() {
crates/amun-replay-engine/src/missing_closure.rs:15:pub struct MissingClosureRequest {
crates/amun-replay-engine/src/missing_closure.rs:23:    pub target_artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/missing_closure.rs:26:    pub context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/missing_closure.rs:29:    pub available_hashes: Vec<ConstitutionalHash>,
crates/amun-replay-engine/src/missing_closure.rs:32:    pub missing_hashes: Vec<ConstitutionalHash>,
crates/amun-replay-engine/src/missing_closure.rs:46:pub enum ClosureType {
crates/amun-replay-engine/src/missing_closure.rs:57:impl MissingClosureRequest {
crates/amun-replay-engine/src/missing_closure.rs:58:    pub fn new(
crates/amun-replay-engine/src/missing_closure.rs:61:        target_artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/missing_closure.rs:62:        context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/missing_closure.rs:79:    pub fn with_available(mut self, hash: ConstitutionalHash) -> Self {
crates/amun-replay-engine/src/missing_closure.rs:85:    pub fn with_missing(mut self, hash: ConstitutionalHash) -> Self {
crates/amun-replay-engine/src/missing_closure.rs:91:    pub fn increment_hop(&mut self) {
crates/amun-replay-engine/src/missing_closure.rs:96:    pub fn is_exhausted(&self) -> bool {
crates/amun-replay-engine/src/operational_hasher.rs:18:/// Constitutional objects use `amun_constitutional::ConstitutionalHasher`.
crates/amun-replay-engine/src/operational_hasher.rs:20:pub struct OperationalHasher {
crates/amun-replay-engine/src/operational_hasher.rs:24:impl OperationalHasher {
crates/amun-replay-engine/src/operational_hasher.rs:25:    pub fn new(tag: &[u8]) -> Self {
crates/amun-replay-engine/src/operational_hasher.rs:32:    pub fn update_u64(&mut self, v: u64) -> &mut Self { self.inner.update(v.to_le_bytes()); self }
crates/amun-replay-engine/src/operational_hasher.rs:33:    pub fn update_u32(&mut self, v: u32) -> &mut Self { self.inner.update(v.to_le_bytes()); self }
crates/amun-replay-engine/src/operational_hasher.rs:34:    pub fn update_u8(&mut self, v: u8) -> &mut Self { self.inner.update(&[v]); self }
crates/amun-replay-engine/src/operational_hasher.rs:35:    pub fn update_bytes(&mut self, v: &[u8]) -> &mut Self { self.inner.update(v); self }
crates/amun-replay-engine/src/operational_hasher.rs:37:    pub fn finalize(self) -> [u8; 32] { self.inner.finalize().into() }
crates/amun-replay-engine/src/operational_hasher.rs:42:pub struct OperationalHash(pub [u8; 32]);
crates/amun-replay-engine/src/operational_hasher.rs:49:    fn test_operational_hash_deterministic() {
crates/amun-replay-engine/src/operational_hasher.rs:50:        let h1 = OperationalHasher::new(b"TEST").update_u64(42).clone().finalize();
crates/amun-replay-engine/src/operational_hasher.rs:51:        let h2 = OperationalHasher::new(b"TEST").update_u64(42).clone().finalize();
crates/amun-replay-engine/src/operational_hasher.rs:56:    fn test_operational_separate_from_constitutional() {
crates/amun-replay-engine/src/operational_hasher.rs:58:        let op_hash = OperationalHasher::new(b"TEST").update_u64(42).clone().finalize();
crates/amun-replay-engine/src/operational_hasher.rs:59:        // Constitutional hasher would produce a different hash for the same data
crates/amun-replay-engine/src/operational_hasher.rs:61:        assert_ne!(op_hash, [0u8; 32]); // Just verify it's not empty
crates/amun-replay-engine/src/proof_routing.rs:108:pub enum RoutingReason {
crates/amun-replay-engine/src/proof_routing.rs:10:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/proof_routing.rs:126:pub fn closure_aware_routing(
crates/amun-replay-engine/src/proof_routing.rs:127:    route: &ProofRoute,
crates/amun-replay-engine/src/proof_routing.rs:130:    known_fingerprints: &[ConstitutionalHash],
crates/amun-replay-engine/src/proof_routing.rs:15:pub struct ProofRoute {
crates/amun-replay-engine/src/proof_routing.rs:183:pub fn equivalence_aware_compression(
crates/amun-replay-engine/src/proof_routing.rs:184:    target_fingerprint: ConstitutionalHash,
crates/amun-replay-engine/src/proof_routing.rs:185:    known_fingerprints: &[ConstitutionalHash],
crates/amun-replay-engine/src/proof_routing.rs:18:    pub target_artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/proof_routing.rs:192:pub fn within_propagation_budget(
crates/amun-replay-engine/src/proof_routing.rs:1://! Proof Routing Fabric — constitutional proof logistics.
crates/amun-replay-engine/src/proof_routing.rs:204:    fn make_frontier(worker: u64, resolved: Vec<[u8; 32]>, unresolved: Vec<([u8; 32], FrontierDependencyReason, bool)>) -> DerivationalFrontier {
crates/amun-replay-engine/src/proof_routing.rs:20:    pub context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/proof_routing.rs:210:        f.recompute_completeness();
crates/amun-replay-engine/src/proof_routing.rs:215:    fn test_quarantine_skip() {
crates/amun-replay-engine/src/proof_routing.rs:216:        let route = ProofRoute::new(1, [0xBB; 32], [0xAB; 32], RoutedProofType::ClosureDelta);
crates/amun-replay-engine/src/proof_routing.rs:217:        let frontier = make_frontier(100, vec![], vec![([0xBB; 32], FrontierDependencyReason::HardWitnessRequired, true)]);
crates/amun-replay-engine/src/proof_routing.rs:228:    fn test_redundant_equivalence() {
crates/amun-replay-engine/src/proof_routing.rs:229:        let route = ProofRoute::new(1, [0xBB; 32], [0xAB; 32], RoutedProofType::EquivalenceFingerprint);
crates/amun-replay-engine/src/proof_routing.rs:22:    pub proof_type: RoutedProofType,
crates/amun-replay-engine/src/proof_routing.rs:241:    fn test_frontier_reduction() {
crates/amun-replay-engine/src/proof_routing.rs:242:        let route = ProofRoute::new(1, [0xBB; 32], [0xAB; 32], RoutedProofType::ClosureDelta);
crates/amun-replay-engine/src/proof_routing.rs:243:        let frontier = make_frontier(300, vec![], vec![([0xBB; 32], FrontierDependencyReason::HardWitnessRequired, true)]);
crates/amun-replay-engine/src/proof_routing.rs:254:    fn test_hop_expiry() {
crates/amun-replay-engine/src/proof_routing.rs:255:        let mut route = ProofRoute::new(1, [0xBB; 32], [0xAB; 32], RoutedProofType::FullWitness);
crates/amun-replay-engine/src/proof_routing.rs:265:    fn test_propagation_budget() {
crates/amun-replay-engine/src/proof_routing.rs:271:    fn test_equivalence_compression() {
crates/amun-replay-engine/src/proof_routing.rs:35:pub enum RoutedProofType {
crates/amun-replay-engine/src/proof_routing.rs:37:    FullWitness,
crates/amun-replay-engine/src/proof_routing.rs:48:pub enum RoutePriority {
crates/amun-replay-engine/src/proof_routing.rs:58:impl ProofRoute {
crates/amun-replay-engine/src/proof_routing.rs:59:    pub fn new(
crates/amun-replay-engine/src/proof_routing.rs:60:        route_id: u64, target: ConstitutionalHash, context: ConstitutionalHash,
crates/amun-replay-engine/src/proof_routing.rs:61:        proof_type: RoutedProofType,
crates/amun-replay-engine/src/proof_routing.rs:71:    pub fn with_frontier_priority(mut self, frontier: &DerivationalFrontier) -> Self {
crates/amun-replay-engine/src/proof_routing.rs:79:    pub fn with_avoid_workers(mut self, workers: Vec<u64>) -> Self {
crates/amun-replay-engine/src/proof_routing.rs:85:    pub fn increment_hop(&mut self) -> bool {
crates/amun-replay-engine/src/proof_routing.rs:91:    pub fn is_expired(&self) -> bool { self.current_hops > self.max_hops }
crates/amun-replay-engine/src/proof_routing.rs:96:pub struct RoutingDecision {
crates/amun-replay-engine/src/replay_result.rs:1://! replay_result — stub (pending migration to constitutional kernel)
crates/amun-replay-engine/src/runtime_anomaly.rs:100:    pub fn should_quarantine(&self) -> bool {
crates/amun-replay-engine/src/runtime_anomaly.rs:110:    fn test_anomaly_classification() {
crates/amun-replay-engine/src/runtime_anomaly.rs:121:    fn test_quarantine_threshold() {
crates/amun-replay-engine/src/runtime_anomaly.rs:12:pub enum AnomalyType {
crates/amun-replay-engine/src/runtime_anomaly.rs:131:    fn test_anomaly_is_not_invalidity() {
crates/amun-replay-engine/src/runtime_anomaly.rs:134:        let anomaly = RuntimeAnomaly::new(1, 100, AnomalyType::SuspiciousWitness, [0xAB; 32]);
crates/amun-replay-engine/src/runtime_anomaly.rs:16:    SuspiciousWitness,
crates/amun-replay-engine/src/runtime_anomaly.rs:33:pub struct RuntimeAnomaly {
crates/amun-replay-engine/src/runtime_anomaly.rs:41:    pub context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/runtime_anomaly.rs:48:impl RuntimeAnomaly {
crates/amun-replay-engine/src/runtime_anomaly.rs:49:    pub fn new(
crates/amun-replay-engine/src/runtime_anomaly.rs:51:        context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/runtime_anomaly.rs:57:    pub fn is_hostile(&self) -> bool {
crates/amun-replay-engine/src/runtime_anomaly.rs:67:    /// Returns true if this anomaly type is potentially benign (network issue, etc.).
crates/amun-replay-engine/src/runtime_anomaly.rs:68:    pub fn is_benign(&self) -> bool {
crates/amun-replay-engine/src/runtime_anomaly.rs:76:    pub fn increment(&mut self) { self.occurrence_count += 1; }
crates/amun-replay-engine/src/runtime_anomaly.rs:81:pub struct RuntimeAnomalySurface {
crates/amun-replay-engine/src/runtime_anomaly.rs:88:impl RuntimeAnomalySurface {
crates/amun-replay-engine/src/runtime_anomaly.rs:89:    pub fn new(worker_id: u64) -> Self {
crates/amun-replay-engine/src/runtime_anomaly.rs:8:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/runtime_anomaly.rs:93:    pub fn record(&mut self, anomaly: RuntimeAnomaly) {
crates/amun-replay-engine/src/runtime_capability.rs:13:pub enum RuntimeCapability {
crates/amun-replay-engine/src/runtime_capability.rs:14:    /// Worker may execute state transitions within a boundary.
crates/amun-replay-engine/src/runtime_capability.rs:24:    RequestWitness,
crates/amun-replay-engine/src/runtime_capability.rs:26:    /// Worker may verify existing artifacts against the constitutional kernel.
crates/amun-replay-engine/src/runtime_capability.rs:35:pub struct CapabilitySet {
crates/amun-replay-engine/src/runtime_capability.rs:39:impl CapabilitySet {
crates/amun-replay-engine/src/runtime_capability.rs:40:    pub fn new(capabilities: Vec<RuntimeCapability>) -> Self {
crates/amun-replay-engine/src/runtime_capability.rs:44:    pub fn has(&self, cap: RuntimeCapability) -> bool {
crates/amun-replay-engine/src/runtime_capability.rs:49:    pub fn artifact_producer() -> Self {
crates/amun-replay-engine/src/runtime_capability.rs:57:    pub fn verifier() -> Self {
crates/amun-replay-engine/src/runtime_capability.rs:61:            RuntimeCapability::RequestWitness,
crates/amun-replay-engine/src/runtime_capability.rs:66:    pub fn recovery() -> Self {
crates/amun-replay-engine/src/runtime_capability.rs:71:            RuntimeCapability::RequestWitness,
crates/amun-replay-engine/src/runtime_capability.rs:81:    fn test_producer_cannot_verify() {
crates/amun-replay-engine/src/runtime_capability.rs:88:    fn test_verifier_cannot_execute() {
crates/amun-replay-engine/src/runtime_capability.rs:95:    fn test_recovery_can_restore() {
crates/amun-replay-engine/src/runtime_receipt.rs:11:pub struct RuntimeReceipt {
crates/amun-replay-engine/src/runtime_receipt.rs:18:    pub produced_artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/runtime_receipt.rs:22:impl RuntimeReceipt {
crates/amun-replay-engine/src/runtime_receipt.rs:23:    pub fn new(
crates/amun-replay-engine/src/runtime_receipt.rs:26:        execution_success: bool, produced_artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/runtime_receipt.rs:34:        r.receipt_hash = r.compute_hash();
crates/amun-replay-engine/src/runtime_receipt.rs:38:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-replay-engine/src/runtime_receipt.rs:47:        h.finalize()
crates/amun-replay-engine/src/runtime_receipt.rs:4://! OperationalHasher, NOT ConstitutionalHasher, because operational
crates/amun-replay-engine/src/runtime_receipt.rs:7:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/state.rs:103:            domain: ReplayDomain::Canonical,
crates/amun-replay-engine/src/state.rs:108:            domain: ReplayDomain::Canonical,
crates/amun-replay-engine/src/state.rs:111:        let r1 = state.apply_entry(&e1).unwrap();
crates/amun-replay-engine/src/state.rs:112:        let r2 = state.apply_entry(&e2).unwrap();
crates/amun-replay-engine/src/state.rs:15:use amun_constitutional::{ConstitutionalHash, TranscriptEntry};
crates/amun-replay-engine/src/state.rs:18:use crate::errors::ReplayFailure;
crates/amun-replay-engine/src/state.rs:21:pub struct ReplayState {
crates/amun-replay-engine/src/state.rs:22:    pub state_root: ConstitutionalHash,
crates/amun-replay-engine/src/state.rs:24:    pub divergences: Vec<ReplayFailure>,
crates/amun-replay-engine/src/state.rs:27:impl ReplayState {
crates/amun-replay-engine/src/state.rs:28:    pub fn new(initial_root: ConstitutionalHash) -> Self {
crates/amun-replay-engine/src/state.rs:36:    pub fn is_divergent(&self) -> bool {
crates/amun-replay-engine/src/state.rs:44:    pub fn apply_entry(&self, entry: &TranscriptEntry) -> Result<Self, ReplayFailure> {
crates/amun-replay-engine/src/state.rs:50:        let new_root = hasher.finalize();
crates/amun-replay-engine/src/state.rs:63:    use amun_constitutional::ReplayDomain;
crates/amun-replay-engine/src/state.rs:66:    fn state_root_evolves_after_entry() {
crates/amun-replay-engine/src/state.rs:67:        let state = ReplayState::new([0xAA; 32]);
crates/amun-replay-engine/src/state.rs:73:            domain: ReplayDomain::Canonical,
crates/amun-replay-engine/src/state.rs:76:        let new_state = state.apply_entry(&entry).unwrap();
crates/amun-replay-engine/src/state.rs:82:    fn same_entry_same_result() {
crates/amun-replay-engine/src/state.rs:83:        let state = ReplayState::new([0xAA; 32]);
crates/amun-replay-engine/src/state.rs:87:            domain: ReplayDomain::Canonical,
crates/amun-replay-engine/src/state.rs:90:        let r1 = state.apply_entry(&entry).unwrap();
crates/amun-replay-engine/src/state.rs:91:        let r2 = state.apply_entry(&entry).unwrap();
crates/amun-replay-engine/src/state.rs:98:    fn different_entries_different_results() {
crates/amun-replay-engine/src/state.rs:99:        let state = ReplayState::new([0xAA; 32]);
crates/amun-replay-engine/src/temporal_drift.rs:100:pub struct CompatibilityDecaySurface {
crates/amun-replay-engine/src/temporal_drift.rs:10:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/temporal_drift.rs:111:impl CompatibilityDecaySurface {
crates/amun-replay-engine/src/temporal_drift.rs:112:    pub fn new(revision: u32, compatibility_depth: u32) -> Self {
crates/amun-replay-engine/src/temporal_drift.rs:118:    pub fn decay(&mut self) {
crates/amun-replay-engine/src/temporal_drift.rs:124:    pub fn is_active(&self) -> bool { self.requires_active_support && !self.compatibility_decayed }
crates/amun-replay-engine/src/temporal_drift.rs:132:pub struct TemporalAttributionLineage {
crates/amun-replay-engine/src/temporal_drift.rs:134:    pub revision_sequence: Vec<(u32, ConstitutionalHash)>,
crates/amun-replay-engine/src/temporal_drift.rs:141:impl TemporalAttributionLineage {
crates/amun-replay-engine/src/temporal_drift.rs:142:    pub fn new(initial_revision: u32, initial_hash: ConstitutionalHash) -> Self {
crates/amun-replay-engine/src/temporal_drift.rs:146:    pub fn record_activation(&mut self, revision: u32, hash: ConstitutionalHash) {
crates/amun-replay-engine/src/temporal_drift.rs:153:    pub fn check_age_authority(&self) -> bool { self.age_based_authority_claimed }
crates/amun-replay-engine/src/temporal_drift.rs:162:pub struct EpochContainmentZone {
crates/amun-replay-engine/src/temporal_drift.rs:171:impl EpochContainmentZone {
crates/amun-replay-engine/src/temporal_drift.rs:172:    pub fn new(max_percent: u8) -> Self {
crates/amun-replay-engine/src/temporal_drift.rs:176:    pub fn check_epoch_dominance(&mut self, epoch_id: u32, lineage_share_percent: u8) {
crates/amun-replay-engine/src/temporal_drift.rs:189:    fn test_derivable_age_boundary() {
crates/amun-replay-engine/src/temporal_drift.rs:197:    fn test_historical_weight_neutrality() {
crates/amun-replay-engine/src/temporal_drift.rs:19:pub struct TemporalDriftBoundary {
crates/amun-replay-engine/src/temporal_drift.rs:205:    fn test_compatibility_decay() {
crates/amun-replay-engine/src/temporal_drift.rs:214:    fn test_lineage_never_grants_age_authority() {
crates/amun-replay-engine/src/temporal_drift.rs:220:    fn test_epoch_containment() {
crates/amun-replay-engine/src/temporal_drift.rs:228:    fn test_temporal_precedence_is_not_constitutional_precedence() {
crates/amun-replay-engine/src/temporal_drift.rs:33:impl TemporalDriftBoundary {
crates/amun-replay-engine/src/temporal_drift.rs:34:    pub fn new(boundary_id: u64, active_revision: u32, max_age: u32) -> Self {
crates/amun-replay-engine/src/temporal_drift.rs:40:    pub fn is_derivable(&self, revision: u32) -> bool {
crates/amun-replay-engine/src/temporal_drift.rs:45:    pub fn record_active_revision(&mut self, revision: u32) {
crates/amun-replay-engine/src/temporal_drift.rs:52:    pub fn check_inertia(&mut self) {
crates/amun-replay-engine/src/temporal_drift.rs:66:pub struct HistoricalWeightNeutralizer {
crates/amun-replay-engine/src/temporal_drift.rs:68:    pub tracked_artifacts: Vec<ConstitutionalHash>,
crates/amun-replay-engine/src/temporal_drift.rs:75:impl HistoricalWeightNeutralizer {
crates/amun-replay-engine/src/temporal_drift.rs:76:    pub fn new(max_usage: u64) -> Self {
crates/amun-replay-engine/src/temporal_drift.rs:81:    pub fn check_historical_usage(&mut self, artifact_hash: ConstitutionalHash, usage_count: u64) -> bool {
crates/amun-replay-engine/src/transition.rs:1://! transition — stub (pending migration to constitutional kernel)
crates/amun-replay-engine/src/version.rs:1:/// Replay protocol version - FROZEN for protocol v1.
crates/amun-replay-engine/src/version.rs:5:/// Constitutional hash domain for protocol version binding.
crates/amun-replay-engine/src/witness_envelope.rs:100:    fn test_envelope_creation() {
crates/amun-replay-engine/src/witness_envelope.rs:101:        let env = WitnessEnvelope::new(
crates/amun-replay-engine/src/witness_envelope.rs:110:    fn test_envelope_hash_deterministic() {
crates/amun-replay-engine/src/witness_envelope.rs:111:        let e1 = WitnessEnvelope::new(1, [0xAA; 32], 100, [0xAB; 32], [0xBB; 32], PropagationScope::ContextLocal);
crates/amun-replay-engine/src/witness_envelope.rs:112:        let e2 = WitnessEnvelope::new(1, [0xAA; 32], 100, [0xAB; 32], [0xBB; 32], PropagationScope::ContextLocal);
crates/amun-replay-engine/src/witness_envelope.rs:117:    fn test_envelope_is_not_witness() {
crates/amun-replay-engine/src/witness_envelope.rs:119:        let e1 = WitnessEnvelope::new(1, [0xAA; 32], 100, [0xAB; 32], [0xBB; 32], PropagationScope::ContextLocal);
crates/amun-replay-engine/src/witness_envelope.rs:120:        let e2 = WitnessEnvelope::new(1, [0xAA; 32], 100, [0xAB; 32], [0xBB; 32], PropagationScope::FullBroadcast);
crates/amun-replay-engine/src/witness_envelope.rs:12:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/witness_envelope.rs:1://! WitnessEnvelope — transport container for witness propagation.
crates/amun-replay-engine/src/witness_envelope.rs:21:pub struct WitnessEnvelope {
crates/amun-replay-engine/src/witness_envelope.rs:26:    pub witness_hash: ConstitutionalHash,
crates/amun-replay-engine/src/witness_envelope.rs:32:    pub context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/witness_envelope.rs:35:    pub target_artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/witness_envelope.rs:49:pub enum PropagationScope {
crates/amun-replay-engine/src/witness_envelope.rs:60:impl WitnessEnvelope {
crates/amun-replay-engine/src/witness_envelope.rs:61:    pub fn new(
crates/amun-replay-engine/src/witness_envelope.rs:63:        witness_hash: ConstitutionalHash,
crates/amun-replay-engine/src/witness_envelope.rs:65:        context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/witness_envelope.rs:66:        target_artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/witness_envelope.rs:79:        e.envelope_hash = e.compute_hash();
crates/amun-replay-engine/src/witness_envelope.rs:83:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-replay-engine/src/witness_envelope.rs:8://! The envelope answers: "Here is a proof surface for you to verify."
crates/amun-replay-engine/src/witness_envelope.rs:91:        h.finalize()
crates/amun-replay-engine/src/zk_adapters.rs:100:        let mut h = ConstitutionalHasher::new(b"DERIVABILITY_COMMITMENT");
crates/amun-replay-engine/src/zk_adapters.rs:104:        h.finalize()
crates/amun-replay-engine/src/zk_adapters.rs:10:use amun_constitutional::kernel_types::ConstitutionalHash;
crates/amun-replay-engine/src/zk_adapters.rs:110:pub struct RecursiveProofBoundary {
crates/amun-replay-engine/src/zk_adapters.rs:117:impl RecursiveProofBoundary {
crates/amun-replay-engine/src/zk_adapters.rs:118:    pub fn new(max_depth: u32) -> Self {
crates/amun-replay-engine/src/zk_adapters.rs:11:use amun_constitutional::ConstitutionalHasher;
crates/amun-replay-engine/src/zk_adapters.rs:123:    pub fn can_recurse(&self) -> bool {
crates/amun-replay-engine/src/zk_adapters.rs:128:    pub fn recurse(&mut self) -> bool {
crates/amun-replay-engine/src/zk_adapters.rs:141:pub struct SelectiveRevealSurface {
crates/amun-replay-engine/src/zk_adapters.rs:143:    pub target_artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:145:    pub revealed_hashes: Vec<ConstitutionalHash>,
crates/amun-replay-engine/src/zk_adapters.rs:147:    pub fingerprint: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:149:    pub surface_binding: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:152:impl SelectiveRevealSurface {
crates/amun-replay-engine/src/zk_adapters.rs:153:    pub fn new(
crates/amun-replay-engine/src/zk_adapters.rs:154:        target: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:155:        revealed: Vec<ConstitutionalHash>,
crates/amun-replay-engine/src/zk_adapters.rs:156:        fingerprint: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:164:        s.surface_binding = s.compute_binding();
crates/amun-replay-engine/src/zk_adapters.rs:168:    fn compute_binding(&self) -> ConstitutionalHash {
crates/amun-replay-engine/src/zk_adapters.rs:169:        let mut h = ConstitutionalHasher::new(b"SELECTIVE_REVEAL");
crates/amun-replay-engine/src/zk_adapters.rs:175:        h.finalize()
crates/amun-replay-engine/src/zk_adapters.rs:184:pub struct ExternalVerifierAdapter {
crates/amun-replay-engine/src/zk_adapters.rs:187:    /// The derivability commitment being exported.
crates/amun-replay-engine/src/zk_adapters.rs:188:    pub commitment: DerivabilityCommitment,
crates/amun-replay-engine/src/zk_adapters.rs:189:    /// A cryptographic proof that the commitment is valid under the kernel.
crates/amun-replay-engine/src/zk_adapters.rs:18:pub struct ZKWitnessEnvelope {
crates/amun-replay-engine/src/zk_adapters.rs:193:impl ExternalVerifierAdapter {
crates/amun-replay-engine/src/zk_adapters.rs:194:    pub fn new(
crates/amun-replay-engine/src/zk_adapters.rs:196:        commitment: DerivabilityCommitment,
crates/amun-replay-engine/src/zk_adapters.rs:199:        Self { verifier_system_id, commitment, export_proof }
crates/amun-replay-engine/src/zk_adapters.rs:1://! ZK Witness Adapters — cryptographic portability for derivability.
crates/amun-replay-engine/src/zk_adapters.rs:209:pub struct ProofInspectabilityGuard {
crates/amun-replay-engine/src/zk_adapters.rs:20:    pub target_artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:211:    pub source_witness_hash: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:218:impl ProofInspectabilityGuard {
crates/amun-replay-engine/src/zk_adapters.rs:219:    pub fn new(source_witness_hash: ConstitutionalHash) -> Self {
crates/amun-replay-engine/src/zk_adapters.rs:224:    pub fn is_inspectable(&self) -> bool {
crates/amun-replay-engine/src/zk_adapters.rs:22:    pub context_hash: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:234:    fn test_zk_envelope_creation() {
crates/amun-replay-engine/src/zk_adapters.rs:235:        let env = ZKWitnessEnvelope::new(
crates/amun-replay-engine/src/zk_adapters.rs:243:    fn test_derivability_commitment() {
crates/amun-replay-engine/src/zk_adapters.rs:245:        assert_ne!(c.commitment, [0; 32]);
crates/amun-replay-engine/src/zk_adapters.rs:249:    fn test_recursive_boundary() {
crates/amun-replay-engine/src/zk_adapters.rs:24:    pub asserted_fingerprint: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:250:        let mut boundary = RecursiveProofBoundary::new(2);
crates/amun-replay-engine/src/zk_adapters.rs:260:    fn test_selective_reveal() {
crates/amun-replay-engine/src/zk_adapters.rs:269:    fn test_external_verifier_adapter() {
crates/amun-replay-engine/src/zk_adapters.rs:270:        let commitment = DerivabilityCommitment::new([0xAA; 32], [0xBB; 32], [0xCC; 32]);
crates/amun-replay-engine/src/zk_adapters.rs:271:        let adapter = ExternalVerifierAdapter::new([0xEE; 32], commitment, b"proof".to_vec());
crates/amun-replay-engine/src/zk_adapters.rs:276:    fn test_inspectability_guard() {
crates/amun-replay-engine/src/zk_adapters.rs:277:        let guard = ProofInspectabilityGuard::new([0xDD; 32]);
crates/amun-replay-engine/src/zk_adapters.rs:30:    pub envelope_hash: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:33:impl ZKWitnessEnvelope {
crates/amun-replay-engine/src/zk_adapters.rs:34:    pub fn new(
crates/amun-replay-engine/src/zk_adapters.rs:35:        target: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:36:        context: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:37:        fingerprint: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:48:        e.envelope_hash = e.compute_hash();
crates/amun-replay-engine/src/zk_adapters.rs:52:    fn compute_hash(&self) -> ConstitutionalHash {
crates/amun-replay-engine/src/zk_adapters.rs:53:        let mut h = ConstitutionalHasher::new(b"ZK_WITNESS_ENVELOPE");
crates/amun-replay-engine/src/zk_adapters.rs:58:        h.finalize()
crates/amun-replay-engine/src/zk_adapters.rs:62:    pub fn mark_verified(&mut self) {
crates/amun-replay-engine/src/zk_adapters.rs:67:/// A commitment to a specific derivability outcome, suitable for ZK wrapping.
crates/amun-replay-engine/src/zk_adapters.rs:69:/// This commitment cryptographically binds the admissibility outcome
crates/amun-replay-engine/src/zk_adapters.rs:72:pub struct DerivabilityCommitment {
crates/amun-replay-engine/src/zk_adapters.rs:74:    pub target_artifact_hash: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:75:    /// The canonical derivation fingerprint being committed to.
crates/amun-replay-engine/src/zk_adapters.rs:76:    pub fingerprint: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:79:    /// The actual commitment hash.
crates/amun-replay-engine/src/zk_adapters.rs:80:    pub commitment: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:83:impl DerivabilityCommitment {
crates/amun-replay-engine/src/zk_adapters.rs:84:    pub fn new(
crates/amun-replay-engine/src/zk_adapters.rs:85:        target: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:86:        fingerprint: ConstitutionalHash,
crates/amun-replay-engine/src/zk_adapters.rs:93:            commitment: [0; 32],
crates/amun-replay-engine/src/zk_adapters.rs:95:        c.commitment = c.compute_commitment();
crates/amun-replay-engine/src/zk_adapters.rs:99:    fn compute_commitment(&self) -> ConstitutionalHash {
crates/amun-replay-optimization/src/lib.rs:11:pub struct CachedHeader {
crates/amun-replay-optimization/src/lib.rs:17:pub struct ReplayCache {
crates/amun-replay-optimization/src/lib.rs:24:impl ReplayCache {
crates/amun-replay-optimization/src/lib.rs:25:    pub fn new() -> Self {
crates/amun-replay-optimization/src/lib.rs:34:    pub fn store_certificate(&mut self, cert: CachedCertificate) {
crates/amun-replay-optimization/src/lib.rs:38:    pub fn check_certificate(&mut self, cert_hash: &[u8; 32]) -> bool {
crates/amun-replay-optimization/src/lib.rs:48:    pub fn store_header(&mut self, header: CachedHeader) {
crates/amun-replay-optimization/src/lib.rs:4:pub struct CachedCertificate {
crates/amun-replay-optimization/src/lib.rs:52:    pub fn get_header(&mut self, height: u64) -> Option<&CachedHeader> {
crates/amun-replay-optimization/src/lib.rs:61:    pub fn hit_rate(&self) -> f64 {
crates/amun-replay-optimization/src/lib.rs:70:    pub fn compute_cache_root(&self) -> [u8; 32] {
crates/amun-replay-optimization/src/lib.rs:75:        hasher.finalize().into()
crates/amun-replay-optimization/src/lib.rs:78:    pub fn batch_verify_certificates(
crates/amun-replay-optimization/src/lib.rs:93:impl Default for ReplayCache {
crates/amun-replay-optimization/src/lib.rs:94:    fn default() -> Self {
crates/amun-replay-optimization/tests/n163_replay_tests.rs:1:use amun_replay_optimization::*;
crates/amun-replay-optimization/tests/n163_replay_tests.rs:21:fn n163_batch_verification_faster_than_individual() {
crates/amun-replay-optimization/tests/n163_replay_tests.rs:22:    let mut cache = ReplayCache::new();
crates/amun-replay-optimization/tests/n163_replay_tests.rs:40:    let valid = cache.batch_verify_certificates(&cert_hashes, true);
crates/amun-replay-optimization/tests/n163_replay_tests.rs:46:fn n163_header_cache_speeds_sync() {
crates/amun-replay-optimization/tests/n163_replay_tests.rs:47:    let mut cache = ReplayCache::new();
crates/amun-replay-optimization/tests/n163_replay_tests.rs:4:fn n163_cache_hit_improves_replay() {
crates/amun-replay-optimization/tests/n163_replay_tests.rs:5:    let mut cache = ReplayCache::new();
crates/amun-replay-optimization/tests/n163_replay_tests.rs:63:fn n163_cache_root_deterministic() {
crates/amun-replay-optimization/tests/n163_replay_tests.rs:64:    let mut cache1 = ReplayCache::new();
crates/amun-replay-optimization/tests/n163_replay_tests.rs:65:    let mut cache2 = ReplayCache::new();
crates/amun-replay-optimization/tests/n163_replay_tests.rs:81:    assert_eq!(cache1.compute_cache_root(), cache2.compute_cache_root());
crates/amun-replay-semantics/src/lib.rs:12:pub struct ReplayEpoch { pub epoch_id: [u8; 32], pub start_sequence: u64, pub end_sequence: Option<u64>, pub replay_version: u32 }
crates/amun-replay-semantics/src/lib.rs:13:impl ReplayEpoch {
crates/amun-replay-semantics/src/lib.rs:14:    pub fn new(epoch_id: [u8; 32], start_sequence: u64, replay_version: u32) -> Self { Self { epoch_id, start_sequence, end_sequence: None, replay_version } }
crates/amun-replay-semantics/src/lib.rs:15:    pub fn contains(&self, sequence: u64) -> bool { sequence >= self.start_sequence && self.end_sequence.map(|e| sequence <= e).unwrap_or(true) }
crates/amun-replay-semantics/src/lib.rs:19:pub struct ReplayBoundary { pub finalized_sequence: u64, pub boundary_chain_hash: [u8; 32], pub boundary_state_root: [u8; 32], pub epoch: ReplayEpoch, pub replay_version: u32 }
crates/amun-replay-semantics/src/lib.rs:1://! Replay Semantics — formal constitutional model for replay.
crates/amun-replay-semantics/src/lib.rs:20:impl ReplayBoundary { pub fn genesis(genesis_hash: [u8; 32]) -> Self { Self { finalized_sequence: 0, boundary_chain_hash: genesis_hash, boundary_state_root: genesis_hash, epoch: ReplayEpoch::new(genesis_hash, 0, 1), replay_version: 1 } } }
crates/amun-replay-semantics/src/lib.rs:23:pub struct ReplayCertificate {
crates/amun-replay-semantics/src/lib.rs:24:    pub domain: ReplayDomain, pub epoch: ReplayEpoch,
crates/amun-replay-semantics/src/lib.rs:27:    pub boundary: ReplayBoundary, pub event_count: u64, pub replay_version: u32,
crates/amun-replay-semantics/src/lib.rs:2://! Layer 0.75 — Constitutional Replay Theory.
crates/amun-replay-semantics/src/lib.rs:30:impl ReplayCertificate {
crates/amun-replay-semantics/src/lib.rs:32:    pub fn new(domain: ReplayDomain, epoch: ReplayEpoch, transcript_root: [u8; 32], state_root: [u8; 32], receipt_root: [u8; 32], ordering_root: [u8; 32], boundary: ReplayBoundary, event_count: u64, replay_version: u32) -> Self {
crates/amun-replay-semantics/src/lib.rs:33:        let mut cert = Self { domain, epoch, transcript_root, state_root, receipt_root, ordering_root, boundary, event_count, replay_version, certificate_hash: [0; 32] };
crates/amun-replay-semantics/src/lib.rs:34:        cert.certificate_hash = cert.compute_hash(); cert
crates/amun-replay-semantics/src/lib.rs:36:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-replay-semantics/src/lib.rs:39:        h.update(self.boundary.boundary_chain_hash); h.update(self.event_count.to_le_bytes()); h.update(self.replay_version.to_le_bytes());
crates/amun-replay-semantics/src/lib.rs:3://! Defines WHAT replay IS, not HOW to implement it.
crates/amun-replay-semantics/src/lib.rs:40:        h.finalize().into()
crates/amun-replay-semantics/src/lib.rs:42:    pub fn verify(&self) -> bool { self.certificate_hash == self.compute_hash() }
crates/amun-replay-semantics/src/lib.rs:43:    pub fn prove_equivalence(a: &Self, b: &Self) -> bool { a.transcript_root == b.transcript_root && a.state_root == b.state_root && a.receipt_root == b.receipt_root && a.ordering_root == b.ordering_root && a.domain == b.domain }
crates/amun-replay-semantics/src/lib.rs:46:#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum ReplayEquivalence { Strict, Semantic, EpochBounded(ReplayEpoch) }
crates/amun-replay-semantics/src/lib.rs:47:impl ReplayEquivalence {
crates/amun-replay-semantics/src/lib.rs:48:    pub fn verify(&self, a: &ReplayCertificate, b: &ReplayCertificate) -> bool {
crates/amun-replay-semantics/src/lib.rs:49:        match self { ReplayEquivalence::Strict => a.certificate_hash == b.certificate_hash, ReplayEquivalence::Semantic => ReplayCertificate::prove_equivalence(a, b), ReplayEquivalence::EpochBounded(ep) => a.epoch.epoch_id == ep.epoch_id && b.epoch.epoch_id == ep.epoch_id && ReplayCertificate::prove_equivalence(a, b) }
crates/amun-replay-semantics/src/lib.rs:54:pub enum ReplayFailure {
crates/amun-replay-semantics/src/lib.rs:58:    EpochBoundaryViolation { expected_epoch: Box<ReplayEpoch>, actual_epoch: Box<ReplayEpoch> },
crates/amun-replay-semantics/src/lib.rs:60:    BoundaryViolation { expected_boundary: Box<ReplayBoundary>, actual_boundary: Box<ReplayBoundary> },
crates/amun-replay-semantics/src/lib.rs:61:    ReplayResourceExhaustion { limit: usize, attempted: usize },
crates/amun-replay-semantics/src/lib.rs:64:#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum ReplayAuthority { SelfVerification, ValidatorQuorum { required_signatures: u64 }, ConstitutionalCourt, PublicVerification }
crates/amun-replay-semantics/src/lib.rs:67:pub struct ReplayWitness { pub start_sequence: u64, pub end_sequence: u64, pub pre_state_root: [u8; 32], pub post_state_root: [u8; 32], pub transcript_fragment_hash: [u8; 32], pub witness_data: Vec<u8> }
crates/amun-replay-semantics/src/lib.rs:70:pub struct ReplayCheckpoint { pub sequence: u64, pub state_root: [u8; 32], pub transcript_chain_hash: [u8; 32], pub certificate: ReplayCertificate }
crates/amun-replay-semantics/src/lib.rs:71:impl ReplayCheckpoint { pub fn verify(&self, boundary: &ReplayBoundary) -> bool { self.sequence >= boundary.finalized_sequence && self.certificate.verify() && self.certificate.boundary.boundary_chain_hash == boundary.boundary_chain_hash } }
crates/amun-replay-semantics/src/lib.rs:75:    pub fn replay_determinism(a: &ReplayCertificate, b: &ReplayCertificate) -> Result<(), ReplayFailure> {
crates/amun-replay-semantics/src/lib.rs:76:        if a.transcript_root != b.transcript_root { return Err(ReplayFailure::TranscriptMismatch { expected_root: a.transcript_root, actual_root: b.transcript_root }); }
crates/amun-replay-semantics/src/lib.rs:77:        if a.state_root != b.state_root { return Err(ReplayFailure::StateDivergence { expected_root: a.state_root, actual_root: b.state_root }); }
crates/amun-replay-semantics/src/lib.rs:85:    fn te() -> ReplayEpoch { ReplayEpoch::new([0xBB; 32], 0, 1) }
crates/amun-replay-semantics/src/lib.rs:86:    fn tb() -> ReplayBoundary { ReplayBoundary::genesis([0xAA; 32]) }
crates/amun-replay-semantics/src/lib.rs:87:    #[test] fn test_cert_self_verifying() { assert!(ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1).verify()); }
crates/amun-replay-semantics/src/lib.rs:88:    #[test] fn test_cert_tamper_detected() { let mut c = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); c.state_root = [0xFF;32]; assert!(!c.verify()); }
crates/amun-replay-semantics/src/lib.rs:89:    #[test] fn test_equivalence_strict() { let c = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); assert!(ReplayEquivalence::Strict.verify(&c, &c)); }
crates/amun-replay-semantics/src/lib.rs:90:    #[test] fn test_law_determinism_divergence() { let a = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); let b = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0xFF;32], [0x03;32], [0x04;32], tb(), 100, 1); assert!(laws::replay_determinism(&a, &b).is_err()); }
crates/amun-replay-semantics/src/lib.rs:9:#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum ReplayDomain { Consensus, Execution, FullSystem, Governance, Transcript }
crates/amun-replay-store/src/lib.rs:100:    fn n45_append_and_load() {
crates/amun-replay-store/src/lib.rs:101:        let path = "/tmp/n45_replay_store.json";
crates/amun-replay-store/src/lib.rs:103:        let store = ReplayStore::new(path);
crates/amun-replay-store/src/lib.rs:113:    fn n45_replay_sequence() {
crates/amun-replay-store/src/lib.rs:116:        let store = ReplayStore::new(path);
crates/amun-replay-store/src/lib.rs:120:        assert!(store.verify_chain().unwrap());
crates/amun-replay-store/src/lib.rs:125:    fn n45_corruption_detection() {
crates/amun-replay-store/src/lib.rs:128:        let store = ReplayStore::new(path);
crates/amun-replay-store/src/lib.rs:132:        assert!(!store.verify_chain().unwrap());
crates/amun-replay-store/src/lib.rs:137:    fn n45_height_lookup() {
crates/amun-replay-store/src/lib.rs:13:    pub commit_hash: String,
crates/amun-replay-store/src/lib.rs:140:        let store = ReplayStore::new(path);
crates/amun-replay-store/src/lib.rs:150:    fn n45_empty_store() {
crates/amun-replay-store/src/lib.rs:153:        let store = ReplayStore::new(path);
crates/amun-replay-store/src/lib.rs:155:        assert!(store.verify_chain().unwrap());
crates/amun-replay-store/src/lib.rs:16:impl ReplayRecord {
crates/amun-replay-store/src/lib.rs:18:    pub fn record_hash(&self) -> String {
crates/amun-replay-store/src/lib.rs:25:        hasher.update(self.commit_hash.as_bytes());
crates/amun-replay-store/src/lib.rs:26:        hex::encode(hasher.finalize().as_bytes())
crates/amun-replay-store/src/lib.rs:30:/// Persistent store for replay journal.
crates/amun-replay-store/src/lib.rs:31:pub struct ReplayStore {
crates/amun-replay-store/src/lib.rs:35:impl ReplayStore {
crates/amun-replay-store/src/lib.rs:36:    pub fn new(path: &str) -> Self {
crates/amun-replay-store/src/lib.rs:42:    /// Append a replay record to the journal.
crates/amun-replay-store/src/lib.rs:43:    pub fn append(&self, record: &ReplayRecord) -> Result<(), String> {
crates/amun-replay-store/src/lib.rs:53:    /// Load all replay records.
crates/amun-replay-store/src/lib.rs:54:    pub fn load_all(&self) -> Result<Vec<ReplayRecord>, String> {
crates/amun-replay-store/src/lib.rs:62:    /// Load a specific replay record by height.
crates/amun-replay-store/src/lib.rs:63:    pub fn load_height(&self, height: u64) -> Result<Option<ReplayRecord>, String> {
crates/amun-replay-store/src/lib.rs:6:/// A single replay record capturing a state transition.
crates/amun-replay-store/src/lib.rs:70:    pub fn verify_chain(&self) -> Result<bool, String> {
crates/amun-replay-store/src/lib.rs:89:    fn make_record(h: u64, before: &str, after: &str) -> ReplayRecord {
crates/amun-replay-store/src/lib.rs:8:pub struct ReplayRecord {
crates/amun-replay-store/src/lib.rs:90:        ReplayRecord {
crates/amun-replay-store/src/lib.rs:95:            commit_hash: format!("commit{h}"),
crates/amun-replay-verifier/src/lib.rs:32:pub mod replay_verifier;
crates/amun-replay-verifier/src/lib.rs:33:pub use replay_verifier::*;
crates/amun-replay-verifier/src/replay_verifier.rs:100:                    return ReplayResult::Divergence {
crates/amun-replay-verifier/src/replay_verifier.rs:103:                        replay: hex::encode(replay_proof.proof_hash),
crates/amun-replay-verifier/src/replay_verifier.rs:106:                ReplayResult::Match {
crates/amun-replay-verifier/src/replay_verifier.rs:108:                    proof_hash: replay_proof.proof_hash,
crates/amun-replay-verifier/src/replay_verifier.rs:111:            Ok(PipelineResult::Rejected { evidence, .. }) => ReplayResult::Error {
crates/amun-replay-verifier/src/replay_verifier.rs:114:            Err(e) => ReplayResult::Error { reason: e },
crates/amun-replay-verifier/src/replay_verifier.rs:124:    fn make_id(seed: u8) -> ResourceId {
crates/amun-replay-verifier/src/replay_verifier.rs:131:    fn w14_replay_equality() {
crates/amun-replay-verifier/src/replay_verifier.rs:132:        let mut r1 = ResourceRegistry::new(1000);
crates/amun-replay-verifier/src/replay_verifier.rs:133:        let mut r2 = ResourceRegistry::new(1000);
crates/amun-replay-verifier/src/replay_verifier.rs:134:        let p = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Push(42), OpCode::Halt]);
crates/amun-replay-verifier/src/replay_verifier.rs:13:pub enum ReplayResult {
crates/amun-replay-verifier/src/replay_verifier.rs:141:            pre_state_root: r1.compute_state_root(),
crates/amun-replay-verifier/src/replay_verifier.rs:144:        let mut h = HotProofStore::new(100);
crates/amun-replay-verifier/src/replay_verifier.rs:145:        let mut a = ProofArchive::new();
crates/amun-replay-verifier/src/replay_verifier.rs:147:            ConstitutionalRuntime::execute(&p, &ctx, &mut r1, &[], 10000, &mut h, &mut a).unwrap();
crates/amun-replay-verifier/src/replay_verifier.rs:150:                transition_proof, ..
crates/amun-replay-verifier/src/replay_verifier.rs:151:            } => transition_proof,
crates/amun-replay-verifier/src/replay_verifier.rs:155:            ReplayVerifier::replay(&proof, &p, &mut r2, &[]),
crates/amun-replay-verifier/src/replay_verifier.rs:156:            ReplayResult::Match { .. }
crates/amun-replay-verifier/src/replay_verifier.rs:161:    fn w14_replay_detects_tampered_proof() {
crates/amun-replay-verifier/src/replay_verifier.rs:162:        let mut r = ResourceRegistry::new(1000);
crates/amun-replay-verifier/src/replay_verifier.rs:163:        let p = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
crates/amun-replay-verifier/src/replay_verifier.rs:170:            pre_state_root: r.compute_state_root(),
crates/amun-replay-verifier/src/replay_verifier.rs:173:        let mut h = HotProofStore::new(100);
crates/amun-replay-verifier/src/replay_verifier.rs:174:        let mut a = ProofArchive::new();
crates/amun-replay-verifier/src/replay_verifier.rs:176:            ConstitutionalRuntime::execute(&p, &ctx, &mut r, &[], 10000, &mut h, &mut a).unwrap();
crates/amun-replay-verifier/src/replay_verifier.rs:179:                transition_proof, ..
crates/amun-replay-verifier/src/replay_verifier.rs:180:            } => transition_proof,
crates/amun-replay-verifier/src/replay_verifier.rs:184:        let mut r2 = ResourceRegistry::new(1000);
crates/amun-replay-verifier/src/replay_verifier.rs:186:            ReplayVerifier::replay(&proof, &p, &mut r2, &[]),
crates/amun-replay-verifier/src/replay_verifier.rs:187:            ReplayResult::Match { .. }
crates/amun-replay-verifier/src/replay_verifier.rs:192:    fn w14_multi_replay_same_result() {
crates/amun-replay-verifier/src/replay_verifier.rs:193:        let p = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
crates/amun-replay-verifier/src/replay_verifier.rs:1:use amun_bytecode::program::ConstitutionalProgram;
crates/amun-replay-verifier/src/replay_verifier.rs:203:        let mut h = HotProofStore::new(100);
crates/amun-replay-verifier/src/replay_verifier.rs:204:        let mut a = ProofArchive::new();
crates/amun-replay-verifier/src/replay_verifier.rs:205:        let mut r = ResourceRegistry::new(1000);
crates/amun-replay-verifier/src/replay_verifier.rs:207:            ConstitutionalRuntime::execute(&p, &ctx, &mut r, &[], 10000, &mut h, &mut a).unwrap();
crates/amun-replay-verifier/src/replay_verifier.rs:210:                transition_proof, ..
crates/amun-replay-verifier/src/replay_verifier.rs:211:            } => transition_proof,
crates/amun-replay-verifier/src/replay_verifier.rs:215:            let mut f = ResourceRegistry::new(1000);
crates/amun-replay-verifier/src/replay_verifier.rs:217:                ReplayVerifier::replay(&proof, &p, &mut f, &[]),
crates/amun-replay-verifier/src/replay_verifier.rs:218:                ReplayResult::Match { .. }
crates/amun-replay-verifier/src/replay_verifier.rs:21:        replay: String,
crates/amun-replay-verifier/src/replay_verifier.rs:28:pub struct ReplayVerifier;
crates/amun-replay-verifier/src/replay_verifier.rs:2:use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
crates/amun-replay-verifier/src/replay_verifier.rs:30:impl ReplayVerifier {
crates/amun-replay-verifier/src/replay_verifier.rs:31:    pub fn replay(
crates/amun-replay-verifier/src/replay_verifier.rs:32:        proof: &TransitionProof,
crates/amun-replay-verifier/src/replay_verifier.rs:33:        program: &ConstitutionalProgram,
crates/amun-replay-verifier/src/replay_verifier.rs:34:        registry: &mut ResourceRegistry,
crates/amun-replay-verifier/src/replay_verifier.rs:35:        invariants: &[InvariantDeclaration],
crates/amun-replay-verifier/src/replay_verifier.rs:36:    ) -> ReplayResult {
crates/amun-replay-verifier/src/replay_verifier.rs:3:use amun_invariant_engine::invariant_types::InvariantDeclaration;
crates/amun-replay-verifier/src/replay_verifier.rs:4:use amun_proof_archive::hot_store::HotProofStore;
crates/amun-replay-verifier/src/replay_verifier.rs:50:                    lineage: ResourceLineage::genesis(meta.lineage.parent_resource_ids[0]),
crates/amun-replay-verifier/src/replay_verifier.rs:5:use amun_proof_archive::proof_archive::ProofArchive;
crates/amun-replay-verifier/src/replay_verifier.rs:67:        let mut hot_store = HotProofStore::new(1000);
crates/amun-replay-verifier/src/replay_verifier.rs:68:        let mut archive = ProofArchive::new();
crates/amun-replay-verifier/src/replay_verifier.rs:69:        let result = ConstitutionalRuntime::execute(
crates/amun-replay-verifier/src/replay_verifier.rs:7:    ResourceArchetype, ResourceLineage, ResourceMetadata, ResourceRegistry, ResourceState,
crates/amun-replay-verifier/src/replay_verifier.rs:82:                transition_proof: replay_proof,
crates/amun-replay-verifier/src/replay_verifier.rs:86:                    return ReplayResult::Divergence {
crates/amun-replay-verifier/src/replay_verifier.rs:89:                        replay: hex::encode(post_state_root),
crates/amun-replay-verifier/src/replay_verifier.rs:92:                if replay_proof.gas_used != proof.gas_used {
crates/amun-replay-verifier/src/replay_verifier.rs:93:                    return ReplayResult::Divergence {
crates/amun-replay-verifier/src/replay_verifier.rs:96:                        replay: replay_proof.gas_used.to_string(),
crates/amun-replay-verifier/src/replay_verifier.rs:99:                if replay_proof.proof_hash != proof.proof_hash {
crates/amun-replay-verifier/src/replay_verifier.rs:9:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-replay/src/certificate.rs:109:    pub fn is_empty(&self) -> bool {
crates/amun-replay/src/certificate.rs:112:    pub fn len(&self) -> usize {
crates/amun-replay/src/certificate.rs:115:    pub fn latest(&self) -> Option<&ReplayCertificate> {
crates/amun-replay/src/certificate.rs:11:    pub replay_root: [u8; 32],
crates/amun-replay/src/certificate.rs:125:    use crate::commit_log::StateCommit;
crates/amun-replay/src/certificate.rs:126:    use crate::validation::{ReplayResult, ReplayValidator};
crates/amun-replay/src/certificate.rs:128:    fn make_commit(h: u64, prev: [u8; 32], new: [u8; 32]) -> StateCommit {
crates/amun-replay/src/certificate.rs:12:    pub commits_checked: usize,
crates/amun-replay/src/certificate.rs:139:    fn valid_result() -> ReplayResult {
crates/amun-replay/src/certificate.rs:140:        ReplayValidator::validate(&[
crates/amun-replay/src/certificate.rs:141:            make_commit(1, [0u8; 32], [10u8; 32]),
crates/amun-replay/src/certificate.rs:142:            make_commit(2, [10u8; 32], [20u8; 32]),
crates/amun-replay/src/certificate.rs:143:            make_commit(3, [20u8; 32], [30u8; 32]),
crates/amun-replay/src/certificate.rs:148:    fn n37_genesis_certificate() {
crates/amun-replay/src/certificate.rs:149:        let genesis = ReplayCertificate::genesis([0xAA; 32], 1000);
crates/amun-replay/src/certificate.rs:150:        assert!(genesis.verify());
crates/amun-replay/src/certificate.rs:156:    fn n37_valid_chain() {
crates/amun-replay/src/certificate.rs:157:        let mut store = ReplayCertificateStore::new();
crates/amun-replay/src/certificate.rs:158:        let genesis = ReplayCertificate::genesis([0xAA; 32], 1000);
crates/amun-replay/src/certificate.rs:162:        let cert1 = ReplayCertificate::issue(&result, 3, prev_id, [0xAA; 32], 2000).unwrap();
crates/amun-replay/src/certificate.rs:165:        let cert2 = ReplayCertificate::issue(&result, 6, prev_id2, [0xAA; 32], 3000).unwrap();
crates/amun-replay/src/certificate.rs:171:    fn n37_broken_chain_rejected() {
crates/amun-replay/src/certificate.rs:172:        let mut store = ReplayCertificateStore::new();
crates/amun-replay/src/certificate.rs:173:        let genesis = ReplayCertificate::genesis([0xAA; 32], 1000);
crates/amun-replay/src/certificate.rs:176:        let mut cert = ReplayCertificate::issue(&result, 3, [0xFF; 32], [0xAA; 32], 2000).unwrap();
crates/amun-replay/src/certificate.rs:17:impl ReplayCertificate {
crates/amun-replay/src/certificate.rs:182:    fn n37_tampered_replay_root_rejected() {
crates/amun-replay/src/certificate.rs:183:        let mut store = ReplayCertificateStore::new();
crates/amun-replay/src/certificate.rs:184:        let genesis = ReplayCertificate::genesis([0xAA; 32], 1000);
crates/amun-replay/src/certificate.rs:188:        let mut cert = ReplayCertificate::issue(&result, 3, prev_id, [0xAA; 32], 2000).unwrap();
crates/amun-replay/src/certificate.rs:189:        cert.replay_root = [0xFF; 32];
crates/amun-replay/src/certificate.rs:18:    pub fn issue(
crates/amun-replay/src/certificate.rs:194:    fn n37_certificate_chain_verification() {
crates/amun-replay/src/certificate.rs:195:        let mut store = ReplayCertificateStore::new();
crates/amun-replay/src/certificate.rs:197:            .store(ReplayCertificate::genesis([0xAA; 32], 1000))
crates/amun-replay/src/certificate.rs:19:        result: &ReplayResult,
crates/amun-replay/src/certificate.rs:1:use crate::validation::ReplayResult;
crates/amun-replay/src/certificate.rs:202:            .store(ReplayCertificate::issue(&r, 3, id1, [0xAA; 32], 2000).unwrap())
crates/amun-replay/src/certificate.rs:206:            .store(ReplayCertificate::issue(&r, 6, id2, [0xAA; 32], 3000).unwrap())
crates/amun-replay/src/certificate.rs:20:        commit_height: u64,
crates/amun-replay/src/certificate.rs:211:            assert!(c.verify());
crates/amun-replay/src/certificate.rs:31:            commit_height,
crates/amun-replay/src/certificate.rs:33:            replay_root: [0u8; 32],
crates/amun-replay/src/certificate.rs:34:            commits_checked: result.commits_checked,
crates/amun-replay/src/certificate.rs:41:        hasher.update(&result.commits_checked.to_le_bytes());
crates/amun-replay/src/certificate.rs:42:        cert.replay_root = hasher.finalize().into();
crates/amun-replay/src/certificate.rs:43:        cert.certificate_id = cert.compute_id();
crates/amun-replay/src/certificate.rs:47:    pub fn genesis(validator_hash: [u8; 32], timestamp: u64) -> Self {
crates/amun-replay/src/certificate.rs:51:            commit_height: 0,
crates/amun-replay/src/certificate.rs:53:            replay_root: [0u8; 32],
crates/amun-replay/src/certificate.rs:54:            commits_checked: 0,
crates/amun-replay/src/certificate.rs:58:        cert.certificate_id = cert.compute_id();
crates/amun-replay/src/certificate.rs:62:    fn compute_id(&self) -> [u8; 32] {
crates/amun-replay/src/certificate.rs:66:        hasher.update(&self.commit_height.to_le_bytes());
crates/amun-replay/src/certificate.rs:68:        hasher.update(&self.replay_root);
crates/amun-replay/src/certificate.rs:69:        hasher.update(&self.commits_checked.to_le_bytes());
crates/amun-replay/src/certificate.rs:6:pub struct ReplayCertificate {
crates/amun-replay/src/certificate.rs:72:        hasher.finalize().into()
crates/amun-replay/src/certificate.rs:75:    pub fn verify(&self) -> bool {
crates/amun-replay/src/certificate.rs:76:        self.certificate_id == self.compute_id()
crates/amun-replay/src/certificate.rs:81:pub struct ReplayCertificateStore {
crates/amun-replay/src/certificate.rs:82:    certificates: HashMap<[u8; 32], ReplayCertificate>,
crates/amun-replay/src/certificate.rs:86:impl ReplayCertificateStore {
crates/amun-replay/src/certificate.rs:87:    pub fn new() -> Self {
crates/amun-replay/src/certificate.rs:91:    pub fn store(&mut self, cert: ReplayCertificate) -> Result<(), &'static str> {
crates/amun-replay/src/certificate.rs:92:        if !cert.verify() {
crates/amun-replay/src/certificate.rs:9:    pub commit_height: u64,
crates/amun-replay/src/commit_log.rs:111:    fn n34_commit_hash_deterministic() {
crates/amun-replay/src/commit_log.rs:114:        assert_eq!(c.commit_hash(), c.commit_hash());
crates/amun-replay/src/commit_log.rs:118:    fn n34_different_commits_different_hash() {
crates/amun-replay/src/commit_log.rs:124:        assert_ne!(c1.commit_hash(), c2.commit_hash());
crates/amun-replay/src/commit_log.rs:14:impl StateCommit {
crates/amun-replay/src/commit_log.rs:15:    pub fn commit_hash(&self) -> [u8; 32] {
crates/amun-replay/src/commit_log.rs:23:        hasher.finalize().into()
crates/amun-replay/src/commit_log.rs:27:/// An append-only log of all state transitions.
crates/amun-replay/src/commit_log.rs:29:pub struct CommitLog {
crates/amun-replay/src/commit_log.rs:30:    pub commits: Vec<StateCommit>,
crates/amun-replay/src/commit_log.rs:33:impl CommitLog {
crates/amun-replay/src/commit_log.rs:34:    pub fn new() -> Self {
crates/amun-replay/src/commit_log.rs:38:    /// Record a new state commit.
crates/amun-replay/src/commit_log.rs:39:    pub fn record(
crates/amun-replay/src/commit_log.rs:3:/// A record of a state transition committed to the chain.
crates/amun-replay/src/commit_log.rs:48:        let commit = StateCommit {
crates/amun-replay/src/commit_log.rs:56:        self.commits.push(commit);
crates/amun-replay/src/commit_log.rs:57:        self.commits
crates/amun-replay/src/commit_log.rs:59:            .expect("commit_log: invariant violated — empty after push")
crates/amun-replay/src/commit_log.rs:5:pub struct StateCommit {
crates/amun-replay/src/commit_log.rs:63:    pub fn latest_root(&self) -> Option<[u8; 32]> {
crates/amun-replay/src/commit_log.rs:64:        self.commits.last().map(|c| c.new_root)
crates/amun-replay/src/commit_log.rs:67:    /// Number of commits.
crates/amun-replay/src/commit_log.rs:68:    pub fn is_empty(&self) -> bool {
crates/amun-replay/src/commit_log.rs:69:        self.commits.is_empty()
crates/amun-replay/src/commit_log.rs:71:    pub fn len(&self) -> usize {
crates/amun-replay/src/commit_log.rs:72:        self.commits.len()
crates/amun-replay/src/commit_log.rs:75:    /// Get a commit by height.
crates/amun-replay/src/commit_log.rs:76:    pub fn get(&self, height: u64) -> Option<&StateCommit> {
crates/amun-replay/src/commit_log.rs:77:        self.commits.get(height as usize - 1)
crates/amun-replay/src/commit_log.rs:86:    fn n34_record_commit() {
crates/amun-replay/src/commit_log.rs:88:        let commit = log.record(1, [1u8; 32], [0u8; 32], [10u8; 32], 3, 1000);
crates/amun-replay/src/commit_log.rs:89:        assert_eq!(commit.height, 1);
crates/amun-replay/src/commit_log.rs:95:    fn n34_commit_chain_continuity() {
crates/amun-replay/src/lib.rs:33:pub mod commit_log;
crates/amun-replay/src/lib.rs:36:pub use certificate::{ReplayCertificate, ReplayCertificateStore};
crates/amun-replay/src/lib.rs:37:pub use commit_log::{CommitLog, StateCommit};
crates/amun-replay/src/lib.rs:38:pub use validation::{ReplayResult, ReplayValidator};
crates/amun-replay/src/lib.rs:40:pub use store::{CertificateProvider, ReplayStore};
crates/amun-replay/src/store.rs:10:impl ReplayStore {
crates/amun-replay/src/store.rs:11:    pub fn new() -> Self {
crates/amun-replay/src/store.rs:18:    pub fn insert(&mut self, cert: ReplayCertificate) {
crates/amun-replay/src/store.rs:1:use amun_constitutional_state::ReplayCertificate;
crates/amun-replay/src/store.rs:24:    pub fn get(&self, hash: &[u8; 32]) -> Option<&ReplayCertificate> {
crates/amun-replay/src/store.rs:29:    pub fn contains(&self, hash: &[u8; 32]) -> bool {
crates/amun-replay/src/store.rs:34:    pub fn len(&self) -> usize {
crates/amun-replay/src/store.rs:39:    pub fn is_empty(&self) -> bool {
crates/amun-replay/src/store.rs:44:impl Default for ReplayStore {
crates/amun-replay/src/store.rs:45:    fn default() -> Self {
crates/amun-replay/src/store.rs:4:/// Stores ReplayCertificates keyed by their certificate_hash.
crates/amun-replay/src/store.rs:50:/// A generic interface for retrieving ReplayCertificates by hash.
crates/amun-replay/src/store.rs:51:pub trait CertificateProvider {
crates/amun-replay/src/store.rs:52:    fn get_certificate(&self, hash: &[u8; 32]) -> Option<ReplayCertificate>;
crates/amun-replay/src/store.rs:55:impl CertificateProvider for ReplayStore {
crates/amun-replay/src/store.rs:56:    fn get_certificate(&self, hash: &[u8; 32]) -> Option<ReplayCertificate> {
crates/amun-replay/src/store.rs:5:/// Enables any node or light client to retrieve and verify state provenance.
crates/amun-replay/src/store.rs:64:    use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-replay/src/store.rs:67:    fn test_store_and_retrieve() {
crates/amun-replay/src/store.rs:68:        let mut rt = ConstitutionalStateRuntime::new();
crates/amun-replay/src/store.rs:69:        rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
crates/amun-replay/src/store.rs:6:pub struct ReplayStore {
crates/amun-replay/src/store.rs:73:        let mut store = ReplayStore::new();
crates/amun-replay/src/store.rs:7:    certificates: BTreeMap<[u8; 32], ReplayCertificate>,
crates/amun-replay/src/store.rs:82:    fn test_missing_certificate() {
crates/amun-replay/src/store.rs:83:        let store = ReplayStore::new();
crates/amun-replay/src/store.rs:89:    fn test_unique_hashes() {
crates/amun-replay/src/store.rs:90:        let mut rt = ConstitutionalStateRuntime::new();
crates/amun-replay/src/store.rs:91:        rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
crates/amun-replay/src/store.rs:94:        let mut rt2 = ConstitutionalStateRuntime::new();
crates/amun-replay/src/store.rs:95:        rt2.apply_transition(&[2u8; 32], &[0xBB; 32]);
crates/amun-replay/src/store.rs:98:        let mut store = ReplayStore::new();
crates/amun-replay/src/validation.rs:103:    fn n35_empty_commits() {
crates/amun-replay/src/validation.rs:104:        let result = ReplayValidator::validate(&[]);
crates/amun-replay/src/validation.rs:106:        assert_eq!(result.commits_checked, 0);
crates/amun-replay/src/validation.rs:110:    fn n35_replay_result_deterministic() {
crates/amun-replay/src/validation.rs:111:        let commits = vec![
crates/amun-replay/src/validation.rs:112:            make_commit(1, [0u8; 32], [10u8; 32]),
crates/amun-replay/src/validation.rs:113:            make_commit(2, [10u8; 32], [20u8; 32]),
crates/amun-replay/src/validation.rs:115:        let r1 = ReplayValidator::validate(&commits);
crates/amun-replay/src/validation.rs:116:        let r2 = ReplayValidator::validate(&commits);
crates/amun-replay/src/validation.rs:11:impl ReplayResult {
crates/amun-replay/src/validation.rs:12:    pub fn success(expected: [u8; 32], count: usize) -> Self {
crates/amun-replay/src/validation.rs:15:            replayed_root: expected,
crates/amun-replay/src/validation.rs:17:            commits_checked: count,
crates/amun-replay/src/validation.rs:1:use crate::commit_log::{CommitLog, StateCommit};
crates/amun-replay/src/validation.rs:21:    pub fn failure(expected: [u8; 32], actual: [u8; 32], count: usize) -> Self {
crates/amun-replay/src/validation.rs:24:            replayed_root: actual,
crates/amun-replay/src/validation.rs:26:            commits_checked: count,
crates/amun-replay/src/validation.rs:31:pub struct ReplayValidator;
crates/amun-replay/src/validation.rs:33:impl ReplayValidator {
crates/amun-replay/src/validation.rs:34:    pub fn validate(commits: &[StateCommit]) -> ReplayResult {
crates/amun-replay/src/validation.rs:35:        if commits.is_empty() {
crates/amun-replay/src/validation.rs:36:            return ReplayResult::success([0u8; 32], 0);
crates/amun-replay/src/validation.rs:38:        for i in 1..commits.len() {
crates/amun-replay/src/validation.rs:39:            let prev = &commits[i - 1];
crates/amun-replay/src/validation.rs:40:            let curr = &commits[i];
crates/amun-replay/src/validation.rs:42:                return ReplayResult::failure(prev.new_root, curr.previous_root, i);
crates/amun-replay/src/validation.rs:45:        let last = commits
crates/amun-replay/src/validation.rs:48:        ReplayResult::success(last.new_root, commits.len())
crates/amun-replay/src/validation.rs:4:pub struct ReplayResult {
crates/amun-replay/src/validation.rs:51:    pub fn validate_log(log: &CommitLog) -> ReplayResult {
crates/amun-replay/src/validation.rs:52:        Self::validate(&log.commits)
crates/amun-replay/src/validation.rs:60:    fn make_commit(h: u64, prev: [u8; 32], new: [u8; 32]) -> StateCommit {
crates/amun-replay/src/validation.rs:6:    pub replayed_root: [u8; 32],
crates/amun-replay/src/validation.rs:72:    fn n35_valid_chain() {
crates/amun-replay/src/validation.rs:73:        let commits = vec![
crates/amun-replay/src/validation.rs:74:            make_commit(1, [0u8; 32], [10u8; 32]),
crates/amun-replay/src/validation.rs:75:            make_commit(2, [10u8; 32], [20u8; 32]),
crates/amun-replay/src/validation.rs:76:            make_commit(3, [20u8; 32], [30u8; 32]),
crates/amun-replay/src/validation.rs:78:        let result = ReplayValidator::validate(&commits);
crates/amun-replay/src/validation.rs:80:        assert_eq!(result.commits_checked, 3);
crates/amun-replay/src/validation.rs:84:    fn n35_broken_chain() {
crates/amun-replay/src/validation.rs:85:        let commits = vec![
crates/amun-replay/src/validation.rs:86:            make_commit(1, [0u8; 32], [10u8; 32]),
crates/amun-replay/src/validation.rs:87:            make_commit(2, [99u8; 32], [20u8; 32]),
crates/amun-replay/src/validation.rs:89:        let result = ReplayValidator::validate(&commits);
crates/amun-replay/src/validation.rs:8:    pub commits_checked: usize,
crates/amun-replay/src/validation.rs:91:        assert_eq!(result.commits_checked, 1);
crates/amun-replay/src/validation.rs:95:    fn n35_single_commit() {
crates/amun-replay/src/validation.rs:96:        let commits = vec![make_commit(1, [0u8; 32], [10u8; 32])];
crates/amun-replay/src/validation.rs:97:        let result = ReplayValidator::validate(&commits);
crates/amun-replay/src/validation.rs:99:        assert_eq!(result.commits_checked, 1);
crates/amun-resource-core/tests/n130_constitutional_nft.rs:114:fn n130_replay_determinism() {
crates/amun-resource-core/tests/n130_constitutional_nft.rs:115:    let mut reg1 = ResourceRegistry::new(10);
crates/amun-resource-core/tests/n130_constitutional_nft.rs:116:    let mut reg2 = ResourceRegistry::new(10);
crates/amun-resource-core/tests/n130_constitutional_nft.rs:119:    let setup = |reg: &mut ResourceRegistry| {
crates/amun-resource-core/tests/n130_constitutional_nft.rs:124:            lineage: ResourceLineage::genesis(col_id),
crates/amun-resource-core/tests/n130_constitutional_nft.rs:142:                    lineage: ResourceLineage::single_ancestor(id, col_id, hash, version + 1),
crates/amun-resource-core/tests/n130_constitutional_nft.rs:153:    assert_eq!(reg1.compute_state_root(), reg2.compute_state_root());
crates/amun-resource-core/tests/n130_constitutional_nft.rs:16:        lineage: ResourceLineage::genesis(col_id),
crates/amun-resource-core/tests/n130_constitutional_nft.rs:2:    RegistryError, ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata,
crates/amun-resource-core/tests/n130_constitutional_nft.rs:31:        lineage: ResourceLineage::single_ancestor(token1, col_id, parent_hash, parent_version + 1),
crates/amun-resource-core/tests/n130_constitutional_nft.rs:3:    ResourceRegistry, ResourceState,
crates/amun-resource-core/tests/n130_constitutional_nft.rs:44:fn n130_invalid_parent_hash_rejected() {
crates/amun-resource-core/tests/n130_constitutional_nft.rs:45:    let mut reg = ResourceRegistry::new(10);
crates/amun-resource-core/tests/n130_constitutional_nft.rs:51:        lineage: ResourceLineage::genesis(col_id),
crates/amun-resource-core/tests/n130_constitutional_nft.rs:62:        lineage: ResourceLineage::single_ancestor(ResourceId([3u8; 32]), col_id, bad_hash, 2),
crates/amun-resource-core/tests/n130_constitutional_nft.rs:73:fn n130_collection_remains_active() {
crates/amun-resource-core/tests/n130_constitutional_nft.rs:74:    let mut reg = ResourceRegistry::new(10);
crates/amun-resource-core/tests/n130_constitutional_nft.rs:7:fn n130_duplicate_token_rejected() {
crates/amun-resource-core/tests/n130_constitutional_nft.rs:80:        lineage: ResourceLineage::genesis(col_id),
crates/amun-resource-core/tests/n130_constitutional_nft.rs:8:    let mut reg = ResourceRegistry::new(10);
crates/amun-resource-core/tests/n130_constitutional_nft.rs:96:            lineage: ResourceLineage::single_ancestor(
crates/amun-resource-core/tests/stress_tests.rs:107:                ResourceRegistry::hash_resource(parent),
crates/amun-resource-core/tests/stress_tests.rs:115:            lineage: ResourceLineage::single_ancestor(child_id, parent_id, parent_hash, version),
crates/amun-resource-core/tests/stress_tests.rs:132:fn stress_004_state_root_10k() {
crates/amun-resource-core/tests/stress_tests.rs:133:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-resource-core/tests/stress_tests.rs:13:fn make_genesis(id: ResourceId, archetype: ResourceArchetype) -> ResourceMetadata {
crates/amun-resource-core/tests/stress_tests.rs:141:    let root = reg.compute_state_root();
crates/amun-resource-core/tests/stress_tests.rs:148:fn stress_005_lookup_under_load() {
crates/amun-resource-core/tests/stress_tests.rs:149:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-resource-core/tests/stress_tests.rs:173:fn stress_006_parent_verification_under_load() {
crates/amun-resource-core/tests/stress_tests.rs:174:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-resource-core/tests/stress_tests.rs:186:                ResourceRegistry::hash_resource(parent),
crates/amun-resource-core/tests/stress_tests.rs:18:        lineage: ResourceLineage::genesis(id),
crates/amun-resource-core/tests/stress_tests.rs:194:            lineage: ResourceLineage::single_ancestor(child_id, parent_id, parent_hash, version),
crates/amun-resource-core/tests/stress_tests.rs:211:        let parent_hash = ResourceRegistry::hash_resource(parent);
crates/amun-resource-core/tests/stress_tests.rs:221:fn stress_007_cycle_detection_at_depth() {
crates/amun-resource-core/tests/stress_tests.rs:222:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-resource-core/tests/stress_tests.rs:234:                ResourceRegistry::hash_resource(parent),
crates/amun-resource-core/tests/stress_tests.rs:242:            lineage: ResourceLineage::single_ancestor(child_id, parent_id, parent_hash, version),
crates/amun-resource-core/tests/stress_tests.rs:254:            ResourceRegistry::hash_resource(tip),
crates/amun-resource-core/tests/stress_tests.rs:25:fn stress_001_10k_genesis_resources() {
crates/amun-resource-core/tests/stress_tests.rs:262:        lineage: ResourceLineage::single_ancestor(make_id(99999), parent_id, tip_hash, version),
crates/amun-resource-core/tests/stress_tests.rs:26:    let mut reg = ResourceRegistry::new(100_000);
crates/amun-resource-core/tests/stress_tests.rs:2:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-resource-core/tests/stress_tests.rs:44:fn stress_002_deep_lineage_chain() {
crates/amun-resource-core/tests/stress_tests.rs:45:    let mut reg = ResourceRegistry::new(10_000);
crates/amun-resource-core/tests/stress_tests.rs:58:                ResourceRegistry::hash_resource(parent),
crates/amun-resource-core/tests/stress_tests.rs:66:            lineage: ResourceLineage::single_ancestor(child_id, parent_id, parent_hash, version),
crates/amun-resource-core/tests/stress_tests.rs:7:fn make_id(seed: u64) -> ResourceId {
crates/amun-resource-core/tests/stress_tests.rs:84:fn stress_003_wide_fanout() {
crates/amun-resource-core/tests/stress_tests.rs:89:    let mut reg = ResourceRegistry::new(100_000);
crates/amun-runtime-benchmarks/benches/runtime_benchmarks.rs:2:fn main() {}
crates/amun-sdk-layer/src/tests.rs:11:    fn test_governance_api_create_proposal() { let mut api = GovernanceApi::new(); let proposer = amun_kernel_types::PublicHash32::new([1u8; 32]); let result = api.create_proposal(proposer, amun_governance::proposal::ProposalType::Text, 1000); assert!(result.success); }
crates/amun-sdk-layer/src/tests.rs:13:    fn test_charity_api_donate() { let mut api = CharityApi::new(); let recipient = amun_kernel_types::PublicHash32::new([1u8; 32]); let result = api.donate(recipient, 100); assert!(result.success); }
crates/amun-sdk-layer/src/tests.rs:15:    fn test_sandbox_simulation() { let mut sandbox = Sandbox::new(); let a0 = sandbox.create_account(1_000_000).data.expect("test invariant"); let a1 = sandbox.create_account(500_000).data.expect("test invariant"); let result = sandbox.simulate_transfer(a0, a1, 100_000); assert!(result.success); }
crates/amun-sdk-layer/src/tests.rs:17:    fn test_transaction_builder_transfer() { let sender = amun_kernel_types::PublicKey::new([1u8; 48]); let recipient = amun_kernel_types::PublicHash32::new([2u8; 32]); let result = TransactionBuilder::build_transfer(42, 0, sender, recipient, 100, 1000); assert!(result.success); }
crates/amun-sdk-layer/src/tests.rs:5:    fn test_token_api_create() { let result = TokenApi::create_account(1_000_000); assert!(result.success); }
crates/amun-sdk-layer/src/tests.rs:7:    fn test_token_api_transfer() { let mut token = TokenApi::create_account(1_000_000).data.expect("test invariant"); let result = TokenApi::transfer(&mut token, 100_000); assert!(result.success); }
crates/amun-sdk-layer/src/tests.rs:9:    fn test_staking_api_register() { let mut api = StakingApi::new(); let pk = amun_kernel_types::PublicKey::new([1u8; 48]); let result = api.register_validator(pk, 1_000_000); assert!(result.success); }
crates/amun-self-preservation/src/legitimacy_guards.rs:14:    ReplayDeterminism,
crates/amun-smt/tests/fuzz.rs:80:        prop_assert!(proof.verify(&root.0).unwrap());
crates/amun-smt/tests/proofs.rs:24:    assert!(proof.verify(&root.0).unwrap(), "Proof must verify");
crates/amun-smt/tests/proofs.rs:41:    assert!(proof.verify(&root.0).unwrap(), "Absence proof must verify");
crates/amun-snapshot-engine-unified/src/lib.rs:105:    fn n43_snapshot_restores_height() {
crates/amun-snapshot-engine-unified/src/lib.rs:127:    fn n43_snapshot_restores_evidence() {
crates/amun-snapshot-engine-unified/src/lib.rs:149:    fn n43_version_validation() {
crates/amun-snapshot-engine-unified/src/lib.rs:85:    fn n43_snapshot_roundtrip() {
crates/amun-snapshot-engine/src/compatibility.rs:123:            CompatibilityLevel::ReplayCompatible
crates/amun-snapshot-engine/src/compatibility.rs:12:    /// State replay is identical, proofs are identical
crates/amun-snapshot-engine/src/compatibility.rs:13:    ReplayCompatible,
crates/amun-snapshot-engine/src/compatibility.rs:145:            CompatibilityLevel::FullyCompatible | CompatibilityLevel::ReplayCompatible
crates/amun-snapshot-engine/src/compatibility.rs:155:                | CompatibilityLevel::ReplayCompatible
crates/amun-snapshot-engine/src/compatibility.rs:166:                | CompatibilityLevel::ReplayCompatible
crates/amun-snapshot-engine/src/constitutional_hash.rs:17:        replay_law_text: &str,
crates/amun-snapshot-engine/src/constitutional_hash.rs:25:        h.update(replay_law_text.as_bytes());
crates/amun-snapshot-engine/src/constitutional_hash.rs:38:        replay_law_text: &str,
crates/amun-snapshot-engine/src/constitutional_hash.rs:46:            replay_law_text,
crates/amun-snapshot-engine/src/lib.rs:14:pub mod replay_continuity;
crates/amun-snapshot-engine/src/lib.rs:31:pub use replay_continuity::{ContinuityResult, ReplayContinuityEngine};
crates/amun-snapshot-engine/src/replay_continuity.rs:10:pub struct ReplayContinuityEngine;
crates/amun-snapshot-engine/src/replay_continuity.rs:13:pub enum ContinuityResult {
crates/amun-snapshot-engine/src/replay_continuity.rs:17:        replayed_frames: u64,
crates/amun-snapshot-engine/src/replay_continuity.rs:1:// Replay Continuity Engine
crates/amun-snapshot-engine/src/replay_continuity.rs:27:impl ReplayContinuityEngine {
crates/amun-snapshot-engine/src/replay_continuity.rs:28:    /// THEOREM 11: Verify that restoring a snapshot and replaying the
crates/amun-snapshot-engine/src/replay_continuity.rs:2:// THEOREM 11: state -> snapshot -> restore -> WAL replay -> final_root
crates/amun-snapshot-engine/src/replay_continuity.rs:30:    pub fn verify_continuity(
crates/amun-snapshot-engine/src/replay_continuity.rs:36:        let mut tree = SnapshotRestoreEngine::restore(snapshot_chunks)
crates/amun-snapshot-engine/src/replay_continuity.rs:41:        // Step 2: Replay WAL from snapshot checkpoint forward
crates/amun-snapshot-engine/src/replay_continuity.rs:43:        let mut replayed_frames: u64 = 0;
crates/amun-snapshot-engine/src/replay_continuity.rs:65:                replayed_frames += 1;
crates/amun-snapshot-engine/src/replay_continuity.rs:6:use super::restore::SnapshotRestoreEngine;
crates/amun-snapshot-engine/src/replay_continuity.rs:74:                at_frame: replayed_frames,
crates/amun-snapshot-engine/src/replay_continuity.rs:82:            replayed_frames,
crates/amun-snapshot-engine/src/replay_continuity.rs:87:    pub fn verify_roundtrip(chunks: &ChunkIndex, expected_root: [u8; 32]) -> Result<bool, String> {
crates/amun-snapshot-engine/src/replay_continuity.rs:89:            SnapshotRestoreEngine::restore(chunks).map_err(|e| format!("Restore failed: {}", e))?;
crates/amun-snapshot-engine/src/transition.rs:30:            CompatibilityLevel::ReplayCompatible | CompatibilityLevel::SnapshotCompatible => {
crates/amun-snapshot-engine/src/verifier.rs:3:// canonical empty root, constitutional hash, and replay equivalence.
crates/amun-snapshot-engine/src/verifier.rs:61:    /// state -> snapshot -> restore -> replay -> same root
crates/amun-snapshot-engine/tests/constitutional_tests.rs:101:            "replay",
crates/amun-snapshot-engine/tests/constitutional_tests.rs:109:            "replay",
crates/amun-snapshot-engine/tests/constitutional_tests.rs:123:        assert!(decoded.verify());
crates/amun-snapshot-engine/tests/constitutional_tests.rs:134:        assert!(!decoded.verify());
crates/amun-snapshot-engine/tests/constitutional_tests.rs:28:        assert!(id.verify());
crates/amun-snapshot-engine/tests/constitutional_tests.rs:59:        assert!(CompatibilityEngine::can_sync(&id, &id));
crates/amun-snapshot-engine/tests/constitutional_tests.rs:66:        assert!(matches!(rel, ConstitutionalRelationship::Identical));
crates/amun-snapshot-engine/tests/constitutional_tests.rs:80:            "replay",
crates/amun-snapshot-engine/tests/constitutional_tests.rs:88:            "replay",
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:10:fn n162_incremental_snapshot_faster_than_full() {
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:11:    let mut reg = ResourceRegistry::new(10000);
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:17:        lineage: ResourceLineage::genesis(col_id),
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:2:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:33:                lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:41:    let base_root = reg.compute_state_root();
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:68:fn n162_restore_from_compressed_is_fast() {
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:69:    let mut reg = ResourceRegistry::new(1000);
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:75:        lineage: ResourceLineage::genesis(col_id),
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:91:                lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
crates/amun-soak-full/tests/n165_full_soak_tests.rs:23:fn n165_full_soak_60s_with_adversarial() {
crates/amun-soak-full/tests/n165_full_soak_tests.rs:43:fn n165_state_consistency_under_full_load() {
crates/amun-soak-full/tests/n165_full_soak_tests.rs:4:fn n165_full_soak_30s() {
crates/amun-soak-test/src/lib.rs:174:    pub fn passed(&self) -> bool {
crates/amun-soak-test/tests/n165_soak_tests.rs:20:fn n165_soak_60_seconds_with_events() {
crates/amun-soak-test/tests/n165_soak_tests.rs:36:fn n165_state_consistency_under_load() {
crates/amun-soak-test/tests/n165_soak_tests.rs:4:fn n165_soak_30_seconds_no_events() {
crates/amun-state-machine/src/absolute_invariants.rs:13:    /// Replay determinism must be preserved across ALL amendments
crates/amun-state-machine/src/absolute_invariants.rs:14:    ReplayDeterminismAbsolute,
crates/amun-state-machine/src/absolute_invariants.rs:41:            Self::ReplayDeterminismAbsolute,
crates/amun-state-machine/src/axioms.rs:12:    /// Replay determinism is preserved across all legal transitions
crates/amun-state-machine/src/axioms.rs:13:    ReplayDeterminismPreserved,
crates/amun-state-machine/src/axioms.rs:26:    /// Hostile forks cannot preserve replay
crates/amun-state-machine/src/axioms.rs:27:    HostileForkReplayImpossible,
crates/amun-state-machine/src/axioms.rs:38:            Self::ReplayDeterminismPreserved => {
crates/amun-state-machine/src/axioms.rs:39:                "Replay must remain deterministic across all transitions"
crates/amun-state-machine/src/axioms.rs:47:            Self::HostileForkReplayImpossible => "Hostile forks cannot preserve replay",
crates/amun-state-machine/src/delta_algebra.rs:11:    ReplayDelta {
crates/amun-state-machine/src/delta_algebra.rs:43:            Self::ReplayDelta { .. } => 0x02,
crates/amun-state-machine/src/delta_algebra.rs:57:            Self::ReplayDelta { .. } => 3,
crates/amun-state-machine/src/delta_algebra.rs:77:            Self::ReplayDelta {
crates/amun-state-machine/src/delta_laws.rs:33:                ConstitutionalDelta::ReplayDelta {
crates/amun-state-machine/src/delta_laws.rs:37:                ConstitutionalDelta::ReplayDelta {
crates/amun-state-machine/src/engine.rs:109:        // Append to replay log
crates/amun-state-machine/src/engine.rs:110:        self.replay_log
crates/amun-state-machine/src/engine.rs:25:    pub replay_log: ConstitutionalReplayDAG,
crates/amun-state-machine/src/engine.rs:38:            replay_log: ConstitutionalReplayDAG::new(),
crates/amun-state-machine/src/engine.rs:4:use super::replay_log::ConstitutionalReplayDAG;
crates/amun-state-machine/src/fork_merge.rs:107:        replay_unified: bool,
crates/amun-state-machine/src/fork_merge.rs:117:            replay_unified,
crates/amun-state-machine/src/fork_merge.rs:11:    pub preserves_replay: bool,
crates/amun-state-machine/src/fork_merge.rs:136:        h.update(&[self.replay_unified as u8]);
crates/amun-state-machine/src/fork_merge.rs:35:        preserves_replay: bool,
crates/amun-state-machine/src/fork_merge.rs:44:            preserves_replay,
crates/amun-state-machine/src/fork_merge.rs:59:        h.update(&[self.preserves_replay as u8]);
crates/amun-state-machine/src/fork_merge.rs:83:    pub replay_unified: bool,
crates/amun-state-machine/src/formal_entropy.rs:27:    /// Replay convergence absorbs entropy
crates/amun-state-machine/src/formal_entropy.rs:28:    ReplayConvergence,
crates/amun-state-machine/src/historical_invariants.rs:108:    pub fn record_replay_divergence(&mut self, bytes: u64) {
crates/amun-state-machine/src/historical_invariants.rs:109:        self.cumulative_replay_divergence += bytes;
crates/amun-state-machine/src/historical_invariants.rs:133:            HistoricalInvariant::MaxReplayDivergence {
crates/amun-state-machine/src/historical_invariants.rs:135:                current: self.cumulative_replay_divergence,
crates/amun-state-machine/src/historical_invariants.rs:14:    /// Replay divergence must not accumulate over lineage
crates/amun-state-machine/src/historical_invariants.rs:15:    MaxReplayDivergence {
crates/amun-state-machine/src/historical_invariants.rs:48:            HistoricalInvariant::MaxReplayDivergence {
crates/amun-state-machine/src/historical_invariants.rs:54:                        "Replay divergence accumulation exceeded: {} > {}",
crates/amun-state-machine/src/historical_invariants.rs:80:    pub cumulative_replay_divergence: u64,
crates/amun-state-machine/src/historical_invariants.rs:96:            cumulative_replay_divergence: 0,
crates/amun-state-machine/src/impossibility.rs:10:    HostileForkReplayPreservationImpossible,
crates/amun-state-machine/src/impossibility.rs:9:    /// Hostile forks cannot preserve replay
crates/amun-state-machine/src/invariants.rs:10:    /// Replay determinism must be preserved.
crates/amun-state-machine/src/invariants.rs:11:    ReplayDeterminismPreserved,
crates/amun-state-machine/src/lib.rs:22:pub mod replay_log;
crates/amun-state-machine/src/lib.rs:50:pub use replay_log::{ConstitutionalReplayDAG, ReplayLogEntry};
crates/amun-state-machine/src/merge_math.rs:10:    /// Replay histories are interleaved by epoch
crates/amun-state-machine/src/merge_math.rs:11:    ReplayEpochInterleave,
crates/amun-state-machine/src/merge_math.rs:27:    /// Resolve replay conflict between merging civilizations.
crates/amun-state-machine/src/merge_math.rs:28:    pub fn resolve_replay(
crates/amun-state-machine/src/merge_math.rs:29:        a_replay_height: u64,
crates/amun-state-machine/src/merge_math.rs:30:        b_replay_height: u64,
crates/amun-state-machine/src/merge_math.rs:34:            super::fork_merge::MergeType::Union => MergeResolution::ReplayUnion,
crates/amun-state-machine/src/merge_math.rs:36:                if a_replay_height >= b_replay_height {
crates/amun-state-machine/src/merge_math.rs:37:                    MergeResolution::ReplayLongestChainWins
crates/amun-state-machine/src/merge_math.rs:39:                    MergeResolution::ReplayLongestChainWins
crates/amun-state-machine/src/merge_math.rs:42:            super::fork_merge::MergeType::Federation => MergeResolution::ReplayEpochInterleave,
crates/amun-state-machine/src/merge_math.rs:6:    /// The merged replay contains all entries from both parents (union)
crates/amun-state-machine/src/merge_math.rs:7:    ReplayUnion,
crates/amun-state-machine/src/merge_math.rs:8:    /// The longer replay chain survives (winner-takes-all)
crates/amun-state-machine/src/merge_math.rs:9:    ReplayLongestChainWins,
crates/amun-state-machine/src/meta_amendment.rs:22:    /// Can amend replay guarantees
crates/amun-state-machine/src/meta_amendment.rs:23:    ReplayGuarantees,
crates/amun-state-machine/src/meta_amendment.rs:53:            MetaAmendmentScope::ProofSemantics | MetaAmendmentScope::ReplayGuarantees => {
crates/amun-state-machine/src/meta_amendment.rs:57:                        AbsoluteInvariant::ReplayDeterminismAbsolute => {
crates/amun-state-machine/src/meta_amendment.rs:58:                            if matches!(scope, MetaAmendmentScope::ReplayGuarantees) {
crates/amun-state-machine/src/meta_amendment.rs:60:                                    "Cannot amend replay guarantees: ReplayDeterminismAbsolute"
crates/amun-state-machine/src/preconditions.rs:15:    /// Replay continuity must be preserved.
crates/amun-state-machine/src/preconditions.rs:16:    ReplayContinuityPreserved,
crates/amun-state-machine/src/replay_log.rs:106:    fn dfs_acyclic(
crates/amun-state-machine/src/replay_log.rs:10:    pub transition_hash: [u8; 32],
crates/amun-state-machine/src/replay_log.rs:18:impl ReplayLogEntry {
crates/amun-state-machine/src/replay_log.rs:19:    pub fn new(sequence: u64, transition: &Transition, parent_entries: Vec<[u8; 32]>) -> Self {
crates/amun-state-machine/src/replay_log.rs:1:use super::transitions::Transition;
crates/amun-state-machine/src/replay_log.rs:22:            transition_hash: transition.transition_id,
crates/amun-state-machine/src/replay_log.rs:24:            epoch: transition.epoch,
crates/amun-state-machine/src/replay_log.rs:25:            generation: transition.generation,
crates/amun-state-machine/src/replay_log.rs:28:        e.entry_hash = e.compute_hash();
crates/amun-state-machine/src/replay_log.rs:32:    fn compute_hash(&self) -> [u8; 32] {
crates/amun-state-machine/src/replay_log.rs:35:        h.update(&self.transition_hash);
crates/amun-state-machine/src/replay_log.rs:42:        h.finalize()
crates/amun-state-machine/src/replay_log.rs:46:/// Constitutional Replay DAG - branching constitutional history.
crates/amun-state-machine/src/replay_log.rs:48:pub struct ConstitutionalReplayDAG {
crates/amun-state-machine/src/replay_log.rs:49:    pub entries: HashMap<[u8; 32], ReplayLogEntry>,
crates/amun-state-machine/src/replay_log.rs:54:impl Default for ConstitutionalReplayDAG {
crates/amun-state-machine/src/replay_log.rs:55:    fn default() -> Self {
crates/amun-state-machine/src/replay_log.rs:5:/// A node in the constitutional replay DAG.
crates/amun-state-machine/src/replay_log.rs:60:impl ConstitutionalReplayDAG {
crates/amun-state-machine/src/replay_log.rs:61:    pub fn new() -> Self {
crates/amun-state-machine/src/replay_log.rs:69:    pub fn append(&mut self, transition: &Transition, parents: Vec<[u8; 32]>) -> ReplayLogEntry {
crates/amun-state-machine/src/replay_log.rs:70:        let entry = ReplayLogEntry::new(self.next_sequence, transition, parents.clone());
crates/amun-state-machine/src/replay_log.rs:82:    pub fn verify_dag(&self) -> bool {
crates/amun-state-machine/src/replay_log.rs:8:pub struct ReplayLogEntry {
crates/amun-state-machine/src/replay_log.rs:95:    fn is_acyclic(&self) -> bool {
crates/amun-state-machine/src/stability.rs:26:            replay_integrity_score: 1.0,
crates/amun-state-machine/src/stability.rs:6:    pub replay_integrity_score: f64,
crates/amun-state-machine/src/thermodynamics.rs:2:/// Every fork, amendment, and replay divergence increases entropy.
crates/amun-state-machine/src/thermodynamics.rs:37:    /// Each replay divergence increases entropy.
crates/amun-state-machine/src/thermodynamics.rs:38:    pub fn record_replay_divergence(&mut self, bytes: u64) {
crates/amun-state-machine/src/thermodynamics.rs:3:/// Every merge, freeze, and replay convergence decreases it.
crates/amun-state-pruning/tests/n166_pruning_tests.rs:109:fn n166_pruned_root_deterministic() {
crates/amun-state-pruning/tests/n166_pruning_tests.rs:120:                lineage: ResourceLineage::genesis(col_id),
crates/amun-state-pruning/tests/n166_pruning_tests.rs:137:                        lineage: ResourceLineage::single_ancestor(
crates/amun-state-pruning/tests/n166_pruning_tests.rs:152:    assert_eq!(pr1.compute_pruned_root(), pr2.compute_pruned_root());
crates/amun-state-pruning/tests/n166_pruning_tests.rs:16:            lineage: ResourceLineage::genesis(col_id),
crates/amun-state-pruning/tests/n166_pruning_tests.rs:2:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
crates/amun-state-pruning/tests/n166_pruning_tests.rs:33:                    lineage: ResourceLineage::single_ancestor(
crates/amun-state-pruning/tests/n166_pruning_tests.rs:58:fn n166_restore_archived_brings_back_resources() {
crates/amun-state-pruning/tests/n166_pruning_tests.rs:66:            lineage: ResourceLineage::genesis(col_id),
crates/amun-state-pruning/tests/n166_pruning_tests.rs:83:                    lineage: ResourceLineage::single_ancestor(
crates/amun-state-pruning/tests/n166_pruning_tests.rs:8:fn n166_prune_by_height_reduces_active_set() {
crates/amun-state-root/src/laws.rs:1:pub const STATE_ROOT_LAW: &str = "assert_eq!(recomputed_root, committed_root)";
crates/amun-state-root/src/laws.rs:2:pub const REPLAY_EQUIVALENCE_LAW: &str = "assert_eq!(live_execution_hash, replay_hash)";
crates/amun-state-root/src/lib.rs:13:pub use replay::{ReplayCertificate, ReplayEquivalenceProof, ReplayTranscript};
crates/amun-state-root/src/lib.rs:6:pub mod replay;
crates/amun-state-root/src/replay.rs:10:impl CanonicalEncode for ReplayEquivalenceProof {
crates/amun-state-root/src/replay.rs:11:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-state-root/src/replay.rs:13:        out.extend_from_slice(&self.replayed_root);
crates/amun-state-root/src/replay.rs:19:pub struct ReplayTranscript {
crates/amun-state-root/src/replay.rs:29:impl CanonicalEncode for ReplayTranscript {
crates/amun-state-root/src/replay.rs:30:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-state-root/src/replay.rs:42:pub struct ReplayCertificate {
crates/amun-state-root/src/replay.rs:43:    pub transcript: ReplayTranscript,
crates/amun-state-root/src/replay.rs:44:    pub proof: ReplayEquivalenceProof,
crates/amun-state-root/src/replay.rs:47:impl CanonicalEncode for ReplayCertificate {
crates/amun-state-root/src/replay.rs:48:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-state-root/src/replay.rs:4:pub struct ReplayEquivalenceProof {
crates/amun-state-root/src/replay.rs:6:    pub replayed_root: [u8; 32],
crates/amun-state-root/src/snapshot.rs:17:    /// Replay equivalence proof.
crates/amun-state-root/src/snapshot.rs:18:    pub replay_equivalence_proof: ReplayEquivalenceProof,
crates/amun-state-root/src/snapshot.rs:22:/// Proof that live and replayed executions produced the same state root.
crates/amun-state-root/src/snapshot.rs:24:pub struct ReplayEquivalenceProof {
crates/amun-state-root/src/snapshot.rs:29:    pub replayed_root: [u8; 32],
crates/amun-state-root/src/snapshot.rs:32:impl ReplayEquivalenceProof {
crates/amun-state-root/src/snapshot.rs:36:        self.live_root == self.replayed_root
crates/amun-state-root/src/snapshot.rs:40:impl CanonicalEncode for ReplayEquivalenceProof {
crates/amun-state-root/src/snapshot.rs:43:        out.extend_from_slice(&self.replayed_root);
crates/amun-state-root/src/snapshot.rs:57:        self.replay_equivalence_proof.encode_canonical(out);
crates/amun-state-root/src/snapshot.rs:64:    /// Uses `debug_assert!` to verify integrity in debug/test builds
crates/amun-state-root/src/snapshot.rs:6:/// A snapshot carrying replay-verifiable constitutional truth.
crates/amun-state-root/src/snapshot.rs:77:            && self.replay_equivalence_proof.verify()
crates/amun-state-root/src/verifier.rs:3:/// remains serializable and replayable without coupling.
crates/amun-state-sync/src/lib.rs:101:    fn n56_chunk_verify() {
crates/amun-state-sync/src/lib.rs:104:        assert!(chunk.verify());
crates/amun-state-sync/src/lib.rs:108:    fn n56_chunk_detects_tampering() {
crates/amun-state-sync/src/lib.rs:112:        assert!(!chunk.verify());
crates/amun-state-sync/src/lib.rs:116:    fn n56_chunk_merkle_tree_proofs() {
crates/amun-state-sync/src/lib.rs:124:            assert!(proof.verify());
crates/amun-state-sync/src/lib.rs:130:    fn n56_full_sync_package_roundtrip() {
crates/amun-state-sync/src/lib.rs:166:        assert!(pkg.verify());
crates/amun-state-sync/src/lib.rs:185:    fn n56_reject_wrong_history_root() {
crates/amun-state-sync/src/lib.rs:203:    fn n56_reject_tampered_chunk() {
crates/amun-state-sync/src/lib.rs:237:    fn n56_reject_missing_chunk() {
crates/amun-state-sync/src/lib.rs:68:    fn n56_snapshot_certificate_verify() {
crates/amun-state-sync/src/lib.rs:80:        assert!(cert.verify());
crates/amun-state-sync/src/lib.rs:84:    fn n56_snapshot_certificate_detects_tampering() {
crates/amun-state-sync/src/lib.rs:97:        assert!(!cert.verify());
crates/amun-state-sync/src/sync_protocol.rs:266:    fn n65_snapshot_create_and_import() {
crates/amun-state-sync/src/sync_protocol.rs:273:        assert_eq!(imported.compute_state_root(), root);
crates/amun-state-sync/src/sync_protocol.rs:277:    fn n65_snapshot_chunked_large_state() {
crates/amun-state-sync/src/sync_protocol.rs:283:        assert_eq!(imported.compute_state_root(), root);
crates/amun-state-sync/src/sync_protocol.rs:287:    fn n65_tampered_chunk_rejected() {
crates/amun-state-sync/src/sync_protocol.rs:297:    fn n65_wrong_history_root_rejected() {
crates/amun-state-sync/src/sync_protocol.rs:304:    fn n65_sync_request_full_snapshot() {
crates/amun-state-sync/src/sync_protocol.rs:319:    fn n65_sync_request_delta() {
crates/amun-state-sync/src/sync_protocol.rs:336:    fn n65_already_synced() {
crates/amun-state-sync/src/sync_protocol.rs:351:    fn n65_delta_apply() {
crates/amun-state-sync/src/sync_protocol.rs:379:        assert_eq!(reg.compute_state_root(), root);
crates/amun-state-sync/src/sync_protocol.rs:384:// to avoid this. N65.1 will fix compute_merkle_siblings to handle odd leaves.
crates/amun-state-types/src/tests.rs:48:    assert_eq!(*committed.inner(), 42);
crates/amun-state-types/src/tests.rs:62:    assert_eq!(*finalized.inner(), 42);
crates/amun-state-types/src/tests.rs:76:    assert_eq!(finalized.into_inner(), 42);
crates/amun-stateless-sync/src/lib.rs:105:    pub fn import_certificate(&mut self, cert: ReplayCertificate) {
crates/amun-stateless-sync/src/lib.rs:220:    fn n11a_header_sync() {
crates/amun-stateless-sync/src/lib.rs:232:    fn n11b_certificate_sync() {
crates/amun-stateless-sync/src/lib.rs:246:    fn n11c_proof_bundle_sync() {
crates/amun-stateless-sync/src/lib.rs:252:            ProofBundleSyncMessage::BundleResponse { bundle: b } => assert!(b.verify().is_ok()),
crates/amun-stateless-sync/src/lib.rs:258:    fn n11d_stateless_node_import() {
crates/amun-stateless-sync/src/lib.rs:268:        assert!(node.verify_height(0).is_ok());
crates/amun-stateless-sync/src/lib.rs:272:    fn n11d_stateless_node_verify_chain() {
crates/amun-stateless-sync/src/lib.rs:285:        assert!(node.verify_chain().is_ok());
crates/amun-stateless-sync/src/lib.rs:289:    fn n11e_trustless_bootstrap() {
crates/amun-stateless-sync/src/lib.rs:296:        assert!(node.verify_chain().is_ok());
crates/amun-stateless-sync/src/lib.rs:300:    fn n11e_tampered_bundle_rejected() {
crates/amun-stateless-sync/src/lib.rs:309:    fn n11_missing_header_detected() {
crates/amun-stateless-sync/src/lib.rs:311:        assert!(node.verify_height(5).is_err());
crates/amun-stateless-sync/src/lib.rs:34:use amun_constitutional_state::{CertificateInclusionProof, ReplayCertificate};
crates/amun-stateless-sync/src/lib.rs:59:        certificates: Vec<ReplayCertificate>,
crates/amun-stateless-sync/src/lib.rs:64:// N11C: Proof Bundle Sync Protocol
crates/amun-stateless-sync/src/lib.rs:80:    certificates: BTreeMap<[u8; 32], ReplayCertificate>,
crates/amun-stf/src/nonce.rs:39:                ConstitutionalFault::ReplayViolation,
crates/amun-stf/src/tests.rs:55:fn test_nonce_rejects_replay() {}
crates/amun-stf/tests/integration_test.rs:27:    assert!(receipt.verify_consistency().is_ok());
crates/amun-stf/tests/integration_test.rs:94:    assert!(transcript.verify_transcript().is_ok());
crates/amun-stf/tests/replay_equivalence.rs:112:        .expect("corrupted replay must still execute");
crates/amun-stf/tests/replay_equivalence.rs:116:        replay_transcript.add_receipt(receipt);
crates/amun-stf/tests/replay_equivalence.rs:119:    let replay_root = replay_transcript
crates/amun-stf/tests/replay_equivalence.rs:11:fn execute_block(txs: &[[u8; 32]]) -> (ExecutionTranscript, Vec<u8>, [u8; 32]) {
crates/amun-stf/tests/replay_equivalence.rs:122:        .expect("replay transcript must not be empty")
crates/amun-stf/tests/replay_equivalence.rs:126:        live_root, replay_root,
crates/amun-stf/tests/replay_equivalence.rs:127:        "divergent replay unexpectedly matched"
crates/amun-stf/tests/replay_equivalence.rs:135:    let replay_hashes: Vec<[u8; 32]> = replay_transcript
crates/amun-stf/tests/replay_equivalence.rs:141:        live_hashes, replay_hashes,
crates/amun-stf/tests/replay_equivalence.rs:142:        "divergent replay produced identical receipts"
crates/amun-stf/tests/replay_equivalence.rs:17:        let receipt = execute_transition_with_receipt(
crates/amun-stf/tests/replay_equivalence.rs:48:fn test_single_block_replay_equivalence() {
crates/amun-stf/tests/replay_equivalence.rs:51:    let (live_transcript, live_state, live_root) = execute_block(&txs);
crates/amun-stf/tests/replay_equivalence.rs:52:    let (replay_transcript, replay_state, replay_root) = execute_block(&txs);
crates/amun-stf/tests/replay_equivalence.rs:54:    assert_eq!(live_root, replay_root, "live and replay roots diverged");
crates/amun-stf/tests/replay_equivalence.rs:55:    assert_eq!(live_state, replay_state, "live and replay state diverged");
crates/amun-stf/tests/replay_equivalence.rs:58:        replay_transcript.receipts.len(),
crates/amun-stf/tests/replay_equivalence.rs:5:use amun_stf::transition_result::{execute_transition_with_receipt, TransitionExecutionResult};
crates/amun-stf/tests/replay_equivalence.rs:62:    for (index, (live, replay)) in live_transcript
crates/amun-stf/tests/replay_equivalence.rs:65:        .zip(replay_transcript.receipts.iter())
crates/amun-stf/tests/replay_equivalence.rs:70:            replay.receipt_hash(),
crates/amun-stf/tests/replay_equivalence.rs:74:        assert_eq!(live.tx_hash, replay.tx_hash);
crates/amun-stf/tests/replay_equivalence.rs:75:        assert_eq!(live.pre_state_root, replay.pre_state_root);
crates/amun-stf/tests/replay_equivalence.rs:76:        assert_eq!(live.post_state_root, replay.post_state_root);
crates/amun-stf/tests/replay_equivalence.rs:77:        assert_eq!(live.execution_result_hash, replay.execution_result_hash);
crates/amun-stf/tests/replay_equivalence.rs:78:        assert_eq!(live.status, replay.status);
crates/amun-stf/tests/replay_equivalence.rs:79:        assert_eq!(live.state_changed, replay.state_changed);
crates/amun-stf/tests/replay_equivalence.rs:80:        assert_eq!(live.previous_receipt_hash, replay.previous_receipt_hash);
crates/amun-stf/tests/replay_equivalence.rs:83:    assert!(live_transcript.verify_transcript().is_ok());
crates/amun-stf/tests/replay_equivalence.rs:84:    assert!(replay_transcript.verify_transcript().is_ok());
crates/amun-stf/tests/replay_equivalence.rs:88:fn test_replay_detects_divergent_execution() {
crates/amun-stf/tests/replay_equivalence.rs:91:    let (live_transcript, _live_state, live_root) = execute_block(&txs);
crates/amun-stf/tests/replay_equivalence.rs:93:    let mut replay_transcript = ExecutionTranscript::new();
crates/amun-stf/tests/replay_equivalence.rs:98:        let receipt = execute_transition_with_receipt(
crates/amun-storage-kernel/ATOMIC_SNAPSHOT_CONSTITUTION.md:20:MUST match the state at the freeze point exactly. Replay of WAL from
crates/amun-storage-kernel/CONSTITUTION.md:65:**Section 3.2: Replay Equivalence**
crates/amun-storage-kernel/CONSTITUTION.md:66:Replaying a valid WAL from genesis SHALL produce the identical state root
crates/amun-storage-kernel/SNAPSHOT_CONSTITUTION.md:141:**Section 8.1: Snapshot + WAL Replay**
crates/amun-storage-kernel/VALIDITY_HIERARCHY.md:33:| StateRootMismatch | ReplayVerifier divergence | Quarantine state, resync from trusted peer |
crates/amun-storage-kernel/VALIDITY_HIERARCHY.md:45:| ReplayDivergence | Replay produces different root | Investigate, rebuild from trusted source |
crates/amun-storage-kernel/VALIDITY_HIERARCHY.md:82:| Local WAL intact | Replay locally |
crates/amun-storage-kernel/src/persistence/wal/iterator.rs:134:                        "replay divergence at {}: {:?} != {:?}",
crates/amun-storage-kernel/src/persistence/wal/iterator.rs:83:pub struct ReplayVerifier;
crates/amun-storage-kernel/src/persistence/wal/iterator.rs:84:impl ReplayVerifier {
crates/amun-storage-kernel/src/persistence/wal/iterator.rs:85:    pub fn verify_full_replay(wal_path: &str) -> Result<([u8; 32], u64), String> {
crates/amun-storage-kernel/src/persistence/wal/mod.rs:5:pub use iterator::{ReplayVerifier, WalIterator};
crates/amun-storage-kernel/tests/proptest_smt.rs:29:                    prop_assert!(proof.verify(root.0), "proof verification failed");
crates/amun-storage-kernel/tests/proptest_smt.rs:54:                    prop_assert!(proof.verify(root.0), "absence proof verification failed");
crates/amun-storage-kernel/tests/replay_equivalence.rs:100:    fn replay_detects_epoch_regression() {
crates/amun-storage-kernel/tests/replay_equivalence.rs:11:    fn create_test_wal(path: &str, entries: &[WalEntry]) -> std::io::Result<()> {
crates/amun-storage-kernel/tests/replay_equivalence.rs:136:        let result = ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap());
crates/amun-storage-kernel/tests/replay_equivalence.rs:28:    fn replay_equivalence_passes() {
crates/amun-storage-kernel/tests/replay_equivalence.rs:4:        persistence::wal::{ReplayVerifier, WalEntry},
crates/amun-storage-kernel/tests/replay_equivalence.rs:53:        let (replayed_root, count) =
crates/amun-storage-kernel/tests/replay_equivalence.rs:54:            ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap()).unwrap();
crates/amun-storage-kernel/tests/replay_equivalence.rs:57:        assert_eq!(replayed_root, root.0);
crates/amun-storage-kernel/tests/replay_equivalence.rs:61:    fn replay_detects_divergence() {
crates/amun-storage-kernel/tests/replay_equivalence.rs:94:        let result = ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap());
crates/amun-storage-kernel/tests/replay_equivalence.rs:96:        assert!(result.unwrap_err().contains("replay divergence"));
crates/amun-storage-kernel/tests/specification_compliance.rs:25:        assert_eq!(root_a.0, root_b.0, "Theorem 1 violated: order independence");
crates/amun-testnet-sim/tests/adversarial_tests.rs:106:    assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:114:// ── N60.3 — Tampered Proof ──────────────────────────────────
crates/amun-testnet-sim/tests/adversarial_tests.rs:116:fn n60_tampered_proof_rejected_by_consensus() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:127:    let mut block = ReplayBackedConsensus::execute_and_replay(
crates/amun-testnet-sim/tests/adversarial_tests.rs:136:    block.replay_verifications[0].replay_success = false;
crates/amun-testnet-sim/tests/adversarial_tests.rs:137:    assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:147:fn n60_crash_recovery_rejoin() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:156:    assert_eq!(recovered_reg.compute_state_root(), state_root_before);
crates/amun-testnet-sim/tests/adversarial_tests.rs:162:fn n60_long_run_blocks_consistent_replay() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:163:    // Use the same initial state for all replays.
crates/amun-testnet-sim/tests/adversarial_tests.rs:164:    // The key insight: replay verifies that the proof is consistent with
crates/amun-testnet-sim/tests/adversarial_tests.rs:206:                // Replay against a registry initialized to the proof's pre-state
crates/amun-testnet-sim/tests/adversarial_tests.rs:209:                                                    // Actually, ReplayVerifier::replay calls execute() which starts
crates/amun-testnet-sim/tests/adversarial_tests.rs:212:                let replay = ReplayVerifier::replay(&transition_proof, &program, &mut fresh, &[]);
crates/amun-testnet-sim/tests/adversarial_tests.rs:213:                if !matches!(replay, ReplayResult::Match { .. }) {
crates/amun-testnet-sim/tests/adversarial_tests.rs:214:                    panic!("Block {} replay failed: {:?}", i + 1, replay);
crates/amun-testnet-sim/tests/adversarial_tests.rs:231:fn n60_byzantine_conflicting_blocks_detected() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:242:    let mut block = ReplayBackedConsensus::execute_and_replay(
crates/amun-testnet-sim/tests/adversarial_tests.rs:251:    block.replay_verifications[0].state_root_match = false;
crates/amun-testnet-sim/tests/adversarial_tests.rs:252:    assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:262:fn n60_large_state_sync() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:270:    assert_eq!(result.unwrap().compute_state_root(), state_root);
crates/amun-testnet-sim/tests/adversarial_tests.rs:44:fn n60_network_partition_no_double_finality() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:55:    let block = ReplayBackedConsensus::execute_and_replay(
crates/amun-testnet-sim/tests/adversarial_tests.rs:63:    assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:69:    assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:6:use amun_replay_consensus::replay_backed_consensus::ReplayBackedConsensus;
crates/amun-testnet-sim/tests/adversarial_tests.rs:76:        assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:7:use amun_replay_verifier::replay_verifier::{ReplayResult, ReplayVerifier};
crates/amun-testnet-sim/tests/adversarial_tests.rs:87:fn n60_malicious_validator_invalid_qc_rejected() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:98:    let block = ReplayBackedConsensus::execute_and_replay(
crates/amun-tokenomics-ledger/src/lib.rs:185:        assert_eq!(ledger.issued_supply(), 0);
crates/amun-tokenomics-ledger/tests/test_ledger.rs:57:    assert_eq!(l1.compute_ledger_root(), l2.compute_ledger_root());
crates/amun-tokenomics-ledger/tests/test_ledger.rs:71:    assert!(ledger.total_issued_ntr > 0, "Should have issued NTR");
crates/amun-transaction/src/tests.rs:110:    assert!(r.expect("test invariant").validate_basic().is_err());
crates/amun-transaction/src/tests.rs:49:    assert!(tx.validate_basic().is_ok());
crates/amun-transaction/src/tests.rs:64:    assert!(r.expect("test invariant").validate_basic().is_ok());
crates/amun-transaction/src/tests.rs:79:    assert!(r.expect("test invariant").validate_basic().is_ok());
crates/amun-transaction/src/tests.rs:94:    assert!(r.expect("test invariant").validate_basic().is_ok());
crates/amun-transactions/src/lib.rs:104:    fn n23_tx_hash_deterministic() {
crates/amun-transactions/src/lib.rs:109:    fn n23_different_nonce_different_hash() {
crates/amun-transactions/src/lib.rs:139:    fn n23_sign_and_verify() {
crates/amun-transactions/src/lib.rs:141:        assert!(tx.verify());
crates/amun-transactions/src/lib.rs:144:    fn n23_tampered_rejected() {
crates/amun-transactions/src/lib.rs:148:        assert!(!tx.verify());
crates/amun-transactions/src/lib.rs:151:    fn n23_wrong_signer_rejected() {
crates/amun-transactions/src/lib.rs:158:        assert!(!fake.verify());
crates/amun-transactions/src/lib.rs:161:    fn n23_transfer_roundtrip() {
crates/amun-transactions/src/lib.rs:171:    fn n23_receipt_error_code() {
crates/amun-transcript-semantics/src/lib.rs:102:    #[test] fn test_causal_chain() { let p = EventIdentity::new([0x01;32],[0x00;32],[0xAA;32],1,ReplayDomain::Consensus,[0xBB;32]); let c = EventIdentity::new([0x02;32],[0x01;32],[0xAA;32],2,ReplayDomain::Consensus,[0xBB;32]); assert!(c.verify_causal_chain(&p)); }
crates/amun-transcript-semantics/src/lib.rs:103:    #[test] fn test_authority_replay_required() { assert!(EventAuthority::Authoritative.is_replay_required()); assert!(!EventAuthority::Derived.is_replay_required()); }
crates/amun-transcript-semantics/src/lib.rs:104:    #[test] fn test_immutable_cert() { let c = ImmutableReplayCertificate::new(ReplayDomain::Consensus,[0xBB;32],[0x01;32],[0x02;32],[0x03;32],[0x04;32],100,1); assert!(c.verify()); let h1 = c.certificate_hash(); let h2 = c.certificate_hash(); assert_eq!(h1, h2); }
crates/amun-transcript-semantics/src/lib.rs:105:    #[test] fn test_witness_hash() { let w = ReplayWitness::MerkleWitness { leaf_hash: [0x01;32], proof_hashes: vec![[0x02;32]], leaf_index: 0 }; assert_ne!(w.witness_hash(), [0;32]); }
crates/amun-transcript-semantics/src/lib.rs:11:pub struct EventIdentity { pub event_hash: [u8; 32], pub causal_parent: [u8; 32], pub authority_root: [u8; 32], pub transcript_position: u64, pub domain: ReplayDomain, pub epoch_id: [u8; 32] }
crates/amun-transcript-semantics/src/lib.rs:13:    pub fn new(event_hash: [u8; 32], causal_parent: [u8; 32], authority_root: [u8; 32], transcript_position: u64, domain: ReplayDomain, epoch_id: [u8; 32]) -> Self { Self { event_hash, causal_parent, authority_root, transcript_position, domain, epoch_id } }
crates/amun-transcript-semantics/src/lib.rs:21:    pub fn is_replay_required(&self) -> bool { matches!(self, EventAuthority::Authoritative | EventAuthority::Certifying) }
crates/amun-transcript-semantics/src/lib.rs:25:// ─── Replay Class ──────────────────────────────────────────
crates/amun-transcript-semantics/src/lib.rs:27:pub enum ReplayClass { ReplayRequired, ReplayRecommended, ReplayDerived, ReplayExcluded, ReplayOptional }
crates/amun-transcript-semantics/src/lib.rs:51:    pub fn is_replay_required(&self) -> bool { self.authority().is_replay_required() }
crates/amun-transcript-semantics/src/lib.rs:52:    pub fn domain(&self) -> ReplayDomain {
crates/amun-transcript-semantics/src/lib.rs:53:        match self { TranscriptEntry::Consensus(_) => ReplayDomain::Consensus, TranscriptEntry::Execution(_) => ReplayDomain::Execution, TranscriptEntry::Governance(_) => ReplayDomain::Governance, TranscriptEntry::Certifying(_) => ReplayDomain::FullSystem }
crates/amun-transcript-semantics/src/lib.rs:59:pub struct ImmutableReplayCertificate { inner: CertifiedEnvelope }
crates/amun-transcript-semantics/src/lib.rs:61:struct CertifiedEnvelope { domain: ReplayDomain, epoch_id: [u8; 32], transcript_root: [u8; 32], state_root: [u8; 32], receipt_root: [u8; 32], ordering_root: [u8; 32], event_count: u64, replay_version: u32, certificate_hash: [u8; 32] }
crates/amun-transcript-semantics/src/lib.rs:62:impl ImmutableReplayCertificate {
crates/amun-transcript-semantics/src/lib.rs:64:    pub fn new(domain: ReplayDomain, epoch_id: [u8; 32], transcript_root: [u8; 32], state_root: [u8; 32], receipt_root: [u8; 32], ordering_root: [u8; 32], event_count: u64, replay_version: u32) -> Self {
crates/amun-transcript-semantics/src/lib.rs:65:        let mut env = CertifiedEnvelope { domain, epoch_id, transcript_root, state_root, receipt_root, ordering_root, event_count, replay_version, certificate_hash: [0; 32] };
crates/amun-transcript-semantics/src/lib.rs:68:    fn compute(env: &CertifiedEnvelope) -> [u8; 32] { let mut h = Sha256::new(); h.update(b"AMUN|IMMUTABLE_CERT|V1"); h.update((env.domain as u8).to_le_bytes()); h.update(env.epoch_id); h.update(env.transcript_root); h.update(env.state_root); h.update(env.receipt_root); h.update(env.ordering_root); h.update(env.event_count.to_le_bytes()); h.update(env.replay_version.to_le_bytes()); h.finalize().into() }
crates/amun-transcript-semantics/src/lib.rs:69:    pub fn domain(&self) -> ReplayDomain { self.inner.domain }
crates/amun-transcript-semantics/src/lib.rs:6:use amun_replay_semantics::ReplayDomain;
crates/amun-transcript-semantics/src/lib.rs:78:pub enum ReplayWitness {
crates/amun-transcript-semantics/src/lib.rs:83:    CompositeWitness { witnesses: Vec<ReplayWitness>, composite_hash: [u8; 32] },
crates/amun-transcript-semantics/src/lib.rs:85:impl ReplayWitness {
crates/amun-transcript-semantics/src/lib.rs:88:        match self { ReplayWitness::MerkleWitness { leaf_hash, proof_hashes, leaf_index } => { h.update(b"MERKLE"); h.update(leaf_hash); h.update(leaf_index.to_le_bytes()); for ph in proof_hashes { h.update(ph); } } ReplayWitness::ExecutionWitness { block_hash, trace_hash, step_count } => { h.update(b"EXECUTION"); h.update(block_hash); h.update(trace_hash); h.update(step_count.to_le_bytes()); } ReplayWitness::TranscriptWitness { start_sequence, end_sequence, fragment_hash } => { h.update(b"TRANSCRIPT"); h.update(start_sequence.to_le_bytes()); h.update(end_sequence.to_le_bytes()); h.update(fragment_hash); } ReplayWitness::ReceiptWitness { receipt_hashes, chain_root } => { h.update(b"RECEIPT"); for rh in receipt_hashes { h.update(rh); } h.update(chain_root); } ReplayWitness::CompositeWitness { composite_hash, .. } => { h.update(b"COMPOSITE"); h.update(composite_hash); } }
crates/amun-transcript-semantics/src/lib.rs:95:    #[derive(Debug, Clone, PartialEq, Eq)] pub enum TranscriptError { CausalChainBroken { parent_hash: [u8; 32], child_parent_hash: [u8; 32] }, AuthorityMismatch { expected: EventAuthority, actual: EventAuthority }, IncompleteReplay { expected: usize, actual: usize } }
crates/amun-transition-proof/src/transition_proof.rs:102:        // Full replay verification comes in W6.
crates/amun-transition-proof/src/transition_proof.rs:99:    /// In a full implementation, this replays the operation log.
crates/amun-truth-engine/src/engine.rs:133:                .map_err(|_| "replay failed")?,
crates/amun-truth-engine/src/engine.rs:197:                    replay_root: self
crates/amun-truth-engine/src/engine.rs:199:                        .map_err(|_| "replay failed")?,
crates/amun-truth-engine/src/engine.rs:23:pub enum ReplayError {
crates/amun-truth-engine/src/engine.rs:248:    pub fn compute_chain_root_until(&self, until: ChainPosition) -> Result<[u8; 32], ReplayError> {
crates/amun-truth-engine/src/engine.rs:265:                        .ok_or(ReplayError::ChunkReadError {
crates/amun-truth-engine/src/engine.rs:276:    pub fn compute_chain_root(&self, target_tx_count: u64) -> Result<[u8; 32], ReplayError> {
crates/amun-truth-engine/src/engine.rs:291:                        .ok_or(ReplayError::ChunkReadError {
crates/amun-truth-engine/src/lib.rs:4:pub use engine::{ReplayError, TruthEngine};
crates/amun-unlock-law/src/lib.rs:81:        assert!(verify_unlock_condition(&qc, &set));
crates/amun-validator-identity/src/signature.rs:55:    fn n105_sign_and_verify_vote() {
crates/amun-validator-identity/src/signature.rs:68:        assert!(verify_ed25519(&pk, &payload, &signature));
crates/amun-validator-identity/src/signature.rs:72:    fn n105_tampered_payload_rejected() {
crates/amun-validator-identity/src/signature.rs:97:        assert!(!verify_ed25519(&pk, &payload, &signature));
crates/amun-validator-identity/src/validator_id.rs:31:    fn n105_validator_id_deterministic() {
crates/amun-validator-networking/src/lib.rs:111:        assert_eq!(imported_reg.compute_state_root(), state_root);
crates/amun-validator-networking/src/lib.rs:116:    fn n57_sync_reject_wrong_history_root() {
crates/amun-validator-networking/src/lib.rs:128:    fn n57_rejoin_protocol_full_roundtrip() {
crates/amun-validator-networking/src/lib.rs:154:    fn n57_rejoin_rejects_wrong_history() {
crates/amun-validator-networking/src/lib.rs:161:    fn n57_network_message_serialization() {
crates/amun-validator-networking/src/lib.rs:75:    fn n57_peer_registry_register_and_lookup() {
crates/amun-validator-networking/src/lib.rs:85:    fn n57_peer_registry_remove() {
crates/amun-validator-networking/src/lib.rs:95:    fn n57_sync_export_and_import() {
crates/amun-validator-runtime/src/validator_cluster.rs:50:    fn n64_cluster_4_nodes_10_blocks() {
crates/amun-validator-runtime/src/validator_cluster.rs:61:    fn n64_cluster_restart_preserves_state() {
crates/amun-validator-runtime/src/validator_node.rs:70:                self.metrics.record_replay();
crates/amun-verification-kernel/src/lib.rs:185:    fn n46_5_issue_certificate() {
crates/amun-verification-kernel/src/lib.rs:191:        let cert = VerificationCertificate::issue("N46", claims, evidence, "verifier-1", 1000);
crates/amun-verification-kernel/src/lib.rs:192:        assert!(cert.verify());
crates/amun-verification-kernel/src/lib.rs:198:    fn n46_5_tampered_certificate_rejected() {
crates/amun-verification-kernel/src/lib.rs:207:        assert!(!cert.verify());
crates/amun-verification-kernel/src/lib.rs:211:    fn n46_5_registry_accepts_valid_certificates() {
crates/amun-verification-kernel/src/lib.rs:225:    fn n46_5_registry_rejects_invalid_certificates() {
crates/amun-verification-kernel/src/lib.rs:239:    fn n46_5_phase_lookup() {
crates/amun-wal/src/authority_index.rs:3:/// Authority index for O(1) replay lookup
crates/amun-wal/src/recovery.rs:15:    Ok(RecoveryPoint { last_sequence: last_seq, last_state_root: None, events_replayed: count, valid: count == wal.frame_count() })
crates/amun-wal/src/recovery.rs:7:    pub events_replayed: u64,
crates/amun-wal/src/wal.rs:36:        wal.replay_frames()?;
crates/amun-wal/src/wal.rs:44:    fn replay_frames(&mut self) -> std::io::Result<()> {
crates/amun-wallet-api/src/lib.rs:105:    fn n48_3_service_get_transaction() {
crates/amun-wallet-api/src/lib.rs:58:    async fn n48_3_build_transaction() {
crates/amun-wallet-api/src/lib.rs:76:    async fn n48_3_build_transaction_invalid() {
crates/amun-wallet-api/src/lib.rs:96:    fn n48_3_service_get_balance() {
crates/amun-wallet-management/src/keystore.rs:10:pub fn save_keystore(keypair: &WalletKeypair, password: &str, path: &str) -> Result<(), String> {
crates/amun-wallet-management/src/keystore.rs:42:pub fn load_keystore(path: &str, password: &str) -> Result<WalletKeypair, String> {
crates/amun-wallet-management/src/lib.rs:47:    fn key05_wrong_password_rejected() {
crates/amun-wallet-management/src/lib.rs:61:        assert!(signer::verify_signature(&kp.public_key, message, &sig));
crates/amun-wallet-management/src/lib.rs:72:        assert!(!signer::verify_signature(&kp.public_key, message, &sig));
crates/amun-wallet-management/src/lib.rs:80:        assert!(signer::verify_signature(&kp.public_key, tx_bytes, &sig));
crates/amun_consensus_math/tests/replay_binary.rs:11:fn read_i64(data: &[u8], pos: &mut usize) -> i64 {
crates/amun_consensus_math/tests/replay_binary.rs:18:fn test_replay_binary_transcript() {
crates/amun_consensus_math/tests/replay_binary.rs:1://! Binary transcript replay test with tolerance for rounding differences
crates/amun_consensus_math/tests/replay_binary.rs:72:    let computed_hash = format!("{:x}", hasher.finalize());
crates/amun_consensus_math/tests/replay_binary.rs:77:            println!("Hash file not found. Computed hash: {}", computed_hash);
crates/amun_consensus_math/tests/replay_binary.rs:84:    println!("Tests executed: {}", test_count);
crates/amun_consensus_math/tests/replay_binary.rs:86:    println!("Rust hash: {}", computed_hash);
crates/amun_consensus_math/tests/replay_binary.rs:91:fn test_consistency_across_calls() {
crates/amun_state_machine/src/certification/mod.rs:61:pub struct ReplayCertificate {
crates/amun_state_machine/src/certification/mod.rs:70:impl ReplayCertificate {
crates/amun_state_machine/src/lib.rs:24:pub use certification::{ExecutionCertificate, ReplayCertificate, compute_execution_fingerprint};
crates/amun_state_machine/src/log.rs:40:/// Causal execution log (for replay)
crates/amun_state_machine/src/snapshot.rs:104:        assert!(snap2.verify_chain(&snap1));
crates/amun_state_machine/src/snapshot.rs:105:        assert!(manager.verify_all());
docs/AUDIT_LAYERS.md:69:# Layer 06 — Replay
docs/CANONICAL_HASH.md:19:| Transcript | `AMUN_TRANSCRIPT_V1` | Replay transcript |
docs/CANONICAL_HASH.md:8:- Replay certificates
docs/CONSTITUTIONAL_MODEL.md:105:# Replayability
docs/CONSTITUTIONAL_MODEL.md:109:Replay is constitutional infrastructure.
docs/CRATE_ARCHITECTURE.md:195:## Layer 9 — Replay
docs/CRATE_ARCHITECTURE.md:201:Replay equivalence and divergence detection.
docs/CRATE_ARCHITECTURE.md:213:Replay semantic interpretation.
docs/CRATE_ARCHITECTURE.md:246:-> Replay
docs/DOCS_INDEX.md:23:    Replay protection through domain-separated signing payloads.
docs/DOCS_INDEX.md:261:    N21: Constitutional identity system.
docs/DOCS_INDEX.md:265:    N48.5A: Constitutional programs specification.
docs/DOCS_INDEX.md:267:    N48.5B: Constitutional resource model.
docs/DOCS_INDEX.md:273:    N48.5D: Constitutional virtual machine.
docs/DOCS_INDEX.md:275:    N48.5E: Constitutional runtime.
docs/DOCS_INDEX.md:283:    N106-N107: Constitutional authority and governance.
docs/DOCS_INDEX.md:311:    Replay physics v1.
docs/DOCS_INDEX.md:317:    Replay law.
docs/DOCS_INDEX.md:319:    Replay model.
docs/DOCS_INDEX.md:435:    Replay cost analysis.
docs/DOCS_INDEX.md:457:    Replay checkpoint fixtures.
docs/DOCS_INDEX.md:459:    Replay divergence fixtures.
docs/DOCS_INDEX.md:461:    Replay equivalence fixtures.
docs/DOCS_INDEX.md:463:    Replay genesis fixtures.
docs/DOCS_INDEX.md:91:    Replay determinism law for storage.
docs/N105_CRYPTOGRAPHIC_VALIDATOR_IDENTITY.md:53:Vote forgery resistance is achieved because without the private key, an attacker cannot produce a valid signature for a given voter_id. The registry ensures that only known validators can attempt to vote. Sybil resistance is enforced because the registry is populated from certificates signed by the authority. An attacker cannot inject fake validators. In a production setting, the authority would be the genesis trust anchor set. Replay protection is provided by the signing payload, which includes the height, block hash, and timestamp, binding each vote to a specific consensus round. The authority key is currently hardcoded to the seed 0x42, which is acceptable for test clusters. In production, this key must be distributed via genesis configuration and protected. Certificate integrity is verified at load time. A tampered certificate will cause a panic in test clusters or an error in production loading code, preventing the node from participating.
docs/N106_N107_CONSTITUTIONAL_AUTHORITY_AND_GOVERNANCE.md:116:### ConstitutionalAuthority
docs/N106_N107_CONSTITUTIONAL_AUTHORITY_AND_GOVERNANCE.md:118:The ConstitutionalAuthority struct defines a versioned authority with an ID derived
docs/N106_N107_CONSTITUTIONAL_AUTHORITY_AND_GOVERNANCE.md:126:ConstitutionalAuthority objects. It tracks the active version and an optional
docs/N106_N107_CONSTITUTIONAL_AUTHORITY_AND_GOVERNANCE.md:158:ConstitutionalAuthority. ScheduleTransition validates both versions exist and
docs/N106_N107_CONSTITUTIONAL_AUTHORITY_AND_GOVERNANCE.md:1:# N106 & N107: Constitutional Authority and Governance
docs/N106_N107_CONSTITUTIONAL_AUTHORITY_AND_GOVERNANCE.md:36:containing ConstitutionalAuthority, AuthorityRegistry, GovernanceProposal, ProposalVotes,
docs/N106_N107_CONSTITUTIONAL_AUTHORITY_AND_GOVERNANCE.md:55:and version number. From this, a ConstitutionalAuthority object is created and used to
docs/N110_COMPLETE.md:150:3. The system does not yet have evidence gossip.  Evidence
docs/N110_COMPLETE.md:153:   that references unknown evidence IDs.  Evidence
docs/N110_COMPLETE.md:38:5. Finality-triggered execution — the actual stake reduction happens
docs/N110_COMPLETE.md:52:  EvidenceRecord stored in EvidenceStore .................. N109.10
docs/N110_COMPLETE.md:87:N109.10            Evidence store ........................ 11
docs/N126_FINAL_BASELINE.md:1:N126 — Evidence-Based Constitutional Enforcement Baseline
docs/N126_FINAL_BASELINE.md:22:    Block → QC → ConstitutionalEnforcementKernel → ConstitutionalVerdict → Commit/Reject
docs/N126_FINAL_BASELINE.md:24:Constitutional Laws Status
docs/N126_FINAL_BASELINE.md:30:5.  SlashingEvidenceBinding   = all certs have non-empty evidence_ids  REAL
docs/N126_FINAL_BASELINE.md:32:7.  ReplayDeterminism         = cert.state_root != [0u8; 32]           PARTIAL
docs/N126_FINAL_BASELINE.md:33:8.  FinalitySupermajority     = cert.qc.verify_quorum()                REAL
docs/N126_FINAL_BASELINE.md:34:9.  StateTransitionValidity   = cert.state_root != [0u8; 32]           PARTIAL
docs/N126_FINAL_BASELINE.md:35:10. EvidenceValidity          = all certs pass .verify()               REAL
docs/N126_FINAL_BASELINE.md:41:Constitution CONSUMES verified evidence from execution layer.
docs/N126_FINAL_BASELINE.md:45:1. ReplayVerifier integration (ReplayDeterminism)
docs/N126_FINAL_BASELINE.md:51:Estimated Constitutional Maturity: ~75-80%
docs/PROJECT_INDEX.md:103:| docs/REPLAY_LAW.md | Replay legal semantics |
docs/PROJECT_INDEX.md:127:| docs/protocol/replay_physics_v1.md | Replay physics |
docs/PROJECT_INDEX.md:159:## Replayability
docs/PROJECT_INDEX.md:55:| fixtures/ | Replay and snapshot fixtures |
docs/PROJECT_INDEX.md:86:| constitution/consensus/REPLAY_DETERMINISM_LAW.md | Replay guarantees |
docs/PROJECT_INDEX.md:97:| docs/REPLAY_MODEL.md | Replay semantics |
docs/PROTOCOL_HARDENING_ROADMAP.md:18:## Phase 3: Replay Certification
docs/PROTOCOL_HARDENING_ROADMAP.md:21:- [ ] Replay certificate generation
docs/PROTOCOL_HARDENING_ROADMAP.md:22:- [ ] Replay attack proof system
docs/REPLAY_LAW.md:109:Replay-equivalent iff:
docs/REPLAY_LAW.md:142:ReplayHash = H(Transcript)
docs/REPLAY_LAW.md:155:        Witness1 ||
docs/REPLAY_LAW.md:156:        Witness2 ||
docs/REPLAY_LAW.md:157:        Witness3
docs/REPLAY_LAW.md:15:Replay guarantees:
docs/REPLAY_LAW.md:166:Replay certificate proves:
docs/REPLAY_LAW.md:188:Replay MUST fail if:
docs/REPLAY_LAW.md:258:Replay security depends on:
docs/REPLAY_LAW.md:269:Replay law applies to:
docs/REPLAY_LAW.md:26:Replay(GenesisState, Transcript) = FinalState
docs/REPLAY_LAW.md:280:Replay does NOT guarantee:
docs/REPLAY_LAW.md:28:ReplayHash = OriginalReplayHash
docs/REPLAY_LAW.md:290:Replay Model:
docs/REPLAY_LAW.md:296:Proof:
docs/REPLAY_LAW.md:30:Replay MUST always produce identical output.
docs/REPLAY_LAW.md:325:Replay becomes frozen after:
docs/REPLAY_LAW.md:334:Replay is the constitutional heart of AmunChain.
docs/REPLAY_LAW.md:36:Replay is defined as:
docs/REPLAY_LAW.md:3:## Phase 81 — Deterministic Replay Constitution
docs/REPLAY_LAW.md:40:Replay is NOT:
docs/REPLAY_LAW.md:51:Replay input:
docs/REPLAY_LAW.md:54:- Constitutional Rules
docs/REPLAY_LAW.md:61:Replay output:
docs/REPLAY_LAW.md:64:- Replay Transcript Hash
docs/REPLAY_LAW.md:65:- Replay Certificate
docs/REPLAY_LAW.md:73:Transcript = Vec<VerifiedTransitionWitness>
docs/REPLAY_MODEL.md:11:# Replay Guarantees
docs/REPLAY_MODEL.md:1:# Replay Model
docs/REPLAY_MODEL.md:28:Replay may not depend on:
docs/REPLAY_MODEL.md:38:Replay divergence must always be detectable.
docs/REPLAY_MODEL.md:48:# Replay DAG
docs/REPLAY_MODEL.md:50:Replay execution is modeled as a DAG.
docs/REPLAY_MODEL.md:5:Replay is a constitutional guarantee.
docs/REPLAY_MODEL.md:60:# Replay Certificates
docs/REPLAY_MODEL.md:62:Replay certificates prove:
docs/REPLAY_MODEL.md:70:# Replay Philosophy
docs/REPLAY_MODEL.md:72:Replay is constitutional historical reconstruction.
docs/REPOSITORY_LAYOUT.md:206:## Replay Layer
docs/REPOSITORY_LAYOUT.md:21:| fixtures/ | Replay and snapshot test fixtures |
docs/REPOSITORY_LAYOUT.md:247:| REPLAY_MODEL.md | Replay semantics |
docs/SECURITY_MODEL.md:39:## Replay Divergence
docs/V7_009_WHY_CLOSURE.md:90:## 4. Experimental Evidence
docs/VALIDATOR_ORDERING.md:28:- Replay equivalence
docs/architecture/CRATE_CLASSIFICATION.md:38:- amun-test-replay - Replay testing
docs/architecture/PHASE_49_COMPLETE.md:100:| R.1 | System replayable from transcript | Replay Law |
docs/architecture/PHASE_49_COMPLETE.md:107:2. **Replay Certification suite** — cross-node equivalence proofs
docs/architecture/PHASE_49_COMPLETE.md:128:The foundation is ready for Phase 50: Formal Replay Infrastructure.
docs/architecture/PHASE_49_COMPLETE.md:41:3. **Capability Firewall**: 6 separated capabilities (Executor, Verifier, Journal, Replay, Finalize, Authority)
docs/audit/AUDIT_EVIDENCE_BUNDLE.md:19:| C1 | Replay before vote | w18_reject_if_replay_fails | amun-replay-consensus | ✅ |
docs/audit/AUDIT_EVIDENCE_BUNDLE.md:41:| Proof replay attack | byz_010 | ✅ Rejected |
docs/audit/AUDIT_EVIDENCE_BUNDLE.md:85:| Replay | amun-replay-verifier | 3 | ✅ |
docs/audit/N103_MAINNET_READINESS_AUDIT.md:14:| A5 | State | Constitutional root determinism (cross-node) | ✅ | 107+ CCA tests pass |
docs/audit/SECURITY_INVARIANTS.md:13:| X1 | Transfer proof single-use | TransferProofRegistry | N47 Verdict Engine |
docs/audit/SECURITY_INVARIANTS.md:29:| C1 | Replay before vote | ReplayBackedConsensus::form_consensus |
docs/audit/SECURITY_INVARIANTS.md:32:| C4 | No conflicting finality | Theorem 5 (Replay-Backed Safety) |
docs/audit/SECURITY_INVARIANTS.md:40:| K3 | Anti-replay protection | AntiReplayGuard::check_and_record |
docs/audit/SECURITY_INVARIANTS.md:8:| R2 | Consumed resources cannot be used | VMKernel::verify | ReplayVerifier |
docs/audit/THREAT_MODEL.md:20:| Consensus | Double voting | Replay-backed QC (C1) |
docs/audit/THREAT_MODEL.md:22:| Network | Replay attack | AntiReplayGuard (K3) |
docs/audit/TRACEABILITY_MATRIX.md:10:| N48.5-W14 Replay | amun-replay-verifier | 3 tests | ✅ |
docs/audit/TRACEABILITY_MATRIX.md:13:| N48.5-W19 Finality | amun-evidence-finality | 3 tests | ✅ |
docs/audit/TRACEABILITY_MATRIX.md:14:| N48.5-W20-21A PCCV | amun-pccv | 11 tests | ✅ |
docs/audit/TRACEABILITY_MATRIX.md:8:| N48.5-W4 Evidence | amun-evidence-engine | 5 tests | ✅ |
docs/audit/TRACEABILITY_MATRIX.md:9:| N48.5-W5 Transition Proof | amun-transition-proof | 5 tests | ✅ |
docs/consensus/constitution.md:111:### Section 6.1: Execution Replay
docs/constitutional-mathematics/PHASE_112_SIMULATION_SPECIFICATION.md:11:Transform Constitutional Mathematics into an experimental computational theory.
docs/constitutional/phase84_freeze.md:24:- Levels: FullyCompatible > ReplayCompatible > SnapshotCompatible > ReadOnlyCompatible > Incompatible
docs/constitutional/phase84_freeze.md:54:- ReplayVerifier checks every frame's state_root against reconstructed state
docs/constitutional/phase85_seal.md:16:8. Graded guarantees: Replay/Snapshot/Proof/Governance/Continuity
docs/federation/FEDERATION_ARCHITECTURE.md:29:| Replay Boundary Engine | Replay isolation |
docs/protocol/FREEZE_CERTIFICATE_v1.md:22:- Genesis Replay Root: fixtures/replay/genesis/replay_root.bin
docs/protocol/FREEZE_CERTIFICATE_v1.md:39:2. New REPLAY_PROTOCOL_VERSION
docs/protocol/FREEZE_CERTIFICATE_v1.md:5:This certifies that the Amun Replay Protocol v1.0 has been
docs/protocol/replay_physics_v1.md:13:### 2.2 Ordering Invariant
docs/protocol/replay_physics_v1.md:14:sequence numbers MUST be strictly monotonic within a replay session. Any gap (expected_sequence != actual_sequence) SHALL produce ReplayFailure::OrderingViolation.
docs/protocol/replay_physics_v1.md:24:execute_and_self_verify() SHALL: 1. Execute trace via DeterministicExecutor, 2. Apply same entries via ReplayState, 3. Compare final roots, 4. Return EquivalenceProof.
docs/protocol/replay_physics_v1.md:28:These laws are FROZEN for protocol v1. Amendment requires: 1. Constitutional amendment proposal, 2. New protocol version identifier, 3. Migration proof for all existing state, 4. Golden value regeneration.
docs/protocol/replay_physics_v1.md:6:All ConstitutionalHash values are raw 32-byte blobs. No length prefix. No type tag. Fixed-width canonical encoding.
docs/protocol/replay_physics_v1.md:7:### 1.3 Hash Algorithm
docs/reports/CCA_IMPL_4_FINAL_REPORT.md:193:Constitutional commitments can no longer be bypassed, ignored, or detached from finalized chain history.
docs/reports/CCA_IMPL_5B_FINAL_REPORT.md:154:3. Replayable Constitutional State
