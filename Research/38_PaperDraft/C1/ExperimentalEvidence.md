1034:crates/amun-constitutional-commitment/tests/state.rs:133:    assert!(ConstitutionalState::load(&[0u8; 10]).is_none());
1035:crates/amun-constitutional-commitment/tests/state.rs:134:    assert!(ConstitutionalState::load(&[0u8; 200]).is_none());
1036:crates/amun-constitutional-commitment/tests/state.rs:135:    assert!(ConstitutionalState::load(&[]).is_none());
1096:crates/amun-constitutional-enforcement/src/lib.rs:252:                assert_eq!(violations[0].law, ConstitutionalLaw::StateRootIntegrity);
1102:crates/amun-constitutional-enforcement/src/proof_engine.rs:276:        assert_eq!(verdict, ConstitutionalVerdict::Constitutional);
1104:crates/amun-constitutional-enforcement/src/state_transition.rs:138:        assert_eq!(verdict, ConstitutionalVerdict::Constitutional);
1116:crates/amun-snapshot-engine/tests/constitutional_tests.rs:4:        CompatibilityEngine, CompatibilityLevel, ConstitutionalHash, ConstitutionalIdentity,
1175:crates/amun-constitutional-proof/src/lib.rs:1415:        assert!(md.contains("N47 Constitutional Validation Report"));
123:crates/amun-byzantine-tests/tests/attack_suite.rs:25:    let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
124:crates/amun-byzantine-tests/tests/attack_suite.rs:299:        ConstitutionalEvidence::ConstitutionalViolation { law, .. } => {
125:crates/amun-byzantine-tests/tests/attack_suite.rs:2:use amun_bytecode::program::ConstitutionalProgram;
126:crates/amun-byzantine-tests/tests/attack_suite.rs:38:    let result = ConstitutionalRuntime::execute(
127:crates/amun-byzantine-tests/tests/attack_suite.rs:3:use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
128:crates/amun-byzantine-tests/tests/attack_suite.rs:6:use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
1402:crates/amun-constitutional/src/causality_type.rs:109:        assert!(!CausalityType::ConstitutionalDependency.is_non_causal());
1403:crates/amun-constitutional/src/causality_type.rs:113:        assert!(CausalityType::ConstitutionalDependency.is_hard_dependency());
1406:crates/amun-constitutional/src/causality_type.rs:97:        assert!(CausalityType::ConstitutionalDependency.is_constitutional_dependency());
1427:crates/amun-constitutional/src/replay_outcome.rs:74:        assert!(!ReplayOutcome::ConstitutionalFailure.is_admitted());
1431:crates/amun-constitutional/src/replay_outcome.rs:82:        assert!(ReplayOutcome::ConstitutionalFailure.is_failure());
1456:crates/amun-constitutional/src/divergence_type.rs:75:        assert!(DivergenceType::ConstitutionalFork.is_admissible());
1458:crates/amun-constitutional/src/divergence_type.rs:77:        assert!(DivergenceType::ConstitutionalSupersession.is_admissible());
1461:crates/amun-constitutional/src/divergence_type.rs:83:        assert!(!DivergenceType::ConstitutionalFork.is_error());
1494:crates/amun-constitutional/src/replay_outcome.rs:74:        assert!(!ReplayOutcome::ConstitutionalFailure.is_admitted());
1498:crates/amun-constitutional/src/replay_outcome.rs:82:        assert!(ReplayOutcome::ConstitutionalFailure.is_failure());
16:crates/amun-audit/tests/audit_layer03_snapshot.rs:4:        CompatibilityEngine, CompatibilityLevel, ConstitutionalIdentity, SnapshotHeader,
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
1757:crates/amun-finality-certificate/src/lib.rs:102:    pub fn latest(&self) -> Option<&ConstitutionalFinalityCertificate> {
177:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:100:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
178:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:108:        let block = ConstitutionalBlock::new(
179:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:133:    let mut rt_check = ConstitutionalStateRuntime::new();
17:crates/amun-audit/tests/audit_layer03_snapshot.rs:54:    // CONST-SNAP-003: Constitutional identity must be deterministic
180:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:138:    let mut rt_verify = ConstitutionalStateRuntime::new();
181:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:13:    let mut rt_a = ConstitutionalStateRuntime::new();
182:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:21:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
183:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:23:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
184:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:31:        let block = ConstitutionalBlock::new(
185:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:69:    let mut rt_b = ConstitutionalStateRuntime::new();
186:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:8:use amun_constitutional_block::ConstitutionalBlock;
187:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:90:    let mut rt = ConstitutionalStateRuntime::new();
1884:crates/amun-light-client/tests/light_client_tests.rs:22:) -> ConstitutionalFinalityCertificate {
1886:crates/amun-light-client/tests/light_client_tests.rs:3:use amun_constitutional_runtime::finality_certificate::ConstitutionalFinalityCertificate;
1887:crates/amun-light-client/tests/light_client_tests.rs:45:    let mut cert = ConstitutionalFinalityCertificate::issue(
188:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:98:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
189:crates/amun-chain-checkpoint/tests/n15_constitutional_join.rs:9:use amun_constitutional_state::ConstitutionalStateRuntime;
18:crates/amun-audit/tests/audit_layer03_snapshot.rs:57:        let id1 = ConstitutionalIdentity::new([0x42u8; 32]);
190:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:14:    let mut rt = ConstitutionalStateRuntime::new();
191:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:22:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
1924:crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:53:    assert_eq!(verdict, ConstitutionalVerdict::Constitutional);
192:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:24:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
193:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:32:        let block = ConstitutionalBlock::new(
1940:crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:39:                    .any(|v| v.law == ConstitutionalLaw::FinalitySupermajority),
194:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:7:use amun_constitutional_block::ConstitutionalBlock;
195:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:8:use amun_constitutional_state::ConstitutionalStateRuntime;
1966:crates/amun-networking/tests/n18_node_rejoin.rs:73:// N18.5 — Constitutional Invariant REJOIN-001
19:crates/amun-audit/tests/audit_layer03_snapshot.rs:58:        let id2 = ConstitutionalIdentity::new([0x42u8; 32]);
20:crates/amun-audit/tests/audit_layer03_snapshot.rs:72:        let id = ConstitutionalIdentity::new([0xAAu8; 32]);
21:crates/amun-audit/tests/audit_layer04_byzantine.rs:10:        let identity = ConstitutionalIdentity::new([0x01u8; 32]);
21:crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:4:        ByzantineSyncEngine, ConstitutionalIdentity, PeerManifest, SnapshotManifest, SyncDecision,
22:crates/amun-audit/tests/audit_layer04_byzantine.rs:4:        ByzantineSyncEngine, ConstitutionalIdentity, PeerManifest, SnapshotManifest, SyncDecision,
23:crates/amun-audit/tests/audit_layer04_byzantine.rs:55:        let local = ConstitutionalIdentity::new([0xAAu8; 32]);
243:crates/amun-constitutional-authority/tests/authority_tests.rs:7:fn build_root(key: &ConstitutionalKeyPair) -> ConstitutionalCertificate {
24:crates/amun-audit/tests/audit_layer04_byzantine.rs:56:        let foreign = ConstitutionalIdentity::new([0xBBu8; 32]);
25:crates/amun-audit/tests/audit_layer05_identity.rs:19:        let id = ConstitutionalIdentity::new([0xFFu8; 32]);
26:crates/amun-audit/tests/audit_layer05_identity.rs:29:        let id = ConstitutionalIdentity::new([0xFFu8; 32]);
27:crates/amun-audit/tests/audit_layer05_identity.rs:31:        let decoded = ConstitutionalIdentity::decode(&encoded)
2850:crates/amun-snapshot-engine/tests/constitutional_tests.rs:66:        assert!(matches!(rel, ConstitutionalRelationship::Identical));
28:crates/amun-audit/tests/audit_layer05_identity.rs:3:    use amun_snapshot_engine::ConstitutionalIdentity;
29:crates/amun-audit/tests/audit_layer05_identity.rs:46:        let id = ConstitutionalIdentity::new([0x13u8; 32]);
30:crates/amun-audit/tests/audit_layer05_identity.rs:51:        if let Some(decoded) = ConstitutionalIdentity::decode(&encoded) {
31:crates/amun-audit/tests/audit_layer05_identity.rs:8:        let id1 = ConstitutionalIdentity::new([0x01u8; 32]);
32:crates/amun-audit/tests/audit_layer05_identity.rs:9:        let id2 = ConstitutionalIdentity::new([0x02u8; 32]);
33:crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:10:        let identity = ConstitutionalIdentity::new([0x01u8; 32]);
34:crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:4:        ByzantineSyncEngine, ConstitutionalIdentity, PeerManifest, SnapshotManifest, SyncDecision,
35:crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:69:        let local = ConstitutionalIdentity::new([0xAAu8; 32]);
36:crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:70:        let foreign = ConstitutionalIdentity::new([0xBBu8; 32]);
391:crates/amun-constitutional-sim/tests/phase_116_engineering_map.rs:31:    println!("\n=== Phase 116C: Constitutional Engineering Map ===");
5:crates/amun-audit/tests/audit_layer03_snapshot.rs:4:        CompatibilityEngine, CompatibilityLevel, ConstitutionalIdentity, SnapshotHeader,
627:crates/amun-failure/src/tests.rs:32:    assert!(!ConstitutionalFault::InvalidStateTransition.should_halt());
726:crates/amun-networking/tests/n18_node_rejoin.rs:73:// N18.5 — Constitutional Invariant REJOIN-001
76:crates/amun-byzantine-tests/tests/attack_suite.rs:299:        ConstitutionalEvidence::ConstitutionalViolation { law, .. } => {
81:crates/amun-byzantine-tests/tests/attack_suite.rs:6:use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
945:crates/amun-peer-identity/tests/peer_identity_tests.rs:2:use amun_peer_identity::{ConstitutionalPeerId, IdentityVerifier, PeerCertificate, PeerRegistry};
9:crates/amun-audit/tests/audit_layer04_byzantine.rs:4:        ByzantineSyncEngine, ConstitutionalIdentity, PeerManifest, SnapshotManifest, SyncDecision,
crates/amun-attestation/src/validator.rs:3:pub struct ValidatorAttestation {
crates/amun-audit/tests/audit_layer02_geometry.rs:7:    fn geo001_proof_depth_invariant() {
crates/amun-audit/tests/audit_layer03_snapshot.rs:4:        CompatibilityEngine, CompatibilityLevel, ConstitutionalIdentity, SnapshotHeader,
crates/amun-audit/tests/audit_layer03_snapshot.rs:54:    // CONST-SNAP-003: Constitutional identity must be deterministic
crates/amun-audit/tests/audit_layer03_snapshot.rs:57:        let id1 = ConstitutionalIdentity::new([0x42u8; 32]);
crates/amun-audit/tests/audit_layer03_snapshot.rs:58:        let id2 = ConstitutionalIdentity::new([0x42u8; 32]);
crates/amun-audit/tests/audit_layer03_snapshot.rs:72:        let id = ConstitutionalIdentity::new([0xAAu8; 32]);
crates/amun-audit/tests/audit_layer04_byzantine.rs:10:        let identity = ConstitutionalIdentity::new([0x01u8; 32]);
crates/amun-audit/tests/audit_layer04_byzantine.rs:4:        ByzantineSyncEngine, ConstitutionalIdentity, PeerManifest, SnapshotManifest, SyncDecision,
crates/amun-audit/tests/audit_layer04_byzantine.rs:55:        let local = ConstitutionalIdentity::new([0xAAu8; 32]);
crates/amun-audit/tests/audit_layer04_byzantine.rs:56:        let foreign = ConstitutionalIdentity::new([0xBBu8; 32]);
crates/amun-audit/tests/audit_layer05_identity.rs:19:        let id = ConstitutionalIdentity::new([0xFFu8; 32]);
crates/amun-audit/tests/audit_layer05_identity.rs:29:        let id = ConstitutionalIdentity::new([0xFFu8; 32]);
crates/amun-audit/tests/audit_layer05_identity.rs:31:        let decoded = ConstitutionalIdentity::decode(&encoded)
crates/amun-audit/tests/audit_layer05_identity.rs:3:    use amun_snapshot_engine::ConstitutionalIdentity;
crates/amun-audit/tests/audit_layer05_identity.rs:46:        let id = ConstitutionalIdentity::new([0x13u8; 32]);
crates/amun-audit/tests/audit_layer05_identity.rs:51:        if let Some(decoded) = ConstitutionalIdentity::decode(&encoded) {
crates/amun-audit/tests/audit_layer05_identity.rs:8:        let id1 = ConstitutionalIdentity::new([0x01u8; 32]);
crates/amun-audit/tests/audit_layer05_identity.rs:9:        let id2 = ConstitutionalIdentity::new([0x02u8; 32]);
crates/amun-audit/tests/audit_layer09_freeze.rs:15:    // CONST-FREEZE-002: Proof version is frozen at 0x01
crates/amun-audit/tests/audit_layer09_freeze.rs:17:    fn freeze002_proof_version_is_v1() {
crates/amun-audit/tests/audit_layer09_freeze.rs:21:            "CONST-FREEZE-002 VIOLATION: Proof version must be 0x01"
crates/amun-audit/tests/audit_layer10_adversarial.rs:37:    // CONST-ADV-002: Malformed proof must not verify
crates/amun-audit/tests/audit_layer10_adversarial.rs:39:    fn adv002_malformed_proof_rejection() {
crates/amun-audit/tests/audit_layer10_adversarial.rs:48:            proof.verify(root.0),
crates/amun-audit/tests/audit_layer10_adversarial.rs:49:            "CONST-ADV-002: Valid proof must verify"
crates/amun-audit/tests/audit_layer10_adversarial.rs:53:            !proof.verify(wrong_root),
crates/amun-audit/tests/audit_layer10_adversarial.rs:54:            "CONST-ADV-002 VIOLATION: Proof verified against wrong root"
crates/amun-audit/tests/audit_layer12_fuzzing.rs:14:            let _ = MerkleProof::decode(&data);
crates/amun-audit/tests/audit_layer12_fuzzing.rs:39:    fn fuzz003_absence_proof_consistency() {
crates/amun-audit/tests/audit_layer12_fuzzing.rs:3:    use amun_storage_kernel::smt::proof::MerkleProof;
crates/amun-audit/tests/audit_layer12_fuzzing.rs:54:                    proof.verify(root.0),
crates/amun-audit/tests/audit_layer12_fuzzing.rs:55:                    "CONST-FUZZ-003 VIOLATION: Absence proof must verify against correct root"
crates/amun-audit/tests/audit_layer12_fuzzing.rs:7:    // CONST-FUZZ-001: Proof decoder never panics on random bytes
crates/amun-audit/tests/audit_layer12_fuzzing.rs:9:    fn fuzz001_proof_decode_random_bytes() {
crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:10:        let identity = ConstitutionalIdentity::new([0x01u8; 32]);
crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:4:        ByzantineSyncEngine, ConstitutionalIdentity, PeerManifest, SnapshotManifest, SyncDecision,
crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:69:        let local = ConstitutionalIdentity::new([0xAAu8; 32]);
crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:70:        let foreign = ConstitutionalIdentity::new([0xBBu8; 32]);
crates/amun-audit/tests/audit_layer16_mutation.rs:17:    // CONST-MUT-002: Proof version must remain 0x01
crates/amun-audit/tests/audit_layer16_mutation.rs:19:    fn mut002_proof_version_frozen() {
crates/amun-audit/tests/audit_layer16_mutation.rs:23:            "CONST-MUT-002: Proof version mutation detected - must be 0x01, got {}",
crates/amun-audit/tests/audit_layer16_mutation.rs:40:    fn mut004_endian_invariant() {
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
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:102:fn n132_3_6_economic_change_changes_constitutional_roots() {
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:2:use amun_constitutional_commitment::EconomicTree;
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:73:    // Recompute the complete constitutional roots using the same canonical
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:78:        .constitutional_roots_with_ledger(&builder.engine.economic);
crates/amun-block-builder/tests/n32_certified_block.rs:15:    pub fn new(block: Block, qc: QuorumCertificate) -> Self {
crates/amun-block-builder/tests/n32_certified_block.rs:9:pub struct CertifiedBlock {
crates/amun-bls/src/tests.rs:18:        assert!(verify(msg, &sig, &kp.public).expect("test invariant"));
crates/amun-bls/src/tests.rs:25:        assert!(!verify(msg, &zero_sig, &kp.public).expect("test invariant"));
crates/amun-byzantine-tests/tests/attack_suite.rs:14:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-byzantine-tests/tests/attack_suite.rs:24:fn byz_001_forged_proof_rejected() {
crates/amun-byzantine-tests/tests/attack_suite.rs:25:    let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
crates/amun-byzantine-tests/tests/attack_suite.rs:262:fn byz_009_proof_tampering_detected() {
crates/amun-byzantine-tests/tests/attack_suite.rs:263:    let proof = TransitionProof::new(
crates/amun-byzantine-tests/tests/attack_suite.rs:276:    assert!(proof.verify_integrity());
crates/amun-byzantine-tests/tests/attack_suite.rs:283:fn byz_010_proof_replay_attack_blocked() {
crates/amun-byzantine-tests/tests/attack_suite.rs:284:    let mut registry = TransferProofRegistry::new();
crates/amun-byzantine-tests/tests/attack_suite.rs:285:    let proof = CrossContractTransferProof::new(
crates/amun-byzantine-tests/tests/attack_suite.rs:299:        ConstitutionalEvidence::ConstitutionalViolation { law, .. } => {
crates/amun-byzantine-tests/tests/attack_suite.rs:2:use amun_bytecode::program::ConstitutionalProgram;
crates/amun-byzantine-tests/tests/attack_suite.rs:35:    let mut hot = HotProofStore::new(100);
crates/amun-byzantine-tests/tests/attack_suite.rs:36:    let mut archive = ProofArchive::new();
crates/amun-byzantine-tests/tests/attack_suite.rs:38:    let result = ConstitutionalRuntime::execute(
crates/amun-byzantine-tests/tests/attack_suite.rs:3:use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
crates/amun-byzantine-tests/tests/attack_suite.rs:4:use amun_cross_contract::transfer_proof::CrossContractTransferProof;
crates/amun-byzantine-tests/tests/attack_suite.rs:50:            transition_proof, ..
crates/amun-byzantine-tests/tests/attack_suite.rs:51:        } => transition_proof,
crates/amun-byzantine-tests/tests/attack_suite.rs:56:    let replay = ReplayVerifier::replay(&proof, &program, &mut fresh_reg, &[]);
crates/amun-byzantine-tests/tests/attack_suite.rs:5:use amun_cross_contract::transfer_registry::TransferProofRegistry;
crates/amun-byzantine-tests/tests/attack_suite.rs:65:    let mut registry = TransferProofRegistry::new();
crates/amun-byzantine-tests/tests/attack_suite.rs:66:    let proof = CrossContractTransferProof::new(
crates/amun-byzantine-tests/tests/attack_suite.rs:6:use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
crates/amun-byzantine-tests/tests/attack_suite.rs:7:use amun_proof_archive::hot_store::HotProofStore;
crates/amun-byzantine-tests/tests/attack_suite.rs:8:use amun_proof_archive::proof_archive::ProofArchive;
crates/amun-certificate-network/src/distribution.rs:128:    fn create_test_bundle() -> LightClientProofBundle {
crates/amun-certificate-network/src/distribution.rs:174:            InclusionProofMessage::InclusionProofResponse { proof } => assert!(proof.verify()),
crates/amun-certificate-network/src/distribution.rs:192:        assert!(bundle.verify().is_ok());
crates/amun-certificate-network/src/distribution.rs:199:        assert!(bundle.verify().is_err());
crates/amun-certificate-network/src/distribution.rs:211:                assert!(received.verify().is_ok());
crates/amun-certificate-network/src/distribution.rs:224:            ProofBundleMessage::BundleNotFound { reason } => assert!(!reason.is_empty()),
crates/amun-certificate-network/src/distribution.rs:250:        assert!(bundle.verify().is_ok());
crates/amun-certificate-network/src/distribution.rs:287:        assert!(deserialized.verify().is_ok());
crates/amun-certificate-network/src/gossip.rs:162:    fn create_test_data() -> (
crates/amun-certificate-network/src/gossip.rs:246:            ProofSync::ProofResponse { proof: p } => assert!(p.verify()),
crates/amun-certificate-network/src/gossip.rs:258:            BundleGossip::BundleResponse { bundle: b } => assert!(b.verify().is_ok()),
crates/amun-certificate-network/src/gossip.rs:348:                assert!(b.verify().is_ok());
crates/amun-chain-checkpoint/src/chain.rs:104:        assert!(RecursiveCheckpointProof::from_checkpoints(&[c1, c2])
crates/amun-chain-checkpoint/src/chain.rs:113:        assert!(RecursiveCheckpointProof::from_checkpoints(&[c1, c2])
crates/amun-chain-checkpoint/src/chain.rs:123:        assert!(proof.verify().is_err());
crates/amun-chain-checkpoint/src/inclusion.rs:167:        assert!(proof.verify());
crates/amun-chain-checkpoint/src/inclusion.rs:176:        assert!(!proof.verify());
crates/amun-chain-checkpoint/src/inclusion.rs:184:        assert!(!proof.verify());
crates/amun-chain-checkpoint/src/inclusion.rs:191:        assert!(CheckpointBundle::new(c, proof).verify().is_ok());
crates/amun-chain-checkpoint/src/lib.rs:200:    pub fn latest(&self) -> Option<&CheckpointCertificate> {
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
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:124:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:14:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:15:    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:1:use amun_certificate_network::distribution::LightClientProofBundle;
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:22:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:24:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:32:        let block = ConstitutionalBlock::new(
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:45:        bundles.push(LightClientProofBundle::new(block, cert, proof));
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:51:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:68:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:7:use amun_constitutional_block::ConstitutionalBlock;
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:8:use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-consensus-execution/src/block_dag.rs:215:        // Invariant assertions
crates/amun-consensus-integration/src/consensus_integrator.rs:134:        assert!(block.verify_all_proofs());
crates/amun-consensus-integration/src/consensus_integrator.rs:239:        assert_eq!(block.proof_root, block.compute_proof_root());
crates/amun-consensus-network/src/certificate_evidence_validation.rs:140:        assert_eq!(result, EvidenceValidationResult::AllPresent);
crates/amun-consensus-network/src/certificate_gossip.rs:76:    fn make_test_cert(id: [u8; 32]) -> SlashingCertificate {
crates/amun-consensus-network/src/finality_gate.rs:69:        assert!(is_certificate_finalized(&cert, 100));
crates/amun-consensus-network/src/finality_gate.rs:70:        assert!(is_certificate_finalized(&cert, 150));
crates/amun-consensus-network/src/finality_gate.rs:79:        assert!(!is_certificate_finalized(&cert, 99));
crates/amun-consensus-network/src/messages.rs:210:        assert!(proof.verify_standalone().is_ok());
crates/amun-consensus-network/src/messages.rs:270:        assert!(proof.verify_standalone().is_err());
crates/amun-consensus-network/src/messages.rs:283:        assert!(proof.verify_standalone().is_err());
crates/amun-consensus-network/src/messages.rs:296:        assert!(proof.verify_standalone().is_err());
crates/amun-consensus-network/src/messages.rs:311:        assert!(proof.verify_standalone().is_err());
crates/amun-consensus-network/src/messages.rs:324:        assert!(proof.verify_standalone().is_err());
crates/amun-consensus-network/src/slashing_certificate.rs:253:        assert!(decoded.verify().is_ok());
crates/amun-consensus-network/src/slashing_certificate.rs:284:        assert_eq!(cert1.compute_hash(), cert1.certificate_hash);
crates/amun-consensus-network/src/slashing_certificate.rs:285:        assert_eq!(cert2.compute_hash(), cert2.certificate_hash);
crates/amun-consensus-network/src/slashing_certificate.rs:332:        assert!(cert.verify().is_err());
crates/amun-consensus-network/src/slashing_fraud_proof.rs:187:        assert!(decoded.verify().is_ok());
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:218:        assert!(build_inclusion_proof(&slashes, 5).is_err());
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:223:        assert!(build_inclusion_proof(&[], 0).is_err());
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:21:struct ExecutionCommitment {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:36:struct ConsensusVote {
crates/amun-consensus-network/tests/n109_block_propagation.rs:19:struct BlockProposal {
crates/amun-consensus-network/tests/n109_block_propagation.rs:30:struct ConsensusVoteStub {
crates/amun-consensus-network/tests/n109_block_propagation.rs:42:enum NetworkMessage {
crates/amun-consensus-network/tests/n109_block_propagation.rs:4:// N109 — Constitutional Block Propagation — Test Suite
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:15:struct BlockProposal {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:198:    // Constitutional check: 2 approvals out of 4 total validators (not > 2/3 of 4=3)
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:29:struct ExecutionReceipt {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:40:struct ConsensusMetrics {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:8:// Constitutional requirement: state_root mismatch → no vote → no QC formed.
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:107:    let result_b = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:12:    build_missing_evidence_request, process_evidence_response, validate_certificate_evidence,
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:13:    CertificateGossip, EvidenceAnnouncement, EvidenceCount, EvidenceGossip, EvidenceRecord,
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:14:    EvidenceStore, EvidenceType, EvidenceValidationResult, SlashingCertificate, ValidatorStatus,
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:153:    let result_b_after = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:167:fn n111_7_certificate_gossip_between_nodes() {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:18:fn make_certificate(validator_id: [u8; 32], evidence_ids: Vec<[u8; 32]>) -> SlashingCertificate {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:192:    let result = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:203:    let result_after = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:93:    let result_a = validate_certificate_evidence(&cert, &store_a);
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:103:    let result_before = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:11:    process_incoming_evidence_push, validate_certificate_evidence, EvidenceCount, EvidenceGossip,
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:128:    // Phase 4: Node B validates the certificate — ACCEPTED without pull
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:130:    let result_after = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:134:        "N112.4 FAIL: After push, certificate must be accepted without MissingEvidenceRequest"
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:177:// N112.4: Certificate validated immediately after push (no pull cycle)
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:180:fn n112_4_certificate_immediately_accepted_after_push() {
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:204:    // Certificate accepted — NO MissingEvidenceRequest needed
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:205:    let result = validate_certificate_evidence(&cert, &store_b);
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:26:fn make_certificate(validator_id: [u8; 32], evidence_ids: Vec<[u8; 32]>) -> SlashingCertificate {
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:6:// MissingEvidenceRequest/Response cycle.  The certificate is accepted
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
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:10:fn make_cert(validator_id: [u8; 32], height: u64, amount: u64) -> SlashingCertificate {
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:3:// Verifies that replaying the same sequence of slashing certificates
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:7:    EvidenceCount, EvidenceType, SlashingCertificate, SlashingState, ValidatorStatus,
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:11:fn make_cert(vid: [u8; 32], h: u64, amt: u64) -> SlashingCertificate {
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:72:    // Rebuild from the same certificates (simulating restore from snapshot)
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:8:    EvidenceCount, EvidenceType, SlashingCertificate, SlashingState, ValidatorStatus,
crates/amun-constitution-builder/tests/determinism_tests.rs:10:    let m2 = ConstitutionalManifest::new(gen, spec, ts);
crates/amun-constitution-builder/tests/determinism_tests.rs:12:    VerificationEngine::verify_replay(&m1, &m2).expect("Manifests must be identical");
crates/amun-constitution-builder/tests/determinism_tests.rs:17:fn test_federation_determinism() {
crates/amun-constitution-builder/tests/determinism_tests.rs:1:use amun_constitution_builder::*;
crates/amun-constitution-builder/tests/determinism_tests.rs:40:    VerificationEngine::verify_replay(&f1, &f2).expect("Federation artifacts must be identical");
crates/amun-constitution-builder/tests/determinism_tests.rs:45:fn test_treaty_determinism() {
crates/amun-constitution-builder/tests/determinism_tests.rs:4:fn test_manifest_determinism() {
crates/amun-constitution-builder/tests/determinism_tests.rs:58:    VerificationEngine::verify_replay(&t1, &t2).expect("Treaties must be identical");
crates/amun-constitution-builder/tests/determinism_tests.rs:9:    let m1 = ConstitutionalManifest::new(gen.clone(), spec.clone(), ts.clone());
crates/amun-constitution/src/tests.rs:10:    assert!(c1.verify_compatible(&c2).is_ok());
crates/amun-constitution/src/tests.rs:14:fn test_quorum_transition_safe() {
crates/amun-constitution/src/tests.rs:20:    assert!(params.verify_safety().is_ok());
crates/amun-constitution/src/tests.rs:24:fn test_quorum_transition_unsafe() {
crates/amun-constitution/src/tests.rs:30:    assert!(params.verify_safety().is_err());
crates/amun-constitution/src/tests.rs:34:fn test_liveness_timeout() {
crates/amun-constitution/src/tests.rs:4:use crate::quorum_transition::*;
crates/amun-constitution/src/tests.rs:7:fn test_protocol_capacities_compatible() {
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:1:use amun_constitutional_authority_semantics::capability::{AuthorityCapability, CapabilityWitness};
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:28:fn test_delegation_chain_valid() {
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:29:    let root_key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:30:    let delegate1 = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:31:    let delegate2 = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:36:        delegate1.verifying_key_hex(),
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:3:use amun_constitutional_authority_semantics::revocation::{RevocationRegistry, RevocationWitness};
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:41:    let w1 = CapabilityWitness::sign(cap1, &root_key);
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:46:        delegate2.verifying_key_hex(),
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:4:use amun_constitutional_signing::ConstitutionalKeyPair;
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:51:    let w2 = CapabilityWitness::sign(cap2, &delegate1);
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:55:    assert!(chain.verify().is_ok());
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:59:fn test_delegation_chain_rejects_broken() {
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:60:    let root_key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:61:    let delegate1 = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:62:    let wrong_key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:67:        delegate1.verifying_key_hex(),
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:72:    let w1 = CapabilityWitness::sign(cap1, &root_key);
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:77:        delegate1.verifying_key_hex(),
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:7:fn test_capability_determinism() {
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:82:    let w2 = CapabilityWitness::sign(cap2, &wrong_key);
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:89:fn test_revocation_registry() {
crates/amun-constitutional-authority-semantics/tests/authority_tests.rs:91:    let w = RevocationWitness {
crates/amun-constitutional-authority/tests/authority_tests.rs:100:    let old = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority/tests/authority_tests.rs:101:    let new = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority/tests/authority_tests.rs:102:    let cert = ConstitutionalCertificate::new_child(
crates/amun-constitutional-authority/tests/authority_tests.rs:104:        new.verifying_key_hex(),
crates/amun-constitutional-authority/tests/authority_tests.rs:112:    assert!(KeyRotationLaw::validate_rotation(&old.verifying_key_hex(), &signed, &old).is_ok());
crates/amun-constitutional-authority/tests/authority_tests.rs:116:fn trust_anchor_is_self_signed() {
crates/amun-constitutional-authority/tests/authority_tests.rs:117:    let key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority/tests/authority_tests.rs:118:    let root = build_root(&key);
crates/amun-constitutional-authority/tests/authority_tests.rs:122:    assert!(anchor.verify().is_ok());
crates/amun-constitutional-authority/tests/authority_tests.rs:126:fn revocation_registry_is_deterministic() {
crates/amun-constitutional-authority/tests/authority_tests.rs:12:        "ConstitutionalRoot".into(),
crates/amun-constitutional-authority/tests/authority_tests.rs:17:fn build_child(
crates/amun-constitutional-authority/tests/authority_tests.rs:18:    parent: &ConstitutionalCertificate,
crates/amun-constitutional-authority/tests/authority_tests.rs:19:    key: &ConstitutionalKeyPair,
crates/amun-constitutional-authority/tests/authority_tests.rs:1:use amun_constitution_builder::digest::ArtifactDigest;
crates/amun-constitutional-authority/tests/authority_tests.rs:20:) -> ConstitutionalCertificate {
crates/amun-constitutional-authority/tests/authority_tests.rs:21:    ConstitutionalCertificate::new_child(
crates/amun-constitutional-authority/tests/authority_tests.rs:23:        key.verifying_key_hex(),
crates/amun-constitutional-authority/tests/authority_tests.rs:33:fn stable_certificate_id() {
crates/amun-constitutional-authority/tests/authority_tests.rs:34:    let key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority/tests/authority_tests.rs:35:    let a = build_root(&key);
crates/amun-constitutional-authority/tests/authority_tests.rs:36:    let b = build_root(&key);
crates/amun-constitutional-authority/tests/authority_tests.rs:3:    CertificateChain, ConstitutionalCertificate, KeyRotationLaw, RevocationRegistry, TrustAnchor,
crates/amun-constitutional-authority/tests/authority_tests.rs:42:fn valid_chain_passes_validation() {
crates/amun-constitutional-authority/tests/authority_tests.rs:43:    let root_key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority/tests/authority_tests.rs:44:    let child_key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority/tests/authority_tests.rs:45:    let root = build_root(&root_key);
crates/amun-constitutional-authority/tests/authority_tests.rs:46:    let child = build_child(&root, &child_key);
crates/amun-constitutional-authority/tests/authority_tests.rs:52:    assert!(chain.validate(&RevocationRegistry::new()).is_ok());
crates/amun-constitutional-authority/tests/authority_tests.rs:56:fn wrong_issuer_key_rejected() {
crates/amun-constitutional-authority/tests/authority_tests.rs:57:    let root_key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority/tests/authority_tests.rs:58:    let child_key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority/tests/authority_tests.rs:59:    let fake = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority/tests/authority_tests.rs:5:use amun_constitutional_signing::{ConstitutionalKeyPair, SignedArtifact};
crates/amun-constitutional-authority/tests/authority_tests.rs:60:    let root = build_root(&root_key);
crates/amun-constitutional-authority/tests/authority_tests.rs:61:    let child = build_child(&root, &child_key);
crates/amun-constitutional-authority/tests/authority_tests.rs:65:    assert!(chain.validate(&RevocationRegistry::new()).is_err());
crates/amun-constitutional-authority/tests/authority_tests.rs:69:fn broken_lineage_rejected() {
crates/amun-constitutional-authority/tests/authority_tests.rs:70:    let root_key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority/tests/authority_tests.rs:71:    let child_key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority/tests/authority_tests.rs:72:    let root = build_root(&root_key);
crates/amun-constitutional-authority/tests/authority_tests.rs:73:    let mut child = build_child(&root, &child_key);
crates/amun-constitutional-authority/tests/authority_tests.rs:7:fn build_root(key: &ConstitutionalKeyPair) -> ConstitutionalCertificate {
crates/amun-constitutional-authority/tests/authority_tests.rs:83:fn revoked_certificate_rejected() {
crates/amun-constitutional-authority/tests/authority_tests.rs:84:    let root_key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority/tests/authority_tests.rs:85:    let child_key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-authority/tests/authority_tests.rs:86:    let root = build_root(&root_key);
crates/amun-constitutional-authority/tests/authority_tests.rs:87:    let child = build_child(&root, &child_key);
crates/amun-constitutional-authority/tests/authority_tests.rs:8:    ConstitutionalCertificate::new_root(
crates/amun-constitutional-authority/tests/authority_tests.rs:95:    assert!(chain.validate(&reg).is_err());
crates/amun-constitutional-authority/tests/authority_tests.rs:99:fn rotation_with_proof_accepted() {
crates/amun-constitutional-authority/tests/authority_tests.rs:9:        key.verifying_key_hex(),
crates/amun-constitutional-block/tests/block_tests.rs:104:    let b1 = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:116:    let b2 = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:11:    ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:133:fn test_evidence_root_sensitivity() {
crates/amun-constitutional-block/tests/block_tests.rs:139:    let b1 = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:151:    let b2 = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:168:fn test_verify_block_provenance_valid() {
crates/amun-constitutional-block/tests/block_tests.rs:169:    use amun_constitutional_block::verify_block_provenance;
crates/amun-constitutional-block/tests/block_tests.rs:170:    use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-constitutional-block/tests/block_tests.rs:172:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-constitutional-block/tests/block_tests.rs:173:    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
crates/amun-constitutional-block/tests/block_tests.rs:175:    let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(
crates/amun-constitutional-block/tests/block_tests.rs:179:    let block = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:192:    assert!(verify_block_provenance(&block, &cert).is_ok());
crates/amun-constitutional-block/tests/block_tests.rs:196:fn test_verify_block_provenance_tampered_state_fails() {
crates/amun-constitutional-block/tests/block_tests.rs:197:    use amun_constitutional_block::verify_block_provenance;
crates/amun-constitutional-block/tests/block_tests.rs:198:    use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-constitutional-block/tests/block_tests.rs:1:use amun_constitutional_block::{Blockchain, ConstitutionalBlock};
crates/amun-constitutional-block/tests/block_tests.rs:200:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-constitutional-block/tests/block_tests.rs:201:    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
crates/amun-constitutional-block/tests/block_tests.rs:203:    let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(
crates/amun-constitutional-block/tests/block_tests.rs:207:    let block = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:220:    assert!(verify_block_provenance(&block, &cert).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:224:fn test_verify_block_provenance_wrong_certificate_fails() {
crates/amun-constitutional-block/tests/block_tests.rs:225:    use amun_constitutional_block::verify_block_provenance;
crates/amun-constitutional-block/tests/block_tests.rs:226:    use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-constitutional-block/tests/block_tests.rs:228:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-constitutional-block/tests/block_tests.rs:229:    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
crates/amun-constitutional-block/tests/block_tests.rs:233:    let wrong_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&[]));
crates/amun-constitutional-block/tests/block_tests.rs:235:    let block = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:248:    assert!(verify_block_provenance(&block, &cert).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:252:fn n6b_full_replay_valid() {
crates/amun-constitutional-block/tests/block_tests.rs:253:    use amun_constitutional_block::verify_full_replay;
crates/amun-constitutional-block/tests/block_tests.rs:254:    use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-constitutional-block/tests/block_tests.rs:256:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-constitutional-block/tests/block_tests.rs:257:    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
crates/amun-constitutional-block/tests/block_tests.rs:258:    rt.apply_transition(&[2u8; 32], &[0xBB; 32]);
crates/amun-constitutional-block/tests/block_tests.rs:260:    let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(
crates/amun-constitutional-block/tests/block_tests.rs:264:    let block = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:26:fn test_genesis_creation() {
crates/amun-constitutional-block/tests/block_tests.rs:277:    assert!(verify_full_replay(&block, &cert, rt.journal()).is_ok());
crates/amun-constitutional-block/tests/block_tests.rs:281:fn n6b_full_replay_tampered_journal_fails() {
crates/amun-constitutional-block/tests/block_tests.rs:282:    use amun_constitutional_block::verify_full_replay;
crates/amun-constitutional-block/tests/block_tests.rs:283:    use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-constitutional-block/tests/block_tests.rs:285:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-constitutional-block/tests/block_tests.rs:286:    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
crates/amun-constitutional-block/tests/block_tests.rs:287:    rt.apply_transition(&[2u8; 32], &[0xBB; 32]);
crates/amun-constitutional-block/tests/block_tests.rs:289:    let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(
crates/amun-constitutional-block/tests/block_tests.rs:293:    let block = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:2:use amun_constitutional_commitments::SparseMerkleTree;
crates/amun-constitutional-block/tests/block_tests.rs:307:    tampered[0].transition_hash = [0xFF; 32];
crates/amun-constitutional-block/tests/block_tests.rs:309:    assert!(verify_full_replay(&block, &cert, &tampered).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:313:fn n6b_full_replay_wrong_count_fails() {
crates/amun-constitutional-block/tests/block_tests.rs:314:    use amun_constitutional_block::verify_full_replay;
crates/amun-constitutional-block/tests/block_tests.rs:315:    use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-constitutional-block/tests/block_tests.rs:317:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-constitutional-block/tests/block_tests.rs:318:    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
crates/amun-constitutional-block/tests/block_tests.rs:319:    rt.apply_transition(&[2u8; 32], &[0xBB; 32]);
crates/amun-constitutional-block/tests/block_tests.rs:321:    let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(
crates/amun-constitutional-block/tests/block_tests.rs:325:    let block = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:339:    assert!(verify_full_replay(&block, &cert, &rt.journal()[..1]).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:33:fn test_block_hash_deterministic() {
crates/amun-constitutional-block/tests/block_tests.rs:343:fn n8_light_client_valid() {
crates/amun-constitutional-block/tests/block_tests.rs:344:    use amun_constitutional_block::verify_light_client_proof;
crates/amun-constitutional-block/tests/block_tests.rs:345:    use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-constitutional-block/tests/block_tests.rs:347:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-constitutional-block/tests/block_tests.rs:348:    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
crates/amun-constitutional-block/tests/block_tests.rs:34:    let b1 = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:351:    let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-constitutional-block/tests/block_tests.rs:354:        ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-constitutional-block/tests/block_tests.rs:356:    let block = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:369:    assert!(verify_light_client_proof(&block, &cert, &inclusion_proof).is_ok());
crates/amun-constitutional-block/tests/block_tests.rs:373:fn n8_light_client_tampered_proof_fails() {
crates/amun-constitutional-block/tests/block_tests.rs:374:    use amun_constitutional_block::verify_light_client_proof;
crates/amun-constitutional-block/tests/block_tests.rs:375:    use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-constitutional-block/tests/block_tests.rs:377:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-constitutional-block/tests/block_tests.rs:378:    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
crates/amun-constitutional-block/tests/block_tests.rs:381:    let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-constitutional-block/tests/block_tests.rs:384:        ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-constitutional-block/tests/block_tests.rs:389:    let block = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:402:    assert!(verify_light_client_proof(&block, &cert, &inclusion_proof).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:406:fn n8_light_client_wrong_certificate_fails() {
crates/amun-constitutional-block/tests/block_tests.rs:407:    use amun_constitutional_block::verify_light_client_proof;
crates/amun-constitutional-block/tests/block_tests.rs:408:    use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-constitutional-block/tests/block_tests.rs:410:    let mut rt1 = ConstitutionalStateRuntime::new();
crates/amun-constitutional-block/tests/block_tests.rs:411:    rt1.apply_transition(&[1u8; 32], &[0xAA; 32]);
crates/amun-constitutional-block/tests/block_tests.rs:414:    let mut rt2 = ConstitutionalStateRuntime::new();
crates/amun-constitutional-block/tests/block_tests.rs:415:    rt2.apply_transition(&[2u8; 32], &[0xBB; 32]);
crates/amun-constitutional-block/tests/block_tests.rs:419:    let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-constitutional-block/tests/block_tests.rs:422:        ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash1).unwrap();
crates/amun-constitutional-block/tests/block_tests.rs:424:    let block = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:438:    assert!(verify_light_client_proof(&block, &cert2, &inclusion_proof).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:442:fn n8_light_client_wrong_block_root_fails() {
crates/amun-constitutional-block/tests/block_tests.rs:443:    use amun_constitutional_block::verify_light_client_proof;
crates/amun-constitutional-block/tests/block_tests.rs:444:    use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-constitutional-block/tests/block_tests.rs:446:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-constitutional-block/tests/block_tests.rs:447:    rt.apply_transition(&[1u8; 32], &[0xAA; 32]);
crates/amun-constitutional-block/tests/block_tests.rs:452:        ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-constitutional-block/tests/block_tests.rs:454:    // Block has a different replay_certificate_root
crates/amun-constitutional-block/tests/block_tests.rs:455:    let block = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:468:    assert!(verify_light_client_proof(&block, &cert, &inclusion_proof).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:46:    let b2 = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:5:fn dummy_receipt(id: &str) -> ExecutionReceipt {
crates/amun-constitutional-block/tests/block_tests.rs:62:fn test_evidence_root_affects_hash() {
crates/amun-constitutional-block/tests/block_tests.rs:63:    let b1 = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:75:    let b2 = ConstitutionalBlock::new(
crates/amun-constitutional-block/tests/block_tests.rs:91:fn test_chain_append() {
crates/amun-constitutional-block/tests/block_tests.rs:98:fn test_evidence_root_determinism() {
crates/amun-constitutional-block/tests/block_tests.rs:9:fn make_genesis() -> ConstitutionalBlock {
crates/amun-constitutional-block/tests/finalizer_tests.rs:12:    let mut state = ConstitutionalStateRuntime::new();
crates/amun-constitutional-block/tests/finalizer_tests.rs:15:    state.apply_transition(&[1u8; 32], &[0xAA; 32]);
crates/amun-constitutional-block/tests/finalizer_tests.rs:26:    let block = BlockFinalizer::finalize(&mut chain, &log, ctx).unwrap();
crates/amun-constitutional-block/tests/finalizer_tests.rs:29:    assert!(!block.replay_certificate_root.is_empty());
crates/amun-constitutional-block/tests/finalizer_tests.rs:33:fn test_state_root_changes_block_hash() {
crates/amun-constitutional-block/tests/finalizer_tests.rs:38:    let mut state1 = ConstitutionalStateRuntime::new();
crates/amun-constitutional-block/tests/finalizer_tests.rs:39:    let mut state2 = ConstitutionalStateRuntime::new();
crates/amun-constitutional-block/tests/finalizer_tests.rs:3:    finalizer::{BlockFinalizer, FinalizationContext},
crates/amun-constitutional-block/tests/finalizer_tests.rs:40:    state1.apply_transition(&[1u8; 32], &[0xAA; 32]);
crates/amun-constitutional-block/tests/finalizer_tests.rs:41:    state2.apply_transition(&[1u8; 32], &[0xBB; 32]);
crates/amun-constitutional-block/tests/finalizer_tests.rs:60:    let b1 = BlockFinalizer::finalize(&mut chain1, &log, ctx1).unwrap();
crates/amun-constitutional-block/tests/finalizer_tests.rs:61:    let b2 = BlockFinalizer::finalize(&mut chain2, &log, ctx2).unwrap();
crates/amun-constitutional-block/tests/finalizer_tests.rs:65:    assert_ne!(b1.replay_certificate_root, b2.replay_certificate_root);
crates/amun-constitutional-block/tests/finalizer_tests.rs:6:use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-constitutional-block/tests/finalizer_tests.rs:9:fn test_commit_creates_block_with_state_root() {
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
crates/amun-constitutional-commitment/tests/determinism.rs:10:fn identical_inputs_produce_identical_constitutional_root() {
crates/amun-constitutional-commitment/tests/determinism.rs:16:    let root_a = compute_constitutional_root(id, ev, gv, ec);
crates/amun-constitutional-commitment/tests/determinism.rs:17:    let root_b = compute_constitutional_root(id, ev, gv, ec);
crates/amun-constitutional-commitment/tests/determinism.rs:1:use amun_constitutional_commitment::{
crates/amun-constitutional-commitment/tests/determinism.rs:22:fn single_byte_change_produces_different_constitutional_root() {
crates/amun-constitutional-commitment/tests/determinism.rs:28:    let root_a = compute_constitutional_root(id, ev, gv, ec);
crates/amun-constitutional-commitment/tests/determinism.rs:2:    commitment_root, compute_constitutional_root, ConstitutionalCommitment, Hash32,
crates/amun-constitutional-commitment/tests/determinism.rs:32:    let root_b = compute_constitutional_root(id2, ev, gv, ec);
crates/amun-constitutional-commitment/tests/determinism.rs:37:fn identical_commitments_produce_identical_commitment_root() {
crates/amun-constitutional-commitment/tests/determinism.rs:42:    let cr = compute_constitutional_root(id, ev, gv, ec);
crates/amun-constitutional-commitment/tests/determinism.rs:44:    let c1 = ConstitutionalCommitment {
crates/amun-constitutional-commitment/tests/determinism.rs:53:    let c2 = ConstitutionalCommitment {
crates/amun-constitutional-commitment/tests/determinism.rs:5:fn make_hash(byte: u8) -> Hash32 {
crates/amun-constitutional-commitment/tests/determinism.rs:62:    assert_eq!(commitment_root(&c1), commitment_root(&c2));
crates/amun-constitutional-commitment/tests/determinism.rs:66:fn single_byte_change_in_commitment_produces_different_commitment_root() {
crates/amun-constitutional-commitment/tests/determinism.rs:71:    let cr = compute_constitutional_root(id, ev, gv, ec);
crates/amun-constitutional-commitment/tests/determinism.rs:73:    let mut c = ConstitutionalCommitment {
crates/amun-constitutional-commitment/tests/determinism.rs:82:    let root_a = commitment_root(&c);
crates/amun-constitutional-commitment/tests/determinism.rs:84:    let root_b = commitment_root(&c);
crates/amun-constitutional-commitment/tests/economic_tree.rs:16:        issued_supply: 200_000,
crates/amun-constitutional-commitment/tests/economic_tree.rs:1:use amun_constitutional_commitment::{
crates/amun-constitutional-commitment/tests/economic_tree.rs:23:fn identical_snapshots_produce_identical_root() {
crates/amun-constitutional-commitment/tests/economic_tree.rs:2:    compute_constitutional_root, EconomicError, EconomicSnapshot, EconomicTree, Hash32,
crates/amun-constitutional-commitment/tests/economic_tree.rs:31:fn treasury_balance_change_produces_different_root() {
crates/amun-constitutional-commitment/tests/economic_tree.rs:41:fn burned_supply_change_produces_different_root() {
crates/amun-constitutional-commitment/tests/economic_tree.rs:51:fn invalid_circulating_supply_returns_error() {
crates/amun-constitutional-commitment/tests/economic_tree.rs:5:fn make_hash(byte: u8) -> Hash32 {
crates/amun-constitutional-commitment/tests/economic_tree.rs:62:fn economic_root_change_propagates_to_constitutional_root() {
crates/amun-constitutional-commitment/tests/economic_tree.rs:65:    let const_root_a = compute_constitutional_root(
crates/amun-constitutional-commitment/tests/economic_tree.rs:76:    let const_root_b = compute_constitutional_root(
crates/amun-constitutional-commitment/tests/economic_tree.rs:9:fn sample_snapshot() -> EconomicSnapshot {
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:100:    let com_a = EndBlockPipeline::execute(id, ev, gv, &snap).unwrap();
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:101:    let com_b = EndBlockPipeline::execute(id, ev, gv, &snap).unwrap();
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:107:    let state_root_a = AppHashPipeline::compute_state_root(acc, stk, gov, &com_a);
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:108:    let state_root_b = AppHashPipeline::compute_state_root(acc, stk, gov, &com_b);
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:119:fn invalid_snapshot_returns_none_from_endblock() {
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:126:    let result = EndBlockPipeline::execute(id, ev, gv, &snap);
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:14:        issued_supply: 200_000,
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:1:use amun_constitutional_commitment::{AppHashPipeline, EconomicSnapshot, EndBlockPipeline, Hash32};
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:21:fn endblock_pipeline_produces_commitment() {
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:27:    let commitment = EndBlockPipeline::execute(id, ev, gv, &snap);
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:28:    assert!(commitment.is_some());
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:29:    let c = commitment.unwrap();
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:37:fn treasury_change_changes_apphash() {
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:3:fn make_hash(byte: u8) -> Hash32 {
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:47:    let com_a = EndBlockPipeline::execute(id, ev, gv, &snap_a).unwrap();
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:48:    let com_b = EndBlockPipeline::execute(id, ev, gv, &snap_b).unwrap();
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:54:    let state_root_a = AppHashPipeline::compute_state_root(acc, stk, gov, &com_a);
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:55:    let state_root_b = AppHashPipeline::compute_state_root(acc, stk, gov, &com_b);
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:67:fn evidence_change_changes_apphash() {
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:75:    let com_a = EndBlockPipeline::execute(id, ev_a, gv, &snap).unwrap();
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:76:    let com_b = EndBlockPipeline::execute(id, ev_b, gv, &snap).unwrap();
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:7:fn sample_snapshot() -> EconomicSnapshot {
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:82:    let state_root_a = AppHashPipeline::compute_state_root(acc, stk, gov, &com_a);
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:83:    let state_root_b = AppHashPipeline::compute_state_root(acc, stk, gov, &com_b);
crates/amun-constitutional-commitment/tests/endblock_pipeline.rs:94:fn same_state_produces_same_apphash() {
crates/amun-constitutional-commitment/tests/rpc.rs:16:        issued_supply: 200_000,
crates/amun-constitutional-commitment/tests/rpc.rs:1:use amun_constitutional_commitment::{
crates/amun-constitutional-commitment/tests/rpc.rs:23:fn rpc_status_contains_all_roots() {
crates/amun-constitutional-commitment/tests/rpc.rs:29:    let commitment = EndBlockPipeline::execute(id, ev, gv, &snap).unwrap();
crates/amun-constitutional-commitment/tests/rpc.rs:2:    AppHashPipeline, ConstitutionalStatus, EconomicSnapshot, EndBlockPipeline, Hash32,
crates/amun-constitutional-commitment/tests/rpc.rs:34:    let state_root = AppHashPipeline::compute_state_root(acc, stk, gov_state, &commitment);
crates/amun-constitutional-commitment/tests/rpc.rs:37:    let status = ConstitutionalStatus::new(7990, &commitment, app_hash);
crates/amun-constitutional-commitment/tests/rpc.rs:52:fn rpc_status_reflects_economic_change() {
crates/amun-constitutional-commitment/tests/rpc.rs:5:fn make_hash(byte: u8) -> Hash32 {
crates/amun-constitutional-commitment/tests/rpc.rs:62:    let com_a = EndBlockPipeline::execute(id, ev, gv, &snap_a).unwrap();
crates/amun-constitutional-commitment/tests/rpc.rs:63:    let com_b = EndBlockPipeline::execute(id, ev, gv, &snap_b).unwrap();
crates/amun-constitutional-commitment/tests/rpc.rs:69:    let sr_a = AppHashPipeline::compute_state_root(acc, stk, gov_state, &com_a);
crates/amun-constitutional-commitment/tests/rpc.rs:70:    let sr_b = AppHashPipeline::compute_state_root(acc, stk, gov_state, &com_b);
crates/amun-constitutional-commitment/tests/rpc.rs:75:    let status_a = ConstitutionalStatus::new(7990, &com_a, ah_a);
crates/amun-constitutional-commitment/tests/rpc.rs:76:    let status_b = ConstitutionalStatus::new(7990, &com_b, ah_b);
crates/amun-constitutional-commitment/tests/rpc.rs:84:fn rpc_json_is_valid_and_parseable() {
crates/amun-constitutional-commitment/tests/rpc.rs:90:    let commitment = EndBlockPipeline::execute(id, ev, gv, &snap).unwrap();
crates/amun-constitutional-commitment/tests/rpc.rs:95:    let state_root = AppHashPipeline::compute_state_root(acc, stk, gov_state, &commitment);
crates/amun-constitutional-commitment/tests/rpc.rs:98:    let status = ConstitutionalStatus::new(7990, &commitment, app_hash);
crates/amun-constitutional-commitment/tests/rpc.rs:9:fn sample_snapshot() -> EconomicSnapshot {
crates/amun-constitutional-commitment/tests/state.rs:102:    let com_a = make_commitment(id, ev_a, gv, ec);
crates/amun-constitutional-commitment/tests/state.rs:103:    let com_b = make_commitment(id, ev_b, gv, ec);
crates/amun-constitutional-commitment/tests/state.rs:105:    let root_a = commitment_root(&com_a);
crates/amun-constitutional-commitment/tests/state.rs:106:    let root_b = commitment_root(&com_b);
crates/amun-constitutional-commitment/tests/state.rs:10:fn sample_snapshot() -> EconomicSnapshot {
crates/amun-constitutional-commitment/tests/state.rs:113:fn same_state_produces_same_commitment_root() {
crates/amun-constitutional-commitment/tests/state.rs:121:    let com_a = make_commitment(id, ev, gv, ec);
crates/amun-constitutional-commitment/tests/state.rs:122:    let com_b = make_commitment(id, ev, gv, ec);
crates/amun-constitutional-commitment/tests/state.rs:124:    let root_a = commitment_root(&com_a);
crates/amun-constitutional-commitment/tests/state.rs:125:    let root_b = commitment_root(&com_b);
crates/amun-constitutional-commitment/tests/state.rs:132:fn load_invalid_data_returns_none() {
crates/amun-constitutional-commitment/tests/state.rs:133:    assert!(ConstitutionalState::load(&[0u8; 10]).is_none());
crates/amun-constitutional-commitment/tests/state.rs:134:    assert!(ConstitutionalState::load(&[0u8; 200]).is_none());
crates/amun-constitutional-commitment/tests/state.rs:135:    assert!(ConstitutionalState::load(&[]).is_none());
crates/amun-constitutional-commitment/tests/state.rs:17:        issued_supply: 200_000,
crates/amun-constitutional-commitment/tests/state.rs:1:use amun_constitutional_commitment::{
crates/amun-constitutional-commitment/tests/state.rs:23:fn make_commitment(id: Hash32, ev: Hash32, gv: Hash32, ec: Hash32) -> ConstitutionalCommitment {
crates/amun-constitutional-commitment/tests/state.rs:24:    let cr = compute_constitutional_root(id, ev, gv, ec);
crates/amun-constitutional-commitment/tests/state.rs:25:    ConstitutionalCommitment {
crates/amun-constitutional-commitment/tests/state.rs:2:    commitment_root, compute_constitutional_root, ConstitutionalCommitment, ConstitutionalState,
crates/amun-constitutional-commitment/tests/state.rs:36:fn save_and_load_roundtrip() {
crates/amun-constitutional-commitment/tests/state.rs:39:    let commitment = make_commitment(make_hash(0x11), make_hash(0x22), make_hash(0x33), ec);
crates/amun-constitutional-commitment/tests/state.rs:41:    let bytes = ConstitutionalState::save(&commitment);
crates/amun-constitutional-commitment/tests/state.rs:42:    let loaded = ConstitutionalState::load(&bytes).unwrap();
crates/amun-constitutional-commitment/tests/state.rs:44:    assert_eq!(commitment.version, loaded.version);
crates/amun-constitutional-commitment/tests/state.rs:45:    assert_eq!(commitment.identity_root, loaded.identity_root);
crates/amun-constitutional-commitment/tests/state.rs:46:    assert_eq!(commitment.evidence_root, loaded.evidence_root);
crates/amun-constitutional-commitment/tests/state.rs:47:    assert_eq!(commitment.governance_root, loaded.governance_root);
crates/amun-constitutional-commitment/tests/state.rs:48:    assert_eq!(commitment.economic_root, loaded.economic_root);
crates/amun-constitutional-commitment/tests/state.rs:49:    assert_eq!(commitment.constitutional_root, loaded.constitutional_root);
crates/amun-constitutional-commitment/tests/state.rs:53:fn commitment_root_from_stored_bytes() {
crates/amun-constitutional-commitment/tests/state.rs:56:    let commitment = make_commitment(make_hash(0x11), make_hash(0x22), make_hash(0x33), ec);
crates/amun-constitutional-commitment/tests/state.rs:58:    let expected_root = commitment_root(&commitment);
crates/amun-constitutional-commitment/tests/state.rs:60:    let bytes = ConstitutionalState::save(&commitment);
crates/amun-constitutional-commitment/tests/state.rs:61:    let loaded = ConstitutionalState::load(&bytes).unwrap();
crates/amun-constitutional-commitment/tests/state.rs:62:    let stored_root = commitment_root(&loaded);
crates/amun-constitutional-commitment/tests/state.rs:68:fn state_root_changes_when_treasury_changes() {
crates/amun-constitutional-commitment/tests/state.rs:6:fn make_hash(byte: u8) -> Hash32 {
crates/amun-constitutional-commitment/tests/state.rs:81:    let com_a = make_commitment(id, ev, gv, ec_a);
crates/amun-constitutional-commitment/tests/state.rs:82:    let com_b = make_commitment(id, ev, gv, ec_b);
crates/amun-constitutional-commitment/tests/state.rs:84:    let root_a = commitment_root(&com_a);
crates/amun-constitutional-commitment/tests/state.rs:85:    let root_b = commitment_root(&com_b);
crates/amun-constitutional-commitment/tests/state.rs:93:fn state_root_changes_when_evidence_changes() {
crates/amun-constitutional-commitment/tests/verify.rs:105:    let commitment = EndBlockPipeline::execute(id, ev, gv, &snap).unwrap();
crates/amun-constitutional-commitment/tests/verify.rs:110:    let state_root = AppHashPipeline::compute_state_root(acc, stk, gov_state, &commitment);
crates/amun-constitutional-commitment/tests/verify.rs:112:    let stored_commitment = commitment_root(&commitment);
crates/amun-constitutional-commitment/tests/verify.rs:114:    let result = Verifier::verify(
crates/amun-constitutional-commitment/tests/verify.rs:115:        commitment.identity_root,
crates/amun-constitutional-commitment/tests/verify.rs:116:        commitment.evidence_root,
crates/amun-constitutional-commitment/tests/verify.rs:117:        commitment.governance_root,
crates/amun-constitutional-commitment/tests/verify.rs:118:        commitment.economic_root,
crates/amun-constitutional-commitment/tests/verify.rs:119:        commitment.constitutional_root,
crates/amun-constitutional-commitment/tests/verify.rs:120:        stored_commitment,
crates/amun-constitutional-commitment/tests/verify.rs:123:    assert!(Verifier::verified(&result));
crates/amun-constitutional-commitment/tests/verify.rs:125:        result.recomputed_constitutional_root,
crates/amun-constitutional-commitment/tests/verify.rs:126:        commitment.constitutional_root
crates/amun-constitutional-commitment/tests/verify.rs:128:    assert_eq!(result.recomputed_commitment_root, stored_commitment);
crates/amun-constitutional-commitment/tests/verify.rs:131:        amun_constitutional_commitment::ConstitutionalStatus::new(7990, &commitment, app_hash);
crates/amun-constitutional-commitment/tests/verify.rs:16:        issued_supply: 200_000,
crates/amun-constitutional-commitment/tests/verify.rs:1:use amun_constitutional_commitment::{
crates/amun-constitutional-commitment/tests/verify.rs:23:fn verifier_confirms_valid_roots() {
crates/amun-constitutional-commitment/tests/verify.rs:29:    let commitment = EndBlockPipeline::execute(id, ev, gv, &snap).unwrap();
crates/amun-constitutional-commitment/tests/verify.rs:2:    roots::commitment_root, AppHashPipeline, EconomicSnapshot, EndBlockPipeline, Hash32, Verifier,
crates/amun-constitutional-commitment/tests/verify.rs:30:    let stored_commitment = commitment_root(&commitment);
crates/amun-constitutional-commitment/tests/verify.rs:32:    let result = Verifier::verify(
crates/amun-constitutional-commitment/tests/verify.rs:36:        commitment.economic_root,
crates/amun-constitutional-commitment/tests/verify.rs:37:        commitment.constitutional_root,
crates/amun-constitutional-commitment/tests/verify.rs:38:        stored_commitment,
crates/amun-constitutional-commitment/tests/verify.rs:42:    assert!(result.commitment_root_match);
crates/amun-constitutional-commitment/tests/verify.rs:43:    assert!(Verifier::verified(&result));
crates/amun-constitutional-commitment/tests/verify.rs:47:fn verifier_rejects_tampered_constitutional_root() {
crates/amun-constitutional-commitment/tests/verify.rs:53:    let commitment = EndBlockPipeline::execute(id, ev, gv, &snap).unwrap();
crates/amun-constitutional-commitment/tests/verify.rs:54:    let stored_commitment = commitment_root(&commitment);
crates/amun-constitutional-commitment/tests/verify.rs:56:    let mut tampered = commitment.constitutional_root;
crates/amun-constitutional-commitment/tests/verify.rs:59:    let result = Verifier::verify(
crates/amun-constitutional-commitment/tests/verify.rs:5:fn make_hash(byte: u8) -> Hash32 {
crates/amun-constitutional-commitment/tests/verify.rs:63:        commitment.economic_root,
crates/amun-constitutional-commitment/tests/verify.rs:65:        stored_commitment,
crates/amun-constitutional-commitment/tests/verify.rs:69:    assert!(!Verifier::verified(&result));
crates/amun-constitutional-commitment/tests/verify.rs:73:fn verifier_rejects_tampered_economic_root() {
crates/amun-constitutional-commitment/tests/verify.rs:79:    let commitment = EndBlockPipeline::execute(id, ev, gv, &snap).unwrap();
crates/amun-constitutional-commitment/tests/verify.rs:80:    let stored_commitment = commitment_root(&commitment);
crates/amun-constitutional-commitment/tests/verify.rs:82:    let mut tampered_economic = commitment.economic_root;
crates/amun-constitutional-commitment/tests/verify.rs:85:    let result = Verifier::verify(
crates/amun-constitutional-commitment/tests/verify.rs:90:        commitment.constitutional_root,
crates/amun-constitutional-commitment/tests/verify.rs:91:        stored_commitment,
crates/amun-constitutional-commitment/tests/verify.rs:95:    assert!(!Verifier::verified(&result));
crates/amun-constitutional-commitment/tests/verify.rs:99:fn explorer_verification_flow_end_to_end() {
crates/amun-constitutional-commitment/tests/verify.rs:9:fn sample_snapshot() -> EconomicSnapshot {
crates/amun-constitutional-commitments/tests/commitment_tests.rs:16:fn domain_roots_are_deterministic() {
crates/amun-constitutional-commitments/tests/commitment_tests.rs:17:    let (s1, g1, e1) = build_domain_roots();
crates/amun-constitutional-commitments/tests/commitment_tests.rs:18:    let (s2, g2, e2) = build_domain_roots();
crates/amun-constitutional-commitments/tests/commitment_tests.rs:1:use amun_constitutional_commitments::SparseMerkleTree;
crates/amun-constitutional-commitments/tests/commitment_tests.rs:25:fn different_domains_produce_different_roots() {
crates/amun-constitutional-commitments/tests/commitment_tests.rs:26:    let (s, g, e) = build_domain_roots();
crates/amun-constitutional-commitments/tests/commitment_tests.rs:33:fn domain_proofs_are_independent() {
crates/amun-constitutional-commitments/tests/commitment_tests.rs:38:    assert!(state_tree.verify(&state_root, &proof));
crates/amun-constitutional-commitments/tests/commitment_tests.rs:3:fn build_domain_roots() -> ([u8; 32], [u8; 32], [u8; 32]) {
crates/amun-constitutional-commitments/tests/commitment_tests.rs:43:    assert!(!gov_tree.verify(&gov_root, &proof));
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:204:        assert!(evidence.replay_deterministic);
crates/amun-constitutional-enforcement/src/evidence_providers.rs:121:        assert!(evidence.transition_valid);
crates/amun-constitutional-enforcement/src/evidence_providers.rs:127:        assert!(evidence.replay_deterministic);
crates/amun-constitutional-enforcement/src/evidence_providers.rs:133:        assert!(!evidence.replay_deterministic);
crates/amun-constitutional-enforcement/src/evidence_providers.rs:149:        assert!(evidence.replay_deterministic);
crates/amun-constitutional-enforcement/src/evidence_records.rs:233:        assert!(record.verify());
crates/amun-constitutional-enforcement/src/evidence_records.rs:253:        assert!(!record.verify());
crates/amun-constitutional-enforcement/src/lib.rs:252:                assert_eq!(violations[0].law, ConstitutionalLaw::StateRootIntegrity);
crates/amun-constitutional-enforcement/src/proof_engine.rs:254:    fn n124_constitutional_block_passes() {
crates/amun-constitutional-enforcement/src/proof_engine.rs:276:        assert_eq!(verdict, ConstitutionalVerdict::Constitutional);
crates/amun-constitutional-enforcement/src/state_transition.rs:138:        assert_eq!(verdict, ConstitutionalVerdict::Constitutional);
crates/amun-constitutional-evidence/src/lib.rs:102:    fn test_evidence_fails_for_insufficient_weight() {
crates/amun-constitutional-evidence/src/lib.rs:107:    fn test_evidence_passes_for_valid_context() {
crates/amun-constitutional-evidence/src/lib.rs:43:/// 2. **Constitutional Membership (Exclusion):** The QC must be attested by a
crates/amun-constitutional-evidence/src/lib.rs:92:    fn test_evidence_fails_for_wrong_epoch() {
crates/amun-constitutional-evidence/src/lib.rs:97:    fn test_evidence_fails_for_foreign_validator() {
crates/amun-constitutional-governance/tests/governance_tests.rs:105:fn tally_is_deterministic() {
crates/amun-constitutional-governance/tests/governance_tests.rs:10:fn capability_has_stable_id() {
crates/amun-constitutional-governance/tests/governance_tests.rs:11:    let key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-governance/tests/governance_tests.rs:134:    let tally = Tally::compute(&proposal, &ballots, &eligible);
crates/amun-constitutional-governance/tests/governance_tests.rs:140:    let tally2 = Tally::compute(&proposal, &ballots, &eligible);
crates/amun-constitutional-governance/tests/governance_tests.rs:145:fn amendment_lifecycle_progression() {
crates/amun-constitutional-governance/tests/governance_tests.rs:15:        key.verifying_key_hex(),
crates/amun-constitutional-governance/tests/governance_tests.rs:1:use amun_constitution_builder::digest::ArtifactDigest;
crates/amun-constitutional-governance/tests/governance_tests.rs:23:        key.verifying_key_hex(),
crates/amun-constitutional-governance/tests/governance_tests.rs:33:fn valid_delegation_chain_accepted() {
crates/amun-constitutional-governance/tests/governance_tests.rs:34:    let root_key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-governance/tests/governance_tests.rs:35:    let delegate1_key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-governance/tests/governance_tests.rs:36:    let delegate2_key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-governance/tests/governance_tests.rs:41:        delegate1_key.verifying_key_hex(),
crates/amun-constitutional-governance/tests/governance_tests.rs:51:        delegate2_key.verifying_key_hex(),
crates/amun-constitutional-governance/tests/governance_tests.rs:59:    assert!(delegation::verify_delegation_chain(&chain, &root_key.verifying_key_hex()).is_ok());
crates/amun-constitutional-governance/tests/governance_tests.rs:63:fn delegation_chain_rejects_unsigned() {
crates/amun-constitutional-governance/tests/governance_tests.rs:64:    let root_key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-governance/tests/governance_tests.rs:65:    let fake_key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-governance/tests/governance_tests.rs:77:    assert!(delegation::verify_delegation_chain(&chain, &root_key.verifying_key_hex()).is_err());
crates/amun-constitutional-governance/tests/governance_tests.rs:7:use amun_constitutional_signing::ConstitutionalKeyPair;
crates/amun-constitutional-governance/tests/governance_tests.rs:81:fn simple_majority_passes() {
crates/amun-constitutional-governance/tests/governance_tests.rs:88:fn super_majority_two_thirds() {
crates/amun-constitutional-governance/tests/governance_tests.rs:95:fn min_participants_not_met() {
crates/amun-constitutional-integration/src/lib.rs:437:                evidence_type: VKEvidenceType::TestResult,
crates/amun-constitutional-kernel/tests/kernel_tests.rs:103:fn test_amendment_activation() {
crates/amun-constitutional-kernel/tests/kernel_tests.rs:104:    let (ctx, capabilities, _) = build_test_context();
crates/amun-constitutional-kernel/tests/kernel_tests.rs:105:    let mut machine = ConstitutionalStateMachine::new();
crates/amun-constitutional-kernel/tests/kernel_tests.rs:12:fn build_test_context() -> (
crates/amun-constitutional-kernel/tests/kernel_tests.rs:135:fn test_proposal_ballot_integration() {
crates/amun-constitutional-kernel/tests/kernel_tests.rs:137:    let (ctx, capabilities, _) = build_test_context();
crates/amun-constitutional-kernel/tests/kernel_tests.rs:138:    let mut machine = ConstitutionalStateMachine::new();
crates/amun-constitutional-kernel/tests/kernel_tests.rs:144:        "kernel transition".into(),
crates/amun-constitutional-kernel/tests/kernel_tests.rs:15:    Vec<ConstitutionalCertificate>,
crates/amun-constitutional-kernel/tests/kernel_tests.rs:178:    let tally = Tally::compute(&proposal, &ballots, &eligible);
crates/amun-constitutional-kernel/tests/kernel_tests.rs:17:    let key = ConstitutionalKeyPair::generate();
crates/amun-constitutional-kernel/tests/kernel_tests.rs:18:    let root_cert = ConstitutionalCertificate::new_root(
crates/amun-constitutional-kernel/tests/kernel_tests.rs:19:        key.verifying_key_hex(),
crates/amun-constitutional-kernel/tests/kernel_tests.rs:1:use amun_constitutional_authority::ConstitutionalCertificate;
crates/amun-constitutional-kernel/tests/kernel_tests.rs:22:        "ConstitutionalRoot".into(),
crates/amun-constitutional-kernel/tests/kernel_tests.rs:30:            key.verifying_key_hex(),
crates/amun-constitutional-kernel/tests/kernel_tests.rs:38:            key.verifying_key_hex(),
crates/amun-constitutional-kernel/tests/kernel_tests.rs:51:fn test_state_machine_determinism() {
crates/amun-constitutional-kernel/tests/kernel_tests.rs:52:    let (ctx, capabilities, _) = build_test_context();
crates/amun-constitutional-kernel/tests/kernel_tests.rs:53:    let mut machine = ConstitutionalStateMachine::new();
crates/amun-constitutional-kernel/tests/kernel_tests.rs:58:        .transition(
crates/amun-constitutional-kernel/tests/kernel_tests.rs:65:        .expect("transition should succeed");
crates/amun-constitutional-kernel/tests/kernel_tests.rs:67:    // Replay must produce identical result
crates/amun-constitutional-kernel/tests/kernel_tests.rs:68:    let mut machine2 = ConstitutionalStateMachine::new();
crates/amun-constitutional-kernel/tests/kernel_tests.rs:70:        .transition(
crates/amun-constitutional-kernel/tests/kernel_tests.rs:77:        .expect("transition should succeed");
crates/amun-constitutional-kernel/tests/kernel_tests.rs:7:    AmendmentActivator, ConstitutionalStateMachine, ExecutionContext,
crates/amun-constitutional-kernel/tests/kernel_tests.rs:85:fn test_unauthorised_action_rejected() {
crates/amun-constitutional-kernel/tests/kernel_tests.rs:86:    let (ctx, _capabilities, _) = build_test_context();
crates/amun-constitutional-kernel/tests/kernel_tests.rs:87:    let mut machine = ConstitutionalStateMachine::new();
crates/amun-constitutional-kernel/tests/kernel_tests.rs:92:    let result = machine.transition(
crates/amun-constitutional-kernel/tests/kernel_tests.rs:9:use amun_constitutional_signing::ConstitutionalKeyPair;
crates/amun-constitutional-proof/src/evidence_type.rs:7:    ExperimentalEvidence,
crates/amun-constitutional-proof/src/lib.rs:1042:        assert_eq!(ev.status, EvidenceStatus::Verified);
crates/amun-constitutional-proof/src/lib.rs:1074:    // --- N47.3-S1: EvidenceArchive tests ---
crates/amun-constitutional-proof/src/lib.rs:110:        assert_eq!(id.namespace(), ObligationNamespace::Safety);
crates/amun-constitutional-proof/src/lib.rs:1165:        assert!(archive.verify("EV-REJ").is_err());
crates/amun-constitutional-proof/src/lib.rs:117:        assert_eq!(id, ObligationId::new(ObligationNamespace::Safety, 1));
crates/amun-constitutional-proof/src/lib.rs:123:        assert_eq!(id.namespace(), ObligationNamespace::Replay);
crates/amun-constitutional-proof/src/lib.rs:1315:        assert!(!EvidenceArchive::is_admissible(&collected));
crates/amun-constitutional-proof/src/lib.rs:1320:        assert!(EvidenceArchive::is_admissible(&verified));
crates/amun-constitutional-proof/src/lib.rs:1325:        assert!(EvidenceArchive::is_admissible(&archived));
crates/amun-constitutional-proof/src/lib.rs:1329:        assert!(!EvidenceArchive::is_admissible(&rejected));
crates/amun-constitutional-proof/src/lib.rs:1357:        assert!(ArticleIIICertificate::issue(&archive, 1000).is_none());
crates/amun-constitutional-proof/src/lib.rs:1415:        assert!(md.contains("N47 Constitutional Validation Report"));
crates/amun-constitutional-proof/src/lib.rs:142:            Err(RegistryError::InvalidObligationIdFormat(s)) => assert_eq!(s, "SAFETY"),
crates/amun-constitutional-proof/src/lib.rs:1476:        assert!(pkg.verify());
crates/amun-constitutional-proof/src/lib.rs:1522:        assert!(pkg.verify());
crates/amun-constitutional-proof/src/lib.rs:1526:    // --- N47.6: Constitutional Certification tests ---
crates/amun-constitutional-proof/src/lib.rs:1529:    fn n47_6_certify_pass() {
crates/amun-constitutional-proof/src/lib.rs:173:    // --- ProofObligation tests (S1) ---
crates/amun-constitutional-proof/src/lib.rs:187:        assert_eq!(obl.kind, ObligationKind::Primary);
crates/amun-constitutional-proof/src/lib.rs:188:        assert_eq!(obl.severity, ObligationSeverity::Critical);
crates/amun-constitutional-proof/src/lib.rs:191:        assert_eq!(obl.status, ObligationStatus::Active);
crates/amun-constitutional-proof/src/lib.rs:209:        assert_eq!(obl.kind, ObligationKind::Derived);
crates/amun-constitutional-proof/src/lib.rs:226:        assert_eq!(obl.status, ObligationStatus::Frozen);
crates/amun-constitutional-proof/src/lib.rs:325:        assert!(graph.validate_derived_terminate_in_primary(&kinds).is_ok());
crates/amun-constitutional-proof/src/lib.rs:350:    // --- ObligationRegistry tests (S3) ---
crates/amun-constitutional-proof/src/lib.rs:435:        assert_eq!(reg.by_severity(ObligationSeverity::Critical).len(), 1);
crates/amun-constitutional-proof/src/lib.rs:436:        assert_eq!(reg.by_severity(ObligationSeverity::Minor).len(), 1);
crates/amun-constitutional-proof/src/lib.rs:437:        assert_eq!(reg.by_severity(ObligationSeverity::Major).len(), 0);
crates/amun-constitutional-proof/src/lib.rs:510:        assert!(ArticleICertificate::issue(&reg, 1000).is_none());
crates/amun-constitutional-proof/src/lib.rs:529:        assert!(ArticleICertificate::issue(&reg, 1000).is_none());
crates/amun-constitutional-proof/src/lib.rs:539:        assert_eq!(result.status, ObligationResultStatus::Satisfied);
crates/amun-constitutional-proof/src/lib.rs:549:        assert_eq!(result.status, ObligationResultStatus::Failed);
crates/amun-constitutional-proof/src/lib.rs:557:        assert_eq!(result.status, ObligationResultStatus::Inconclusive);
crates/amun-constitutional-proof/src/lib.rs:562:    fn n47_2_s0_verdict_pass() {
crates/amun-constitutional-proof/src/lib.rs:585:    // --- N47.2-S1: ConstitutionalVerdict tests ---
crates/amun-constitutional-proof/src/lib.rs:612:        assert!(verdict.verify());
crates/amun-constitutional-proof/src/lib.rs:639:            VerdictResult::ConditionalPass(vec!["Replay-2 failed".into()]),
crates/amun-constitutional-proof/src/lib.rs:730:        assert!(parsed.verify());
crates/amun-constitutional-proof/src/lib.rs:799:    fn n47_2_s2_conditional_pass_on_one_major() {
crates/amun-constitutional-proof/src/lib.rs:826:    fn n47_2_s2_pass_with_minor_failures() {
crates/amun-constitutional-proof/src/lib.rs:853:    fn n47_2_s2_pass_with_advisory_failures() {
crates/amun-constitutional-proof/src/lib.rs:880:    fn n47_2_s2_pass_all_satisfied() {
crates/amun-constitutional-proof/src/lib.rs:979:    // --- N47.3-S0: Evidence Foundation Types tests ---
crates/amun-constitutional-proof/src/lib.rs:994:        assert_eq!(ev.status, EvidenceStatus::Collected);
crates/amun-constitutional-quarantine/src/pipeline.rs:34:    pub fn verify_physics(&mut self, snapshot_root: [u8; 32], passed: bool) {
crates/amun-constitutional-quarantine/src/pipeline.rs:47:    pub fn verify_replay(&mut self, snapshot_root: [u8; 32], passed: bool) {
crates/amun-constitutional-quarantine/src/pipeline.rs:54:            record.replay_verified = passed;
crates/amun-constitutional-quarantine/src/pipeline.rs:61:    pub fn verify_lineage(&mut self, snapshot_root: [u8; 32], passed: bool) {
crates/amun-constitutional-runtime/src/block_validator.rs:101:    fn n51_valid_block_all_transactions_pass() {
crates/amun-constitutional-runtime/src/block_validator.rs:129:        assert_eq!(result.committed, 5);
crates/amun-constitutional-runtime/src/block_validator.rs:178:        assert_eq!(result.committed, 5);
crates/amun-constitutional-runtime/src/block_validator.rs:212:        assert_eq!(result.state_root, registry.compute_state_root());
crates/amun-constitutional-runtime/src/block_validator.rs:22:/// Validates that every transaction in a block passes PCCV.
crates/amun-constitutional-runtime/src/block_validator.rs:28:    /// The block is valid iff ALL transactions pass PCCV.
crates/amun-constitutional-runtime/src/certificate_chain.rs:158:        assert!(chain.verify_chain());
crates/amun-constitutional-runtime/src/certificate_chain.rs:186:        assert!(!chain.verify_chain());
crates/amun-constitutional-runtime/src/certificate_chain.rs:210:        assert!(!chain.verify_chain());
crates/amun-constitutional-runtime/src/finality_certificate.rs:193:        assert!(cert.verify());
crates/amun-constitutional-runtime/src/finality_certificate.rs:227:        assert!(!cert.verify());
crates/amun-constitutional-runtime/src/history_root.rs:181:        assert!(root.verify_chain(&chain));
crates/amun-constitutional-runtime/src/history_root.rs:196:        assert!(!root.verify_chain(&tampered_chain));
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:269:                assert!(pccv_verified, "PCCV must pass for valid execution");
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:280:        // so PCCV passes trivially. Full illegal execution testing
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:97:        let passed = VMKernel::verify(&mut buffer, registry);
crates/amun-constitutional-semantics/src/lib.rs:114:    #[test] fn test_commitment_verification() { let e = vec![mk(1,[0x01;32],[0x00;32]), mk(2,[0x02;32],[0x01;32])]; let c = TranscriptCommitment::new_sequential(&e, [0xBB;32]); assert!(c.verify_events(&e)); }
crates/amun-constitutional-semantics/src/lib.rs:115:    #[test] fn test_commitment_detects_tamper() { let e1 = mk(1,[0x01;32],[0x00;32]); let e2 = mk(2,[0x02;32],[0x01;32]); let e3 = mk(3,[0x03;32],[0x02;32]); let c = TranscriptCommitment::new_sequential(&vec![e1.clone(), e2], [0xBB;32]); assert!(!c.verify_events(&vec![e1, e3])); }
crates/amun-constitutional-semantics/src/lib.rs:116:    #[test] fn test_finality_progression() { assert!(EventFinality::Tentative < EventFinality::Finalized); assert!(EventFinality::Finalized < EventFinality::ReplayCertified); }
crates/amun-constitutional-semantics/src/lib.rs:117:    #[test] fn test_finality_replay_safety() { assert!(!EventFinality::Tentative.is_replay_safe()); assert!(EventFinality::Finalized.is_replay_safe()); }
crates/amun-constitutional-semantics/src/lib.rs:118:    #[test] fn test_witness_normalization_deterministic() { let w = vec![(ReplayDomain::Consensus,2,[0x02;32]),(ReplayDomain::Consensus,1,[0x01;32])]; assert_eq!(WitnessNormalization::normalize(&w).normalization_root, WitnessNormalization::normalize(&w).normalization_root); }
crates/amun-constitutional-semantics/src/lib.rs:120:    #[test] fn test_replay_policy() { assert!(ReplayPolicy::CONSENSUS_AUTHORITATIVE.replay_required); assert!(!ReplayPolicy::EPHEMERAL.replay_required); }
crates/amun-constitutional-semantics/src/lib.rs:121:    #[test] fn test_authority_binding() { let b = AuthorityBinding { authority: EventAuthority::Authoritative, authority_set_root: [0xAA;32], authority_epoch: [0xBB;32], authority_proof: AuthorityProof::SingleSignature { validator_id: 1, signature: [0;64] } }; assert!(b.verify_binding(EventAuthority::Authoritative)); assert!(!b.verify_binding(EventAuthority::Derived)); }
crates/amun-constitutional-signing/tests/signing_tests.rs:13:    let keypair = ConstitutionalKeyPair::generate();
crates/amun-constitutional-signing/tests/signing_tests.rs:15:    assert!(signed.verify().is_ok());
crates/amun-constitutional-signing/tests/signing_tests.rs:19:fn test_sign_and_verify_federation() {
crates/amun-constitutional-signing/tests/signing_tests.rs:1:use amun_constitution_builder::{
crates/amun-constitutional-signing/tests/signing_tests.rs:2:    digest::ArtifactDigest, ConstitutionalManifest, FederationArtifact, TreatyArtifact,
crates/amun-constitutional-signing/tests/signing_tests.rs:30:    let keypair = ConstitutionalKeyPair::generate();
crates/amun-constitutional-signing/tests/signing_tests.rs:32:    assert!(signed.verify().is_ok());
crates/amun-constitutional-signing/tests/signing_tests.rs:36:fn test_sign_and_verify_treaty() {
crates/amun-constitutional-signing/tests/signing_tests.rs:42:    let keypair = ConstitutionalKeyPair::generate();
crates/amun-constitutional-signing/tests/signing_tests.rs:44:    assert!(signed.verify().is_ok());
crates/amun-constitutional-signing/tests/signing_tests.rs:48:fn test_digest_determinism_with_domain_separation() {
crates/amun-constitutional-signing/tests/signing_tests.rs:49:    let m1 = ConstitutionalManifest::new("gen".into(), "spec".into(), "ts".into());
crates/amun-constitutional-signing/tests/signing_tests.rs:4:use amun_constitutional_signing::{ConstitutionalKeyPair, SignedArtifact};
crates/amun-constitutional-signing/tests/signing_tests.rs:50:    let m2 = ConstitutionalManifest::new("gen".into(), "spec".into(), "ts".into());
crates/amun-constitutional-signing/tests/signing_tests.rs:59:fn test_signature_public_verifiability() {
crates/amun-constitutional-signing/tests/signing_tests.rs:60:    let manifest = ConstitutionalManifest::new("gen".into(), "spec".into(), "ts".into());
crates/amun-constitutional-signing/tests/signing_tests.rs:61:    let keypair = ConstitutionalKeyPair::generate();
crates/amun-constitutional-signing/tests/signing_tests.rs:65:    assert!(signed.verify().is_ok());
crates/amun-constitutional-signing/tests/signing_tests.rs:67:    // Tampering with the artifact must invalidate the signature.
crates/amun-constitutional-signing/tests/signing_tests.rs:74:    assert!(tampered_signed.verify().is_err());
crates/amun-constitutional-signing/tests/signing_tests.rs:7:fn test_sign_and_verify_manifest() {
crates/amun-constitutional-signing/tests/signing_tests.rs:8:    let manifest = ConstitutionalManifest::new(
crates/amun-constitutional-sim/src/protocol.rs:4:pub struct ExperimentalProtocol {
crates/amun-constitutional-sim/src/runner.rs:22:    pub fn run(protocol: &ExperimentalProtocol) -> Vec<SimulationStep> {
crates/amun-constitutional-sim/tests/phase_116_engineering_map.rs:31:    println!("\n=== Phase 116C: Constitutional Engineering Map ===");
crates/amun-constitutional-sim/tests/phase_116_engineering_map.rs:4:fn test_engineering_map() {
crates/amun-constitutional-sim/tests/phase_116_interactions.rs:5:fn test_interaction_effects() {
crates/amun-constitutional-sim/tests/phase_116_main_effects.rs:4:fn test_main_effects() {
crates/amun-constitutional-sim/tests/quick_validation.rs:13:        let (_f1, _f2, _f3, _f4, _f5, e) = EffectivenessEngine::debug_claim(claim, &state);
crates/amun-constitutional-sim/tests/quick_validation.rs:1:use amun_constitutional_sim::{EffectivenessEngine, SimulationState};
crates/amun-constitutional-sim/tests/quick_validation.rs:4:fn test_effectiveness_bounds() {
crates/amun-constitutional-sim/tests/quick_validation.rs:6:    EffectivenessEngine::update_all(&mut state);
crates/amun-constitutional-sim/tests/recalibrated_n100.rs:4:fn test_n100_validation_recalibrated() {
crates/amun-constitutional-sim/tests/recalibrated_ratio.rs:4:fn test_ratio_scan_recalibrated() {
crates/amun-constitutional-sim/tests/recalibrated_tests.rs:4:fn test_factor_isolation_recalibrated() {
crates/amun-constitutional-sim/tests/recognition_erosion.rs:49:fn test_saturation_ratio_scan_n100() {
crates/amun-constitutional-sim/tests/recognition_erosion.rs:4:fn test_saturation_at_n100() {
crates/amun-constitutional-state/src/lib.rs:103:    fn test_set_and_get() {
crates/amun-constitutional-state/src/lib.rs:110:    fn test_delete() {
crates/amun-constitutional-state/src/lib.rs:118:    fn test_state_root_determinism() {
crates/amun-constitutional-state/src/lib.rs:127:    fn test_state_root_sensitivity() {
crates/amun-constitutional-state/src/lib.rs:136:    fn test_delete_returns_empty_root() {
crates/amun-constitutional-state/src/lib.rs:145:    fn test_canonical_key() {
crates/amun-constitutional-state/src/lib.rs:148:        assert!(key.starts_with(b"transition/"));
crates/amun-constitutional-state/src/lib.rs:152:    fn test_transition_changes_state_root() {
crates/amun-constitutional-state/src/lib.rs:160:    fn test_same_transition_same_root() {
crates/amun-constitutional-state/src/lib.rs:169:    fn test_replay_produces_same_root() {
crates/amun-constitutional-state/src/lib.rs:181:    fn test_replay_order_independence() {
crates/amun-constitutional-state/src/lib.rs:249:    fn test_replay_certificate_valid() {
crates/amun-constitutional-state/src/lib.rs:256:        assert!(cert.verify(rt.journal()));
crates/amun-constitutional-state/src/lib.rs:260:    fn test_replay_certificate_detects_tampering() {
crates/amun-constitutional-state/src/lib.rs:269:        assert!(!cert.verify(&tampered));
crates/amun-constitutional-state/src/lib.rs:472:        assert!(proof.verify());
crates/amun-constitutional-state/src/lib.rs:492:        assert!(!proof.verify());
crates/amun-constitutional-verifier/src/verifier.rs:145:        assert!(verify_qc(&qc, &set));
crates/amun-constitutional-verifier/src/verifier.rs:146:        assert!(verify_vote_uniqueness(&qc));
crates/amun-constitutional-verifier/src/verifier.rs:99:    fn test_verify_qc() {
crates/amun-constitutional/src/architectural_invariants.rs:106:/// deterministic replay lineage and attested transcript scope,
crates/amun-constitutional/src/architectural_invariants.rs:109:/// CONSEQUENCE: State roots are attestations of replay outcomes,
crates/amun-constitutional/src/architectural_invariants.rs:24:/// A ReplayCertificate attests replay ADMISSIBILITY, not
crates/amun-constitutional/src/architectural_invariants.rs:32:    "Certificate attests admissibility, not finality. Finality is above the kernel.";
crates/amun-constitutional/src/architectural_invariants.rs:421:    fn test_all_invariants_documented() {
crates/amun-constitutional/src/architectural_invariants.rs:424:            assert!(!invariant.is_empty(), "Invariant must be documented");
crates/amun-constitutional/src/architectural_invariants.rs:429:    fn test_no_duplicate_invariants() {
crates/amun-constitutional/src/architectural_invariants.rs:439:    fn test_invariant_count() {
crates/amun-constitutional/src/artifact_graph.rs:153:    fn test_empty_graph() {
crates/amun-constitutional/src/artifact_graph.rs:155:        assert!(g.verify_all_edges().is_ok());
crates/amun-constitutional/src/artifact_graph.rs:159:    fn test_add_valid() {
crates/amun-constitutional/src/artifact_graph.rs:171:    fn test_reject_invalid() {
crates/amun-constitutional/src/artifact_graph.rs:185:    fn test_filter_by_type() {
crates/amun-constitutional/src/artifact_graph.rs:213:    fn test_edge_types_distinct() {
crates/amun-constitutional/src/artifact_graph.rs:230:    fn test_edge_is_invalid_delegates() {
crates/amun-constitutional/src/canonical_serialize.rs:145:    fn test_u64_roundtrip() {
crates/amun-constitutional/src/canonical_serialize.rs:153:    fn test_bytes_roundtrip() {
crates/amun-constitutional/src/canonical_serialize.rs:164:    fn test_optional_hash_some() {
crates/amun-constitutional/src/canonical_serialize.rs:172:    fn test_optional_hash_none() {
crates/amun-constitutional/src/canonical_serialize.rs:180:    fn test_deterministic_output() {
crates/amun-constitutional/src/canonical_witness.rs:104:        assert_eq!(entries[0].witness_type, WitnessType::HardDependency);
crates/amun-constitutional/src/canonical_witness.rs:105:        assert_eq!(entries[3].witness_type, WitnessType::CompressionElidable);
crates/amun-constitutional/src/canonical_witness.rs:109:    fn test_canonical_order_same_type_lexicographic() {
crates/amun-constitutional/src/canonical_witness.rs:122:    fn test_is_canonical() {
crates/amun-constitutional/src/canonical_witness.rs:132:    fn test_is_not_canonical_wrong_priority_order() {
crates/amun-constitutional/src/canonical_witness.rs:141:    fn test_normalize_produces_canonical() {
crates/amun-constitutional/src/canonical_witness.rs:154:    fn test_normalize_idempotent() {
crates/amun-constitutional/src/canonical_witness.rs:95:    fn test_canonical_order_respects_priority() {
crates/amun-constitutional/src/causal_edge.rs:165:    fn test_edge_verifies() {
crates/amun-constitutional/src/causal_edge.rs:175:        assert!(e.verify().is_ok());
crates/amun-constitutional/src/causal_edge.rs:178:    fn test_self_referential_rejected() {
crates/amun-constitutional/src/causal_edge.rs:188:        assert!(e.verify_structure().is_err());
crates/amun-constitutional/src/causal_edge.rs:191:    fn test_hard_dependency() {
crates/amun-constitutional/src/causal_edge.rs:204:    fn test_non_causal() {
crates/amun-constitutional/src/causal_edge.rs:217:    fn test_hash_deterministic() {
crates/amun-constitutional/src/causality_chain.rs:199:    fn test_empty_chain_verifies() {
crates/amun-constitutional/src/causality_chain.rs:201:        assert!(c.verify().is_ok());
crates/amun-constitutional/src/causality_chain.rs:205:    fn test_single_edge_chain_verifies() {
crates/amun-constitutional/src/causality_chain.rs:216:        assert!(c.verify().is_ok());
crates/amun-constitutional/src/causality_chain.rs:220:    fn test_non_causal_rejected() {
crates/amun-constitutional/src/causality_chain.rs:231:        assert!(c.verify_structure().is_err());
crates/amun-constitutional/src/causality_chain.rs:234:    fn test_discontinuous_chain_rejected() {
crates/amun-constitutional/src/causality_chain.rs:254:        assert!(c.verify_constitutional().is_err());
crates/amun-constitutional/src/causality_chain.rs:257:    fn test_hash_deterministic() {
crates/amun-constitutional/src/causality_type.rs:106:    fn test_non_causal() {
crates/amun-constitutional/src/causality_type.rs:109:        assert!(!CausalityType::ConstitutionalDependency.is_non_causal());
crates/amun-constitutional/src/causality_type.rs:112:    fn test_hard_dependency() {
crates/amun-constitutional/src/causality_type.rs:113:        assert!(CausalityType::ConstitutionalDependency.is_hard_dependency());
crates/amun-constitutional/src/causality_type.rs:96:    fn test_constitutional_dependencies() {
crates/amun-constitutional/src/causality_type.rs:97:        assert!(CausalityType::ConstitutionalDependency.is_constitutional_dependency());
crates/amun-constitutional/src/certificate_scope.rs:187:    fn test_identical_scopes() {
crates/amun-constitutional/src/certificate_scope.rs:195:    fn test_extends() {
crates/amun-constitutional/src/certificate_scope.rs:202:    fn test_supersedes() {
crates/amun-constitutional/src/certificate_scope.rs:212:    fn test_divergent_different_context() {
crates/amun-constitutional/src/certificate_scope.rs:220:    fn test_overlapping_conflicting() {
crates/amun-constitutional/src/certificate_scope.rs:228:    fn test_contains_position() {
crates/amun-constitutional/src/certificate_scope.rs:238:    fn test_span_length() {
crates/amun-constitutional/src/certificate_scope.rs:250:    fn test_verify_against_parent_ok() {
crates/amun-constitutional/src/certificate_scope.rs:253:        assert!(child.verify_against_parent(&parent).is_ok());
crates/amun-constitutional/src/certificate_scope.rs:257:    fn test_verify_against_parent_divergent() {
crates/amun-constitutional/src/certificate_scope.rs:261:        assert!(child.verify_against_parent(&parent).is_err());
crates/amun-constitutional/src/certificate_scope.rs:265:    fn test_scope_hash_deterministic() {
crates/amun-constitutional/src/certificate_scope.rs:272:    fn test_different_span_different_hash() {
crates/amun-constitutional/src/constitutional_failure.rs:233:        assert!(m().verify().is_ok());
crates/amun-constitutional/src/constitutional_hasher.rs:108:    fn test_schema_affects_hash() {
crates/amun-constitutional/src/constitutional_hasher.rs:109:        let h1 = ConstitutionalHasher::new(b"TEST")
crates/amun-constitutional/src/constitutional_hasher.rs:114:        let mut hasher = ConstitutionalHasher::new(b"TEST");
crates/amun-constitutional/src/constitutional_hasher.rs:82:    fn test_deterministic() {
crates/amun-constitutional/src/constitutional_hasher.rs:83:        let h1 = ConstitutionalHasher::new(b"TEST")
crates/amun-constitutional/src/constitutional_hasher.rs:87:        let h2 = ConstitutionalHasher::new(b"TEST")
crates/amun-constitutional/src/constitutional_hasher.rs:95:    fn test_domain_separation() {
crates/amun-constitutional/src/constitutional_witness.rs:255:    fn test_witness_verifies() {
crates/amun-constitutional/src/constitutional_witness.rs:262:        assert!(w.verify().is_ok());
crates/amun-constitutional/src/constitutional_witness.rs:268:    fn test_minimal_subset() {
crates/amun-constitutional/src/constitutional_witness.rs:282:    fn test_compressed_subset() {
crates/amun-constitutional/src/constitutional_witness.rs:295:    fn test_empty_witness_rejected() {
crates/amun-constitutional/src/constitutional_witness.rs:297:        assert!(w.verify_structure().is_err());
crates/amun-constitutional/src/constitutional_witness.rs:301:    fn test_hard_count_mismatch_rejected() {
crates/amun-constitutional/src/constitutional_witness.rs:306:        assert!(w.verify_constitutional().is_err());
crates/amun-constitutional/src/constitutional_witness.rs:310:    fn test_hash_deterministic() {
crates/amun-constitutional/src/constitutional_witness.rs:325:    fn test_order_affects_hash() {
crates/amun-constitutional/src/continuation_chain.rs:212:    fn test_chain_verifies() {
crates/amun-constitutional/src/continuation_chain.rs:217:        assert!(chain.verify().is_ok());
crates/amun-constitutional/src/continuation_chain.rs:220:    fn test_verify_continuation_ok() {
crates/amun-constitutional/src/continuation_chain.rs:225:        assert!(chain.verify_continuation(&rp).is_ok());
crates/amun-constitutional/src/continuation_chain.rs:228:    fn test_context_mismatch_rejected() {
crates/amun-constitutional/src/continuation_chain.rs:233:        assert!(chain.verify_continuation(&rp).is_err());
crates/amun-constitutional/src/continuation_chain.rs:236:    fn test_hash_deterministic() {
crates/amun-constitutional/src/cycle_detection.rs:125:    fn test_acyclic() {
crates/amun-constitutional/src/cycle_detection.rs:134:    fn test_self_loop() {
crates/amun-constitutional/src/cycle_detection.rs:139:    fn test_simple_cycle() {
crates/amun-constitutional/src/cycle_detection.rs:148:    fn test_single_edge() {
crates/amun-constitutional/src/cycle_detection.rs:152:    fn test_empty() {
crates/amun-constitutional/src/divergence_point.rs:181:    fn test_divergence_verifies() {
crates/amun-constitutional/src/divergence_point.rs:195:        assert!(d.verify().is_ok());
crates/amun-constitutional/src/divergence_point.rs:198:    fn test_admissible_fork() {
crates/amun-constitutional/src/divergence_point.rs:216:    fn test_replay_error() {
crates/amun-constitutional/src/divergence_point.rs:234:    fn test_hash_deterministic() {
crates/amun-constitutional/src/divergence_resolution.rs:177:    fn test_resolution_verifies() {
crates/amun-constitutional/src/divergence_resolution.rs:190:        assert!(r.verify().is_ok());
crates/amun-constitutional/src/divergence_resolution.rs:193:    fn test_hash_deterministic() {
crates/amun-constitutional/src/divergence_type.rs:74:    fn test_admissible_divergences() {
crates/amun-constitutional/src/divergence_type.rs:75:        assert!(DivergenceType::ConstitutionalFork.is_admissible());
crates/amun-constitutional/src/divergence_type.rs:77:        assert!(DivergenceType::ConstitutionalSupersession.is_admissible());
crates/amun-constitutional/src/divergence_type.rs:80:    fn test_error_divergences() {
crates/amun-constitutional/src/divergence_type.rs:81:        assert!(DivergenceType::ReplayError.is_error());
crates/amun-constitutional/src/divergence_type.rs:83:        assert!(!DivergenceType::ConstitutionalFork.is_error());
crates/amun-constitutional/src/divergence_type.rs:86:    fn test_ambiguous() {
crates/amun-constitutional/src/divergence_type.rs:88:        assert!(!DivergenceType::ReplayError.is_ambiguous());
crates/amun-constitutional/src/execution_boundary.rs:183:        assert!(mb().verify().is_ok());
crates/amun-constitutional/src/execution_context.rs:132:        assert!(ExecutionContext::new(1, [0xAB; 32], 0).verify().is_ok());
crates/amun-constitutional/src/execution_journal.rs:300:        assert!(me(&mc(), 1, 0, None).verify().is_ok());
crates/amun-constitutional/src/execution_limits.rs:194:        assert!(ExecutionLimits::constitutional_default().verify().is_ok());
crates/amun-constitutional/src/execution_receipt.rs:15://!   The certificate_hash links to a ReplayCertificate that attests
crates/amun-constitutional/src/execution_receipt.rs:242:    fn test_admitted_verifies() {
crates/amun-constitutional/src/execution_receipt.rs:243:        assert!(make_receipt(1, 0, [0xAB; 32], None).verify().is_ok());
crates/amun-constitutional/src/execution_receipt.rs:246:    fn test_hash_deterministic() {
crates/amun-constitutional/src/execution_receipt.rs:253:    fn test_parent_does_not_affect_validity() {
crates/amun-constitutional/src/execution_receipt.rs:255:        assert!(r.verify().is_ok());
crates/amun-constitutional/src/execution_receipt.rs:259:    fn test_different_contexts_can_share_sequence() {
crates/amun-constitutional/src/execution_receipt.rs:266:    fn test_failure_reference_not_in_hash() {
crates/amun-constitutional/src/execution_receipt.rs:28:/// A terminal constitutional witness attesting to replay admissibility.
crates/amun-constitutional/src/execution_receipt.rs:72:    /// Certificate attesting replay admissibility (admissibility-first model).
crates/amun-constitutional/src/hash_domain_registry.rs:109:    fn test_no_constitutional_operational_overlap() {
crates/amun-constitutional/src/hash_domain_registry.rs:78:    fn test_constitutional_domains_unique() {
crates/amun-constitutional/src/hash_domains.rs:54:    fn test_all_domains_unique() {
crates/amun-constitutional/src/replay_certificate.rs:212:    fn test_cert_verifies() {
crates/amun-constitutional/src/replay_certificate.rs:213:        assert!(mc(1, 0, 99, ReplayOutcome::Admitted, None).verify().is_ok());
crates/amun-constitutional/src/replay_certificate.rs:216:    fn test_hash_det() {
crates/amun-constitutional/src/replay_certificate.rs:223:    fn test_scope_affects_hash() {
crates/amun-constitutional/src/replay_certificate.rs:230:    fn test_outcome_affects_hash() {
crates/amun-constitutional/src/replay_certificate.rs:237:    fn test_monotonicity_ok() {
crates/amun-constitutional/src/replay_certificate.rs:240:        assert!(c.verify_scope_against_parent(&p).is_ok());
crates/amun-constitutional/src/replay_certificate.rs:244:    fn test_monotonicity_violated() {
crates/amun-constitutional/src/replay_certificate.rs:260:        assert!(c.verify_scope_against_parent(&p).is_err());
crates/amun-constitutional/src/replay_certificate.rs:264:    fn test_invalid_span_rejected() {
crates/amun-constitutional/src/replay_certificate.rs:267:        assert!(c.verify_structure().is_err());
crates/amun-constitutional/src/replay_certificate.rs:270:    fn test_revision_mismatch_rejected() {
crates/amun-constitutional/src/replay_certificate.rs:274:        assert!(c.verify_constitutional().is_err());
crates/amun-constitutional/src/replay_outcome.rs:54:    fn test_outcome_hash_deterministic() {
crates/amun-constitutional/src/replay_outcome.rs:62:    fn test_different_outcomes_different_hashes() {
crates/amun-constitutional/src/replay_outcome.rs:70:    fn test_is_admitted() {
crates/amun-constitutional/src/replay_outcome.rs:71:        assert!(ReplayOutcome::Admitted.is_admitted());
crates/amun-constitutional/src/replay_outcome.rs:72:        assert!(!ReplayOutcome::Divergent.is_admitted());
crates/amun-constitutional/src/replay_outcome.rs:73:        assert!(!ReplayOutcome::BoundaryViolation.is_admitted());
crates/amun-constitutional/src/replay_outcome.rs:74:        assert!(!ReplayOutcome::ConstitutionalFailure.is_admitted());
crates/amun-constitutional/src/replay_outcome.rs:78:    fn test_is_failure() {
crates/amun-constitutional/src/replay_outcome.rs:79:        assert!(!ReplayOutcome::Admitted.is_failure());
crates/amun-constitutional/src/replay_outcome.rs:80:        assert!(ReplayOutcome::Divergent.is_failure());
crates/amun-constitutional/src/replay_outcome.rs:81:        assert!(ReplayOutcome::BoundaryViolation.is_failure());
crates/amun-constitutional/src/replay_outcome.rs:82:        assert!(ReplayOutcome::ConstitutionalFailure.is_failure());
crates/amun-constitutional/src/restoration_point.rs:191:    fn test_restoration_point_verifies() {
crates/amun-constitutional/src/restoration_point.rs:194:        assert!(rp.verify().is_ok());
crates/amun-constitutional/src/restoration_point.rs:197:    fn test_resume_position_is_after_snapshot_end() {
crates/amun-constitutional/src/restoration_point.rs:203:    fn test_hash_deterministic() {
crates/amun-constitutional/src/restoration_point.rs:210:    fn test_preceding_entry_affects_hash() {
crates/amun-constitutional/src/revision_migration.rs:116:    fn test_fully_compatible_is_usable() {
crates/amun-constitutional/src/revision_migration.rs:123:    fn test_incompatible_is_breaking() {
crates/amun-constitutional/src/revision_migration.rs:130:    fn test_upgrade_proof() {
crates/amun-constitutional/src/schema_registry.rs:162:    fn test_all_core_schemas_unique() {
crates/amun-constitutional/src/schema_registry.rs:163:        assert!(core_schemas::verify_uniqueness());
crates/amun-constitutional/src/schema_registry.rs:167:    fn test_all_core_schemas_in_core_range() {
crates/amun-constitutional/src/schema_registry.rs:168:        assert!(core_schemas::verify_range());
crates/amun-constitutional/src/schema_registry.rs:172:    fn test_ranges_disjoint() {
crates/amun-constitutional/src/schema_registry.rs:183:    fn test_range_classification() {
crates/amun-constitutional/src/snapshot.rs:260:    fn test_snapshot_verifies() {
crates/amun-constitutional/src/snapshot.rs:268:    fn test_hash_deterministic() {
crates/amun-constitutional/src/snapshot.rs:275:    fn test_anchor_affects_hash() {
crates/amun-constitutional/src/snapshot.rs:282:    fn test_outcome_affects_hash() {
crates/amun-constitutional/src/snapshot.rs:297:    fn test_is_restorable() {
crates/amun-constitutional/src/snapshot.rs:310:    fn test_monotonicity_ok() {
crates/amun-constitutional/src/snapshot.rs:320:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/snapshot.rs:323:    fn test_monotonicity_violated() {
crates/amun-constitutional/src/snapshot.rs:338:        assert!(c.verify_against_parent(&p).is_err());
crates/amun-constitutional/src/snapshot.rs:341:    fn test_scope_anchor_mismatch_rejected() {
crates/amun-constitutional/src/snapshot.rs:345:        assert!(s.verify_constitutional().is_err());
crates/amun-constitutional/src/snapshot_scope.rs:187:    fn test_identical() {
crates/amun-constitutional/src/snapshot_scope.rs:192:    fn test_extends_same_anchor() {
crates/amun-constitutional/src/snapshot_scope.rs:198:    fn test_supersedes_different_anchor() {
crates/amun-constitutional/src/snapshot_scope.rs:204:    fn test_divergent_different_context() {
crates/amun-constitutional/src/snapshot_scope.rs:214:    fn test_verify_against_parent_ok() {
crates/amun-constitutional/src/snapshot_scope.rs:217:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/snapshot_scope.rs:220:    fn test_verify_against_parent_divergent_rejected() {
crates/amun-constitutional/src/snapshot_scope.rs:224:        assert!(c.verify_against_parent(&p).is_err());
crates/amun-constitutional/src/snapshot_scope.rs:227:    fn test_is_restorable() {
crates/amun-constitutional/src/state_anchor.rs:15://!   replay lineage and attested transcript scope, never from mutable storage
crates/amun-constitutional/src/state_anchor.rs:1://! ConstitutionalStateAnchor — immutable, replay-derived state attestation.
crates/amun-constitutional/src/state_anchor.rs:242:    fn test_anchor_verifies() {
crates/amun-constitutional/src/state_anchor.rs:243:        assert!(ma(1, 0, 99, [0x11; 32], None).verify().is_ok());
crates/amun-constitutional/src/state_anchor.rs:246:    fn test_hash_deterministic() {
crates/amun-constitutional/src/state_anchor.rs:253:    fn test_state_root_affects_hash() {
crates/amun-constitutional/src/state_anchor.rs:260:    fn test_span_affects_hash() {
crates/amun-constitutional/src/state_anchor.rs:267:    fn test_monotonicity_ok() {
crates/amun-constitutional/src/state_anchor.rs:270:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/state_anchor.rs:273:    fn test_state_transition_ok() {
crates/amun-constitutional/src/state_anchor.rs:276:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/state_anchor.rs:279:    fn test_monotonicity_violated() {
crates/amun-constitutional/src/state_anchor.rs:28:/// An immutable constitutional attestation of replay-derived state.
crates/amun-constitutional/src/state_anchor.rs:294:        assert!(c.verify_against_parent(&p).is_err());
crates/amun-constitutional/src/state_anchor.rs:297:    fn test_invalid_span_rejected() {
crates/amun-constitutional/src/state_anchor.rs:300:        assert!(a.verify_structure().is_err());
crates/amun-constitutional/src/state_anchor.rs:303:    fn test_revision_mismatch_rejected() {
crates/amun-constitutional/src/state_anchor.rs:307:        assert!(a.verify_constitutional().is_err());
crates/amun-constitutional/src/state_anchor.rs:6://! A StateAnchor attests that a specific replay lineage, within a specific
crates/amun-constitutional/src/state_anchor_scope.rs:154:    fn test_identical() {
crates/amun-constitutional/src/state_anchor_scope.rs:159:    fn test_extends_same_root() {
crates/amun-constitutional/src/state_anchor_scope.rs:165:    fn test_state_transition() {
crates/amun-constitutional/src/state_anchor_scope.rs:174:    fn test_divergent_different_context() {
crates/amun-constitutional/src/state_anchor_scope.rs:181:    fn test_overlapping() {
crates/amun-constitutional/src/state_anchor_scope.rs:190:    fn test_verify_against_parent_ok() {
crates/amun-constitutional/src/state_anchor_scope.rs:193:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/state_anchor_scope.rs:196:    fn test_verify_against_parent_state_transition_ok() {
crates/amun-constitutional/src/state_anchor_scope.rs:199:        assert!(c.verify_against_parent(&p).is_ok());
crates/amun-constitutional/src/state_anchor_scope.rs:202:    fn test_verify_against_parent_divergent_rejected() {
crates/amun-constitutional/src/state_anchor_scope.rs:206:        assert!(c.verify_against_parent(&p).is_err());
crates/amun-constitutional/src/state_anchor_scope.rs:209:    fn test_contains_position() {
crates/amun-constitutional/src/state_anchor_scope.rs:215:    fn test_span_length() {
crates/amun-constitutional/src/transition_commitment.rs:126:        assert!(c.verify().is_ok());
crates/amun-constitutional/src/transition_evidence.rs:130:        assert!(e.verify().is_ok());
crates/amun-constitutional/src/witness_type.rs:65:    fn test_hard_is_required() {
crates/amun-constitutional/src/witness_type.rs:66:        assert!(WitnessType::HardDependency.is_required());
crates/amun-constitutional/src/witness_type.rs:67:        assert!(!WitnessType::SupportingDependency.is_required());
crates/amun-constitutional/src/witness_type.rs:68:        assert!(!WitnessType::AuditDependency.is_required());
crates/amun-constitutional/src/witness_type.rs:69:        assert!(!WitnessType::CompressionElidable.is_required());
crates/amun-constitutional/src/witness_type.rs:72:    fn test_elidable() {
crates/amun-constitutional/src/witness_type.rs:73:        assert!(WitnessType::CompressionElidable.is_elidable());
crates/amun-constitutional/src/witness_type.rs:74:        assert!(!WitnessType::HardDependency.is_elidable());
crates/amun-constitutional/src/witness_type.rs:77:    fn test_non_essential() {
crates/amun-constitutional/src/witness_type.rs:78:        assert!(WitnessType::AuditDependency.is_non_essential());
crates/amun-constitutional/src/witness_type.rs:79:        assert!(WitnessType::CompressionElidable.is_non_essential());
crates/amun-constitutional/src/witness_type.rs:80:        assert!(!WitnessType::HardDependency.is_non_essential());
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:19:    let program = ConstitutionalProgram::new(1, 0, 0, code.clone());
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:1:use amun_bytecode::{ConstitutionalProgram, OpCode};
crates/amun-evidence/src/tests.rs:12:            proof: EvidenceProof::Equivocation {
crates/amun-evidence/src/tests.rs:42:            proof: EvidenceProof::Equivocation {
crates/amun-execution/src/tests.rs:22:fn test_wasm_profile_constitutional() {
crates/amun-experimental-framework/src/main.rs:116:fn workload_halt() -> ConstitutionalProgram {
crates/amun-experimental-framework/src/main.rs:117:    ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt])
crates/amun-experimental-framework/src/main.rs:121:fn workload_push10() -> ConstitutionalProgram {
crates/amun-experimental-framework/src/main.rs:122:    ConstitutionalProgram::new(
crates/amun-experimental-framework/src/main.rs:143:fn workload_transform() -> ConstitutionalProgram {
crates/amun-experimental-framework/src/main.rs:144:    ConstitutionalProgram::new(
crates/amun-experimental-framework/src/main.rs:159:fn workload_split() -> ConstitutionalProgram {
crates/amun-experimental-framework/src/main.rs:160:    ConstitutionalProgram::new(
crates/amun-experimental-framework/src/main.rs:193:        let mut hot = HotProofStore::new(1000);
crates/amun-experimental-framework/src/main.rs:194:        let mut archive = ProofArchive::new();
crates/amun-experimental-framework/src/main.rs:195:        let result = ConstitutionalRuntime::execute(
crates/amun-experimental-framework/src/main.rs:207:                transition_proof, ..
crates/amun-experimental-framework/src/main.rs:208:            } => transition_proof,
crates/amun-experimental-framework/src/main.rs:20:struct Stats {
crates/amun-experimental-framework/src/main.rs:214:            ReplayVerifier::replay(&proof, &program, &mut fresh, &[]);
crates/amun-experimental-framework/src/main.rs:232:    let workloads: Vec<(&str, ConstitutionalProgram)> = vec![
crates/amun-experimental-framework/src/main.rs:254:        let mut hot = HotProofStore::new(1000);
crates/amun-experimental-framework/src/main.rs:255:        let mut archive = ProofArchive::new();
crates/amun-experimental-framework/src/main.rs:260:            match ConstitutionalRuntime::execute(
crates/amun-experimental-framework/src/main.rs:277:        let result = ConstitutionalRuntime::execute(
crates/amun-experimental-framework/src/main.rs:288:                transition_proof, ..
crates/amun-experimental-framework/src/main.rs:289:            }) => transition_proof,
crates/amun-experimental-framework/src/main.rs:291:                transition_proof, ..
crates/amun-experimental-framework/src/main.rs:294:                transition_proof
crates/amun-experimental-framework/src/main.rs:2:use amun_bytecode::program::ConstitutionalProgram;
crates/amun-experimental-framework/src/main.rs:311:            ReplayVerifier::replay(&proof, program, &mut fresh, &[]);
crates/amun-experimental-framework/src/main.rs:340:            let mut hot = HotProofStore::new(10000);
crates/amun-experimental-framework/src/main.rs:341:            let mut archive = ProofArchive::new();
crates/amun-experimental-framework/src/main.rs:356:                let result = ConstitutionalRuntime::execute(
crates/amun-experimental-framework/src/main.rs:367:                    transition_proof, ..
crates/amun-experimental-framework/src/main.rs:371:                    ReplayVerifier::replay(&transition_proof, &program, &mut fresh, &[]);
crates/amun-experimental-framework/src/main.rs:3:use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
crates/amun-experimental-framework/src/main.rs:426:// ── Experiment 5: Witness Bundle Size ───────────────────────
crates/amun-experimental-framework/src/main.rs:427:fn exp5_witness_size() {
crates/amun-experimental-framework/src/main.rs:428:    println!("\n=== Experiment 5: Witness Bundle Size ===");
crates/amun-experimental-framework/src/main.rs:430:    let workloads: Vec<(&str, ConstitutionalProgram)> = vec![
crates/amun-experimental-framework/src/main.rs:448:            let mut hot = HotProofStore::new(1000);
crates/amun-experimental-framework/src/main.rs:449:            let mut archive = ProofArchive::new();
crates/amun-experimental-framework/src/main.rs:450:            let result = ConstitutionalRuntime::execute(
crates/amun-experimental-framework/src/main.rs:462:                    transition_proof, ..
crates/amun-experimental-framework/src/main.rs:463:                } => transition_proof,
crates/amun-experimental-framework/src/main.rs:4:use amun_proof_archive::hot_store::HotProofStore;
crates/amun-experimental-framework/src/main.rs:5:use amun_proof_archive::proof_archive::ProofArchive;
crates/amun-failure/src/tests.rs:10:    assert!(ConstitutionalFault::ConstitutionalViolation.should_halt());
crates/amun-failure/src/tests.rs:110:        ConstitutionalFault::UnsafeContractViolation,
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
crates/amun-failure/src/tests.rs:43:        ConstitutionalFault::CapacityExceeded,
crates/amun-failure/src/tests.rs:47:    assert_eq!(ctx.fault, ConstitutionalFault::CapacityExceeded);
crates/amun-failure/src/tests.rs:55:        ConstitutionalFault::EquivocationDetected,
crates/amun-failure/src/tests.rs:70:    let health = KernelHealth::healthy().poison(ConstitutionalFault::UnsafeContractViolation, 1, 5);
crates/amun-failure/src/tests.rs:77:        .poison(ConstitutionalFault::EquivocationDetected, 1, 3)
crates/amun-failure/src/tests.rs:78:        .poison(ConstitutionalFault::InvalidQuorum, 2, 7);
crates/amun-failure/src/tests.rs:83:        assert_eq!(fault, ConstitutionalFault::EquivocationDetected);
crates/amun-failure/src/tests.rs:8:    assert!(ConstitutionalFault::EquivocationDetected.should_halt());
crates/amun-failure/src/tests.rs:92:    let health = KernelHealth::healthy().poison(ConstitutionalFault::UnsafeContractViolation, 1, 0);
crates/amun-failure/src/tests.rs:9:    assert!(ConstitutionalFault::UnsafeContractViolation.should_halt());
crates/amun-finality-certificate/src/lib.rs:102:    pub fn latest(&self) -> Option<&ConstitutionalFinalityCertificate> {
crates/amun-finality-certificate/src/lib.rs:127:        assert!(cert.verify());
crates/amun-finality-certificate/src/lib.rs:142:        assert!(!cert.verify());
crates/amun-finality-certificate/src/lib.rs:149:        assert!(!cert.verify());
crates/amun-invariant-engine/src/invariant_engine.rs:105:    fn w8_all_invariants_pass() {
crates/amun-invariant-engine/src/invariant_engine.rs:115:        assert!(InvariantEngine::all_passed(&results));
crates/amun-invariant-engine/src/invariant_engine.rs:116:        assert!(!InvariantEngine::has_critical_failure(&results));
crates/amun-invariant-engine/src/invariant_engine.rs:131:        assert!(!InvariantEngine::all_passed(&results));
crates/amun-invariant-engine/src/invariant_engine.rs:132:        assert!(InvariantEngine::has_critical_failure(&results));
crates/amun-invariant-engine/src/invariant_engine.rs:153:        assert!(!InvariantEngine::all_passed(&results));
crates/amun-invariant-engine/src/invariant_engine.rs:154:        assert!(!InvariantEngine::has_critical_failure(&results)); // No Critical failed
crates/amun-invariant-engine/src/invariant_engine.rs:60:            .any(|r| r.severity == InvariantSeverity::Critical && !r.passed)
crates/amun-invariant-engine/src/invariant_engine.rs:64:    pub fn all_passed(results: &[InvariantResult]) -> bool {
crates/amun-kernel/src/governance.rs:219:pub struct Attestation {
crates/amun-light-client/src/constitutional_client.rs:181:        assert!(client.verify_certificate(&cert));
crates/amun-light-client/src/constitutional_client.rs:189:        assert!(!client.verify_certificate(&cert));
crates/amun-light-client/src/constitutional_client.rs:210:        assert!(client.verify_chain_extension(&chain));
crates/amun-light-client/tests/light_client_tests.rs:100:    let mut client = ConstitutionalLightClient::new();
crates/amun-light-client/tests/light_client_tests.rs:22:) -> ConstitutionalFinalityCertificate {
crates/amun-light-client/tests/light_client_tests.rs:32:    let transitions = vec![TransitionProof::new(
crates/amun-light-client/tests/light_client_tests.rs:3:use amun_constitutional_runtime::finality_certificate::ConstitutionalFinalityCertificate;
crates/amun-light-client/tests/light_client_tests.rs:45:    let mut cert = ConstitutionalFinalityCertificate::issue(
crates/amun-light-client/tests/light_client_tests.rs:4:use amun_constitutional_runtime::history_root::ConstitutionalHistoryRoot;
crates/amun-light-client/tests/light_client_tests.rs:53:    cert.certificate_hash = cert.compute_hash();
crates/amun-light-client/tests/light_client_tests.rs:61:    let history_root = ConstitutionalHistoryRoot::from_chain(&chain1);
crates/amun-light-client/tests/light_client_tests.rs:63:    let checkpoint = ConstitutionalCheckpoint {
crates/amun-light-client/tests/light_client_tests.rs:6:    ConstitutionalCheckpoint, ConstitutionalLightClient,
crates/amun-light-client/tests/light_client_tests.rs:74:    let mut client = ConstitutionalLightClient::new();
crates/amun-light-client/tests/light_client_tests.rs:89:    let checkpoint = ConstitutionalCheckpoint {
crates/amun-light-client/tests/light_client_tests.rs:9:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-live-cluster/src/validator.rs:604:                        // Evidence validity: all certificates pass .verify()
crates/amun-live-cluster/src/validator.rs:85:        // N105.4A: Deterministic key matching committed test certificates
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:22:fn make_valid_certificate(validator_id: [u8; 32]) -> SlashingCertificate {
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:70:fn n110_4c_duplicate_certificate_not_reapplied() {
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
crates/amun-merkle/src/proof.rs:153:    fn test_proof_verify() {
crates/amun-merkle/src/proof.rs:159:        assert!(proof.verify(&l1, &root));
crates/amun-merkle/src/proof.rs:163:    fn test_proof_wire_freeze_depth_1() {
crates/amun-merkle/src/proof.rs:171:        assert!(decoded.verify(&l1, &MerkleTree::compute_root(&[l1, l2])));
crates/amun-merkle/src/proof.rs:175:    fn test_proof_decode_exact_rejects_trailing() {
crates/amun-merkle/src/proof.rs:181:        assert!(MerkleProof::decode_exact(&buf[..len + 1]).is_err());
crates/amun-merkle/src/proof.rs:185:    fn test_proof_rejects_excessive_depth() {
crates/amun-merkle/src/proof.rs:188:        assert!(MerkleProof::decode(&buf).is_err());
crates/amun-merkle/src/proof.rs:192:    fn test_proof_rejects_invalid_direction() {
crates/amun-merkle/src/proof.rs:199:        assert!(MerkleProof::decode(&buf).is_err());
crates/amun-merkle/src/tests.rs:100:    assert!(!proof.verify(&l3, &root));
crates/amun-merkle/src/tests.rs:79:fn test_proof_verify() {
crates/amun-merkle/src/tests.rs:83:    let mut proof = MerkleProof::new();
crates/amun-merkle/src/tests.rs:87:    assert!(proof.verify(&l1, &root));
crates/amun-merkle/src/tests.rs:91:fn test_proof_rejects_wrong_leaf() {
crates/amun-merkle/src/tests.rs:96:    let mut proof = MerkleProof::new();
crates/amun-networking/src/validator_certificate.rs:118:        assert!(cert.verify(&authority.verifying_key.to_bytes()));
crates/amun-networking/src/validator_certificate.rs:120:        assert_eq!(cert.issuer, authority.peer_id());
crates/amun-networking/src/validator_certificate.rs:138:        assert!(!cert.verify(&impostor.verifying_key.to_bytes()));
crates/amun-networking/src/validator_certificate.rs:156:        assert!(!cert.verify(&authority.verifying_key.to_bytes()));
crates/amun-networking/src/validator_certificate.rs:201:        assert!(decoded.verify(&authority.verifying_key.to_bytes()));
crates/amun-networking/tests/harness/event_scheduler.rs:15:pub struct ScheduledEvent {
crates/amun-networking/tests/harness/event_scheduler.rs:42:pub trait SchedulingPolicy {
crates/amun-networking/tests/harness/event_scheduler.rs:51:pub struct DefaultPolicy;
crates/amun-networking/tests/harness/event_scheduler.rs:54:pub struct EventScheduler {
crates/amun-networking/tests/harness/event_scheduler.rs:7:pub enum EventType {
crates/amun-networking/tests/harness/message_delivery.rs:25:pub struct DelayedEnvelope {
crates/amun-networking/tests/harness/message_delivery.rs:31:pub struct MessageDeliveryEngine {
crates/amun-networking/tests/harness/message_delivery.rs:8:pub struct DeliveryPolicy {
crates/amun-networking/tests/harness/scenario.rs:10:pub trait ConsensusScenario {
crates/amun-networking/tests/harness/scenario.rs:16:pub struct ScenarioConfig {
crates/amun-networking/tests/harness/scenario.rs:38:pub struct ScenarioResult {
crates/amun-networking/tests/harness/scenario.rs:43:pub struct ScenarioRunner {
crates/amun-networking/tests/harness/simulation_node.rs:11:pub struct SimulationNodeCore {
crates/amun-networking/tests/harness/simulation_node.rs:44:pub struct ScenarioNodeState {}
crates/amun-networking/tests/n17_multi_node_network.rs:16:struct Network {
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:5:// N18.5 — Constitutional Catch-up (no network)
crates/amun-networking/tests/n18_checkpoint_sync.rs:13:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n18_checkpoint_sync.rs:14:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-networking/tests/n18_checkpoint_sync.rs:15:    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
crates/amun-networking/tests/n18_checkpoint_sync.rs:1:use amun_certificate_network::distribution::LightClientProofBundle;
crates/amun-networking/tests/n18_checkpoint_sync.rs:22:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-networking/tests/n18_checkpoint_sync.rs:24:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n18_checkpoint_sync.rs:32:        let block = ConstitutionalBlock::new(
crates/amun-networking/tests/n18_checkpoint_sync.rs:45:        bundles.push(LightClientProofBundle::new(block, cert, proof));
crates/amun-networking/tests/n18_checkpoint_sync.rs:57:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp_a.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n18_checkpoint_sync.rs:7:use amun_constitutional_block::ConstitutionalBlock;
crates/amun-networking/tests/n18_checkpoint_sync.rs:8:use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-networking/tests/n18_full_rejoin.rs:123:    let proof1 = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n18_full_rejoin.rs:124:    let proof2 = prove_checkpoint_inclusion(&checkpoints, &cp2.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n18_full_rejoin.rs:125:    let proof3 = prove_checkpoint_inclusion(&checkpoints, &cp3.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n18_full_rejoin.rs:13:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n18_full_rejoin.rs:14:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-networking/tests/n18_full_rejoin.rs:15:    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
crates/amun-networking/tests/n18_full_rejoin.rs:1:use amun_certificate_network::distribution::LightClientProofBundle;
crates/amun-networking/tests/n18_full_rejoin.rs:22:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-networking/tests/n18_full_rejoin.rs:24:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n18_full_rejoin.rs:32:        let block = ConstitutionalBlock::new(
crates/amun-networking/tests/n18_full_rejoin.rs:45:        bundles.push(LightClientProofBundle::new(block, cert, proof));
crates/amun-networking/tests/n18_full_rejoin.rs:66:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp_late.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n18_full_rejoin.rs:7:use amun_constitutional_block::ConstitutionalBlock;
crates/amun-networking/tests/n18_full_rejoin.rs:8:use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-networking/tests/n18_node_rejoin.rs:5:// N18.2 — Lifecycle Invariants
crates/amun-networking/tests/n18_node_rejoin.rs:73:// N18.5 — Constitutional Invariant REJOIN-001
crates/amun-networking/tests/n19_adversarial_rejoin.rs:106:    let proof1 = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:107:    let proof2 = prove_checkpoint_inclusion(&checkpoints, &cp2.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:11:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n19_adversarial_rejoin.rs:12:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:134:    let proof1 = prove_checkpoint_inclusion(&checkpoints, &cp1.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:13:    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:159:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:1:use amun_certificate_network::distribution::LightClientProofBundle;
crates/amun-networking/tests/n19_adversarial_rejoin.rs:20:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-networking/tests/n19_adversarial_rejoin.rs:22:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:30:        let block = ConstitutionalBlock::new(
crates/amun-networking/tests/n19_adversarial_rejoin.rs:43:        bundles.push(LightClientProofBundle::new(block, cert, proof));
crates/amun-networking/tests/n19_adversarial_rejoin.rs:58:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:7:use amun_constitutional_block::ConstitutionalBlock;
crates/amun-networking/tests/n19_adversarial_rejoin.rs:81:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp_high.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:8:use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:104:        let block = ConstitutionalBlock::new(
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:117:        bundles.push(LightClientProofBundle::new(block, cert, proof));
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:11:use amun_constitutional_block::ConstitutionalBlock;
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:129:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:12:use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:201:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:5:use amun_certificate_network::distribution::LightClientProofBundle;
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:85:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:86:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:87:    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:94:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:96:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:122:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:13:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:14:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:150:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:15:    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:1:use amun_certificate_network::distribution::LightClientProofBundle;
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:22:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:24:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:32:        let block = ConstitutionalBlock::new(
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:45:        bundles.push(LightClientProofBundle::new(block, cert, proof));
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:7:use amun_constitutional_block::ConstitutionalBlock;
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:86:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:8:use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:110:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:12:fn build_checkpoint(start: u64, end: u64) -> CheckpointCertificate {
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:13:    let mut rt = ConstitutionalStateRuntime::new();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:14:    let mut bundles: Vec<LightClientProofBundle> = Vec::new();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:165:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:1:use amun_certificate_network::distribution::LightClientProofBundle;
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:21:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:23:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:31:        let block = ConstitutionalBlock::new(
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:44:        bundles.push(LightClientProofBundle::new(block, cert, proof));
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:61:    let proof = prove_checkpoint_inclusion(&checkpoints, &cp_late.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:7:use amun_constitutional_block::ConstitutionalBlock;
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:8:use amun_constitutional_state::ConstitutionalStateRuntime;
crates/amun-networking/tests/v2_001_time_driven_consensus.rs:2:// V2-001: Happy Path — first Stable Scenario on Constitutional Harness
crates/amun-networking/tests/v2_001_time_driven_consensus.rs:7:struct HappyPathScenario;
crates/amun-networking/tests/v2_002_multi_round_consensus.rs:6:struct MultiRoundScenario {
crates/amun-networking/tests/v2_003_multi_height_consensus.rs:6:struct MultiHeightScenario {
crates/amun-networking/tests/v2_004_long_run_stability.rs:7:struct LongRunScenario {
crates/amun-networking/tests/v2_013_baseline_matrix.rs:2:// V2-013: Baseline Matrix — Constitutional Harness
crates/amun-networking/tests/v2_013_baseline_matrix.rs:8:struct BaselineScenario {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:102:struct TimeoutInjectionSim {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:21:enum EventType {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:31:struct ScheduledEvent {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:58:struct EventScheduler {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:83:struct DelayedEnvelope {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:89:struct ConsensusNode {
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:126:    let mut constitutional = ConstitutionalRegistry::new();
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:151:    constitutional.register(NftConstitutionalRecord {
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:162:    let can_sell = amun_nft_constitutional_enforcement::EnforcementEngine::can_be_sold(
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:2:use amun_nft_constitutional_registry::{ConstitutionalRegistry, NftConstitutionalRecord};
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
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:10:    reg.register(NftConstitutionalRecord {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:1:use amun_nft_constitutional_registry::{ConstitutionalRegistry, NftConstitutionalRecord};
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:31:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:34:        reg.register(NftConstitutionalRecord {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:76:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:77:    reg.register(NftConstitutionalRecord {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:8:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:97:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:99:    reg.register(NftConstitutionalRecord {
crates/amun-nft-integration/tests/n132_integration_tests.rs:30:        [7u8; 32], // replay_certificate
crates/amun-nft-integration/tests/n132_integration_tests.rs:41:    assert_eq!(extended.replay_certificate, [7u8; 32]);
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:100:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:112:    reg.register(NftConstitutionalRecord {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:145:fn n141_produce_enforcement_proof() {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:146:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:147:    reg.register(NftConstitutionalRecord {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:160:    let proof = RightsEnforcementEngine::produce_enforcement_proof(
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:164:    let proof2 = RightsEnforcementEngine::produce_enforcement_proof(
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:27:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:29:    reg.register(NftConstitutionalRecord {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:2:use amun_nft_constitutional_registry::{ConstitutionalRegistry, NftConstitutionalRecord};
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:59:    let mut reg = ConstitutionalRegistry::new();
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:72:    reg.register(NftConstitutionalRecord {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:9:    let reg = ConstitutionalRegistry::new();
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:148:        amun_nft_constitutional_registry::NftConstitutionalRecord {
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:26:        amun_nft_constitutional_registry::NftConstitutionalRecord {
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:113:    let const_root = const_reg.compute_constitutional_root();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:207:    const_reg.register(NftConstitutionalRecord {
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:221:    assert_ne!(cr_before, const_reg.compute_constitutional_root());
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:22:    ConstitutionalRegistry,
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:2:use amun_nft_constitutional_registry::{ConstitutionalRegistry, NftConstitutionalRecord};
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:35:    let mut const_reg = ConstitutionalRegistry::new();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:88:        const_reg.register(NftConstitutionalRecord {
crates/amun-nft-stress/src/lib.rs:7:pub struct StressTestResult {
crates/amun-nft-stress/tests/n146_stress_tests.rs:100:    use amun_nft_constitutional_enforcement::EnforcementEngine;
crates/amun-nft-stress/tests/n146_stress_tests.rs:101:    use amun_nft_constitutional_registry::{ConstitutionalRegistry, NftConstitutionalRecord};
crates/amun-nft-stress/tests/n146_stress_tests.rs:113:    let mut constitutional = ConstitutionalRegistry::new();
crates/amun-nft-stress/tests/n146_stress_tests.rs:156:    constitutional.register(NftConstitutionalRecord {
crates/amun-nft-stress/tests/n146_stress_tests.rs:183:    let root_before_sale = constitutional.compute_constitutional_root();
crates/amun-nft-stress/tests/n146_stress_tests.rs:195:        EnforcementEngine::enforce_royalty(&constitutional, &token_id, 1000).unwrap();
crates/amun-nft-stress/tests/n146_stress_tests.rs:208:    EnforcementEngine::transfer_governance(&mut constitutional, &token_id, &buyer);
crates/amun-nft-stress/tests/n146_stress_tests.rs:250:    let root_after_all = constitutional.compute_constitutional_root();
crates/amun-nft-stress/tests/n146_stress_tests.rs:98:fn n147_full_constitutional_flow() {
crates/amun-node/src/bin/test_constitutional_determinism.rs:1:use amun_bytecode::program::ConstitutionalProgram;
crates/amun-node/src/bin/test_constitutional_determinism.rs:24:            let program = ConstitutionalProgram::new(level, 0, 0, vec![]);
crates/amun-node/src/bin/test_constitutional_determinism.rs:2:use amun_constitutional_runtime::runtime_pipeline::ConstitutionalRuntime;
crates/amun-node/src/bin/test_constitutional_determinism.rs:34:            let mut hot = amun_proof_archive::hot_store::HotProofStore::new(1000);
crates/amun-node/src/bin/test_constitutional_determinism.rs:35:            let mut archive = amun_proof_archive::proof_archive::ProofArchive::new();
crates/amun-node/src/bin/test_constitutional_determinism.rs:37:            let result = ConstitutionalRuntime::execute(
crates/amun-node/src/bin/test_constitutional_determinism.rs:66:        println!("\nPASS: Constitutional runtime execution is deterministic");
crates/amun-node/src/bin/test_constitutional_determinism.rs:8:fn main() {
crates/amun-node/src/bin/test_constitutional_multi_block.rs:11:fn main() {
crates/amun-node/src/bin/test_constitutional_multi_block.rs:125:        println!("\nPASS: Constitutional multi-block state evolution is deterministic");
crates/amun-node/src/bin/test_constitutional_multi_block.rs:2:use amun_bytecode::program::ConstitutionalProgram;
crates/amun-node/src/bin/test_constitutional_multi_block.rs:3:use amun_constitutional_runtime::runtime_pipeline::ConstitutionalRuntime;
crates/amun-node/src/bin/test_constitutional_multi_block.rs:40:                lineage: ResourceLineage::genesis(resource_id),
crates/amun-node/src/bin/test_constitutional_multi_block.rs:49:            let program = ConstitutionalProgram::new(
crates/amun-node/src/bin/test_constitutional_multi_block.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
crates/amun-node/src/bin/test_constitutional_multi_block.rs:73:            let mut hot = amun_proof_archive::hot_store::HotProofStore::new(1000);
crates/amun-node/src/bin/test_constitutional_multi_block.rs:74:            let mut archive = amun_proof_archive::proof_archive::ProofArchive::new();
crates/amun-node/src/bin/test_constitutional_multi_block.rs:76:            let result = ConstitutionalRuntime::execute(
crates/amun-node/src/bin/test_constitutional_mutation.rs:27:            let program = ConstitutionalProgram::new(
crates/amun-node/src/bin/test_constitutional_mutation.rs:2:use amun_bytecode::program::ConstitutionalProgram;
crates/amun-node/src/bin/test_constitutional_mutation.rs:3:use amun_constitutional_runtime::runtime_pipeline::ConstitutionalRuntime;
crates/amun-node/src/bin/test_constitutional_mutation.rs:49:            let mut hot = amun_proof_archive::hot_store::HotProofStore::new(1000);
crates/amun-node/src/bin/test_constitutional_mutation.rs:50:            let mut archive = amun_proof_archive::proof_archive::ProofArchive::new();
crates/amun-node/src/bin/test_constitutional_mutation.rs:52:            let result = ConstitutionalRuntime::execute(
crates/amun-node/src/bin/test_constitutional_mutation.rs:90:        println!("\nPASS: Constitutional state mutation is deterministic");
crates/amun-node/src/bin/test_constitutional_mutation.rs:9:fn main() {
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
crates/amun-node/src/bin/test_replay_stress.rs:121:            let mut hot = amun_proof_archive::hot_store::HotProofStore::new(1000);
crates/amun-node/src/bin/test_replay_stress.rs:122:            let mut archive = amun_proof_archive::proof_archive::ProofArchive::new();
crates/amun-node/src/bin/test_replay_stress.rs:123:            ConstitutionalRuntime::execute(
crates/amun-node/src/bin/test_replay_stress.rs:19:    let mut mutation_log: Vec<(ResourceId, ConstitutionalProgram)> = Vec::new();
crates/amun-node/src/bin/test_replay_stress.rs:2:use amun_bytecode::program::ConstitutionalProgram;
crates/amun-node/src/bin/test_replay_stress.rs:3:use amun_constitutional_runtime::runtime_pipeline::ConstitutionalRuntime;
crates/amun-node/src/bin/test_replay_stress.rs:41:        let program = ConstitutionalProgram::new(
crates/amun-node/src/bin/test_replay_stress.rs:63:        let mut hot = amun_proof_archive::hot_store::HotProofStore::new(1000);
crates/amun-node/src/bin/test_replay_stress.rs:64:        let mut archive = amun_proof_archive::proof_archive::ProofArchive::new();
crates/amun-node/src/bin/test_replay_stress.rs:65:        ConstitutionalRuntime::execute(
crates/amun-node/src/bin/test_runtime_mutation_integration.rs:2:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
crates/amun-node/src/bin/test_runtime_mutation_integration.rs:41:                    lineage: ResourceLineage::genesis(id),
crates/amun-node/src/bin/test_runtime_mutation_integration.rs:7:fn main() {
crates/amun-node/src/certificate_loader.rs:120:    fn n106_2_certificate_with_matching_genesis_passes() {
crates/amun-pccv/src/transition_proof_engine.rs:166:        assert!(matches!(result, PCCVResult::Verified { .. }));
crates/amun-pccv/src/transition_proof_engine.rs:235:        assert!(matches!(result, PCCVResult::Failed { ref reason } if reason.contains("T1")));
crates/amun-pccv/tests/replay_equivalence.rs:100:    let result1 = amun_pccv::pccv_verifier::PCCVVerifier::verify(&proof1, &reg1);
crates/amun-pccv/tests/replay_equivalence.rs:101:    let result2 = amun_pccv::pccv_verifier::PCCVVerifier::verify(&proof2, &reg2);
crates/amun-pccv/tests/replay_equivalence.rs:126:        archetype: ResourceArchetype::ConstitutionalAsset,
crates/amun-pccv/tests/replay_equivalence.rs:144:            TransitionProofEngine::build_proof(
crates/amun-pccv/tests/replay_equivalence.rs:16:fn n49c_replay_produces_identical_proof() {
crates/amun-pccv/tests/replay_equivalence.rs:2:use amun_pccv::transition_proof_engine::TransitionProofEngine;
crates/amun-pccv/tests/replay_equivalence.rs:40:        archetype: ResourceArchetype::ConstitutionalAsset,
crates/amun-pccv/tests/replay_equivalence.rs:73:    let proof1 = TransitionProofEngine::build_proof(
crates/amun-pccv/tests/replay_equivalence.rs:84:    let proof2 = TransitionProofEngine::build_proof(
crates/amun-peer-identity/tests/peer_identity_tests.rs:15:    let a = ConstitutionalPeerId::new("key".into(), GENESIS.into());
crates/amun-peer-identity/tests/peer_identity_tests.rs:16:    let b = ConstitutionalPeerId::new("key".into(), "other_genesis".into());
crates/amun-peer-identity/tests/peer_identity_tests.rs:1:use amun_constitutional_signing::ConstitutionalKeyPair;
crates/amun-peer-identity/tests/peer_identity_tests.rs:21:fn test_self_signed_certificate_verification() {
crates/amun-peer-identity/tests/peer_identity_tests.rs:22:    let keypair = ConstitutionalKeyPair::generate();
crates/amun-peer-identity/tests/peer_identity_tests.rs:23:    let peer_id = ConstitutionalPeerId::new(keypair.verifying_key_hex(), GENESIS.into());
crates/amun-peer-identity/tests/peer_identity_tests.rs:29:fn test_certificate_rejected_for_wrong_civilisation() {
crates/amun-peer-identity/tests/peer_identity_tests.rs:2:use amun_peer_identity::{ConstitutionalPeerId, IdentityVerifier, PeerCertificate, PeerRegistry};
crates/amun-peer-identity/tests/peer_identity_tests.rs:30:    let keypair = ConstitutionalKeyPair::generate();
crates/amun-peer-identity/tests/peer_identity_tests.rs:31:    let peer_id = ConstitutionalPeerId::new(keypair.verifying_key_hex(), "other_genesis".into());
crates/amun-peer-identity/tests/peer_identity_tests.rs:38:    let keypair = ConstitutionalKeyPair::generate();
crates/amun-peer-identity/tests/peer_identity_tests.rs:39:    let peer_id = ConstitutionalPeerId::new(keypair.verifying_key_hex(), GENESIS.into());
crates/amun-peer-identity/tests/peer_identity_tests.rs:8:    let a = ConstitutionalPeerId::new("key_a".into(), GENESIS.into());
crates/amun-peer-identity/tests/peer_identity_tests.rs:9:    let b = ConstitutionalPeerId::new("key_a".into(), GENESIS.into());
crates/amun-proof-carrying/tests/proof_tests.rs:11:fn test_proof_carrying_receipt_creation() {
crates/amun-proof-carrying/tests/proof_tests.rs:19:    let block = BlockBuilder::build(
crates/amun-proof-carrying/tests/proof_tests.rs:1:use amun_constitutional_block::BlockBuilder;
crates/amun-proof-carrying/tests/proof_tests.rs:2:use amun_constitutional_commitments::SparseMerkleTree;
crates/amun-proof-carrying/tests/proof_tests.rs:31:    let pcr = ProofCarryingReceipt::new(
crates/amun-proof-carrying/tests/proof_tests.rs:4:use amun_proof_carrying::ProofCarryingReceipt;
crates/amun-proof-carrying/tests/proof_tests.rs:6:fn dummy_receipt(id: &str) -> ExecutionReceipt {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:121:            return Err("Not all transition proofs passed replay verification".into());
crates/amun-replay-engine/src/byzantine_witness_filter.rs:108:    fn test_inflation_suspected() {
crates/amun-replay-engine/src/byzantine_witness_filter.rs:120:    fn test_self_reference_rejected() {
crates/amun-replay-engine/src/byzantine_witness_filter.rs:19:    /// Witness passes all structural checks.
crates/amun-replay-engine/src/byzantine_witness_filter.rs:92:    fn test_empty_witness_rejected() {
crates/amun-replay-engine/src/byzantine_witness_filter.rs:94:        assert!(matches!(filter_incoming_witness(&w), FilterResult::EmptyWitness));
crates/amun-replay-engine/src/byzantine_witness_filter.rs:98:    fn test_valid_witness_accepted() {
crates/amun-replay-engine/src/constitutional_economics.rs:200:    fn test_market_is_economic_not_constitutional() {
crates/amun-replay-engine/src/constitutional_economics.rs:207:    fn test_lease_never_grants_authority() {
crates/amun-replay-engine/src/constitutional_economics.rs:214:    fn test_lease_capacity_exhaustion() {
crates/amun-replay-engine/src/constitutional_economics.rs:216:        assert!(lease.execute());
crates/amun-replay-engine/src/constitutional_economics.rs:217:        assert!(lease.execute());
crates/amun-replay-engine/src/constitutional_economics.rs:218:        assert!(!lease.execute()); // exhausted
crates/amun-replay-engine/src/constitutional_economics.rs:222:    fn test_semantic_quality_reward_is_zero() {
crates/amun-replay-engine/src/constitutional_economics.rs:224:        assert!(incentives.verify_no_semantic_reward());
crates/amun-replay-engine/src/constitutional_economics.rs:229:    fn test_economic_containment() {
crates/amun-replay-engine/src/constitutional_economics.rs:237:    fn test_scarcity_neutrality() {
crates/amun-replay-engine/src/constitutional_economics.rs:245:    fn test_economics_does_not_affect_truth() {
crates/amun-replay-engine/src/constitutional_governance.rs:248:    fn test_ratification_surface() {
crates/amun-replay-engine/src/constitutional_governance.rs:256:    fn test_amendment_admissible() {
crates/amun-replay-engine/src/constitutional_governance.rs:262:    fn test_self_referential_amendment_flagged() {
crates/amun-replay-engine/src/constitutional_governance.rs:268:    fn test_revision_regression_rejected() {
crates/amun-replay-engine/src/constitutional_governance.rs:274:    fn test_immutable_invariants() {
crates/amun-replay-engine/src/constitutional_governance.rs:282:    fn test_amendment_depth_boundary() {
crates/amun-replay-engine/src/constitutional_governance.rs:290:    fn test_governance_containment() {
crates/amun-replay-engine/src/constitutional_governance.rs:300:    fn test_temporal_lineage() {
crates/amun-replay-engine/src/constitutional_governance.rs:312:    fn test_governance_does_not_manufacture_truth() {
crates/amun-replay-engine/src/constitutional_identity.rs:167:    fn test_identity_never_has_authority() {
crates/amun-replay-engine/src/constitutional_identity.rs:173:    fn test_identity_fingerprint_deterministic() {
crates/amun-replay-engine/src/constitutional_identity.rs:180:    fn test_attribution_never_has_semantic_weight() {
crates/amun-replay-engine/src/constitutional_identity.rs:186:    fn test_reputation_neutrality() {
crates/amun-replay-engine/src/constitutional_identity.rs:194:    fn test_identity_containment() {
crates/amun-replay-engine/src/constitutional_identity.rs:202:    fn test_identity_is_not_authority() {
crates/amun-replay-engine/src/cross_constitution_federation.rs:175:    fn test_boundary_never_allows_override() {
crates/amun-replay-engine/src/cross_constitution_federation.rs:182:    fn test_translation_preserves_sovereignty() {
crates/amun-replay-engine/src/cross_constitution_federation.rs:186:        assert!(translation.verify_sovereignty());
crates/amun-replay-engine/src/cross_constitution_federation.rs:191:    fn test_bridge_never_assumes_equivalence() {
crates/amun-replay-engine/src/cross_constitution_federation.rs:199:    fn test_federation_is_not_unification() {
crates/amun-replay-engine/src/cross_constitution_federation.rs:207:    fn test_translation_fingerprint_deterministic() {
crates/amun-replay-engine/src/cross_constitution_federation.rs:214:    fn test_bridge_records_translations() {
crates/amun-replay-engine/src/execution_dependency.rs:64:        assert!(ExecutionDependencyType::RequiresWitness.is_mandatory());
crates/amun-replay-engine/src/execution_task.rs:86:        assert_eq!(task.task_type, TaskType::StateTransition);
crates/amun-replay-engine/src/operational_hasher.rs:56:    fn test_operational_separate_from_constitutional() {
crates/amun-replay-engine/src/proof_routing.rs:215:    fn test_quarantine_skip() {
crates/amun-replay-engine/src/proof_routing.rs:228:    fn test_redundant_equivalence() {
crates/amun-replay-engine/src/proof_routing.rs:241:    fn test_frontier_reduction() {
crates/amun-replay-engine/src/proof_routing.rs:254:    fn test_hop_expiry() {
crates/amun-replay-engine/src/proof_routing.rs:265:    fn test_propagation_budget() {
crates/amun-replay-engine/src/proof_routing.rs:271:    fn test_equivalence_compression() {
crates/amun-replay-engine/src/runtime_anomaly.rs:110:    fn test_anomaly_classification() {
crates/amun-replay-engine/src/runtime_anomaly.rs:121:    fn test_quarantine_threshold() {
crates/amun-replay-engine/src/runtime_anomaly.rs:131:    fn test_anomaly_is_not_invalidity() {
crates/amun-replay-engine/src/runtime_capability.rs:81:    fn test_producer_cannot_verify() {
crates/amun-replay-engine/src/runtime_capability.rs:88:    fn test_verifier_cannot_execute() {
crates/amun-replay-engine/src/runtime_capability.rs:95:    fn test_recovery_can_restore() {
crates/amun-replay-engine/src/temporal_drift.rs:228:    fn test_temporal_precedence_is_not_constitutional_precedence() {
crates/amun-replay-engine/src/witness_envelope.rs:100:    fn test_envelope_creation() {
crates/amun-replay-engine/src/witness_envelope.rs:110:    fn test_envelope_hash_deterministic() {
crates/amun-replay-engine/src/witness_envelope.rs:117:    fn test_envelope_is_not_witness() {
crates/amun-replay-engine/src/zk_adapters.rs:24:    pub asserted_fingerprint: ConstitutionalHash,
crates/amun-replay-optimization/tests/n163_replay_tests.rs:40:    let valid = cache.batch_verify_certificates(&cert_hashes, true);
crates/amun-replay-semantics/src/lib.rs:87:    #[test] fn test_cert_self_verifying() { assert!(ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1).verify()); }
crates/amun-replay-semantics/src/lib.rs:88:    #[test] fn test_cert_tamper_detected() { let mut c = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); c.state_root = [0xFF;32]; assert!(!c.verify()); }
crates/amun-replay-semantics/src/lib.rs:89:    #[test] fn test_equivalence_strict() { let c = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); assert!(ReplayEquivalence::Strict.verify(&c, &c)); }
crates/amun-replay-semantics/src/lib.rs:90:    #[test] fn test_law_determinism_divergence() { let a = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); let b = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0xFF;32], [0x03;32], [0x04;32], tb(), 100, 1); assert!(laws::replay_determinism(&a, &b).is_err()); }
crates/amun-replay/src/certificate.rs:115:    pub fn latest(&self) -> Option<&ReplayCertificate> {
crates/amun-replay/src/certificate.rs:150:        assert!(genesis.verify());
crates/amun-replay/src/certificate.rs:157:        let mut store = ReplayCertificateStore::new();
crates/amun-replay/src/certificate.rs:172:        let mut store = ReplayCertificateStore::new();
crates/amun-replay/src/certificate.rs:183:        let mut store = ReplayCertificateStore::new();
crates/amun-replay/src/certificate.rs:195:        let mut store = ReplayCertificateStore::new();
crates/amun-replay/src/certificate.rs:211:            assert!(c.verify());
crates/amun-replay/src/certificate.rs:81:pub struct ReplayCertificateStore {
crates/amun-replay/src/certificate.rs:86:impl ReplayCertificateStore {
crates/amun-replay/src/lib.rs:36:pub use certificate::{ReplayCertificate, ReplayCertificateStore};
crates/amun-replay/src/store.rs:82:    fn test_missing_certificate() {
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
crates/amun-runtime-benchmarks/benches/runtime_benchmarks.rs:2:fn main() {}
crates/amun-sdk-layer/src/tests.rs:15:    fn test_sandbox_simulation() { let mut sandbox = Sandbox::new(); let a0 = sandbox.create_account(1_000_000).data.expect("test invariant"); let a1 = sandbox.create_account(500_000).data.expect("test invariant"); let result = sandbox.simulate_transfer(a0, a1, 100_000); assert!(result.success); }
crates/amun-sdk-layer/src/tests.rs:7:    fn test_token_api_transfer() { let mut token = TokenApi::create_account(1_000_000).data.expect("test invariant"); let result = TokenApi::transfer(&mut token, 100_000); assert!(result.success); }
crates/amun-smt/tests/fuzz.rs:69:    fn proof_roundtrip_random(
crates/amun-smt/tests/fuzz.rs:80:        prop_assert!(proof.verify(&root.0).unwrap());
crates/amun-smt/tests/proofs.rs:11:fn inclusion_proof_roundtrip() {
crates/amun-smt/tests/proofs.rs:23:        .expect("Proof must exist");
crates/amun-smt/tests/proofs.rs:24:    assert!(proof.verify(&root.0).unwrap(), "Proof must verify");
crates/amun-smt/tests/proofs.rs:28:fn absence_proof_roundtrip() {
crates/amun-smt/tests/proofs.rs:40:        .expect("Proof must exist");
crates/amun-smt/tests/proofs.rs:41:    assert!(proof.verify(&root.0).unwrap(), "Absence proof must verify");
crates/amun-smt/tests/proofs.rs:45:fn empty_tree_absence() {
crates/amun-smt/tests/proofs.rs:4:fn random_key(rng: &mut impl Rng) -> Key256 {
crates/amun-smt/tests/proofs.rs:53:        .expect("Proof must exist");
crates/amun-smt/tests/proofs.rs:55:        proof.verify(&root.0).unwrap(),
crates/amun-smt/tests/proofs.rs:56:        "Empty tree absence must verify"
crates/amun-snapshot-engine/tests/constitutional_tests.rs:101:            "replay",
crates/amun-snapshot-engine/tests/constitutional_tests.rs:106:        let h2 = ConstitutionalHash::compute(
crates/amun-snapshot-engine/tests/constitutional_tests.rs:109:            "replay",
crates/amun-snapshot-engine/tests/constitutional_tests.rs:118:    fn identity_encode_decode_roundtrip() {
crates/amun-snapshot-engine/tests/constitutional_tests.rs:119:        let id = ConstitutionalIdentity::new([0xFFu8; 32]);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:11:        let id1 = ConstitutionalIdentity::new(hash);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:121:        let decoded = ConstitutionalIdentity::decode(&encoded).unwrap();
crates/amun-snapshot-engine/tests/constitutional_tests.rs:123:        assert!(decoded.verify());
crates/amun-snapshot-engine/tests/constitutional_tests.rs:127:    fn tampered_identity_fails_verification() {
crates/amun-snapshot-engine/tests/constitutional_tests.rs:128:        let id = ConstitutionalIdentity::new([0x13u8; 32]);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:12:        let id2 = ConstitutionalIdentity::new(hash);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:133:        let decoded = ConstitutionalIdentity::decode(&encoded).unwrap();
crates/amun-snapshot-engine/tests/constitutional_tests.rs:134:        assert!(!decoded.verify());
crates/amun-snapshot-engine/tests/constitutional_tests.rs:18:    fn different_constitutions_produce_different_identities() {
crates/amun-snapshot-engine/tests/constitutional_tests.rs:19:        let id1 = ConstitutionalIdentity::new([0x01u8; 32]);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:20:        let id2 = ConstitutionalIdentity::new([0x02u8; 32]);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:26:    fn identity_self_verification() {
crates/amun-snapshot-engine/tests/constitutional_tests.rs:27:        let id = ConstitutionalIdentity::new([0xAAu8; 32]);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:28:        assert!(id.verify());
crates/amun-snapshot-engine/tests/constitutional_tests.rs:32:    fn fully_compatible_when_identical() {
crates/amun-snapshot-engine/tests/constitutional_tests.rs:33:        let id = ConstitutionalIdentity::new([0x11u8; 32]);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:34:        let matrix = CompatibilityEngine::compute(&id, &id);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:39:    fn different_constitutional_hash_allows_readonly() {
crates/amun-snapshot-engine/tests/constitutional_tests.rs:40:        let id1 = ConstitutionalIdentity::new([0xAAu8; 32]);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:41:        let id2 = ConstitutionalIdentity::new([0xBBu8; 32]);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:42:        let matrix = CompatibilityEngine::compute(&id1, &id2);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:47:    fn truly_incompatible_when_structural_universe_differs() {
crates/amun-snapshot-engine/tests/constitutional_tests.rs:48:        let mut id1 = ConstitutionalIdentity::new([0xCCu8; 32]);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:49:        let mut id2 = ConstitutionalIdentity::new([0xDDu8; 32]);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:4:        CompatibilityEngine, CompatibilityLevel, ConstitutionalHash, ConstitutionalIdentity,
crates/amun-snapshot-engine/tests/constitutional_tests.rs:52:        let matrix = CompatibilityEngine::compute(&id1, &id2);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:57:    fn can_sync_only_when_compatible() {
crates/amun-snapshot-engine/tests/constitutional_tests.rs:58:        let id = ConstitutionalIdentity::new([0xCCu8; 32]);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:59:        assert!(CompatibilityEngine::can_sync(&id, &id));
crates/amun-snapshot-engine/tests/constitutional_tests.rs:5:        ConstitutionalRelationship, TransitionClassifier,
crates/amun-snapshot-engine/tests/constitutional_tests.rs:63:    fn classify_identical_as_identical() {
crates/amun-snapshot-engine/tests/constitutional_tests.rs:64:        let id = ConstitutionalIdentity::new([0xDDu8; 32]);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:66:        assert!(matches!(rel, ConstitutionalRelationship::Identical));
crates/amun-snapshot-engine/tests/constitutional_tests.rs:70:    fn sync_possible_between_identical() {
crates/amun-snapshot-engine/tests/constitutional_tests.rs:71:        let id = ConstitutionalIdentity::new([0xEEu8; 32]);
crates/amun-snapshot-engine/tests/constitutional_tests.rs:76:    fn constitutional_hash_is_deterministic() {
crates/amun-snapshot-engine/tests/constitutional_tests.rs:77:        let h1 = ConstitutionalHash::compute(
crates/amun-snapshot-engine/tests/constitutional_tests.rs:80:            "replay",
crates/amun-snapshot-engine/tests/constitutional_tests.rs:85:        let h2 = ConstitutionalHash::compute(
crates/amun-snapshot-engine/tests/constitutional_tests.rs:88:            "replay",
crates/amun-snapshot-engine/tests/constitutional_tests.rs:97:    fn constitutional_hash_changes_with_different_input() {
crates/amun-snapshot-engine/tests/constitutional_tests.rs:98:        let h1 = ConstitutionalHash::compute(
crates/amun-snapshot-engine/tests/constitutional_tests.rs:9:    fn identical_identities_match() {
crates/amun-soak-test/src/lib.rs:12:pub struct ValidatorSimulator {
crates/amun-soak-test/src/lib.rs:165:pub struct SoakResult {
crates/amun-state-sync/src/lib.rs:124:            assert!(proof.verify());
crates/amun-stateless-sync/src/lib.rs:252:            ProofBundleSyncMessage::BundleResponse { bundle: b } => assert!(b.verify().is_ok()),
crates/amun-stf/src/state.rs:6:pub trait StateStore {
crates/amun-stf/src/tests.rs:14:    stf.apply_set(key, val).expect("test invariant");
crates/amun-stf/src/tests.rs:15:    let new_root = stf.commit().expect("test invariant");
crates/amun-stf/src/tests.rs:27:    stf.apply_set(key, val).expect("test invariant");
crates/amun-stf/src/tests.rs:43:        stf2.apply_set(k, v).expect("test invariant");
crates/amun-stf/src/tests.rs:45:    let r1 = stf1.commit().expect("test invariant");
crates/amun-stf/src/tests.rs:46:    let r2 = stf2.commit().expect("test invariant");
crates/amun-storage-kernel/tests/canonical_constants.rs:7:    fn canonical_empty_root_matches_runtime() {
crates/amun-storage-kernel/tests/proptest_smt.rs:16:    fn proof_roundtrip() {
crates/amun-storage-kernel/tests/proptest_smt.rs:29:                    prop_assert!(proof.verify(root.0), "proof verification failed");
crates/amun-storage-kernel/tests/proptest_smt.rs:37:    fn absence_proof() {
crates/amun-storage-kernel/tests/proptest_smt.rs:54:                    prop_assert!(proof.verify(root.0), "absence proof verification failed");
crates/amun-storage-kernel/tests/specification_compliance.rs:110:    // THEOREM 6: Proof Verification Roundtrip
crates/amun-storage-kernel/tests/specification_compliance.rs:113:    fn theorem_proof_roundtrip() {
crates/amun-storage-kernel/tests/specification_compliance.rs:121:            proof.verify(root),
crates/amun-storage-kernel/tests/specification_compliance.rs:122:            "Theorem 6 violated: proof verification failed"
crates/amun-storage-kernel/tests/specification_compliance.rs:139:    // THEOREM 8: Empty Tree Absence Proof
crates/amun-storage-kernel/tests/specification_compliance.rs:142:    fn theorem_empty_tree_absence_proof() {
crates/amun-storage-kernel/tests/specification_compliance.rs:148:            proof.verify(root),
crates/amun-storage-kernel/tests/specification_compliance.rs:149:            "Theorem 8 violated: empty tree absence proof"
crates/amun-storage-kernel/tests/specification_compliance.rs:67:    // THEOREM 4: Proof Depth Invariant
crates/amun-storage-kernel/tests/specification_compliance.rs:70:    fn theorem_proof_depth() {
crates/amun-storage-kernel/tests/specification_compliance.rs:79:            "Theorem 4 violated: proof depth != 256"
crates/amun-storage-kernel/tests/specification_compliance.rs:86:            "Theorem 4 violated: absence proof depth != 256"
crates/amun-testnet-sim/tests/adversarial_tests.rs:114:// ── N60.3 — Tampered Proof ──────────────────────────────────
crates/amun-testnet-sim/tests/adversarial_tests.rs:116:fn n60_tampered_proof_rejected_by_consensus() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:164:    // The key insight: replay verifies that the proof is consistent with
crates/amun-testnet-sim/tests/adversarial_tests.rs:188:        let mut hot = HotProofStore::new(10000);
crates/amun-testnet-sim/tests/adversarial_tests.rs:189:        let mut archive = ProofArchive::new();
crates/amun-testnet-sim/tests/adversarial_tests.rs:190:        let result = ConstitutionalRuntime::execute(
crates/amun-testnet-sim/tests/adversarial_tests.rs:203:                transition_proof,
crates/amun-testnet-sim/tests/adversarial_tests.rs:206:                // Replay against a registry initialized to the proof's pre-state
crates/amun-testnet-sim/tests/adversarial_tests.rs:212:                let replay = ReplayVerifier::replay(&transition_proof, &program, &mut fresh, &[]);
crates/amun-testnet-sim/tests/adversarial_tests.rs:2:use amun_bytecode::program::ConstitutionalProgram;
crates/amun-testnet-sim/tests/adversarial_tests.rs:38:fn make_program() -> ConstitutionalProgram {
crates/amun-testnet-sim/tests/adversarial_tests.rs:39:    ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt])
crates/amun-testnet-sim/tests/adversarial_tests.rs:3:use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
crates/amun-testnet-sim/tests/adversarial_tests.rs:4:use amun_proof_archive::hot_store::HotProofStore;
crates/amun-testnet-sim/tests/adversarial_tests.rs:5:use amun_proof_archive::proof_archive::ProofArchive;
crates/amun-transaction/src/tests.rs:110:    assert!(r.expect("test invariant").validate_basic().is_err());
crates/amun-transaction/src/tests.rs:64:    assert!(r.expect("test invariant").validate_basic().is_ok());
crates/amun-transaction/src/tests.rs:79:    assert!(r.expect("test invariant").validate_basic().is_ok());
crates/amun-transaction/src/tests.rs:94:    assert!(r.expect("test invariant").validate_basic().is_ok());
crates/amun-transcript-semantics/src/lib.rs:104:    #[test] fn test_immutable_cert() { let c = ImmutableReplayCertificate::new(ReplayDomain::Consensus,[0xBB;32],[0x01;32],[0x02;32],[0x03;32],[0x04;32],100,1); assert!(c.verify()); let h1 = c.certificate_hash(); let h2 = c.certificate_hash(); assert_eq!(h1, h2); }
crates/amun-transcript-semantics/src/lib.rs:105:    #[test] fn test_witness_hash() { let w = ReplayWitness::MerkleWitness { leaf_hash: [0x01;32], proof_hashes: vec![[0x02;32]], leaf_index: 0 }; assert_ne!(w.witness_hash(), [0;32]); }
crates/amun-unsafe/src/tests.rs:116:    struct DropCount<'a> {
crates/amun-unsafe/src/tests.rs:133:    struct DropCount<'a> {
crates/amun-unsafe/src/tests.rs:77:    struct DropCount<'a> {
crates/amun-validator-attestation/src/attestation.rs:5:pub struct ValidatorAttestation {
crates/amun-validator-attestation/src/validator_set.rs:12:pub struct ValidatorSet {
crates/amun-validator-attestation/src/validator_set.rs:5:pub struct ValidatorInfo {
crates/amun-validator-attestation/src/validator_set.rs:69:pub enum ValidatorSetError {
crates/constitutional-linter/src/edge_semantics.rs:103:    fn test_edge_classification() {
crates/constitutional-linter/src/edge_semantics.rs:111:    fn test_edge_names() {
crates/constitutional-linter/src/main.rs:69:enum Layer { Kernel, Constitutional, Interface, Consensus, Execution, Persistence, Network, Governance, Testing, Unknown }
crates/constitutional-linter/src/scc.rs:111:    fn test_no_cycles() {
crates/constitutional-linter/src/scc.rs:122:    fn test_cycle_detection() {
crates/constitutional-linter/src/scc.rs:135:    fn test_self_loop() {
docs/N105_CRYPTOGRAPHIC_VALIDATOR_IDENTITY.md:53:Vote forgery resistance is achieved because without the private key, an attacker cannot produce a valid signature for a given voter_id. The registry ensures that only known validators can attempt to vote. Sybil resistance is enforced because the registry is populated from certificates signed by the authority. An attacker cannot inject fake validators. In a production setting, the authority would be the genesis trust anchor set. Replay protection is provided by the signing payload, which includes the height, block hash, and timestamp, binding each vote to a specific consensus round. The authority key is currently hardcoded to the seed 0x42, which is acceptable for test clusters. In production, this key must be distributed via genesis configuration and protected. Certificate integrity is verified at load time. A tampered certificate will cause a panic in test clusters or an error in production loading code, preventing the node from participating.
docs/architecture/DEPENDENCY_RULES.md:18:- Core ← Crypto ← Consensus ← Constitution ← Execution ← Storage ← Network ← Test
docs/audit/AUDIT_EVIDENCE_BUNDLE.md:3:## 1. Invariant-to-Test Traceability
docs/audit/AUDIT_EVIDENCE_BUNDLE.md:5:| Invariant | Description | Test ID | Crate | Status |
docs/audit/N103_MAINNET_READINESS_AUDIT.md:14:| A5 | State | Constitutional root determinism (cross-node) | ✅ | 107+ CCA tests pass |
docs/audit/TRACEABILITY_MATRIX.md:9:| N48.5-W5 Transition Proof | amun-transition-proof | 5 tests | ✅ |
docs/constitutional-mathematics/PHASE_112_SIMULATION_SPECIFICATION.md:11:Transform Constitutional Mathematics into an experimental computational theory.
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:223:### Test B: Evidence State Change
docs/reports/CCA_IMPL_4_FINAL_REPORT.md:193:Constitutional commitments can no longer be bypassed, ignored, or detached from finalized chain history.
crates/amun-audit/tests/audit_layer06_replay.rs:135:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer06_replay.rs:50:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer06_replay.rs:91:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer11_crash.rs:124:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer15_temporal.rs:50:        let result1 = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer15_temporal.rs:51:        let result2 = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-authority-registry/src/registry.rs:493:        assert!(!reg.can_issue_at(1, 150));
crates/amun-authority-registry/src/registry.rs:494:        assert!(reg.can_issue_at(2, 150));
crates/amun-bench/tests/n161_state_root_bench.rs:41:        let _root = reg.compute_state_root();
crates/amun-bench/tests/n162_snapshot_bench.rs:9:fn n162_bench_snapshot_build_and_restore() {
crates/amun-benchmarks/benches/sync_bench.rs:47:            black_box(imported.compute_state_root());
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:38:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:70:    let block1 = builder1.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:71:    let block2 = builder2.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:100:        block.verify_slashing_root(&[0u8; 32]).is_ok(),
crates/amun-block-builder/tests/n120_2_slashing_root.rs:104:        block.verify_slashing_root(&[0x01; 32]).is_err(),
crates/amun-block-builder/tests/n120_2_slashing_root.rs:15:    let block1 = build_block_with_root([0u8; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:16:    let mut block2 = build_block_with_root([0u8; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:29:    let block1 = build_block_with_root([0x42; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:30:    let block2 = build_block_with_root([0x42; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:42:    let block = build_block_with_root([0u8; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:54:    let block_a = build_block_with_root(root_a);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:55:    let block_b = build_block_with_root(root_b);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:5:fn build_block_with_root(root: [u8; 32]) -> Block {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:65:    let block_zero = build_block_with_root([0u8; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:66:    let block_nonzero = build_block_with_root([0xAB; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:78:    let block = build_block_with_root(root);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:80:        block.verify_slashing_root(&root).is_ok(),
crates/amun-block-builder/tests/n120_2_slashing_root.rs:87:    let block = build_block_with_root([0x42; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:88:    let result = block.verify_slashing_root(&[0xFF; 32]);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:8:    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-block-builder/tests/n120_2_slashing_root.rs:98:    let block = build_block_with_root([0u8; 32]);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:119:    let block1 = b1.build_block(1, [0u8; 32], &mut mp1, 10, [0u8; 32], 1000);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:121:    let block2 = b2.build_block(1, [0u8; 32], &mut Mempool::new(), 0, [0u8; 32], 1000);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:42:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:71:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-block-builder/tests/n28_first_economic_block.rs:110:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-block-builder/tests/n28_first_economic_block.rs:30:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 123456);
crates/amun-block-builder/tests/n28_first_economic_block.rs:81:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-block-builder/tests/n32_certified_block.rs:100:    let block1 = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-block-builder/tests/n32_certified_block.rs:123:    let block2 = builder.build_block(
crates/amun-block-builder/tests/n32_certified_block.rs:173:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-block-builder/tests/n32_certified_block.rs:52:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 123456);
crates/amun-block/src/tests.rs:85:    assert_eq!(blk.compute_id(), blk.compute_id());
crates/amun-byzantine-tests/tests/attack_suite.rs:279:    assert!(!tampered.verify_integrity());
crates/amun-chain-checkpoint/src/inclusion.rs:202:        assert!(verify_checkpoint_sequence(&bundles, &root).is_ok());
crates/amun-chain-checkpoint/src/inclusion.rs:209:        assert!(verify_checkpoint_sequence(&[CheckpointBundle::new(c1, p1)], &[0x11; 32]).is_err());
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:103:    let (_, bundle1, root) = build_checkpoint_bundle(0, 2);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:105:    let (_, mut bundle2, _) = build_checkpoint_bundle(3, 5);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:10:fn build_checkpoint_bundle(
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:121:    let (cp, bundle, root) = build_checkpoint_bundle(0, 4);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:57:fn build_bundle(start: u64, end: u64) -> (CheckpointBundle, [u8; 32]) {
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:58:    let (_, bundle, root) = build_checkpoint_bundle(start, end);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:64:    let (cp1, _, root) = build_checkpoint_bundle(0, 2);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:65:    let (bundle2, _) = build_bundle(3, 5);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:77:    let (_, mut bundle, root) = build_checkpoint_bundle(0, 4);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:86:    let (_, bundle1, root) = build_checkpoint_bundle(0, 2);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:87:    let (bundle2, _) = build_bundle(4, 6);
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:95:    let (bundle, _) = build_bundle(0, 4);
crates/amun-chain-store/examples/snapshot_verification_test.rs:2:use amun_chain_store::snapshot::{create_snapshot, verify_snapshot};
crates/amun-chain-store/examples/snapshot_verification_test.rs:38:    let manifest = verify_snapshot(&snapshot_dir).unwrap();
crates/amun-chain-store/examples/snapshot_verification_test.rs:46:    match verify_snapshot(&snapshot_dir) {
crates/amun-chain-store/examples/snapshot_verification_test.rs:61:    match verify_snapshot(&snapshot_dir) {
crates/amun-consensus-integration/src/consensus_integrator.rs:238:        assert_eq!(block.state_root, registry.compute_state_root());
crates/amun-consensus-network/src/evidence_gossip.rs:178:        assert!(EvidenceGossip::verify_announcement(&ann, 50).is_ok());
crates/amun-consensus-network/src/evidence_gossip.rs:203:        assert!(EvidenceGossip::verify_announcement(&ann, 50).is_err());
crates/amun-consensus-network/src/evidence_gossip.rs:215:        assert!(EvidenceGossip::verify_announcement(&ann, 50).is_err());
crates/amun-consensus-network/src/messages.rs:231:        assert!(qc.verify_strict(&powers).is_err());
crates/amun-consensus-network/src/messages.rs:257:        assert!(qc.verify_strict(&powers).is_err());
crates/amun-consensus-network/src/messages.rs:362:        assert!(!qc.verify_quorum());
crates/amun-consensus-network/src/messages.rs:380:        assert!(!qc.verify_consistency());
crates/amun-consensus-network/src/slashing_state.rs:117:        assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/src/slashing_state.rs:150:        assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/src/slashing_state.rs:98:        assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/src/validator_identity.rs:169:        assert!(id.verify_binding());
crates/amun-consensus-network/src/validator_identity.rs:176:        assert!(!id.verify_binding());
crates/amun-consensus-network/src/validator_identity.rs:274:        assert!(decoded.verify_binding());
crates/amun-consensus-network/src/vote_binding.rs:135:        assert!(verify_vote_binding(&vote).is_ok());
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:162:    let recomputed = compute_execution_root(
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:185:        compute_execution_root(&pk, commit.height, &commit.block_hash, &commit.state_root);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:258:    let recomputed = compute_execution_root(&voter, height, &block_hash, &state_root);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:52:fn compute_execution_root(
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:74:    let execution_root = compute_execution_root(&validator_id, height, &block_hash, &state_root);
crates/amun-consensus-network/tests/n109_block_propagation.rs:257:fn n109_validate_basic_accepts_valid_proposal() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:260:    let result = validate_basic_testable(&p, 0, &parent, 1050);
crates/amun-consensus-network/tests/n109_block_propagation.rs:272:fn n109_validate_basic_rejects_wrong_height() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:275:    let result = validate_basic_testable(&p, 0, &parent, 5000);
crates/amun-consensus-network/tests/n109_block_propagation.rs:287:fn n109_validate_basic_rejects_parent_mismatch() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:289:    let result = validate_basic_testable(&p, 0, &[0xBB; 32], 1000);
crates/amun-consensus-network/tests/n109_block_propagation.rs:304:fn n109_validate_basic_rejects_hash_mismatch() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:307:    let result = validate_basic_testable(&p, 0, &[0u8; 32], 1000);
crates/amun-consensus-network/tests/n109_block_propagation.rs:73:fn validate_basic_testable(
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:108:    let result = verify_block_execution(&proposal, |bytes| simulate_executor(bytes, 42));
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:137:    let result = verify_block_execution(&proposal, |bytes| simulate_executor(bytes, 99));
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:181:        let result = verify_block_execution(&proposal, |bytes| simulate_executor(bytes, seed));
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:248:    let result = verify_block_execution(&proposal, |bytes| simulate_executor(bytes, 1));
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:277:    let result = verify_block_execution(&proposal, |_bytes| {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:317:    let result = verify_block_execution(cached, |bytes| simulate_executor(bytes, 1));
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:51:fn verify_block_execution<F>(
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:121:    let request = build_missing_evidence_request([0xBB; 32], missing_ids);
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:100:    assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:131:        state.verify_consistency().is_err(),
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:46:        state.verify_consistency().is_ok(),
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:82:    assert!(restored.verify_consistency().is_ok());
crates/amun-contract-events/tests/n173_events_storage_tests.rs:31:        storage1.compute_events_root(),
crates/amun-contract-events/tests/n173_events_storage_tests.rs:32:        storage2.compute_events_root()
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:46:    let root1 = ContractExecutor::compute_contract_evidence_root(&reg1);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:47:    let root2 = ContractExecutor::compute_contract_evidence_root(&reg2);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:91:    assert_eq!(cr1.compute_registry_root(), cr2.compute_registry_root());
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:44:    let root_before = cr.compute_registry_root();
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:46:    let root_after = cr.compute_registry_root();
crates/amun-core-optimization/tests/n161_optimization_tests.rs:48:    let _root1 = reg.compute_state_root();
crates/amun-core-optimization/tests/n161_optimization_tests.rs:52:    let _root2 = opt_reg.compute_state_root();
crates/amun-crypto-hardening/src/key_rotation.rs:160:        assert!(chain.verify_chain());
crates/amun-defi-amm/tests/n153_amm_tests.rs:32:    assert_eq!(amm1.compute_evidence_root(), amm2.compute_evidence_root());
crates/amun-defi-amm/tests/n153_amm_tests.rs:43:    let root_before = amm.compute_evidence_root();
crates/amun-defi-amm/tests/n153_amm_tests.rs:45:    let root_after = amm.compute_evidence_root();
crates/amun-defi-governance/tests/n157_governance_tests.rs:25:        engine1.compute_governance_root(),
crates/amun-defi-governance/tests/n157_governance_tests.rs:26:        engine2.compute_governance_root()
crates/amun-defi-governance/tests/n157_governance_tests.rs:33:    let root_before = engine.compute_governance_root();
crates/amun-defi-governance/tests/n157_governance_tests.rs:37:    let root_after = engine.compute_governance_root();
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:109:        engine1.compute_lending_root(),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:110:        engine2.compute_lending_root()
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:32:        engine1.compute_stablecoin_root(),
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:33:        engine2.compute_stablecoin_root()
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:29:    let block = builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:68:        let b = builder.build_block(i, parent, &mut mempool, 100, [9u8; 32], i * 1000);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:98:    builder.build_block(1, [0u8; 32], &mut mempool, 100, [9u8; 32], 1000);
crates/amun-evidence/src/tests.rs:31:        let hash = evidence.compute_hash();
crates/amun-evidence/src/tests.rs:36:    fn test_evidence_verify_same_hash_rejected() {
crates/amun-execution/src/tests.rs:31:fn test_wasm_profile_verify_module_ok() {
crates/amun-execution/src/tests.rs:33:    assert!(profile.verify_module(b"").is_ok());
crates/amun-execution/src/tests.rs:50:fn test_verify_deterministic_wasm_ok() {
crates/amun-execution/src/tests.rs:52:    assert!(wasm_deterministic_subset::verify_deterministic_wasm(b"").is_ok());
crates/amun-experimental-framework/src/main.rs:183:        let mut reg = build_registry(size);
crates/amun-experimental-framework/src/main.rs:190:            pre_state_root: reg.compute_state_root(),
crates/amun-experimental-framework/src/main.rs:242:        let mut reg = build_registry(size);
crates/amun-experimental-framework/src/main.rs:249:            pre_state_root: reg.compute_state_root(),
crates/amun-experimental-framework/src/main.rs:256:        let exec_stats = measure_us(&format!("execute_{}", name), 5, 30, || {
crates/amun-experimental-framework/src/main.rs:26:fn compute_stats(times: &[f64]) -> Stats {
crates/amun-experimental-framework/src/main.rs:353:                    pre_state_root: reg.compute_state_root(),
crates/amun-experimental-framework/src/main.rs:396:            let (mut reg, tip) = build_deep_chain(depth);
crates/amun-experimental-framework/src/main.rs:438:            let mut reg = build_registry(size);
crates/amun-experimental-framework/src/main.rs:445:                pre_state_root: reg.compute_state_root(),
crates/amun-experimental-framework/src/main.rs:46:    let s = compute_stats(&times);
crates/amun-experimental-framework/src/main.rs:489:        let stats = measure_us(&format!("verify_{}_resources", size), 5, 30, || {
crates/amun-experimental-framework/src/main.rs:490:            let reg = build_registry(size);
crates/amun-experimental-framework/src/main.rs:491:            let _root = reg.compute_state_root();
crates/amun-experimental-framework/src/main.rs:65:fn build_registry(size: u64) -> ResourceRegistry {
crates/amun-experimental-framework/src/main.rs:81:fn build_deep_chain(depth: u64) -> (ResourceRegistry, ResourceId) {
crates/amun-failure/src/tests.rs:96:    assert!(actions.invalidate_snapshots);
crates/amun-light-client/tests/light_client_tests.rs:107:    assert!(!client.verify_chain_extension(&chain));
crates/amun-live-cluster/src/bin/e2e_test.rs:24:    let app = build_app(state);
crates/amun-live-cluster/src/bin/e2e_test.rs:4:use amun_rpc::{build_app, AppState};
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:12:    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:17:        block.verify_slashing_root(&root).is_ok(),
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:26:    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:30:    let result = block.verify_slashing_root(&[0xFF; 32]);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:41:    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:50:    let result = block.verify_slashing_root(&[0x42; 32]);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:66:    let mut block = builder.build_block(1, [0u8; 32], &mut mempool, 10, [0u8; 32], 1000);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:78:        block.verify_slashing_root(&validator_root).is_err(),
crates/amun-mempool-gossip/src/messages.rs:91:        assert!(tx.verify_hash());
crates/amun-mempool-gossip/src/messages.rs:98:        assert!(!tx.verify_hash());
crates/amun-merkle/src/tests.rs:33:    let root = MerkleTree::compute_root(&[leaf]);
crates/amun-merkle/src/tests.rs:42:        MerkleTree::compute_root(&[l1, l2]),
crates/amun-merkle/src/tests.rs:43:        MerkleTree::compute_root(&[l1, l2])
crates/amun-merkle/src/tests.rs:53:        MerkleTree::compute_root(&[l1, l2, l3]),
crates/amun-merkle/src/tests.rs:54:        MerkleTree::compute_root(&[l1, l2, l3])
crates/amun-merkle/src/tests.rs:63:    let root3 = MerkleTree::compute_root(&[l1, l2, l3]);
crates/amun-merkle/src/tests.rs:64:    let root4 = MerkleTree::compute_root(&[l1, l2, l3, l3]);
crates/amun-merkle/src/tests.rs:73:        MerkleTree::compute_root(&[l1, l2]),
crates/amun-merkle/src/tests.rs:74:        MerkleTree::compute_root(&[l2, l1])
crates/amun-merkle/src/tests.rs:82:    let root = MerkleTree::compute_root(&[l1, l2]);
crates/amun-merkle/src/tests.rs:95:    let root = MerkleTree::compute_root(&[l1, l2]);
crates/amun-networking/tests/harness/handlers.rs:22:pub fn build_prevote_envelope(
crates/amun-networking/tests/harness/handlers.rs:45:pub fn build_precommit_envelope(
crates/amun-networking/tests/harness/handlers.rs:5:pub fn build_proposal_envelope(node_id: &str, leader_id: [u8; 32], round: u64) -> Envelope {
crates/amun-networking/tests/n18_checkpoint_sync.rs:116:    let cp1 = build_checkpoint(0, 9);
crates/amun-networking/tests/n18_checkpoint_sync.rs:117:    let cp2 = build_checkpoint(10, 19);
crates/amun-networking/tests/n18_checkpoint_sync.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n18_checkpoint_sync.rs:54:    let cp_a = build_checkpoint(0, 49);
crates/amun-networking/tests/n18_full_rejoin.rs:116:    let cp1 = build_checkpoint(0, 19);
crates/amun-networking/tests/n18_full_rejoin.rs:117:    let cp2 = build_checkpoint(20, 39);
crates/amun-networking/tests/n18_full_rejoin.rs:118:    let cp3 = build_checkpoint(40, 59);
crates/amun-networking/tests/n18_full_rejoin.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n18_full_rejoin.rs:58:    let _cp_early = build_checkpoint(0, 9);
crates/amun-networking/tests/n18_full_rejoin.rs:62:    let cp_late = build_checkpoint(10, 49);
crates/amun-networking/tests/n18_node_rejoin.rs:77:fn n18_rejoin001_bootstrapping_node_must_verify_before_active() {
crates/amun-networking/tests/n19_adversarial_rejoin.rs:101:    let cp1 = build_checkpoint(0, 19);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:102:    let cp2 = build_checkpoint(40, 59);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:129:    let cp1 = build_checkpoint(0, 9);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:130:    let cp2 = build_checkpoint(10, 19);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:138:        prove_checkpoint_inclusion(&checkpoints, &cp2.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n19_adversarial_rejoin.rs:156:    let cp = build_checkpoint(0, 49);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n19_adversarial_rejoin.rs:55:    let cp = build_checkpoint(0, 9);
crates/amun-networking/tests/n19_adversarial_rejoin.rs:73:    let cp_high = build_checkpoint(50, 59);
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:126:    let cp = build_checkpoint(0, 49);
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:198:    let cp = build_checkpoint(50, 99);
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:8:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:120:    let cp = build_checkpoint(0, 9);
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:147:    let cp = build_checkpoint(0, 49);
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:65:    let cp = build_checkpoint(0, 9);
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:83:    let cp = build_checkpoint(0, 49);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:107:    let cp = build_checkpoint(100, 199);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:131:    let cp_old = build_checkpoint(0, 9);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:135:        prove_checkpoint_inclusion(&checkpoints_old, &cp_old.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:141:    let cp_new = build_checkpoint(50, 99);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:145:        prove_checkpoint_inclusion(&checkpoints_new, &cp_new.checkpoint_hash_bytes()).unwrap();
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:162:    let cp = build_checkpoint(0, 49);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:4:    inclusion::{checkpoint_merkle_root, prove_checkpoint_inclusion, CheckpointBundle},
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:57:    let cp_late = build_checkpoint(50, 99);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:118:    let result = NftEvidenceKernel::verify_ownership(&reg, &token_id, &thief);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:97:    let result = NftEvidenceKernel::verify_metadata_hash(&[1u8; 32], &[2u8; 32]);
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:159:    let root = reg.compute_state_root();
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:105:    assert_ne!(l1.compute_bridge_root(), l2.compute_bridge_root());
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:81:    assert_eq!(l1.compute_bridge_root(), l2.compute_bridge_root());
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:98:    assert_eq!(l1.compute_bridge_root(), l2.compute_bridge_root());
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:144:        engine1.compute_evidence_root(),
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:145:        engine2.compute_evidence_root()
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:103:    let result = NftEvidenceKernel::verify_replay_protection(1000, 500);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:159:    assert!(NftEvidenceKernel::verify_mint(ctx).is_ok());
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:40:    assert!(NftEvidenceKernel::verify_mint(ctx).is_ok());
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:72:    let result = NftEvidenceKernel::verify_transfer(&reg, &token_id, &thief, 2000, 1000);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:91:    let result = NftEvidenceKernel::verify_non_duplicate(&reg, &token_id);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:97:    let result = NftEvidenceKernel::verify_metadata_hash(&[1u8; 32], &[2u8; 32]);
crates/amun-nft-governance-execution/tests/n143_governance_execution_tests.rs:37:fn n143_execute_passing_proposal() {
crates/amun-nft-governance-execution/tests/n143_governance_execution_tests.rs:98:    assert_eq!(e1.compute_execution_root(), e2.compute_execution_root());
crates/amun-nft-governance/tests/n138_governance_tests.rs:79:    assert_eq!(l1.compute_governance_root(), l2.compute_governance_root());
crates/amun-nft-governance/tests/n138_governance_tests.rs:97:    assert_ne!(l1.compute_governance_root(), l2.compute_governance_root());
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:112:    let old_root = indexer.compute_index_root();
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:119:    let new_root = indexer.compute_index_root();
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:92:    assert_eq!(i1.compute_index_root(), i2.compute_index_root());
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:182:    let root = mp.compute_evidence_root();
crates/amun-nft-mining/tests/n133_mining_tests.rs:1:use amun_nft_mining::{evaluate_contribution, issue_mining_reward, ContributionType};
crates/amun-nft-mining/tests/n133_mining_tests.rs:35:    let nft_id = issue_mining_reward(
crates/amun-nft-mining/tests/n133_mining_tests.rs:71:    let id1 = issue_mining_reward(
crates/amun-nft-mining/tests/n133_mining_tests.rs:80:    let id2 = issue_mining_reward(
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:100:    let (reg2, royalty2, gov2, bridge2) = build_full_state();
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:102:    assert_eq!(state_root_before, reg2.compute_state_root());
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:103:    assert_eq!(royalty_root_before, royalty2.compute_accounting_root());
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:104:    assert_eq!(gov_root_before, gov2.compute_governance_root());
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:105:    assert_eq!(bridge_root_before, bridge2.compute_bridge_root());
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:16:fn build_full_state() -> (
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:93:    let (reg1, royalty1, gov1, bridge1) = build_full_state();
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:95:    let state_root_before = reg1.compute_state_root();
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:96:    let royalty_root_before = royalty1.compute_accounting_root();
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:97:    let gov_root_before = gov1.compute_governance_root();
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:98:    let bridge_root_before = bridge1.compute_bridge_root();
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:12:    let result = RightsEnforcementEngine::validate_transfer(
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:131:    let result = RightsEnforcementEngine::validate_transfer(
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:41:    let result = RightsEnforcementEngine::validate_transfer(
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:82:    let result = RightsEnforcementEngine::validate_transfer(
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:74:        ledger1.compute_accounting_root(),
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:75:        ledger2.compute_accounting_root()
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:58:    assert_eq!(e1.compute_settlement_root(), e2.compute_settlement_root());
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:31:    let amount = RoyaltyEngine::compute_royalty(1000, policy.royalty_bps);
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:41:    let amount = RoyaltyEngine::compute_royalty(u64::MAX, policy.royalty_bps);
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:9:    let amount = RoyaltyEngine::compute_royalty(1000, policy.royalty_bps);
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:109:    let state_root = reg.compute_state_root();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:110:    let royalty_root = royalty.compute_accounting_root();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:111:    let gov_root = gov.compute_governance_root();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:112:    let bridge_root = bridge.compute_bridge_root();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:131:    let (_, _, _, _, _, sr1, rr1, gr1, br1, cr1) = build_state();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:132:    let (_, _, _, _, _, sr2, rr2, gr2, br2, cr2) = build_state();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:143:    let (_, _, _, _, _, sr, rr, gr, br, cr) = build_state();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:165:    ) = build_state();
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:218:    assert_ne!(sr_before, reg.compute_state_root());
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:219:    assert_ne!(rr_before, royalty.compute_accounting_root());
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:220:    assert_ne!(gr_before, gov.compute_governance_root());
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:30:fn build_state() -> SnapshotTestState {
crates/amun-nft-stress/tests/n146_stress_tests.rs:254:    let state_root = reg.compute_state_root();
crates/amun-nft-stress/tests/n146_stress_tests.rs:94:    assert_eq!(reg1.compute_state_root(), reg2.compute_state_root());
crates/amun-operations/src/backup_recovery.rs:141:        assert_eq!(restored.compute_state_root(), state_root);
crates/amun-pccv/tests/replay_equivalence.rs:118:    let pre_root = reg.compute_state_root();
crates/amun-pccv/tests/replay_equivalence.rs:33:    let pre_root1 = reg1.compute_state_root();
crates/amun-pccv/tests/replay_equivalence.rs:34:    let pre_root2 = reg2.compute_state_root();
crates/amun-replay-consensus/src/replay_backed_consensus.rs:293:        assert_eq!(block.replay_root, block.compute_replay_root());
crates/amun-replay-optimization/tests/n163_replay_tests.rs:81:    assert_eq!(cache1.compute_cache_root(), cache2.compute_cache_root());
crates/amun-replay-store/src/lib.rs:120:        assert!(store.verify_chain().unwrap());
crates/amun-replay-store/src/lib.rs:132:        assert!(!store.verify_chain().unwrap());
crates/amun-replay-store/src/lib.rs:155:        assert!(store.verify_chain().unwrap());
crates/amun-resource-core/tests/stress_tests.rs:141:    let root = reg.compute_state_root();
crates/amun-sdk-layer/src/tests.rs:17:    fn test_transaction_builder_transfer() { let sender = amun_kernel_types::PublicKey::new([1u8; 48]); let recipient = amun_kernel_types::PublicHash32::new([2u8; 32]); let result = TransactionBuilder::build_transfer(42, 0, sender, recipient, 100, 1000); assert!(result.success); }
crates/amun-smt/tests/fuzz.rs:60:            let _ = validate_tree(&t.internal_root(), t.context());
crates/amun-smt/tests/validator.rs:11:fn validate_after_inserts() {
crates/amun-smt/tests/validator.rs:18:    validate_tree(&t.internal_root(), t.context()).unwrap();
crates/amun-smt/tests/validator.rs:22:fn validate_after_delete() {
crates/amun-smt/tests/validator.rs:29:    validate_tree(&t.internal_root(), t.context()).unwrap();
crates/amun-smt/tests/validator.rs:5:fn validate_empty_tree() {
crates/amun-smt/tests/validator.rs:7:    validate_tree(&t.internal_root(), t.context()).unwrap();
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:41:    let base_root = reg.compute_state_root();
crates/amun-soak-test/src/lib.rs:124:                    let _root = self.registry.lock().unwrap().compute_state_root();
crates/amun-soak-test/src/lib.rs:141:                        let root1 = self.registry.lock().unwrap().compute_state_root();
crates/amun-soak-test/src/lib.rs:142:                        let root2 = self.registry.lock().unwrap().compute_state_root();
crates/amun-soak-test/src/lib.rs:160:            state_root: self.registry.lock().unwrap().compute_state_root(),
crates/amun-soak-test/src/lib.rs:94:                    let pool_id = amun_defi_core::DefiPool::compute_pool_id(token_a, token_b);
crates/amun-state-pruning/tests/n166_pruning_tests.rs:152:    assert_eq!(pr1.compute_pruned_root(), pr2.compute_pruned_root());
crates/amun-state-sync/src/sync_protocol.rs:273:        assert_eq!(imported.compute_state_root(), root);
crates/amun-state-sync/src/sync_protocol.rs:283:        assert_eq!(imported.compute_state_root(), root);
crates/amun-state-sync/src/sync_protocol.rs:379:        assert_eq!(reg.compute_state_root(), root);
crates/amun-state-types/src/tests.rs:11:fn test_state_verify_success() {
crates/amun-state-types/src/tests.rs:23:fn test_state_verify_failure() {
crates/amun-stateless-sync/src/lib.rs:268:        assert!(node.verify_height(0).is_ok());
crates/amun-stateless-sync/src/lib.rs:285:        assert!(node.verify_chain().is_ok());
crates/amun-stateless-sync/src/lib.rs:296:        assert!(node.verify_chain().is_ok());
crates/amun-stateless-sync/src/lib.rs:311:        assert!(node.verify_height(5).is_err());
crates/amun-stf/src/nonce.rs:27:    pub fn validate_nonce<S: StateStore>(
crates/amun-stf/tests/integration_test.rs:10:    let receipt = execute_transition_with_receipt(
crates/amun-stf/tests/integration_test.rs:27:    assert!(receipt.verify_consistency().is_ok());
crates/amun-stf/tests/integration_test.rs:2:use amun_stf::transition_result::{execute_transition_with_receipt, TransitionExecutionResult};
crates/amun-stf/tests/integration_test.rs:41:    let receipt = execute_transition_with_receipt(
crates/amun-stf/tests/integration_test.rs:68:        let receipt = execute_transition_with_receipt(
crates/amun-stf/tests/integration_test.rs:94:    assert!(transcript.verify_transcript().is_ok());
crates/amun-stf/tests/replay_equivalence.rs:11:fn execute_block(txs: &[[u8; 32]]) -> (ExecutionTranscript, Vec<u8>, [u8; 32]) {
crates/amun-stf/tests/replay_equivalence.rs:17:        let receipt = execute_transition_with_receipt(
crates/amun-stf/tests/replay_equivalence.rs:51:    let (live_transcript, live_state, live_root) = execute_block(&txs);
crates/amun-stf/tests/replay_equivalence.rs:52:    let (replay_transcript, replay_state, replay_root) = execute_block(&txs);
crates/amun-stf/tests/replay_equivalence.rs:5:use amun_stf::transition_result::{execute_transition_with_receipt, TransitionExecutionResult};
crates/amun-stf/tests/replay_equivalence.rs:83:    assert!(live_transcript.verify_transcript().is_ok());
crates/amun-stf/tests/replay_equivalence.rs:84:    assert!(replay_transcript.verify_transcript().is_ok());
crates/amun-stf/tests/replay_equivalence.rs:91:    let (live_transcript, _live_state, live_root) = execute_block(&txs);
crates/amun-stf/tests/replay_equivalence.rs:98:        let receipt = execute_transition_with_receipt(
crates/amun-storage-kernel/tests/replay_equivalence.rs:136:        let result = ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap());
crates/amun-storage-kernel/tests/replay_equivalence.rs:54:            ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap()).unwrap();
crates/amun-storage-kernel/tests/replay_equivalence.rs:94:        let result = ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap());
crates/amun-storage-kernel/tests/terminal_empty.rs:9:        // Verified by build_empty_ladder which sets ladder[256] = NodeHash::ZERO.
crates/amun-testnet-sim/tests/adversarial_tests.rs:117:    let mut reg = build_registry(10);
crates/amun-testnet-sim/tests/adversarial_tests.rs:124:        pre_state_root: reg.compute_state_root(),
crates/amun-testnet-sim/tests/adversarial_tests.rs:127:    let mut block = ReplayBackedConsensus::execute_and_replay(
crates/amun-testnet-sim/tests/adversarial_tests.rs:148:    let source_reg = build_registry(100);
crates/amun-testnet-sim/tests/adversarial_tests.rs:149:    let state_root_before = source_reg.compute_state_root();
crates/amun-testnet-sim/tests/adversarial_tests.rs:156:    assert_eq!(recovered_reg.compute_state_root(), state_root_before);
crates/amun-testnet-sim/tests/adversarial_tests.rs:167:    let initial_reg = build_registry(10);
crates/amun-testnet-sim/tests/adversarial_tests.rs:185:            pre_state_root: reg.compute_state_root(),
crates/amun-testnet-sim/tests/adversarial_tests.rs:207:                let mut fresh = build_registry(10); // Same genesis state
crates/amun-testnet-sim/tests/adversarial_tests.rs:22:fn build_registry(count: u64) -> ResourceRegistry {
crates/amun-testnet-sim/tests/adversarial_tests.rs:232:    let mut reg = build_registry(10);
crates/amun-testnet-sim/tests/adversarial_tests.rs:239:        pre_state_root: reg.compute_state_root(),
crates/amun-testnet-sim/tests/adversarial_tests.rs:242:    let mut block = ReplayBackedConsensus::execute_and_replay(
crates/amun-testnet-sim/tests/adversarial_tests.rs:263:    let source_reg = build_registry(100);
crates/amun-testnet-sim/tests/adversarial_tests.rs:264:    let state_root = source_reg.compute_state_root();
crates/amun-testnet-sim/tests/adversarial_tests.rs:270:    assert_eq!(result.unwrap().compute_state_root(), state_root);
crates/amun-testnet-sim/tests/adversarial_tests.rs:45:    let mut reg = build_registry(10);
crates/amun-testnet-sim/tests/adversarial_tests.rs:52:        pre_state_root: reg.compute_state_root(),
crates/amun-testnet-sim/tests/adversarial_tests.rs:55:    let block = ReplayBackedConsensus::execute_and_replay(
crates/amun-testnet-sim/tests/adversarial_tests.rs:88:    let mut reg = build_registry(10);
crates/amun-testnet-sim/tests/adversarial_tests.rs:95:        pre_state_root: reg.compute_state_root(),
crates/amun-testnet-sim/tests/adversarial_tests.rs:98:    let block = ReplayBackedConsensus::execute_and_replay(
crates/amun-tokenomics-ledger/tests/test_ledger.rs:57:    assert_eq!(l1.compute_ledger_root(), l2.compute_ledger_root());
crates/amun-tokenomics/tests/test_tokenomics.rs:19:    let (treasury, validators, ecosystem) = EpochEconomics::compute_distribution(reward);
crates/amun-tokenomics/tests/test_tokenomics.rs:8:    let reward = EpochEconomics::compute_epoch_rewards(total_supply);
crates/amun-transaction/src/tests.rs:49:    assert!(tx.validate_basic().is_ok());
crates/amun-transcript-semantics/src/lib.rs:102:    #[test] fn test_causal_chain() { let p = EventIdentity::new([0x01;32],[0x00;32],[0xAA;32],1,ReplayDomain::Consensus,[0xBB;32]); let c = EventIdentity::new([0x02;32],[0x01;32],[0xAA;32],2,ReplayDomain::Consensus,[0xBB;32]); assert!(c.verify_causal_chain(&p)); }
crates/amun-unlock-law/src/lib.rs:81:        assert!(verify_unlock_condition(&qc, &set));
crates/amun-validator-identity/src/signature.rs:68:        assert!(verify_ed25519(&pk, &payload, &signature));
crates/amun-validator-identity/src/signature.rs:97:        assert!(!verify_ed25519(&pk, &payload, &signature));
crates/amun-validator-networking/src/lib.rs:111:        assert_eq!(imported_reg.compute_state_root(), state_root);
crates/amun-wallet-management/src/lib.rs:61:        assert!(signer::verify_signature(&kp.public_key, message, &sig));
crates/amun-wallet-management/src/lib.rs:72:        assert!(!signer::verify_signature(&kp.public_key, message, &sig));
crates/amun-wallet-management/src/lib.rs:80:        assert!(signer::verify_signature(&kp.public_key, tx_bytes, &sig));
crates/amun_state_machine/src/snapshot.rs:104:        assert!(snap2.verify_chain(&snap1));
crates/amun_state_machine/src/snapshot.rs:105:        assert!(manager.verify_all());
## Tests
crates/amun-attestation/src/validator.rs:11:    pub fn create(signer: &Ed25519Signer, chain_id: u64, epoch: u64) -> Option<Self> {
crates/amun-attestation/src/validator.rs:24:    pub fn verify(&self) -> bool {
crates/amun-audit/tests/audit_layer01_physics.rs:101:    fn phys005_empty_root_is_constant() {
crates/amun-audit/tests/audit_layer01_physics.rs:14:    fn phys001_domain_separators_are_unique() {
crates/amun-audit/tests/audit_layer01_physics.rs:40:    fn phys002_hash_determinism() {
crates/amun-audit/tests/audit_layer01_physics.rs:59:    fn phys003_endian_consistency() {
crates/amun-audit/tests/audit_layer01_physics.rs:81:    fn phys004_serialization_stability() {
crates/amun-audit/tests/audit_layer02_geometry.rs:36:    fn geo002_empty_ladder_terminal() {
crates/amun-audit/tests/audit_layer02_geometry.rs:46:    fn geo003_empty_root_not_zero() {
crates/amun-audit/tests/audit_layer02_geometry.rs:56:    fn geo004_insert_delete_cycle() {
crates/amun-audit/tests/audit_layer02_geometry.rs:70:    fn geo005_max_depth_frozen() {
crates/amun-audit/tests/audit_layer03_snapshot.rs:10:    fn snap001_magic_bytes() {
crates/amun-audit/tests/audit_layer03_snapshot.rs:34:    fn snap002_manifest_self_verification() {
crates/amun-audit/tests/audit_layer03_snapshot.rs:56:    fn snap003_identity_determinism() {
crates/amun-audit/tests/audit_layer03_snapshot.rs:71:    fn snap004_self_compatibility() {
crates/amun-audit/tests/audit_layer04_byzantine.rs:54:    fn byz002_identity_mismatch_rejection() {
crates/amun-audit/tests/audit_layer04_byzantine.rs:9:    fn byz001_quorum_detection() {
crates/amun-audit/tests/audit_layer05_identity.rs:18:    fn id002_identity_self_verification() {
crates/amun-audit/tests/audit_layer05_identity.rs:28:    fn id003_encode_decode_roundtrip() {
crates/amun-audit/tests/audit_layer05_identity.rs:45:    fn id004_tampered_identity_detection() {
crates/amun-audit/tests/audit_layer05_identity.rs:7:    fn id001_different_hash_different_identity() {
crates/amun-audit/tests/audit_layer06_replay.rs:100:    fn replay003_epoch_regression() {
crates/amun-audit/tests/audit_layer06_replay.rs:27:    fn replay001_equivalence() {
crates/amun-audit/tests/audit_layer06_replay.rs:69:    fn replay002_divergence_detection() {
crates/amun-audit/tests/audit_layer06_replay.rs:9:    fn create_wal(path: &str, entries: &[WalEntry]) -> std::io::Result<()> {
crates/amun-audit/tests/audit_layer07_resources.rs:21:    fn res002_chunk_size_frozen() {
crates/amun-audit/tests/audit_layer07_resources.rs:8:    fn res001_allocation_guard() {
crates/amun-audit/tests/audit_layer08_domains.rs:13:    fn domain001_all_domains_unique() {
crates/amun-audit/tests/audit_layer08_domains.rs:38:    fn domain002_chain_id_32_bytes() {
crates/amun-audit/tests/audit_layer08_domains.rs:52:    fn domain003_domains_are_versioned() {
crates/amun-audit/tests/audit_layer09_freeze.rs:27:    fn freeze003_empty_root_stability() {
crates/amun-audit/tests/audit_layer09_freeze.rs:7:    fn freeze001_max_depth_is_256() {
crates/amun-audit/tests/audit_layer10_adversarial.rs:60:    fn adv003_delete_nonexistent_noop() {
crates/amun-audit/tests/audit_layer10_adversarial.rs:76:    fn adv004_insert_delete_insert_cycle() {
crates/amun-audit/tests/audit_layer10_adversarial.rs:8:    fn adv001_random_order_independence() {
crates/amun-audit/tests/audit_layer11_crash.rs:11:    fn crash001_truncated_wal_detection() {
crates/amun-audit/tests/audit_layer11_crash.rs:58:    fn crash002_partial_frame_rejection() {
crates/amun-audit/tests/audit_layer11_crash.rs:83:    fn crash003_mid_frame_corruption() {
crates/amun-audit/tests/audit_layer12_fuzzing.rs:20:    fn fuzz002_random_key_insertions() {
crates/amun-audit/tests/audit_layer13_differential.rs:27:    fn diff002_domain_hash_determinism() {
crates/amun-audit/tests/audit_layer13_differential.rs:45:    fn diff003_empty_root_consistency() {
crates/amun-audit/tests/audit_layer13_differential.rs:9:    fn diff001_canonical_encoding_determinism() {
crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:68:    fn mesh002_foreign_civilization_rejection() {
crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:9:    fn mesh001_conflicting_manifests_detected() {
crates/amun-audit/tests/audit_layer15_temporal.rs:27:    fn temp001_replay_twice_same_root() {
crates/amun-audit/tests/audit_layer15_temporal.rs:68:    fn temp002_temporal_order_independence() {
crates/amun-audit/tests/audit_layer15_temporal.rs:9:    fn create_wal(path: &str, entries: &[WalEntry]) -> std::io::Result<()> {
crates/amun-audit/tests/audit_layer16_mutation.rs:30:    fn mut003_empty_root_not_mutable() {
crates/amun-audit/tests/audit_layer16_mutation.rs:8:    fn mut001_max_depth_is_frozen() {
crates/amun-authority-registry/src/registry.rs:190:    fn n107_2_active_tracks_latest() {
crates/amun-bench/tests/n161_state_root_bench.rs:9:fn n161_bench_state_root_10k_nft() {
crates/amun-bench/tests/n163_wal_bench.rs:5:fn n163_bench_wal_write_and_read() {
crates/amun-benchmarks/benches/consensus_bench.rs:18:fn bench_single_round(c: &mut Criterion) {
crates/amun-benchmarks/benches/consensus_bench.rs:34:fn bench_multi_round_10(c: &mut Criterion) {
crates/amun-benchmarks/benches/consensus_bench.rs:58:fn bench_vote_serialization(c: &mut Criterion) {
crates/amun-benchmarks/benches/consensus_bench.rs:5:fn make_vote(voter: u8, height: u64, block_hash: [u8; 32]) -> ConsensusVote {
crates/amun-benchmarks/benches/storage_bench.rs:25:fn bench_append_100_records(c: &mut Criterion) {
crates/amun-benchmarks/benches/storage_bench.rs:38:fn bench_read_100_records(c: &mut Criterion) {
crates/amun-benchmarks/benches/storage_bench.rs:55:fn bench_record_serialization(c: &mut Criterion) {
crates/amun-benchmarks/benches/storage_bench.rs:5:fn make_record(h: u64) -> FinalizedChainRecord {
crates/amun-benchmarks/benches/sync_bench.rs:30:fn bench_snapshot_create_1k(c: &mut Criterion) {
crates/amun-benchmarks/benches/sync_bench.rs:40:fn bench_snapshot_import_1k(c: &mut Criterion) {
crates/amun-benchmarks/benches/sync_bench.rs:52:fn bench_snapshot_create_10k(c: &mut Criterion) {
crates/amun-benchmarks/benches/sync_bench.rs:5:fn create_test_registry(size: u64) -> ResourceRegistry {
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:23:fn n111_cca_state_root_preserved_through_block() {
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:58:fn n111_cca_state_root_changes_reflected_in_block() {
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:7:fn create_signed_transfer(seed: u8, nonce: u64, amount: u64, to: [u8; 32]) -> Transaction {
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:85:fn n111_cca_raw_state_root_differs_from_cca_state_root() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:14:fn n120_2_block_hash_changes_with_slashing_root() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:28:fn n120_2_same_root_same_hash() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:41:fn n120_2_empty_root_allowed() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:51:fn n120_2_different_roots_different_hashes() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:64:fn n120_2_zero_root_vs_nonzero_root_different_hash() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:76:fn n120_3_matching_root_accepted() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:86:fn n120_3_mismatched_root_rejected() {
crates/amun-block-builder/tests/n120_2_slashing_root.rs:97:fn n120_3_zero_root_verified_correctly() {
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:25:fn n132_3_6_economic_root_consistent_with_snapshot() {
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:54:fn n132_3_6_consistent_with_snapshot() {
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:7:fn create_signed_transfer(seed: u8, nonce: u64, amount: u64, to: [u8; 32]) -> Transaction {
crates/amun-block-builder/tests/n28_first_economic_block.rs:41:fn n28_multiple_transfers_in_block() {
crates/amun-block-builder/tests/n28_first_economic_block.rs:7:fn n28_first_economic_block() {
crates/amun-block-builder/tests/n28_first_economic_block.rs:89:fn n28_failed_transaction_in_block() {
crates/amun-block-builder/tests/n32_certified_block.rs:150:fn n32_certified_block_preserves_state_root() {
crates/amun-block-builder/tests/n32_certified_block.rs:24:fn n32_certified_block_created() {
crates/amun-block-builder/tests/n32_certified_block.rs:77:fn n32_multiple_certified_blocks() {
crates/amun-block/src/tests.rs:100:fn test_body_decode_rejects_high_tx_count() {
crates/amun-block/src/tests.rs:110:fn test_block_body_wire_freeze() {
crates/amun-block/src/tests.rs:126:fn test_block_body_wire_freeze_boundary() {
crates/amun-block/src/tests.rs:23:fn test_header_size_freeze() {
crates/amun-block/src/tests.rs:27:fn test_limits_freeze() {
crates/amun-block/src/tests.rs:31:fn test_header_roundtrip() {
crates/amun-block/src/tests.rs:43:fn test_body_add_tx_hash() {
crates/amun-block/src/tests.rs:50:fn test_body_rejects_exceeding_limit() {
crates/amun-block/src/tests.rs:63:fn test_block_roundtrip() {
crates/amun-block/src/tests.rs:79:fn test_block_id_deterministic() {
crates/amun-block/src/tests.rs:7:fn hdr() -> BlockHeader {
crates/amun-block/src/tests.rs:88:fn test_header_decode_rejects_short() {
crates/amun-block/src/tests.rs:92:fn test_decode_exact_rejects_trailing() {
crates/amun-bls/src/tests.rs:14:    fn test_sign_and_verify() {
crates/amun-bls/src/tests.rs:21:    fn test_invalid_signature_rejected() {
crates/amun-bls/src/tests.rs:28:    fn test_aggregate_signatures() {
crates/amun-bls/src/tests.rs:34:    fn test_aggregate_public_keys() {
crates/amun-bls/src/tests.rs:41:    fn test_aggregate_empty_rejected() {
crates/amun-bls/src/tests.rs:6:    fn test_keygen_deterministic() {
crates/amun-byzantine-tests/tests/attack_suite.rs:125:fn byz_004_version_regression_rejected() {
crates/amun-byzantine-tests/tests/attack_suite.rs:153:fn byz_005_parent_hash_forgery_rejected() {
crates/amun-byzantine-tests/tests/attack_suite.rs:17:fn make_id(seed: u64) -> ResourceId {
crates/amun-byzantine-tests/tests/attack_suite.rs:181:fn byz_006_illegal_transformation_rejected() {
crates/amun-byzantine-tests/tests/attack_suite.rs:209:fn byz_007_deep_lineage_no_crash() {
crates/amun-byzantine-tests/tests/attack_suite.rs:244:fn byz_008_wide_fanout_no_crash() {
crates/amun-byzantine-tests/tests/attack_suite.rs:59:        amun_replay_verifier::replay_verifier::ReplayResult::Match { .. }
crates/amun-byzantine-tests/tests/attack_suite.rs:64:fn byz_002_double_transfer_rejected() {
crates/amun-byzantine-tests/tests/attack_suite.rs:81:fn byz_003_lineage_cycle_rejected() {
crates/amun-byzantine-tests/tests/attack_suite.rs:9:use amun_replay_verifier::replay_verifier::ReplayVerifier;
crates/amun-canonical-collections/src/lib.rs:179:    #[test] fn test_replay_root_deterministic() { let mut a = CanonicalSet::new(); let mut b = CanonicalSet::new(); a.insert(1u64).unwrap(); a.insert(2u64).unwrap(); b.insert(2u64).unwrap(); b.insert(1u64).unwrap(); assert_eq!(a.canonical_root(), b.canonical_root()); }
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:102:fn n16_byzantine_source_mixed_valid_invalid() {
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:113:fn n16_empty_bundle_list_rejected() {
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:120:fn n16_duplicate_checkpoint_accepted() {
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:63:fn n16_forged_checkpoint_rejected() {
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:76:fn n16_tampered_bundle_rejected() {
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:85:fn n16_chain_gap_detected() {
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:94:fn n16_wrong_trusted_root_rejected() {
crates/amun-chain-store/examples/snapshot_builder_test.rs:26:fn main() {
crates/amun-chain-store/examples/snapshot_builder_test.rs:6:fn make_record(height: u64) -> FinalizedChainRecord {
crates/amun-chain-store/examples/snapshot_restore_test.rs:26:fn main() {
crates/amun-chain-store/examples/snapshot_restore_test.rs:6:fn make_record(height: u64) -> FinalizedChainRecord {
crates/amun-chain-store/examples/snapshot_verification_test.rs:26:fn main() {
crates/amun-chain-store/examples/snapshot_verification_test.rs:6:fn make_record(height: u64) -> FinalizedChainRecord {
crates/amun-chain-store/src/store.rs:126:    pub fn latest_height(&self) -> u64 {
crates/amun-chain-store/tests/n120_2_record_roundtrip.rs:4:fn n120_2_record_roundtrip_preserves_slashing_root() {
crates/amun-cli/src/main.rs:35:fn prompt_password(prompt: &str) -> String {
crates/amun-codec/src/tests.rs:101:fn test_bytes32_roundtrip() {
crates/amun-codec/src/tests.rs:10:fn test_u8_encode() {
crates/amun-codec/src/tests.rs:115:fn test_decode_short_buffer_rejected() {
crates/amun-codec/src/tests.rs:122:fn test_decode_exact_trailing_bytes_rejected() {
crates/amun-codec/src/tests.rs:130:fn test_decode_exact_exact_fits() {
crates/amun-codec/src/tests.rs:137:fn test_encode_buffer_too_small() {
crates/amun-codec/src/tests.rs:147:fn test_epoch_roundtrip() {
crates/amun-codec/src/tests.rs:156:fn test_round_roundtrip() {
crates/amun-codec/src/tests.rs:165:fn test_hash32_roundtrip() {
crates/amun-codec/src/tests.rs:174:fn test_validator_id_roundtrip() {
crates/amun-codec/src/tests.rs:17:fn test_u16_little_endian_freeze() {
crates/amun-codec/src/tests.rs:187:fn test_domain_separation_all_distinct() {
crates/amun-codec/src/tests.rs:198:fn test_domain_hash_deterministic() {
crates/amun-codec/src/tests.rs:205:fn test_domain_from_byte_valid() {
crates/amun-codec/src/tests.rs:24:fn test_u32_little_endian_freeze() {
crates/amun-codec/src/tests.rs:31:fn test_u64_little_endian_freeze() {
crates/amun-codec/src/tests.rs:41:fn test_u128_little_endian_freeze() {
crates/amun-codec/src/tests.rs:51:fn test_u8_roundtrip() {
crates/amun-codec/src/tests.rs:61:fn test_u16_roundtrip() {
crates/amun-codec/src/tests.rs:71:fn test_u32_roundtrip() {
crates/amun-codec/src/tests.rs:81:fn test_u64_roundtrip() {
crates/amun-codec/src/tests.rs:91:fn test_u128_roundtrip() {
crates/amun-consensus-network/src/evidence_gossip.rs:170:    fn n111_4_valid_announcement_passes() {
crates/amun-consensus-network/src/vote_binding.rs:132:    fn n109_9_valid_vote_with_commitment_passes() {
crates/amun-consensus-network/src/vote_binding.rs:196:    fn n109_9_legacy_vote_without_commitment_passes() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:107:fn n109_8_commitment_roundtrip() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:124:fn n109_8_same_execution_same_commitment() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:140:fn n109_8_different_execution_different_commitment() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:155:fn n109_8_tampered_commitment_rejected() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:178:fn n109_8_validator_cannot_repudiate() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:202:fn n109_8_different_validator_different_commitment() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:224:fn n109_8_vote_commitment_matches_vote_target() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:269:fn n109_8_mismatched_vote_commitment_detected() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:299:fn n109_8_vote_with_commitment_roundtrip() {
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:68:fn make_commitment(
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:86:fn make_vote(
crates/amun-consensus-network/tests/n109_block_propagation.rs:110:fn n109_proposal_roundtrip() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:127:fn n109_block_hash_matches_serialized_block() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:140:fn n109_network_message_vote_roundtrip() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:169:fn n109_network_message_proposal_roundtrip() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:193:fn n109_proposal_cache_insert() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:211:fn n109_proposal_cache_cleanup() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:229:fn n109_listener_stores_proposal() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:319:fn n109_vote_before_proposal_does_not_crash() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:342:fn n109_duplicate_proposal_is_idempotent() {
crates/amun-consensus-network/tests/n109_block_propagation.rs:51:fn make_test_proposal(height: u64, parent: [u8; 32]) -> BlockProposal {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:122:fn n109_7_re_execution_rejects_mismatched_state_root() {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:154:fn n109_7_three_validators_one_mismatch_no_qc() {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:233:fn n109_7b_metrics_counts_state_root_mismatches() {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:264:fn n109_7_execution_failure_does_not_vote() {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:297:fn n109_7c_proposal_retained_until_finalized() {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:341:fn n109_7_execution_is_deterministic() {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:363:fn n109_7_different_blocks_different_roots() {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:81:fn simulate_executor(block_bytes: &[u8], seed: u64) -> Result<[u8; 32], String> {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:93:fn n109_7_re_execution_accepts_matching_state_root() {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:38:fn make_evidence_record(
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:57:fn n111_7_evidence_sync_end_to_end() {
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:154:fn n112_4_push_duplicate_is_harmless() {
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:16:fn make_evidence(validator_id: [u8; 32], height: u64, seed: u8) -> EvidenceRecord {
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:49:fn n112_4_push_sync_end_to_end() {
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:105:fn n121_2_single_event_replay_deterministic() {
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:122:fn n121_2_consistency_verification_catches_corruption() {
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:31:fn n121_2_three_nodes_same_replay_same_root() {
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:71:fn n121_2_different_order_different_root() {
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:94:fn n121_2_replay_empty_produces_zero_root() {
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:130:fn n121_6_total_slash_count_auditable() {
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:147:fn n121_6_empty_state_empty_history() {
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:32:fn n121_5_snapshot_root_matches_state() {
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:57:fn n121_5_restored_state_from_snapshot() {
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:87:fn n121_6_audit_trail_by_validator() {
crates/amun-consensus-types/src/tests.rs:18:fn test_phase_is_vote() {
crates/amun-consensus-types/src/tests.rs:28:fn test_round_encoding() {
crates/amun-consensus-types/src/tests.rs:37:fn test_bitmap_set_get() {
crates/amun-consensus-types/src/tests.rs:44:fn test_vote_wire_size_frozen() {
crates/amun-consensus-types/src/tests.rs:49:fn test_qc_wire_size_frozen() {
crates/amun-consensus-types/src/tests.rs:54:fn test_vote_new_rejects_proposal() {
crates/amun-consensus-types/src/tests.rs:67:fn test_qc_decode_rejects_proposal() {
crates/amun-consensus-types/src/tests.rs:74:fn test_message_roundtrip() {
crates/amun-consensus-types/src/tests.rs:8:fn test_phase_discriminants_frozen() {
crates/amun-consensus/amun_consensus/src/block/mod.rs:138:    fn test_block_header_hash() {
crates/amun-consensus/amun_consensus/src/block/mod.rs:154:    fn test_block_body_sorting() {
crates/amun-contract-events/tests/n173_events_storage_tests.rs:14:fn n173_emit_and_query_events() {
crates/amun-contract-events/tests/n173_events_storage_tests.rs:24:fn n173_events_root_deterministic() {
crates/amun-contract-events/tests/n173_events_storage_tests.rs:5:fn n173_store_and_retrieve() {
crates/amun-contract-fuzzing/src/lib.rs:30:    pub fn passed(&self) -> bool {
crates/amun-contract-fuzzing/tests/n171_fuzzing_tests.rs:19:fn n171_fuzz_call_5000() {
crates/amun-contract-fuzzing/tests/n171_fuzzing_tests.rs:36:fn n171_fuzz_gas_limits_5000() {
crates/amun-contract-fuzzing/tests/n171_fuzzing_tests.rs:4:fn n171_fuzz_deploy_10000() {
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:38:fn n167_contract_evidence_root_deterministic() {
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:52:fn n167_invalid_program_rejected() {
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:60:fn n167_contract_state_persistence() {
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:6:fn n167_deploy_and_call_contract() {
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:53:fn n169_multiple_contracts_independent() {
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:73:fn n169_contract_registry_root_deterministic() {
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:9:fn n169_contract_interacts_with_nft() {
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:32:fn n172_contract_a_cannot_modify_contract_b_state() {
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:6:fn n172_cross_contract_call_succeeds() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:10:fn n170_audit_gas_exhaustion_pass() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:16:fn n170_audit_state_isolation_pass() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:22:fn n170_audit_determinism_pass() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:28:fn n170_audit_malicious_bytecode_pass() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:34:fn n170_audit_evidence_consistency_pass() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:40:fn n170_full_security_suite() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:4:fn n170_audit_reentrancy_pass() {
crates/amun-contract-upgrade/tests/n174_upgrade_tests.rs:8:fn n174_upgrade_contract_success() {
crates/amun-core-optimization/tests/n161_optimization_tests.rs:10:fn n161_compare_cached_vs_uncached() {
crates/amun-defi-amm/tests/n153_amm_tests.rs:19:fn n153_pool_evidence_root_deterministic() {
crates/amun-defi-amm/tests/n153_amm_tests.rs:36:fn n153_swap_changes_evidence_root() {
crates/amun-defi-amm/tests/n153_amm_tests.rs:5:fn n153_create_pool_and_swap() {
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:11:fn n153_swap_evidence_differs_by_amount() {
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:18:fn n153_liquidity_evidence_deterministic() {
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:4:fn n153_swap_evidence_deterministic() {
crates/amun-defi-governance/tests/n157_governance_tests.rs:15:fn n157_governance_root_deterministic() {
crates/amun-defi-governance/tests/n157_governance_tests.rs:31:fn n157_parameter_change_detected() {
crates/amun-defi-governance/tests/n157_governance_tests.rs:4:fn n157_propose_vote_and_execute() {
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:25:fn n154_interest_accrual_increases_debt() {
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:44:fn n154_liquidation_triggered_when_health_factor_low() {
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:5:fn n154_loan_creation_and_repayment() {
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:63:fn n154_full_repayment_closes_loan() {
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:83:fn n154_lending_root_deterministic() {
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:16:fn n155_cannot_mint_above_collateral_ratio() {
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:24:fn n155_stablecoin_root_deterministic() {
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:5:fn n155_mint_and_burn() {
crates/amun-defi-stress/src/lib.rs:34:    pub fn passed(&self) -> bool {
crates/amun-defi-stress/tests/n158_stress_tests.rs:15:fn n158_stress_lending_liquidations_500() {
crates/amun-defi-stress/tests/n158_stress_tests.rs:26:fn n158_stress_stablecoin_mint_burn_1000() {
crates/amun-defi-stress/tests/n158_stress_tests.rs:37:fn n158_stress_nft_collateral_flow_100() {
crates/amun-defi-stress/tests/n158_stress_tests.rs:48:fn n158_full_defi_integration_stress() {
crates/amun-defi-stress/tests/n158_stress_tests.rs:4:fn n158_stress_amm_swaps_1000() {
crates/amun-economics/src/tests.rs:10:fn test_token_transfer_fail() {
crates/amun-economics/src/tests.rs:15:fn test_token_stake() {
crates/amun-economics/src/tests.rs:22:fn test_token_slash() {
crates/amun-economics/src/tests.rs:29:fn test_fee() {
crates/amun-economics/src/tests.rs:34:fn test_reward() {
crates/amun-economics/src/tests.rs:40:fn test_treasury() {
crates/amun-economics/src/tests.rs:4:fn test_token_transfer() {
crates/amun-evidence-finality/src/evidence_finality.rs:337:        assert_eq!(rr, &cert.replay_root);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:44:fn n40_evidence_chain_across_blocks() {
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:76:fn n40_evidence_root_changes_with_state() {
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:8:fn n40_evidence_backed_block_created() {
crates/amun-evidence/src/tests.rs:6:    fn test_evidence_type_creation() {
crates/amun-execution/src/tests.rs:14:fn test_gas_table_new_consistent() {
crates/amun-execution/src/tests.rs:37:fn test_canonical_nan_constants() {
crates/amun-execution/src/tests.rs:43:fn test_allowed_instructions_not_empty() {
crates/amun-execution/src/tests.rs:56:fn test_verified_interpreter_new() {
crates/amun-execution/src/tests.rs:5:fn test_gas_table_default() {
crates/amun-experimental-framework/src/main.rs:14:fn make_id(seed: u64) -> ResourceId {
crates/amun-experimental-framework/src/main.rs:176:fn exp1_state_scale() {
crates/amun-experimental-framework/src/main.rs:212:        let stats = measure_us(&format!("replay_{}_active", size), 5, 30, || {
crates/amun-experimental-framework/src/main.rs:224:        &["active_resources", "replay_time_us", "ci95_us"],
crates/amun-experimental-framework/src/main.rs:230:fn exp2_replay_vs_execute() {
crates/amun-experimental-framework/src/main.rs:309:        let replay_stats = measure_us(&format!("replay_{}", name), 5, 30, || {
crates/amun-experimental-framework/src/main.rs:314:        let speedup = exec_stats.mean / replay_stats.mean;
crates/amun-experimental-framework/src/main.rs:319:            format!("{:.4}", replay_stats.mean),
crates/amun-experimental-framework/src/main.rs:325:        &["workload", "execution_us", "replay_us", "speedup"],
crates/amun-experimental-framework/src/main.rs:331:fn exp3_full_pipeline() {
crates/amun-experimental-framework/src/main.rs:35:fn measure_us(name: &str, warmup: u32, iterations: u32, mut f: impl FnMut()) -> Stats {
crates/amun-experimental-framework/src/main.rs:389:fn exp4_cycle_detection() {
crates/amun-experimental-framework/src/main.rs:483:fn exp6_law_verification() {
crates/amun-experimental-framework/src/main.rs:507:fn main() {
crates/amun-experimental-framework/src/main.rs:511:    exp2_replay_vs_execute();
crates/amun-experimental-framework/src/main.rs:54:fn write_csv(filename: &str, header: &[&str], rows: &[Vec<String>]) {
crates/amun-experimental-framework/src/main.rs:6:use amun_replay_verifier::replay_verifier::ReplayVerifier;
crates/amun-failure/src/tests.rs:101:fn test_kernel_state_check_healthy() {
crates/amun-failure/src/tests.rs:107:fn test_kernel_state_record_fault() {
crates/amun-failure/src/tests.rs:119:fn test_fault_severity_ordering() {
crates/amun-failure/src/tests.rs:23:fn test_degraded_faults_no_halt() {
crates/amun-failure/src/tests.rs:31:fn test_rejected_faults_no_halt() {
crates/amun-failure/src/tests.rs:41:fn test_failure_context_creation() {
crates/amun-failure/src/tests.rs:53:fn test_failure_context_severity() {
crates/amun-failure/src/tests.rs:63:fn test_kernel_health_healthy() {
crates/amun-failure/src/tests.rs:69:fn test_kernel_health_poisoned() {
crates/amun-failure/src/tests.rs:6:fn test_all_fault_severities() {
crates/amun-failure/src/tests.rs:75:fn test_kernel_health_idempotent_poison() {
crates/amun-failure/src/tests.rs:91:fn test_quarantine_actions() {
crates/amun-finality-law/src/finality.rs:40:    fn test_finality() {
crates/amun-finality-law/src/finality.rs:69:    fn test_not_finalized_wrong_parent() {
crates/amun-fork-choice-law/src/fork_choice.rs:71:    fn test_select_best_qc() {
crates/amun-fork-choice-law/src/fork_choice.rs:82:    fn test_qc_extends() {
crates/amun-fork-choice-law/src/fork_choice.rs:91:    fn test_find_chain_simple() {
crates/amun-gossip/src/tests.rs:18:fn test_dedup_detects_duplicate() {
crates/amun-gossip/src/tests.rs:26:fn test_fanout_selects_correct_count() {
crates/amun-gossip/src/tests.rs:33:fn test_broadcaster_rejects_duplicate() {
crates/amun-gossip/src/tests.rs:46:fn test_receiver_counts_unique() {
crates/amun-gossip/src/tests.rs:56:fn test_retry_backoff() {
crates/amun-gossip/src/tests.rs:5:fn test_topic_roundtrip() {
crates/amun-governance/src/tests.rs:16:fn test_proposal_failing() {
crates/amun-governance/src/tests.rs:4:fn test_proposal_new() {
crates/amun-governance/src/tests.rs:9:fn test_proposal_passing() {
crates/amun-header-gossip/tests/gossip_tests.rs:4:fn test_gossip_message_creation() {
crates/amun-kernel-types/src/tests.rs:13:fn test_epoch_previous() {
crates/amun-kernel-types/src/tests.rs:17:fn test_epoch_default_zero() {
crates/amun-kernel-types/src/tests.rs:21:fn test_round_overflow() {
crates/amun-kernel-types/src/tests.rs:25:fn test_round_next_valid() {
crates/amun-kernel-types/src/tests.rs:29:fn test_round_previous() {
crates/amun-kernel-types/src/tests.rs:33:fn test_hash32_default() {
crates/amun-kernel-types/src/tests.rs:37:fn test_hash32_new() {
crates/amun-kernel-types/src/tests.rs:41:fn test_secret_hash32_drop() {
crates/amun-kernel-types/src/tests.rs:45:fn test_validator_id_default() {
crates/amun-kernel-types/src/tests.rs:49:fn test_public_key_default() {
crates/amun-kernel-types/src/tests.rs:53:fn test_signature_default() {
crates/amun-kernel-types/src/tests.rs:57:fn test_newtypes() {
crates/amun-kernel-types/src/tests.rs:5:fn test_epoch_overflow() {
crates/amun-kernel-types/src/tests.rs:65:fn test_capacity_constants() {
crates/amun-kernel-types/src/tests.rs:9:fn test_epoch_next_valid() {
crates/amun-kernel/src/canonical.rs:186:    fn test_deterministic_hash() {
crates/amun-kernel/src/canonical.rs:193:    fn test_domain_separation() {
crates/amun-kernel/src/canonical.rs:200:    fn test_vec_encoding() {
crates/amun-kernel/src/canonical.rs:208:    fn test_optional_encoding() {
crates/amun-keystore/src/store.rs:41:    pub fn decrypt(&self, password: &str) -> Result<Vec<u8>, &'static str> {
crates/amun-light-client/tests/light_client_tests.rs:11:fn make_id(seed: u8) -> ResourceId {
crates/amun-light-client/tests/light_client_tests.rs:17:fn make_cert(
crates/amun-light-client/tests/light_client_tests.rs:58:fn n55_light_client_full_workflow() {
crates/amun-light-client/tests/light_client_tests.rs:87:fn n55_light_client_rejects_broken_chain() {
crates/amun-live-cluster/src/bin/benchmark.rs:6:fn main() {
crates/amun-live-cluster/src/bin/benchmark_10.rs:6:fn main() {
crates/amun-live-cluster/src/bin/benchmark_10_event.rs:6:fn main() {
crates/amun-live-cluster/src/bin/byzantine_partition_test.rs:6:fn main() {
crates/amun-live-cluster/src/bin/byzantine_test.rs:9:fn main() {
crates/amun-live-cluster/src/bin/latency_audit.rs:6:fn main() {
crates/amun-live-cluster/src/bin/load_test.rs:9:fn main() {
crates/amun-live-cluster/src/bin/multi_validator_test.rs:6:fn main() {
crates/amun-live-cluster/src/bin/partition_test.rs:6:fn main() {
crates/amun-live-cluster/src/bin/quorum_exclusion_test.rs:8:fn main() {
crates/amun-live-cluster/src/bin/soak_test.rs:6:fn main() {
crates/amun-live-cluster/src/config.rs:61:    pub fn test_cluster(validator_index: usize, ports: &[u16; 4]) -> Self {
crates/amun-live-cluster/src/testing/mod.rs:13:pub fn free_ports<const N: usize>() -> [u16; N] {
crates/amun-live-cluster/src/testing/mod.rs:23:pub fn unique_test_dir(name: &str, validator: usize) -> String {
crates/amun-live-cluster/src/testing/mod.rs:34:pub fn cleanup(path: &str) {
crates/amun-live-cluster/src/testing/mod.rs:5:pub fn free_port() -> u16 {
crates/amun-live-cluster/tests/n102_catchup_test.rs:8:fn n102_3_catchup_after_50_block_gap() {
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:10:fn make_pk(id: u8) -> PublicKey {
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:122:fn n110_4c_applied_hashes_prevent_replay() {
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:16:fn pk_to_id(pk: &PublicKey) -> [u8; 32] {
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:42:fn n110_4c_slash_applied_after_finality() {
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:12:fn n118_slashing_executes_after_finality() {
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:23:fn n120_4b_validator_rejects_mismatched_slashing_root() {
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:38:fn n120_4c_block_with_tampered_root_rejected() {
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:58:fn n120_4d_mismatched_root_prevents_voting() {
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:7:fn n120_4a_validator_accepts_matching_slashing_root() {
crates/amun-live-cluster/tests/n129_evidence_continuity_audit.rs:8:fn n129_4_evidence_continuity_audit() {
crates/amun-load-generator/src/main.rs:143:fn run_load_test(config: LoadConfig) -> LoadResult {
crates/amun-mempool/src/lib.rs:58:    fn create_test_tx(sender_seed: u8, nonce: u64, amount: u64) -> Transaction {
crates/amun-merkle/src/tests.rs:13:fn test_leaf_hash_freeze() {
crates/amun-merkle/src/tests.rs:20:fn test_internal_hash_freeze() {
crates/amun-merkle/src/tests.rs:31:fn test_single_leaf_root() {
crates/amun-merkle/src/tests.rs:38:fn test_two_leaf_root_deterministic() {
crates/amun-merkle/src/tests.rs:48:fn test_three_leaf_root_deterministic() {
crates/amun-merkle/src/tests.rs:59:fn test_odd_leaf_duplication_rule() {
crates/amun-merkle/src/tests.rs:69:fn test_different_order_produces_different_root() {
crates/amun-merkle/src/tests.rs:6:fn test_empty_root_freeze() {
crates/amun-network-fastpath/src/lib.rs:61:pub fn benchmark_batching(message_count: u64, batch_size: usize) -> FastPathResult {
crates/amun-network-fastpath/tests/n164_fastpath_tests.rs:29:fn n164_large_message_throughput() {
crates/amun-network-fastpath/tests/n164_fastpath_tests.rs:44:fn n164_batch_hash_deterministic() {
crates/amun-network-fastpath/tests/n164_fastpath_tests.rs:4:fn n164_message_batching_reduces_overhead() {
crates/amun-network-simulator/src/delivery.rs:16:    pub fn latency_for(&self, sender: u64, receiver: u64) -> u64 {
crates/amun-network-simulator/src/delivery.rs:9:    pub fn new(base_latency_rounds: u64, jitter_rounds: u64) -> Self {
crates/amun-network/src/tests.rs:12:fn test_peer_state_transitions() {
crates/amun-network/src/tests.rs:23:fn test_discovery_add_peer() {
crates/amun-network/src/tests.rs:32:fn test_connection_limit() {
crates/amun-network/src/tests.rs:47:fn test_rate_limiter() {
crates/amun-network/src/tests.rs:56:fn test_heartbeat_timeout() {
crates/amun-network/src/tests.rs:5:fn make_addr() -> heapless::String<128> {
crates/amun-network/src/tests.rs:62:fn test_framing_encode() {
crates/amun-networking/tests/harness/event_scheduler.rs:103:    pub fn is_empty(&self) -> bool {
crates/amun-networking/tests/harness/event_scheduler.rs:23:    fn cmp(&self, other: &Self) -> Ordering {
crates/amun-networking/tests/harness/event_scheduler.rs:31:    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
crates/amun-networking/tests/harness/event_scheduler.rs:36:    fn eq(&self, other: &Self) -> bool {
crates/amun-networking/tests/harness/event_scheduler.rs:43:    fn transform_time(&self, time: u64, _event_type: &EventType) -> u64 {
crates/amun-networking/tests/harness/event_scheduler.rs:46:    fn filter_event(&self, _event: &ScheduledEvent) -> bool {
crates/amun-networking/tests/harness/event_scheduler.rs:61:    pub fn new() -> Self {
crates/amun-networking/tests/harness/event_scheduler.rs:69:    pub fn with_policy(policy: Box<dyn SchedulingPolicy>) -> Self {
crates/amun-networking/tests/harness/event_scheduler.rs:77:    pub fn schedule(&mut self, time: u64, node_id: String, event_type: EventType) {
crates/amun-networking/tests/harness/event_scheduler.rs:98:    pub fn next_event(&mut self) -> Option<ScheduledEvent> {
crates/amun-networking/tests/harness/handlers.rs:69:pub fn process_envelope(
crates/amun-networking/tests/harness/message_delivery.rs:15:    fn default() -> Self {
crates/amun-networking/tests/harness/message_delivery.rs:38:    pub fn new(policy: DeliveryPolicy, seed: u64) -> Self {
crates/amun-networking/tests/harness/message_delivery.rs:46:    pub fn schedule(&mut self, delivery_time: u64, recipient: String, envelope: Envelope) {
crates/amun-networking/tests/harness/message_delivery.rs:54:    pub fn broadcast(
crates/amun-networking/tests/harness/message_delivery.rs:84:    pub fn drain_ready(&mut self, current_time: u64) -> Vec<DelayedEnvelope> {
crates/amun-networking/tests/harness/scenario.rs:11:    fn config(&self) -> ScenarioConfig;
crates/amun-networking/tests/harness/scenario.rs:12:    fn schedule_events(&self, scheduler: &mut EventScheduler, node_ids: &[String]);
crates/amun-networking/tests/harness/scenario.rs:26:    fn default() -> Self {
crates/amun-networking/tests/harness/scenario.rs:50:    pub fn new(config: ScenarioConfig, _max_time_ms: u64) -> Self {
crates/amun-networking/tests/harness/scenario.rs:65:    pub fn run(&mut self, _scenario: &dyn ConsensusScenario) -> ScenarioResult {
crates/amun-networking/tests/harness/scenario.rs:73:pub fn drain_and_broadcast(core: &mut SimulationNodeCore) -> Vec<Envelope> {
crates/amun-networking/tests/harness/simulation_node.rs:21:    pub fn new(validator: Validator, state_machine: RoundStateMachine) -> Self {
crates/amun-networking/tests/harness/simulation_node.rs:34:    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
crates/amun-networking/tests/n17_multi_node_network.rs:101:fn n17_four_node_network_reaches_first_commit() {
crates/amun-networking/tests/n17_multi_node_network.rs:107:fn n17_seven_node_network_reaches_first_commit() {
crates/amun-networking/tests/n17_multi_node_network.rs:113:fn n17_node_crash_and_recovery() {
crates/amun-networking/tests/n17_multi_node_network.rs:147:fn n17_bootstrap_trusted_root_persists() {
crates/amun-networking/tests/n17_multi_node_network.rs:154:fn n17_all_nodes_eventually_commit_multiple_blocks() {
crates/amun-networking/tests/n17_multi_node_network.rs:23:    fn new(count: usize) -> Self {
crates/amun-networking/tests/n17_multi_node_network.rs:42:    fn tick(&mut self) {
crates/amun-networking/tests/n17_multi_node_network.rs:85:    fn run_until_commits(&mut self, max_ticks: usize, target_commits: usize) -> bool {
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:22:fn n18_full_lifecycle_with_catchup() {
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:50:fn n18_catchup_preserves_height_after_activation() {
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:65:fn n18_sync_request_roundtrip() {
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:73:fn n18_sync_response_serialization_roundtrip() {
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:89:fn n18_full_rejoin_matches_network_state() {
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:9:fn n18_catchup_import_checkpoint_height() {
crates/amun-networking/tests/n18_checkpoint_sync.rs:100:fn n18_bootstrapping_node_rejects_proposal_before_activation() {
crates/amun-networking/tests/n18_checkpoint_sync.rs:115:fn n18_sync_response_with_multiple_checkpoints() {
crates/amun-networking/tests/n18_checkpoint_sync.rs:52:fn n18_checkpoint_sync_between_nodes() {
crates/amun-networking/tests/n18_full_rejoin.rs:115:fn n18_rejoin_preserves_network_height_across_multiple_checkpoints() {
crates/amun-networking/tests/n18_full_rejoin.rs:146:fn n18_bootstrapping_node_cannot_activate_directly() {
crates/amun-networking/tests/n18_full_rejoin.rs:56:fn n18_rejoin_after_network_progress() {
crates/amun-networking/tests/n18_node_rejoin.rs:19:fn n18_active_node_can_propose() {
crates/amun-networking/tests/n18_node_rejoin.rs:30:fn n18_lifecycle_transitions() {
crates/amun-networking/tests/n18_node_rejoin.rs:48:fn n18_bootstrapping_node_stores_trusted_root() {
crates/amun-networking/tests/n18_node_rejoin.rs:59:fn n18_full_rejoin_after_crash() {
crates/amun-networking/tests/n18_node_rejoin.rs:9:fn n18_bootstrapping_node_cannot_propose() {
crates/amun-networking/tests/n19_adversarial_rejoin.rs:100:fn n19_checkpoint_gap_detected() {
crates/amun-networking/tests/n19_adversarial_rejoin.rs:128:fn n19_mixed_checkpoint_stream_rejected() {
crates/amun-networking/tests/n19_adversarial_rejoin.rs:155:fn n19_byzantine_rejoin_source_rejected() {
crates/amun-networking/tests/n19_adversarial_rejoin.rs:54:fn n19_wrong_trusted_root_rejected() {
crates/amun-networking/tests/n19_adversarial_rejoin.rs:72:fn n19_checkpoint_rollback_rejected() {
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:124:fn n20_10_new_node_bootstraps_from_existing_peer() {
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:164:fn n20_10_four_nodes_start_together() {
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:197:fn n20_10_network_survives_disconnect() {
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:23:fn find_available_port() -> u16 {
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:32:fn n20_10_tcp_bind_and_connect() {
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:59:fn n20_10_peer_identity_exchange() {
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:119:fn n20_8_wrong_trusted_root_rejected_over_tcp() {
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:132:fn n20_8_empty_bootstrap_rejected() {
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:146:fn n20_8_activation_after_successful_bootstrap() {
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:56:fn n20_8_bootstrap_request_roundtrip() {
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:64:fn n20_8_bootstrap_response_roundtrip() {
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:81:fn n20_8_node_bootstraps_over_tcp() {
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:106:fn n20_9_rejoin_preserves_height_after_long_absence() {
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:130:fn n20_9_rejoin_rejects_outdated_checkpoint() {
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:161:fn n20_9_rejoin_requires_full_lifecycle() {
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:55:fn n20_9_rejoin_after_crash_over_tcp() {
crates/amun-networking/tests/v2_001_time_driven_consensus.rs:10:    fn schedule_events(&self, _scheduler: &mut EventScheduler, _node_ids: &[String]) {}
crates/amun-networking/tests/v2_001_time_driven_consensus.rs:11:    fn config(&self) -> ScenarioConfig {
crates/amun-networking/tests/v2_001_time_driven_consensus.rs:24:fn test_happy_path_consensus() {
crates/amun-networking/tests/v2_002_multi_round_consensus.rs:11:    fn config(&self) -> ScenarioConfig {
crates/amun-networking/tests/v2_002_multi_round_consensus.rs:21:    fn schedule_events(&self, s: &mut EventScheduler, nids: &[String]) {
crates/amun-networking/tests/v2_002_multi_round_consensus.rs:47:fn test_multi_round_consensus() {
crates/amun-networking/tests/v2_003_multi_height_consensus.rs:11:    fn config(&self) -> ScenarioConfig {
crates/amun-networking/tests/v2_003_multi_height_consensus.rs:21:    fn schedule_events(&self, s: &mut EventScheduler, nids: &[String]) {
crates/amun-networking/tests/v2_003_multi_height_consensus.rs:46:fn test_multi_height_consensus() {
crates/amun-networking/tests/v2_004_long_run_stability.rs:12:    fn config(&self) -> ScenarioConfig {
crates/amun-networking/tests/v2_004_long_run_stability.rs:22:    fn schedule_events(&self, scheduler: &mut EventScheduler, node_ids: &[String]) {
crates/amun-networking/tests/v2_004_long_run_stability.rs:47:fn test_long_run_stability() {
crates/amun-networking/tests/v2_013_baseline_matrix.rs:14:    fn new(delay_ms: u64, loss_rate: f64) -> Self {
crates/amun-networking/tests/v2_013_baseline_matrix.rs:23:    fn schedule_events(&self, _scheduler: &mut EventScheduler, _node_ids: &[String]) {
crates/amun-networking/tests/v2_013_baseline_matrix.rs:26:    fn config(&self) -> ScenarioConfig {
crates/amun-networking/tests/v2_013_baseline_matrix.rs:39:fn test_baseline_matrix() {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:122:    fn new(
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:151:    fn add_node(&mut self, id: String, validator: Validator, pacemaker_config: PacemakerConfig) {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:171:    fn schedule_delivery(&mut self, delivery_time: u64, recipient: String, envelope: Envelope) {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:181:    fn deliver_ready_messages(&mut self, current_time: u64) {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:195:    fn broadcast_message(&mut self, sender_id: &str, envelope: Envelope, include_self: bool) {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:220:    fn process_event(&mut self, event: ScheduledEvent) {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:349:                let _ = node.state_machine.finalize_commit();
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:373:    fn run_timeout_recovery_cycle(&mut self, inject_after_rounds: u64) {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:39:    fn cmp(&self, other: &Self) -> Ordering {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:406:fn test_timeout_injection_recovery() {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:47:    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:52:    fn eq(&self, other: &Self) -> bool {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:63:    fn new() -> Self {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:69:    fn schedule(&mut self, time: u64, node_id: String, event_type: EventType) {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:78:    fn next_event(&mut self) -> Option<ScheduledEvent> {
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:102:fn n149_unauthorized_transfer_rejected() {
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:11:fn n149_double_mint_rejected() {
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:123:fn n149_bridge_locked_sale_rejected() {
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:57:fn n149_double_spend_prevented() {
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:83:fn n149_invalid_evidence_rejected() {
crates/amun-nft-benchmark/src/lib.rs:10:    fn it_works() {
crates/amun-nft-benchmark/src/lib.rs:1:pub fn add(left: u64, right: u64) -> u64 {
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:122:fn n152_benchmark_state_root_10k() {
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:16:fn n152_benchmark_mint_10k_nfts() {
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:63:fn n152_benchmark_rapid_trades() {
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:9:fn unique_id(seed: u64) -> [u8; 32] {
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:30:fn n139_unlock_without_lock_fails() {
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:42:fn n139_double_lock_differs() {
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:4:fn n139_lock_and_unlock_flow() {
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:68:fn n139_deterministic_bridge_root() {
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:85:fn n139_bridge_root_changes_after_unlock() {
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:108:fn n156_evidence_root_deterministic() {
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:34:fn n156_repay_and_unlock() {
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:60:fn n156_cannot_transfer_locked_nft() {
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:80:fn n156_liquidation_removes_lock() {
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:9:fn n156_lock_nft_and_borrow() {
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:102:fn n131_law4_replay_protection_rejects_old_timestamp() {
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:108:fn n131_evidence_root_matches() {
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:11:fn n131_mint_produces_valid_evidence() {
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:130:fn n131_full_mint_flow_with_evidence() {
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:56:fn n131_law1_prevents_unauthorized_transfer() {
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:77:fn n131_law2_prevents_duplicate_mint() {
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:96:fn n131_law3_rejects_invalid_metadata_hash() {
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:27:fn n135_query_nft_by_id() {
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:45:fn n135_query_owner_nfts() {
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:8:fn n135_query_collections() {
crates/amun-nft-fuzz/src/lib.rs:15:    pub fn new(test_name: &str) -> Self {
crates/amun-nft-fuzz/src/lib.rs:24:    pub fn passed(&self) -> bool {
crates/amun-nft-fuzz/tests/n148_fuzz_tests.rs:15:fn n148_fuzz_marketplace_500_iterations() {
crates/amun-nft-fuzz/tests/n148_fuzz_tests.rs:26:fn n148_fuzz_royalty_10000_iterations() {
crates/amun-nft-fuzz/tests/n148_fuzz_tests.rs:37:fn n148_fuzz_governance_1000_iterations() {
crates/amun-nft-fuzz/tests/n148_fuzz_tests.rs:48:fn n148_fuzz_bridge_1000_iterations() {
crates/amun-nft-fuzz/tests/n148_fuzz_tests.rs:4:fn n148_fuzz_mint_1000_iterations() {
crates/amun-nft-governance-execution/tests/n143_governance_execution_tests.rs:5:fn n143_create_proposal_and_vote() {
crates/amun-nft-governance-execution/tests/n143_governance_execution_tests.rs:65:fn n143_proposal_fails_without_rights() {
crates/amun-nft-governance-execution/tests/n143_governance_execution_tests.rs:75:fn n143_execution_root_deterministic() {
crates/amun-nft-governance/tests/n138_governance_tests.rs:23:fn n138_revoke_rights() {
crates/amun-nft-governance/tests/n138_governance_tests.rs:41:fn n138_multiple_tokens_independent_rights() {
crates/amun-nft-governance/tests/n138_governance_tests.rs:4:fn n138_grant_and_check_rights() {
crates/amun-nft-governance/tests/n138_governance_tests.rs:67:fn n138_deterministic_governance_root() {
crates/amun-nft-governance/tests/n138_governance_tests.rs:83:fn n138_revoked_changes_root() {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:30:fn n144_query_by_owner() {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:52:fn n144_index_events_and_query() {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:75:fn n144_deterministic_index_root() {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:7:fn n144_index_and_query_nft() {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:96:fn n144_index_updates_after_registry_change() {
crates/amun-nft-integration/tests/n132_integration_tests.rs:15:fn n132_different_nft_changes_root() {
crates/amun-nft-integration/tests/n132_integration_tests.rs:26:fn n132_height_and_metadata_preserved() {
crates/amun-nft-integration/tests/n132_integration_tests.rs:5:fn n132_extended_root_deterministic() {
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:117:fn n134_1_prevent_self_purchase() {
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:140:fn n134_1_prevent_bid_below_highest() {
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:164:fn n134_1_marketplace_evidence_root() {
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:37:fn n134_cancel_listing() {
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:61:fn n134_auction_flow() {
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:8:fn n134_list_and_buy_nft() {
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:90:fn n134_1_prevent_double_buy() {
crates/amun-nft-mining/tests/n133_mining_tests.rs:17:fn n133_mining_reward_creates_nft() {
crates/amun-nft-mining/tests/n133_mining_tests.rs:54:fn n133_multiple_contributions_get_different_nfts() {
crates/amun-nft-mining/tests/n133_mining_tests.rs:8:fn n133_validator_contribution_evaluates() {
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:10:fn unique_id(seed: u8) -> [u8; 32] {
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:92:fn n150_state_roots_survive_rebuild() {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:26:fn n141_reject_non_owner_seller() {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:58:fn n141_reject_bridge_locked_transfer() {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:8:fn n141_reject_unregistered_token() {
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:99:fn n141_allow_valid_transfer_with_royalty() {
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:20:fn n137_multiple_sales_accumulation() {
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:37:fn n137_multiple_creators_independent_balances() {
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:5:fn n137_single_creator_accrual() {
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:60:fn n137_deterministic_accounting_root() {
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:80:fn n137_overflow_safety() {
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:33:fn n142_no_settlement_for_zero_balance() {
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:41:fn n142_deterministic_settlement_root() {
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:62:fn n142_multiple_settlements_differ() {
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:6:fn n142_settle_accumulated_royalties() {
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:14:fn n136_auction_royalty_10_percent() {
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:26:fn n136_zero_royalty() {
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:36:fn n136_overflow_safety() {
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:47:fn n136_deterministic_evidence_root() {
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:4:fn n136_direct_sale_royalty_5_percent() {
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:61:fn n136_royalty_record_serialization_roundtrip() {
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:116:fn n145_full_sdk_integration_flow() {
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:44:fn n145_transfer_via_sdk() {
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:67:fn n145_list_and_buy_via_sdk() {
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:6:fn n145_mint_and_query_via_sdk() {
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:91:fn n145_auction_flow_via_sdk() {
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:11:fn unique_id(seed: u8) -> [u8; 32] {
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:130:fn n151_all_roots_deterministic_after_rebuild() {
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:142:fn n151_snapshot_roots_are_nonzero() {
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:153:fn n151_root_changes_after_mutation() {
crates/amun-nft-stress/tests/n146_stress_tests.rs:17:fn n146_stress_mint_1000_nfts() {
crates/amun-nft-stress/tests/n146_stress_tests.rs:36:fn n146_stress_marketplace_rapid_trades() {
crates/amun-nft-stress/tests/n146_stress_tests.rs:76:fn n146_stress_state_root_consistent_under_load() {
crates/amun-nft-stress/tests/n146_stress_tests.rs:9:fn token_id(seed: u8, salt: u64) -> [u8; 32] {
crates/amun-node/src/bin/test_byzantine_fault.rs:1:fn main() {}
crates/amun-node/src/bin/test_checkpoint_rejoin.rs:14:fn main() {
crates/amun-node/src/bin/test_crash_recovery.rs:10:fn main() {
crates/amun-node/src/bin/test_crash_recovery_7.rs:10:fn main() {
crates/amun-node/src/bin/test_live_state.rs:5:fn main() {
crates/amun-node/src/bin/test_multi_byzantine.rs:10:fn main() {
crates/amun-node/src/bin/test_multi_height_determinism.rs:7:fn main() {
crates/amun-node/src/bin/test_network_state_convergence.rs:10:fn main() {
crates/amun-node/src/bin/test_persistence_determinism.rs:7:fn main() {
crates/amun-node/src/bin/test_post_rejoin_convergence.rs:12:fn main() {
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
crates/amun-node/src/bin/test_replay_stress.rs:32:            lineage: ResourceLineage::genesis(resource_id),
crates/amun-node/src/bin/test_replay_stress.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
crates/amun-node/src/bin/test_replay_stress.rs:89:    // Phase 2: Replay on 4 independent stores from scratch
crates/amun-node/src/bin/test_replay_stress.rs:90:    let mut replay_roots = Vec::new();
crates/amun-node/src/bin/test_replay_stress.rs:92:        let replay_dir = format!("{}/replay{}", prefix, v);
crates/amun-node/src/bin/test_replay_stress.rs:93:        std::fs::create_dir_all(&replay_dir).expect("Failed to create dir");
crates/amun-node/src/bin/test_replay_stress.rs:94:        let mut replay_store =
crates/amun-node/src/bin/test_replay_stress.rs:95:            PersistentValidatorStore::open(&replay_dir).expect("Failed to open store");
crates/amun-node/src/bin/test_state_determinism.rs:7:fn main() {
crates/amun-node/src/bin/test_state_evolution.rs:8:fn main() {
crates/amun-node/src/bin/test_state_recovery_rejoin.rs:10:fn main() {
crates/amun-node/src/bin/test_sync_rejoin.rs:11:fn main() {
crates/amun-node/src/bin/test_validator_cluster_determinism.rs:5:fn main() {
crates/amun-node/src/peer_handshake.rs:94:    fn create_test_handshake() -> (HandshakeMessage, [u8; 32]) {
crates/amun-node/src/peer_registry.rs:57:    fn create_test_peer(name: &str, port: u16) -> AuthenticatedPeer {
crates/amun-pacemaker/src/timeout_law.rs:21:    fn test_exponential_backoff() {
crates/amun-pacemaker/src/timeout_law.rs:31:    fn test_max_cap() {
crates/amun-pacemaker/src/timeout_law.rs:37:    fn test_halt_detection() {
crates/amun-pccv/src/lib.rs:214:    fn n49_full_semantic_verification_passes() {
crates/amun-pccv/tests/replay_equivalence.rs:102:    assert!(matches!(result1, PCCVResult::Verified { .. }));
crates/amun-pccv/tests/replay_equivalence.rs:103:    assert!(matches!(result2, PCCVResult::Verified { .. }));
crates/amun-pccv/tests/replay_equivalence.rs:107:fn n49c_replay_consistent_across_iterations() {
crates/amun-pccv/tests/replay_equivalence.rs:108:    let mut reg = ResourceRegistry::new(1000);
crates/amun-pccv/tests/replay_equivalence.rs:113:        lineage: ResourceLineage::genesis(make_id(1)),
crates/amun-pccv/tests/replay_equivalence.rs:119:    let parent_hash = ResourceRegistry::hash_resource(reg.get(&make_id(1)).unwrap());
crates/amun-pccv/tests/replay_equivalence.rs:128:        lineage: ResourceLineage::transformation(child_id, make_id(1), parent_hash, 2),
crates/amun-pccv/tests/replay_equivalence.rs:17:    let mut reg1 = ResourceRegistry::new(1000);
crates/amun-pccv/tests/replay_equivalence.rs:18:    let mut reg2 = ResourceRegistry::new(1000);
crates/amun-pccv/tests/replay_equivalence.rs:1:use amun_pccv::pccv_verifier::PCCVResult;
crates/amun-pccv/tests/replay_equivalence.rs:24:        lineage: ResourceLineage::genesis(make_id(1)),
crates/amun-pccv/tests/replay_equivalence.rs:37:    let parent_hash = ResourceRegistry::hash_resource(reg1.get(&make_id(1)).unwrap());
crates/amun-pccv/tests/replay_equivalence.rs:42:        lineage: ResourceLineage::transformation(child_id, make_id(1), parent_hash, 2),
crates/amun-pccv/tests/replay_equivalence.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-pccv/tests/replay_equivalence.rs:9:fn make_id(seed: u8) -> ResourceId {
crates/amun-peer-identity/tests/peer_identity_tests.rs:14:fn test_different_genesis_produces_different_id() {
crates/amun-peer-identity/tests/peer_identity_tests.rs:37:fn test_registry_determinism() {
crates/amun-peer-identity/tests/peer_identity_tests.rs:7:fn test_peer_id_determinism() {
crates/amun-qc-canonical/src/canonicalize.rs:83:    fn test_canonicalize_removes_duplicates() {
crates/amun-recovery/src/lib.rs:131:        assert_eq!(recovered.replay_count, 1);
crates/amun-recovery/src/lib.rs:145:        assert_eq!(recovered.replay_count, 0);
crates/amun-recovery/src/lib.rs:186:        assert_eq!(recovered.replay_count, 3);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:187:        assert_eq!(block.replay_verifications.len(), 1);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:188:        assert!(block.replay_verifications[0].is_verified());
crates/amun-replay-consensus/src/replay_backed_consensus.rs:214:        assert!(cert.verify());
crates/amun-replay-consensus/src/replay_backed_consensus.rs:242:        assert!(ReplayBackedConsensus::form_consensus(&block, 5, sigs).is_err());
crates/amun-replay-consensus/src/replay_backed_consensus.rs:294:        assert_ne!(block.replay_root, [0u8; 32]);
crates/amun-replay-engine/src/adaptive_relay.rs:225:    fn test_relay_selection_avoids_congested() {
crates/amun-replay-engine/src/adaptive_relay.rs:234:    fn test_relay_avoids_avoid_list() {
crates/amun-replay-engine/src/adaptive_relay.rs:243:    fn test_frontier_pressure() {
crates/amun-replay-engine/src/adaptive_relay.rs:253:    fn test_adaptive_compression_under_pressure() {
crates/amun-replay-engine/src/adaptive_relay.rs:260:    fn test_backpressure() {
crates/amun-replay-engine/src/adaptive_relay.rs:269:    fn test_equivalence_cache() {
crates/amun-replay-engine/src/adaptive_relay.rs:278:    fn test_anti_centrality_guard() {
crates/amun-replay-engine/src/adaptive_relay.rs:290:    fn test_no_centrality_when_balanced() {
crates/amun-replay-engine/src/containment_boundary.rs:105:    fn test_clean_worker_is_operational() {
crates/amun-replay-engine/src/containment_boundary.rs:113:    fn test_hostile_worker_is_quarantined() {
crates/amun-replay-engine/src/containment_boundary.rs:124:    fn test_containment_is_not_invalidation() {
crates/amun-replay-engine/src/containment_boundary.rs:136:    fn test_containment_zone() {
crates/amun-replay-engine/src/derivational_equivalence.rs:141:    fn test_fingerprint_deterministic() {
crates/amun-replay-engine/src/derivational_equivalence.rs:148:    fn test_different_outcome_different_fingerprint() {
crates/amun-replay-engine/src/derivational_equivalence.rs:155:    fn test_equivalence_class_creation() {
crates/amun-replay-engine/src/derivational_equivalence.rs:166:    fn test_add_surface_keeps_smaller_core() {
crates/amun-replay-engine/src/derivational_equivalence.rs:177:    fn test_reduce_to_minimal_core() {
crates/amun-replay-engine/src/derivational_equivalence.rs:189:    fn test_equivalence_detection() {
crates/amun-replay-engine/src/derivational_equivalence.rs:196:    fn test_non_equivalence() {
crates/amun-replay-engine/src/execution_dag.rs:118:    fn test_empty_dag() {
crates/amun-replay-engine/src/execution_dag.rs:124:    fn test_add_vertex() {
crates/amun-replay-engine/src/execution_dag.rs:131:    fn test_topological_order() {
crates/amun-replay-engine/src/execution_dag.rs:157:    fn test_dag_is_scheduler_oblivious() {
crates/amun-replay-engine/src/execution_dependency.rs:62:    fn test_mandatory_dependencies() {
crates/amun-replay-engine/src/execution_dependency.rs:69:    fn test_optional_dependencies() {
crates/amun-replay-engine/src/execution_scheduler.rs:107:    fn test_block_and_unblock() {
crates/amun-replay-engine/src/execution_scheduler.rs:118:    fn test_pending_count() {
crates/amun-replay-engine/src/execution_scheduler.rs:94:    fn test_scheduler_fifo_order() {
crates/amun-replay-engine/src/execution_task.rs:76:    fn test_task_creation() {
crates/amun-replay-engine/src/execution_vertex.rs:102:    fn test_vertex_creation() {
crates/amun-replay-engine/src/execution_vertex.rs:113:    fn test_vertex_is_not_truth() {
crates/amun-replay-engine/src/isolation_boundary.rs:147:    fn test_capability_check_allowed() {
crates/amun-replay-engine/src/isolation_boundary.rs:154:    fn test_capability_check_denied() {
crates/amun-replay-engine/src/isolation_boundary.rs:161:    fn test_context_check_allowed() {
crates/amun-replay-engine/src/isolation_boundary.rs:168:    fn test_context_check_denied() {
crates/amun-replay-engine/src/missing_closure.rs:111:    fn test_closure_request() {
crates/amun-replay-engine/src/missing_closure.rs:125:    fn test_hop_exhaustion() {
crates/amun-replay-engine/src/operational_hasher.rs:49:    fn test_operational_hash_deterministic() {
crates/amun-replay-engine/src/temporal_drift.rs:189:    fn test_derivable_age_boundary() {
crates/amun-replay-engine/src/temporal_drift.rs:197:    fn test_historical_weight_neutrality() {
crates/amun-replay-engine/src/temporal_drift.rs:205:    fn test_compatibility_decay() {
crates/amun-replay-engine/src/temporal_drift.rs:214:    fn test_lineage_never_grants_age_authority() {
crates/amun-replay-engine/src/temporal_drift.rs:220:    fn test_epoch_containment() {
crates/amun-replay-engine/src/zk_adapters.rs:234:    fn test_zk_envelope_creation() {
crates/amun-replay-engine/src/zk_adapters.rs:243:    fn test_derivability_commitment() {
crates/amun-replay-engine/src/zk_adapters.rs:249:    fn test_recursive_boundary() {
crates/amun-replay-engine/src/zk_adapters.rs:260:    fn test_selective_reveal() {
crates/amun-replay-engine/src/zk_adapters.rs:269:    fn test_external_verifier_adapter() {
crates/amun-replay-engine/src/zk_adapters.rs:276:    fn test_inspectability_guard() {
crates/amun-replay-optimization/tests/n163_replay_tests.rs:1:use amun_replay_optimization::*;
crates/amun-replay-optimization/tests/n163_replay_tests.rs:21:fn n163_batch_verification_faster_than_individual() {
crates/amun-replay-optimization/tests/n163_replay_tests.rs:22:    let mut cache = ReplayCache::new();
crates/amun-replay-optimization/tests/n163_replay_tests.rs:46:fn n163_header_cache_speeds_sync() {
crates/amun-replay-optimization/tests/n163_replay_tests.rs:47:    let mut cache = ReplayCache::new();
crates/amun-replay-optimization/tests/n163_replay_tests.rs:4:fn n163_cache_hit_improves_replay() {
crates/amun-replay-optimization/tests/n163_replay_tests.rs:5:    let mut cache = ReplayCache::new();
crates/amun-replay-optimization/tests/n163_replay_tests.rs:63:fn n163_cache_root_deterministic() {
crates/amun-replay-optimization/tests/n163_replay_tests.rs:64:    let mut cache1 = ReplayCache::new();
crates/amun-replay-optimization/tests/n163_replay_tests.rs:65:    let mut cache2 = ReplayCache::new();
crates/amun-replay/src/commit_log.rs:63:    pub fn latest_root(&self) -> Option<[u8; 32]> {
crates/amun-replay/src/store.rs:67:    fn test_store_and_retrieve() {
crates/amun-replay/src/store.rs:89:    fn test_unique_hashes() {
crates/amun-resource-core/tests/stress_tests.rs:132:fn stress_004_state_root_10k() {
crates/amun-resource-core/tests/stress_tests.rs:13:fn make_genesis(id: ResourceId, archetype: ResourceArchetype) -> ResourceMetadata {
crates/amun-resource-core/tests/stress_tests.rs:148:fn stress_005_lookup_under_load() {
crates/amun-resource-core/tests/stress_tests.rs:173:fn stress_006_parent_verification_under_load() {
crates/amun-resource-core/tests/stress_tests.rs:221:fn stress_007_cycle_detection_at_depth() {
crates/amun-resource-core/tests/stress_tests.rs:25:fn stress_001_10k_genesis_resources() {
crates/amun-resource-core/tests/stress_tests.rs:44:fn stress_002_deep_lineage_chain() {
crates/amun-resource-core/tests/stress_tests.rs:7:fn make_id(seed: u64) -> ResourceId {
crates/amun-resource-core/tests/stress_tests.rs:84:fn stress_003_wide_fanout() {
crates/amun-sdk-layer/src/tests.rs:11:    fn test_governance_api_create_proposal() { let mut api = GovernanceApi::new(); let proposer = amun_kernel_types::PublicHash32::new([1u8; 32]); let result = api.create_proposal(proposer, amun_governance::proposal::ProposalType::Text, 1000); assert!(result.success); }
crates/amun-sdk-layer/src/tests.rs:13:    fn test_charity_api_donate() { let mut api = CharityApi::new(); let recipient = amun_kernel_types::PublicHash32::new([1u8; 32]); let result = api.donate(recipient, 100); assert!(result.success); }
crates/amun-sdk-layer/src/tests.rs:5:    fn test_token_api_create() { let result = TokenApi::create_account(1_000_000); assert!(result.success); }
crates/amun-sdk-layer/src/tests.rs:9:    fn test_staking_api_register() { let mut api = StakingApi::new(); let pk = amun_kernel_types::PublicKey::new([1u8; 48]); let result = api.register_validator(pk, 1_000_000); assert!(result.success); }
crates/amun-smt/src/canonical_model.rs:45:pub fn assert_equivalent(
crates/amun-smt/tests/determinism.rs:12:fn insertion_order_independence() {
crates/amun-smt/tests/determinism.rs:42:fn delete_stability() {
crates/amun-smt/tests/determinism.rs:5:fn random_key(rng: &mut impl Rng) -> Key256 {
crates/amun-smt/tests/differential.rs:50:fn differential_delete_all_restores_empty() {
crates/amun-smt/tests/differential.rs:5:fn differential_insert_delete_chain() {
crates/amun-smt/tests/exhaustive.rs:13:fn exhaustive_2_keys() {
crates/amun-smt/tests/exhaustive.rs:20:fn exhaustive_3_keys() {
crates/amun-smt/tests/exhaustive.rs:27:fn exhaustive_4_keys() {
crates/amun-smt/tests/exhaustive.rs:38:fn test_all_permutations(keys: &[Key256], n: usize) {
crates/amun-smt/tests/exhaustive.rs:6:fn exhaustive_1_key() {
crates/amun-smt/tests/fuzz.rs:11:    fn determinism_under_random_inserts(
crates/amun-smt/tests/fuzz.rs:31:    fn delete_stability(
crates/amun-smt/tests/fuzz.rs:5:fn arb_key() -> impl Strategy<Value = Key256> {
crates/amun-smt/tests/identity_laws.rs:24:fn law_delete_insert_canonicality() {
crates/amun-smt/tests/identity_laws.rs:50:fn law_structural_determinism() {
crates/amun-smt/tests/identity_laws.rs:7:fn law_insert_delete_identity() {
crates/amun-snapshot-engine-unified/src/lib.rs:74:    fn create_test_state() -> PersistentState {
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:10:fn n162_incremental_snapshot_faster_than_full() {
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:68:fn n162_restore_from_compressed_is_fast() {
crates/amun-soak-full/tests/n165_full_soak_tests.rs:23:fn n165_full_soak_60s_with_adversarial() {
crates/amun-soak-full/tests/n165_full_soak_tests.rs:43:fn n165_state_consistency_under_full_load() {
crates/amun-soak-full/tests/n165_full_soak_tests.rs:4:fn n165_full_soak_30s() {
crates/amun-soak-test/src/lib.rs:174:    pub fn passed(&self) -> bool {
crates/amun-soak-test/src/lib.rs:180:    fn default() -> Self {
crates/amun-soak-test/src/lib.rs:21:    pub fn new() -> Self {
crates/amun-soak-test/src/lib.rs:31:    pub fn run(&self, duration_secs: u64, events_enabled: bool) -> SoakResult {
crates/amun-soak-test/tests/n165_soak_tests.rs:20:fn n165_soak_60_seconds_with_events() {
crates/amun-soak-test/tests/n165_soak_tests.rs:36:fn n165_state_consistency_under_load() {
crates/amun-soak-test/tests/n165_soak_tests.rs:4:fn n165_soak_30_seconds_no_events() {
crates/amun-staking/src/tests.rs:12:fn test_slash() {
crates/amun-staking/src/tests.rs:24:fn test_delegate() {
crates/amun-staking/src/tests.rs:5:fn test_register() {
crates/amun-state-pruning/tests/n166_pruning_tests.rs:109:fn n166_pruned_root_deterministic() {
crates/amun-state-pruning/tests/n166_pruning_tests.rs:58:fn n166_restore_archived_brings_back_resources() {
crates/amun-state-pruning/tests/n166_pruning_tests.rs:8:fn n166_prune_by_height_reduces_active_set() {
crates/amun-state-root/src/laws.rs:2:pub const REPLAY_EQUIVALENCE_LAW: &str = "assert_eq!(live_execution_hash, replay_hash)";
crates/amun-state-sync/src/sync_protocol.rs:245:    fn create_test_resource(i: u64) -> ResourceMetadata {
crates/amun-state-sync/src/sync_protocol.rs:257:    fn create_test_registry(n: u64) -> ResourceRegistry {
crates/amun-state-types/src/tests.rs:38:fn test_state_commit() {
crates/amun-state-types/src/tests.rs:52:fn test_state_finalize() {
crates/amun-state-types/src/tests.rs:5:fn test_state_new() {
crates/amun-state-types/src/tests.rs:66:fn test_state_into_inner() {
crates/amun-state-types/src/tests.rs:80:fn test_state_make_durable() {
crates/amun-state-types/src/tests.rs:87:fn test_state_mark_voted() {
crates/amun-stf/src/nonce.rs:18:    pub fn increment_nonce<S: StateStore>(store: &mut S, account: &[u8]) -> AmunResult<u64> {
crates/amun-stf/src/nonce.rs:7:    pub fn get_nonce<S: StateStore>(store: &S, account: &[u8]) -> AmunResult<u64> {
crates/amun-stf/src/state.rs:75:    fn assert_canonical_order(&self) {
crates/amun-stf/src/tests.rs:20:fn test_stf_rollback_preserves_root() {
crates/amun-stf/src/tests.rs:33:fn test_stf_deterministic_root() {
crates/amun-stf/src/tests.rs:51:fn test_nonce_basics() {}
crates/amun-stf/src/tests.rs:53:fn test_apply_block_deterministic() {}
crates/amun-stf/src/tests.rs:55:fn test_nonce_rejects_replay() {}
crates/amun-stf/src/tests.rs:57:fn test_root_deterministic() {}
crates/amun-stf/src/tests.rs:59:fn test_state_set_get() {}
crates/amun-stf/src/tests.rs:61:fn test_state_delete() {}
crates/amun-stf/src/tests.rs:7:fn test_stf_commit_changes_root() {
crates/amun-stf/src/transition.rs:11:    pub fn apply_block<S: StateStore>(
crates/amun-stf/tests/integration_test.rs:35:fn test_truth_bound_execution_detects_anomaly() {
crates/amun-stf/tests/integration_test.rs:5:fn test_truth_bound_execution_creates_valid_receipt() {
crates/amun-stf/tests/integration_test.rs:61:fn test_transcript_continuity_with_evolving_state() {
crates/amun-stf/tests/replay_equivalence.rs:112:        .expect("corrupted replay must still execute");
crates/amun-stf/tests/replay_equivalence.rs:116:        replay_transcript.add_receipt(receipt);
crates/amun-stf/tests/replay_equivalence.rs:119:    let replay_root = replay_transcript
crates/amun-stf/tests/replay_equivalence.rs:122:        .expect("replay transcript must not be empty")
crates/amun-stf/tests/replay_equivalence.rs:126:        live_root, replay_root,
crates/amun-stf/tests/replay_equivalence.rs:127:        "divergent replay unexpectedly matched"
crates/amun-stf/tests/replay_equivalence.rs:135:    let replay_hashes: Vec<[u8; 32]> = replay_transcript
crates/amun-stf/tests/replay_equivalence.rs:141:        live_hashes, replay_hashes,
crates/amun-stf/tests/replay_equivalence.rs:142:        "divergent replay produced identical receipts"
crates/amun-stf/tests/replay_equivalence.rs:48:fn test_single_block_replay_equivalence() {
crates/amun-stf/tests/replay_equivalence.rs:54:    assert_eq!(live_root, replay_root, "live and replay roots diverged");
crates/amun-stf/tests/replay_equivalence.rs:55:    assert_eq!(live_state, replay_state, "live and replay state diverged");
crates/amun-stf/tests/replay_equivalence.rs:58:        replay_transcript.receipts.len(),
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
crates/amun-stf/tests/replay_equivalence.rs:88:fn test_replay_detects_divergent_execution() {
crates/amun-stf/tests/replay_equivalence.rs:93:    let mut replay_transcript = ExecutionTranscript::new();
crates/amun-storage-kernel/tests/compute_real_empty_root.rs:6:    fn print_real_empty_root() {
crates/amun-storage-kernel/tests/delete_equivalence.rs:21:    fn delete_nonexistent_does_not_change_root() {
crates/amun-storage-kernel/tests/delete_equivalence.rs:36:    fn reinsert_after_delete_produces_same_root() {
crates/amun-storage-kernel/tests/delete_equivalence.rs:6:    fn insert_delete_returns_to_canonical_empty() {
crates/amun-storage-kernel/tests/ladder_consistency.rs:7:    fn empty_ladder_matches_node_hash() {
crates/amun-storage-kernel/tests/proptest_smt.rs:62:    fn insertion_order_independence() {
crates/amun-storage-kernel/tests/proptest_smt.rs:7:    fn proptest_config() -> Config {
crates/amun-storage-kernel/tests/replay_equivalence.rs:100:    fn replay_detects_epoch_regression() {
crates/amun-storage-kernel/tests/replay_equivalence.rs:11:    fn create_test_wal(path: &str, entries: &[WalEntry]) -> std::io::Result<()> {
crates/amun-storage-kernel/tests/replay_equivalence.rs:28:    fn replay_equivalence_passes() {
crates/amun-storage-kernel/tests/replay_equivalence.rs:4:        persistence::wal::{ReplayVerifier, WalEntry},
crates/amun-storage-kernel/tests/replay_equivalence.rs:53:        let (replayed_root, count) =
crates/amun-storage-kernel/tests/replay_equivalence.rs:57:        assert_eq!(replayed_root, root.0);
crates/amun-storage-kernel/tests/replay_equivalence.rs:61:    fn replay_detects_divergence() {
crates/amun-storage-kernel/tests/replay_equivalence.rs:96:        assert!(result.unwrap_err().contains("replay divergence"));
crates/amun-storage-kernel/tests/specification_compliance.rs:130:    fn theorem_terminal_empty_is_zero() {
crates/amun-storage-kernel/tests/specification_compliance.rs:32:    fn theorem_delete_reinsert_identity() {
crates/amun-storage-kernel/tests/specification_compliance.rs:53:    fn theorem_empty_identity() {
crates/amun-storage-kernel/tests/specification_compliance.rs:94:    fn theorem_delete_nonexistent_noop() {
crates/amun-storage-kernel/tests/specification_compliance.rs:9:    fn theorem_order_independence_two_keys() {
crates/amun-storage-kernel/tests/terminal_empty.rs:6:    fn terminal_empty_is_zero() {
crates/amun-sync/examples/sync_protocol_test.rs:6:fn main() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:136:    block.replay_verifications[0].replay_success = false;
crates/amun-testnet-sim/tests/adversarial_tests.rs:147:fn n60_crash_recovery_rejoin() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:162:fn n60_long_run_blocks_consistent_replay() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:16:fn make_id(seed: u64) -> ResourceId {
crates/amun-testnet-sim/tests/adversarial_tests.rs:231:fn n60_byzantine_conflicting_blocks_detected() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:251:    block.replay_verifications[0].state_root_match = false;
crates/amun-testnet-sim/tests/adversarial_tests.rs:262:fn n60_large_state_sync() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:44:fn n60_network_partition_no_double_finality() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:6:use amun_replay_consensus::replay_backed_consensus::ReplayBackedConsensus;
crates/amun-testnet-sim/tests/adversarial_tests.rs:7:use amun_replay_verifier::replay_verifier::{ReplayResult, ReplayVerifier};
crates/amun-testnet-sim/tests/adversarial_tests.rs:87:fn n60_malicious_validator_invalid_qc_rejected() {
crates/amun-tokenomics-ledger/tests/test_ledger.rs:11:fn test_ledger_epoch_advances() {
crates/amun-tokenomics-ledger/tests/test_ledger.rs:46:fn test_ledger_root_deterministic() {
crates/amun-tokenomics-ledger/tests/test_ledger.rs:4:fn sample_economics() -> EpochEconomics {
crates/amun-tokenomics-ledger/tests/test_ledger.rs:61:fn test_multiple_epochs() {
crates/amun-tokenomics/tests/test_tokenomics.rs:16:fn test_distribution_splits() {
crates/amun-tokenomics/tests/test_tokenomics.rs:38:fn test_distribution_matches_configured_bps() {
crates/amun-tokenomics/tests/test_tokenomics.rs:5:fn test_epoch_reward_calculation() {
crates/amun-transaction/src/tests.rs:21:fn test_transfer_rejects_zero_gas() {
crates/amun-transaction/src/tests.rs:36:fn test_transfer_roundtrip_ok() {
crates/amun-transaction/src/tests.rs:53:fn test_stake_ok() {
crates/amun-transaction/src/tests.rs:68:fn test_unstake_ok() {
crates/amun-transaction/src/tests.rs:6:fn test_transfer_rejects_zero_chain() {
crates/amun-transaction/src/tests.rs:83:fn test_contract_call_ok() {
crates/amun-transaction/src/tests.rs:98:fn test_rejects_zero_pubkey() {
crates/amun-transcript-semantics/src/lib.rs:103:    #[test] fn test_authority_replay_required() { assert!(EventAuthority::Authoritative.is_replay_required()); assert!(!EventAuthority::Derived.is_replay_required()); }
crates/amun-unlock-law/src/lib.rs:33:    fn test_unlock() {
crates/amun-unsafe/src/tests.rs:100:fn guard_as_ref() {
crates/amun-unsafe/src/tests.rs:107:fn guard_as_mut() {
crates/amun-unsafe/src/tests.rs:114:fn guard_drops_if_not_taken() {
crates/amun-unsafe/src/tests.rs:120:        fn drop(&mut self) {
crates/amun-unsafe/src/tests.rs:131:fn guard_does_not_drop_if_taken() {
crates/amun-unsafe/src/tests.rs:137:        fn drop(&mut self) {
crates/amun-unsafe/src/tests.rs:14:fn slot_write_take() {
crates/amun-unsafe/src/tests.rs:23:fn slot_write_get() {
crates/amun-unsafe/src/tests.rs:30:fn slot_write_get_mut() {
crates/amun-unsafe/src/tests.rs:38:fn slot_replace() {
crates/amun-unsafe/src/tests.rs:47:fn slot_double_write_rejected() {
crates/amun-unsafe/src/tests.rs:54:fn slot_take_empty_rejected() {
crates/amun-unsafe/src/tests.rs:60:fn slot_get_empty_rejected() {
crates/amun-unsafe/src/tests.rs:66:fn slot_reuse_after_take() {
crates/amun-unsafe/src/tests.rs:75:fn slot_drop_releases_value() {
crates/amun-unsafe/src/tests.rs:81:        fn drop(&mut self) {
crates/amun-unsafe/src/tests.rs:8:fn slot_new_is_empty() {
crates/amun-unsafe/src/tests.rs:94:fn guard_take_returns_value() {
crates/amun-validator-attestation/src/attestation.rs:15:    pub fn new(
crates/amun-validator-attestation/src/attestation.rs:35:    pub fn verify(&self) -> bool {
crates/amun-validator-attestation/src/validator_set.rs:20:    pub fn new(epoch: Epoch, validators: Vec<ValidatorInfo>) -> Result<Self, ValidatorSetError> {
crates/amun-validator-attestation/src/validator_set.rs:39:    pub fn get_validator(&self, id: u64) -> Option<&ValidatorInfo> {
crates/amun-validator-attestation/src/validator_set.rs:43:    pub fn has_quorum(&self, weight: u64) -> bool {
crates/amun-validator-attestation/src/validator_set.rs:47:    pub fn total_stake(&self) -> u64 {
crates/amun-validator-attestation/src/validator_set.rs:51:    pub fn quorum_threshold(&self) -> u64 {
crates/amun-validator-attestation/src/validator_set.rs:55:    pub fn validator_ids(&self) -> Vec<u64> {
crates/amun-validator-attestation/src/validator_set.rs:59:    pub fn len(&self) -> usize {
crates/amun-validator-attestation/src/validator_set.rs:63:    pub fn is_empty(&self) -> bool {
crates/amun-wal/src/lib.rs:914:    pub fn reset_for_testing(base_path: &str) -> Result<(), String> {
crates/amun-wal/src/lib.rs:934:    fn test_footer_roundtrip() {
crates/amun-wal/src/lib.rs:955:    fn test_clean() {
crates/amun-wal/src/lib.rs:966:    fn test_entry() {
crates/amun-wal/src/lib.rs:977:    fn test_strict_mode_rejects_corrupt_segment() {
crates/amun-wal/src/lib.rs:986:    fn test_policy_clean() {
crates/amun-wal/src/lib.rs:999:    fn test_magic_error_detected() {
crates/amun-wallet-management/src/keystore.rs:10:pub fn save_keystore(keypair: &WalletKeypair, password: &str, path: &str) -> Result<(), String> {
crates/amun-wallet-management/src/keystore.rs:42:pub fn load_keystore(path: &str, password: &str) -> Result<WalletKeypair, String> {
crates/amun-wallet-management/src/lib.rs:47:    fn key05_wrong_password_rejected() {
crates/amun_consensus_math/src/constants.rs:55:    fn test_div_floor() {
crates/amun_consensus_math/src/constants.rs:63:    fn test_mod_floor() {
crates/amun_consensus_math/src/exp.rs:109:    fn test_exp_two() {
crates/amun_consensus_math/src/exp.rs:38:    fn test_exp_zero() {
crates/amun_consensus_math/src/exp.rs:45:    fn test_exp_half() {
crates/amun_consensus_math/src/exp.rs:62:    fn test_exp_one() {
crates/amun_consensus_math/src/exp.rs:79:    fn test_exp_negative_one() {
crates/amun_consensus_math/src/exp.rs:92:    fn test_exp_negative_half() {
crates/amun_consensus_math/src/fixed.rs:204:    fn test_bounds() {
crates/amun_consensus_math/src/rounding.rs:33:    fn test_rounding() {
crates/amun_consensus_math/src/sqrt.rs:50:    fn test_sqrt_basic() {
crates/amun_consensus_math/tests/bounds.rs:16:fn test_fixed_bounds() {
crates/amun_consensus_math/tests/bounds.rs:4:fn test_legitimacy_bounds() {
crates/amun_consensus_math/tests/monotonicity.rs:28:fn test_sqrt_identity() {
crates/amun_consensus_math/tests/monotonicity.rs:3:fn fixed_from_f64(v: f64) -> Fixed {
crates/amun_consensus_math/tests/monotonicity.rs:8:fn test_sqrt_monotonicity() {
crates/amun_consensus_math/tests/replay_binary.rs:11:fn read_i64(data: &[u8], pos: &mut usize) -> i64 {
crates/amun_consensus_math/tests/replay_binary.rs:18:fn test_replay_binary_transcript() {
crates/amun_consensus_math/tests/replay_binary.rs:1://! Binary transcript replay test with tolerance for rounding differences
crates/amun_consensus_math/tests/replay_binary.rs:72:    let computed_hash = format!("{:x}", hasher.finalize());
crates/amun_consensus_math/tests/replay_binary.rs:77:            println!("Hash file not found. Computed hash: {}", computed_hash);
crates/amun_consensus_math/tests/replay_binary.rs:84:    println!("Tests executed: {}", test_count);
crates/amun_consensus_math/tests/replay_binary.rs:86:    println!("Rust hash: {}", computed_hash);
crates/amun_consensus_math/tests/replay_binary.rs:91:fn test_consistency_across_calls() {
crates/amun_consensus_math/tests/saturation.rs:33:fn test_floor_division_consistency() {
crates/amun_consensus_math/tests/saturation.rs:4:fn test_saturation_edges() {
crates/amun_state_machine/src/event.rs:82:    fn test_event_serialization() {
crates/amun_state_machine/src/log.rs:97:    fn test_log_root_hash() {
crates/amun_state_machine/src/ordering.rs:86:    fn test_canonical_ordering() {
crates/amun_state_machine/src/scheduler.rs:97:    fn test_scheduler_deterministic() {
crates/amun_state_machine/src/snapshot.rs:63:    pub fn latest_snapshot(&self) -> Option<&StateSnapshot> {
crates/amun_state_machine/src/snapshot.rs:97:    fn test_snapshot_chain() {
crates/amun_state_machine/src/state.rs:109:    fn test_state_hash_deterministic() {
crates/amun_state_machine/src/transition.rs:261:    fn test_failed_transition_state_unchanged() {
crates/amun_state_machine/src/transition.rs:275:    fn test_invalid_amount_rejected() {
crates/amun-audit/tests/audit_layer06_replay.rs:3:    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry};
crates/amun-audit/tests/audit_layer11_crash.rs:3:    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry, WalIterator};
crates/amun-audit/tests/audit_layer15_temporal.rs:3:    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry};
crates/amun-testnet-sim/tests/adversarial_tests.rs:209:                                                    // Actually, ReplayVerifier::replay calls execute() which starts
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:103:    let mut b1 = BlockBuilder::new();
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:104:    let mut b2 = BlockBuilder::new();
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:108:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:10:    let sender = sk.verifying_key().to_bytes();
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:1:use amun_block_builder::BlockBuilder;
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:26:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:31:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:36:    builder.engine.state.create_account(a1, 10000);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:44:    let recomputed_economic =
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:48:        block.economic_root, recomputed_economic,
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:49:        "economic_root must match recomputed root from snapshot"
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:55:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:60:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:65:    builder.engine.state.create_account(a1, 10000);
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:74:    // pipeline as BlockBuilder.
crates/amun-block-builder/tests/n132_3_6_economic_consistency.rs:75:    let expected = builder
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:103:        state_root: honest_root, // Proposer computed correctly
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:119:// TEST: Validator C re-executes and MISMATCHES → vote rejected
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:294:// TEST: Cache retention — proposal stays until finalized
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:313:    // N109.7: Re-execute using cached proposal
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:58:    let computed_root =
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:61:    if computed_root != proposal.state_root {
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:63:            "N109.7 STATE_ROOT_MISMATCH: height={} proposed={} computed={}",
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:66:            hex::encode(computed_root),
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:71:        state_root: computed_root,
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:86:    Ok(hasher.finalize().into())
crates/amun-consensus-network/tests/n109_deterministic_execution.rs:90:// TEST: Validator B re-executes and matches → vote accepted
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:103:/// N121.2: Single event replay is deterministic
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:109:    state1.execute(&cert, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:112:    state2.execute(&cert, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:116:        "N121.2 FAIL: single event replay must be deterministic"
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:125:    state.execute(&cert, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:15:        vec![EvidenceCount {
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:16:            evidence_type: EvidenceType::DoubleVote,
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:1:// N121.2 — Deterministic Replay of Slashing State
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:29:/// N121.2 Gatekeeper: Three nodes replay the same events → same root
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:41:        state_a.execute(cert, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:47:        state_b.execute(cert, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:53:        state_c.execute(cert, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:64:    assert_eq!(state_a.executed_count(), 3);
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:65:    assert_eq!(state_b.executed_count(), 3);
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:66:    assert_eq!(state_c.executed_count(), 3);
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:77:    state_a.execute(&cert1, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:78:    state_a.execute(&cert2, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:82:    state_b.execute(&cert2, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:83:    state_b.execute(&cert1, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:92:/// N121.2: Replay from empty always produces zero root
crates/amun-kernel/src/governance.rs:232:impl CanonicalEncode for Attestation {
crates/amun-smt/tests/identity_laws.rs:22:/// delete(k) + insert(k,v) == canonical rebuild with {k,v}
crates/amun-smt/tests/identity_laws.rs:41:    // Should match canonical rebuild
crates/amun-storage-kernel/tests/canonical_constants.rs:10:            CANONICAL_EMPTY_ROOT_V1, computed,
crates/amun-storage-kernel/tests/canonical_constants.rs:8:        let computed = SparseMerkleTree::canonical_empty_root();
crates/amun-storage-kernel/tests/specification_compliance.rs:105:            "Theorem 5 violated: delete nonexistent changed root"
crates/amun-storage-kernel/tests/specification_compliance.rs:134:            "Theorem 7 violated: terminal empty != ZERO"
crates/amun-storage-kernel/tests/specification_compliance.rs:25:        assert_eq!(root_a.0, root_b.0, "Theorem 1 violated: order independence");
crates/amun-storage-kernel/tests/specification_compliance.rs:45:            "Theorem 2 violated: delete-reinsert identity"
crates/amun-storage-kernel/tests/specification_compliance.rs:62:            "Theorem 3 violated: empty identity"
crates/amun-attestation/src/validator.rs:10:impl ValidatorAttestation {
crates/amun-attestation/src/validator.rs:25:        Ed25519Signer::verify(
crates/amun-audit/tests/audit_layer03_snapshot.rs:73:        let matrix = CompatibilityEngine::compute(&id, &id);
crates/amun-audit/tests/audit_layer03_snapshot.rs:80:            CompatibilityEngine::can_sync(&id, &id),
crates/amun-audit/tests/audit_layer04_byzantine.rs:26:        let mut engine = ByzantineSyncEngine::new(identity.clone(), 2);
crates/amun-audit/tests/audit_layer04_byzantine.rs:57:        let mut engine = ByzantineSyncEngine::new(local.clone(), 1);
crates/amun-audit/tests/audit_layer06_replay.rs:102:        let path = dir.path().join("replay003.wal");
crates/amun-audit/tests/audit_layer06_replay.rs:25:    // CONST-REPLAY-001: Replay produces identical state
crates/amun-audit/tests/audit_layer06_replay.rs:29:        let path = dir.path().join("replay001.wal");
crates/amun-audit/tests/audit_layer06_replay.rs:2:mod audit_replay {
crates/amun-audit/tests/audit_layer06_replay.rs:52:            Ok((replayed_root, count)) => {
crates/amun-audit/tests/audit_layer06_replay.rs:59:                    replayed_root, root.0,
crates/amun-audit/tests/audit_layer06_replay.rs:60:                    "CONST-REPLAY-001 VIOLATION: Replayed root diverges"
crates/amun-audit/tests/audit_layer06_replay.rs:63:            Err(e) => panic!("CONST-REPLAY-001: Replay failed: {}", e),
crates/amun-audit/tests/audit_layer06_replay.rs:67:    // CONST-REPLAY-002: Replay detects state root divergence
crates/amun-audit/tests/audit_layer06_replay.rs:71:        let path = dir.path().join("replay002.wal");
crates/amun-audit/tests/audit_layer06_replay.rs:98:    // CONST-REPLAY-003: Replay detects epoch regression
crates/amun-audit/tests/audit_layer11_crash.rs:127:            "CONST-CRASH-003 VIOLATION: Corrupted frame must fail replay"
crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:38:        let mut engine = ByzantineSyncEngine::new(identity.clone(), 2);
crates/amun-audit/tests/audit_layer15_temporal.rs:25:    // CONST-TEMP-001: Replaying same WAL twice produces identical root
crates/amun-audit/tests/audit_layer15_temporal.rs:58:                    "CONST-TEMP-001 VIOLATION: Replay twice produces different roots"
crates/amun-audit/tests/audit_layer15_temporal.rs:62:            (Err(e), _) | (_, Err(e)) => panic!("CONST-TEMP-001: Replay failed: {}", e),
crates/amun-authority-registry/src/executor.rs:145:        assert!(journal.is_executed(&proposal.proposal_id));
crates/amun-authority-registry/src/executor.rs:165:        assert!(reg.transition.is_some());
crates/amun-authority-registry/src/recovery.rs:4:/// Engine that restores governance state from the latest snapshot
crates/amun-benchmarks/benches/consensus_bench.rs:14:        commitment: None,
crates/amun-benchmarks/benches/consensus_bench.rs:1:use amun_consensus_network::engine::ConsensusEngine;
crates/amun-benchmarks/benches/consensus_bench.rs:21:            let mut engine = ConsensusEngine::new([0u8; 32], 4);
crates/amun-benchmarks/benches/consensus_bench.rs:37:            let mut engine = ConsensusEngine::new([0u8; 32], 4);
crates/amun-benchmarks/benches/storage_bench.rs:13:        commitment_root: [0u8; 32],
crates/amun-block-builder/src/lib.rs:222:        assert_eq!(builder.engine.state.balance_of(&a1), 900);
crates/amun-block-builder/src/lib.rs:223:        assert_eq!(builder.engine.state.balance_of(&a2), 600);
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:33:    builder.engine.state.create_account(a1, 1000);
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:42:        .state_root_with_ledger(&builder.engine.economic);
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:67:    builder1.engine.state.create_account(a, 1000);
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:68:    builder2.engine.state.create_account(a, 999);
crates/amun-block-builder/tests/n28_first_economic_block.rs:113:    assert_eq!(builder.engine.state.balance_of(&alice), 100);
crates/amun-block-builder/tests/n28_first_economic_block.rs:15:    builder.engine.state.create_account(alice, 1000);
crates/amun-block-builder/tests/n28_first_economic_block.rs:29:    let genesis_root = builder.engine.state.state_root();
crates/amun-block-builder/tests/n28_first_economic_block.rs:31:    assert_eq!(builder.engine.state.balance_of(&alice), 700);
crates/amun-block-builder/tests/n28_first_economic_block.rs:32:    assert_eq!(builder.engine.state.balance_of(&bob), 300);
crates/amun-block-builder/tests/n28_first_economic_block.rs:54:    builder.engine.state.create_account(a1, 1000);
crates/amun-block-builder/tests/n28_first_economic_block.rs:55:    builder.engine.state.create_account(a2, 500);
crates/amun-block-builder/tests/n28_first_economic_block.rs:84:    assert_eq!(builder.engine.state.balance_of(&a1), 800);
crates/amun-block-builder/tests/n28_first_economic_block.rs:85:    assert_eq!(builder.engine.state.balance_of(&a3), 350);
crates/amun-block-builder/tests/n28_first_economic_block.rs:96:    builder.engine.state.create_account(alice, 100);
crates/amun-block-builder/tests/n32_certified_block.rs:146:    assert_eq!(builder.engine.state.balance_of(&alice), 700);
crates/amun-block-builder/tests/n32_certified_block.rs:157:    builder.engine.state.create_account(alice, 1000);
crates/amun-block-builder/tests/n32_certified_block.rs:158:    let state_root_before = builder.engine.state.state_root();
crates/amun-block-builder/tests/n32_certified_block.rs:191:            .state_root_with_ledger(&builder.engine.economic)
crates/amun-block-builder/tests/n32_certified_block.rs:34:    builder.engine.state.create_account(alice, 1000);
crates/amun-block-builder/tests/n32_certified_block.rs:72:    assert_eq!(builder.engine.state.balance_of(&alice), 700);
crates/amun-block-builder/tests/n32_certified_block.rs:73:    assert_eq!(builder.engine.state.balance_of(&bob), 300);
crates/amun-block-builder/tests/n32_certified_block.rs:84:    builder.engine.state.create_account(alice, 1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:186:        archetype: ResourceArchetype::Evidence,
crates/amun-byzantine-tests/tests/attack_suite.rs:58:        replay,
crates/amun-consensus-integration/src/consensus_integrator.rs:133:        assert_eq!(block.transitions.len(), 1);
crates/amun-consensus-integration/src/consensus_integrator.rs:161:        assert!(cert.verify());
crates/amun-consensus-integration/src/consensus_integrator.rs:240:        assert!(!block.transitions.is_empty());
crates/amun-consensus-network/src/engine.rs:614:        assert!(engine.is_finalized(1));
crates/amun-consensus-network/src/engine.rs:718:            assert!(engine.is_finalized(height));
crates/amun-consensus-network/src/evidence_push.rs:4:// and full EvidenceRecords to known peers.  This reduces latency in
crates/amun-consensus-network/src/evidence_store.rs:316:        assert_eq!(s.get_by_id(&id).unwrap().status, EvidenceStatus::Pending);
crates/amun-consensus-network/src/evidence_store.rs:318:        assert_eq!(s.get_by_id(&id).unwrap().status, EvidenceStatus::Confirmed);
crates/amun-consensus-network/src/evidence_store.rs:320:        assert_eq!(s.get_by_id(&id).unwrap().status, EvidenceStatus::Slashed);
crates/amun-consensus-network/src/execution_commitment.rs:184:        assert!(commit.verify().is_ok(), "signature must verify");
crates/amun-consensus-network/src/finality_gate.rs:73:        assert_eq!(result.unwrap(), "executed");
crates/amun-consensus-network/src/finality_gate.rs:83:        assert!(result.unwrap_err().contains("not finalized"));
crates/amun-consensus-network/src/integrated_slashing.rs:355:        assert_eq!(updated.status, EvidenceStatus::Slashed);
crates/amun-consensus-network/src/messages.rs:343:        assert!(qc.verify());
crates/amun-consensus-network/src/messages.rs:406:        assert!(decoded.verify());
crates/amun-consensus-network/src/misbehavior_registry.rs:340:        assert!(reg.record_misbehavior(&[1u8; 32], &ev_id, &EvidenceType::DoubleVote, 1));
crates/amun-consensus-network/src/misbehavior_registry.rs:343:        assert!(!reg.record_misbehavior(&[1u8; 32], &ev_id, &EvidenceType::DoubleVote, 2));
crates/amun-consensus-network/src/real_staking_adapter.rs:141:        assert!(result.is_some(), "N110.1b FAIL: slash should execute");
crates/amun-consensus-network/src/slashing_ledger.rs:165:        assert!(ledger.is_executed(&id));
crates/amun-consensus-network/src/slashing_ledger.rs:169:        assert!(r2.is_err(), "N119.2 FAIL: replay must be rejected");
crates/amun-consensus-network/src/slashing_ledger.rs:170:        assert!(r2.unwrap_err().contains("already executed"));
crates/amun-consensus-network/src/slashing_ledger.rs:171:        assert_eq!(ledger.executed_count(), 1);
crates/amun-consensus-network/src/slashing_ledger.rs:180:        assert_eq!(ledger.executed_count(), 1);
crates/amun-consensus-network/src/slashing_state.rs:118:        assert_eq!(state.executed_count(), 1);
crates/amun-consensus-network/src/slashing_state.rs:140:        assert_eq!(state.executed_count(), 1);
crates/amun-consensus-network/src/slashing_state.rs:144:        assert_eq!(state.executed_count(), 2);
crates/amun-consensus-network/src/slashing_state.rs:163:        assert!(result.is_err(), "N121.1 FAIL: replay must be rejected");
crates/amun-consensus-network/src/slashing_state.rs:164:        assert!(result.unwrap_err().contains("already executed"));
crates/amun-consensus-network/src/slashing_state.rs:171:        assert_eq!(state.executed_count(), 1);
crates/amun-consensus-network/src/slashing_state.rs:97:        assert_eq!(state.executed_count(), 0);
crates/amun-consensus-network/src/vote_binding.rs:10:/// If the vote has NO commitment (legacy vote), the check passes
crates/amun-consensus-network/src/vote_binding.rs:207:        // Simulate: 3 votes total, 2 with commitment, 1 legacy — all pass
crates/amun-consensus-network/src/vote_binding.rs:218:            "vote with commitment must pass"
crates/amun-consensus-network/src/vote_binding.rs:222:            "vote with commitment must pass"
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:108:    let commit = make_commitment([1u8; 32], 42, [2u8; 32], [3u8; 32]);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:110:    let encoded = postcard::to_stdvec(&commit).expect("N109.8: serialize");
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:113:    assert_eq!(decoded.validator_id, commit.validator_id);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:114:    assert_eq!(decoded.height, commit.height);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:115:    assert_eq!(decoded.block_hash, commit.block_hash);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:116:    assert_eq!(decoded.state_root, commit.state_root);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:117:    assert_eq!(decoded.execution_root, commit.execution_root);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:11://   7. GATEKEEPER: vote fields match commitment fields
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:121:// TEST 2: Same execution → same commitment (determinism)
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:125:    let c1 = make_commitment([1u8; 32], 5, [0xAA; 32], [0xBB; 32]);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:126:    let c2 = make_commitment([1u8; 32], 5, [0xAA; 32], [0xBB; 32]);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:137:// TEST 3: Different execution → different commitment
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:141:    let c1 = make_commitment([1u8; 32], 5, [0xAA; 32], [0xBB; 32]);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:142:    let c2 = make_commitment([1u8; 32], 5, [0xAA; 32], [0xCC; 32]); // Different state_root
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:152:// TEST 4: Tampered commitment detected
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:156:    let mut commit = make_commitment([9u8; 32], 3, [0x11; 32], [0x22; 32]);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:159:    commit.state_root = [0xFF; 32];
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:161:    // Recompute — should not match the old execution_root
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:163:        &commit.validator_id,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:164:        commit.height,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:165:        &commit.block_hash,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:166:        &commit.state_root,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:169:        recomputed, commit.execution_root,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:180:    let commit = make_commitment(pk, 7, [0xDE; 32], [0xAD; 32]);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:183:    // Any third party can recompute it and verify the signature covers it.
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:184:    let recomputed =
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:187:        recomputed, commit.execution_root,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:191:    // The commitment carries the validator's identity
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:193:        commit.validator_id, pk,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:194:        "N109.8 FAIL: commitment must identify the validator"
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:199:// TEST 6: Different validator → different commitment even with same data
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:206:    let c1 = make_commitment([1u8; 32], 1, bh, sr);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:207:    let c2 = make_commitment([2u8; 32], 1, bh, sr);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:216:// TEST 7: GATEKEEPER — vote fields match commitment fields
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:222:// The vote and its commitment must refer to the exact same execution.
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
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:260:        vote.commitment.execution_root, recomputed,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:266:// TEST 8: Vote with mismatched commitment is detectable
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:275:    // Replace commitment with one for block B (attack attempt)
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:276:    vote.commitment = make_commitment(voter, 3, [0xFF; 32], [0xEE; 32]);
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:279:    let mismatch = vote.block_hash != vote.commitment.block_hash
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:280:        || vote.state_root != vote.commitment.state_root
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:281:        || vote.height != vote.commitment.height
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:282:        || vote.voter_id != vote.commitment.validator_id;
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:286:        "N109.8 FAIL: vote with mismatched commitment must be detectable"
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:290:        vote.block_hash, vote.commitment.block_hash,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:296:// TEST 9: Vote roundtrip with commitment
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:310:        decoded.commitment.execution_root,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:311:        vote.commitment.execution_root
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:314:        decoded.commitment.validator_id,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:315:        vote.commitment.validator_id
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:46:    commitment: ExecutionCommitment,
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:50:// Helper: compute execution_root = blake3(validator_id || height || block_hash || state_root)
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:64:    hasher.finalize().into()
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:67:// Helper: create a commitment with computed execution_root
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:6://   2. Sign → verify cycle
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:7://   3. Same execution → same commitment
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:85:// Helper: create a vote with commitment
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:8://   4. Different execution → different commitment
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:99:        commitment: make_commitment(voter_id, height, block_hash, state_root),
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:9://   5. Tampered commitment rejected
crates/amun-consensus-network/tests/n109_block_propagation.rs:100:            hex::encode(computed),
crates/amun-consensus-network/tests/n109_block_propagation.rs:129:    let computed: [u8; 32] = blake3::hash(&proposal.block_bytes).into();
crates/amun-consensus-network/tests/n109_block_propagation.rs:131:        proposal.block_hash, computed,
crates/amun-consensus-network/tests/n109_block_propagation.rs:56:    let block_bytes = hasher.finalize().as_bytes().to_vec();
crates/amun-consensus-network/tests/n109_block_propagation.rs:61:    let state_root: [u8; 32] = state_hasher.finalize().into();
crates/amun-consensus-network/tests/n109_block_propagation.rs:6:// Tests N109.1 through N109.6 before building N109.7.
crates/amun-consensus-network/tests/n109_block_propagation.rs:95:    let computed: [u8; 32] = blake3::hash(&p.block_bytes).into();
crates/amun-consensus-network/tests/n109_block_propagation.rs:96:    if computed != p.block_hash {
crates/amun-consensus-network/tests/n109_block_propagation.rs:98:            "HASH_INTEGRITY: stated={} computed={}",
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:103:    let mut store_b = EvidenceStore::new();
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:104:    let _gossip_b = EvidenceGossip::new();
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:106:    // Node B tries to validate — evidence is missing
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:109:        EvidenceValidationResult::MissingEvidence { missing_ids } => missing_ids,
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:131:    let response = amun_consensus_network::MissingEvidenceResponse {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:151:    // Phase 5: Node B re-validates — all evidence now present
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:156:        EvidenceValidationResult::AllPresent,
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:171:    let mut store_a = EvidenceStore::new();
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:172:    let ev = make_evidence_record(validator_id, 1, EvidenceType::DoubleVote, 1);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:184:    let mut store_b = EvidenceStore::new();
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:193:    assert_ne!(result, EvidenceValidationResult::AllPresent);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:197:    let response = amun_consensus_network::MissingEvidenceResponse {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:204:    assert_eq!(result_after, EvidenceValidationResult::AllPresent);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:23:        vec![EvidenceCount {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:24:            evidence_type: EvidenceType::DoubleVote,
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:2:// N111.7 — End-to-End Evidence Sync Across Two Simulated Nodes
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:37:/// Helper: create a realistic EvidenceRecord.
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:41:    evidence_type: EvidenceType,
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:43:) -> EvidenceRecord {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:44:    EvidenceRecord::new(
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:63:    let mut store_a = EvidenceStore::new();
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:64:    let mut gossip_a = EvidenceGossip::new();
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:67:    let ev1 = make_evidence_record(validator_id, 1, EvidenceType::DoubleVote, 1);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:68:    let ev2 = make_evidence_record(validator_id, 2, EvidenceType::DoubleVote, 2);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:69:    let ev3 = make_evidence_record(validator_id, 3, EvidenceType::DoubleVote, 3);
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:80:        gossip_a.receive_announcement(EvidenceAnnouncement {
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:92:    // Node A validates locally — all evidence present
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:96:        EvidenceValidationResult::AllPresent,
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:100:    let mut push_b = EvidencePush::default();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:105:        EvidenceValidationResult::MissingEvidence { missing_ids } => {
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:112:        _ => panic!("Expected MissingEvidence before push"),
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:12:    EvidencePush, EvidenceRecord, EvidenceStore, EvidenceType, EvidenceValidationResult,
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:133:        EvidenceValidationResult::AllPresent,
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:147:    eprintln!("N112.4 GATEKEEPER PASSED: push sync eliminated MissingEvidenceRequest cycle");
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:155:    let mut store = EvidenceStore::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:156:    let mut gossip = EvidenceGossip::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:157:    let mut push = EvidencePush::default();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:17:    EvidenceRecord::new(
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:184:    let mut store_a = EvidenceStore::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:185:    let gossip_a = EvidenceGossip::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:186:    let mut push_a = EvidencePush::default();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:197:    let mut store_b = EvidenceStore::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:198:    let mut gossip_b = EvidenceGossip::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:199:    let mut push_b = EvidencePush::default();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:206:    assert_eq!(result, EvidenceValidationResult::AllPresent);
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:20:        EvidenceType::DoubleVote,
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:2:// N112.4 — End-to-End Push Evidence Sync
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:31:        vec![EvidenceCount {
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:32:            evidence_type: EvidenceType::DoubleVote,
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:46:// N112.4 GATEKEEPER — Push sync eliminates the need for MissingEvidenceRequest
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:55:    let mut store_a = EvidenceStore::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:56:    let gossip_a = EvidenceGossip::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:57:    let mut push_a = EvidencePush::default();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:98:    let mut store_b = EvidenceStore::new();
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:99:    let mut gossip_b = EvidenceGossip::new();
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:132:    assert_eq!(state.executed_count(), 0);
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:135:        .execute(&make_cert([0x42; 32], 100, 15000), || Ok(()))
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:137:    assert_eq!(state.executed_count(), 1);
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:140:        .execute(&make_cert([0x99; 32], 200, 5000), || Ok(()))
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:142:    assert_eq!(state.executed_count(), 2);
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:150:    assert_eq!(state.executed_count(), 0);
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:16:        vec![EvidenceCount {
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:17:            evidence_type: EvidenceType::DoubleVote,
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:35:        .execute(&make_cert([0x42; 32], 100, 15000), || Ok(()))
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:38:        .execute(&make_cert([0x99; 32], 200, 5000), || Ok(()))
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:58:    // Simulate: build state, snapshot it, rebuild from scratch
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:68:        original.execute(cert, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:75:        restored.execute(cert, || Ok(())).unwrap();
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:91:        .execute(&make_cert([0x42; 32], 100, 15000), || Ok(()))
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:94:        .execute(&make_cert([0x99; 32], 200, 5000), || Ok(()))
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:97:        .execute(&make_cert([0x42; 32], 300, 10000), || Ok(()))
crates/amun-contract-fuzzing/tests/n171_fuzzing_tests.rs:12:        "Evidence mismatches: {}",
crates/amun-contract-fuzzing/tests/n171_fuzzing_tests.rs:30:        "Evidence mismatches: {}",
crates/amun-contract-fuzzing/tests/n171_fuzzing_tests.rs:47:        "Evidence mismatches: {}",
crates/amun-crypto-hardening/src/anti_replay.rs:63:        assert!(!guard.check_and_record(&hash)); // replay
crates/amun-defi-amm/tests/n153_amm_tests.rs:1:use amun_defi_amm::AmmEngine;
crates/amun-defi-amm/tests/n153_amm_tests.rs:22:    let mut amm1 = AmmEngine::new();
crates/amun-defi-amm/tests/n153_amm_tests.rs:23:    let mut amm2 = AmmEngine::new();
crates/amun-defi-amm/tests/n153_amm_tests.rs:38:    let mut amm = AmmEngine::new();
crates/amun-defi-amm/tests/n153_amm_tests.rs:7:    let mut amm = AmmEngine::new();
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:12:    let e1 = DefiEvidence::generate_swap_evidence([1u8; 32], [10u8; 32], 100, 90, 42);
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:13:    let e2 = DefiEvidence::generate_swap_evidence([1u8; 32], [10u8; 32], 200, 180, 42);
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:19:    let e1 = DefiEvidence::generate_liquidity_evidence([2u8; 32], [20u8; 32], 500, 500, 1000, 10);
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:1:use amun_defi_evidence::DefiEvidence;
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:20:    let e2 = DefiEvidence::generate_liquidity_evidence([2u8; 32], [20u8; 32], 500, 500, 1000, 10);
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:5:    let e1 = DefiEvidence::generate_swap_evidence([1u8; 32], [10u8; 32], 100, 90, 42);
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:6:    let e2 = DefiEvidence::generate_swap_evidence([1u8; 32], [10u8; 32], 100, 90, 42);
crates/amun-defi-governance/tests/n157_governance_tests.rs:10:    assert!(engine.execute(&prop_id));
crates/amun-defi-governance/tests/n157_governance_tests.rs:16:    let mut engine1 = GovernanceEngine::new();
crates/amun-defi-governance/tests/n157_governance_tests.rs:17:    let mut engine2 = GovernanceEngine::new();
crates/amun-defi-governance/tests/n157_governance_tests.rs:1:use amun_defi_governance::GovernanceEngine;
crates/amun-defi-governance/tests/n157_governance_tests.rs:22:    engine1.execute(&id1);
crates/amun-defi-governance/tests/n157_governance_tests.rs:23:    engine2.execute(&id2);
crates/amun-defi-governance/tests/n157_governance_tests.rs:32:    let mut engine = GovernanceEngine::new();
crates/amun-defi-governance/tests/n157_governance_tests.rs:36:    engine.execute(&prop_id);
crates/amun-defi-governance/tests/n157_governance_tests.rs:5:    let mut engine = GovernanceEngine::new();
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:10:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:26:    let mut engine = LendingEngine::new();
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:2:use amun_defi_lending_engine::LendingEngine;
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:30:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:45:    let mut engine = LendingEngine::new();
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:49:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:64:    let mut engine = LendingEngine::new();
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:68:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:6:    let mut engine = LendingEngine::new();
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:84:    let mut engine1 = LendingEngine::new();
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:85:    let mut engine2 = LendingEngine::new();
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:88:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:99:            &mut amun_resource_core::ResourceRegistry::new(100),
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:18:    let mut engine = StablecoinEngine::new();
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:1:use amun_defi_stablecoin::StablecoinEngine;
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:27:    let mut engine1 = StablecoinEngine::new();
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:28:    let mut engine2 = StablecoinEngine::new();
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:7:    let mut engine = StablecoinEngine::new();
crates/amun-evidence-finality/src/evidence_finality.rs:233:            return Err("Not all transitions passed replay verification".into());
crates/amun-evidence-finality/src/evidence_finality.rs:307:        assert!(cert.verify());
crates/amun-evidence-root/src/lib.rs:127:        assert!(genesis.verify());
crates/amun-evidence-root/src/lib.rs:151:        assert!(chain.verify());
crates/amun-evidence-root/src/lib.rs:162:        assert!(!chain.verify());
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:100:    let e1 = EvidenceRoot::compute(before, [0u8; 32], [0u8; 32], [0u8; 32], [0u8; 32], 1);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:101:    let e2 = EvidenceRoot::compute(after, [0u8; 32], [0u8; 32], [0u8; 32], [0u8; 32], 1);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:13:        sk.verifying_key().to_bytes()
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:15:    builder.engine.state.create_account(alice, 1000);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:1:use amun_block_builder::BlockBuilder;
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:2:use amun_evidence_root::{EvidenceChain, EvidenceRoot};
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:30:    let evidence = EvidenceRoot::compute(
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:38:    assert!(evidence.verify());
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:45:    let mut builder = BlockBuilder::new();
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:49:        sk.verifying_key().to_bytes()
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:51:    builder.engine.state.create_account(alice, 1000);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:52:    let mut chain = EvidenceChain::new();
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:71:    assert!(chain.verify());
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:77:    let mut builder = BlockBuilder::new();
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:81:        sk.verifying_key().to_bytes()
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:83:    builder.engine.state.create_account(alice, 1000);
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:84:    let before = builder.engine.state.state_root();
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:99:    let after = builder.engine.state.state_root();
crates/amun-evidence-root/tests/n40_evidence_backed_block.rs:9:    let mut builder = BlockBuilder::new();
crates/amun-evidence/src/tests.rs:37:        let evidence = Evidence {
crates/amun-evidence/src/tests.rs:38:            evidence_type: EvidenceType::Equivocation,
crates/amun-evidence/src/tests.rs:61:        assert!(evidence.verify().is_err());
crates/amun-evidence/src/tests.rs:7:        let evidence = Evidence {
crates/amun-evidence/src/tests.rs:8:            evidence_type: EvidenceType::Equivocation,
crates/amun-experimental-framework/src/main.rs:229:// ── Experiment 2: Replay vs Execution (all workloads) ───────
crates/amun-experimental-framework/src/main.rs:231:    println!("\n=== Experiment 2: Replay vs Execution ===");
crates/amun-experimental-framework/src/main.rs:308:        // Measure replay
crates/amun-gas-engine/src/gas_engine.rs:118:        assert!(GasEngine::can_execute(1000, 500));
crates/amun-gas-engine/src/gas_engine.rs:119:        assert!(!GasEngine::can_execute(100, 500));
crates/amun-gas-engine/src/gas_engine.rs:129:        assert!(matches!(result, GasEngineResult::Success { .. }));
crates/amun-gas-engine/src/gas_engine.rs:94:        assert!(matches!(result, GasEngineResult::Success { gas_used: 300 }));
crates/amun-live-cluster/src/bin/multi_validator_test.rs:55:    // Phase 2: kill validator 2, verify quorum continues (30s)
crates/amun-live-cluster/src/bin/multi_validator_test.rs:80:    // Phase 3: restart validator 2, verify catch-up (60s)
crates/amun-live-cluster/src/validator.rs:1034:            assert_eq!(executed.len(), 1, "RetireAuthority should execute");
crates/amun-live-cluster/src/validator.rs:875:            assert_eq!(executed.len(), 1, "Proposal should be executed");
crates/amun-live-cluster/src/validator.rs:947:            assert_eq!(executed.len(), 1, "AddAuthority should execute");
crates/amun-live-cluster/src/validator.rs:979:            assert_eq!(executed.len(), 1, "ScheduleTransition should execute");
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:27:        vec![EvidenceCount {
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:28:            evidence_type: EvidenceType::DoubleVote,
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:2:    EvidenceCount, EvidenceType, MisbehaviorRegistry, MisbehaviorThresholds, RealStakingExecutor,
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:50:    misbehavior.record_misbehavior(&vid, &[0x01; 32], &EvidenceType::DoubleVote, 1);
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:51:    misbehavior.record_misbehavior(&vid, &[0x02; 32], &EvidenceType::DoubleVote, 2);
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:52:    misbehavior.record_misbehavior(&vid, &[0x03; 32], &EvidenceType::DoubleVote, 3);
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:78:    misbehavior.record_misbehavior(&vid, &[0x01; 32], &EvidenceType::DoubleVote, 1);
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:79:    misbehavior.record_misbehavior(&vid, &[0x02; 32], &EvidenceType::DoubleVote, 2);
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:80:    misbehavior.record_misbehavior(&vid, &[0x03; 32], &EvidenceType::DoubleVote, 3);
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:20:    misbehavior.record_misbehavior(&validator_id, &[0xA1; 32], &EvidenceType::DoubleVote, 1);
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:21:    misbehavior.record_misbehavior(&validator_id, &[0xA2; 32], &EvidenceType::DoubleVote, 2);
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:22:    misbehavior.record_misbehavior(&validator_id, &[0xA3; 32], &EvidenceType::DoubleVote, 3);
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:5:    EvidenceType, MisbehaviorRegistry, MisbehaviorThresholds, RealStakingExecutor, StakingAdapter,
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:10:    let mut builder = BlockBuilder::new();
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:24:    let mut builder = BlockBuilder::new();
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:2:use amun_block_builder::BlockBuilder;
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:39:    let mut builder = BlockBuilder::new();
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:59:    // Simulates: proposer builds block with root X, validator computes root Y
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:64:    let mut builder = BlockBuilder::new();
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:8:    // Setup: build a block with a known slashing_root
crates/amun-live-cluster/tests/n129_evidence_continuity_audit.rs:103:        root.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
crates/amun-live-cluster/tests/n129_evidence_continuity_audit.rs:124:        root.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
crates/amun-network-fastpath/tests/n164_fastpath_tests.rs:54:    let hash1 = batch1.finalize();
crates/amun-network-fastpath/tests/n164_fastpath_tests.rs:55:    let hash2 = batch2.finalize();
crates/amun-networking/src/crypto_identity.rs:102:        assert_eq!(peer_id.0, keypair.verifying_key.to_bytes());
crates/amun-networking/src/crypto_identity.rs:110:        assert!(PeerKeyPair::verify(
crates/amun-networking/src/crypto_identity.rs:122:        assert!(!PeerKeyPair::verify(
crates/amun-networking/src/crypto_identity.rs:135:        assert!(!PeerKeyPair::verify(
crates/amun-networking/src/crypto_identity.rs:147:        assert!(signed.verify());
crates/amun-networking/src/crypto_identity.rs:148:        assert_eq!(signed.sender_peer_id().0, keypair.verifying_key.to_bytes());
crates/amun-networking/src/crypto_identity.rs:157:        assert!(decoded.verify());
crates/amun-networking/src/peer_discovery.rs:100:        assert!(PeerAnnouncement::verify(&announcement, &signed));
crates/amun-networking/src/peer_discovery.rs:110:        assert!(!PeerAnnouncement::verify(&announcement, &signed));
crates/amun-networking/src/signed_envelope.rs:104:        assert!(directed.signed_envelope.verify());
crates/amun-networking/src/signed_envelope.rs:67:        assert!(signed.verify());
crates/amun-networking/src/signed_envelope.rs:83:        assert!(!signed.verify());
crates/amun-networking/tests/harness/event_scheduler.rs:22:impl Ord for ScheduledEvent {
crates/amun-networking/tests/harness/event_scheduler.rs:30:impl PartialOrd for ScheduledEvent {
crates/amun-networking/tests/harness/event_scheduler.rs:35:impl PartialEq for ScheduledEvent {
crates/amun-networking/tests/harness/event_scheduler.rs:40:impl Eq for ScheduledEvent {}
crates/amun-networking/tests/harness/event_scheduler.rs:52:impl SchedulingPolicy for DefaultPolicy {}
crates/amun-networking/tests/harness/event_scheduler.rs:60:impl EventScheduler {
crates/amun-networking/tests/harness/handlers.rs:56:        vote_type: VoteType::Precommit,
crates/amun-networking/tests/harness/handlers.rs:64:        message_type: "precommit".into(),
crates/amun-networking/tests/harness/handlers.rs:94:        "precommit" => {
crates/amun-networking/tests/harness/message_delivery.rs:14:impl Default for DeliveryPolicy {
crates/amun-networking/tests/harness/message_delivery.rs:37:impl MessageDeliveryEngine {
crates/amun-networking/tests/harness/mod.rs:12:pub use message_delivery::{DelayedEnvelope, DeliveryPolicy, MessageDeliveryEngine};
crates/amun-networking/tests/harness/scenario.rs:25:impl Default for ScenarioConfig {
crates/amun-networking/tests/harness/scenario.rs:40:    pub commits: usize,
crates/amun-networking/tests/harness/scenario.rs:45:    pub delivery_engine: MessageDeliveryEngine,
crates/amun-networking/tests/harness/scenario.rs:49:impl ScenarioRunner {
crates/amun-networking/tests/harness/scenario.rs:4:use super::message_delivery::MessageDeliveryEngine;
crates/amun-networking/tests/harness/scenario.rs:53:            delivery_engine: MessageDeliveryEngine::new(
crates/amun-networking/tests/harness/scenario.rs:68:            commits: 0,
crates/amun-networking/tests/harness/scenario.rs:91:            ConsensusAction::BroadcastPrecommit(v) => {
crates/amun-networking/tests/harness/scenario.rs:98:                        message_type: "precommit".into(),
crates/amun-networking/tests/harness/simulation_node.rs:20:impl SimulationNodeCore {
crates/amun-networking/tests/harness/simulation_node.rs:33:impl fmt::Debug for SimulationNodeCore {
crates/amun-networking/tests/n17_multi_node_network.rs:103:    assert!(net.run_until_commits(500, 1));
crates/amun-networking/tests/n17_multi_node_network.rs:109:    assert!(net.run_until_commits(800, 1));
crates/amun-networking/tests/n17_multi_node_network.rs:116:    net.run_until_commits(500, 1);
crates/amun-networking/tests/n17_multi_node_network.rs:121:    // Rebuild validator set from remaining live nodes
crates/amun-networking/tests/n17_multi_node_network.rs:134:    let _ = net.run_until_commits(2000, 2);
crates/amun-networking/tests/n17_multi_node_network.rs:135:    // If it doesn't reach 2 more commits, that's expected with tight quorum
crates/amun-networking/tests/n17_multi_node_network.rs:138:    // At minimum, we should have at least the original commit
crates/amun-networking/tests/n17_multi_node_network.rs:139:    let total_commits: usize = net.nodes.values().map(|n| n.committed_blocks.len()).sum();
crates/amun-networking/tests/n17_multi_node_network.rs:141:        total_commits >= 3,
crates/amun-networking/tests/n17_multi_node_network.rs:142:        "All nodes should have at least 1 commit each"
crates/amun-networking/tests/n17_multi_node_network.rs:156:    assert!(net.run_until_commits(1000, 3));
crates/amun-networking/tests/n17_multi_node_network.rs:15:///   5. When a QC forms, consensus emits Commit → block finalized
crates/amun-networking/tests/n17_multi_node_network.rs:22:impl Network {
crates/amun-networking/tests/n17_multi_node_network.rs:91:                .all(|n| n.committed_blocks.len() >= target_commits);
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:90:    // 4 nodes commit 10 blocks
crates/amun-networking/tests/n18_catchup_and_rejoin.rs:92:    // Network commits 20 more
crates/amun-networking/tests/n18_checkpoint_sync.rs:12:/// Helper: build a checkpoint covering blocks [start, end].
crates/amun-networking/tests/n18_checkpoint_sync.rs:19:        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
crates/amun-networking/tests/n18_checkpoint_sync.rs:86:    // Node B transitions through lifecycle
crates/amun-networking/tests/n18_full_rejoin.rs:156:    // Only after proper lifecycle transition
crates/amun-networking/tests/n18_full_rejoin.rs:19:        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
crates/amun-networking/tests/n18_full_rejoin.rs:57:    // Phase 1: Network commits 10 blocks, then node 3 is removed
crates/amun-networking/tests/n18_full_rejoin.rs:61:    // Phase 2: Network commits 40 more blocks (50 total)
crates/amun-networking/tests/n18_node_rejoin.rs:61:    // 1. 4-node network commits block 1
crates/amun-networking/tests/n18_node_rejoin.rs:63:    // 3. Network commits 20 more blocks
crates/amun-networking/tests/n19_adversarial_rejoin.rs:17:        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:132:    // Phase 2: New node starts bootstrapping
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:137:    // Phase 3: New node receives checkpoint from existing peer
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:171:            keypair.verifying_key.to_bytes(),
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:68:        alice_keypair.verifying_key.to_bytes(),
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:73:        bob_keypair.verifying_key.to_bytes(),
crates/amun-networking/tests/n20_10_multi_machine_testnet.rs:91:        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:111:    // Node B transitions to active
crates/amun-networking/tests/n20_8_bootstrap_over_tcp.rs:19:        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:152:    // Trying to verify old checkpoint against new root must fail
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:18:        rt.apply_transition(&[height as u8; 32], &[0xAA; 32]);
crates/amun-networking/tests/n20_9_rejoin_over_tcp.rs:84:    // Phase 6: Full lifecycle transition
crates/amun-networking/tests/v2_001_time_driven_consensus.rs:30:        "Happy path consensus failed: {} commits",
crates/amun-networking/tests/v2_001_time_driven_consensus.rs:31:        result.commits
crates/amun-networking/tests/v2_001_time_driven_consensus.rs:9:impl ConsensusScenario for HappyPathScenario {
crates/amun-networking/tests/v2_002_multi_round_consensus.rs:10:impl ConsensusScenario for MultiRoundScenario {
crates/amun-networking/tests/v2_002_multi_round_consensus.rs:30:            // Protocol emits prevote/precommit via actions, just need inbox to process
crates/amun-networking/tests/v2_002_multi_round_consensus.rs:54:            "Multi-round failed at {} rounds: {} commits",
crates/amun-networking/tests/v2_002_multi_round_consensus.rs:55:            rounds, result.commits
crates/amun-networking/tests/v2_003_multi_height_consensus.rs:10:impl ConsensusScenario for MultiHeightScenario {
crates/amun-networking/tests/v2_003_multi_height_consensus.rs:53:            "Multi-height failed at {} heights: {} commits",
crates/amun-networking/tests/v2_003_multi_height_consensus.rs:54:            heights, result.commits
crates/amun-networking/tests/v2_004_long_run_stability.rs:11:impl ConsensusScenario for LongRunScenario {
crates/amun-networking/tests/v2_004_long_run_stability.rs:54:        "V2-004: {} heights -> {} commits (min={})",
crates/amun-networking/tests/v2_004_long_run_stability.rs:55:        heights, result.commits, min
crates/amun-networking/tests/v2_013_baseline_matrix.rs:13:impl BaselineScenario {
crates/amun-networking/tests/v2_013_baseline_matrix.rs:22:impl ConsensusScenario for BaselineScenario {
crates/amun-networking/tests/v2_013_baseline_matrix.rs:48:        let mut committed_counts = Vec::new();
crates/amun-networking/tests/v2_013_baseline_matrix.rs:55:            committed_counts.push(result.commits);
crates/amun-networking/tests/v2_013_baseline_matrix.rs:58:        committed_counts.sort();
crates/amun-networking/tests/v2_013_baseline_matrix.rs:59:        let min = committed_counts.first().unwrap();
crates/amun-networking/tests/v2_013_baseline_matrix.rs:60:        let max = committed_counts.last().unwrap();
crates/amun-networking/tests/v2_013_baseline_matrix.rs:61:        let avg = committed_counts.iter().map(|x| *x as u64).sum::<u64>() as f64 / trials as f64;
crates/amun-networking/tests/v2_013_baseline_matrix.rs:63:            committed_counts.iter().filter(|&&c| c >= 27).count() as f64 / trials as f64 * 100.0;
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:114:    commits: u64,
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:121:impl TimeoutInjectionSim {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:143:            commits: 0,
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:162:                precommits_broadcast: false,
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:166:                commit_counted: false,
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:24:    PrecommitBroadcast,
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:282:            EventType::PrecommitBroadcast => {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:284:                if !node.precommits_broadcast
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:285:                    && node.state_machine.state.step == ConsensusStep::Precommit
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:293:                            vote_type: VoteType::Precommit,
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:301:                            message_type: "precommit".into(),
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:307:                            n.precommits_broadcast = true;
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:339:                        "precommit" => {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:350:                if node.state_machine.last_committed_height == 1 && !node.commit_counted {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:351:                    self.commits += 1;
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:352:                    node.commit_counted = true;
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:357:                if node.state_machine.last_committed_height < 1 {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:365:                            node.precommits_broadcast = false;
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:38:impl Ord for ScheduledEvent {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:397:                .schedule(inject_time + 40, nid.clone(), EventType::PrecommitBroadcast);
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:3:// Root cause: Prevote/Precommit were not scheduled after timeout recovery.
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:422:        base_precommit_timeout_ms: timeout_ms / 2,
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:440:    let commits = sim.commits;
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:442:        "=== Timeout Injection Test: {} commits after injecting proposal at round {} ===",
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:443:        commits, inject_after_rounds
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:446:        commits >= 27,
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:46:impl PartialOrd for ScheduledEvent {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:51:impl PartialEq for ScheduledEvent {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:56:impl Eq for ScheduledEvent {}
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:62:impl EventScheduler {
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:95:    precommits_broadcast: bool,
crates/amun-networking/tests/v2_015a_real_timeout_injection.rs:99:    commit_counted: bool,
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:3:use amun_nft_evidence::NftEvidenceKernel;
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:4:use amun_nft_marketplace::MarketplaceEngine;
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:59:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:101:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:1:use amun_nft_marketplace::MarketplaceEngine;
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:133:    let mut engine1 = NftCollateralEngine::new();
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:134:    let mut engine2 = NftCollateralEngine::new();
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:23:    let mut engine = NftCollateralEngine::new();
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:2:use amun_nft_collateral::NftCollateralEngine;
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:48:    let mut engine = NftCollateralEngine::new();
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:74:    let mut engine = NftCollateralEngine::new();
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:94:    let mut engine = NftCollateralEngine::new();
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:104:    assert_eq!(result, Err(CekError::Law4ReplayDetected));
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:121:    let ev1 = NftEvidence::new(event1, 1000, 1);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:122:    let ev2 = NftEvidence::new(event2, 2000, 1);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:12:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:131:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:138:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:167:    let evidence = NftEvidenceKernel::generate_evidence(event.clone(), timestamp, 42).unwrap();
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:19:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:1:use amun_nft_core::{NftEvent, NftEvidence};
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:3:    accumulate_nft_evidence_root, CekError, MintVerificationContext, NftEvidenceKernel,
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:48:    let evidence = NftEvidenceKernel::generate_evidence(event, timestamp, 1).unwrap();
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:57:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:66:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:78:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-evidence/tests/n131_evidence_tests.rs:85:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:1:use amun_nft_explorer::ExplorerEngine;
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:21:    let collections = ExplorerEngine::get_collections(&reg);
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:40:    let nft = ExplorerEngine::get_nft(&reg, &token_id).unwrap();
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:69:    let owner_data = ExplorerEngine::get_owner_nfts(&reg, &owner);
crates/amun-nft-integration/tests/n132_integration_tests.rs:16:    let original = EvidenceRoot::genesis();
crates/amun-nft-integration/tests/n132_integration_tests.rs:1:use amun_evidence_root::EvidenceRoot;
crates/amun-nft-integration/tests/n132_integration_tests.rs:27:    let original = EvidenceRoot::compute(
crates/amun-nft-integration/tests/n132_integration_tests.rs:6:    let original = EvidenceRoot::genesis();
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:107:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:132:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:155:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:179:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:1:use amun_nft_marketplace::MarketplaceEngine;
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:24:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:52:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:78:    let mut mp = MarketplaceEngine::new();
crates/amun-nft-rights-enforcement/tests/n141_rights_enforcement_tests.rs:4:use amun_nft_rights_enforcement::RightsEnforcementEngine;
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:26:    let mut engine = SettlementEngine::new();
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:35:    let mut engine = SettlementEngine::new();
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:3:use amun_nft_royalty_settlement::SettlementEngine;
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:53:    let mut e1 = SettlementEngine::new();
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:54:    let mut e2 = SettlementEngine::new();
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:83:    let mut engine = SettlementEngine::new();
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:19:    let record = RoyaltyEngine::generate_royalty_record([10u8; 32], &policy, [3u8; 32], 5000, 42);
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:52:    let record1 = RoyaltyEngine::generate_royalty_record([10u8; 32], &policy, [3u8; 32], 1000, 1);
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:53:    let record2 = RoyaltyEngine::generate_royalty_record([11u8; 32], &policy, [4u8; 32], 2000, 2);
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:62:    let record = RoyaltyEngine::generate_royalty_record(
crates/amun-nft-stress/tests/n146_stress_tests.rs:103:    use amun_nft_marketplace::MarketplaceEngine;
crates/amun-nft-stress/tests/n146_stress_tests.rs:112:    let mut marketplace = MarketplaceEngine::new();
crates/amun-nft-stress/tests/n146_stress_tests.rs:1:use amun_nft_marketplace::MarketplaceEngine;
crates/amun-nft-stress/tests/n146_stress_tests.rs:227:    assert!(!EnforcementEngine::can_be_sold(
crates/amun-nft-stress/tests/n146_stress_tests.rs:243:    assert!(EnforcementEngine::can_be_sold(
crates/amun-nft-stress/tests/n146_stress_tests.rs:69:    let mut mp = MarketplaceEngine::new();
crates/amun-node/src/bin/test_crash_recovery.rs:109:    println!("Phase 1: Running network to 5 commits...");
crates/amun-node/src/bin/test_crash_recovery.rs:131:    // Rebuild validator set with 3 nodes
crates/amun-node/src/bin/test_crash_recovery.rs:166:    new_node.consensus.last_committed_height = current_height.saturating_sub(1);
crates/amun-node/src/bin/test_crash_recovery.rs:171:    // Rebuild full validator set
crates/amun-node/src/bin/test_crash_recovery.rs:202:        println!("\nResult: Network did not reach target commits after rejoin.");
crates/amun-node/src/bin/test_crash_recovery_7.rs:106:    // Phase 1: Run with 7 validators to 5 commits
crates/amun-node/src/bin/test_crash_recovery_7.rs:107:    println!("Phase 1: Running 7-validator network to 5 commits...");
crates/amun-node/src/bin/test_crash_recovery_7.rs:152:        println!("\nPASS: Network survived but did not reach target commits");
crates/amun-node/src/bin/test_multi_byzantine.rs:171:        println!("\nPARTIAL: Honest validators converge but commit rate affected");
crates/amun-node/src/bin/test_network_state_convergence.rs:35:    // Run until we have 3 commits
crates/amun-node/src/bin/test_network_state_convergence.rs:38:        .map(|n| n.committed_blocks.len())
crates/amun-node/src/bin/test_network_state_convergence.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
crates/amun-node/src/bin/test_network_state_convergence.rs:76:        // Process incoming + update state store when commits happen
crates/amun-node/src/bin/test_network_state_convergence.rs:79:                let prev_commits = node.committed_blocks.len();
crates/amun-node/src/bin/test_network_state_convergence.rs:82:                // If a new block was committed, update the state store
crates/amun-node/src/bin/test_network_state_convergence.rs:83:                if node.committed_blocks.len() > prev_commits {
crates/amun-node/src/bin/test_network_state_convergence.rs:85:                        let height = node.committed_blocks.len() as u64;
crates/amun-node/src/bin/test_network_state_convergence.rs:92:                            lineage: ResourceLineage::genesis(resource_id),
crates/amun-node/src/bin/test_state_recovery_rejoin.rs:113:    // Phase 1: Run with 7 validators to 5 commits
crates/amun-node/src/bin/test_state_recovery_rejoin.rs:114:    println!("Phase 1: Running 7-validator network to 5 commits...");
crates/amun-node/src/bin/test_state_recovery_rejoin.rs:171:    new_node.consensus.last_committed_height = current_height.saturating_sub(1);
crates/amun-node/src/bin/test_state_recovery_rejoin.rs:189:    new_node_4.consensus.last_committed_height = current_height.saturating_sub(1);
crates/amun-node/src/bin/test_state_recovery_rejoin.rs:194:    // Rebuild full validator set
crates/amun-node/src/bin/test_state_recovery_rejoin.rs:229:        println!("\nResult: Network did not reach target commits after full rejoin.");
crates/amun-node/src/bin/test_sync_rejoin.rs:109:    // Phase 1: Run with 7 validators to 5 commits
crates/amun-node/src/bin/test_sync_rejoin.rs:110:    println!("Phase 1: Running 7-validator network to 5 commits...");
crates/amun-replay-engine/src/lib.rs:158:        assert!(cert.verify());
crates/amun-replay-engine/src/lib.rs:166:        assert!(session.replay(&entries).is_err());
crates/amun-replay-engine/src/operational_hasher.rs:50:        let h1 = OperationalHasher::new(b"TEST").update_u64(42).clone().finalize();
crates/amun-replay-engine/src/operational_hasher.rs:51:        let h2 = OperationalHasher::new(b"TEST").update_u64(42).clone().finalize();
crates/amun-replay-engine/src/operational_hasher.rs:58:        let op_hash = OperationalHasher::new(b"TEST").update_u64(42).clone().finalize();
crates/amun-replay-engine/src/operational_hasher.rs:61:        assert_ne!(op_hash, [0u8; 32]); // Just verify it's not empty
crates/amun-replay-engine/src/zk_adapters.rs:245:        assert_ne!(c.commitment, [0; 32]);
crates/amun-replay/src/commit_log.rs:114:        assert_eq!(c.commit_hash(), c.commit_hash());
crates/amun-replay/src/commit_log.rs:124:        assert_ne!(c1.commit_hash(), c2.commit_hash());
crates/amun-replay/src/commit_log.rs:89:        assert_eq!(commit.height, 1);
crates/amun-replay/src/validation.rs:106:        assert_eq!(result.commits_checked, 0);
crates/amun-replay/src/validation.rs:80:        assert_eq!(result.commits_checked, 3);
crates/amun-replay/src/validation.rs:91:        assert_eq!(result.commits_checked, 1);
crates/amun-replay/src/validation.rs:99:        assert_eq!(result.commits_checked, 1);
crates/amun-soak-test/src/lib.rs:14:    pub amm: Arc<Mutex<AmmEngine>>,
crates/amun-soak-test/src/lib.rs:15:    pub lending: Arc<Mutex<LendingEngine>>,
crates/amun-soak-test/src/lib.rs:179:impl Default for ValidatorSimulator {
crates/amun-soak-test/src/lib.rs:1:use amun_defi_amm::AmmEngine;
crates/amun-soak-test/src/lib.rs:20:impl ValidatorSimulator {
crates/amun-soak-test/src/lib.rs:24:            amm: Arc::new(Mutex::new(AmmEngine::new())),
crates/amun-soak-test/src/lib.rs:25:            lending: Arc::new(Mutex::new(LendingEngine::new())),
crates/amun-soak-test/src/lib.rs:2:use amun_defi_lending_engine::LendingEngine;
crates/amun-testnet-sim/tests/adversarial_tests.rs:106:    assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:137:    assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:163:    // Use the same initial state for all replays.
crates/amun-testnet-sim/tests/adversarial_tests.rs:213:                if !matches!(replay, ReplayResult::Match { .. }) {
crates/amun-testnet-sim/tests/adversarial_tests.rs:214:                    panic!("Block {} replay failed: {:?}", i + 1, replay);
crates/amun-testnet-sim/tests/adversarial_tests.rs:252:    assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:63:    assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:69:    assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:76:        assert!(ReplayBackedConsensus::form_consensus(
crates/amun-validator-attestation/src/attestation.rs:14:impl ValidatorAttestation {
crates/amun-validator-attestation/src/attestation.rs:30:        attestation_hash.copy_from_slice(&h.finalize().as_bytes()[..32]);
crates/amun-validator-attestation/src/attestation.rs:43:        let mut computed = [0u8; 32];
crates/amun-validator-attestation/src/attestation.rs:44:        computed.copy_from_slice(&h.finalize().as_bytes()[..32]);
crates/amun-validator-attestation/src/attestation.rs:45:        computed == self.attestation_hash
crates/amun-validator-attestation/src/validator_set.rs:19:impl ValidatorSet {
crates/amun-verification-kernel/src/lib.rs:176:            evidence_type: EvidenceType::TestResult,
docs/N126_FINAL_BASELINE.md:35:10. EvidenceValidity          = all certs pass .verify()               REAL
docs/REPOSITORY_LAYOUT.md:21:| fixtures/ | Replay and snapshot test fixtures |
docs/V7_009_WHY_CLOSURE.md:90:## 4. Experimental Evidence
docs/architecture/CRATE_CLASSIFICATION.md:38:- amun-test-replay - Replay testing
docs/audit/TRACEABILITY_MATRIX.md:10:| N48.5-W14 Replay | amun-replay-verifier | 3 tests | ✅ |
docs/audit/TRACEABILITY_MATRIX.md:13:| N48.5-W19 Finality | amun-evidence-finality | 3 tests | ✅ |
docs/audit/TRACEABILITY_MATRIX.md:8:| N48.5-W4 Evidence | amun-evidence-engine | 5 tests | ✅ |
crates/amun-bench/tests/n161_state_root_bench.rs:10:    let mut reg = ResourceRegistry::new(20000);
crates/amun-bench/tests/n161_state_root_bench.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-bench/tests/n162_snapshot_bench.rs:10:    let mut reg = ResourceRegistry::new(20000);
crates/amun-bench/tests/n162_snapshot_bench.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-benchmarks/benches/sync_bench.rs:1:use amun_resource_core::ResourceRegistry;
crates/amun-benchmarks/benches/sync_bench.rs:6:    let mut reg = ResourceRegistry::new(size as usize * 2);
crates/amun-byzantine-tests/tests/attack_suite.rs:107:    let hash_b = ResourceRegistry::hash_resource(reg.get(&b).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:11:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-byzantine-tests/tests/attack_suite.rs:126:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:137:    let parent_hash = ResourceRegistry::hash_resource(reg.get(&parent_id).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:154:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:182:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:193:    let parent_hash = ResourceRegistry::hash_resource(reg.get(&ev).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:210:    let mut reg = ResourceRegistry::new(100_000);
crates/amun-byzantine-tests/tests/attack_suite.rs:224:        let hash = ResourceRegistry::hash_resource(reg.get(&parent).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:245:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-byzantine-tests/tests/attack_suite.rs:37:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:55:    let mut fresh_reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:82:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:94:    let hash_a = ResourceRegistry::hash_resource(reg.get(&a).unwrap());
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:39:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:3:use amun_resource_core::{ResourceId, ResourceRegistry};
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:40:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:53:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:61:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:7:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:10:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:54:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:74:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:75:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:33:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:3:use amun_resource_core::{ResourceId, ResourceRegistry};
crates/amun-contract-integration/tests/n172_cross_contract_tests.rs:7:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-upgrade/tests/n174_upgrade_tests.rs:5:use amun_resource_core::ResourceRegistry;
crates/amun-contract-upgrade/tests/n174_upgrade_tests.rs:9:    let mut reg = ResourceRegistry::new(100);
crates/amun-core-optimization/tests/n161_optimization_tests.rs:11:    let mut reg = ResourceRegistry::new(20000);
crates/amun-core-optimization/tests/n161_optimization_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-defi-amm/tests/n153_amm_tests.rs:20:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-defi-amm/tests/n153_amm_tests.rs:21:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-defi-amm/tests/n153_amm_tests.rs:2:use amun_resource_core::ResourceRegistry;
crates/amun-defi-amm/tests/n153_amm_tests.rs:37:    let mut reg = ResourceRegistry::new(100);
crates/amun-defi-amm/tests/n153_amm_tests.rs:6:    let mut reg = ResourceRegistry::new(100);
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:17:    let mut reg = ResourceRegistry::new(10);
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:25:    let mut reg1 = ResourceRegistry::new(10);
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:26:    let mut reg2 = ResourceRegistry::new(10);
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:2:use amun_resource_core::ResourceRegistry;
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:6:    let mut reg = ResourceRegistry::new(10);
crates/amun-experimental-framework/src/main.rs:213:            let mut fresh = ResourceRegistry::new((size * 2) as usize);
crates/amun-experimental-framework/src/main.rs:310:            let mut fresh = ResourceRegistry::new((size * 2) as usize);
crates/amun-experimental-framework/src/main.rs:339:            let mut reg = ResourceRegistry::new((n * 10) as usize);
crates/amun-experimental-framework/src/main.rs:370:                    let mut fresh = ResourceRegistry::new(10000);
crates/amun-experimental-framework/src/main.rs:398:            let tip_hash = ResourceRegistry::hash_resource(tip_meta);
crates/amun-experimental-framework/src/main.rs:66:    let mut reg = ResourceRegistry::new((size * 2) as usize);
crates/amun-experimental-framework/src/main.rs:82:    let mut reg = ResourceRegistry::new((depth * 2) as usize);
crates/amun-experimental-framework/src/main.rs:8:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-experimental-framework/src/main.rs:96:        let hash = ResourceRegistry::hash_resource(reg.get(&parent).unwrap());
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:103:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:124:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:12:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:58:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:84:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:123:    let mut reg = ResourceRegistry::new(20000);
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:17:    let mut reg = ResourceRegistry::new(20000);
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:64:    let mut reg = ResourceRegistry::new(2000);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:109:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:10:    let mut reg = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:110:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:35:    let mut reg = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:61:    let mut reg = ResourceRegistry::new(100);
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:81:    let mut reg = ResourceRegistry::new(100);
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:28:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:46:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:9:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:118:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:141:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:165:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:38:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:62:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:91:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:9:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-mining/tests/n133_mining_tests.rs:18:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-mining/tests/n133_mining_tests.rs:3:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-mining/tests/n133_mining_tests.rs:55:    let mut reg = ResourceRegistry::new(10);
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:17:    ResourceRegistry,
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:22:    let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:5:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:18:    ResourceRegistry,
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:31:    let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-stress/tests/n146_stress_tests.rs:107:        ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-stress/tests/n146_stress_tests.rs:111:    let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:18:    let mut reg = ResourceRegistry::new(10000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:37:    let mut reg = ResourceRegistry::new(1000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-nft-stress/tests/n146_stress_tests.rs:77:    let mut reg1 = ResourceRegistry::new(1000);
crates/amun-nft-stress/tests/n146_stress_tests.rs:78:    let mut reg2 = ResourceRegistry::new(1000);
crates/amun-resource-core/tests/stress_tests.rs:107:                ResourceRegistry::hash_resource(parent),
crates/amun-resource-core/tests/stress_tests.rs:133:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-resource-core/tests/stress_tests.rs:149:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-resource-core/tests/stress_tests.rs:174:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-resource-core/tests/stress_tests.rs:186:                ResourceRegistry::hash_resource(parent),
crates/amun-resource-core/tests/stress_tests.rs:211:        let parent_hash = ResourceRegistry::hash_resource(parent);
crates/amun-resource-core/tests/stress_tests.rs:222:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-resource-core/tests/stress_tests.rs:234:                ResourceRegistry::hash_resource(parent),
crates/amun-resource-core/tests/stress_tests.rs:254:            ResourceRegistry::hash_resource(tip),
crates/amun-resource-core/tests/stress_tests.rs:26:    let mut reg = ResourceRegistry::new(100_000);
crates/amun-resource-core/tests/stress_tests.rs:2:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-resource-core/tests/stress_tests.rs:45:    let mut reg = ResourceRegistry::new(10_000);
crates/amun-resource-core/tests/stress_tests.rs:58:                ResourceRegistry::hash_resource(parent),
crates/amun-resource-core/tests/stress_tests.rs:89:    let mut reg = ResourceRegistry::new(100_000);
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:11:    let mut reg = ResourceRegistry::new(10000);
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:2:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:69:    let mut reg = ResourceRegistry::new(1000);
crates/amun-soak-test/src/lib.rs:13:    pub registry: Arc<Mutex<ResourceRegistry>>,
crates/amun-soak-test/src/lib.rs:23:            registry: Arc::new(Mutex::new(ResourceRegistry::new(5000))),
crates/amun-soak-test/src/lib.rs:4:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-testnet-sim/tests/adversarial_tests.rs:23:    let mut reg = ResourceRegistry::new((count * 2) as usize);
crates/amun-testnet-sim/tests/adversarial_tests.rs:9:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-audit-trail/src/lib.rs:102:        assert!(trail.verify());
crates/amun-audit-trail/src/lib.rs:113:        assert!(!trail.verify());
crates/amun-audit-trail/src/lib.rs:93:        assert!(trail.verify());
crates/amun-authority-registry/src/recovery.rs:163:        assert!(recovered.journal.is_executed(&proposal.proposal_id));
crates/amun-authority-registry/src/registry.rs:275:        assert!(reg.transition.is_some());
crates/amun-authority-registry/src/transaction.rs:160:        assert_eq!(executed.len(), 1);
crates/amun-authority-registry/src/transaction.rs:238:        assert!(restored.journal.is_executed(&proposal.proposal_id));
crates/amun-authority-registry/src/wal.rs:134:        assert!(restored.journal.is_executed(&proposal.proposal_id));
crates/amun-bench/tests/n161_state_root_bench.rs:16:        lineage: ResourceLineage::genesis(col_id),
crates/amun-bench/tests/n161_state_root_bench.rs:32:                lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
crates/amun-bench/tests/n162_snapshot_bench.rs:16:        lineage: ResourceLineage::genesis(col_id),
crates/amun-bench/tests/n162_snapshot_bench.rs:32:                lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
crates/amun-benchmarks/benches/sync_bench.rs:21:            lineage: ResourceLineage::genesis(id),
crates/amun-benchmarks/benches/sync_bench.rs:7:    use amun_resource_core::resource_lineage::ResourceLineage;
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:10:    let sender = sk.verifying_key().to_bytes();
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:24:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:29:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:2:use amun_block_builder::BlockBuilder;
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:39:    let expected_state_root = builder
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:59:    let mut builder1 = BlockBuilder::new();
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:60:    let mut builder2 = BlockBuilder::new();
crates/amun-block-builder/tests/n111_cca_end_to_end.rs:64:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n120_2_slashing_root.rs:2:use amun_block_builder::{Block, BlockBuilder};
crates/amun-block-builder/tests/n120_2_slashing_root.rs:6:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n28_first_economic_block.rs:12:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n28_first_economic_block.rs:1:use amun_block_builder::BlockBuilder;
crates/amun-block-builder/tests/n28_first_economic_block.rs:42:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n28_first_economic_block.rs:47:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n28_first_economic_block.rs:51:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n28_first_economic_block.rs:8:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n28_first_economic_block.rs:90:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n28_first_economic_block.rs:94:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n32_certified_block.rs:14:impl CertifiedBlock {
crates/amun-block-builder/tests/n32_certified_block.rs:151:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n32_certified_block.rs:155:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n32_certified_block.rs:188:        builder
crates/amun-block-builder/tests/n32_certified_block.rs:1:use amun_block_builder::{Block, BlockBuilder};
crates/amun-block-builder/tests/n32_certified_block.rs:25:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n32_certified_block.rs:31:        sk.verifying_key().to_bytes()
crates/amun-block-builder/tests/n32_certified_block.rs:78:    let mut builder = BlockBuilder::new();
crates/amun-block-builder/tests/n32_certified_block.rs:82:        sk.verifying_key().to_bytes()
crates/amun-bytecode/src/lib.rs:27:        assert!(p1.verify());
crates/amun-chain-checkpoint/src/lib.rs:274:        assert!(cert.verify().is_ok());
crates/amun-chain-checkpoint/src/lib.rs:284:        assert!(cert.verify().is_ok());
crates/amun-chain-checkpoint/src/lib.rs:315:        assert!(cert.verify().is_err());
crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:19:        rt.apply_transition(&[height as u8; 32], &[0xCC; 32]);
crates/amun-chain-store/examples/snapshot_builder_test.rs:45:    println!("N99.1 Snapshot Builder Test: PASS");
crates/amun-chain-store/src/record.rs:64:        assert_eq!(decoded.commitment_root, record.commitment_root);
crates/amun-chain-store/tests/n120_2_record_roundtrip.rs:16:        commitment_root: [0u8; 32],
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:20:    assert!(program.verify());
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:25:        lineage: ResourceLineage::genesis(col_id),
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:40:            lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
crates/amun-core-optimization/tests/n161_optimization_tests.rs:21:                lineage: ResourceLineage::genesis(col_id),
crates/amun-core-optimization/tests/n161_optimization_tests.rs:36:            lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
crates/amun-crypto-hardening/src/key_rotation.rs:130:        assert!(cert.verify());
crates/amun-crypto-hardening/src/production_keys.rs:65:        assert!(pubkey.verify(msg, &sig));
crates/amun-crypto-hardening/src/production_keys.rs:75:        assert!(!pubkey.verify(msg, &sig));
crates/amun-crypto-hardening/src/production_keys.rs:83:        assert!(!pubkey.verify(b"message B", &sig));
crates/amun-execution/src/tests.rs:59:    assert!(interpreter.execute(b"", "", &[]).is_ok());
crates/amun-experimental-framework/src/main.rs:104:                lineage: ResourceLineage::single_ancestor(child, parent, hash, version),
crates/amun-experimental-framework/src/main.rs:120:/// Workload B: Push10 — compute overhead.
crates/amun-experimental-framework/src/main.rs:407:                    lineage: ResourceLineage::single_ancestor(new_id, tip, tip_hash, version),
crates/amun-experimental-framework/src/main.rs:72:            lineage: ResourceLineage::genesis(make_id(i)),
crates/amun-experimental-framework/src/main.rs:88:        lineage: ResourceLineage::genesis(root),
crates/amun-finality-law/src/finality.rs:65:        assert!(is_finalized_simple(&qc_a, &qc_b));
crates/amun-finality-law/src/finality.rs:94:        assert!(!is_finalized_simple(&qc_a, &qc_b));
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:98:    // execute another slash. The idempotency protection is at the
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:1:// N118 — Finality-Gated Slashing Verification
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:2:// Verifies that slashing executes in the finality path and reduces stake.
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:34:        "N118: Slashing must execute in finality path"
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:41:    eprintln!("N118 PASSED: Slashing executes in finality-gated path");
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:112:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:135:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:21:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:35:            lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:48:            lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:69:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-adversarial/tests/n149_adversarial_tests.rs:91:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:129:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:12:    hasher.finalize().into()
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:145:                lineage: ResourceLineage::single_ancestor(
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:23:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:40:                lineage: ResourceLineage::single_ancestor(
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:70:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-benchmark/tests/n152_benchmark_tests.rs:87:                lineage: ResourceLineage::single_ancestor(
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:118:        lineage: ResourceLineage::genesis(token_id1),
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:127:        lineage: ResourceLineage::genesis(token_id2),
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:17:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:42:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:68:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:88:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:15:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:34:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:54:        lineage: ResourceLineage::genesis(id1),
crates/amun-nft-explorer/tests/n135_explorer_tests.rs:63:        lineage: ResourceLineage::genesis(id2),
crates/amun-nft-governance-execution/tests/n143_governance_execution_tests.rs:61:    assert!(exec.execute(&id, 50));
crates/amun-nft-integration/tests/n132_integration_tests.rs:29:        [8u8; 32], // commit_hash
crates/amun-nft-integration/tests/n132_integration_tests.rs:40:    assert_eq!(extended.commit_hash, [8u8; 32]);
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:101:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:126:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:149:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:173:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:18:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:46:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-marketplace/tests/n134_marketplace_tests.rs:72:        lineage: ResourceLineage::genesis(token_id),
crates/amun-nft-mining/tests/n133_mining_tests.rs:26:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-mining/tests/n133_mining_tests.rs:62:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:13:    hasher.finalize().into()
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:32:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-persistence/tests/n150_persistence_tests.rs:49:                lineage: ResourceLineage::single_ancestor(
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:103:            lineage: amun_resource_core::ResourceLineage::genesis(ResourceId(token)),
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:56:            lineage: amun_resource_core::ResourceLineage::genesis(ResourceId(token)),
crates/amun-nft-sdk/tests/n145_sdk_tests.rs:79:            lineage: amun_resource_core::ResourceLineage::genesis(ResourceId(token)),
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:14:    hasher.finalize().into()
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:178:            lineage: ResourceLineage::single_ancestor(
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:17:type SnapshotTestState = (
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:42:        lineage: ResourceLineage::genesis(col_id),
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:59:                lineage: ResourceLineage::single_ancestor(
crates/amun-nft-stress/tests/n146_stress_tests.rs:129:        lineage: ResourceLineage::genesis(ResourceId(col_id)),
crates/amun-nft-stress/tests/n146_stress_tests.rs:13:    hasher.finalize().into()
crates/amun-nft-stress/tests/n146_stress_tests.rs:143:            lineage: ResourceLineage::single_ancestor(
crates/amun-nft-stress/tests/n146_stress_tests.rs:24:        lineage: ResourceLineage::genesis(ResourceId(col_id)),
crates/amun-nft-stress/tests/n146_stress_tests.rs:43:        lineage: ResourceLineage::genesis(ResourceId(col_id)),
crates/amun-nft-stress/tests/n146_stress_tests.rs:55:                lineage: ResourceLineage::single_ancestor(
crates/amun-nft-stress/tests/n146_stress_tests.rs:85:            lineage: ResourceLineage::genesis(ResourceId(col_id)),
crates/amun-node/src/genesis.rs:137:        assert!(genesis.validate().is_ok());
crates/amun-node/src/genesis.rs:144:        assert!(genesis.validate().is_err());
crates/amun-node/src/genesis.rs:151:        assert!(genesis.validate().is_err());
crates/amun-node/src/genesis.rs:161:        assert!(genesis.validate().is_err());
crates/amun-node/src/genesis.rs:184:        assert!(genesis.validate().is_err());
crates/amun-node/src/genesis.rs:191:        assert!(genesis.validate().is_err());
crates/amun-node/src/peer_handshake.rs:111:        assert!(handshake.verify(&genesis_hash).is_ok());
crates/amun-node/src/peer_handshake.rs:118:        assert!(handshake.verify(&wrong_genesis).is_err());
crates/amun-node/src/peer_handshake.rs:125:        assert!(handshake.verify(&genesis_hash).is_err());
crates/amun-operations/src/backup_recovery.rs:113:        assert!(backup.verify());
crates/amun-pccv/src/lib.rs:106:        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("R1")));
crates/amun-pccv/src/lib.rs:158:        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("R6")));
crates/amun-pccv/src/lib.rs:210:        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("T1")));
crates/amun-pccv/src/lib.rs:268:        assert!(matches!(result, PCCVResult::Verified { .. }));
crates/amun-pccv/src/lib.rs:51:        assert!(matches!(result, PCCVResult::Verified { .. }));
crates/amun-peer-identity/tests/peer_identity_tests.rs:25:    assert!(IdentityVerifier::verify(&cert, GENESIS).is_ok());
crates/amun-peer-identity/tests/peer_identity_tests.rs:33:    assert!(IdentityVerifier::verify(&cert, GENESIS).is_err());
crates/amun-persistence/src/lib.rs:91:        assert_eq!(restored.last_commit_hash, "jkl012");
crates/amun-persistent-node/src/persistent_store.rs:212:        assert!(backup.verify());
crates/amun-persistent-node/src/persistent_store.rs:214:        assert!(!backup.verify(), "Tampered backup should fail verification");
crates/amun-resource-core/tests/stress_tests.rs:115:            lineage: ResourceLineage::single_ancestor(child_id, parent_id, parent_hash, version),
crates/amun-resource-core/tests/stress_tests.rs:18:        lineage: ResourceLineage::genesis(id),
crates/amun-resource-core/tests/stress_tests.rs:194:            lineage: ResourceLineage::single_ancestor(child_id, parent_id, parent_hash, version),
crates/amun-resource-core/tests/stress_tests.rs:242:            lineage: ResourceLineage::single_ancestor(child_id, parent_id, parent_hash, version),
crates/amun-resource-core/tests/stress_tests.rs:262:        lineage: ResourceLineage::single_ancestor(make_id(99999), parent_id, tip_hash, version),
crates/amun-resource-core/tests/stress_tests.rs:66:            lineage: ResourceLineage::single_ancestor(child_id, parent_id, parent_hash, version),
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:17:        lineage: ResourceLineage::genesis(col_id),
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:33:                lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:75:        lineage: ResourceLineage::genesis(col_id),
crates/amun-snapshot-optimization/tests/n162_snapshot_optimization_tests.rs:91:                lineage: ResourceLineage::single_ancestor(token_id, col_id, parent_hash, version),
crates/amun-state-pruning/tests/n166_pruning_tests.rs:120:                lineage: ResourceLineage::genesis(col_id),
crates/amun-state-pruning/tests/n166_pruning_tests.rs:137:                        lineage: ResourceLineage::single_ancestor(
crates/amun-state-pruning/tests/n166_pruning_tests.rs:16:            lineage: ResourceLineage::genesis(col_id),
crates/amun-state-pruning/tests/n166_pruning_tests.rs:2:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
crates/amun-state-pruning/tests/n166_pruning_tests.rs:33:                    lineage: ResourceLineage::single_ancestor(
crates/amun-state-pruning/tests/n166_pruning_tests.rs:66:            lineage: ResourceLineage::genesis(col_id),
crates/amun-state-pruning/tests/n166_pruning_tests.rs:83:                    lineage: ResourceLineage::single_ancestor(
crates/amun-state-root/src/laws.rs:1:pub const STATE_ROOT_LAW: &str = "assert_eq!(recomputed_root, committed_root)";
crates/amun-state-root/src/snapshot.rs:64:    /// Uses `debug_assert!` to verify integrity in debug/test builds
crates/amun-state-sync/src/lib.rs:104:        assert!(chunk.verify());
crates/amun-state-sync/src/lib.rs:112:        assert!(!chunk.verify());
crates/amun-state-sync/src/lib.rs:166:        assert!(pkg.verify());
crates/amun-state-sync/src/lib.rs:80:        assert!(cert.verify());
crates/amun-state-sync/src/lib.rs:97:        assert!(!cert.verify());
crates/amun-state-types/src/tests.rs:48:    assert_eq!(*committed.inner(), 42);
crates/amun-state-types/src/tests.rs:62:    assert_eq!(*finalized.inner(), 42);
crates/amun-state-types/src/tests.rs:76:    assert_eq!(finalized.into_inner(), 42);
crates/amun-tokenomics-ledger/src/lib.rs:185:        assert_eq!(ledger.issued_supply(), 0);
crates/amun-tokenomics-ledger/tests/test_ledger.rs:71:    assert!(ledger.total_issued_ntr > 0, "Should have issued NTR");
crates/amun-transactions/src/lib.rs:141:        assert!(tx.verify());
crates/amun-transactions/src/lib.rs:148:        assert!(!tx.verify());
crates/amun-transactions/src/lib.rs:158:        assert!(!fake.verify());
crates/amun-verification-kernel/src/lib.rs:192:        assert!(cert.verify());
crates/amun-verification-kernel/src/lib.rs:207:        assert!(!cert.verify());
docs/audit/TRACEABILITY_MATRIX.md:14:| N48.5-W20-21A PCCV | amun-pccv | 11 tests | ✅ |
crates/amun-byzantine-tests/tests/attack_suite.rs:101:            lineage: ResourceLineage::single_ancestor(b, a, hash_a, 2),
crates/amun-byzantine-tests/tests/attack_suite.rs:116:                lineage: ResourceLineage::single_ancestor(c, b, hash_b, 3),
crates/amun-byzantine-tests/tests/attack_suite.rs:132:        lineage: ResourceLineage::genesis(parent_id),
crates/amun-byzantine-tests/tests/attack_suite.rs:144:            lineage: ResourceLineage::single_ancestor(make_id(2), parent_id, parent_hash, 1),
crates/amun-byzantine-tests/tests/attack_suite.rs:160:        lineage: ResourceLineage::genesis(parent_id),
crates/amun-byzantine-tests/tests/attack_suite.rs:172:            lineage: ResourceLineage::single_ancestor(make_id(2), parent_id, forged_hash, 2),
crates/amun-byzantine-tests/tests/attack_suite.rs:188:        lineage: ResourceLineage::genesis(ev),
crates/amun-byzantine-tests/tests/attack_suite.rs:200:            lineage: ResourceLineage::single_ancestor(make_id(2), ev, parent_hash, 2),
crates/amun-byzantine-tests/tests/attack_suite.rs:216:        lineage: ResourceLineage::genesis(root),
crates/amun-byzantine-tests/tests/attack_suite.rs:232:                lineage: ResourceLineage::single_ancestor(child, parent, hash, version),
crates/amun-byzantine-tests/tests/attack_suite.rs:252:            lineage: ResourceLineage::genesis(id),
crates/amun-byzantine-tests/tests/attack_suite.rs:89:        lineage: ResourceLineage::genesis(a),
crates/amun-live-cluster/src/bin/byzantine_test.rs:44:            commitment: None,
crates/amun-node/src/bin/test_multi_byzantine.rs:102:                    let prev = node.committed_blocks.len();
crates/amun-node/src/bin/test_multi_byzantine.rs:104:                    if node.committed_blocks.len() > prev {
crates/amun-node/src/bin/test_multi_byzantine.rs:106:                            let h = node.committed_blocks.len() as u64;
crates/amun-node/src/bin/test_multi_byzantine.rs:112:                                lineage: ResourceLineage::genesis(rid),
crates/amun-node/src/bin/test_multi_byzantine.rs:37:                         target_commits: usize,
crates/amun-node/src/bin/test_multi_byzantine.rs:46:            .map(|n| n.committed_blocks.len())
crates/amun-node/src/bin/test_multi_byzantine.rs:49:            < target_commits
crates/amun-node/src/bin/test_multi_byzantine.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
crates/amun-testnet-sim/tests/adversarial_tests.rs:29:            lineage: ResourceLineage::genesis(make_id(i)),
