1096:crates/amun-constitutional-enforcement/src/lib.rs:252:                assert_eq!(violations[0].law, ConstitutionalLaw::StateRootIntegrity);
123:crates/amun-byzantine-tests/tests/attack_suite.rs:25:    let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
124:crates/amun-byzantine-tests/tests/attack_suite.rs:299:        ConstitutionalEvidence::ConstitutionalViolation { law, .. } => {
125:crates/amun-byzantine-tests/tests/attack_suite.rs:2:use amun_bytecode::program::ConstitutionalProgram;
126:crates/amun-byzantine-tests/tests/attack_suite.rs:38:    let result = ConstitutionalRuntime::execute(
127:crates/amun-byzantine-tests/tests/attack_suite.rs:3:use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
128:crates/amun-byzantine-tests/tests/attack_suite.rs:6:use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
1742:crates/amun-failure/src/tests.rs:83:        assert_eq!(fault, ConstitutionalFault::EquivocationDetected);
1744:crates/amun-failure/src/tests.rs:8:    assert!(ConstitutionalFault::EquivocationDetected.should_halt());
190:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:14:    let mut rt = ConstitutionalStateRuntime::new();
191:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:22:        let merkle_root = hex::encode(ConstitutionalStateRuntime::certificate_merkle_root(&certs));
192:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:24:        let proof = ConstitutionalStateRuntime::prove_certificate_inclusion(&certs, &hash).unwrap();
193:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:32:        let block = ConstitutionalBlock::new(
194:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:7:use amun_constitutional_block::ConstitutionalBlock;
195:crates/amun-chain-checkpoint/tests/n16_adversarial_bootstrap.rs:8:use amun_constitutional_state::ConstitutionalStateRuntime;
21:crates/amun-audit/tests/audit_layer04_byzantine.rs:10:        let identity = ConstitutionalIdentity::new([0x01u8; 32]);
21:crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:4:        ByzantineSyncEngine, ConstitutionalIdentity, PeerManifest, SnapshotManifest, SyncDecision,
22:crates/amun-audit/tests/audit_layer04_byzantine.rs:4:        ByzantineSyncEngine, ConstitutionalIdentity, PeerManifest, SnapshotManifest, SyncDecision,
23:crates/amun-audit/tests/audit_layer04_byzantine.rs:55:        let local = ConstitutionalIdentity::new([0xAAu8; 32]);
2474:crates/amun-replay-engine/src/byzantine_witness_filter.rs:13:use amun_constitutional::ConstitutionalWitness;
2483:crates/amun-replay-engine/src/byzantine_witness_filter.rs:42:pub fn filter_incoming_witness(witness: &ConstitutionalWitness) -> FilterResult {
2488:crates/amun-replay-engine/src/byzantine_witness_filter.rs:87:    fn make_witness(entries: Vec<WitnessEntry>) -> ConstitutionalWitness {
24:crates/amun-audit/tests/audit_layer04_byzantine.rs:56:        let foreign = ConstitutionalIdentity::new([0xBBu8; 32]);
33:crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:10:        let identity = ConstitutionalIdentity::new([0x01u8; 32]);
34:crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:4:        ByzantineSyncEngine, ConstitutionalIdentity, PeerManifest, SnapshotManifest, SyncDecision,
35:crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:69:        let local = ConstitutionalIdentity::new([0xAAu8; 32]);
36:crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:70:        let foreign = ConstitutionalIdentity::new([0xBBu8; 32]);
731:crates/amun-constitutional-enforcement/src/lib.rs:108:                ConstitutionalLaw::SlashingEvidenceBinding,
735:crates/amun-constitutional-enforcement/src/lib.rs:159:                &ConstitutionalLaw::SlashingEvidenceBinding,
746:crates/amun-constitutional-enforcement/src/proof_engine.rs:155:                law: ConstitutionalLaw::SlashingEvidenceBinding,
76:crates/amun-byzantine-tests/tests/attack_suite.rs:299:        ConstitutionalEvidence::ConstitutionalViolation { law, .. } => {
81:crates/amun-byzantine-tests/tests/attack_suite.rs:6:use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
9:crates/amun-audit/tests/audit_layer04_byzantine.rs:4:        ByzantineSyncEngine, ConstitutionalIdentity, PeerManifest, SnapshotManifest, SyncDecision,
crates/amun-attack-lab/src/network.rs:17:    pub fn partitioned() -> Self {
crates/amun-attack-lab/src/network.rs:2:pub struct NetworkConditions {
crates/amun-attack-lab/src/network.rs:8:impl NetworkConditions {
crates/amun-attack-lab/src/network.rs:9:    pub fn normal() -> Self {
crates/amun-attack-lab/src/scenario.rs:12:pub struct AttackScenario {
crates/amun-attack-lab/src/scenario.rs:5:pub enum ExpectedOutcome {
crates/amun-attack-lab/src/simulator.rs:101:            finalized_blocks: Vec::new(),
crates/amun-attack-lab/src/simulator.rs:138:            if state.finalized_blocks.len() >= 10 {
crates/amun-attack-lab/src/simulator.rs:140:                    blocks_finalized: state.finalized_blocks.len() as u64,
crates/amun-attack-lab/src/simulator.rs:151:                state.finalized_blocks.push(FinalizedBlock {
crates/amun-attack-lab/src/simulator.rs:152:                    height: state.finalized_blocks.len() as u64 + 1,
crates/amun-attack-lab/src/simulator.rs:15:pub struct SimulationState {
crates/amun-attack-lab/src/simulator.rs:160:            blocks_finalized: state.finalized_blocks.len() as u64,
crates/amun-attack-lab/src/simulator.rs:165:    fn check_safety(&self, state: &SimulationState) -> Option<String> {
crates/amun-attack-lab/src/simulator.rs:167:        for block in &state.finalized_blocks {
crates/amun-attack-lab/src/simulator.rs:17:    pub finalized_blocks: Vec<FinalizedBlock>,
crates/amun-attack-lab/src/simulator.rs:182:impl Default for AttackSimulator {
crates/amun-attack-lab/src/simulator.rs:183:    fn default() -> Self {
crates/amun-attack-lab/src/simulator.rs:25:pub struct FinalizedBlock {
crates/amun-attack-lab/src/simulator.rs:32:pub struct EquivocationEvent {
crates/amun-attack-lab/src/simulator.rs:38:pub struct LockViolationEvent {
crates/amun-attack-lab/src/simulator.rs:44:pub struct AttackSimulator {
crates/amun-attack-lab/src/simulator.rs:49:pub enum SimulationResult {
crates/amun-attack-lab/src/simulator.rs:51:        blocks_finalized: u64,
crates/amun-attack-lab/src/simulator.rs:60:impl AttackSimulator {
crates/amun-attack-lab/src/simulator.rs:61:    pub fn new() -> Self {
crates/amun-attack-lab/src/simulator.rs:67:    pub fn register_scenario(&mut self, scenario: AttackScenario) -> Result<(), &'static str> {
crates/amun-attack-lab/src/simulator.rs:6:pub struct WeightedValidator {
crates/amun-attack-lab/src/simulator.rs:75:    pub fn scenario_count(&self) -> usize {
crates/amun-attack-lab/src/simulator.rs:79:    pub fn simulate(
crates/amun-attack-lab/src/strategy.rs:2:pub enum ByzantineStrategy {
crates/amun-audit/tests/audit_layer04_byzantine.rs:10:        let identity = ConstitutionalIdentity::new([0x01u8; 32]);
crates/amun-audit/tests/audit_layer04_byzantine.rs:26:        let mut engine = ByzantineSyncEngine::new(identity.clone(), 2);
crates/amun-audit/tests/audit_layer04_byzantine.rs:4:        ByzantineSyncEngine, ConstitutionalIdentity, PeerManifest, SnapshotManifest, SyncDecision,
crates/amun-audit/tests/audit_layer04_byzantine.rs:54:    fn byz002_identity_mismatch_rejection() {
crates/amun-audit/tests/audit_layer04_byzantine.rs:55:        let local = ConstitutionalIdentity::new([0xAAu8; 32]);
crates/amun-audit/tests/audit_layer04_byzantine.rs:56:        let foreign = ConstitutionalIdentity::new([0xBBu8; 32]);
crates/amun-audit/tests/audit_layer04_byzantine.rs:57:        let mut engine = ByzantineSyncEngine::new(local.clone(), 1);
crates/amun-audit/tests/audit_layer04_byzantine.rs:9:    fn byz001_quorum_detection() {
crates/amun-audit/tests/audit_layer10_adversarial.rs:37:    // CONST-ADV-002: Malformed proof must not verify
crates/amun-audit/tests/audit_layer10_adversarial.rs:39:    fn adv002_malformed_proof_rejection() {
crates/amun-audit/tests/audit_layer10_adversarial.rs:48:            proof.verify(root.0),
crates/amun-audit/tests/audit_layer10_adversarial.rs:49:            "CONST-ADV-002: Valid proof must verify"
crates/amun-audit/tests/audit_layer10_adversarial.rs:53:            !proof.verify(wrong_root),
crates/amun-audit/tests/audit_layer10_adversarial.rs:54:            "CONST-ADV-002 VIOLATION: Proof verified against wrong root"
crates/amun-audit/tests/audit_layer10_adversarial.rs:60:    fn adv003_delete_nonexistent_noop() {
crates/amun-audit/tests/audit_layer10_adversarial.rs:76:    fn adv004_insert_delete_insert_cycle() {
crates/amun-audit/tests/audit_layer10_adversarial.rs:8:    fn adv001_random_order_independence() {
crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:10:        let identity = ConstitutionalIdentity::new([0x01u8; 32]);
crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:38:        let mut engine = ByzantineSyncEngine::new(identity.clone(), 2);
crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:4:        ByzantineSyncEngine, ConstitutionalIdentity, PeerManifest, SnapshotManifest, SyncDecision,
crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:68:    fn mesh002_foreign_civilization_rejection() {
crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:69:        let local = ConstitutionalIdentity::new([0xAAu8; 32]);
crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:70:        let foreign = ConstitutionalIdentity::new([0xBBu8; 32]);
crates/amun-audit/tests/audit_layer14_byzantine_mesh.rs:9:    fn mesh001_conflicting_manifests_detected() {
crates/amun-block-builder/src/lib.rs:29:    pub fn verify_slashing_certificates(&self) -> Result<(), String> {
crates/amun-block-builder/src/lib.rs:52:    pub fn verify_slashing_root(&self, expected_root: &[u8; 32]) -> Result<(), String> {
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
crates/amun-byzantine-harness/src/harness.rs:10:    pub fn check_uniqueness(votes: &[ConsensusVote]) -> bool {
crates/amun-byzantine-harness/src/harness.rs:15:    pub fn detect_equivocation(votes: &[ConsensusVote]) -> Vec<EquivocationEvidence> {
crates/amun-byzantine-harness/src/harness.rs:20:    pub fn create_vote(
crates/amun-byzantine-harness/src/harness.rs:3:use amun_safety_laws::{check_vote_uniqueness, detect_equivocation, EquivocationEvidence};
crates/amun-byzantine-harness/src/harness.rs:6:pub struct ByzantineHarness;
crates/amun-byzantine-harness/src/harness.rs:8:impl ByzantineHarness {
crates/amun-byzantine-harness/src/scenarios.rs:11:impl ByzantineScenario {
crates/amun-byzantine-harness/src/scenarios.rs:12:    pub fn equivocation_attack() -> Self {
crates/amun-byzantine-harness/src/scenarios.rs:23:    pub fn triple_equivocation() -> Self {
crates/amun-byzantine-harness/src/scenarios.rs:2:pub struct ByzantineScenario {
crates/amun-byzantine-harness/src/scenarios.rs:34:    pub fn honest() -> Self {
crates/amun-byzantine-tests/tests/attack_suite.rs:101:            lineage: ResourceLineage::single_ancestor(b, a, hash_a, 2),
crates/amun-byzantine-tests/tests/attack_suite.rs:107:    let hash_b = ResourceRegistry::hash_resource(reg.get(&b).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:116:                lineage: ResourceLineage::single_ancestor(c, b, hash_b, 3),
crates/amun-byzantine-tests/tests/attack_suite.rs:11:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-byzantine-tests/tests/attack_suite.rs:125:fn byz_004_version_regression_rejected() {
crates/amun-byzantine-tests/tests/attack_suite.rs:126:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:132:        lineage: ResourceLineage::genesis(parent_id),
crates/amun-byzantine-tests/tests/attack_suite.rs:137:    let parent_hash = ResourceRegistry::hash_resource(reg.get(&parent_id).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:144:            lineage: ResourceLineage::single_ancestor(make_id(2), parent_id, parent_hash, 1),
crates/amun-byzantine-tests/tests/attack_suite.rs:14:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-byzantine-tests/tests/attack_suite.rs:153:fn byz_005_parent_hash_forgery_rejected() {
crates/amun-byzantine-tests/tests/attack_suite.rs:154:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:160:        lineage: ResourceLineage::genesis(parent_id),
crates/amun-byzantine-tests/tests/attack_suite.rs:172:            lineage: ResourceLineage::single_ancestor(make_id(2), parent_id, forged_hash, 2),
crates/amun-byzantine-tests/tests/attack_suite.rs:17:fn make_id(seed: u64) -> ResourceId {
crates/amun-byzantine-tests/tests/attack_suite.rs:181:fn byz_006_illegal_transformation_rejected() {
crates/amun-byzantine-tests/tests/attack_suite.rs:182:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:186:        archetype: ResourceArchetype::Evidence,
crates/amun-byzantine-tests/tests/attack_suite.rs:188:        lineage: ResourceLineage::genesis(ev),
crates/amun-byzantine-tests/tests/attack_suite.rs:193:    let parent_hash = ResourceRegistry::hash_resource(reg.get(&ev).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:200:            lineage: ResourceLineage::single_ancestor(make_id(2), ev, parent_hash, 2),
crates/amun-byzantine-tests/tests/attack_suite.rs:209:fn byz_007_deep_lineage_no_crash() {
crates/amun-byzantine-tests/tests/attack_suite.rs:210:    let mut reg = ResourceRegistry::new(100_000);
crates/amun-byzantine-tests/tests/attack_suite.rs:216:        lineage: ResourceLineage::genesis(root),
crates/amun-byzantine-tests/tests/attack_suite.rs:224:        let hash = ResourceRegistry::hash_resource(reg.get(&parent).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:232:                lineage: ResourceLineage::single_ancestor(child, parent, hash, version),
crates/amun-byzantine-tests/tests/attack_suite.rs:244:fn byz_008_wide_fanout_no_crash() {
crates/amun-byzantine-tests/tests/attack_suite.rs:245:    let mut reg = ResourceRegistry::new(200_000);
crates/amun-byzantine-tests/tests/attack_suite.rs:24:fn byz_001_forged_proof_rejected() {
crates/amun-byzantine-tests/tests/attack_suite.rs:252:            lineage: ResourceLineage::genesis(id),
crates/amun-byzantine-tests/tests/attack_suite.rs:25:    let program = ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt]);
crates/amun-byzantine-tests/tests/attack_suite.rs:262:fn byz_009_proof_tampering_detected() {
crates/amun-byzantine-tests/tests/attack_suite.rs:263:    let proof = TransitionProof::new(
crates/amun-byzantine-tests/tests/attack_suite.rs:276:    assert!(proof.verify_integrity());
crates/amun-byzantine-tests/tests/attack_suite.rs:279:    assert!(!tampered.verify_integrity());
crates/amun-byzantine-tests/tests/attack_suite.rs:283:fn byz_010_proof_replay_attack_blocked() {
crates/amun-byzantine-tests/tests/attack_suite.rs:284:    let mut registry = TransferProofRegistry::new();
crates/amun-byzantine-tests/tests/attack_suite.rs:285:    let proof = CrossContractTransferProof::new(
crates/amun-byzantine-tests/tests/attack_suite.rs:299:        ConstitutionalEvidence::ConstitutionalViolation { law, .. } => {
crates/amun-byzantine-tests/tests/attack_suite.rs:2:use amun_bytecode::program::ConstitutionalProgram;
crates/amun-byzantine-tests/tests/attack_suite.rs:35:    let mut hot = HotProofStore::new(100);
crates/amun-byzantine-tests/tests/attack_suite.rs:36:    let mut archive = ProofArchive::new();
crates/amun-byzantine-tests/tests/attack_suite.rs:37:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:38:    let result = ConstitutionalRuntime::execute(
crates/amun-byzantine-tests/tests/attack_suite.rs:3:use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
crates/amun-byzantine-tests/tests/attack_suite.rs:4:use amun_cross_contract::transfer_proof::CrossContractTransferProof;
crates/amun-byzantine-tests/tests/attack_suite.rs:50:            transition_proof, ..
crates/amun-byzantine-tests/tests/attack_suite.rs:51:        } => transition_proof,
crates/amun-byzantine-tests/tests/attack_suite.rs:55:    let mut fresh_reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:56:    let replay = ReplayVerifier::replay(&proof, &program, &mut fresh_reg, &[]);
crates/amun-byzantine-tests/tests/attack_suite.rs:58:        replay,
crates/amun-byzantine-tests/tests/attack_suite.rs:59:        amun_replay_verifier::replay_verifier::ReplayResult::Match { .. }
crates/amun-byzantine-tests/tests/attack_suite.rs:5:use amun_cross_contract::transfer_registry::TransferProofRegistry;
crates/amun-byzantine-tests/tests/attack_suite.rs:64:fn byz_002_double_transfer_rejected() {
crates/amun-byzantine-tests/tests/attack_suite.rs:65:    let mut registry = TransferProofRegistry::new();
crates/amun-byzantine-tests/tests/attack_suite.rs:66:    let proof = CrossContractTransferProof::new(
crates/amun-byzantine-tests/tests/attack_suite.rs:6:use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
crates/amun-byzantine-tests/tests/attack_suite.rs:7:use amun_proof_archive::hot_store::HotProofStore;
crates/amun-byzantine-tests/tests/attack_suite.rs:81:fn byz_003_lineage_cycle_rejected() {
crates/amun-byzantine-tests/tests/attack_suite.rs:82:    let mut reg = ResourceRegistry::new(1000);
crates/amun-byzantine-tests/tests/attack_suite.rs:89:        lineage: ResourceLineage::genesis(a),
crates/amun-byzantine-tests/tests/attack_suite.rs:8:use amun_proof_archive::proof_archive::ProofArchive;
crates/amun-byzantine-tests/tests/attack_suite.rs:94:    let hash_a = ResourceRegistry::hash_resource(reg.get(&a).unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:9:use amun_replay_verifier::replay_verifier::ReplayVerifier;
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
crates/amun-chain-store/tests/n120_2_record_roundtrip.rs:4:fn n120_2_record_roundtrip_preserves_slashing_root() {
crates/amun-consensus-execution/src/voter.rs:160:    pub fn get_evidence(&self) -> &[EquivocationProof] {
crates/amun-consensus-execution/src/voter.rs:22:    evidence: Vec<EquivocationProof>,
crates/amun-consensus-execution/src/voter.rs:53:                let proof = EquivocationProof {
crates/amun-consensus-execution/src/voter.rs:8:pub struct EquivocationProof {
crates/amun-consensus-integration/src/consensus_types.rs:35:            if !proof.verify_integrity() {
crates/amun-consensus-law/src/validator.rs:34:    pub fn slashable_offense(equivocation_proven: bool, downtime_exceeded: bool) -> bool {
crates/amun-consensus-network/src/certificate_evidence_validation.rs:86:    use crate::slashing_certificate::{EvidenceCount, SlashingCertificate};
crates/amun-consensus-network/src/certificate_evidence_validation.rs:88:    fn make_cert_with_evidence_ids(ids: Vec<[u8; 32]>) -> SlashingCertificate {
crates/amun-consensus-network/src/certificate_gossip.rs:25:    pub fn receive_certificate(&mut self, cert: SlashingCertificate) -> Result<bool, String> {
crates/amun-consensus-network/src/certificate_gossip.rs:44:    pub fn get_pending(&self) -> Vec<&SlashingCertificate> {
crates/amun-consensus-network/src/certificate_gossip.rs:74:    use crate::slashing_certificate::{EvidenceCount, SlashingCertificate};
crates/amun-consensus-network/src/certificate_gossip.rs:76:    fn make_test_cert(id: [u8; 32]) -> SlashingCertificate {
crates/amun-consensus-network/src/engine.rs:228:/// Evidence of equivocation — same validator voted for two different blocks at the same height.
crates/amun-consensus-network/src/engine.rs:2:    ConsensusVote, EquivocationProof, FinalityCertificate, QuorumCertificate, SignedVote,
crates/amun-consensus-network/src/engine.rs:416:                let proof = EquivocationProof {
crates/amun-consensus-network/src/engine.rs:670:    fn n68_byzantine_wrong_height_rejected() {
crates/amun-consensus-network/src/execution_commitment.rs:18:// This is the foundation for N110 Slashing and N111 Evidence.
crates/amun-consensus-network/src/finality_gate.rs:1:// N118.1 — Finality Gate for Slashing Certificates
crates/amun-consensus-network/src/finality_gate.rs:40:    use crate::slashing_certificate::{EvidenceCount, SlashingCertificate};
crates/amun-consensus-network/src/finality_gate.rs:43:    fn make_certificate(height: u64) -> SlashingCertificate {
crates/amun-consensus-network/src/integrated_slashing.rs:124:    fn calculate_penalty(&self, score: u64) -> u64 {
crates/amun-consensus-network/src/integrated_slashing.rs:12://   EvidenceStore::store_evidence()
crates/amun-consensus-network/src/integrated_slashing.rs:139:    pub fn get_penalty(&self, validator_id: &[u8; 32]) -> Option<u64> {
crates/amun-consensus-network/src/integrated_slashing.rs:150:    pub fn mark_slashed(&mut self, evidence_id: &[u8; 32]) -> bool {
crates/amun-consensus-network/src/integrated_slashing.rs:152:            .update_status(evidence_id, EvidenceStatus::Slashed)
crates/amun-consensus-network/src/integrated_slashing.rs:156:impl Default for IntegratedSlashingPipeline {
crates/amun-consensus-network/src/integrated_slashing.rs:157:    fn default() -> Self {
crates/amun-consensus-network/src/integrated_slashing.rs:165:    use crate::evidence_store::EvidenceType;
crates/amun-consensus-network/src/integrated_slashing.rs:168:    /// N109.12 GATEKEEPER: Evidence triggers the full slashing pipeline
crates/amun-consensus-network/src/integrated_slashing.rs:170:    fn n109_12_evidence_triggers_slashing_pipeline() {
crates/amun-consensus-network/src/integrated_slashing.rs:180:                EvidenceType::StateRootMismatch,
crates/amun-consensus-network/src/integrated_slashing.rs:18://   SlashingEngine::process() → penalty_bps
crates/amun-consensus-network/src/integrated_slashing.rs:216:            EvidenceType::DoubleVote,
crates/amun-consensus-network/src/integrated_slashing.rs:230:            EvidenceType::DoubleVote,
crates/amun-consensus-network/src/integrated_slashing.rs:23:use crate::evidence_store::{EvidenceRecord, EvidenceStatus, EvidenceStore, EvidenceType};
crates/amun-consensus-network/src/integrated_slashing.rs:255:    fn n109_12_duplicate_evidence_ignored() {
crates/amun-consensus-network/src/integrated_slashing.rs:262:            EvidenceType::InvalidSignature,
crates/amun-consensus-network/src/integrated_slashing.rs:271:            EvidenceType::InvalidSignature,
crates/amun-consensus-network/src/integrated_slashing.rs:280:    /// N109.12: EvidenceStore records are accessible after pipeline processing
crates/amun-consensus-network/src/integrated_slashing.rs:282:    fn n109_12_evidence_store_accessible() {
crates/amun-consensus-network/src/integrated_slashing.rs:288:            EvidenceType::VoteBindingViolation,
crates/amun-consensus-network/src/integrated_slashing.rs:28:pub enum PipelineResult {
crates/amun-consensus-network/src/integrated_slashing.rs:296:            EvidenceType::VoteBindingViolation
crates/amun-consensus-network/src/integrated_slashing.rs:29:    /// Evidence was new and stored
crates/amun-consensus-network/src/integrated_slashing.rs:302:    fn n109_12_different_validators_isolated() {
crates/amun-consensus-network/src/integrated_slashing.rs:306:        pipeline.process_violation(&[1u8; 32], 1, EvidenceType::DoubleVote, "dv"); // +10
crates/amun-consensus-network/src/integrated_slashing.rs:30:    EvidenceStored,
crates/amun-consensus-network/src/integrated_slashing.rs:31:    /// Evidence was a duplicate, not counted
crates/amun-consensus-network/src/integrated_slashing.rs:323:    fn n109_12_custom_thresholds_pipeline() {
crates/amun-consensus-network/src/integrated_slashing.rs:334:        let r1 = pipeline.process_violation(&[1u8; 32], 1, EvidenceType::DoubleVote, "dv");
crates/amun-consensus-network/src/integrated_slashing.rs:338:        let r2 = pipeline.process_violation(&[1u8; 32], 2, EvidenceType::DoubleVote, "dv2");
crates/amun-consensus-network/src/integrated_slashing.rs:344:    fn n109_12_mark_slashed_updates_status() {
crates/amun-consensus-network/src/integrated_slashing.rs:347:        pipeline.process_violation(&[0x42; 32], 1, EvidenceType::DoubleVote, "dv");
crates/amun-consensus-network/src/integrated_slashing.rs:355:        assert_eq!(updated.status, EvidenceStatus::Slashed);
crates/amun-consensus-network/src/integrated_slashing.rs:44:pub struct IntegratedSlashingPipeline {
crates/amun-consensus-network/src/integrated_slashing.rs:45:    pub evidence_store: EvidenceStore,
crates/amun-consensus-network/src/integrated_slashing.rs:4:// Connects EvidenceStore → MisbehaviorRegistry(N109.11) → SlashingEngine → Stake Penalty
crates/amun-consensus-network/src/integrated_slashing.rs:53:impl IntegratedSlashingPipeline {
crates/amun-consensus-network/src/integrated_slashing.rs:54:    pub fn new(thresholds: MisbehaviorThresholds) -> Self {
crates/amun-consensus-network/src/integrated_slashing.rs:56:            evidence_store: EvidenceStore::new(),
crates/amun-consensus-network/src/integrated_slashing.rs:65:    /// This is the main entry point. Every time verify_vote_binding fails,
crates/amun-consensus-network/src/integrated_slashing.rs:69:    pub fn process_violation(
crates/amun-consensus-network/src/integrated_slashing.rs:6:// This is the bridge between N109 (Evidence + Misbehavior) and
crates/amun-consensus-network/src/integrated_slashing.rs:73:        evidence_type: EvidenceType,
crates/amun-consensus-network/src/integrated_slashing.rs:82:        let evidence = EvidenceRecord::new(
crates/amun-consensus-network/src/integrated_slashing.rs:90:        // Step 2: Store in EvidenceStore (dedup)
crates/amun-consensus-network/src/lib.rs:30:pub mod slashing_certificate_builder;
crates/amun-consensus-network/src/lib.rs:62:pub use slashing_certificate::{CertificateResultingStatus, EvidenceCount, SlashingCertificate};
crates/amun-consensus-network/src/lib.rs:63:pub use slashing_certificate_builder::SlashingCertificateBuilder;
crates/amun-consensus-network/src/lib.rs:64:pub use slashing_fraud_proof::SlashingFraudProof;
crates/amun-consensus-network/src/lib.rs:65:pub use slashing_inclusion_proof::{build_inclusion_proof, SlashingInclusionProof};
crates/amun-consensus-network/src/messages.rs:197:    fn n101_2_valid_equivocation_proof_accepted() {
crates/amun-consensus-network/src/messages.rs:202:        let proof = EquivocationProof {
crates/amun-consensus-network/src/messages.rs:262:        let proof = EquivocationProof {
crates/amun-consensus-network/src/messages.rs:275:        let proof = EquivocationProof {
crates/amun-consensus-network/src/messages.rs:288:        let proof = EquivocationProof {
crates/amun-consensus-network/src/messages.rs:303:        let proof = EquivocationProof {
crates/amun-consensus-network/src/messages.rs:316:        let proof = EquivocationProof {
crates/amun-consensus-network/src/messages.rs:34:pub struct EquivocationProof {
crates/amun-consensus-network/src/messages.rs:43:impl EquivocationProof {
crates/amun-consensus-network/src/metrics.rs:8:    ///   - The proposer executed incorrectly (bug or byzantine)
crates/amun-consensus-network/src/misbehavior.rs:151:        let mut proof = EquivocationProof {
crates/amun-consensus-network/src/misbehavior.rs:170:        let proof = EquivocationProof {
crates/amun-consensus-network/src/misbehavior.rs:185:        let mut proof1 = EquivocationProof {
crates/amun-consensus-network/src/misbehavior.rs:197:        let mut proof2 = EquivocationProof {
crates/amun-consensus-network/src/misbehavior.rs:1:use crate::messages::EquivocationProof;
crates/amun-consensus-network/src/misbehavior.rs:20:    pub proof: EquivocationProof,
crates/amun-consensus-network/src/misbehavior.rs:218:        let mut proof = EquivocationProof {
crates/amun-consensus-network/src/misbehavior.rs:236:        let mut proof = EquivocationProof {
crates/amun-consensus-network/src/misbehavior.rs:248:        let mut proof2 = EquivocationProof {
crates/amun-consensus-network/src/misbehavior.rs:271:        let mut proof = EquivocationProof {
crates/amun-consensus-network/src/misbehavior.rs:64:    pub fn hash_proof(proof: &EquivocationProof) -> [u8; 32] {
crates/amun-consensus-network/src/misbehavior.rs:69:    pub fn add_proof(&mut self, proof: EquivocationProof) -> Result<[u8; 32], String> {
crates/amun-consensus-network/src/misbehavior_registry.rs:320:    fn n109_11_slashing_threshold_triggered() {
crates/amun-consensus-network/src/misbehavior_registry.rs:398:        reg.record_misbehavior(&[1u8; 32], &[0x03; 32], &EvidenceType::StateRootMismatch, 3); // +3 → 8 → slashing
crates/amun-consensus-network/src/multi_signer_certificate.rs:91:    use crate::slashing_certificate::{EvidenceCount, SlashingCertificate};
crates/amun-consensus-network/src/multi_signer_certificate.rs:95:    fn make_certificate() -> SlashingCertificate {
crates/amun-consensus-network/src/real_staking_adapter.rs:51:impl SlashingExecutor for RealStakingExecutor {
crates/amun-consensus-network/src/slashing.rs:108:    fn n109_13_mixed_offenses_accumulate() {
crates/amun-consensus-network/src/slashing.rs:111:        registry.record_misbehavior(&[1u8; 32], &[0xD1; 32], &EvidenceType::InvalidSignature, 1); // +2
crates/amun-consensus-network/src/slashing.rs:112:        registry.record_misbehavior(&[1u8; 32], &[0xD2; 32], &EvidenceType::StateRootMismatch, 2); // +3
crates/amun-consensus-network/src/slashing.rs:116:            &EvidenceType::VoteBindingViolation,
crates/amun-consensus-network/src/slashing.rs:119:        registry.record_misbehavior(&[1u8; 32], &[0xD4; 32], &EvidenceType::FutureVote, 4); // +1
crates/amun-consensus-network/src/slashing.rs:126:    fn n109_13_all_slashable_offenses_flow_through_unified_api() {
crates/amun-consensus-network/src/slashing.rs:131:            (EvidenceType::DoubleVote, 10),
crates/amun-consensus-network/src/slashing.rs:132:            (EvidenceType::StateRootMismatch, 13),
crates/amun-consensus-network/src/slashing.rs:133:            (EvidenceType::InvalidSignature, 2),
crates/amun-consensus-network/src/slashing.rs:134:            (EvidenceType::VoteBindingViolation, 2),
crates/amun-consensus-network/src/slashing.rs:135:            (EvidenceType::ExecutionFailure, 3),
crates/amun-consensus-network/src/slashing.rs:136:            (EvidenceType::FutureVote, 1),
crates/amun-consensus-network/src/slashing.rs:157:        registry.record_misbehavior(&[0x42; 32], &[0xFF; 32], &EvidenceType::DoubleVote, 200);
crates/amun-consensus-network/src/slashing.rs:16:pub fn should_slash(registry: &MisbehaviorRegistry, validator_id: &[u8; 32]) -> bool {
crates/amun-consensus-network/src/slashing.rs:22:pub fn slash_percentage(registry: &MisbehaviorRegistry, validator_id: &[u8; 32]) -> u8 {
crates/amun-consensus-network/src/slashing.rs:33:pub fn validator_status(
crates/amun-consensus-network/src/slashing.rs:41:pub fn misbehavior_score(registry: &MisbehaviorRegistry, validator_id: &[u8; 32]) -> u64 {
crates/amun-consensus-network/src/slashing.rs:48:    use crate::evidence_store::EvidenceType;
crates/amun-consensus-network/src/slashing.rs:51:    fn make_registry() -> MisbehaviorRegistry {
crates/amun-consensus-network/src/slashing.rs:56:    fn n109_13_no_offenses_no_slash() {
crates/amun-consensus-network/src/slashing.rs:64:    fn n109_13_single_offense_below_threshold() {
crates/amun-consensus-network/src/slashing.rs:70:            &EvidenceType::StateRootMismatch,
crates/amun-consensus-network/src/slashing.rs:82:    fn n109_13_two_offenses_triggers_warning() {
crates/amun-consensus-network/src/slashing.rs:85:        registry.record_misbehavior(&[1u8; 32], &[0xB1; 32], &EvidenceType::DoubleVote, 10);
crates/amun-consensus-network/src/slashing.rs:86:        registry.record_misbehavior(&[1u8; 32], &[0xB2; 32], &EvidenceType::DoubleVote, 20);
crates/amun-consensus-network/src/slashing.rs:93:    fn n109_13_three_offenses_triggers_slash() {
crates/amun-consensus-network/src/slashing.rs:96:        registry.record_misbehavior(&[1u8; 32], &[0xC1; 32], &EvidenceType::DoubleVote, 10);
crates/amun-consensus-network/src/slashing.rs:97:        registry.record_misbehavior(&[1u8; 32], &[0xC2; 32], &EvidenceType::DoubleVote, 20);
crates/amun-consensus-network/src/slashing.rs:98:        registry.record_misbehavior(&[1u8; 32], &[0xC3; 32], &EvidenceType::DoubleVote, 30);
crates/amun-consensus-network/src/slashing_certificate.rs:102:        data.extend_from_slice(&self.executed_at_height.to_le_bytes());
crates/amun-consensus-network/src/slashing_certificate.rs:10:// can independently verify that the slashing was justified.
crates/amun-consensus-network/src/slashing_certificate.rs:112:    pub fn verify_signature(&self) -> Result<(), String> {
crates/amun-consensus-network/src/slashing_certificate.rs:113:        use ed25519_dalek::Verifier;
crates/amun-consensus-network/src/slashing_certificate.rs:117:        let verifying_key = ed25519_dalek::VerifyingKey::from_bytes(&self.signer_public_key)
crates/amun-consensus-network/src/slashing_certificate.rs:121:        verifying_key
crates/amun-consensus-network/src/slashing_certificate.rs:122:            .verify(&payload, &sig)
crates/amun-consensus-network/src/slashing_certificate.rs:128:    pub fn sign(&mut self, signing_key: &ed25519_dalek::SigningKey) {
crates/amun-consensus-network/src/slashing_certificate.rs:130:        self.signer_public_key = signing_key.verifying_key().to_bytes();
crates/amun-consensus-network/src/slashing_certificate.rs:136:    pub fn compute_hash(&self) -> [u8; 32] {
crates/amun-consensus-network/src/slashing_certificate.rs:147:        hasher.finalize().into()
crates/amun-consensus-network/src/slashing_certificate.rs:152:    /// For a more ergonomic construction, use SlashingCertificateBuilder.
crates/amun-consensus-network/src/slashing_certificate.rs:154:    pub fn from_slash_result(
crates/amun-consensus-network/src/slashing_certificate.rs:158:        evidence_summary: Vec<EvidenceCount>,
crates/amun-consensus-network/src/slashing_certificate.rs:164:        executed_at_height: u64,
crates/amun-consensus-network/src/slashing_certificate.rs:16:use crate::evidence_store::EvidenceType;
crates/amun-consensus-network/src/slashing_certificate.rs:192:            executed_at_height,
crates/amun-consensus-network/src/slashing_certificate.rs:199:        cert.certificate_hash = cert.compute_hash();
crates/amun-consensus-network/src/slashing_certificate.rs:204:    pub fn verify(&self) -> Result<(), String> {
crates/amun-consensus-network/src/slashing_certificate.rs:205:        let recomputed = self.compute_hash();
crates/amun-consensus-network/src/slashing_certificate.rs:206:        if recomputed != self.certificate_hash {
crates/amun-consensus-network/src/slashing_certificate.rs:215:        // N114.2: If signed, verify the signature
crates/amun-consensus-network/src/slashing_certificate.rs:217:            self.verify_signature()?;
crates/amun-consensus-network/src/slashing_certificate.rs:228:    fn n110_2_certificate_roundtrip() {
crates/amun-consensus-network/src/slashing_certificate.rs:22:/// Contains all information needed for any third party to verify:
crates/amun-consensus-network/src/slashing_certificate.rs:233:            vec![EvidenceCount {
crates/amun-consensus-network/src/slashing_certificate.rs:234:                evidence_type: EvidenceType::DoubleVote,
crates/amun-consensus-network/src/slashing_certificate.rs:253:        assert!(decoded.verify().is_ok());
crates/amun-consensus-network/src/slashing_certificate.rs:257:    fn n110_2_certificate_hash_is_deterministic() {
crates/amun-consensus-network/src/slashing_certificate.rs:26:///   - PROOF: evidence_ids that can be looked up in EvidenceStore
crates/amun-consensus-network/src/slashing_certificate.rs:284:        assert_eq!(cert1.compute_hash(), cert1.certificate_hash);
crates/amun-consensus-network/src/slashing_certificate.rs:285:        assert_eq!(cert2.compute_hash(), cert2.certificate_hash);
crates/amun-consensus-network/src/slashing_certificate.rs:289:    fn n110_2_different_validators_different_hash() {
crates/amun-consensus-network/src/slashing_certificate.rs:29:pub struct SlashingCertificate {
crates/amun-consensus-network/src/slashing_certificate.rs:318:    fn n110_2_verify_rejects_tampered_amount() {
crates/amun-consensus-network/src/slashing_certificate.rs:332:        assert!(cert.verify().is_err());
crates/amun-consensus-network/src/slashing_certificate.rs:38:    pub evidence_summary: Vec<EvidenceCount>,
crates/amun-consensus-network/src/slashing_certificate.rs:40:    /// Evidence IDs that can be independently verified
crates/amun-consensus-network/src/slashing_certificate.rs:66:    /// Height at which slashing was executed
crates/amun-consensus-network/src/slashing_certificate.rs:67:    pub executed_at_height: u64,
crates/amun-consensus-network/src/slashing_certificate.rs:79:pub struct EvidenceCount {
crates/amun-consensus-network/src/slashing_certificate.rs:7:// Unlike FinalityCertificate (produced by Quorum), SlashingCertificate is
crates/amun-consensus-network/src/slashing_certificate.rs:80:    pub evidence_type: EvidenceType,
crates/amun-consensus-network/src/slashing_certificate.rs:87:pub enum CertificateResultingStatus {
crates/amun-consensus-network/src/slashing_certificate.rs:93:impl SlashingCertificate {
crates/amun-consensus-network/src/slashing_certificate.rs:95:    pub fn signing_bytes(&self) -> Vec<u8> {
crates/amun-consensus-network/src/slashing_certificate_builder.rs:14:    executed_at_height: Option<u64>,
crates/amun-consensus-network/src/slashing_certificate_builder.rs:17:impl SlashingCertificateBuilder {
crates/amun-consensus-network/src/slashing_certificate_builder.rs:18:    pub fn new() -> Self {
crates/amun-consensus-network/src/slashing_certificate_builder.rs:29:            executed_at_height: None,
crates/amun-consensus-network/src/slashing_certificate_builder.rs:2:use crate::slashing_certificate::{EvidenceCount, SlashingCertificate};
crates/amun-consensus-network/src/slashing_certificate_builder.rs:33:    pub fn validator_id(mut self, v: [u8; 32]) -> Self {
crates/amun-consensus-network/src/slashing_certificate_builder.rs:37:    pub fn score(mut self, v: u64) -> Self {
crates/amun-consensus-network/src/slashing_certificate_builder.rs:41:    pub fn evidence_ids(mut self, v: Vec<[u8; 32]>) -> Self {
crates/amun-consensus-network/src/slashing_certificate_builder.rs:45:    pub fn evidence_summary(mut self, v: Vec<EvidenceCount>) -> Self {
crates/amun-consensus-network/src/slashing_certificate_builder.rs:49:    pub fn penalty_bps(mut self, v: u64) -> Self {
crates/amun-consensus-network/src/slashing_certificate_builder.rs:4:pub struct SlashingCertificateBuilder {
crates/amun-consensus-network/src/slashing_certificate_builder.rs:53:    pub fn amount_slashed(mut self, v: u64) -> Self {
crates/amun-consensus-network/src/slashing_certificate_builder.rs:57:    pub fn remaining_stake(mut self, v: u64) -> Self {
crates/amun-consensus-network/src/slashing_certificate_builder.rs:61:    pub fn offense_count(mut self, v: u32) -> Self {
crates/amun-consensus-network/src/slashing_certificate_builder.rs:65:    pub fn status(mut self, v: ValidatorStatus) -> Self {
crates/amun-consensus-network/src/slashing_certificate_builder.rs:69:    pub fn executed_at_height(mut self, v: u64) -> Self {
crates/amun-consensus-network/src/slashing_certificate_builder.rs:70:        self.executed_at_height = Some(v);
crates/amun-consensus-network/src/slashing_certificate_builder.rs:74:    pub fn build(self) -> SlashingCertificate {
crates/amun-consensus-network/src/slashing_certificate_builder.rs:85:            self.executed_at_height.unwrap(),
crates/amun-consensus-network/src/slashing_certificate_builder.rs:8:    evidence_summary: Option<Vec<EvidenceCount>>,
crates/amun-consensus-network/src/slashing_certificate_builder.rs:90:impl Default for SlashingCertificateBuilder {
crates/amun-consensus-network/src/slashing_certificate_builder.rs:91:    fn default() -> Self {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:102:    fn make_slash(id: u8, height: u64) -> ExecutedSlash {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:113:    fn n121_3_valid_fraud_proof_verifies() {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:118:        let proof = SlashingFraudProof::new([0xAA; 32], 42, claimed, history);
crates/amun-consensus-network/src/slashing_fraud_proof.rs:122:            proof.verify().is_ok(),
crates/amun-consensus-network/src/slashing_fraud_proof.rs:123:            "N121.3 FAIL: valid fraud proof must verify"
crates/amun-consensus-network/src/slashing_fraud_proof.rs:129:    fn n121_3_matching_roots_not_fraud() {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:12:pub struct SlashingFraudProof {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:133:        let proof = SlashingFraudProof::new([0xAA; 32], 42, root, history);
crates/amun-consensus-network/src/slashing_fraud_proof.rs:136:            proof.verify().is_err(),
crates/amun-consensus-network/src/slashing_fraud_proof.rs:143:    fn n121_3_tampered_history_rejected() {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:148:        let mut proof = SlashingFraudProof::new([0xAA; 32], 42, claimed, history);
crates/amun-consensus-network/src/slashing_fraud_proof.rs:154:            proof.verify().is_err(),
crates/amun-consensus-network/src/slashing_fraud_proof.rs:155:            "N121.3 FAIL: tampered slash history must invalidate proof"
crates/amun-consensus-network/src/slashing_fraud_proof.rs:160:    fn n121_3_tampered_proof_id_rejected() {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:165:        let mut proof = SlashingFraudProof::new([0xAA; 32], 42, claimed, history);
crates/amun-consensus-network/src/slashing_fraud_proof.rs:170:            proof.verify().is_err(),
crates/amun-consensus-network/src/slashing_fraud_proof.rs:176:    fn n121_3_roundtrip_serialization() {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:179:        let proof = SlashingFraudProof::new([0xAA; 32], 42, [0xBA; 32], history);
crates/amun-consensus-network/src/slashing_fraud_proof.rs:182:        let decoded: SlashingFraudProof = postcard::from_bytes(&encoded).unwrap();
crates/amun-consensus-network/src/slashing_fraud_proof.rs:187:        assert!(decoded.verify().is_ok());
crates/amun-consensus-network/src/slashing_fraud_proof.rs:1:// N121.3 — Slashing Fraud Proof Engine
crates/amun-consensus-network/src/slashing_fraud_proof.rs:24:    /// The correctly computed slashing_root
crates/amun-consensus-network/src/slashing_fraud_proof.rs:28:    /// The executed slashes that produce the expected_root
crates/amun-consensus-network/src/slashing_fraud_proof.rs:36:impl SlashingFraudProof {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:38:    pub fn new(
crates/amun-consensus-network/src/slashing_fraud_proof.rs:4:// match the locally computed root. These proofs can be gossiped
crates/amun-consensus-network/src/slashing_fraud_proof.rs:53:        proof.proof_id = proof.compute_proof_id();
crates/amun-consensus-network/src/slashing_fraud_proof.rs:58:    fn compute_proof_id(&self) -> [u8; 32] {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:5:// so other validators can independently verify the mismatch.
crates/amun-consensus-network/src/slashing_fraud_proof.rs:65:        hasher.finalize().into()
crates/amun-consensus-network/src/slashing_fraud_proof.rs:70:    pub fn verify(&self) -> Result<(), String> {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:72:        let recomputed_id = self.compute_proof_id();
crates/amun-consensus-network/src/slashing_fraud_proof.rs:73:        if recomputed_id != self.proof_id {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:77:        // Recompute the expected root from the provided history
crates/amun-consensus-network/src/slashing_fraud_proof.rs:78:        let recomputed_root = merkle_root(&self.slash_history);
crates/amun-consensus-network/src/slashing_fraud_proof.rs:79:        if recomputed_root != self.expected_root {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:92:    pub fn is_fraudulent(&self) -> bool {
crates/amun-consensus-network/src/slashing_fraud_proof.rs:93:        self.claimed_root != self.expected_root && self.verify().is_ok()
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:10:/// N121.4: A Merkle inclusion proof for an executed slash.
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:111:        .map(SlashingInclusionProof::leaf_hash)
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:12:pub struct SlashingInclusionProof {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:142:            next_level.push(hasher.finalize().into());
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:151:    Ok(SlashingInclusionProof {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:166:    fn make_slash(id: u8, height: u64) -> ExecutedSlash {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:177:    fn n121_4_single_element_inclusion_proof() {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:179:        let proof = build_inclusion_proof(&slashes, 0).unwrap();
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:181:            proof.verify().is_ok(),
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:182:            "N121.4 FAIL: single element proof must verify"
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:189:    fn n121_4_multi_element_inclusion_proof() {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:193:        let proof = build_inclusion_proof(&slashes, 2).unwrap();
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:195:            proof.verify().is_ok(),
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:196:            "N121.4 FAIL: inclusion proof for index 2 must verify"
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:1:// N121.4 — Merkle Inclusion Proofs for Slashing Ledger
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:202:    fn n121_4_wrong_element_rejected() {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:204:        let mut proof = build_inclusion_proof(&slashes, 1).unwrap();
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:210:            proof.verify().is_err(),
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:216:    fn n121_4_out_of_bounds_index_rejected() {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:218:        assert!(build_inclusion_proof(&slashes, 5).is_err());
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:222:    fn n121_4_empty_ledger_rejected() {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:223:        assert!(build_inclusion_proof(&[], 0).is_err());
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:227:    fn n121_4_proof_matches_merkle_root() {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:232:            let proof = build_inclusion_proof(&slashes, i).unwrap();
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:239:                proof.verify().is_ok(),
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:240:                "N121.4 FAIL: proof must verify at index {}",
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:30:impl SlashingInclusionProof {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:32:    fn leaf_hash(slash: &ExecutedSlash) -> [u8; 32] {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:3:// Enables light clients and validators to verify that a specific
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:40:        hasher.finalize().into()
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:44:    pub fn verify(&self) -> Result<(), String> {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:76:            current = hasher.finalize().into();
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:94:pub fn build_inclusion_proof(
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:97:) -> Result<SlashingInclusionProof, String> {
crates/amun-consensus-network/src/slashing_inclusion_proof.rs:99:        return Err("N121.4: cannot build proof for empty ledger".into());
crates/amun-consensus-network/src/slashing_ledger.rs:100:    /// N119.5: Get all executed slashes for a validator.
crates/amun-consensus-network/src/slashing_ledger.rs:101:    pub fn history_for(&self, validator_id: &[u8; 32]) -> Vec<&ExecutedSlash> {
crates/amun-consensus-network/src/slashing_ledger.rs:109:impl Default for SlashingLedger {
crates/amun-consensus-network/src/slashing_ledger.rs:110:    fn default() -> Self {
crates/amun-consensus-network/src/slashing_ledger.rs:118:    use crate::evidence_store::EvidenceType;
crates/amun-consensus-network/src/slashing_ledger.rs:119:    use crate::slashing_certificate::{EvidenceCount, SlashingCertificate};
crates/amun-consensus-network/src/slashing_ledger.rs:11:pub fn certificate_id(cert: &SlashingCertificate) -> [u8; 32] {
crates/amun-consensus-network/src/slashing_ledger.rs:122:    fn make_cert(id: u64) -> SlashingCertificate {
crates/amun-consensus-network/src/slashing_ledger.rs:127:            vec![EvidenceCount {
crates/amun-consensus-network/src/slashing_ledger.rs:128:                evidence_type: EvidenceType::DoubleVote,
crates/amun-consensus-network/src/slashing_ledger.rs:142:    fn n119_1_certificate_id_stable() {
crates/amun-consensus-network/src/slashing_ledger.rs:157:    fn n119_2_replay_rejected() {
crates/amun-consensus-network/src/slashing_ledger.rs:163:        let r1 = ledger.execute(&cert, || Ok("executed"));
crates/amun-consensus-network/src/slashing_ledger.rs:165:        assert!(ledger.is_executed(&id));
crates/amun-consensus-network/src/slashing_ledger.rs:167:        // Second execution: rejected (replay protection)
crates/amun-consensus-network/src/slashing_ledger.rs:168:        let r2 = ledger.execute(&cert, || Ok("executed again"));
crates/amun-consensus-network/src/slashing_ledger.rs:169:        assert!(r2.is_err(), "N119.2 FAIL: replay must be rejected");
crates/amun-consensus-network/src/slashing_ledger.rs:170:        assert!(r2.unwrap_err().contains("already executed"));
crates/amun-consensus-network/src/slashing_ledger.rs:171:        assert_eq!(ledger.executed_count(), 1);
crates/amun-consensus-network/src/slashing_ledger.rs:175:    fn n119_3_ledger_records_execution() {
crates/amun-consensus-network/src/slashing_ledger.rs:179:        ledger.execute(&cert, || Ok(())).unwrap();
crates/amun-consensus-network/src/slashing_ledger.rs:180:        assert_eq!(ledger.executed_count(), 1);
crates/amun-consensus-network/src/slashing_ledger.rs:187:    fn n119_4_different_certificates_different_ids() {
crates/amun-consensus-network/src/slashing_ledger.rs:18:    hasher.update(&cert.executed_at_height.to_le_bytes());
crates/amun-consensus-network/src/slashing_ledger.rs:199:    fn n119_5_duplicate_execution_no_effect() {
crates/amun-consensus-network/src/slashing_ledger.rs:1:// N119 — Deterministic Slashing Ledger & Replay Protection
crates/amun-consensus-network/src/slashing_ledger.rs:204:        ledger.execute(&cert, || Ok(100u64)).unwrap();
crates/amun-consensus-network/src/slashing_ledger.rs:207:        let r2 = ledger.execute(&cert, || Ok(200u64));
crates/amun-consensus-network/src/slashing_ledger.rs:210:            ledger.executed_count(),
crates/amun-consensus-network/src/slashing_ledger.rs:218:    fn n119_5_audit_trail_by_validator() {
crates/amun-consensus-network/src/slashing_ledger.rs:224:            vec![EvidenceCount {
crates/amun-consensus-network/src/slashing_ledger.rs:225:                evidence_type: EvidenceType::InvalidSignature,
crates/amun-consensus-network/src/slashing_ledger.rs:238:        ledger.execute(&cert1, || Ok(())).unwrap();
crates/amun-consensus-network/src/slashing_ledger.rs:239:        ledger.execute(&cert2, || Ok(())).unwrap();
crates/amun-consensus-network/src/slashing_ledger.rs:25:    hasher.finalize().into()
crates/amun-consensus-network/src/slashing_ledger.rs:28:/// N119.5: Record of an executed slash for auditability.
crates/amun-consensus-network/src/slashing_ledger.rs:30:pub struct ExecutedSlash {
crates/amun-consensus-network/src/slashing_ledger.rs:3:// Prevents the same SlashingCertificate from being executed
crates/amun-consensus-network/src/slashing_ledger.rs:40:/// N119.1: Persistent ledger preventing replay of slashing certificates.
crates/amun-consensus-network/src/slashing_ledger.rs:42:pub struct SlashingLedger {
crates/amun-consensus-network/src/slashing_ledger.rs:43:    executed_ids: HashSet<[u8; 32]>,
crates/amun-consensus-network/src/slashing_ledger.rs:44:    /// N119.5: Audit trail of all executed slashes.
crates/amun-consensus-network/src/slashing_ledger.rs:48:impl SlashingLedger {
crates/amun-consensus-network/src/slashing_ledger.rs:49:    pub fn new() -> Self {
crates/amun-consensus-network/src/slashing_ledger.rs:51:            executed_ids: HashSet::new(),
crates/amun-consensus-network/src/slashing_ledger.rs:56:    /// N119.1: Check if a certificate has already been executed.
crates/amun-consensus-network/src/slashing_ledger.rs:57:    pub fn is_executed(&self, id: &[u8; 32]) -> bool {
crates/amun-consensus-network/src/slashing_ledger.rs:58:        self.executed_ids.contains(id)
crates/amun-consensus-network/src/slashing_ledger.rs:62:    /// Returns Err if the certificate was already executed (replay protection).
crates/amun-consensus-network/src/slashing_ledger.rs:63:    pub fn execute<F, T>(&mut self, cert: &SlashingCertificate, execute_fn: F) -> Result<T, String>
crates/amun-consensus-network/src/slashing_ledger.rs:69:        // N119.2: Replay protection
crates/amun-consensus-network/src/slashing_ledger.rs:70:        if self.executed_ids.contains(&id) {
crates/amun-consensus-network/src/slashing_ledger.rs:72:                "N119: certificate already executed: {:02x?}",
crates/amun-consensus-network/src/slashing_ledger.rs:78:        let result = execute_fn()?;
crates/amun-consensus-network/src/slashing_ledger.rs:81:        self.executed_ids.insert(id);
crates/amun-consensus-network/src/slashing_ledger.rs:88:            height: cert.executed_at_height,
crates/amun-consensus-network/src/slashing_ledger.rs:95:    /// N119.1: Get the number of executed slashes.
crates/amun-consensus-network/src/slashing_ledger.rs:96:    pub fn executed_count(&self) -> usize {
crates/amun-consensus-network/src/slashing_ledger.rs:97:        self.executed_ids.len()
crates/amun-consensus-network/src/slashing_merkle.rs:109:    fn n120_1_root_changes_with_new_slash() {
crates/amun-consensus-network/src/slashing_merkle.rs:10:/// N120.1: Leaf hash for a single executed slash.
crates/amun-consensus-network/src/slashing_merkle.rs:11:fn leaf_hash(slash: &ExecutedSlash) -> [u8; 32] {
crates/amun-consensus-network/src/slashing_merkle.rs:121:    fn n120_1_larger_tree_is_deterministic() {
crates/amun-consensus-network/src/slashing_merkle.rs:132:    fn n120_1_order_affects_root() {
crates/amun-consensus-network/src/slashing_merkle.rs:19:    hasher.finalize().into()
crates/amun-consensus-network/src/slashing_merkle.rs:22:/// N120.1: Compute the Merkle root from a slice of executed slashes.
crates/amun-consensus-network/src/slashing_merkle.rs:25:pub fn merkle_root(slashes: &[ExecutedSlash]) -> [u8; 32] {
crates/amun-consensus-network/src/slashing_merkle.rs:3:// Computes a Merkle root from the SlashingLedger's executed certificates,
crates/amun-consensus-network/src/slashing_merkle.rs:43:            next.push(hasher.finalize().into());
crates/amun-consensus-network/src/slashing_merkle.rs:4:// enabling the root to be committed in block headers for consensus
crates/amun-consensus-network/src/slashing_merkle.rs:56:    fn make_slash(id: u8, height: u64) -> ExecutedSlash {
crates/amun-consensus-network/src/slashing_merkle.rs:67:    fn n120_1_empty_ledger_gives_zero_root() {
crates/amun-consensus-network/src/slashing_merkle.rs:72:    fn n120_1_single_leaf_is_deterministic() {
crates/amun-consensus-network/src/slashing_merkle.rs:84:    fn n120_1_same_order_same_root() {
crates/amun-consensus-network/src/slashing_merkle.rs:97:    fn n120_1_different_slashes_different_root() {
crates/amun-consensus-network/src/slashing_state.rs:102:    fn n121_1_root_updates_after_execution() {
crates/amun-consensus-network/src/slashing_state.rs:107:        state.execute(&cert, || Ok(())).unwrap();
crates/amun-consensus-network/src/slashing_state.rs:117:        assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/src/slashing_state.rs:118:        assert_eq!(state.executed_count(), 1);
crates/amun-consensus-network/src/slashing_state.rs:11:/// Wraps the deterministic ledger and exposes a root for consensus commitment.
crates/amun-consensus-network/src/slashing_state.rs:122:    fn n121_1_multiple_executions_update_root() {
crates/amun-consensus-network/src/slashing_state.rs:138:        state.execute(&cert1, || Ok(())).unwrap();
crates/amun-consensus-network/src/slashing_state.rs:13:pub struct SlashingState {
crates/amun-consensus-network/src/slashing_state.rs:140:        assert_eq!(state.executed_count(), 1);
crates/amun-consensus-network/src/slashing_state.rs:142:        state.execute(&cert2, || Ok(())).unwrap();
crates/amun-consensus-network/src/slashing_state.rs:144:        assert_eq!(state.executed_count(), 2);
crates/amun-consensus-network/src/slashing_state.rs:150:        assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/src/slashing_state.rs:154:    fn n121_1_replay_protection_preserved() {
crates/amun-consensus-network/src/slashing_state.rs:158:        state.execute(&cert, || Ok(())).unwrap();
crates/amun-consensus-network/src/slashing_state.rs:162:        let result = state.execute(&cert, || Ok(()));
crates/amun-consensus-network/src/slashing_state.rs:163:        assert!(result.is_err(), "N121.1 FAIL: replay must be rejected");
crates/amun-consensus-network/src/slashing_state.rs:164:        assert!(result.unwrap_err().contains("already executed"));
crates/amun-consensus-network/src/slashing_state.rs:169:            "N121.1 FAIL: root must not change on replay"
crates/amun-consensus-network/src/slashing_state.rs:171:        assert_eq!(state.executed_count(), 1);
crates/amun-consensus-network/src/slashing_state.rs:18:impl SlashingState {
crates/amun-consensus-network/src/slashing_state.rs:20:    pub fn new() -> Self {
crates/amun-consensus-network/src/slashing_state.rs:27:    pub fn execute<F, T>(
crates/amun-consensus-network/src/slashing_state.rs:30:        execute_fn: F,
crates/amun-consensus-network/src/slashing_state.rs:35:        let result = self.ledger.execute(cert, execute_fn)?;
crates/amun-consensus-network/src/slashing_state.rs:42:    pub fn verify_consistency(&self) -> Result<(), String> {
crates/amun-consensus-network/src/slashing_state.rs:43:        let recomputed = merkle_root(&self.ledger.history);
crates/amun-consensus-network/src/slashing_state.rs:44:        if recomputed != self.root {
crates/amun-consensus-network/src/slashing_state.rs:50:    /// Get the number of executed slashes.
crates/amun-consensus-network/src/slashing_state.rs:51:    pub fn executed_count(&self) -> usize {
crates/amun-consensus-network/src/slashing_state.rs:52:        self.ledger.executed_count()
crates/amun-consensus-network/src/slashing_state.rs:56:    pub fn history(&self) -> &[ExecutedSlash] {
crates/amun-consensus-network/src/slashing_state.rs:5:// This root is committed alongside state_root and history_root.
crates/amun-consensus-network/src/slashing_state.rs:61:impl Default for SlashingState {
crates/amun-consensus-network/src/slashing_state.rs:62:    fn default() -> Self {
crates/amun-consensus-network/src/slashing_state.rs:70:    use crate::evidence_store::EvidenceType;
crates/amun-consensus-network/src/slashing_state.rs:71:    use crate::slashing_certificate::{EvidenceCount, SlashingCertificate};
crates/amun-consensus-network/src/slashing_state.rs:74:    fn make_cert() -> SlashingCertificate {
crates/amun-consensus-network/src/slashing_state.rs:79:            vec![EvidenceCount {
crates/amun-consensus-network/src/slashing_state.rs:80:                evidence_type: EvidenceType::DoubleVote,
crates/amun-consensus-network/src/slashing_state.rs:94:    fn n121_1_initial_state_has_zero_root() {
crates/amun-consensus-network/src/slashing_state.rs:97:        assert_eq!(state.executed_count(), 0);
crates/amun-consensus-network/src/slashing_state.rs:98:        assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/src/staking_adapter.rs:177:    impl SlashingExecutor for SimulatedStaking {
crates/amun-consensus-network/src/staking_adapter.rs:198:    fn n110_1_slashing_reduces_real_validator_stake() {
crates/amun-consensus-network/src/staking_adapter.rs:31:pub trait SlashingExecutor {
crates/amun-consensus-network/src/staking_adapter.rs:38:pub struct StakingAdapter<E: SlashingExecutor> {
crates/amun-consensus-network/src/staking_adapter.rs:55:    /// Returns SlashResult if slashing was executed, None if validator
crates/amun-consensus-network/src/validation.rs:30:                "HASH_INTEGRITY: stated={} computed={}",
crates/amun-consensus-network/tests/n109_8_execution_commitment.rs:275:    // Replace commitment with one for block B (attack attempt)
crates/amun-consensus-network/tests/n109_block_propagation.rs:98:            "HASH_INTEGRITY: stated={} computed={}",
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:14:    EvidenceStore, EvidenceType, EvidenceValidationResult, SlashingCertificate, ValidatorStatus,
crates/amun-consensus-network/tests/n111_7_end_to_end_evidence_sync.rs:18:fn make_certificate(validator_id: [u8; 32], evidence_ids: Vec<[u8; 32]>) -> SlashingCertificate {
crates/amun-consensus-network/tests/n112_4_push_sync_end_to_end.rs:26:fn make_certificate(validator_id: [u8; 32], evidence_ids: Vec<[u8; 32]>) -> SlashingCertificate {
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:4:use amun_consensus_network::{EvidenceCount, EvidenceType, SlashingCertificate, ValidatorStatus};
crates/amun-consensus-network/tests/n114_3_certificate_signatures.rs:7:fn make_unsigned_certificate(validator_id: [u8; 32]) -> SlashingCertificate {
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:10:fn make_cert(validator_id: [u8; 32], height: u64, amount: u64) -> SlashingCertificate {
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:1:// N121.2 — Deterministic Replay of Slashing State
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:3:// Verifies that replaying the same sequence of slashing certificates
crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:7:    EvidenceCount, EvidenceType, SlashingCertificate, SlashingState, ValidatorStatus,
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:11:fn make_cert(vid: [u8; 32], h: u64, amt: u64) -> SlashingCertificate {
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:8:    EvidenceCount, EvidenceType, SlashingCertificate, SlashingState, ValidatorStatus,
crates/amun-constitution/src/stake_quorum.rs:30:    pub fn byzantine_threshold_stake(&self) -> u64 {
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:29:    // From EvidenceStore (slashing certificates)
crates/amun-constitutional-enforcement/src/lib.rs:104:                ConstitutionalLaw::StateRootIntegrity,
crates/amun-constitutional-enforcement/src/lib.rs:108:                ConstitutionalLaw::SlashingEvidenceBinding,
crates/amun-constitutional-enforcement/src/lib.rs:139:                &ConstitutionalLaw::StateRootIntegrity,
crates/amun-constitutional-enforcement/src/lib.rs:159:                &ConstitutionalLaw::SlashingEvidenceBinding,
crates/amun-constitutional-enforcement/src/lib.rs:252:                assert_eq!(violations[0].law, ConstitutionalLaw::StateRootIntegrity);
crates/amun-constitutional-enforcement/src/lib.rs:274:        k.deactivate_law(&ConstitutionalLaw::StateRootIntegrity);
crates/amun-constitutional-enforcement/src/lib.rs:61:    SlashingEvidenceBinding,
crates/amun-constitutional-enforcement/src/proof_engine.rs:113:        if !Self::verify_state_root_integrity(block_state_root, execution_state_root) {
crates/amun-constitutional-enforcement/src/proof_engine.rs:115:                law: ConstitutionalLaw::StateRootIntegrity,
crates/amun-constitutional-enforcement/src/proof_engine.rs:14:    pub fn verify_state_root_integrity(
crates/amun-constitutional-enforcement/src/proof_engine.rs:152:        // Slashing Evidence Binding
crates/amun-constitutional-enforcement/src/proof_engine.rs:153:        if !Self::verify_slashing_evidence_binding(evidence_ids, evidence_available) {
crates/amun-constitutional-enforcement/src/proof_engine.rs:155:                law: ConstitutionalLaw::SlashingEvidenceBinding,
crates/amun-constitutional-enforcement/src/proof_engine.rs:247:                    .any(|v| v.law == ConstitutionalLaw::StateRootIntegrity));
crates/amun-constitutional-enforcement/src/proof_engine.rs:52:    pub fn verify_slashing_evidence_binding(
crates/amun-constitutional-proof/src/lib.rs:1171:    fn n47_3_s1_lineage_integrity() {
crates/amun-contract-security/src/lib.rs:103:pub fn audit_determinism() -> SecurityAuditResult {
crates/amun-contract-security/src/lib.rs:104:    let mut reg1 = ResourceRegistry::new(100);
crates/amun-contract-security/src/lib.rs:105:    let mut reg2 = ResourceRegistry::new(100);
crates/amun-contract-security/src/lib.rs:113:    let root1 = cr1.compute_registry_root();
crates/amun-contract-security/src/lib.rs:114:    let root2 = cr2.compute_registry_root();
crates/amun-contract-security/src/lib.rs:122:pub fn audit_malicious_bytecode() -> SecurityAuditResult {
crates/amun-contract-security/src/lib.rs:123:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-security/src/lib.rs:12:pub fn audit_reentrancy() -> SecurityAuditResult {
crates/amun-contract-security/src/lib.rs:134:pub fn audit_evidence_consistency() -> SecurityAuditResult {
crates/amun-contract-security/src/lib.rs:135:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-security/src/lib.rs:13:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-security/src/lib.rs:141:    let root1 = contract_reg.compute_registry_root();
crates/amun-contract-security/src/lib.rs:142:    let root2 = contract_reg.compute_registry_root();
crates/amun-contract-security/src/lib.rs:146:        details: "Evidence root consistent".into(),
crates/amun-contract-security/src/lib.rs:48:pub fn audit_gas_exhaustion() -> SecurityAuditResult {
crates/amun-contract-security/src/lib.rs:49:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-security/src/lib.rs:4:use amun_resource_core::{ResourceId, ResourceRegistry};
crates/amun-contract-security/src/lib.rs:6:pub struct SecurityAuditResult {
crates/amun-contract-security/src/lib.rs:81:pub fn audit_state_isolation() -> SecurityAuditResult {
crates/amun-contract-security/src/lib.rs:82:    let mut reg = ResourceRegistry::new(100);
crates/amun-contract-security/tests/n170_security_audit_tests.rs:10:fn n170_audit_gas_exhaustion_pass() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:16:fn n170_audit_state_isolation_pass() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:22:fn n170_audit_determinism_pass() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:28:fn n170_audit_malicious_bytecode_pass() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:34:fn n170_audit_evidence_consistency_pass() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:40:fn n170_full_security_suite() {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:4:fn n170_audit_reentrancy_pass() {
crates/amun-evidence/src/equivocation.rs:10:impl EquivocationProof {
crates/amun-evidence/src/equivocation.rs:11:    pub fn verify(&self, chain_id: u64) -> bool {
crates/amun-evidence/src/equivocation.rs:13:            && amun_crypto::Ed25519Signer::verify(
crates/amun-evidence/src/equivocation.rs:1:pub struct EquivocationProof {
crates/amun-evidence/src/equivocation.rs:21:            && amun_crypto::Ed25519Signer::verify(
crates/amun-evidence/src/equivocation.rs:31:    pub fn compute_id(&self) -> [u8; 32] {
crates/amun-evidence/src/equivocation.rs:36:        let h = hasher.finalize();
crates/amun-evidence/src/evidence.rs:16:pub struct EquivocationPosition {
crates/amun-evidence/src/evidence.rs:72:            EvidenceProof::Equivocation { first, second, .. } => {
crates/amun-evidence/src/lib.rs:4:pub use equivocation::EquivocationProof;
crates/amun-evidence/src/lib.rs:5:pub use slashing::SlashingEngine;
crates/amun-evidence/src/slashing.rs:16:    pub fn process(
crates/amun-evidence/src/slashing.rs:18:        proof: &EquivocationProof,
crates/amun-evidence/src/slashing.rs:1:use crate::equivocation::EquivocationProof;
crates/amun-evidence/src/slashing.rs:23:        if !proof.verify(chain_id) {
crates/amun-evidence/src/slashing.rs:3:pub struct SlashingEngine {
crates/amun-evidence/src/slashing.rs:45:impl Default for SlashingEngine {
crates/amun-evidence/src/slashing.rs:46:    fn default() -> Self {
crates/amun-evidence/src/slashing.rs:8:impl SlashingEngine {
crates/amun-evidence/src/slashing.rs:9:    pub fn new() -> Self {
crates/amun-evidence/src/tests.rs:12:            proof: EvidenceProof::Equivocation {
crates/amun-evidence/src/tests.rs:38:            evidence_type: EvidenceType::Equivocation,
crates/amun-evidence/src/tests.rs:42:            proof: EvidenceProof::Equivocation {
crates/amun-evidence/src/tests.rs:8:            evidence_type: EvidenceType::Equivocation,
crates/amun-failure/src/tests.rs:55:        ConstitutionalFault::EquivocationDetected,
crates/amun-failure/src/tests.rs:77:        .poison(ConstitutionalFault::EquivocationDetected, 1, 3)
crates/amun-failure/src/tests.rs:83:        assert_eq!(fault, ConstitutionalFault::EquivocationDetected);
crates/amun-failure/src/tests.rs:8:    assert!(ConstitutionalFault::EquivocationDetected.should_halt());
crates/amun-genesis/src/block.rs:41:    pub fn verify_integrity(&self) -> bool {
crates/amun-live-cluster/src/bin/byzantine_partition_test.rs:6:fn main() {
crates/amun-live-cluster/src/bin/byzantine_test.rs:44:            commitment: None,
crates/amun-live-cluster/src/bin/byzantine_test.rs:9:fn main() {
crates/amun-live-cluster/src/validator.rs:418:                    // Recompute hash after setting slashing_root (N120.2 requires it in hash)
crates/amun-live-cluster/tests/n110_4c_state_transition.rs:22:fn make_valid_certificate(validator_id: [u8; 32]) -> SlashingCertificate {
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:12:fn n118_slashing_executes_after_finality() {
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:1:// N118 — Finality-Gated Slashing Verification
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:20:    misbehavior.record_misbehavior(&validator_id, &[0xA1; 32], &EvidenceType::DoubleVote, 1);
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:21:    misbehavior.record_misbehavior(&validator_id, &[0xA2; 32], &EvidenceType::DoubleVote, 2);
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:22:    misbehavior.record_misbehavior(&validator_id, &[0xA3; 32], &EvidenceType::DoubleVote, 3);
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:2:// Verifies that slashing executes in the finality path and reduces stake.
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:34:        "N118: Slashing must execute in finality path"
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:41:    eprintln!("N118 PASSED: Slashing executes in finality-gated path");
crates/amun-live-cluster/tests/n118_finality_gated_slashing.rs:5:    EvidenceType, MisbehaviorRegistry, MisbehaviorThresholds, RealStakingExecutor, StakingAdapter,
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:17:        block.verify_slashing_root(&root).is_ok(),
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:23:fn n120_4b_validator_rejects_mismatched_slashing_root() {
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:30:    let result = block.verify_slashing_root(&[0xFF; 32]);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:50:    let result = block.verify_slashing_root(&[0x42; 32]);
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:78:        block.verify_slashing_root(&validator_root).is_err(),
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:7:fn n120_4a_validator_accepts_matching_slashing_root() {
crates/amun-live-cluster/tests/n120_4_consensus_enforcement.rs:8:    // Setup: build a block with a known slashing_root
crates/amun-live-cluster/tests/n126_3_unconstitutional_block_rejected.rs:19:                    .any(|v| v.law == ConstitutionalLaw::StateRootIntegrity),
crates/amun-network-transport/src/message.rs:226:    fn n110_3b_roundtrip_slashing_certificate() {
crates/amun-network-transport/src/message.rs:51:pub struct SlashingCertificateAnnounce {
crates/amun-networking/src/risk.rs:67:            ConstitutionalRisk::ByzantineBehavior { .. } => {
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
crates/amun-nft-adversarial/src/lib.rs:10:    fn it_works() {
crates/amun-nft-adversarial/src/lib.rs:1:pub fn add(left: u64, right: u64) -> u64 {
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
crates/amun-node/src/bin/test_byzantine_fault.rs:1:fn main() {}
crates/amun-node/src/bin/test_multi_byzantine.rs:102:                    let prev = node.committed_blocks.len();
crates/amun-node/src/bin/test_multi_byzantine.rs:104:                    if node.committed_blocks.len() > prev {
crates/amun-node/src/bin/test_multi_byzantine.rs:106:                            let h = node.committed_blocks.len() as u64;
crates/amun-node/src/bin/test_multi_byzantine.rs:10:fn main() {
crates/amun-node/src/bin/test_multi_byzantine.rs:112:                                lineage: ResourceLineage::genesis(rid),
crates/amun-node/src/bin/test_multi_byzantine.rs:171:        println!("\nPARTIAL: Honest validators converge but commit rate affected");
crates/amun-node/src/bin/test_multi_byzantine.rs:37:                         target_commits: usize,
crates/amun-node/src/bin/test_multi_byzantine.rs:46:            .map(|n| n.committed_blocks.len())
crates/amun-node/src/bin/test_multi_byzantine.rs:49:            < target_commits
crates/amun-node/src/bin/test_multi_byzantine.rs:6:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceState,
crates/amun-recovery/src/lib.rs:67:        // Verify replay chain integrity
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
crates/amun-replay-engine/src/containment_boundary.rs:73:pub struct ByzantineContainmentZone {
crates/amun-replay-engine/src/containment_boundary.rs:77:impl ByzantineContainmentZone {
crates/amun-replay-engine/src/deterministic.rs:113:    pub fn verify_integrity(&self) -> bool {
crates/amun-safety-laws/src/equivocation.rs:10:impl EquivocationEvidence {
crates/amun-safety-laws/src/equivocation.rs:11:    pub fn new(vote_a: ConsensusVote, vote_b: ConsensusVote) -> Self {
crates/amun-safety-laws/src/equivocation.rs:23:    pub fn verify(&self) -> bool {
crates/amun-safety-laws/src/equivocation.rs:32:    pub fn validator_id(&self) -> u64 {
crates/amun-safety-laws/src/equivocation.rs:36:    pub fn round(&self) -> u64 {
crates/amun-safety-laws/src/equivocation.rs:41:pub fn detect_equivocation(votes: &[ConsensusVote]) -> Vec<EquivocationEvidence> {
crates/amun-safety-laws/src/equivocation.rs:53:                evidence.push(EquivocationEvidence::new(existing.clone(), vote.clone()));
crates/amun-safety-laws/src/equivocation.rs:5:pub struct EquivocationEvidence {
crates/amun-safety-laws/src/lib.rs:4:pub use equivocation::{detect_equivocation, EquivocationEvidence};
crates/amun-safety-laws/src/slash.rs:10:impl SlashingCondition {
crates/amun-safety-laws/src/slash.rs:17:        proof: &EquivocationProof,
crates/amun-safety-laws/src/slash.rs:1:use crate::equivocation::EquivocationProof;
crates/amun-safety-laws/src/slash.rs:5:pub struct SlashingCondition {
crates/amun-snapshot-engine/src/byzantine_sync.rs:118:    pub fn verify_peer_identity(&self, peer: &PeerManifest) -> bool {
crates/amun-snapshot-engine/src/byzantine_sync.rs:119:        peer.identity.matches(&self.local_identity) && peer.identity.verify()
crates/amun-snapshot-engine/src/byzantine_sync.rs:14:pub enum SyncDecision {
crates/amun-snapshot-engine/src/byzantine_sync.rs:1:use super::constitutional_identity::ConstitutionalIdentity;
crates/amun-snapshot-engine/src/byzantine_sync.rs:28:        local: ConstitutionalIdentity,
crates/amun-snapshot-engine/src/byzantine_sync.rs:29:        remote: ConstitutionalIdentity,
crates/amun-snapshot-engine/src/byzantine_sync.rs:34:pub struct CivilizationGroup {
crates/amun-snapshot-engine/src/byzantine_sync.rs:45:type ConsensusKey = ([u8; 32], [u8; 32]);
crates/amun-snapshot-engine/src/byzantine_sync.rs:47:pub struct ByzantineSyncEngine {
crates/amun-snapshot-engine/src/byzantine_sync.rs:48:    pub local_identity: ConstitutionalIdentity,
crates/amun-snapshot-engine/src/byzantine_sync.rs:53:impl ByzantineSyncEngine {
crates/amun-snapshot-engine/src/byzantine_sync.rs:54:    pub fn new(local_identity: ConstitutionalIdentity, required_quorum: u64) -> Self {
crates/amun-snapshot-engine/src/byzantine_sync.rs:62:    pub fn add_peer_manifest(&mut self, peer: PeerManifest) {
crates/amun-snapshot-engine/src/byzantine_sync.rs:68:    pub fn decide(&self) -> SyncDecision {
crates/amun-snapshot-engine/src/byzantine_sync.rs:6:pub struct PeerManifest {
crates/amun-snapshot-engine/src/byzantine_sync.rs:9:    pub identity: ConstitutionalIdentity,
crates/amun-snapshot-engine/src/lib.rs:22:pub use byzantine_sync::{ByzantineSyncEngine, CivilizationGroup, PeerManifest, SyncDecision};
crates/amun-soak-full/tests/n165_full_soak_tests.rs:23:fn n165_full_soak_60s_with_adversarial() {
crates/amun-staking/src/slashing.rs:13:    pub fn calculate_slash(&self, stake: u64, count: u32) -> u64 {
crates/amun-staking/src/slashing.rs:24:impl Default for SlashingConditions {
crates/amun-staking/src/slashing.rs:25:    fn default() -> Self {
crates/amun-staking/src/slashing.rs:2:pub struct SlashingConditions {
crates/amun-staking/src/slashing.rs:6:impl SlashingConditions {
crates/amun-staking/src/slashing.rs:7:    pub fn new() -> Self {
crates/amun-staking/src/validator.rs:49:    pub fn slash(&mut self, pk: &PublicKey, rules: &SlashingConditions) -> AmunResult<u64> {
crates/amun-state-machine/src/stability.rs:26:            replay_integrity_score: 1.0,
crates/amun-state-machine/src/stability.rs:6:    pub replay_integrity_score: f64,
crates/amun-state-root/src/snapshot.rs:64:    /// Uses `debug_assert!` to verify integrity in debug/test builds
crates/amun-state-root/src/snapshot.rs:68:            self.verify_constitutional_integrity(),
crates/amun-state-root/src/snapshot.rs:75:    pub fn verify_constitutional_integrity(&self) -> bool {
crates/amun-storage-kernel/VALIDITY_HIERARCHY.md:48:### Level 4: ByzantineEvidence (Malicious Behavior Detected)
crates/amun-storage-kernel/VALIDITY_HIERARCHY.md:73:| Equivocation Evidence | Permanent Ban + Slash |
crates/amun-storage-kernel/VALIDITY_HIERARCHY.md:98:| ByzantineEvidence | Permanent | Never released |
crates/amun-testnet-sim/tests/adversarial_tests.rs:106:    assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:114:// ── N60.3 — Tampered Proof ──────────────────────────────────
crates/amun-testnet-sim/tests/adversarial_tests.rs:116:fn n60_tampered_proof_rejected_by_consensus() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:117:    let mut reg = build_registry(10);
crates/amun-testnet-sim/tests/adversarial_tests.rs:124:        pre_state_root: reg.compute_state_root(),
crates/amun-testnet-sim/tests/adversarial_tests.rs:127:    let mut block = ReplayBackedConsensus::execute_and_replay(
crates/amun-testnet-sim/tests/adversarial_tests.rs:136:    block.replay_verifications[0].replay_success = false;
crates/amun-testnet-sim/tests/adversarial_tests.rs:137:    assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:147:fn n60_crash_recovery_rejoin() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:148:    let source_reg = build_registry(100);
crates/amun-testnet-sim/tests/adversarial_tests.rs:149:    let state_root_before = source_reg.compute_state_root();
crates/amun-testnet-sim/tests/adversarial_tests.rs:156:    assert_eq!(recovered_reg.compute_state_root(), state_root_before);
crates/amun-testnet-sim/tests/adversarial_tests.rs:162:fn n60_long_run_blocks_consistent_replay() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:163:    // Use the same initial state for all replays.
crates/amun-testnet-sim/tests/adversarial_tests.rs:164:    // The key insight: replay verifies that the proof is consistent with
crates/amun-testnet-sim/tests/adversarial_tests.rs:167:    let initial_reg = build_registry(10);
crates/amun-testnet-sim/tests/adversarial_tests.rs:16:fn make_id(seed: u64) -> ResourceId {
crates/amun-testnet-sim/tests/adversarial_tests.rs:185:            pre_state_root: reg.compute_state_root(),
crates/amun-testnet-sim/tests/adversarial_tests.rs:188:        let mut hot = HotProofStore::new(10000);
crates/amun-testnet-sim/tests/adversarial_tests.rs:189:        let mut archive = ProofArchive::new();
crates/amun-testnet-sim/tests/adversarial_tests.rs:190:        let result = ConstitutionalRuntime::execute(
crates/amun-testnet-sim/tests/adversarial_tests.rs:203:                transition_proof,
crates/amun-testnet-sim/tests/adversarial_tests.rs:206:                // Replay against a registry initialized to the proof's pre-state
crates/amun-testnet-sim/tests/adversarial_tests.rs:207:                let mut fresh = build_registry(10); // Same genesis state
crates/amun-testnet-sim/tests/adversarial_tests.rs:209:                                                    // Actually, ReplayVerifier::replay calls execute() which starts
crates/amun-testnet-sim/tests/adversarial_tests.rs:212:                let replay = ReplayVerifier::replay(&transition_proof, &program, &mut fresh, &[]);
crates/amun-testnet-sim/tests/adversarial_tests.rs:213:                if !matches!(replay, ReplayResult::Match { .. }) {
crates/amun-testnet-sim/tests/adversarial_tests.rs:214:                    panic!("Block {} replay failed: {:?}", i + 1, replay);
crates/amun-testnet-sim/tests/adversarial_tests.rs:22:fn build_registry(count: u64) -> ResourceRegistry {
crates/amun-testnet-sim/tests/adversarial_tests.rs:231:fn n60_byzantine_conflicting_blocks_detected() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:232:    let mut reg = build_registry(10);
crates/amun-testnet-sim/tests/adversarial_tests.rs:239:        pre_state_root: reg.compute_state_root(),
crates/amun-testnet-sim/tests/adversarial_tests.rs:23:    let mut reg = ResourceRegistry::new((count * 2) as usize);
crates/amun-testnet-sim/tests/adversarial_tests.rs:242:    let mut block = ReplayBackedConsensus::execute_and_replay(
crates/amun-testnet-sim/tests/adversarial_tests.rs:251:    block.replay_verifications[0].state_root_match = false;
crates/amun-testnet-sim/tests/adversarial_tests.rs:252:    assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:262:fn n60_large_state_sync() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:263:    let source_reg = build_registry(100);
crates/amun-testnet-sim/tests/adversarial_tests.rs:264:    let state_root = source_reg.compute_state_root();
crates/amun-testnet-sim/tests/adversarial_tests.rs:270:    assert_eq!(result.unwrap().compute_state_root(), state_root);
crates/amun-testnet-sim/tests/adversarial_tests.rs:29:            lineage: ResourceLineage::genesis(make_id(i)),
crates/amun-testnet-sim/tests/adversarial_tests.rs:2:use amun_bytecode::program::ConstitutionalProgram;
crates/amun-testnet-sim/tests/adversarial_tests.rs:38:fn make_program() -> ConstitutionalProgram {
crates/amun-testnet-sim/tests/adversarial_tests.rs:39:    ConstitutionalProgram::new(1, 0, 0, vec![OpCode::Halt])
crates/amun-testnet-sim/tests/adversarial_tests.rs:3:use amun_constitutional_runtime::runtime_pipeline::{ConstitutionalRuntime, PipelineResult};
crates/amun-testnet-sim/tests/adversarial_tests.rs:44:fn n60_network_partition_no_double_finality() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:45:    let mut reg = build_registry(10);
crates/amun-testnet-sim/tests/adversarial_tests.rs:4:use amun_proof_archive::hot_store::HotProofStore;
crates/amun-testnet-sim/tests/adversarial_tests.rs:52:        pre_state_root: reg.compute_state_root(),
crates/amun-testnet-sim/tests/adversarial_tests.rs:55:    let block = ReplayBackedConsensus::execute_and_replay(
crates/amun-testnet-sim/tests/adversarial_tests.rs:5:use amun_proof_archive::proof_archive::ProofArchive;
crates/amun-testnet-sim/tests/adversarial_tests.rs:63:    assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:69:    assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:6:use amun_replay_consensus::replay_backed_consensus::ReplayBackedConsensus;
crates/amun-testnet-sim/tests/adversarial_tests.rs:76:        assert!(ReplayBackedConsensus::form_consensus(
crates/amun-testnet-sim/tests/adversarial_tests.rs:7:use amun_replay_verifier::replay_verifier::{ReplayResult, ReplayVerifier};
crates/amun-testnet-sim/tests/adversarial_tests.rs:87:fn n60_malicious_validator_invalid_qc_rejected() {
crates/amun-testnet-sim/tests/adversarial_tests.rs:88:    let mut reg = build_registry(10);
crates/amun-testnet-sim/tests/adversarial_tests.rs:95:        pre_state_root: reg.compute_state_root(),
crates/amun-testnet-sim/tests/adversarial_tests.rs:98:    let block = ReplayBackedConsensus::execute_and_replay(
crates/amun-testnet-sim/tests/adversarial_tests.rs:9:    ResourceArchetype, ResourceId, ResourceLineage, ResourceMetadata, ResourceRegistry,
crates/amun-transcript-semantics/src/lib.rs:96:    pub fn causal_integrity(parent: &EventIdentity, child: &EventIdentity) -> Result<(), TranscriptError> { if !child.verify_causal_chain(parent) { Err(TranscriptError::CausalChainBroken { parent_hash: parent.event_hash, child_parent_hash: child.causal_parent }) } else { Ok(()) } }
crates/amun-transition-proof/src/transition_proof.rs:93:    pub fn verify_integrity(&self) -> bool {
crates/amun-wal/src/lib.rs:636:    pub fn check_integrity(&self) -> Result<WALIntegrity, String> {
crates/amun-wal/src/lib.rs:69:pub struct WALIntegrity {
docs/N105_CRYPTOGRAPHIC_VALIDATOR_IDENTITY.md:53:Vote forgery resistance is achieved because without the private key, an attacker cannot produce a valid signature for a given voter_id. The registry ensures that only known validators can attempt to vote. Sybil resistance is enforced because the registry is populated from certificates signed by the authority. An attacker cannot inject fake validators. In a production setting, the authority would be the genesis trust anchor set. Replay protection is provided by the signing payload, which includes the height, block hash, and timestamp, binding each vote to a specific consensus round. The authority key is currently hardcoded to the seed 0x42, which is acceptable for test clusters. In production, this key must be distributed via genesis configuration and protected. Certificate integrity is verified at load time. A tampered certificate will cause a panic in test clusters or an error in production loading code, preventing the node from participating.
docs/N126_FINAL_BASELINE.md:30:5.  SlashingEvidenceBinding   = all certs have non-empty evidence_ids  REAL
docs/PROTOCOL_HARDENING_ROADMAP.md:22:- [ ] Replay attack proof system
docs/REPLAY_LAW.md:258:Replay security depends on:
docs/SECURITY_MODEL.md:39:## Replay Divergence
docs/SECURITY_MODEL.md:59:# Constitutional Security Principle
docs/V2_026_ADVERSARIAL_RECONCILIATION.md:1:# V2-026: Adversarial Reconciliation – Constitutional State Arbitration
docs/V2_026_ADVERSARIAL_RECONCILIATION.md:20:## Constitutional Rules Validated
docs/V3_005D_QC_CRYPTOGRAPHIC_PROOF.md:28:V3-006: Byzantine Constitutional Attacks – now that QCs carry real crypto,
docs/V3_006_BYZANTINE_CONSTITUTIONAL_ATTACKS.md:1:# V3-006: Byzantine Constitutional Attacks – All Rejected
docs/architecture/PHASE_49_COMPLETE.md:110:5. **Byzantine Evidence System** — canonical proof generation
docs/audit/AUDIT_EVIDENCE_BUNDLE.md:41:| Proof replay attack | byz_010 | ✅ Rejected |
docs/audit/SECURITY_INVARIANTS.md:10:| R4 | Certificates are terminal | TransformationMatrix::is_terminal | PCCVVerifier |
docs/audit/SECURITY_INVARIANTS.md:11:| R5 | Cross-contract uniqueness | TransferProofRegistry::consume | StatelessVerifier |
docs/audit/SECURITY_INVARIANTS.md:12:| R6 | Version monotonicity | ResourceRegistry::consume_and_derive | WitnessBuilder |
docs/audit/SECURITY_INVARIANTS.md:13:| X1 | Transfer proof single-use | TransferProofRegistry | N47 Verdict Engine |
docs/audit/SECURITY_INVARIANTS.md:15:## Execution Invariants
docs/audit/SECURITY_INVARIANTS.md:17:| ID | Invariant | Enforcement |
docs/audit/SECURITY_INVARIANTS.md:1:# AmunChain Security Invariants Catalog
docs/audit/SECURITY_INVARIANTS.md:25:## Consensus Invariants
docs/audit/SECURITY_INVARIANTS.md:27:| ID | Invariant | Enforcement |
docs/audit/SECURITY_INVARIANTS.md:29:| C1 | Replay before vote | ReplayBackedConsensus::form_consensus |
docs/audit/SECURITY_INVARIANTS.md:31:| C3 | Five-root binding | ConstitutionalFinalityCertificate::verify |
docs/audit/SECURITY_INVARIANTS.md:32:| C4 | No conflicting finality | Theorem 5 (Replay-Backed Safety) |
docs/audit/SECURITY_INVARIANTS.md:34:## Cryptographic Invariants
docs/audit/SECURITY_INVARIANTS.md:36:| ID | Invariant | Enforcement |
docs/audit/SECURITY_INVARIANTS.md:3:## Resource Invariants (R1–R6 + X1)
docs/audit/SECURITY_INVARIANTS.md:40:| K3 | Anti-replay protection | AntiReplayGuard::check_and_record |
docs/audit/SECURITY_INVARIANTS.md:5:| ID | Invariant | Enforcement | Verification |
docs/audit/SECURITY_INVARIANTS.md:7:| R1 | No duplicate active resource IDs | ResourceRegistry::register_genesis | PCCVVerifier |
docs/audit/SECURITY_INVARIANTS.md:8:| R2 | Consumed resources cannot be used | VMKernel::verify | ReplayVerifier |
docs/audit/SECURITY_INVARIANTS.md:9:| R3 | Child requires consumed parent | ResourceRegistry::consume_and_derive | WitnessBuilder |
docs/audit/THREAT_MODEL.md:14:| Execution | Forged TransitionProof | Proof hash verification (Theorem 1) |
docs/audit/THREAT_MODEL.md:20:| Consensus | Double voting | Replay-backed QC (C1) |
docs/audit/THREAT_MODEL.md:22:| Network | Replay attack | AntiReplayGuard (K3) |
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:308:| III | Constitutional Security | Constitutional proof system, identity and evidence roots |
