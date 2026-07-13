## Formal Objects
1004:docs/V7_004_CONSTITUTIONAL_CONFLUENCE.md:27:## 1. Axioms of Constitutional Confluence
1008:docs/V7_004_CONSTITUTIONAL_CONFLUENCE.md:46:### Axiom CF4: Constitutional Confluence
1011:docs/V7_004_CONSTITUTIONAL_CONFLUENCE.md:63:**Constitutional Church-Rosser Theorem:**
1012:docs/V7_005_CONSTITUTIONAL_PRUNING.md:43:## 2. Axioms of Constitutional Pruning
1019:docs/V7_005_CONSTITUTIONAL_PRUNING.md:79:**Constitutional Pruning Theorem:**
1020:docs/V7_006_CONSTITUTIONAL_EXCLUSION.md:39:## 2. Axioms of Constitutional Exclusion
1027:docs/V7_006_CONSTITUTIONAL_EXCLUSION.md:73:**Constitutional Exclusion Theorem:**
1029:docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:125:**Constitutional Completeness Theorem:**
1039:docs/V7_008_CONSTITUTIONAL_CLOSURE.md:50:**Axiom C1: Constitutional Decidability**
1045:docs/V7_008_CONSTITUTIONAL_CLOSURE.md:82:**Constitutional Closure Theorem:**
1053:docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:55:**Constitutional Closure Theorem (Final Version):**
1057:docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:126:| Constitutional Comparability (CC) | Axiom |
1061:docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:48:**Axiom CC: Constitutional Comparability**
1094:docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:254:## 12. Consensus Invariants
1095:docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:258:**Invariant 1:** `constitutional_root == BLAKE3("AMUN_CONSTITUTIONAL_ROOT_V1" || identity_root || evidence_root || governance_root || economic_root)`
1096:docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:260:**Invariant 2:** `constitutional_commitment_root == BLAKE3("AMUN_CONSTITUTIONAL_COMMITMENT_V1" || CanonicalSerialization(ConstitutionalCommitment))`
1097:docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:262:**Invariant 3:** `state_root` must include `constitutional_commitment_root` as a leaf in its Merkle tree computation.
1098:docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:264:**Invariant 4:** The `AppHash` MUST commit to `state_root`. For CCA v1.0, this is defined as `AppHash = state_root`. Future versions may introduce additional roots (such as `execution_root` or `receipt_root`) into the `AppHash` computation, in which case the invariant will be updated to reflect the new hash composition while preserving the requirement that `state_root` remains a committed input.
1099:docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:266:**Invariant 5:** Given identical state transition inputs (same genesis, same transactions up to height N), all validators must compute identical values for all roots defined in this specification.
10:crates/amun-codec/src/writer.rs:128:                ConstitutionalFault::InvalidStateTransition,
1100:docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:33:12. Consensus Invariants
1101:docs/protocol/FREEZE_CERTIFICATE_v1.md:29:### Constitutional Invariants (FROZEN)
1135:crates/amun-state-machine/src/axioms.rs:62:/// The ConstitutionalAxiomEngine verifies that all axioms hold for a given state.
1136:crates/amun-state-machine/src/axioms.rs:63:pub struct ConstitutionalAxiomEngine;
1137:crates/amun-state-machine/src/axioms.rs:65:impl ConstitutionalAxiomEngine {
1160:crates/amun-state-machine/src/lib.rs:30:pub use axioms::{AxiomVerification, ConstitutionalAxiom, ConstitutionalAxiomEngine};
1250:crates/amun-constitutional/src/canonical_witness.rs:142:        use crate::constitutional_witness::ConstitutionalWitness;
1253:crates/amun-constitutional/src/canonical_witness.rs:147:        let w = ConstitutionalWitness::new(1, 1, 1, [0xAA; 32], [0xAB; 32], entries);
1255:crates/amun-constitutional/src/canonical_witness.rs:155:        use crate::constitutional_witness::ConstitutionalWitness;
1258:crates/amun-constitutional/src/canonical_witness.rs:160:        let w = ConstitutionalWitness::new(1, 1, 1, [0xAA; 32], [0xAB; 32], entries);
1268:crates/amun-constitutional/src/canonical_witness.rs:55:    witness: &crate::constitutional_witness::ConstitutionalWitness,
1269:crates/amun-constitutional/src/canonical_witness.rs:56:) -> crate::constitutional_witness::ConstitutionalWitness {
1270:crates/amun-constitutional/src/canonical_witness.rs:59:    crate::constitutional_witness::ConstitutionalWitness::new(
129:crates/amun-canonical-collections/src/lib.rs:16:// ─── Constitutional Traits ─────────────────────────────────
1309:crates/amun-constitutional/src/constitutional_witness.rs:1://! ConstitutionalWitness — a formally sufficient constitutional proof surface.
130:crates/amun-canonical-collections/src/lib.rs:4://! Constitutional guarantees:
1596:crates/amun-evidence-engine/src/evidence_engine.rs:153:        archive.insert(ConstitutionalEvidence::InvariantViolation {
1603:crates/amun-evidence-engine/src/evidence_engine.rs:34:                ConstitutionalEvidence::InvariantViolation {
1700:crates/amun-execution-receipt/src/lib.rs:6://! ## Constitutional Invariants
1730:crates/amun-failure/src/tests.rs:32:    assert!(!ConstitutionalFault::InvalidStateTransition.should_halt());
1797:crates/amun-invariant-engine/src/invariant_engine.rs:135:            ConstitutionalEvidence::InvariantViolation { obligation_id, .. } => {
17:crates/amun-audit/tests/audit_layer03_snapshot.rs:54:    // CONST-SNAP-003: Constitutional identity must be deterministic
1807:crates/amun-invariant-engine/src/invariant_engine.rs:194:                ConstitutionalEvidence::InvariantViolation { state_root: sr, .. } => {
1809:crates/amun-invariant-engine/src/invariant_engine.rs:19:    ) -> (Vec<InvariantResult>, Vec<ConstitutionalEvidence>)
1810:crates/amun-invariant-engine/src/invariant_engine.rs:1:use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
1813:crates/amun-invariant-engine/src/invariant_engine.rs:35:                evidence.push(ConstitutionalEvidence::InvariantViolation {
1966:crates/amun-networking/tests/n18_node_rejoin.rs:73:// N18.5 — Constitutional Invariant REJOIN-001
25:crates/amun-consensus-types/src/errors.rs:19:            ConstitutionalFault::InvalidStateTransition,
34:crates/amun-constitutional-enforcement/src/lib.rs:112:                ConstitutionalLaw::StateTransitionValidity,
35:crates/amun-constitutional-enforcement/src/lib.rs:179:                &ConstitutionalLaw::StateTransitionValidity,
37:crates/amun-constitutional-enforcement/src/proof_engine.rs:194:                law: ConstitutionalLaw::StateTransitionValidity,
40:crates/amun-constitutional-enforcement/src/state_transition.rs:178:                    .any(|v| v.law == ConstitutionalLaw::StateTransitionValidity));
42:crates/amun-constitutional-enforcement/src/state_transition.rs:70:                law: ConstitutionalLaw::StateTransitionValidity,
46:crates/amun-constitutional-geometry/src/flow_dynamics.rs:61:                ConstitutionalForce::InvariantForce { strength, .. } => {
617:crates/amun-evidence-engine/src/evidence_engine.rs:153:        archive.insert(ConstitutionalEvidence::InvariantViolation {
619:crates/amun-evidence-engine/src/evidence_engine.rs:34:                ConstitutionalEvidence::InvariantViolation {
623:crates/amun-execution-receipt/src/lib.rs:6://! ## Constitutional Invariants
627:crates/amun-failure/src/tests.rs:32:    assert!(!ConstitutionalFault::InvalidStateTransition.should_halt());
638:crates/amun-invariant-engine/src/invariant_engine.rs:135:            ConstitutionalEvidence::InvariantViolation { obligation_id, .. } => {
648:crates/amun-invariant-engine/src/invariant_engine.rs:194:                ConstitutionalEvidence::InvariantViolation { state_root: sr, .. } => {
650:crates/amun-invariant-engine/src/invariant_engine.rs:19:    ) -> (Vec<InvariantResult>, Vec<ConstitutionalEvidence>)
653:crates/amun-invariant-engine/src/invariant_engine.rs:35:                evidence.push(ConstitutionalEvidence::InvariantViolation {
726:crates/amun-networking/tests/n18_node_rejoin.rs:73:// N18.5 — Constitutional Invariant REJOIN-001
775:crates/amun-constitutional-geometry/src/flow_dynamics.rs:61:                ConstitutionalForce::InvariantForce { strength, .. } => {
803:crates/amun-self-preservation/src/lib.rs:15:pub use action_principle::{ConstitutionalAction, LeastInvariantViolation};
813:crates/amun-state-machine/src/axioms.rs:30:impl ConstitutionalAxiom {
817:crates/amun-state-machine/src/axioms.rs:57:        axiom: ConstitutionalAxiom,
818:crates/amun-state-machine/src/axioms.rs:5:pub enum ConstitutionalAxiom {
819:crates/amun-state-machine/src/axioms.rs:62:/// The ConstitutionalAxiomEngine verifies that all axioms hold for a given state.
820:crates/amun-state-machine/src/axioms.rs:63:pub struct ConstitutionalAxiomEngine;
821:crates/amun-state-machine/src/axioms.rs:65:impl ConstitutionalAxiomEngine {
834:crates/amun-state-machine/src/derivation.rs:1:use super::axioms::ConstitutionalAxiom;
836:crates/amun-state-machine/src/derivation.rs:35:    Axiom(ConstitutionalAxiom),
838:crates/amun-state-machine/src/derivation.rs:43:    pub axioms_checked: Vec<ConstitutionalAxiom>,
840:crates/amun-state-machine/src/derivation.rs:8:    pub depends_on: Vec<ConstitutionalAxiom>,
841:crates/amun-state-machine/src/engine.rs:123:        invariants: &[ConstitutionalInvariant],
842:crates/amun-state-machine/src/engine.rs:130:                ConstitutionalInvariant::NoImpossibleState => {
843:crates/amun-state-machine/src/engine.rs:137:                ConstitutionalInvariant::TransitionHistoryAcyclic => {
844:crates/amun-state-machine/src/engine.rs:142:                ConstitutionalInvariant::LineageIntact
847:crates/amun-state-machine/src/engine.rs:2:use super::invariants::ConstitutionalInvariant;
862:crates/amun-state-machine/src/invariants.rs:3:pub enum ConstitutionalInvariant {
864:crates/amun-state-machine/src/lib.rs:30:pub use axioms::{AxiomVerification, ConstitutionalAxiom, ConstitutionalAxiomEngine};
867:crates/amun-state-machine/src/lib.rs:44:pub use invariants::ConstitutionalInvariant;
876:crates/amun-state-machine/src/verifier.rs:104:                ConstitutionalInvariant::TransitionHistoryAcyclic => {
877:crates/amun-state-machine/src/verifier.rs:109:                ConstitutionalInvariant::LineageIntact
878:crates/amun-state-machine/src/verifier.rs:1:use super::invariants::ConstitutionalInvariant;
879:crates/amun-state-machine/src/verifier.rs:92:        invariants: &[ConstitutionalInvariant],
880:crates/amun-state-machine/src/verifier.rs:97:                ConstitutionalInvariant::NoImpossibleState => {
884:crates/amun-storage-kernel/CONSTITUTION.md:108:Ratified upon verification of all 10 Constitutional Theorems.
912:docs/CCS_Core_Specification_v1.0.md:158:**Constitutional Authority Uniqueness Theorem:**
915:docs/CCS_Core_Specification_v1.0.md:79:### Axiom 2: Constitutional Determinism
917:docs/CCS_Core_Specification_v1.0.md:91:### Axiom 4: Constitutional Recoverability
920:docs/CCS_Core_Specification_v1.1.md:143:Axiom 2 (Constitutional Determinism).
925:docs/CCS_Core_Specification_v1.1.md:77:### Axiom 2: Constitutional Determinism (Foundational Axiom)
926:docs/CCS_Core_Specification_v1.1.md:86:### Axiom 3: Constitutional Recoverability
937:docs/V6_001_GEOMETRY_OF_AUTHORITY.md:58:## 3. Axioms of Constitutional Space
953:docs/V6_003_CANONICALIZATION.md:75:### Axiom 2: Constitutional Determinism (Final Form)
955:docs/V6_003_CANONICALIZATION.md:86:### Axiom 4: Constitutional Monotonicity
957:docs/V6_004_CONSTITUTIONAL_SELECTION_PRINCIPLE.md:42:**Constitutional Authority Theorem:**
964:docs/V6_005_CONTINUITY_ORDERING.md:81:**Constitutional Authority Theorem (Order Version):**
966:docs/V6_006_CONSTITUTIONAL_CONVERGENCE.md:54:**Constitutional Convergence Theorem:**
968:docs/V6_007_CONSTITUTIONAL_PREFERENCE.md:50:**Constitutional Preference Theorem:**
978:docs/V6_008_CONSTITUTIONAL_RESOLUTION.md:85:**Constitutional Resolution Theorem:**
980:crates/amun-replay-engine/src/canonical.rs:178:    pub fn finalize(&self) -> ConstitutionalHash {
980:docs/V6_009_CONSTITUTIONAL_META_RESOLUTION.md:61:**Constitutional Meta-Resolution Theorem:**
982:docs/V6_010_CONSTITUTIONAL_CONSTRAINT_PRINCIPLE.md:70:**Constitutional Finality Theorem:**
986:docs/V7_001_CONSTITUTIONAL_DERIVABILITY.md:144:**Constitutional Completeness Theorem:**
987:docs/V7_001_CONSTITUTIONAL_DERIVABILITY.md:50:## 2. Axioms of Constitutional Derivability
998:docs/V7_003_CONSTITUTIONAL_REDUCTION.md:35:## 2. Axioms of Constitutional Reduction
99:crates/amun-block/src/block.rs:47:    /// For now uses a deterministic multi-field hash; will integrate ConstitutionalHasher.
9:crates/amun-codec/src/writer.rs:115:                ConstitutionalFault::InvalidStateTransition,
crates/amun-audit-trail/src/lib.rs:117:    fn n38_record_hash_deterministic() {
crates/amun-audit/src/lib.rs:2:// Invariant ownership map:
crates/amun-audit/tests/audit_layer01_physics.rs:59:    fn phys003_endian_consistency() {
crates/amun-audit/tests/audit_layer02_geometry.rs:7:    fn geo001_proof_depth_invariant() {
crates/amun-audit/tests/audit_layer03_snapshot.rs:54:    // CONST-SNAP-003: Constitutional identity must be deterministic
crates/amun-audit/tests/audit_layer12_fuzzing.rs:39:    fn fuzz003_absence_proof_consistency() {
crates/amun-audit/tests/audit_layer13_differential.rs:45:    fn diff003_empty_root_consistency() {
crates/amun-audit/tests/audit_layer13_differential.rs:9:    fn diff001_canonical_encoding_determinism() {
crates/amun-audit/tests/audit_layer16_mutation.rs:40:    fn mut004_endian_invariant() {
crates/amun-authority-registry/src/authority.rs:35:    fn n107_1_authority_id_deterministic() {
crates/amun-authority-registry/src/recovery.rs:210:    fn n107_8c_deterministic_recovery() {
crates/amun-authority-registry/src/wal.rs:160:    fn n107_8b_replay_deterministic() {
crates/amun-binary-codec/src/codec.rs:10:pub struct CanonicalEncoder {
crates/amun-binary-codec/src/codec.rs:114:impl Default for CanonicalEncoder {
crates/amun-binary-codec/src/codec.rs:14:impl CanonicalEncoder {
crates/amun-binary-codec/src/codec.rs:49:pub struct CanonicalDecoder<'a> {
crates/amun-binary-codec/src/event_codec.rs:12:pub fn decode_event_position(dec: &mut CanonicalDecoder) -> Option<ChainPosition> {
crates/amun-binary-codec/src/event_codec.rs:7:pub fn encode_event_position(enc: &mut CanonicalEncoder, position: ChainPosition) {
crates/amun-block-builder/src/lib.rs:229:    fn n27_block_hash_deterministic() {
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
crates/amun-block/src/block.rs:47:    /// For now uses a deterministic multi-field hash; will integrate ConstitutionalHasher.
crates/amun-block/src/body.rs:56:impl CanonicalEncode for BlockBody {
crates/amun-block/src/body.rs:59:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-block/src/body.rs:68:impl CanonicalDecode for BlockBody {
crates/amun-block/src/chain.rs:4:/// Deterministic finalized blockchain.
crates/amun-block/src/header.rs:51:impl CanonicalEncode for BlockHeader {
crates/amun-block/src/header.rs:53:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-block/src/header.rs:68:impl CanonicalDecode for BlockHeader {
crates/amun-block/src/tests.rs:79:fn test_block_id_deterministic() {
crates/amun-bls/src/keygen.rs:16:    pub fn generate_deterministic(seed: &[u8; 32]) -> Self {
crates/amun-bls/src/tests.rs:18:        assert!(verify(msg, &sig, &kp.public).expect("test invariant"));
crates/amun-bls/src/tests.rs:25:        assert!(!verify(msg, &zero_sig, &kp.public).expect("test invariant"));
crates/amun-bls/src/tests.rs:6:    fn test_keygen_deterministic() {
crates/amun-bytecode/src/interpreter.rs:218:                OpCode::CheckInvariant { .. } => {
crates/amun-bytecode/src/lib.rs:22:    fn w6_program_hash_deterministic() {
crates/amun-bytecode/src/opcodes.rs:14:    CheckInvariant { invariant_idx: u32 },
crates/amun-bytecode/src/opcodes.rs:40:            OpCode::CheckInvariant { .. } => "OP_CHECK_INVARIANT",
crates/amun-bytecode/src/opcodes.rs:63:            OpCode::CheckInvariant { .. } => 50,
crates/amun-bytecode/src/opcodes.rs:94:            OpCode::CheckInvariant { .. } | OpCode::EmitClaim { .. }
crates/amun-bytecode/src/program.rs:21:    pub fn new(level: u8, invariant_count: u32, entry_point: u32, code: Vec<OpCode>) -> Self {
crates/amun-byzantine-tests/tests/attack_suite.rs:14:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-byzantine-tests/tests/attack_suite.rs:263:    let proof = TransitionProof::new(
crates/amun-canonical-codec/src/hasher.rs:11:impl CanonicalHasher {
crates/amun-canonical-codec/src/hasher.rs:12:    pub fn new() -> Self {
crates/amun-canonical-codec/src/hasher.rs:17:    pub fn with_domain(domain: &[u8]) -> Self {
crates/amun-canonical-codec/src/hasher.rs:1:pub struct CanonicalHasher {
crates/amun-canonical-codec/src/hasher.rs:22:    pub fn update(&mut self, data: &[u8]) {
crates/amun-canonical-codec/src/hasher.rs:25:    pub fn update_u64(&mut self, v: u64) {
crates/amun-canonical-codec/src/hasher.rs:28:    pub fn update_bool(&mut self, v: bool) {
crates/amun-canonical-codec/src/hasher.rs:31:    pub fn finalize(self) -> [u8; 32] {
crates/amun-canonical-codec/src/hasher.rs:32:        self.hasher.finalize().into()
crates/amun-canonical-codec/src/hasher.rs:5:impl Default for CanonicalHasher {
crates/amun-canonical-codec/src/hasher.rs:6:    fn default() -> Self {
crates/amun-canonical-codec/src/reader.rs:13:    pub fn remaining(&self) -> usize {
crates/amun-canonical-codec/src/reader.rs:16:    pub fn is_finished(&self) -> bool {
crates/amun-canonical-codec/src/reader.rs:20:    pub fn read_u8(&mut self) -> Option<u8> {
crates/amun-canonical-codec/src/reader.rs:30:    pub fn read_bool(&mut self) -> Option<bool> {
crates/amun-canonical-codec/src/reader.rs:34:    pub fn read_u16(&mut self) -> Option<u16> {
crates/amun-canonical-codec/src/reader.rs:3:pub struct CanonicalReader<'a> {
crates/amun-canonical-codec/src/reader.rs:44:    pub fn read_u32(&mut self) -> Option<u32> {
crates/amun-canonical-codec/src/reader.rs:59:    pub fn read_u64(&mut self) -> Option<u64> {
crates/amun-canonical-codec/src/reader.rs:69:    pub fn read_bytes(&mut self) -> Option<Vec<u8>> {
crates/amun-canonical-codec/src/reader.rs:83:    pub fn read_hash(&mut self) -> Option<[u8; 32]> {
crates/amun-canonical-codec/src/reader.rs:9:    pub fn new(data: &'a [u8]) -> Self {
crates/amun-canonical-codec/src/writer.rs:11:impl CanonicalWriter {
crates/amun-canonical-codec/src/writer.rs:12:    pub fn new() -> Self {
crates/amun-canonical-codec/src/writer.rs:15:    pub fn with_capacity(cap: usize) -> Self {
crates/amun-canonical-codec/src/writer.rs:1:pub struct CanonicalWriter {
crates/amun-canonical-codec/src/writer.rs:20:    pub fn write_u8(&mut self, v: u8) {
crates/amun-canonical-codec/src/writer.rs:23:    pub fn write_bool(&mut self, v: bool) {
crates/amun-canonical-codec/src/writer.rs:26:    pub fn write_u16(&mut self, v: u16) {
crates/amun-canonical-codec/src/writer.rs:29:    pub fn write_u32(&mut self, v: u32) {
crates/amun-canonical-codec/src/writer.rs:32:    pub fn write_u64(&mut self, v: u64) {
crates/amun-canonical-codec/src/writer.rs:35:    pub fn write_bytes(&mut self, bytes: &[u8]) {
crates/amun-canonical-codec/src/writer.rs:43:    pub fn write_hash(&mut self, hash: &[u8; 32]) {
crates/amun-canonical-codec/src/writer.rs:46:    pub fn into_bytes(self) -> Vec<u8> {
crates/amun-canonical-codec/src/writer.rs:49:    pub fn as_bytes(&self) -> &[u8] {
crates/amun-canonical-codec/src/writer.rs:52:    pub fn len(&self) -> usize {
crates/amun-canonical-codec/src/writer.rs:55:    pub fn is_empty(&self) -> bool {
crates/amun-canonical-codec/src/writer.rs:5:impl Default for CanonicalWriter {
crates/amun-canonical-codec/src/writer.rs:6:    fn default() -> Self {
crates/amun-canonical-collections/src/lib.rs:103:    pub fn get(&self, key: &K) -> Option<&V> { self.inner.get(key) }
crates/amun-canonical-collections/src/lib.rs:104:    pub fn contains_key(&self, key: &K) -> bool { self.inner.contains_key(key) }
crates/amun-canonical-collections/src/lib.rs:105:    pub fn remove(&mut self, key: &K) -> Option<V> { self.inner.remove(key) }
crates/amun-canonical-collections/src/lib.rs:106:    pub fn len(&self) -> usize { self.inner.len() }
crates/amun-canonical-collections/src/lib.rs:107:    pub fn is_empty(&self) -> bool { self.inner.is_empty() }
crates/amun-canonical-collections/src/lib.rs:108:    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> { self.inner.iter() }
crates/amun-canonical-collections/src/lib.rs:109:    pub fn keys(&self) -> impl Iterator<Item = &K> { self.inner.keys() }
crates/amun-canonical-collections/src/lib.rs:110:    pub fn values(&self) -> impl Iterator<Item = &V> { self.inner.values() }
crates/amun-canonical-collections/src/lib.rs:111:    pub fn retain(&mut self, f: impl Fn(&K, &mut V) -> bool) { self.inner.retain(f); }
crates/amun-canonical-collections/src/lib.rs:112:    pub fn clear(&mut self) { self.inner.clear(); }
crates/amun-canonical-collections/src/lib.rs:117:    fn capacity(&self) -> usize { self.max_capacity }
crates/amun-canonical-collections/src/lib.rs:118:    fn remaining(&self) -> usize { self.max_capacity.saturating_sub(self.len()) }
crates/amun-canonical-collections/src/lib.rs:120:impl<K: Ord + CanonicalEncode, V: CanonicalEncode> ReplaySafe for CanonicalMap<K, V> {
crates/amun-canonical-collections/src/lib.rs:121:    fn canonical_root(&self) -> [u8; 32] {
crates/amun-canonical-collections/src/lib.rs:124:        h.finalize().into()
crates/amun-canonical-collections/src/lib.rs:128:    fn encode_canonical(&self, out: &mut Vec<u8>) { (self.len() as u64).encode_canonical(out); for (k, v) in &self.inner { k.encode_canonical(out); v.encode_canonical(out); } }
crates/amun-canonical-collections/src/lib.rs:134:pub struct CanonicalDeque<T> { inner: VecDeque<T>, max_capacity: usize }
crates/amun-canonical-collections/src/lib.rs:137:    pub fn new() -> Self { Self { inner: VecDeque::new(), max_capacity: DEFAULT_MAX_CAPACITY } }
crates/amun-canonical-collections/src/lib.rs:138:    pub fn try_with_capacity(max: usize) -> Result<Self, CollectionError> {
crates/amun-canonical-collections/src/lib.rs:143:    pub fn push_back(&mut self, value: T) -> Result<(), CollectionError> { if self.inner.len() >= self.max_capacity { return Err(CollectionError::CapacityExceeded); } self.inner.push_back(value); Ok(()) }
crates/amun-canonical-collections/src/lib.rs:144:    pub fn push_front(&mut self, value: T) -> Result<(), CollectionError> { if self.inner.len() >= self.max_capacity { return Err(CollectionError::CapacityExceeded); } self.inner.push_front(value); Ok(()) }
crates/amun-canonical-collections/src/lib.rs:145:    pub fn pop_front(&mut self) -> Option<T> { self.inner.pop_front() }
crates/amun-canonical-collections/src/lib.rs:146:    pub fn pop_back(&mut self) -> Option<T> { self.inner.pop_back() }
crates/amun-canonical-collections/src/lib.rs:147:    pub fn front(&self) -> Option<&T> { self.inner.front() }
crates/amun-canonical-collections/src/lib.rs:148:    pub fn back(&self) -> Option<&T> { self.inner.back() }
crates/amun-canonical-collections/src/lib.rs:149:    pub fn len(&self) -> usize { self.inner.len() }
crates/amun-canonical-collections/src/lib.rs:150:    pub fn is_empty(&self) -> bool { self.inner.is_empty() }
crates/amun-canonical-collections/src/lib.rs:151:    pub fn iter(&self) -> impl Iterator<Item = &T> { self.inner.iter() }
crates/amun-canonical-collections/src/lib.rs:152:    pub fn clear(&mut self) { self.inner.clear(); }
crates/amun-canonical-collections/src/lib.rs:157:    fn capacity(&self) -> usize { self.max_capacity }
crates/amun-canonical-collections/src/lib.rs:158:    fn remaining(&self) -> usize { self.max_capacity.saturating_sub(self.len()) }
crates/amun-canonical-collections/src/lib.rs:160:impl<T: CanonicalEncode> ReplaySafe for CanonicalDeque<T> {
crates/amun-canonical-collections/src/lib.rs:161:    fn canonical_root(&self) -> [u8; 32] {
crates/amun-canonical-collections/src/lib.rs:164:        h.finalize().into()
crates/amun-canonical-collections/src/lib.rs:168:    fn encode_canonical(&self, out: &mut Vec<u8>) { (self.len() as u64).encode_canonical(out); for item in &self.inner { item.encode_canonical(out); } }
crates/amun-canonical-collections/src/lib.rs:16:// ─── Constitutional Traits ─────────────────────────────────
crates/amun-canonical-collections/src/lib.rs:179:    #[test] fn test_replay_root_deterministic() { let mut a = CanonicalSet::new(); let mut b = CanonicalSet::new(); a.insert(1u64).unwrap(); a.insert(2u64).unwrap(); b.insert(2u64).unwrap(); b.insert(1u64).unwrap(); assert_eq!(a.canonical_root(), b.canonical_root()); }
crates/amun-canonical-collections/src/lib.rs:17:pub trait DeterministicCollection { fn is_deterministic(&self) -> bool { true } }
crates/amun-canonical-collections/src/lib.rs:18:pub trait ReplaySafe: DeterministicCollection + CanonicalEncode {
crates/amun-canonical-collections/src/lib.rs:19:    fn canonical_root(&self) -> [u8; 32];
crates/amun-canonical-collections/src/lib.rs:1://! Canonical Collections — deterministic, replay-safe container types.
crates/amun-canonical-collections/src/lib.rs:20:    fn verify_root(&self, expected: &[u8; 32]) -> bool { self.canonical_root() == *expected }
crates/amun-canonical-collections/src/lib.rs:21:    fn is_replay_stable(&self) -> bool { true }
crates/amun-canonical-collections/src/lib.rs:23:pub trait BoundedCollection {
crates/amun-canonical-collections/src/lib.rs:24:    fn capacity(&self) -> usize;
crates/amun-canonical-collections/src/lib.rs:25:    fn remaining(&self) -> usize;
crates/amun-canonical-collections/src/lib.rs:26:    fn is_full(&self) -> bool { self.remaining() == 0 }
crates/amun-canonical-collections/src/lib.rs:37:pub enum CollectionError {
crates/amun-canonical-collections/src/lib.rs:45:pub struct CanonicalSet<T: Ord> { inner: BTreeSet<T>, max_capacity: usize }
crates/amun-canonical-collections/src/lib.rs:48:    pub fn new() -> Self { Self { inner: BTreeSet::new(), max_capacity: DEFAULT_MAX_CAPACITY } }
crates/amun-canonical-collections/src/lib.rs:49:    pub fn try_with_capacity(max: usize) -> Result<Self, CollectionError> {
crates/amun-canonical-collections/src/lib.rs:4://! Constitutional guarantees:
crates/amun-canonical-collections/src/lib.rs:54:    pub fn insert(&mut self, value: T) -> Result<bool, CollectionError> {
crates/amun-canonical-collections/src/lib.rs:58:    pub fn contains(&self, value: &T) -> bool { self.inner.contains(value) }
crates/amun-canonical-collections/src/lib.rs:59:    pub fn remove(&mut self, value: &T) -> bool { self.inner.remove(value) }
crates/amun-canonical-collections/src/lib.rs:60:    pub fn len(&self) -> usize { self.inner.len() }
crates/amun-canonical-collections/src/lib.rs:61:    pub fn is_empty(&self) -> bool { self.inner.is_empty() }
crates/amun-canonical-collections/src/lib.rs:62:    pub fn iter(&self) -> impl Iterator<Item = &T> { self.inner.iter() }
crates/amun-canonical-collections/src/lib.rs:63:    pub fn retain(&mut self, f: impl Fn(&T) -> bool) { self.inner.retain(f); }
crates/amun-canonical-collections/src/lib.rs:64:    pub fn clear(&mut self) { self.inner.clear(); }
crates/amun-canonical-collections/src/lib.rs:69:    fn capacity(&self) -> usize { self.max_capacity }
crates/amun-canonical-collections/src/lib.rs:70:    fn remaining(&self) -> usize { self.max_capacity.saturating_sub(self.len()) }
crates/amun-canonical-collections/src/lib.rs:72:impl<T: Ord + CanonicalEncode> ReplaySafe for CanonicalSet<T> {
crates/amun-canonical-collections/src/lib.rs:73:    fn canonical_root(&self) -> [u8; 32] {
crates/amun-canonical-collections/src/lib.rs:76:        h.finalize().into()
crates/amun-canonical-collections/src/lib.rs:80:    fn encode_canonical(&self, out: &mut Vec<u8>) { (self.len() as u64).encode_canonical(out); for item in &self.inner { item.encode_canonical(out); } }
crates/amun-canonical-collections/src/lib.rs:84:    type Item = T; type IntoIter = alloc::collections::btree_set::IntoIter<T>;
crates/amun-canonical-collections/src/lib.rs:85:    fn into_iter(self) -> Self::IntoIter { self.inner.into_iter() }
crates/amun-canonical-collections/src/lib.rs:8://!   - REPLAY-SAFE: canonical_root() produces verifiable commitments
crates/amun-canonical-collections/src/lib.rs:90:pub struct CanonicalMap<K: Ord, V> { inner: BTreeMap<K, V>, max_capacity: usize }
crates/amun-canonical-collections/src/lib.rs:93:    pub fn new() -> Self { Self { inner: BTreeMap::new(), max_capacity: DEFAULT_MAX_CAPACITY } }
crates/amun-canonical-collections/src/lib.rs:94:    pub fn try_with_capacity(max: usize) -> Result<Self, CollectionError> {
crates/amun-canonical-collections/src/lib.rs:99:    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, CollectionError> {
crates/amun-canonical/src/decoder.rs:13:    pub fn read_u64(&mut self) -> Result<u64, CanonicalError> {
crates/amun-canonical/src/decoder.rs:23:    pub fn read_u32(&mut self) -> Result<u32, CanonicalError> {
crates/amun-canonical/src/decoder.rs:33:    pub fn read_bytes(&mut self) -> Result<&[u8], CanonicalError> {
crates/amun-canonical/src/decoder.rs:3:pub struct CanonicalDecoder<'a> {
crates/amun-canonical/src/decoder.rs:43:    pub fn read_u8(&mut self) -> Result<u8, CanonicalError> {
crates/amun-canonical/src/decoder.rs:9:    pub fn new(data: &'a [u8]) -> Self {
crates/amun-canonical/src/encoder.rs:10:    pub fn new(schema: SchemaVersion) -> Self {
crates/amun-canonical/src/encoder.rs:19:    pub fn write_u64(&mut self, v: u64) -> Result<(), CanonicalError> {
crates/amun-canonical/src/encoder.rs:25:    pub fn write_u32(&mut self, v: u32) -> Result<(), CanonicalError> {
crates/amun-canonical/src/encoder.rs:31:    pub fn write_u16(&mut self, v: u16) -> Result<(), CanonicalError> {
crates/amun-canonical/src/encoder.rs:37:    pub fn write_u8(&mut self, v: u8) -> Result<(), CanonicalError> {
crates/amun-canonical/src/encoder.rs:41:    pub fn write_bytes(&mut self, b: &[u8]) -> Result<(), CanonicalError> {
crates/amun-canonical/src/encoder.rs:4:pub struct CanonicalEncoder {
crates/amun-canonical/src/encoder.rs:51:    pub fn finish(&self) -> [u8; 32] {
crates/amun-canonical/src/encoder.rs:55:        let h = hasher.finalize();
crates/amun-canonical/src/encoder.rs:61:    pub fn as_bytes(&self) -> &[u8] {
crates/amun-canonical/src/encoder.rs:9:impl CanonicalEncoder {
crates/amun-canonical/src/enum_registry.rs:11:impl EnumRegistry {
crates/amun-canonical/src/enum_registry.rs:12:    pub fn new() -> Self {
crates/amun-canonical/src/enum_registry.rs:18:    pub fn register(
crates/amun-canonical/src/enum_registry.rs:1:pub struct EnumRegistry {
crates/amun-canonical/src/enum_registry.rs:35:    pub fn validate(&self, name: &str, variant: u8, version: u32) -> bool {
crates/amun-canonical/src/enum_registry.rs:42:impl Default for EnumRegistry {
crates/amun-canonical/src/enum_registry.rs:43:    fn default() -> Self {
crates/amun-canonical/src/enum_registry.rs:5:pub struct EnumEntry {
crates/amun-canonical/src/error.rs:2:pub enum CanonicalError {
crates/amun-canonical/src/float_ban.rs:18:    pub fn verify_type<T: 'static>() -> Result<(), &'static str> {
crates/amun-canonical/src/float_ban.rs:1:pub struct FloatBan;
crates/amun-canonical/src/float_ban.rs:3:impl FloatBan {
crates/amun-canonical/src/float_ban.rs:4:    pub fn verify_no_floats(code: &str) -> Result<(), Vec<&'static str>> {
crates/amun-canonical/src/schema.rs:2:pub enum SchemaVersion {
crates/amun-canonical/src/schema.rs:6:impl SchemaVersion {
crates/amun-canonical/src/schema.rs:7:    pub fn as_tag(&self) -> &[u8] {
crates/amun-canonical/src/sorter.rs:10:    pub fn sort_validators(validators: &mut Vec<([u8; 32], u64), 128>) {
crates/amun-canonical/src/sorter.rs:3:pub struct CanonicalSorter;
crates/amun-canonical/src/sorter.rs:5:impl CanonicalSorter {
crates/amun-canonical/src/sorter.rs:6:    pub fn sort_bytes(pairs: &mut Vec<(&[u8], &[u8]), 256>) {
crates/amun-chain-checkpoint/src/lib.rs:354:    fn n12a_checkpoint_hash_deterministic() {
crates/amun-chunked-snapshot/src/manifest.rs:15:    pub fn new(snapshot: &CanonicalSnapshot, chunks: &[SnapshotChunk]) -> Self {
crates/amun-codec/src/canonical_sort.rs:11:    fn write_sort_key(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/canonical_sort.rs:16:pub fn compare_by_canonical_bytes<T: CanonicalEncode>(a: &T, b: &T) -> core::cmp::Ordering {
crates/amun-codec/src/canonical_sort.rs:44:pub fn canonical_sort<T: CanonicalEncode>(items: &mut [T]) {
crates/amun-codec/src/canonical_sort.rs:6:pub trait CanonicalSortKey {
crates/amun-codec/src/canonical_sort.rs:7:    fn write_sort_key(&self, writer: &mut impl CanonicalWriter) -> WriteResult;
crates/amun-codec/src/containers.rs:45:pub fn encode_sequence<T: CanonicalEncode>(
crates/amun-codec/src/containers.rs:64:pub fn encode_set<T: CanonicalEncode + Ord>(
crates/amun-codec/src/containers.rs:95:pub fn encode_map<K: CanonicalEncode + Ord, V: CanonicalEncode>(
crates/amun-codec/src/decode.rs:108:impl CanonicalDecode for [u8; 48] {
crates/amun-codec/src/decode.rs:123:impl CanonicalDecode for [u8; 96] {
crates/amun-codec/src/decode.rs:140:        impl CanonicalDecode for $t {
crates/amun-codec/src/decode.rs:176:impl CanonicalDecode for amun_kernel_types::PublicHash32 {
crates/amun-codec/src/decode.rs:191:impl CanonicalDecode for amun_kernel_types::CommitmentHash32 {
crates/amun-codec/src/decode.rs:22:impl CanonicalDecode for u8 {
crates/amun-codec/src/decode.rs:35:impl CanonicalDecode for u16 {
crates/amun-codec/src/decode.rs:48:impl CanonicalDecode for u32 {
crates/amun-codec/src/decode.rs:63:impl CanonicalDecode for u64 {
crates/amun-codec/src/decode.rs:6:pub trait CanonicalDecode: Sized {
crates/amun-codec/src/decode.rs:78:impl CanonicalDecode for u128 {
crates/amun-codec/src/decode.rs:93:impl CanonicalDecode for [u8; 32] {
crates/amun-codec/src/encode.rs:103:impl CanonicalEncode for Amount {
crates/amun-codec/src/encode.rs:105:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/encode.rs:110:impl CanonicalEncode for ValidatorId {
crates/amun-codec/src/encode.rs:112:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/encode.rs:117:impl CanonicalEncode for PublicKey {
crates/amun-codec/src/encode.rs:119:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/encode.rs:124:impl CanonicalEncode for Signature {
crates/amun-codec/src/encode.rs:126:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/encode.rs:135:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/encode.rs:13:pub trait CanonicalEncode {
crates/amun-codec/src/encode.rs:140:impl CanonicalEncode for amun_kernel_types::PublicHash32 {
crates/amun-codec/src/encode.rs:142:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/encode.rs:147:impl CanonicalEncode for amun_kernel_types::CommitmentHash32 {
crates/amun-codec/src/encode.rs:149:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/encode.rs:15:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult;
crates/amun-codec/src/encode.rs:29:impl CanonicalEncode for u8 {
crates/amun-codec/src/encode.rs:31:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/encode.rs:36:impl CanonicalEncode for u16 {
crates/amun-codec/src/encode.rs:38:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/encode.rs:43:impl CanonicalEncode for u32 {
crates/amun-codec/src/encode.rs:45:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/encode.rs:50:impl CanonicalEncode for u64 {
crates/amun-codec/src/encode.rs:52:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/encode.rs:57:impl CanonicalEncode for u128 {
crates/amun-codec/src/encode.rs:59:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/encode.rs:64:impl CanonicalEncode for [u8; 32] {
crates/amun-codec/src/encode.rs:66:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/encode.rs:71:impl CanonicalEncode for [u8; 48] {
crates/amun-codec/src/encode.rs:73:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/encode.rs:78:impl CanonicalEncode for [u8; 96] {
crates/amun-codec/src/encode.rs:80:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/encode.rs:87:        impl CanonicalEncode for $t {
crates/amun-codec/src/encode.rs:89:            fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/hash.rs:12:    fn canonical_hash(&self) -> AmunResult<PublicHash32> {
crates/amun-codec/src/hash.rs:20:pub fn hash_value_streaming<T: CanonicalEncode>(value: &T) -> AmunResult<PublicHash32> {
crates/amun-codec/src/hash.rs:7:pub trait CanonicalHash {
crates/amun-codec/src/hash.rs:8:    fn canonical_hash(&self) -> AmunResult<PublicHash32>;
crates/amun-codec/src/tests.rs:198:fn test_domain_hash_deterministic() {
crates/amun-codec/src/transducer.rs:10:impl CanonicalTransducer {
crates/amun-codec/src/transducer.rs:11:    pub fn compare<A: CanonicalCursor, B: CanonicalCursor>(
crates/amun-codec/src/transducer.rs:40:impl CanonicalCursor for U64Cursor {
crates/amun-codec/src/transducer.rs:4:pub trait CanonicalCursor {
crates/amun-codec/src/transducer.rs:8:pub struct CanonicalTransducer;
crates/amun-codec/src/versioned.rs:35:    pub fn write(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-codec/src/writer.rs:115:                ConstitutionalFault::InvalidStateTransition,
crates/amun-codec/src/writer.rs:128:                ConstitutionalFault::InvalidStateTransition,
crates/amun-codec/src/writer.rs:8:pub trait CanonicalWriter {
crates/amun-consensus-execution/src/block_dag.rs:186:    pub fn update_canonical_spine(&mut self, finalized_block: [u8; 32]) {
crates/amun-consensus-execution/src/block_dag.rs:215:        // Invariant assertions
crates/amun-consensus-execution/src/block_dag.rs:232:        self.update_canonical_spine(finalized_block);
crates/amun-consensus-execution/src/block_dag.rs:290:    pub fn is_on_canonical_spine(&self, hash: &[u8; 32]) -> bool {
crates/amun-consensus-execution/src/fork_choice.rs:123:    pub fn canonical_tip(&self, dag: &BlockDAG) -> Option<[u8; 32]> {
crates/amun-consensus-integration/src/consensus_integrator.rs:191:    fn w17_finality_certificate_deterministic() {
crates/amun-consensus-integration/src/consensus_types.rs:12:    pub transitions: Vec<TransitionProof>,
crates/amun-consensus-integration/src/consensus_types.rs:2:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-consensus-law/src/invariants.rs:14:    pub fn single_finality_holds(finalized: &[(u64, [u8; 32])], f: u64, n: u64) -> bool {
crates/amun-consensus-law/src/invariants.rs:18:        for i in 0..finalized.len() {
crates/amun-consensus-law/src/invariants.rs:19:            for j in (i + 1)..finalized.len() {
crates/amun-consensus-law/src/invariants.rs:1:use amun_invariants::kernel::IrreducibleInvariants;
crates/amun-consensus-law/src/invariants.rs:20:                let (h1, b1) = finalized[i];
crates/amun-consensus-law/src/invariants.rs:21:                let (h2, b2) = finalized[j];
crates/amun-consensus-law/src/invariants.rs:30:    pub fn eventual_progress_possible(active_validators: usize, threshold: usize) -> bool {
crates/amun-consensus-law/src/invariants.rs:34:    pub fn kernel_bound() -> [u8; 32] {
crates/amun-consensus-law/src/invariants.rs:35:        IrreducibleInvariants::kernel_hash()
crates/amun-consensus-law/src/invariants.rs:3:pub struct ConsensusInvariants;
crates/amun-consensus-law/src/invariants.rs:5:impl ConsensusInvariants {
crates/amun-consensus-law/src/invariants.rs:6:    pub fn single_truth_holds(state_roots: &[[u8; 32]]) -> bool {
crates/amun-consensus-law/src/lib.rs:8:pub use invariants::ConsensusInvariants;
crates/amun-consensus-law/src/lib.rs:9:pub use safety::SafetyAxioms;
crates/amun-consensus-law/src/safety.rs:1:pub struct SafetyAxioms;
crates/amun-consensus-law/src/safety.rs:3:impl SafetyAxioms {
crates/amun-consensus-network/src/execution_commitment.rs:142:    fn n109_8_compute_execution_root_is_deterministic() {
crates/amun-consensus-network/src/messages.rs:102:        if !self.verify_consistency() {
crates/amun-consensus-network/src/messages.rs:380:        assert!(!qc.verify_consistency());
crates/amun-consensus-network/src/messages.rs:440:    fn n104_1_deterministic_hash() {
crates/amun-consensus-network/src/messages.rs:85:    pub fn verify_consistency(&self) -> bool {
crates/amun-consensus-network/src/messages.rs:92:        self.verify_quorum() && self.verify_consistency()
crates/amun-consensus-network/src/slashing_certificate.rs:257:    fn n110_2_certificate_hash_is_deterministic() {
crates/amun-consensus-network/src/slashing_ledger.rs:1:// N119 — Deterministic Slashing Ledger & Replay Protection
crates/amun-consensus-network/src/slashing_merkle.rs:121:    fn n120_1_larger_tree_is_deterministic() {
crates/amun-consensus-network/src/slashing_merkle.rs:72:    fn n120_1_single_leaf_is_deterministic() {
crates/amun-consensus-network/src/slashing_state.rs:117:        assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/src/slashing_state.rs:11:/// Wraps the deterministic ledger and exposes a root for consensus commitment.
crates/amun-consensus-network/src/slashing_state.rs:150:        assert!(state.verify_consistency().is_ok());
crates/amun-consensus-network/src/slashing_state.rs:42:    pub fn verify_consistency(&self) -> Result<(), String> {
crates/amun-consensus-network/src/slashing_state.rs:98:        assert!(state.verify_consistency().is_ok());
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
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:46:        state.verify_consistency().is_ok(),
crates/amun-consensus-network/tests/n121_5_snapshot_and_audit.rs:82:    assert!(restored.verify_consistency().is_ok());
crates/amun-consensus-types/src/bitmap.rs:36:impl CanonicalEncode for SignerBitmap {
crates/amun-consensus-types/src/bitmap.rs:38:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-consensus-types/src/bitmap.rs:43:impl CanonicalDecode for SignerBitmap {
crates/amun-consensus-types/src/errors.rs:19:            ConstitutionalFault::InvalidStateTransition,
crates/amun-consensus-types/src/message.rs:52:impl CanonicalEncode for ConsensusMessage {
crates/amun-consensus-types/src/message.rs:54:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-consensus-types/src/message.rs:66:impl CanonicalDecode for ConsensusMessage {
crates/amun-consensus-types/src/phase.rs:55:impl CanonicalEncode for ConsensusPhase {
crates/amun-consensus-types/src/phase.rs:57:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-consensus-types/src/phase.rs:62:impl CanonicalDecode for ConsensusPhase {
crates/amun-consensus-types/src/qc.rs:25:impl CanonicalEncode for QuorumCertificate {
crates/amun-consensus-types/src/qc.rs:27:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-consensus-types/src/qc.rs:38:impl CanonicalDecode for QuorumCertificate {
crates/amun-consensus-types/src/round.rs:27:impl CanonicalEncode for ConsensusRound {
crates/amun-consensus-types/src/round.rs:29:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-consensus-types/src/round.rs:34:impl CanonicalDecode for ConsensusRound {
crates/amun-consensus-types/src/validator.rs:17:impl CanonicalEncode for ValidatorIndex {
crates/amun-consensus-types/src/validator.rs:19:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-consensus-types/src/validator.rs:24:impl CanonicalDecode for ValidatorIndex {
crates/amun-consensus-types/src/vote.rs:43:impl CanonicalEncode for Vote {
crates/amun-consensus-types/src/vote.rs:45:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-consensus-types/src/vote.rs:56:impl CanonicalDecode for Vote {
crates/amun-consensus/amun_consensus/src/block/mod.rs:104:    pub fn sort_canonical(&mut self, block_height: u64) {
crates/amun-consensus/amun_consensus/src/block/mod.rs:108:    pub fn is_canonically_ordered(&self, block_height: u64) -> bool {
crates/amun-consensus/amun_consensus/src/block/mod.rs:57:    pub fn canonical_bytes(&self) -> Vec<u8> {
crates/amun-consensus/amun_consensus/src/block/mod.rs:89:    pub fn canonical_bytes(&self) -> Vec<u8> {
crates/amun-consensus/src/round_state_machine.rs:133:        //   - StateTransitionValidity (pre_state != post_state && valid)
crates/amun-consensus/src/round_state_machine.rs:143:                true, // replay_deterministic
crates/amun-constitution-builder/src/canonical_bytes.rs:3:// Licensed under the GNU AGPLv3 with Constitutional Sovereignty Addendum.
crates/amun-constitution-builder/src/canonical_bytes.rs:5:pub trait CanonicalSerialize {
crates/amun-constitution-builder/src/canonical_bytes.rs:6:    fn canonical_bytes(&self) -> Vec<u8>;
crates/amun-constitution-builder/src/certificate.rs:59:impl CanonicalEmit for FreezeCertificate {
crates/amun-constitution-builder/src/certificate.rs:60:    fn emit_canonical(&self) -> String {
crates/amun-constitution-builder/src/certificate.rs:81:impl CanonicalSerialize for FreezeCertificate {
crates/amun-constitution-builder/src/certificate.rs:82:    fn canonical_bytes(&self) -> Vec<u8> {
crates/amun-constitution-builder/src/digest.rs:8:pub trait ArtifactDigest: CanonicalSerialize {
crates/amun-constitution-builder/src/emitter.rs:5:pub trait CanonicalEmit {
crates/amun-constitution-builder/src/emitter.rs:6:    fn emit_canonical(&self) -> String;
crates/amun-constitution-builder/src/federation.rs:105:impl CanonicalSerialize for FederationArtifact {
crates/amun-constitution-builder/src/federation.rs:106:    fn canonical_bytes(&self) -> Vec<u8> {
crates/amun-constitution-builder/src/federation.rs:70:impl CanonicalEmit for FederationArtifact {
crates/amun-constitution-builder/src/federation.rs:71:    fn emit_canonical(&self) -> String {
crates/amun-constitution-builder/src/manifest.rs:51:impl CanonicalEmit for ConstitutionalManifest {
crates/amun-constitution-builder/src/manifest.rs:52:    fn emit_canonical(&self) -> String {
crates/amun-constitution-builder/src/manifest.rs:82:impl CanonicalSerialize for ConstitutionalManifest {
crates/amun-constitution-builder/src/manifest.rs:83:    fn canonical_bytes(&self) -> Vec<u8> {
crates/amun-constitution-builder/src/normalize.rs:5:pub struct DeterministicNormalizer;
crates/amun-constitution-builder/src/normalize.rs:7:impl DeterministicNormalizer {
crates/amun-constitution-builder/src/treaty.rs:49:impl CanonicalEmit for TreatyArtifact {
crates/amun-constitution-builder/src/treaty.rs:50:    fn emit_canonical(&self) -> String {
crates/amun-constitution-builder/src/treaty.rs:82:impl CanonicalSerialize for TreatyArtifact {
crates/amun-constitution-builder/src/treaty.rs:83:    fn canonical_bytes(&self) -> Vec<u8> {
crates/amun-constitution-builder/src/verify.rs:11:    pub fn verify_replay<T: CanonicalSerialize + PartialEq>(
crates/amun-constitution-core/src/ordering.rs:3:pub fn canonical_validator_order(ids: &[u64]) -> bool {
crates/amun-constitution/src/activation.rs:1:use crate::canonical_form::ConstitutionDomain;
crates/amun-constitution/src/canonical_form.rs:10:pub struct ConstitutionalCanonicalForm {
crates/amun-constitution/src/canonical_form.rs:17:    pub transition_algebra_version: u16,
crates/amun-constitution/src/canonical_form.rs:1:// Constitutional Canonical Form (CCF) — the protocol specification.
crates/amun-constitution/src/canonical_form.rs:20:    pub parent_constitution_hash: Option<Hash<ConstitutionDomain>>,
crates/amun-constitution/src/canonical_form.rs:23:impl ConstitutionalCanonicalForm {
crates/amun-constitution/src/canonical_form.rs:24:    pub fn compute_hash(&self) -> Hash<ConstitutionDomain> {
crates/amun-constitution/src/canonical_form.rs:34:        hasher.update(&self.transition_algebra_version.to_le_bytes());
crates/amun-constitution/src/canonical_form.rs:37:        Hash::new(hasher.finalize().into())
crates/amun-constitution/src/canonical_form.rs:41:pub const CURRENT_CCF: ConstitutionalCanonicalForm = ConstitutionalCanonicalForm {
crates/amun-constitution/src/canonical_form.rs:48:    transition_algebra_version: 1,
crates/amun-constitution/src/canonical_form.rs:7:pub struct ConstitutionDomain;
crates/amun-constitution/src/deterministic_execution.rs:10:impl DeterministicExecution for [u8; 32] {}
crates/amun-constitution/src/deterministic_execution.rs:11:impl DeterministicExecution for [u8; 64] {}
crates/amun-constitution/src/deterministic_execution.rs:4:pub trait DeterministicExecution: Sized {}
crates/amun-constitution/src/deterministic_execution.rs:6:impl DeterministicExecution for u8 {}
crates/amun-constitution/src/deterministic_execution.rs:7:impl DeterministicExecution for u16 {}
crates/amun-constitution/src/deterministic_execution.rs:8:impl DeterministicExecution for u32 {}
crates/amun-constitution/src/deterministic_execution.rs:9:impl DeterministicExecution for u64 {}
crates/amun-constitution/src/fork_choice.rs:1:// Formal fork-choice function.
crates/amun-constitution/src/quorum_transition.rs:1:// Quorum transition safety theorem parameters.
crates/amun-constitution/src/replay.rs:53:impl CanonicalEncode for ReplayContext {
crates/amun-constitution/src/replay.rs:55:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-constitution/src/replay.rs:61:impl CanonicalDecode for ReplayContext {
crates/amun-constitutional-authority-semantics/src/capability.rs:1:use amun_constitution_builder::canonical_bytes::CanonicalSerialize;
crates/amun-constitutional-authority-semantics/src/capability.rs:60:impl CanonicalSerialize for AuthorityCapability {
crates/amun-constitutional-authority-semantics/src/capability.rs:61:    fn canonical_bytes(&self) -> Vec<u8> {
crates/amun-constitutional-authority/src/certificate.rs:141:impl CanonicalSerialize for ConstitutionalCertificate {
crates/amun-constitutional-authority/src/certificate.rs:142:    fn canonical_bytes(&self) -> Vec<u8> {
crates/amun-constitutional-authority/src/certificate.rs:5:use amun_constitution_builder::{canonical_bytes::CanonicalSerialize, digest::ArtifactDigest};
crates/amun-constitutional-authority/tests/authority_tests.rs:126:fn revocation_registry_is_deterministic() {
crates/amun-constitutional-block/src/lib.rs:75:use amun_constitutional_state::StateTransitionRecord;
crates/amun-constitutional-block/src/lib.rs:89:    records: &[StateTransitionRecord],
crates/amun-constitutional-block/tests/block_tests.rs:33:fn test_block_hash_deterministic() {
crates/amun-constitutional-commitments/tests/commitment_tests.rs:16:fn domain_roots_are_deterministic() {
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:118:    pub replay_deterministic: bool,
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:160:            replay_deterministic: replay.replay_deterministic,
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:191:            replay_deterministic: true,
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:204:        assert!(evidence.replay_deterministic);
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:20:    pub replay_deterministic: bool,
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:45:        replay_deterministic: bool,
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:57:            replay_deterministic,
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:80:    pub fn from_replay(replay_deterministic: bool) -> ReplayEvidence {
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:82:            replay_deterministic,
crates/amun-constitutional-enforcement/src/evidence_providers.rs:102:        replay.replay_deterministic,
crates/amun-constitutional-enforcement/src/evidence_providers.rs:127:        assert!(evidence.replay_deterministic);
crates/amun-constitutional-enforcement/src/evidence_providers.rs:133:        assert!(!evidence.replay_deterministic);
crates/amun-constitutional-enforcement/src/evidence_providers.rs:149:        assert!(evidence.replay_deterministic);
crates/amun-constitutional-enforcement/src/evidence_providers.rs:48:            replay_deterministic: qc_verified && state_root_valid && transition_valid,
crates/amun-constitutional-enforcement/src/evidence_records.rs:204:    fn n128_replay_evidence_deterministic() {
crates/amun-constitutional-enforcement/src/evidence_records.rs:90:            deterministic: original == replay,
crates/amun-constitutional-enforcement/src/lib.rs:112:                ConstitutionalLaw::StateTransitionValidity,
crates/amun-constitutional-enforcement/src/lib.rs:131:        replay_deterministic: bool,
crates/amun-constitutional-enforcement/src/lib.rs:170:                replay_deterministic,
crates/amun-constitutional-enforcement/src/lib.rs:179:                &ConstitutionalLaw::StateTransitionValidity,
crates/amun-constitutional-enforcement/src/lib.rs:65:    StateTransitionValidity,
crates/amun-constitutional-enforcement/src/lib.rs:93:            evidence.replay_deterministic,
crates/amun-constitutional-enforcement/src/proof_engine.rs:194:                law: ConstitutionalLaw::StateTransitionValidity,
crates/amun-constitutional-enforcement/src/state_transition.rs:111:    let result = StateTransitionResult::new(
crates/amun-constitutional-enforcement/src/state_transition.rs:11:pub struct StateTransitionResult {
crates/amun-constitutional-enforcement/src/state_transition.rs:178:                    .any(|v| v.law == ConstitutionalLaw::StateTransitionValidity));
crates/amun-constitutional-enforcement/src/state_transition.rs:26:impl StateTransitionResult {
crates/amun-constitutional-enforcement/src/state_transition.rs:3:// Verifies that state transitions are deterministic and
crates/amun-constitutional-enforcement/src/state_transition.rs:70:                law: ConstitutionalLaw::StateTransitionValidity,
crates/amun-constitutional-geometry/src/emergent_horizons.rs:25:    /// Invariant mass creates an inescapable region
crates/amun-constitutional-geometry/src/emergent_horizons.rs:26:    InvariantSingularity,
crates/amun-constitutional-geometry/src/flow_dynamics.rs:24:    InvariantForce {
crates/amun-constitutional-geometry/src/flow_dynamics.rs:61:                ConstitutionalForce::InvariantForce { strength, .. } => {
crates/amun-constitutional-geometry/src/metric_tensor.rs:49:    pub fn apply_gravitational_mass(&mut self, invariant_mass: f64, at_dimension: usize) {
crates/amun-constitutional-geometry/src/stability.rs:48:    InvariantAttractor,
crates/amun-constitutional-governance/src/capability.rs:5:use amun_constitution_builder::{canonical_bytes::CanonicalSerialize, digest::ArtifactDigest};
crates/amun-constitutional-governance/src/capability.rs:72:impl CanonicalSerialize for Capability {
crates/amun-constitutional-governance/src/capability.rs:73:    fn canonical_bytes(&self) -> Vec<u8> {
crates/amun-constitutional-governance/src/lib.rs:15://! model formal and replay-verifiable.
crates/amun-constitutional-governance/src/voting.rs:64:impl CanonicalSerialize for Proposal {
crates/amun-constitutional-governance/src/voting.rs:65:    fn canonical_bytes(&self) -> Vec<u8> {
crates/amun-constitutional-governance/src/voting.rs:6:use amun_constitution_builder::canonical_bytes::CanonicalSerialize;
crates/amun-constitutional-governance/tests/governance_tests.rs:105:fn tally_is_deterministic() {
crates/amun-constitutional-integration/src/lib.rs:107:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:117:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:127:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:137:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:147:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:159:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:173:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:17:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:186:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:199:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:212:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:225:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:239:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:254:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:267:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:27:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:281:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:294:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:355:            let obligations: Vec<ProofObligation> = registry
crates/amun-constitutional-integration/src/lib.rs:370:            let obligations: Vec<ProofObligation> = registry
crates/amun-constitutional-integration/src/lib.rs:37:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:47:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:4:    ObligationResult, ObligationResultStatus, ObligationSeverity, ProofObligation,
crates/amun-constitutional-integration/src/lib.rs:57:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:60:            "Replay execution is deterministic",
crates/amun-constitutional-integration/src/lib.rs:67:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:77:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:87:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:97:        reg.register(ProofObligation::new(
crates/amun-constitutional-kernel/src/receipt.rs:5:use amun_constitution_builder::{canonical_bytes::CanonicalSerialize, digest::ArtifactDigest};
crates/amun-constitutional-kernel/src/receipt.rs:61:impl CanonicalSerialize for ExecutionReceipt {
crates/amun-constitutional-kernel/src/receipt.rs:62:    fn canonical_bytes(&self) -> Vec<u8> {
crates/amun-constitutional-proof/src/evidence_type.rs:15:    FormalProofEvidence,
crates/amun-constitutional-proof/src/lib.rs:1365:        reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:1430:        reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:1495:        reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:1532:            reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:1643:            reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:1737:            reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:173:    // --- ProofObligation tests (S1) ---
crates/amun-constitutional-proof/src/lib.rs:178:        let obl = ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:198:        let obl = ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:216:        let obl = ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:231:        let obl = ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:242:        let obl2: ProofObligation = serde_json::from_str(&json).unwrap();
crates/amun-constitutional-proof/src/lib.rs:258:        let obl: ProofObligation = serde_json::from_str(json).unwrap();
crates/amun-constitutional-proof/src/lib.rs:352:    fn simple_obl(id: ObligationId) -> ProofObligation {
crates/amun-constitutional-proof/src/lib.rs:353:        ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:396:        let obl = ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:417:        reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:426:        reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:443:        reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:452:        reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:473:            reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:500:            reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:518:            reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:735:    fn make_obl(id: ObligationId, severity: ObligationSeverity) -> ProofObligation {
crates/amun-constitutional-proof/src/lib.rs:736:        ProofObligation::new(
crates/amun-constitutional-proof/src/obligation_registry.rs:10:    obligations: BTreeMap<ObligationId, ProofObligation>,
crates/amun-constitutional-proof/src/obligation_registry.rs:24:    pub fn register(&mut self, obligation: ProofObligation) -> Result<(), RegistryError> {
crates/amun-constitutional-proof/src/obligation_registry.rs:5:    ProofObligation, RegistryError,
crates/amun-constitutional-proof/src/obligation_registry.rs:72:    pub fn get(&self, id: &ObligationId) -> Option<&ProofObligation> {
crates/amun-constitutional-proof/src/obligation_registry.rs:76:    pub fn all_obligations(&self) -> impl Iterator<Item = &ProofObligation> {
crates/amun-constitutional-proof/src/obligation_registry.rs:80:    pub fn by_namespace(&self, ns: ObligationSeverity) -> Vec<&ProofObligation> {
crates/amun-constitutional-proof/src/obligation_registry.rs:87:    pub fn by_severity(&self, severity: ObligationSeverity) -> Vec<&ProofObligation> {
crates/amun-constitutional-proof/src/obligation_registry.rs:94:    pub fn by_phase(&self, phase: &str) -> Vec<&ProofObligation> {
crates/amun-constitutional-proof/src/proof_obligation.rs:11:pub struct ProofObligation {
crates/amun-constitutional-proof/src/proof_obligation.rs:24:impl ProofObligation {
crates/amun-constitutional-proof/src/verdict_evaluator.rs:27:        obligations: &[ProofObligation],
crates/amun-constitutional-proof/src/verdict_evaluator.rs:48:        obligations: &[ProofObligation],
crates/amun-constitutional-proof/src/verdict_evaluator.rs:4:    ProofObligation, VerdictResult,
crates/amun-constitutional-runtime/src/block_validator.rs:2:use amun_invariant_engine::invariant_types::InvariantDeclaration;
crates/amun-constitutional-runtime/src/block_validator.rs:32:        invariants: &[InvariantDeclaration],
crates/amun-constitutional-runtime/src/certificate_chain.rs:103:    use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-constitutional-runtime/src/certificate_chain.rs:126:        let transitions = vec![TransitionProof::new(
crates/amun-constitutional-runtime/src/certificate_chain.rs:162:    fn n53_chain_deterministic_root() {
crates/amun-constitutional-runtime/src/finality_certificate.rs:109:    pub fn compute_evidence_root(transitions: &[TransitionProof]) -> [u8; 32] {
crates/amun-constitutional-runtime/src/finality_certificate.rs:117:    pub fn compute_pccv_root(transitions: &[TransitionProof]) -> [u8; 32] {
crates/amun-constitutional-runtime/src/finality_certificate.rs:173:        let transitions = vec![TransitionProof::new(
crates/amun-constitutional-runtime/src/finality_certificate.rs:17:    pub transitions: Vec<TransitionProof>,
crates/amun-constitutional-runtime/src/finality_certificate.rs:198:    fn n52_certificate_deterministic() {
crates/amun-constitutional-runtime/src/finality_certificate.rs:1:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-constitutional-runtime/src/finality_certificate.rs:25:        transitions: Vec<TransitionProof>,
crates/amun-constitutional-runtime/src/history_root.rs:118:        let transitions = vec![TransitionProof::new(
crates/amun-constitutional-runtime/src/history_root.rs:160:    fn n54_history_root_deterministic() {
crates/amun-constitutional-runtime/src/history_root.rs:95:    use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:104:            let proof = TransitionProof::new(
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:10:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:135:        // Phase 5: Invariant Evaluation
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:136:        let (_invariant_results, invariant_evidence) = InvariantEngine::evaluate(
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:149:        // Phase 6: Build TransitionProof via ProofBuilder
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:163:        use amun_pccv::transition_proof_engine::TransitionProofEngine as PCCVEngine;
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:195:            let proof = TransitionProof::new(
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:20:        transition_proof: TransitionProof,
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:26:        transition_proof: TransitionProof,
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:36:    /// → Invariants → Evidence → TransitionProof → PCCV → Archive
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:41:        invariants: &[InvariantDeclaration],
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:4:use amun_invariant_engine::invariant_engine::InvariantEngine;
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:5:use amun_invariant_engine::invariant_types::InvariantDeclaration;
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:76:            let proof = TransitionProof::new(
crates/amun-constitutional-semantics/src/lib.rs:118:    #[test] fn test_witness_normalization_deterministic() { let w = vec![(ReplayDomain::Consensus,2,[0x02;32]),(ReplayDomain::Consensus,1,[0x01;32])]; assert_eq!(WitnessNormalization::normalize(&w).normalization_root, WitnessNormalization::normalize(&w).normalization_root); }
crates/amun-constitutional-state/src/lib.rs:13:/// A record of a state transition for deterministic replay.
crates/amun-constitutional-state/src/lib.rs:145:    fn test_canonical_key() {
crates/amun-constitutional-state/src/lib.rs:147:        let key = ConstitutionalStateRuntime::canonical_key(keys::TRANSITION, &id);
crates/amun-constitutional-state/src/lib.rs:15:pub struct StateTransitionRecord {
crates/amun-constitutional-state/src/lib.rs:236:    pub fn verify(&self, records: &[StateTransitionRecord]) -> bool {
crates/amun-constitutional-state/src/lib.rs:23:    journal: Vec<StateTransitionRecord>,
crates/amun-constitutional-state/src/lib.rs:306:    fn n1_hash_deterministic() {
crates/amun-constitutional-state/src/lib.rs:46:    pub fn canonical_key(prefix: &[u8], id: &[u8; 32]) -> Vec<u8> {
crates/amun-constitutional-state/src/lib.rs:69:        let key = Self::canonical_key(keys::TRANSITION, transition_id);
crates/amun-constitutional-state/src/lib.rs:71:        self.journal.push(StateTransitionRecord {
crates/amun-constitutional-state/src/lib.rs:78:    pub fn journal(&self) -> &[StateTransitionRecord] {
crates/amun-constitutional-state/src/lib.rs:83:    pub fn replay(records: &[StateTransitionRecord]) -> Self {
crates/amun-constitutional/src/architectural_invariants.rs:103:/// INVARIANT 7: Replay-Derived State Identity
crates/amun-constitutional/src/architectural_invariants.rs:105:/// Constitutional state identity is derived exclusively from
crates/amun-constitutional/src/architectural_invariants.rs:106:/// deterministic replay lineage and attested transcript scope,
crates/amun-constitutional/src/architectural_invariants.rs:109:/// CONSEQUENCE: State roots are attestations of replay outcomes,
crates/amun-constitutional/src/architectural_invariants.rs:111:/// the same replay lineage produced it — not the same database
crates/amun-constitutional/src/architectural_invariants.rs:114:    "State identity is replay-derived, not storage-derived.";
crates/amun-constitutional/src/architectural_invariants.rs:118:/// Snapshot restoration validity is derived from replay lineage,
crates/amun-constitutional/src/architectural_invariants.rs:11:/// replay locality, and admissibility are all local within a
crates/amun-constitutional/src/architectural_invariants.rs:131:/// Restoration does not create a new replay lineage. It continues
crates/amun-constitutional/src/architectural_invariants.rs:133:/// The restored execution is a CONTINUATION of the original replay,
crates/amun-constitutional/src/architectural_invariants.rs:136:/// CONSEQUENCE: After restoration, the replay journal continues
crates/amun-constitutional/src/architectural_invariants.rs:138:/// (context → boundary → evidence → commitment → receipt) remains
crates/amun-constitutional/src/architectural_invariants.rs:143:/// INVARIANT 10: Constitutional Causal Validity
crates/amun-constitutional/src/architectural_invariants.rs:145:/// Constitutional validity is causally derived, not temporally derived.
crates/amun-constitutional/src/architectural_invariants.rs:153:    "Constitutional validity is causally derived, not temporally derived.";
crates/amun-constitutional/src/architectural_invariants.rs:155:/// INVARIANT 11: Constitutional Proof Sufficiency
crates/amun-constitutional/src/architectural_invariants.rs:157:/// Constitutional admissibility requires sufficient witnesses, not complete
crates/amun-constitutional/src/architectural_invariants.rs:180:/// INVARIANT 13: Constitutional-Operational Domain Separation
crates/amun-constitutional/src/architectural_invariants.rs:183:/// object. Constitutional hashing and operational hashing are separate
crates/amun-constitutional/src/architectural_invariants.rs:185:/// accidental replay equivalence, and operational proof leakage.
crates/amun-constitutional/src/architectural_invariants.rs:187:    "Constitutional and operational hash domains are separate namespaces.";
crates/amun-constitutional/src/architectural_invariants.rs:1://! Architectural Invariants — constitutional laws of the kernel.
crates/amun-constitutional/src/architectural_invariants.rs:204:/// The runtime executes and produces artifacts; the constitutional kernel
crates/amun-constitutional/src/architectural_invariants.rs:234:/// CONSEQUENCE: Constitutional truth must be derivable identically
crates/amun-constitutional/src/architectural_invariants.rs:24:/// A ReplayCertificate attests replay ADMISSIBILITY, not
crates/amun-constitutional/src/architectural_invariants.rs:250:/// Constitutional derivability alone defines semantic legitimacy.
crates/amun-constitutional/src/architectural_invariants.rs:26:/// that the state transition achieved consensus or finality.
crates/amun-constitutional/src/architectural_invariants.rs:280:/// INVARIANT 22: Proof Routing Non-Influence
crates/amun-constitutional/src/architectural_invariants.rs:282:/// Proof routing must never influence constitutional admissibility.
crates/amun-constitutional/src/architectural_invariants.rs:28:/// CONSEQUENCE: Certificates are replay witnesses, not
crates/amun-constitutional/src/architectural_invariants.rs:291:    "Proof routing must never influence constitutional admissibility.";
crates/amun-constitutional/src/architectural_invariants.rs:29:/// consensus proofs. Finality is a consensus concern layered
crates/amun-constitutional/src/architectural_invariants.rs:324:/// Constitutional evolution must remain derivationally constrained across
crates/amun-constitutional/src/architectural_invariants.rs:32:    "Certificate attests admissibility, not finality. Finality is above the kernel.";
crates/amun-constitutional/src/architectural_invariants.rs:345:/// Constitutional truth is not for sale.
crates/amun-constitutional/src/architectural_invariants.rs:38:/// constitutional governance action that invalidates ALL
crates/amun-constitutional/src/architectural_invariants.rs:392:/// Constitutional translation must preserve sovereignty attribution.
crates/amun-constitutional/src/architectural_invariants.rs:421:    fn test_all_invariants_documented() {
crates/amun-constitutional/src/architectural_invariants.rs:424:            assert!(!invariant.is_empty(), "Invariant must be documented");
crates/amun-constitutional/src/architectural_invariants.rs:429:    fn test_no_duplicate_invariants() {
crates/amun-constitutional/src/architectural_invariants.rs:439:    fn test_invariant_count() {
crates/amun-constitutional/src/architectural_invariants.rs:80:    "Constitutional artifacts are immutable. Identity = hash.";
crates/amun-constitutional/src/canonical_serialize.rs:109:pub fn read_bytes(bytes: &[u8], pos: &mut usize) -> Option<Vec<u8>> {
crates/amun-constitutional/src/canonical_serialize.rs:120:pub fn read_hash(bytes: &[u8], pos: &mut usize) -> Option<[u8; 32]> {
crates/amun-constitutional/src/canonical_serialize.rs:131:pub fn read_optional_hash(bytes: &[u8], pos: &mut usize) -> Option<Option<[u8; 32]>> {
crates/amun-constitutional/src/canonical_serialize.rs:145:    fn test_u64_roundtrip() {
crates/amun-constitutional/src/canonical_serialize.rs:153:    fn test_bytes_roundtrip() {
crates/amun-constitutional/src/canonical_serialize.rs:15:pub trait CanonicalEncode {
crates/amun-constitutional/src/canonical_serialize.rs:164:    fn test_optional_hash_some() {
crates/amun-constitutional/src/canonical_serialize.rs:172:    fn test_optional_hash_none() {
crates/amun-constitutional/src/canonical_serialize.rs:17:    fn canonical_encode(&self) -> Vec<u8>;
crates/amun-constitutional/src/canonical_serialize.rs:180:    fn test_deterministic_output() {
crates/amun-constitutional/src/canonical_serialize.rs:21:pub trait CanonicalDecode: Sized {
crates/amun-constitutional/src/canonical_serialize.rs:23:    fn canonical_decode(bytes: &[u8]) -> Option<Self>;
crates/amun-constitutional/src/canonical_serialize.rs:27:pub fn write_u64(buf: &mut Vec<u8>, v: u64) {
crates/amun-constitutional/src/canonical_serialize.rs:32:pub fn write_u32(buf: &mut Vec<u8>, v: u32) {
crates/amun-constitutional/src/canonical_serialize.rs:37:pub fn write_u16(buf: &mut Vec<u8>, v: u16) {
crates/amun-constitutional/src/canonical_serialize.rs:42:pub fn write_u8(buf: &mut Vec<u8>, v: u8) {
crates/amun-constitutional/src/canonical_serialize.rs:47:pub fn write_bytes(buf: &mut Vec<u8>, bytes: &[u8]) {
crates/amun-constitutional/src/canonical_serialize.rs:53:pub fn write_hash(buf: &mut Vec<u8>, hash: &[u8; 32]) {
crates/amun-constitutional/src/canonical_serialize.rs:58:pub fn write_optional_hash(buf: &mut Vec<u8>, hash: Option<&[u8; 32]>) {
crates/amun-constitutional/src/canonical_serialize.rs:69:pub fn read_u64(bytes: &[u8], pos: &mut usize) -> Option<u64> {
crates/amun-constitutional/src/canonical_serialize.rs:79:pub fn read_u32(bytes: &[u8], pos: &mut usize) -> Option<u32> {
crates/amun-constitutional/src/canonical_serialize.rs:89:pub fn read_u16(bytes: &[u8], pos: &mut usize) -> Option<u16> {
crates/amun-constitutional/src/canonical_serialize.rs:99:pub fn read_u8(bytes: &[u8], pos: &mut usize) -> Option<u8> {
crates/amun-constitutional/src/canonical_witness.rs:100:            entry(WitnessType::AuditDependency, [0x04; 32]),
crates/amun-constitutional/src/canonical_witness.rs:104:        assert_eq!(entries[0].witness_type, WitnessType::HardDependency);
crates/amun-constitutional/src/canonical_witness.rs:105:        assert_eq!(entries[3].witness_type, WitnessType::CompressionElidable);
crates/amun-constitutional/src/canonical_witness.rs:109:    fn test_canonical_order_same_type_lexicographic() {
crates/amun-constitutional/src/canonical_witness.rs:10://!   1. WitnessType priority (HardDependency first, then Supporting, Audit, Elidable)
crates/amun-constitutional/src/canonical_witness.rs:111:            entry(WitnessType::HardDependency, [0xCC; 32]),
crates/amun-constitutional/src/canonical_witness.rs:112:            entry(WitnessType::HardDependency, [0xAA; 32]),
crates/amun-constitutional/src/canonical_witness.rs:113:            entry(WitnessType::HardDependency, [0xBB; 32]),
crates/amun-constitutional/src/canonical_witness.rs:11://!   2. Artifact hash (lexicographic) within same WitnessType
crates/amun-constitutional/src/canonical_witness.rs:122:    fn test_is_canonical() {
crates/amun-constitutional/src/canonical_witness.rs:124:            entry(WitnessType::HardDependency, [0xAA; 32]),
crates/amun-constitutional/src/canonical_witness.rs:125:            entry(WitnessType::HardDependency, [0xBB; 32]),
crates/amun-constitutional/src/canonical_witness.rs:126:            entry(WitnessType::SupportingDependency, [0xCC; 32]),
crates/amun-constitutional/src/canonical_witness.rs:132:    fn test_is_not_canonical_wrong_priority_order() {
crates/amun-constitutional/src/canonical_witness.rs:134:            entry(WitnessType::SupportingDependency, [0xAA; 32]),
crates/amun-constitutional/src/canonical_witness.rs:135:            entry(WitnessType::HardDependency, [0xBB; 32]),
crates/amun-constitutional/src/canonical_witness.rs:13:use crate::constitutional_witness::WitnessEntry;
crates/amun-constitutional/src/canonical_witness.rs:141:    fn test_normalize_produces_canonical() {
crates/amun-constitutional/src/canonical_witness.rs:142:        use crate::constitutional_witness::ConstitutionalWitness;
crates/amun-constitutional/src/canonical_witness.rs:144:            entry(WitnessType::CompressionElidable, [0x01; 32]),
crates/amun-constitutional/src/canonical_witness.rs:145:            entry(WitnessType::HardDependency, [0x02; 32]),
crates/amun-constitutional/src/canonical_witness.rs:147:        let w = ConstitutionalWitness::new(1, 1, 1, [0xAA; 32], [0xAB; 32], entries);
crates/amun-constitutional/src/canonical_witness.rs:14:use crate::witness_type::WitnessType;
crates/amun-constitutional/src/canonical_witness.rs:154:    fn test_normalize_idempotent() {
crates/amun-constitutional/src/canonical_witness.rs:155:        use crate::constitutional_witness::ConstitutionalWitness;
crates/amun-constitutional/src/canonical_witness.rs:157:            entry(WitnessType::HardDependency, [0xAA; 32]),
crates/amun-constitutional/src/canonical_witness.rs:158:            entry(WitnessType::SupportingDependency, [0xBB; 32]),
crates/amun-constitutional/src/canonical_witness.rs:160:        let w = ConstitutionalWitness::new(1, 1, 1, [0xAA; 32], [0xAB; 32], entries);
crates/amun-constitutional/src/canonical_witness.rs:1://! Canonical Witness Ordering — deterministic proof surface.
crates/amun-constitutional/src/canonical_witness.rs:23:///   5. Within the same WitnessType, entries are sorted by artifact_hash (lexicographic)
crates/amun-constitutional/src/canonical_witness.rs:27:pub fn canonical_order(entries: &mut [WitnessEntry]) {
crates/amun-constitutional/src/canonical_witness.rs:29:        // Primary sort: WitnessType priority
crates/amun-constitutional/src/canonical_witness.rs:43:fn type_priority(wt: WitnessType) -> u8 {
crates/amun-constitutional/src/canonical_witness.rs:45:        WitnessType::HardDependency => 0,
crates/amun-constitutional/src/canonical_witness.rs:46:        WitnessType::SupportingDependency => 1,
crates/amun-constitutional/src/canonical_witness.rs:47:        WitnessType::AuditDependency => 2,
crates/amun-constitutional/src/canonical_witness.rs:48:        WitnessType::CompressionElidable => 3,
crates/amun-constitutional/src/canonical_witness.rs:54:pub fn normalize(
crates/amun-constitutional/src/canonical_witness.rs:55:    witness: &crate::constitutional_witness::ConstitutionalWitness,
crates/amun-constitutional/src/canonical_witness.rs:56:) -> crate::constitutional_witness::ConstitutionalWitness {
crates/amun-constitutional/src/canonical_witness.rs:59:    crate::constitutional_witness::ConstitutionalWitness::new(
crates/amun-constitutional/src/canonical_witness.rs:5://!   - Proof portability across runtimes
crates/amun-constitutional/src/canonical_witness.rs:62:        witness.replay_revision,
crates/amun-constitutional/src/canonical_witness.rs:6://!   - Witness caching and equivalence checking
crates/amun-constitutional/src/canonical_witness.rs:70:pub fn is_canonical(entries: &[WitnessEntry]) -> bool {
crates/amun-constitutional/src/canonical_witness.rs:90:    fn entry(wt: WitnessType, hash: [u8; 32]) -> WitnessEntry {
crates/amun-constitutional/src/canonical_witness.rs:91:        WitnessEntry::new(hash, wt)
crates/amun-constitutional/src/canonical_witness.rs:95:    fn test_canonical_order_respects_priority() {
crates/amun-constitutional/src/canonical_witness.rs:97:            entry(WitnessType::CompressionElidable, [0x01; 32]),
crates/amun-constitutional/src/canonical_witness.rs:98:            entry(WitnessType::HardDependency, [0x02; 32]),
crates/amun-constitutional/src/canonical_witness.rs:99:            entry(WitnessType::SupportingDependency, [0x03; 32]),
crates/amun-constitutional/src/canonical_witness.rs:9://! RULE: Witness entries are ordered canonically by:
crates/amun-constitutional/src/causal_edge.rs:217:    fn test_hash_deterministic() {
crates/amun-constitutional/src/causality_chain.rs:257:    fn test_hash_deterministic() {
crates/amun-constitutional/src/certificate_scope.rs:265:    fn test_scope_hash_deterministic() {
crates/amun-constitutional/src/constitutional_failure.rs:206:    pub fn with_invariant(mut self, id: u64, lineage: ConstitutionalHash) -> Self {
crates/amun-constitutional/src/constitutional_failure.rs:54:    pub invariant_lineage_root: Option<ConstitutionalHash>,
crates/amun-constitutional/src/constitutional_hasher.rs:82:    fn test_deterministic() {
crates/amun-constitutional/src/constitutional_witness.rs:1://! ConstitutionalWitness — a formally sufficient constitutional proof surface.
crates/amun-constitutional/src/constitutional_witness.rs:310:    fn test_hash_deterministic() {
crates/amun-constitutional/src/constitutional_witness.rs:6://! INVARIANT: Every artifact in the witness has a defined WitnessType.
crates/amun-constitutional/src/continuation_chain.rs:236:    fn test_hash_deterministic() {
crates/amun-constitutional/src/divergence_point.rs:234:    fn test_hash_deterministic() {
crates/amun-constitutional/src/divergence_resolution.rs:193:    fn test_hash_deterministic() {
crates/amun-constitutional/src/execution_limits.rs:25:pub struct InvariantLimits {
crates/amun-constitutional/src/execution_limits.rs:47:    pub invariant: InvariantLimits,
crates/amun-constitutional/src/execution_limits.rs:78:            invariant: InvariantLimits {
crates/amun-constitutional/src/execution_receipt.rs:246:    fn test_hash_deterministic() {
crates/amun-constitutional/src/replay_outcome.rs:54:    fn test_outcome_hash_deterministic() {
crates/amun-constitutional/src/restoration_point.rs:203:    fn test_hash_deterministic() {
crates/amun-constitutional/src/snapshot.rs:268:    fn test_hash_deterministic() {
crates/amun-constitutional/src/state_anchor.rs:13://! INVARIANT (Replay-Derived State Identity):
crates/amun-constitutional/src/state_anchor.rs:14://!   Constitutional state identity is derived exclusively from deterministic
crates/amun-constitutional/src/state_anchor.rs:246:    fn test_hash_deterministic() {
crates/amun-constitutional/src/state_anchor_scope.rs:104:            AnchorScopeRelationship::StateTransition
crates/amun-constitutional/src/state_anchor_scope.rs:108:            AnchorScopeRelationship::StateTransition
crates/amun-constitutional/src/state_anchor_scope.rs:121:    /// StateTransition is accepted (constitutional evolution),
crates/amun-constitutional/src/state_anchor_scope.rs:131:            | AnchorScopeRelationship::StateTransition => Ok(rel),
crates/amun-constitutional/src/state_anchor_scope.rs:170:            AnchorScopeRelationship::StateTransition
crates/amun-constitutional/src/state_anchor_scope.rs:27:    StateTransition,
crates/amun-contract-events/tests/n173_events_storage_tests.rs:24:fn n173_events_root_deterministic() {
crates/amun-contract-integration/tests/n167_contract_integration_tests.rs:38:fn n167_contract_evidence_root_deterministic() {
crates/amun-contract-integration/tests/n169_contract_nft_defi_tests.rs:73:fn n169_contract_registry_root_deterministic() {
crates/amun-contract-security/src/lib.rs:134:pub fn audit_evidence_consistency() -> SecurityAuditResult {
crates/amun-contract-security/tests/n170_security_audit_tests.rs:34:fn n170_audit_evidence_consistency_pass() {
crates/amun-cross-contract/src/transfer_registry.rs:122:    fn w11_proof_id_deterministic() {
crates/amun-defi-amm/tests/n153_amm_tests.rs:19:fn n153_pool_evidence_root_deterministic() {
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:18:fn n153_liquidity_evidence_deterministic() {
crates/amun-defi-evidence/tests/n153_evidence_tests.rs:4:fn n153_swap_evidence_deterministic() {
crates/amun-defi-governance/tests/n157_governance_tests.rs:15:fn n157_governance_root_deterministic() {
crates/amun-defi-lending-engine/tests/n154_lending_tests.rs:83:fn n154_lending_root_deterministic() {
crates/amun-defi-stablecoin/tests/n155_stablecoin_tests.rs:24:fn n155_stablecoin_root_deterministic() {
crates/amun-deterministic-allocator/src/arena.rs:10:    pub fn new() -> Self {
crates/amun-deterministic-allocator/src/arena.rs:18:    pub fn allocate(&mut self, size: usize) -> Option<(&mut [u8], usize)> {
crates/amun-deterministic-allocator/src/arena.rs:28:    pub fn get_slice(&self, offset: usize, len: usize) -> Option<&[u8]> {
crates/amun-deterministic-allocator/src/arena.rs:35:    pub fn used(&self) -> usize {
crates/amun-deterministic-allocator/src/arena.rs:39:    pub fn remaining(&self) -> usize {
crates/amun-deterministic-allocator/src/arena.rs:3:pub struct DeterministicArena {
crates/amun-deterministic-allocator/src/arena.rs:43:    pub fn reset(&mut self) {
crates/amun-deterministic-allocator/src/arena.rs:48:    pub fn as_bytes(&self) -> &[u8] {
crates/amun-deterministic-allocator/src/arena.rs:53:impl Default for DeterministicArena {
crates/amun-deterministic-allocator/src/arena.rs:54:    fn default() -> Self {
crates/amun-deterministic-allocator/src/arena.rs:9:impl DeterministicArena {
crates/amun-deterministic-allocator/src/deterministic_map.rs:15:    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, &'static str> {
crates/amun-deterministic-allocator/src/deterministic_map.rs:33:    pub fn get(&self, key: &K) -> Option<&V> {
crates/amun-deterministic-allocator/src/deterministic_map.rs:40:    pub fn remove(&mut self, key: &K) -> Option<V> {
crates/amun-deterministic-allocator/src/deterministic_map.rs:47:    pub fn contains_key(&self, key: &K) -> bool {
crates/amun-deterministic-allocator/src/deterministic_map.rs:4:pub struct DeterministicMap<K: Ord + Clone, V: Clone> {
crates/amun-deterministic-allocator/src/deterministic_map.rs:51:    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> + '_ {
crates/amun-deterministic-allocator/src/deterministic_map.rs:55:    pub fn is_empty(&self) -> bool {
crates/amun-deterministic-allocator/src/deterministic_map.rs:59:    pub fn len(&self) -> usize {
crates/amun-deterministic-allocator/src/deterministic_map.rs:65:    fn default() -> Self {
crates/amun-deterministic-allocator/src/deterministic_map.rs:9:    pub fn new() -> Self {
crates/amun-deterministic-allocator/src/deterministic_set.rs:11:    pub fn new() -> Self {
crates/amun-deterministic-allocator/src/deterministic_set.rs:17:    pub fn insert(&mut self, value: T) -> Result<bool, &'static str> {
crates/amun-deterministic-allocator/src/deterministic_set.rs:29:    pub fn contains(&self, value: &T) -> bool {
crates/amun-deterministic-allocator/src/deterministic_set.rs:33:    pub fn is_empty(&self) -> bool {
crates/amun-deterministic-allocator/src/deterministic_set.rs:37:    pub fn len(&self) -> usize {
crates/amun-deterministic-allocator/src/deterministic_set.rs:43:    fn default() -> Self {
crates/amun-deterministic-allocator/src/deterministic_set.rs:6:pub struct DeterministicSet<T: Ord + Clone> {
crates/amun-deterministic-allocator/src/sorted_vec.rs:11:    pub fn new() -> Self {
crates/amun-deterministic-allocator/src/sorted_vec.rs:17:    pub fn push(&mut self, value: T) -> Result<(), &'static str> {
crates/amun-deterministic-allocator/src/sorted_vec.rs:26:    pub fn get(&self, index: usize) -> Option<&T> {
crates/amun-deterministic-allocator/src/sorted_vec.rs:30:    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
crates/amun-deterministic-allocator/src/sorted_vec.rs:34:    pub fn is_empty(&self) -> bool {
crates/amun-deterministic-allocator/src/sorted_vec.rs:38:    pub fn len(&self) -> usize {
crates/amun-deterministic-allocator/src/sorted_vec.rs:42:    pub fn as_slice(&self) -> &[T] {
crates/amun-deterministic-allocator/src/sorted_vec.rs:48:    fn default() -> Self {
crates/amun-deterministic-allocator/src/sorted_vec.rs:6:pub struct SortedVec<T: Ord + Clone> {
crates/amun-deterministic-scheduler/src/budget.rs:10:    pub fn new(max_cost_per_round: u64) -> Self {
crates/amun-deterministic-scheduler/src/budget.rs:17:    pub fn consume(&mut self, cost: u64) -> Result<(), &'static str> {
crates/amun-deterministic-scheduler/src/budget.rs:26:    pub fn remaining(&self) -> u64 {
crates/amun-deterministic-scheduler/src/budget.rs:30:    pub fn reset(&mut self) {
crates/amun-deterministic-scheduler/src/budget.rs:4:pub struct ResourceBudget {
crates/amun-deterministic-scheduler/src/budget.rs:9:impl ResourceBudget {
crates/amun-deterministic-scheduler/src/queue.rs:11:pub struct DeterministicQueue {
crates/amun-deterministic-scheduler/src/queue.rs:16:impl DeterministicQueue {
crates/amun-deterministic-scheduler/src/queue.rs:17:    pub fn new() -> Self {
crates/amun-deterministic-scheduler/src/queue.rs:24:    pub fn with_capacity(max_capacity: usize) -> Self {
crates/amun-deterministic-scheduler/src/queue.rs:31:    pub fn push(
crates/amun-deterministic-scheduler/src/queue.rs:41:        // Lower priority number = higher scheduling priority (executed first)
crates/amun-deterministic-scheduler/src/queue.rs:4:pub struct QueueEntry {
crates/amun-deterministic-scheduler/src/queue.rs:63:    pub fn pop(&mut self) -> Option<QueueEntry> {
crates/amun-deterministic-scheduler/src/queue.rs:67:    pub fn len(&self) -> usize {
crates/amun-deterministic-scheduler/src/queue.rs:71:    pub fn is_empty(&self) -> bool {
crates/amun-deterministic-scheduler/src/queue.rs:75:    pub fn capacity(&self) -> usize {
crates/amun-deterministic-scheduler/src/queue.rs:80:impl Default for DeterministicQueue {
crates/amun-deterministic-scheduler/src/queue.rs:81:    fn default() -> Self {
crates/amun-deterministic-scheduler/src/scheduler.rs:112:        while executed.len() < max_tasks && self.io_consumed_this_round < self.io_quota {
crates/amun-deterministic-scheduler/src/scheduler.rs:117:                    executed.push(ExecutedTask {
crates/amun-deterministic-scheduler/src/scheduler.rs:134:        executed
crates/amun-deterministic-scheduler/src/scheduler.rs:137:    pub fn pending_count(&self) -> usize {
crates/amun-deterministic-scheduler/src/scheduler.rs:140:    pub fn io_pending_count(&self) -> usize {
crates/amun-deterministic-scheduler/src/scheduler.rs:143:    pub fn round(&self) -> u64 {
crates/amun-deterministic-scheduler/src/scheduler.rs:146:    pub fn io_quota_remaining(&self) -> u64 {
crates/amun-deterministic-scheduler/src/scheduler.rs:152:pub struct ExecutedTask {
crates/amun-deterministic-scheduler/src/scheduler.rs:16:impl DeterministicScheduler {
crates/amun-deterministic-scheduler/src/scheduler.rs:17:    pub fn new(budget: ResourceBudget) -> Self {
crates/amun-deterministic-scheduler/src/scheduler.rs:28:    pub fn with_io_quota(budget: ResourceBudget, io_quota: u64) -> Self {
crates/amun-deterministic-scheduler/src/scheduler.rs:39:    pub fn enqueue(
crates/amun-deterministic-scheduler/src/scheduler.rs:48:    pub fn enqueue_io(
crates/amun-deterministic-scheduler/src/scheduler.rs:58:    /// I/O gets up to io_quota tasks per round, then regular tasks execute.
crates/amun-deterministic-scheduler/src/scheduler.rs:59:    pub fn execute_batch(&mut self, max_tasks: usize) -> Vec<ExecutedTask> {
crates/amun-deterministic-scheduler/src/scheduler.rs:60:        let mut executed = Vec::new();
crates/amun-deterministic-scheduler/src/scheduler.rs:65:        while executed.len() < max_tasks && self.io_consumed_this_round < self.io_quota {
crates/amun-deterministic-scheduler/src/scheduler.rs:70:                    executed.push(ExecutedTask {
crates/amun-deterministic-scheduler/src/scheduler.rs:7:pub struct DeterministicScheduler {
crates/amun-deterministic-scheduler/src/scheduler.rs:89:        while executed.len() < max_tasks {
crates/amun-deterministic-scheduler/src/scheduler.rs:93:                    executed.push(ExecutedTask {
crates/amun-deterministic-timer/src/timeout.rs:10:    pub fn new(base_timeout_rounds: u64, max_timeout_rounds: u64) -> Self {
crates/amun-deterministic-timer/src/timeout.rs:19:    pub fn on_timeout(&mut self) -> u64 {
crates/amun-deterministic-timer/src/timeout.rs:2:pub struct TimeoutLaw {
crates/amun-deterministic-timer/src/timeout.rs:30:    pub fn on_progress(&mut self) {
crates/amun-deterministic-timer/src/timeout.rs:35:    pub fn current_timeout(&self) -> u64 {
crates/amun-deterministic-timer/src/timeout.rs:9:impl TimeoutLaw {
crates/amun-deterministic-timer/src/wheel.rs:10:impl DeterministicTimerWheel {
crates/amun-deterministic-timer/src/wheel.rs:11:    pub fn new() -> Self {
crates/amun-deterministic-timer/src/wheel.rs:19:    pub fn schedule(&mut self, round: u64) -> u64 {
crates/amun-deterministic-timer/src/wheel.rs:31:    pub fn cancel(&mut self, id: u64) {
crates/amun-deterministic-timer/src/wheel.rs:35:    pub fn advance(&mut self) -> Vec<u64> {
crates/amun-deterministic-timer/src/wheel.rs:4:pub struct DeterministicTimerWheel {
crates/amun-deterministic-timer/src/wheel.rs:51:    pub fn current_round(&self) -> u64 {
crates/amun-deterministic-timer/src/wheel.rs:54:    pub fn pending_count(&self) -> usize {
crates/amun-deterministic-timer/src/wheel.rs:59:impl Default for DeterministicTimerWheel {
crates/amun-deterministic-timer/src/wheel.rs:60:    fn default() -> Self {
crates/amun-dual-verification/src/dual_verifier.rs:18:        invariants: &[InvariantDeclaration],
crates/amun-dual-verification/src/dual_verifier.rs:3:use amun_invariant_engine::invariant_types::InvariantDeclaration;
crates/amun-dual-verification/src/dual_verifier.rs:5:use amun_pccv::transition_proof_engine::TransitionProofEngine as PCCVEngine;
crates/amun-entropy-transcript/src/source.rs:13:pub struct DeterministicEntropy {
crates/amun-entropy-transcript/src/source.rs:18:impl DeterministicEntropy {
crates/amun-evidence-engine/src/evidence_engine.rs:118:    fn w4_evidence_id_deterministic() {
crates/amun-evidence-engine/src/evidence_engine.rs:153:        archive.insert(ConstitutionalEvidence::InvariantViolation {
crates/amun-evidence-engine/src/evidence_engine.rs:33:            VMEvidence::InvariantViolation { obligation_id } => {
crates/amun-evidence-engine/src/evidence_engine.rs:34:                ConstitutionalEvidence::InvariantViolation {
crates/amun-evidence-engine/src/evidence_types.rs:23:    /// Contract invariant failure — evaluated post-commit.
crates/amun-evidence-engine/src/evidence_types.rs:24:    InvariantViolation {
crates/amun-evidence-engine/src/evidence_types.rs:57:            Self::InvariantViolation {
crates/amun-evidence-engine/src/evidence_types.rs:77:            Self::InvariantViolation { .. } => "invariant_violation",
crates/amun-evidence-finality/src/evidence_finality.rs:343:    fn w19_certificate_deterministic() {
crates/amun-evidence-root/src/lib.rs:132:    fn n39_evidence_root_deterministic() {
crates/amun-evolution/src/certificate.rs:137:        h.update(&[self.replay_guarantee.canonical_tag()]);
crates/amun-evolution/src/validator.rs:22:        proof.replay_guarantee.rank() >= ReplayGuarantee::Deterministic.rank()
crates/amun-evolution/src/validator.rs:9:    pub fn verify_consistency(
crates/amun-execution-receipt/src/lib.rs:110:impl CanonicalEncode for ExecutionTranscript {
crates/amun-execution-receipt/src/lib.rs:111:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-execution-receipt/src/lib.rs:167:            if let Err(e) = receipt.verify_consistency() {
crates/amun-execution-receipt/src/lib.rs:27:impl CanonicalEncode for ExecutionStatus {
crates/amun-execution-receipt/src/lib.rs:28:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-execution-receipt/src/lib.rs:51:impl CanonicalEncode for ExecutionReceipt {
crates/amun-execution-receipt/src/lib.rs:52:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-execution-receipt/src/lib.rs:6://! ## Constitutional Invariants
crates/amun-execution-receipt/src/lib.rs:93:    pub fn verify_consistency(&self) -> Result<(), &'static str> {
crates/amun-execution/src/executor.rs:20:        let result = self.interpreter.execute_deterministic(code, input)?;
crates/amun-execution/src/tests.rs:37:fn test_canonical_nan_constants() {
crates/amun-execution/src/tests.rs:50:fn test_verify_deterministic_wasm_ok() {
crates/amun-execution/src/tests.rs:52:    assert!(wasm_deterministic_subset::verify_deterministic_wasm(b"").is_ok());
crates/amun-execution/src/verified_interpreter.rs:27:    pub fn new(profile: DeterministicWasmProfile) -> Self {
crates/amun-execution/src/wasm_deterministic.rs:46:    pub fn verify_deterministic_wasm(_module: &[u8]) -> Result<(), &'static str> {
crates/amun-execution/src/wasm_profile.rs:53:pub struct DeterministicWasmProfile {
crates/amun-execution/src/wasm_profile.rs:64:impl DeterministicWasmProfile {
crates/amun-failure/src/taxonomy.rs:12:    InvalidStateTransition = 0x2001,
crates/amun-failure/src/taxonomy.rs:27:    // Contract & Invariant (0x6XXX)
crates/amun-failure/src/taxonomy.rs:51:            Self::InvalidStateTransition
crates/amun-failure/src/tests.rs:32:    assert!(!ConstitutionalFault::InvalidStateTransition.should_halt());
crates/amun-finality-certificate/src/lib.rs:132:    fn n41_hash_deterministic() {
crates/amun-fork/src/rule.rs:60:            // Tie-break: canonical ordering commitment
crates/amun-formal/src/invariants.rs:12:    pub fn stake_consistency(total_staked: u64, validator_stakes: &[u64]) -> bool {
crates/amun-formal/src/invariants.rs:16:    pub fn quorum_safety(votes: u64, total: u64, threshold_bps: u16) -> bool {
crates/amun-formal/src/invariants.rs:1:pub struct FormalInvariants;
crates/amun-formal/src/invariants.rs:24:    pub fn no_overflow_in_supply(a: u64, b: u64) -> bool {
crates/amun-formal/src/invariants.rs:3:impl FormalInvariants {
crates/amun-formal/src/invariants.rs:4:    pub fn supply_conservation(minted: u64, burned: u64, initial: u64, current: u64) -> bool {
crates/amun-formal/src/invariants.rs:8:    pub fn nonce_monotonicity(prev_nonce: u64, new_nonce: u64) -> bool {
crates/amun-formal/src/lib.rs:2:pub use invariants::FormalInvariants;
crates/amun-gas-engine/src/gas_engine.rs:134:    fn w7_gas_accounting_deterministic() {
crates/amun-gas-engine/src/opcode_costs.rs:22:            OpCode::CheckInvariant { .. } => 50,
crates/amun-genesis/src/constitution.rs:12:    pub fn new(invariant_kernel_hash: [u8; 32], complexity_budget_json: String) -> Self {
crates/amun-host/src/boundary.rs:17:    pub fn canonical_hash(data: &[u8]) -> [u8; 32] {
crates/amun-invariant-engine/src/invariant_engine.rs:105:    fn w8_all_invariants_pass() {
crates/amun-invariant-engine/src/invariant_engine.rs:107:        let (results, evidence) = InvariantEngine::evaluate(
crates/amun-invariant-engine/src/invariant_engine.rs:115:        assert!(InvariantEngine::all_passed(&results));
crates/amun-invariant-engine/src/invariant_engine.rs:116:        assert!(!InvariantEngine::has_critical_failure(&results));
crates/amun-invariant-engine/src/invariant_engine.rs:121:    fn w8_critical_failure_produces_evidence() {
crates/amun-invariant-engine/src/invariant_engine.rs:123:        let (results, evidence) = InvariantEngine::evaluate(
crates/amun-invariant-engine/src/invariant_engine.rs:12:    pub fn evaluate<F>(
crates/amun-invariant-engine/src/invariant_engine.rs:131:        assert!(!InvariantEngine::all_passed(&results));
crates/amun-invariant-engine/src/invariant_engine.rs:132:        assert!(InvariantEngine::has_critical_failure(&results));
crates/amun-invariant-engine/src/invariant_engine.rs:135:            ConstitutionalEvidence::InvariantViolation { obligation_id, .. } => {
crates/amun-invariant-engine/src/invariant_engine.rs:138:            _ => panic!("Expected InvariantViolation"),
crates/amun-invariant-engine/src/invariant_engine.rs:13:        invariants: &[InvariantDeclaration],
crates/amun-invariant-engine/src/invariant_engine.rs:143:    fn w8_minor_failure_produces_evidence_but_not_critical() {
crates/amun-invariant-engine/src/invariant_engine.rs:145:        let (results, evidence) = InvariantEngine::evaluate(
crates/amun-invariant-engine/src/invariant_engine.rs:153:        assert!(!InvariantEngine::all_passed(&results));
crates/amun-invariant-engine/src/invariant_engine.rs:154:        assert!(!InvariantEngine::has_critical_failure(&results)); // No Critical failed
crates/amun-invariant-engine/src/invariant_engine.rs:159:    fn w8_count_by_severity() {
crates/amun-invariant-engine/src/invariant_engine.rs:161:        let (results, _) = InvariantEngine::evaluate(
crates/amun-invariant-engine/src/invariant_engine.rs:170:            InvariantEngine::count_by_severity(&results, InvariantSeverity::Critical),
crates/amun-invariant-engine/src/invariant_engine.rs:174:            InvariantEngine::count_by_severity(&results, InvariantSeverity::Minor),
crates/amun-invariant-engine/src/invariant_engine.rs:180:    fn w8_evidence_contains_state_root() {
crates/amun-invariant-engine/src/invariant_engine.rs:183:        let (_, evidence) = InvariantEngine::evaluate(
crates/amun-invariant-engine/src/invariant_engine.rs:194:                ConstitutionalEvidence::InvariantViolation { state_root: sr, .. } => {
crates/amun-invariant-engine/src/invariant_engine.rs:197:                _ => panic!("Expected InvariantViolation"),
crates/amun-invariant-engine/src/invariant_engine.rs:19:    ) -> (Vec<InvariantResult>, Vec<ConstitutionalEvidence>)
crates/amun-invariant-engine/src/invariant_engine.rs:1:use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
crates/amun-invariant-engine/src/invariant_engine.rs:21:        F: Fn(&InvariantDeclaration) -> bool,
crates/amun-invariant-engine/src/invariant_engine.rs:28:            results.push(InvariantResult {
crates/amun-invariant-engine/src/invariant_engine.rs:35:                evidence.push(ConstitutionalEvidence::InvariantViolation {
crates/amun-invariant-engine/src/invariant_engine.rs:49:    pub fn count_by_severity(results: &[InvariantResult], severity: InvariantSeverity) -> usize {
crates/amun-invariant-engine/src/invariant_engine.rs:4:use crate::invariant_types::{InvariantDeclaration, InvariantResult, InvariantSeverity};
crates/amun-invariant-engine/src/invariant_engine.rs:57:    pub fn has_critical_failure(results: &[InvariantResult]) -> bool {
crates/amun-invariant-engine/src/invariant_engine.rs:60:            .any(|r| r.severity == InvariantSeverity::Critical && !r.passed)
crates/amun-invariant-engine/src/invariant_engine.rs:64:    pub fn all_passed(results: &[InvariantResult]) -> bool {
crates/amun-invariant-engine/src/invariant_engine.rs:6:/// Evaluates contract invariants after commit (Phase 5 in N48.5-D).
crates/amun-invariant-engine/src/invariant_engine.rs:72:    use crate::invariant_types::{InvariantDeclaration, InvariantScope};
crates/amun-invariant-engine/src/invariant_engine.rs:75:    fn make_id(seed: u8) -> ResourceId {
crates/amun-invariant-engine/src/invariant_engine.rs:7:pub struct InvariantEngine;
crates/amun-invariant-engine/src/invariant_engine.rs:81:    fn sample_invariants() -> Vec<InvariantDeclaration> {
crates/amun-invariant-engine/src/invariant_engine.rs:83:            InvariantDeclaration {
crates/amun-invariant-engine/src/invariant_engine.rs:86:                severity: InvariantSeverity::Critical,
crates/amun-invariant-engine/src/invariant_engine.rs:87:                scope: InvariantScope::State,
crates/amun-invariant-engine/src/invariant_engine.rs:89:            InvariantDeclaration {
crates/amun-invariant-engine/src/invariant_engine.rs:92:                severity: InvariantSeverity::Critical,
crates/amun-invariant-engine/src/invariant_engine.rs:93:                scope: InvariantScope::State,
crates/amun-invariant-engine/src/invariant_engine.rs:95:            InvariantDeclaration {
crates/amun-invariant-engine/src/invariant_engine.rs:98:                severity: InvariantSeverity::Minor,
crates/amun-invariant-engine/src/invariant_engine.rs:99:                scope: InvariantScope::Local,
crates/amun-invariant-engine/src/invariant_engine.rs:9:impl InvariantEngine {
crates/amun-invariant-engine/src/invariant_types.rs:18:pub enum InvariantScope {
crates/amun-invariant-engine/src/invariant_types.rs:26:    Constitutional,
crates/amun-invariant-engine/src/invariant_types.rs:31:pub struct InvariantDeclaration {
crates/amun-invariant-engine/src/invariant_types.rs:34:    pub severity: InvariantSeverity,
crates/amun-invariant-engine/src/invariant_types.rs:35:    pub scope: InvariantScope,
crates/amun-invariant-engine/src/invariant_types.rs:40:pub struct InvariantResult {
crates/amun-invariant-engine/src/invariant_types.rs:43:    pub severity: InvariantSeverity,
crates/amun-invariant-engine/src/invariant_types.rs:5:pub enum InvariantSeverity {
crates/amun-invariants/src/kernel.rs:13:    pub severity: InvariantSeverity,
crates/amun-invariants/src/kernel.rs:16:    pub requires_replay_verification: bool,
crates/amun-invariants/src/kernel.rs:20:pub struct IrreducibleInvariants;
crates/amun-invariants/src/kernel.rs:22:impl IrreducibleInvariants {
crates/amun-invariants/src/kernel.rs:23:    pub const SINGLE_TRUTH: InvariantDef = InvariantDef {
crates/amun-invariants/src/kernel.rs:27:        severity: InvariantSeverity::Fatal,
crates/amun-invariants/src/kernel.rs:2:pub enum InvariantSeverity {
crates/amun-invariants/src/kernel.rs:30:        requires_replay_verification: true,
crates/amun-invariants/src/kernel.rs:34:    pub const SINGLE_FINALITY: InvariantDef = InvariantDef {
crates/amun-invariants/src/kernel.rs:36:        name: "Single Finality",
crates/amun-invariants/src/kernel.rs:37:        specification: "forall h: |{finalized_blocks_at_height(h)}| <= 1 under f < n/3",
crates/amun-invariants/src/kernel.rs:38:        severity: InvariantSeverity::Fatal,
crates/amun-invariants/src/kernel.rs:41:        requires_replay_verification: false,
crates/amun-invariants/src/kernel.rs:45:    pub const EVENTUAL_PROGRESS: InvariantDef = InvariantDef {
crates/amun-invariants/src/kernel.rs:48:        specification: "diamond(new_block_finalized) under eventual synchrony",
crates/amun-invariants/src/kernel.rs:49:        severity: InvariantSeverity::Critical,
crates/amun-invariants/src/kernel.rs:52:        requires_replay_verification: false,
crates/amun-invariants/src/kernel.rs:56:    pub const FAILURE_MEMORY: InvariantDef = InvariantDef {
crates/amun-invariants/src/kernel.rs:60:        severity: InvariantSeverity::Degraded,
crates/amun-invariants/src/kernel.rs:63:        requires_replay_verification: false,
crates/amun-invariants/src/kernel.rs:67:    pub fn all() -> [InvariantDef; 4] {
crates/amun-invariants/src/kernel.rs:76:    pub fn kernel_hash() -> [u8; 32] {
crates/amun-invariants/src/kernel.rs:84:        out.copy_from_slice(&hasher.finalize().as_bytes()[..32]);
crates/amun-invariants/src/kernel.rs:9:pub struct InvariantDef {
crates/amun-invariants/src/lib.rs:6:pub use kernel::{InvariantDef, InvariantSeverity, IrreducibleInvariants};
crates/amun-invariants/src/lib.rs:7:pub use registry::{InvariantHealth, InvariantRegistry};
crates/amun-invariants/src/lib.rs:8:pub use status::InvariantStatus;
crates/amun-invariants/src/registry.rs:11:impl InvariantRegistry {
crates/amun-invariants/src/registry.rs:12:    pub fn with_kernel() -> Self {
crates/amun-invariants/src/registry.rs:16:        for inv in IrreducibleInvariants::all().iter() {
crates/amun-invariants/src/registry.rs:18:            statuses.push(InvariantStatus::new(inv.id));
crates/amun-invariants/src/registry.rs:1:use crate::kernel::{InvariantDef, InvariantSeverity, IrreducibleInvariants};
crates/amun-invariants/src/registry.rs:27:    pub fn register(&mut self, inv: InvariantDef) -> Result<(), &'static str> {
crates/amun-invariants/src/registry.rs:2:use crate::status::InvariantStatus;
crates/amun-invariants/src/registry.rs:35:        self.statuses.push(InvariantStatus::new(inv.id));
crates/amun-invariants/src/registry.rs:39:    pub fn record_violation(
crates/amun-invariants/src/registry.rs:52:    pub fn invariant_count(&self) -> usize {
crates/amun-invariants/src/registry.rs:56:    pub fn overall_health(&self) -> InvariantHealth {
crates/amun-invariants/src/registry.rs:6:pub struct InvariantRegistry {
crates/amun-invariants/src/registry.rs:70:                    .any(|i| i.id == s.invariant_id && i.severity == InvariantSeverity::Fatal)
crates/amun-invariants/src/registry.rs:75:            InvariantHealth::Critical
crates/amun-invariants/src/registry.rs:77:            InvariantHealth::Degraded
crates/amun-invariants/src/registry.rs:79:            InvariantHealth::Unknown
crates/amun-invariants/src/registry.rs:7:    invariants: Vec<InvariantDef>,
crates/amun-invariants/src/registry.rs:81:            InvariantHealth::AllInvariantsHold
crates/amun-invariants/src/registry.rs:87:pub enum InvariantHealth {
crates/amun-invariants/src/registry.rs:88:    AllInvariantsHold,
crates/amun-invariants/src/registry.rs:8:    statuses: Vec<InvariantStatus>,
crates/amun-invariants/src/status.rs:12:impl InvariantStatus {
crates/amun-invariants/src/status.rs:13:    pub fn new(invariant_id: u32) -> Self {
crates/amun-invariants/src/status.rs:23:    pub fn record_violation(&mut self, record: ViolationRecord) {
crates/amun-invariants/src/status.rs:33:    pub fn is_currently_violated(&self) -> bool {
crates/amun-invariants/src/status.rs:4:pub struct InvariantStatus {
crates/amun-invariants/src/violation.rs:2:pub struct ViolationRecord {
crates/amun-kernel/src/canonical.rs:104:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/canonical.rs:112:pub struct CanonicalBytes<'a>(pub &'a [u8]);
crates/amun-kernel/src/canonical.rs:115:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/canonical.rs:122:pub fn encode_hash_slice(items: &[[u8; 32]], out: &mut Vec<u8>) {
crates/amun-kernel/src/canonical.rs:130:pub struct CanonicalEncoder;
crates/amun-kernel/src/canonical.rs:132:impl CanonicalEncoder {
crates/amun-kernel/src/canonical.rs:134:    pub fn hash_sorted<I>(items: I, domain_tag: &[u8]) -> [u8; 32]
crates/amun-kernel/src/canonical.rs:14:    fn encode_canonical(&self, out: &mut Vec<u8>);
crates/amun-kernel/src/canonical.rs:156:        hasher.finalize().into()
crates/amun-kernel/src/canonical.rs:160:    pub fn hash_unsorted_canonicalized<T>(items: &[T], domain_tag: &[u8]) -> [u8; 32]
crates/amun-kernel/src/canonical.rs:170:    pub fn hash_value<V: CanonicalEncode + ?Sized>(value: &V, domain_tag: &[u8]) -> [u8; 32] {
crates/amun-kernel/src/canonical.rs:177:        hasher.finalize().into()
crates/amun-kernel/src/canonical.rs:186:    fn test_deterministic_hash() {
crates/amun-kernel/src/canonical.rs:193:    fn test_domain_separation() {
crates/amun-kernel/src/canonical.rs:19:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/canonical.rs:200:    fn test_vec_encoding() {
crates/amun-kernel/src/canonical.rs:208:    fn test_optional_encoding() {
crates/amun-kernel/src/canonical.rs:26:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/canonical.rs:36:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/canonical.rs:45:impl CanonicalEncode for u64 {
crates/amun-kernel/src/canonical.rs:46:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/canonical.rs:50:impl CanonicalEncode for u32 {
crates/amun-kernel/src/canonical.rs:51:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/canonical.rs:55:impl CanonicalEncode for u8 {
crates/amun-kernel/src/canonical.rs:56:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/canonical.rs:60:impl CanonicalEncode for [u8; 32] {
crates/amun-kernel/src/canonical.rs:61:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/canonical.rs:65:impl CanonicalEncode for [u8; 64] {
crates/amun-kernel/src/canonical.rs:66:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/canonical.rs:70:impl CanonicalEncode for str {
crates/amun-kernel/src/canonical.rs:71:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/canonical.rs:77:impl CanonicalEncode for String {
crates/amun-kernel/src/canonical.rs:78:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/canonical.rs:7:pub trait CanonicalEncode {
crates/amun-kernel/src/canonical.rs:82:impl CanonicalEncode for bool {
crates/amun-kernel/src/canonical.rs:83:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/canonical.rs:89:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/canonical.rs:8:    fn canonical_encode(&self) -> Vec<u8> {
crates/amun-kernel/src/governance.rs:108:impl CanonicalEncode for AuthorityRole {
crates/amun-kernel/src/governance.rs:109:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/governance.rs:134:impl CanonicalEncode for FreezeBoundary {
crates/amun-kernel/src/governance.rs:135:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/governance.rs:153:impl CanonicalEncode for FreezeDomain {
crates/amun-kernel/src/governance.rs:154:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/governance.rs:203:impl CanonicalEncode for AuthorityRegistry {
crates/amun-kernel/src/governance.rs:204:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/governance.rs:22:impl CanonicalEncode for Signature {
crates/amun-kernel/src/governance.rs:232:impl CanonicalEncode for Attestation {
crates/amun-kernel/src/governance.rs:233:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/governance.rs:23:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/governance.rs:287:impl CanonicalEncode for Snapshot {
crates/amun-kernel/src/governance.rs:288:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/governance.rs:347:impl CanonicalEncode for ReleaseSeal {
crates/amun-kernel/src/governance.rs:348:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-kernel/src/governance.rs:89:impl CanonicalEncode for Authority {
crates/amun-kernel/src/governance.rs:90:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-light-client/src/constitutional_client.rs:109:    use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-light-client/src/constitutional_client.rs:132:        let transitions = vec![TransitionProof::new(
crates/amun-light-client/tests/light_client_tests.rs:32:    let transitions = vec![TransitionProof::new(
crates/amun-light-client/tests/light_client_tests.rs:9:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-lineage-law/src/compatibility.rs:18:impl CompatibilityTheorem {
crates/amun-lineage-law/src/compatibility.rs:48:        theorem.theorem_hash = theorem.compute_hash();
crates/amun-lineage-law/src/compatibility.rs:4:/// A CompatibilityTheorem proves that two protocol versions can
crates/amun-lineage-law/src/compatibility.rs:65:        self.compute_hash() == self.theorem_hash
crates/amun-lineage-law/src/compatibility.rs:7:pub struct CompatibilityTheorem {
crates/amun-lineage-law/src/compatibility.rs:85:    pub fn determine(theorem: &CompatibilityTheorem) -> Self {
crates/amun-lineage-law/src/compatibility.rs:90:            && theorem.is_replay_compatible
crates/amun-lineage-law/src/lib.rs:9:pub use compatibility::{CompatibilityTheorem, CompatibilityVerdict};
crates/amun-lineage/src/compatibility.rs:13:    pub fn canonical_tag(&self) -> u8 {
crates/amun-lineage/src/record.rs:105:    pub fn canonical_tag(&self) -> u8 {
crates/amun-lineage/src/record.rs:15:    pub fn canonical_tag(&self) -> u8 {
crates/amun-lineage/src/record.rs:18:            ReplayGuarantee::Deterministic => 0x02,
crates/amun-lineage/src/record.rs:205:        h.update(&[self.replay_guarantee.canonical_tag()]);
crates/amun-lineage/src/record.rs:37:    pub fn canonical_tag(&self) -> u8 {
crates/amun-lineage/src/record.rs:59:    pub fn canonical_tag(&self) -> u8 {
crates/amun-lineage/src/record.rs:81:    pub fn canonical_tag(&self) -> u8 {
crates/amun-lineage/src/serialization.rs:19:            fn constitutional_encode(&self, w: &mut CanonicalWriter) {
crates/amun-lineage/src/serialization.rs:22:            fn constitutional_decode(r: &mut CanonicalReader) -> Option<Self> {
crates/amun-lineage/src/serialization.rs:8:    fn constitutional_encode(&self, w: &mut CanonicalWriter);
crates/amun-lineage/src/serialization.rs:9:    fn constitutional_decode(r: &mut CanonicalReader) -> Option<Self>
crates/amun-live-cluster/src/validator.rs:596:                        let replay_deterministic = cert.state_root == history_root;
crates/amun-live-cluster/src/validator.rs:618:                            replay_deterministic,
crates/amun-live-cluster/src/validator.rs:85:        // N105.4A: Deterministic key matching committed test certificates
crates/amun-mempool-gossip/src/messages.rs:75:    fn n74_tx_hash_deterministic() {
crates/amun-mempool/src/ordering.rs:3:pub struct CanonicalOrdering;
crates/amun-mempool/src/ordering.rs:5:impl CanonicalOrdering {
crates/amun-merkle/src/proof.rs:60:impl CanonicalEncode for MerkleProof {
crates/amun-merkle/src/proof.rs:62:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-merkle/src/proof.rs:77:impl CanonicalDecode for MerkleProof {
crates/amun-merkle/src/tests.rs:38:fn test_two_leaf_root_deterministic() {
crates/amun-merkle/src/tests.rs:48:fn test_three_leaf_root_deterministic() {
crates/amun-network-fastpath/tests/n164_fastpath_tests.rs:44:fn n164_batch_hash_deterministic() {
crates/amun-networking/src/peer_discovery.rs:169:    fn n20_7_registry_deterministic() {
crates/amun-networking/tests/n18_node_rejoin.rs:5:// N18.2 — Lifecycle Invariants
crates/amun-networking/tests/n18_node_rejoin.rs:73:// N18.5 — Constitutional Invariant REJOIN-001
crates/amun-nft-bridge/tests/n139_bridge_tests.rs:68:fn n139_deterministic_bridge_root() {
crates/amun-nft-collateral/tests/n156_nft_collateral_tests.rs:108:fn n156_evidence_root_deterministic() {
crates/amun-nft-constitutional-registry/tests/n140_constitutional_registry_tests.rs:66:fn n140_deterministic_constitutional_root() {
crates/amun-nft-fuzz/src/lib.rs:156:        // Invariant: marketplace should not crash the registry
crates/amun-nft-fuzz/src/lib.rs:178:        // Invariant: royalty cannot exceed sale price
crates/amun-nft-fuzz/src/lib.rs:206:        // Invariant: after revoke, cannot propose or veto
crates/amun-nft-fuzz/src/lib.rs:247:        // Invariant: bridge root should be deterministic
crates/amun-nft-fuzz/src/lib.rs:86:        // Invariant: collection must remain active
crates/amun-nft-fuzz/src/lib.rs:93:        // Invariant: active count >= minted NFTs
crates/amun-nft-governance-execution/tests/n143_governance_execution_tests.rs:75:fn n143_execution_root_deterministic() {
crates/amun-nft-governance/tests/n138_governance_tests.rs:67:fn n138_deterministic_governance_root() {
crates/amun-nft-indexer/tests/n144_indexer_tests.rs:75:fn n144_deterministic_index_root() {
crates/amun-nft-integration/tests/n132_integration_tests.rs:5:fn n132_extended_root_deterministic() {
crates/amun-nft-royalty-accounting/tests/n137_accounting_tests.rs:60:fn n137_deterministic_accounting_root() {
crates/amun-nft-royalty-settlement/tests/n142_settlement_tests.rs:41:fn n142_deterministic_settlement_root() {
crates/amun-nft-royalty/tests/n136_royalty_tests.rs:47:fn n136_deterministic_evidence_root() {
crates/amun-nft-snapshot/tests/n151_snapshot_tests.rs:130:fn n151_all_roots_deterministic_after_rebuild() {
crates/amun-node/src/bin/test_constitutional_determinism.rs:66:        println!("\nPASS: Constitutional runtime execution is deterministic");
crates/amun-node/src/bin/test_constitutional_multi_block.rs:125:        println!("\nPASS: Constitutional multi-block state evolution is deterministic");
crates/amun-node/src/bin/test_constitutional_mutation.rs:90:        println!("\nPASS: Constitutional state mutation is deterministic");
crates/amun-node/src/genesis.rs:165:    fn n22_3_genesis_hash_deterministic() {
crates/amun-pccv/src/lib.rs:122:        let proof = EnhancedTransitionProof {
crates/amun-pccv/src/lib.rs:174:        let proof = EnhancedTransitionProof {
crates/amun-pccv/src/lib.rs:228:        let proof = EnhancedTransitionProof {
crates/amun-pccv/src/lib.rs:28:        let proof = EnhancedTransitionProof {
crates/amun-pccv/src/lib.rs:67:        let proof = EnhancedTransitionProof {
crates/amun-pccv/src/pccv_verifier.rs:148:    pub fn compute_proof_hash(proof: &EnhancedTransitionProof) -> [u8; 32] {
crates/amun-pccv/src/pccv_verifier.rs:1:use crate::enhanced_proof::EnhancedTransitionProof;
crates/amun-pccv/src/pccv_verifier.rs:20:    pub fn verify(proof: &EnhancedTransitionProof, _registry: &ResourceRegistry) -> PCCVResult {
crates/amun-pccv/src/transition_proof_engine.rs:101:    use crate::enhanced_proof::EnhancedTransitionProof;
crates/amun-pccv/src/transition_proof_engine.rs:105:    use crate::TransitionProofEngine;
crates/amun-pccv/src/transition_proof_engine.rs:11:impl TransitionProofEngine {
crates/amun-pccv/src/transition_proof_engine.rs:154:        let proof = TransitionProofEngine::build_proof(
crates/amun-pccv/src/transition_proof_engine.rs:218:        let mut proof = EnhancedTransitionProof {
crates/amun-pccv/src/transition_proof_engine.rs:22:    ) -> EnhancedTransitionProof {
crates/amun-pccv/src/transition_proof_engine.rs:239:    fn n49b_proof_hash_deterministic() {
crates/amun-pccv/src/transition_proof_engine.rs:253:        let proof1 = TransitionProofEngine::build_proof(
crates/amun-pccv/src/transition_proof_engine.rs:264:        let proof2 = TransitionProofEngine::build_proof(
crates/amun-pccv/src/transition_proof_engine.rs:52:        let mut proof = EnhancedTransitionProof {
crates/amun-pccv/src/transition_proof_engine.rs:5:use crate::enhanced_proof::EnhancedTransitionProof;
crates/amun-pccv/src/transition_proof_engine.rs:82:    ) -> (EnhancedTransitionProof, PCCVResult) {
crates/amun-pccv/src/transition_proof_engine.rs:9:pub struct TransitionProofEngine;
crates/amun-pccv/tests/replay_equivalence.rs:144:            TransitionProofEngine::build_proof(
crates/amun-pccv/tests/replay_equivalence.rs:2:use amun_pccv::transition_proof_engine::TransitionProofEngine;
crates/amun-pccv/tests/replay_equivalence.rs:73:    let proof1 = TransitionProofEngine::build_proof(
crates/amun-pccv/tests/replay_equivalence.rs:84:    let proof2 = TransitionProofEngine::build_proof(
crates/amun-proof-archive/src/hot_store.rs:1:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-proof-archive/src/hot_store.rs:20:    pub fn store(&mut self, proof: TransitionProof, block_height: u64) {
crates/amun-proof-archive/src/hot_store.rs:26:    pub fn get(&self, proof_hash: &[u8; 32]) -> Option<&TransitionProof> {
crates/amun-proof-archive/src/hot_store.rs:66:    fn make_proof(tx_hash: [u8; 32]) -> TransitionProof {
crates/amun-proof-archive/src/hot_store.rs:67:        TransitionProof::new(
crates/amun-proof-archive/src/hot_store.rs:6:    proofs: HashMap<[u8; 32], TransitionProof>,
crates/amun-proof-archive/src/proof_archive.rs:17:    pub fn archive_permanent(&mut self, proof: TransitionProof) {
crates/amun-proof-archive/src/proof_archive.rs:25:    pub fn get_permanent(&self, proof_hash: &[u8; 32]) -> Option<&TransitionProof> {
crates/amun-proof-archive/src/proof_archive.rs:3:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-proof-archive/src/proof_archive.rs:61:    fn make_proof(tx_hash: [u8; 32]) -> TransitionProof {
crates/amun-proof-archive/src/proof_archive.rs:62:        TransitionProof::new(
crates/amun-proof-archive/src/proof_archive.rs:8:    permanent: HashMap<[u8; 32], TransitionProof>,
crates/amun-qc-canonical/src/canonicalize.rs:37:fn is_vote_valid(vote: &ConsensusVote, validator_set: &ValidatorSet) -> bool {
crates/amun-qc-canonical/src/canonicalize.rs:60:pub fn has_quorum_after_canonicalization(
crates/amun-qc-canonical/src/canonicalize.rs:7:pub fn canonicalize_qc(qc: &QuorumCertificate, validator_set: &ValidatorSet) -> QuorumCertificate {
crates/amun-qc-canonical/src/canonicalize.rs:83:    fn test_canonicalize_removes_duplicates() {
crates/amun-qc-canonical/src/validator.rs:5:pub fn validate_qc_validators(
crates/amun-replay-consensus/src/replay_backed_consensus.rs:140:    fn compute_proof_root(transitions: &[TransitionProof]) -> [u8; 32] {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:246:    fn w18_replay_finality_certificate_deterministic() {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:7:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-replay-consensus/src/replay_backed_types.rs:1:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-replay-consensus/src/replay_backed_types.rs:21:/// A block that has been verified through deterministic replay.
crates/amun-replay-consensus/src/replay_backed_types.rs:29:    pub transitions: Vec<TransitionProof>,
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
crates/amun-replay-engine/src/constitutional_governance.rs:125:        // Invariants 1-25 are all constitutional law.
crates/amun-replay-engine/src/constitutional_governance.rs:136:    pub fn can_modify_invariant(&self, invariant_index: u8) -> bool {
crates/amun-replay-engine/src/constitutional_governance.rs:274:    fn test_immutable_invariants() {
crates/amun-replay-engine/src/constitutional_governance.rs:68:    InvariantViolation {
crates/amun-replay-engine/src/constitutional_identity.rs:173:    fn test_identity_fingerprint_deterministic() {
crates/amun-replay-engine/src/cross_constitution_federation.rs:207:    fn test_translation_fingerprint_deterministic() {
crates/amun-replay-engine/src/derivational_equivalence.rs:116:pub fn canonical_order_for_transport(hashes: &mut [ConstitutionalHash]) {
crates/amun-replay-engine/src/derivational_equivalence.rs:141:    fn test_fingerprint_deterministic() {
crates/amun-replay-engine/src/derivational_equivalence.rs:142:        let fp1 = CanonicalDerivationFingerprint::compute([0xAA; 32], true, [0xAB; 32]);
crates/amun-replay-engine/src/derivational_equivalence.rs:143:        let fp2 = CanonicalDerivationFingerprint::compute([0xAA; 32], true, [0xAB; 32]);
crates/amun-replay-engine/src/derivational_equivalence.rs:149:        let fp1 = CanonicalDerivationFingerprint::compute([0xAA; 32], true, [0xAB; 32]);
crates/amun-replay-engine/src/derivational_equivalence.rs:150:        let fp2 = CanonicalDerivationFingerprint::compute([0xAA; 32], false, [0xAB; 32]);
crates/amun-replay-engine/src/derivational_equivalence.rs:20:pub struct CanonicalDerivationFingerprint(pub [u8; 32]);
crates/amun-replay-engine/src/derivational_equivalence.rs:22:impl CanonicalDerivationFingerprint {
crates/amun-replay-engine/src/derivational_equivalence.rs:67:        let fingerprint = CanonicalDerivationFingerprint::compute(
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
crates/amun-replay-engine/src/equivalence.rs:111:            domain: ReplayDomain::Canonical,
crates/amun-replay-engine/src/equivalence.rs:137:            DeterministicExecutor::execute_with_trace(&entries, state.state_root, 1).unwrap();
crates/amun-replay-engine/src/equivalence.rs:21:    pub deterministic_transcript_hash: ConstitutionalHash,
crates/amun-replay-engine/src/equivalence.rs:73:        let trace = DeterministicExecutor::execute_with_trace(
crates/amun-replay-engine/src/execution_dag.rs:126:        dag.add_vertex(ExecutionVertex::new(1, VertexType::StateTransition, 100, [0xAB; 32], [0xBC; 32]));
crates/amun-replay-engine/src/execution_dag.rs:133:        let v1 = ExecutionVertex::new(1, VertexType::StateTransition, 100, [0xAB; 32], [0xBC; 32])
crates/amun-replay-engine/src/execution_dag.rs:135:        let v2 = ExecutionVertex::new(2, VertexType::StateTransition, 100, [0xAB; 32], [0xBC; 32])
crates/amun-replay-engine/src/execution_dag.rs:161:        let v1 = ExecutionVertex::new(1, VertexType::StateTransition, 100, [0xAB; 32], [0xBC; 32])
crates/amun-replay-engine/src/execution_dag.rs:163:        let v2 = ExecutionVertex::new(2, VertexType::StateTransition, 200, [0xAB; 32], [0xBC; 32])
crates/amun-replay-engine/src/execution_scheduler.rs:90:        ExecutionTask::new(id, TaskType::StateTransition, [0xAB; 32], [0xBC; 32], [0xCD; 32], seq)
crates/amun-replay-engine/src/execution_task.rs:14:    StateTransition = 0x01,
crates/amun-replay-engine/src/execution_task.rs:42:    /// For StateTransition: the preceding journal entry.
crates/amun-replay-engine/src/execution_task.rs:79:            TaskType::StateTransition,
crates/amun-replay-engine/src/execution_task.rs:86:        assert_eq!(task.task_type, TaskType::StateTransition);
crates/amun-replay-engine/src/execution_vertex.rs:103:        let v = ExecutionVertex::new(1, VertexType::StateTransition, 100, [0xAB; 32], [0xBC; 32])
crates/amun-replay-engine/src/execution_vertex.rs:19:    StateTransition,
crates/amun-replay-engine/src/isolation_boundary.rs:7://! is a constitutional requirement (Invariant 15).
crates/amun-replay-engine/src/lib.rs:149:            domain: ReplayDomain::Canonical,
crates/amun-replay-engine/src/lib.rs:37:pub use deterministic::{ConstitutionalStep, ExecutionResult, ExecutionTrace};
crates/amun-replay-engine/src/lib.rs:62:        let transcript_hash = DeterministicExecutor::compute_transcript_hash(entries);
crates/amun-replay-engine/src/operational_hasher.rs:49:    fn test_operational_hash_deterministic() {
crates/amun-replay-engine/src/state.rs:103:            domain: ReplayDomain::Canonical,
crates/amun-replay-engine/src/state.rs:108:            domain: ReplayDomain::Canonical,
crates/amun-replay-engine/src/state.rs:73:            domain: ReplayDomain::Canonical,
crates/amun-replay-engine/src/state.rs:87:            domain: ReplayDomain::Canonical,
crates/amun-replay-engine/src/witness_envelope.rs:110:    fn test_envelope_hash_deterministic() {
crates/amun-replay-engine/src/zk_adapters.rs:75:    /// The canonical derivation fingerprint being committed to.
crates/amun-replay-optimization/tests/n163_replay_tests.rs:63:fn n163_cache_root_deterministic() {
crates/amun-replay-semantics/src/lib.rs:1://! Replay Semantics — formal constitutional model for replay.
crates/amun-replay-verifier/src/replay_verifier.rs:32:        proof: &TransitionProof,
crates/amun-replay-verifier/src/replay_verifier.rs:35:        invariants: &[InvariantDeclaration],
crates/amun-replay-verifier/src/replay_verifier.rs:3:use amun_invariant_engine::invariant_types::InvariantDeclaration;
crates/amun-replay-verifier/src/replay_verifier.rs:9:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-replay/src/commit_log.rs:111:    fn n34_commit_hash_deterministic() {
crates/amun-replay/src/commit_log.rs:59:            .expect("commit_log: invariant violated — empty after push")
crates/amun-replay/src/validation.rs:110:    fn n35_replay_result_deterministic() {
crates/amun-sdk-layer/src/tests.rs:15:    fn test_sandbox_simulation() { let mut sandbox = Sandbox::new(); let a0 = sandbox.create_account(1_000_000).data.expect("test invariant"); let a1 = sandbox.create_account(500_000).data.expect("test invariant"); let result = sandbox.simulate_transfer(a0, a1, 100_000); assert!(result.success); }
crates/amun-sdk-layer/src/tests.rs:7:    fn test_token_api_transfer() { let mut token = TokenApi::create_account(1_000_000).data.expect("test invariant"); let result = TokenApi::transfer(&mut token, 100_000); assert!(result.success); }
crates/amun-self-preservation/src/action_principle.rs:47:pub struct LeastInvariantViolation;
crates/amun-self-preservation/src/action_principle.rs:49:impl LeastInvariantViolation {
crates/amun-self-preservation/src/consistency.rs:16:pub enum ConsistencyViolation {
crates/amun-self-preservation/src/consistency.rs:24:        invariant: super::legitimacy_guards::GuardedInvariant,
crates/amun-self-preservation/src/consistency.rs:26:    /// A catastrophic phase transition has occurred
crates/amun-self-preservation/src/consistency.rs:28:    /// The amendment process itself has been invalidated
crates/amun-self-preservation/src/consistency.rs:29:    AmendmentProcessInvalidated,
crates/amun-self-preservation/src/consistency.rs:2:use super::phase_transitions::PhaseTransition;
crates/amun-self-preservation/src/consistency.rs:32:impl MetaConsistency {
crates/amun-self-preservation/src/consistency.rs:33:    pub fn new(constitution_hash: [u8; 32]) -> Self {
crates/amun-self-preservation/src/consistency.rs:42:    pub fn add_guard(&mut self, guard: LegitimacyGuard) {
crates/amun-self-preservation/src/consistency.rs:47:    pub fn verify_consistency(
crates/amun-self-preservation/src/consistency.rs:50:        transition: Option<&PhaseTransition>,
crates/amun-self-preservation/src/consistency.rs:61:        // Check catastrophic transitions
crates/amun-self-preservation/src/consistency.rs:62:        if let Some(t) = transition {
crates/amun-self-preservation/src/consistency.rs:90:    pub fn can_prove_legitimacy(&self) -> bool {
crates/amun-self-preservation/src/consistency.rs:9:pub struct MetaConsistency {
crates/amun-self-preservation/src/legitimacy_guards.rs:13:pub enum GuardedInvariant {
crates/amun-self-preservation/src/legitimacy_guards.rs:27:    InvariantBroken {
crates/amun-self-preservation/src/legitimacy_guards.rs:28:        invariant: GuardedInvariant,
crates/amun-self-preservation/src/legitimacy_guards.rs:38:    pub fn new(invariant: GuardedInvariant, max_violations: u64) -> Self {
crates/amun-self-preservation/src/legitimacy_guards.rs:63:        Some(GuardViolation::InvariantBroken {
crates/amun-self-preservation/src/legitimacy_guards.rs:6:    pub protects: GuardedInvariant,
crates/amun-self-preservation/src/legitimacy_guards.rs:71:    pub fn can_guard(invariant: &GuardedInvariant) -> bool {
crates/amun-self-preservation/src/legitimacy_guards.rs:72:        !matches!(invariant, GuardedInvariant::MetaAmendmentBounds)
crates/amun-self-preservation/src/lib.rs:15:pub use action_principle::{ConstitutionalAction, LeastInvariantViolation};
crates/amun-smt/src/canonical_model.rs:12:pub struct CanonicalModel {
crates/amun-smt/src/canonical_model.rs:16:impl CanonicalModel {
crates/amun-smt/src/canonical_model.rs:17:    pub fn new() -> Self {
crates/amun-smt/src/canonical_model.rs:21:    pub fn insert(&mut self, key: Key256, value_hash: [u8; 32], version: u64) {
crates/amun-smt/src/canonical_model.rs:25:    pub fn delete(&mut self, key: &Key256) {
crates/amun-smt/src/canonical_model.rs:29:    pub fn leaf_count(&self) -> usize {
crates/amun-smt/src/canonical_model.rs:33:    pub fn rebuild(&self) -> Result<SparseMerkleTree, SmtError> {
crates/amun-smt/src/canonical_model.rs:45:pub fn assert_equivalent(
crates/amun-smt/src/canonical_model.rs:50:    let canonical = model.rebuild().map_err(|e| format!("Rebuild failed: {:?}", e))?;
crates/amun-smt/src/canonical_model.rs:6:pub struct LeafEntry {
crates/amun-smt/src/hash.rs:85:pub fn canonicalize_prefix(prefix: &mut [u8; 32], skip_len: u8) {
crates/amun-smt/src/node.rs:16:/// Canonical hash of an empty subtree — computed once.
crates/amun-smt/src/node.rs:3://! # Invariants (enforced by tree, verified by validator)
crates/amun-smt/src/tree.rs:149:                    return self.build_canonical_subtree(&new_leaves, depth);
crates/amun-smt/src/tree.rs:348:    /// Uses canonical subtree rebuild to guarantee correct topology.
crates/amun-smt/src/tree.rs:365:        // Rebuild the subtree canonically from the surviving leaves.
crates/amun-smt/src/tree.rs:368:        self.build_canonical_subtree(&leaves, depth)
crates/amun-smt/src/tree.rs:3://! # Core Invariant
crates/amun-smt/src/tree.rs:665:    fn build_canonical_subtree(
crates/amun-smt/src/tree.rs:703:        let lh = self.build_canonical_subtree(&ll, next_depth + 1)?;
crates/amun-smt/src/tree.rs:704:        let rh = self.build_canonical_subtree(&rl, next_depth + 1)?;
crates/amun-smt/src/validator.rs:117:            // Invariant: maximal skip (canonical minimality)
crates/amun-smt/src/validator.rs:46:            // Invariant: no empty children
crates/amun-smt/src/validator.rs:51:            // Invariant: skip_len within bounds
crates/amun-smt/tests/identity_laws.rs:22:/// delete(k) + insert(k,v) == canonical rebuild with {k,v}
crates/amun-smt/tests/identity_laws.rs:24:fn law_delete_insert_canonicality() {
crates/amun-smt/tests/identity_laws.rs:41:    // Should match canonical rebuild
crates/amun-snapshot-constitution/src/snapshot.rs:49:pub struct CanonicalSnapshot {
crates/amun-snapshot-constitution/src/snapshot.rs:59:impl CanonicalSnapshot {
crates/amun-snapshot-engine/src/manifest.rs:183:    pub fn with_canonical_empty_root(mut self, root: [u8; 32]) -> Self {
crates/amun-snapshot-engine/src/replay_continuity.rs:28:    /// THEOREM 11: Verify that restoring a snapshot and replaying the
crates/amun-snapshot-engine/src/replay_continuity.rs:2:// THEOREM 11: state -> snapshot -> restore -> WAL replay -> final_root
crates/amun-snapshot-engine/src/traversal.rs:12:pub struct CanonicalTraversal {
crates/amun-snapshot-engine/src/traversal.rs:16:impl CanonicalTraversal {
crates/amun-snapshot-engine/src/verifier.rs:3:// canonical empty root, constitutional hash, and replay equivalence.
crates/amun-snapshot-engine/tests/constitutional_tests.rs:76:    fn constitutional_hash_is_deterministic() {
crates/amun-soak-full/tests/n165_full_soak_tests.rs:43:fn n165_state_consistency_under_full_load() {
crates/amun-soak-test/tests/n165_soak_tests.rs:36:fn n165_state_consistency_under_load() {
crates/amun-state-machine/src/absolute_invariants.rs:13:    /// Replay determinism must be preserved across ALL amendments
crates/amun-state-machine/src/absolute_invariants.rs:14:    ReplayDeterminismAbsolute,
crates/amun-state-machine/src/absolute_invariants.rs:19:    /// Every state transition must be cryptographically provable
crates/amun-state-machine/src/absolute_invariants.rs:26:    MetaConstitutionalImmutability,
crates/amun-state-machine/src/absolute_invariants.rs:29:impl AbsoluteInvariant {
crates/amun-state-machine/src/absolute_invariants.rs:31:    pub fn level(&self) -> u8 {
crates/amun-state-machine/src/absolute_invariants.rs:36:    pub fn all() -> Vec<AbsoluteInvariant> {
crates/amun-state-machine/src/absolute_invariants.rs:41:            Self::ReplayDeterminismAbsolute,
crates/amun-state-machine/src/absolute_invariants.rs:47:            Self::MetaConstitutionalImmutability,
crates/amun-state-machine/src/absolute_invariants.rs:6:pub enum AbsoluteInvariant {
crates/amun-state-machine/src/axioms.rs:10:    /// Every transition preserves constitutional identity or provides an identity delta
crates/amun-state-machine/src/axioms.rs:12:    /// Replay determinism is preserved across all legal transitions
crates/amun-state-machine/src/axioms.rs:13:    ReplayDeterminismPreserved,
crates/amun-state-machine/src/axioms.rs:14:    /// No transition can decrease the epoch
crates/amun-state-machine/src/axioms.rs:16:    /// No transition can decrease generation within the same epoch
crates/amun-state-machine/src/axioms.rs:18:    /// Constitutional freeze boundaries are immutable unless explicitly amended
crates/amun-state-machine/src/axioms.rs:1:/// Constitutional axioms - the foundational mathematical truths
crates/amun-state-machine/src/axioms.rs:21:    EmptyRootInvariant,
crates/amun-state-machine/src/axioms.rs:26:    /// Hostile forks cannot preserve replay
crates/amun-state-machine/src/axioms.rs:27:    HostileForkReplayImpossible,
crates/amun-state-machine/src/axioms.rs:30:impl ConstitutionalAxiom {
crates/amun-state-machine/src/axioms.rs:31:    pub fn description(&self) -> &'static str {
crates/amun-state-machine/src/axioms.rs:36:                "Every transition preserves or explicitly changes identity"
crates/amun-state-machine/src/axioms.rs:38:            Self::ReplayDeterminismPreserved => {
crates/amun-state-machine/src/axioms.rs:39:                "Replay must remain deterministic across all transitions"
crates/amun-state-machine/src/axioms.rs:44:            Self::EmptyRootInvariant => "Empty root is invariant across protocol versions",
crates/amun-state-machine/src/axioms.rs:47:            Self::HostileForkReplayImpossible => "Hostile forks cannot preserve replay",
crates/amun-state-machine/src/axioms.rs:52:/// Axiom verification result.
crates/amun-state-machine/src/axioms.rs:54:pub enum AxiomVerification {
crates/amun-state-machine/src/axioms.rs:57:        axiom: ConstitutionalAxiom,
crates/amun-state-machine/src/axioms.rs:5:pub enum ConstitutionalAxiom {
crates/amun-state-machine/src/axioms.rs:62:/// The ConstitutionalAxiomEngine verifies that all axioms hold for a given state.
crates/amun-state-machine/src/axioms.rs:63:pub struct ConstitutionalAxiomEngine;
crates/amun-state-machine/src/axioms.rs:65:impl ConstitutionalAxiomEngine {
crates/amun-state-machine/src/axioms.rs:66:    pub fn verify_all() -> Vec<AxiomVerification> {
crates/amun-state-machine/src/axioms.rs:68:            AxiomVerification::Holds,
crates/amun-state-machine/src/axioms.rs:69:            AxiomVerification::Holds,
crates/amun-state-machine/src/axioms.rs:70:            AxiomVerification::Holds,
crates/amun-state-machine/src/axioms.rs:71:            AxiomVerification::Holds,
crates/amun-state-machine/src/axioms.rs:72:            AxiomVerification::Holds,
crates/amun-state-machine/src/axioms.rs:73:            AxiomVerification::Holds,
crates/amun-state-machine/src/axioms.rs:74:            AxiomVerification::Holds,
crates/amun-state-machine/src/axioms.rs:75:            AxiomVerification::Holds,
crates/amun-state-machine/src/axioms.rs:76:            AxiomVerification::Holds,
crates/amun-state-machine/src/axioms.rs:77:            AxiomVerification::Holds,
crates/amun-state-machine/src/axioms.rs:78:            AxiomVerification::Holds,
crates/amun-state-machine/src/delta_algebra.rs:3:/// Formal constitutional delta types.
crates/amun-state-machine/src/delta_algebra.rs:40:    pub fn canonical_tag(&self) -> u8 {
crates/amun-state-machine/src/derivation.rs:1:use super::axioms::ConstitutionalAxiom;
crates/amun-state-machine/src/derivation.rs:29:    pub theorem: Theorem,
crates/amun-state-machine/src/derivation.rs:35:    Axiom(ConstitutionalAxiom),
crates/amun-state-machine/src/derivation.rs:37:    TheoremApplication(usize),
crates/amun-state-machine/src/derivation.rs:42:pub struct ConsistencyProof {
crates/amun-state-machine/src/derivation.rs:43:    pub axioms_checked: Vec<ConstitutionalAxiom>,
crates/amun-state-machine/src/derivation.rs:5:pub struct Theorem {
crates/amun-state-machine/src/derivation.rs:8:    pub depends_on: Vec<ConstitutionalAxiom>,
crates/amun-state-machine/src/engine.rs:123:        invariants: &[ConstitutionalInvariant],
crates/amun-state-machine/src/engine.rs:130:                ConstitutionalInvariant::NoImpossibleState => {
crates/amun-state-machine/src/engine.rs:137:                ConstitutionalInvariant::TransitionHistoryAcyclic => {
crates/amun-state-machine/src/engine.rs:142:                ConstitutionalInvariant::LineageIntact
crates/amun-state-machine/src/engine.rs:1:use super::historical_invariants::HistoricalInvariantEngine;
crates/amun-state-machine/src/engine.rs:24:    pub history: HistoricalInvariantEngine,
crates/amun-state-machine/src/engine.rs:2:use super::invariants::ConstitutionalInvariant;
crates/amun-state-machine/src/engine.rs:37:            history: HistoricalInvariantEngine::new(),
crates/amun-state-machine/src/fork_merge.rs:145:    pub fn canonical_tag(&self) -> u8 {
crates/amun-state-machine/src/fork_merge.rs:66:    pub fn canonical_tag(&self) -> u8 {
crates/amun-state-machine/src/formal_entropy.rs:16:pub struct EntropySink {
crates/amun-state-machine/src/formal_entropy.rs:22:pub enum EntropySinkType {
crates/amun-state-machine/src/formal_entropy.rs:27:    /// Replay convergence absorbs entropy
crates/amun-state-machine/src/formal_entropy.rs:28:    ReplayConvergence,
crates/amun-state-machine/src/formal_entropy.rs:29:    /// Constitutional court ruling absorbs entropy
crates/amun-state-machine/src/formal_entropy.rs:35:pub struct EntropyConservationLaws;
crates/amun-state-machine/src/formal_entropy.rs:37:impl EntropyConservationLaws {
crates/amun-state-machine/src/formal_entropy.rs:39:    pub fn first_law(entropy: &FormalEntropy) -> bool {
crates/amun-state-machine/src/formal_entropy.rs:45:    pub fn second_law(entropy: &FormalEntropy) -> bool {
crates/amun-state-machine/src/formal_entropy.rs:4:pub struct FormalEntropy {
crates/amun-state-machine/src/formal_entropy.rs:50:    pub fn third_law(entropy: &FormalEntropy) -> bool {
crates/amun-state-machine/src/formal_entropy.rs:58:pub struct EntropyCollapseThreshold {
crates/amun-state-machine/src/formal_entropy.rs:64:impl Default for EntropyCollapseThreshold {
crates/amun-state-machine/src/formal_entropy.rs:65:    fn default() -> Self {
crates/amun-state-machine/src/historical_invariants.rs:102:    pub fn record_freeze_unfreeze(&mut self) {
crates/amun-state-machine/src/historical_invariants.rs:105:    pub fn record_amendment(&mut self) {
crates/amun-state-machine/src/historical_invariants.rs:108:    pub fn record_replay_divergence(&mut self, bytes: u64) {
crates/amun-state-machine/src/historical_invariants.rs:109:        self.cumulative_replay_divergence += bytes;
crates/amun-state-machine/src/historical_invariants.rs:111:    pub fn update_governance_rank(&mut self, rank: u8) {
crates/amun-state-machine/src/historical_invariants.rs:115:    pub fn new_epoch(&mut self, epoch: u64) {
crates/amun-state-machine/src/historical_invariants.rs:123:    pub fn check_all(&self) -> Result<(), Vec<String>> {
crates/amun-state-machine/src/historical_invariants.rs:125:            HistoricalInvariant::MaxFreezeUnfreezeCycles {
crates/amun-state-machine/src/historical_invariants.rs:129:            HistoricalInvariant::MaxAmendmentsPerEpoch {
crates/amun-state-machine/src/historical_invariants.rs:133:            HistoricalInvariant::MaxReplayDivergence {
crates/amun-state-machine/src/historical_invariants.rs:135:                current: self.cumulative_replay_divergence,
crates/amun-state-machine/src/historical_invariants.rs:137:            HistoricalInvariant::MinGovernanceCompatibility {
crates/amun-state-machine/src/historical_invariants.rs:14:    /// Replay divergence must not accumulate over lineage
crates/amun-state-machine/src/historical_invariants.rs:15:    MaxReplayDivergence {
crates/amun-state-machine/src/historical_invariants.rs:1:/// Historical invariants that span multiple transitions.
crates/amun-state-machine/src/historical_invariants.rs:23:impl HistoricalInvariant {
crates/amun-state-machine/src/historical_invariants.rs:24:    pub fn check(&self) -> Result<(), String> {
crates/amun-state-machine/src/historical_invariants.rs:26:            HistoricalInvariant::MaxFreezeUnfreezeCycles {
crates/amun-state-machine/src/historical_invariants.rs:37:            HistoricalInvariant::MaxAmendmentsPerEpoch {
crates/amun-state-machine/src/historical_invariants.rs:3:pub enum HistoricalInvariant {
crates/amun-state-machine/src/historical_invariants.rs:48:            HistoricalInvariant::MaxReplayDivergence {
crates/amun-state-machine/src/historical_invariants.rs:54:                        "Replay divergence accumulation exceeded: {} > {}",
crates/amun-state-machine/src/historical_invariants.rs:59:            HistoricalInvariant::MinGovernanceCompatibility {
crates/amun-state-machine/src/historical_invariants.rs:75:/// Engine that tracks historical invariants across multiple transitions.
crates/amun-state-machine/src/historical_invariants.rs:77:pub struct HistoricalInvariantEngine {
crates/amun-state-machine/src/historical_invariants.rs:80:    pub cumulative_replay_divergence: u64,
crates/amun-state-machine/src/historical_invariants.rs:85:impl Default for HistoricalInvariantEngine {
crates/amun-state-machine/src/historical_invariants.rs:86:    fn default() -> Self {
crates/amun-state-machine/src/historical_invariants.rs:91:impl HistoricalInvariantEngine {
crates/amun-state-machine/src/historical_invariants.rs:92:    pub fn new() -> Self {
crates/amun-state-machine/src/historical_invariants.rs:96:            cumulative_replay_divergence: 0,
crates/amun-state-machine/src/invariants.rs:10:    /// Replay determinism must be preserved.
crates/amun-state-machine/src/invariants.rs:11:    ReplayDeterminismPreserved,
crates/amun-state-machine/src/invariants.rs:14:    /// The transition history must be acyclic (no state loops).
crates/amun-state-machine/src/invariants.rs:16:    /// Constitutional identity must not have been silently mutated.
crates/amun-state-machine/src/invariants.rs:1:/// Constitutional invariants that must be preserved across ALL transitions.
crates/amun-state-machine/src/invariants.rs:3:pub enum ConstitutionalInvariant {
crates/amun-state-machine/src/invariants.rs:7:    ConstitutionConsistent,
crates/amun-state-machine/src/lib.rs:29:pub use absolute_invariants::AbsoluteInvariant;
crates/amun-state-machine/src/lib.rs:30:pub use axioms::{AxiomVerification, ConstitutionalAxiom, ConstitutionalAxiomEngine};
crates/amun-state-machine/src/lib.rs:34:pub use derivation::{ConsistencyProof, Derivation, DerivationStep, InferenceRule, Theorem};
crates/amun-state-machine/src/lib.rs:40:    EntropyCollapseThreshold, EntropyConservationLaws, EntropySink, EntropySinkType, FormalEntropy,
crates/amun-state-machine/src/lib.rs:42:pub use historical_invariants::{HistoricalInvariant, HistoricalInvariantEngine};
crates/amun-state-machine/src/lib.rs:44:pub use invariants::ConstitutionalInvariant;
crates/amun-state-machine/src/lib.rs:55:    Transition, TransitionAlgebra, TransitionId, TransitionProof, TransitionType,
crates/amun-state-machine/src/meta_amendment.rs:1:use super::absolute_invariants::AbsoluteInvariant;
crates/amun-state-machine/src/meta_amendment.rs:47:        absolute_invariants: &[AbsoluteInvariant],
crates/amun-state-machine/src/meta_amendment.rs:54:                // Must verify that no absolute invariant is touched
crates/amun-state-machine/src/meta_amendment.rs:57:                        AbsoluteInvariant::ReplayDeterminismAbsolute => {
crates/amun-state-machine/src/meta_amendment.rs:65:                        AbsoluteInvariant::ProvableTransitionAbsolute => {
crates/amun-state-machine/src/meta_amendment.rs:9:    pub absolute_invariants_untouchable: Vec<AbsoluteInvariant>,
crates/amun-state-machine/src/postconditions.rs:15:    /// Constitutional invariants must still hold.
crates/amun-state-machine/src/postconditions.rs:16:    InvariantsPreserved,
crates/amun-state-machine/src/states.rs:18:    pub fn canonical_tag(&self) -> u8 {
crates/amun-state-machine/src/transitions.rs:166:        h.update(&[self.transition_type.canonical_tag()]);
crates/amun-state-machine/src/transitions.rs:202:pub struct TransitionProof {
crates/amun-state-machine/src/transitions.rs:21:    pub fn canonical_tag(&self) -> u8 {
crates/amun-state-machine/src/verifier.rs:104:                ConstitutionalInvariant::TransitionHistoryAcyclic => {
crates/amun-state-machine/src/verifier.rs:109:                ConstitutionalInvariant::LineageIntact
crates/amun-state-machine/src/verifier.rs:1:use super::invariants::ConstitutionalInvariant;
crates/amun-state-machine/src/verifier.rs:89:    pub fn verify_invariants(
crates/amun-state-machine/src/verifier.rs:92:        invariants: &[ConstitutionalInvariant],
crates/amun-state-machine/src/verifier.rs:97:                ConstitutionalInvariant::NoImpossibleState => {
crates/amun-state-pruning/tests/n166_pruning_tests.rs:109:fn n166_pruned_root_deterministic() {
crates/amun-state-root/src/identity.rs:14:impl CanonicalEncode for ChainIdentityRoot {
crates/amun-state-root/src/identity.rs:15:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-state-root/src/replay.rs:10:impl CanonicalEncode for ReplayEquivalenceProof {
crates/amun-state-root/src/replay.rs:11:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-state-root/src/replay.rs:29:impl CanonicalEncode for ReplayTranscript {
crates/amun-state-root/src/replay.rs:30:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-state-root/src/replay.rs:47:impl CanonicalEncode for ReplayCertificate {
crates/amun-state-root/src/replay.rs:48:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-state-root/src/root.rs:12:impl CanonicalEncode for StateLeaf {
crates/amun-state-root/src/root.rs:13:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-state-root/src/snapshot.rs:40:impl CanonicalEncode for ReplayEquivalenceProof {
crates/amun-state-root/src/snapshot.rs:41:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-state-root/src/snapshot.rs:48:impl CanonicalEncode for ConstitutionalSnapshot {
crates/amun-state-root/src/snapshot.rs:49:    fn encode_canonical(&self, out: &mut Vec<u8>) {
crates/amun-state-root/src/snapshot.rs:57:        self.replay_equivalence_proof.encode_canonical(out);
crates/amun-state-transition/src/state.rs:48:    pub fn commit_to(self, target: &mut DeterministicMap<[u8; 32], Vec<u8>>) {
crates/amun-stf/src/lib.rs:13:pub use transition::StateTransition;
crates/amun-stf/src/receipt.rs:33:impl CanonicalEncode for ExecutionReceipt {
crates/amun-stf/src/receipt.rs:35:    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
crates/amun-stf/src/receipt.rs:45:impl CanonicalDecode for ExecutionReceipt {
crates/amun-stf/src/state.rs:75:    fn assert_canonical_order(&self) {
crates/amun-stf/src/tests.rs:14:    stf.apply_set(key, val).expect("test invariant");
crates/amun-stf/src/tests.rs:15:    let new_root = stf.commit().expect("test invariant");
crates/amun-stf/src/tests.rs:27:    stf.apply_set(key, val).expect("test invariant");
crates/amun-stf/src/tests.rs:33:fn test_stf_deterministic_root() {
crates/amun-stf/src/tests.rs:43:        stf2.apply_set(k, v).expect("test invariant");
crates/amun-stf/src/tests.rs:45:    let r1 = stf1.commit().expect("test invariant");
crates/amun-stf/src/tests.rs:46:    let r2 = stf2.commit().expect("test invariant");
crates/amun-stf/src/tests.rs:53:fn test_apply_block_deterministic() {}
crates/amun-stf/src/tests.rs:57:fn test_root_deterministic() {}
crates/amun-stf/src/transition.rs:10:impl StateTransition {
crates/amun-stf/src/transition.rs:8:pub struct StateTransition;
crates/amun-stf/src/transition_result.rs:52:    receipt.verify_consistency().map_err(alloc::string::String::from)?;
crates/amun-stf/tests/integration_test.rs:27:    assert!(receipt.verify_consistency().is_ok());
crates/amun-storage-kernel/CANONICAL_TRAVERSAL_LAW.md:17:- Proof step ordering within Merkle proofs
crates/amun-storage-kernel/CANONICAL_TRAVERSAL_LAW.md:2:## Formal specification of deterministic state traversal
crates/amun-storage-kernel/CANONICAL_TRAVERSAL_LAW.md:70:4. Within same NodeHash (impossible per Article V of Constitution),
crates/amun-storage-kernel/CANONICAL_TRAVERSAL_LAW.md:79:**Section 4.3: Proof Step Order**
crates/amun-storage-kernel/CONSTITUTION.md:108:Ratified upon verification of all 10 Constitutional Theorems.
crates/amun-storage-kernel/CONSTITUTION.md:85:**Section 5.1: Uniqueness Invariant**
crates/amun-storage-kernel/SNAPSHOT_CONSTITUTION.md:74:**Section 4.2: Consistency Proof**
crates/amun-storage-kernel/VALIDITY_HIERARCHY.md:2:## Formal taxonomy of state validity and failure modes
crates/amun-storage-kernel/src/canonical/mod.rs:107:    pub fn is_finished(&self) -> bool {
crates/amun-storage-kernel/src/canonical/mod.rs:11:impl Encoder {
crates/amun-storage-kernel/src/canonical/mod.rs:12:    pub fn new() -> Self {
crates/amun-storage-kernel/src/canonical/mod.rs:15:    pub fn write_u8(&mut self, v: u8) {
crates/amun-storage-kernel/src/canonical/mod.rs:18:    pub fn write_u16(&mut self, v: u16) {
crates/amun-storage-kernel/src/canonical/mod.rs:1:pub struct Encoder {
crates/amun-storage-kernel/src/canonical/mod.rs:21:    pub fn write_u32(&mut self, v: u32) {
crates/amun-storage-kernel/src/canonical/mod.rs:24:    pub fn write_u64(&mut self, v: u64) {
crates/amun-storage-kernel/src/canonical/mod.rs:27:    pub fn write_bytes(&mut self, bytes: &[u8]) {
crates/amun-storage-kernel/src/canonical/mod.rs:31:    pub fn write_hash(&mut self, hash: &[u8; 32]) {
crates/amun-storage-kernel/src/canonical/mod.rs:34:    pub fn into_bytes(self) -> Vec<u8> {
crates/amun-storage-kernel/src/canonical/mod.rs:39:pub struct Decoder<'a> {
crates/amun-storage-kernel/src/canonical/mod.rs:44:    pub fn new(data: &'a [u8]) -> Self {
crates/amun-storage-kernel/src/canonical/mod.rs:47:    pub fn read_u8(&mut self) -> Option<u8> {
crates/amun-storage-kernel/src/canonical/mod.rs:56:    pub fn read_u16(&mut self) -> Option<u16> {
crates/amun-storage-kernel/src/canonical/mod.rs:5:impl Default for Encoder {
crates/amun-storage-kernel/src/canonical/mod.rs:65:    pub fn read_u32(&mut self) -> Option<u32> {
crates/amun-storage-kernel/src/canonical/mod.rs:6:    fn default() -> Self {
crates/amun-storage-kernel/src/canonical/mod.rs:79:    pub fn read_u64(&mut self) -> Option<u64> {
crates/amun-storage-kernel/src/canonical/mod.rs:88:    pub fn read_bytes(&mut self) -> Option<Vec<u8>> {
crates/amun-storage-kernel/src/canonical/mod.rs:98:    pub fn read_hash(&mut self) -> Option<[u8; 32]> {
crates/amun-storage-kernel/src/smt/proof.rs:7:pub fn canonical_empty_root() -> [u8; 32] {
crates/amun-storage-kernel/src/smt/tree.rs:128:                    return self.build_canonical_branch(
crates/amun-storage-kernel/src/smt/tree.rs:171:    fn build_canonical_branch(
crates/amun-storage-kernel/src/smt/tree.rs:198:        let child = self.build_canonical_branch(
crates/amun-storage-kernel/src/smt/tree.rs:44:    pub fn canonical_empty_root() -> [u8; 32] {
crates/amun-storage-kernel/tests/canonical_constants.rs:10:            CANONICAL_EMPTY_ROOT_V1, computed,
crates/amun-storage-kernel/tests/canonical_constants.rs:7:    fn canonical_empty_root_matches_runtime() {
crates/amun-storage-kernel/tests/canonical_constants.rs:8:        let computed = SparseMerkleTree::canonical_empty_root();
crates/amun-storage-kernel/tests/delete_equivalence.rs:6:    fn insert_delete_returns_to_canonical_empty() {
crates/amun-storage-kernel/tests/ladder_consistency.rs:7:    fn empty_ladder_matches_node_hash() {
crates/amun-storage-kernel/tests/specification_compliance.rs:105:            "Theorem 5 violated: delete nonexistent changed root"
crates/amun-storage-kernel/tests/specification_compliance.rs:110:    // THEOREM 6: Proof Verification Roundtrip
crates/amun-storage-kernel/tests/specification_compliance.rs:113:    fn theorem_proof_roundtrip() {
crates/amun-storage-kernel/tests/specification_compliance.rs:122:            "Theorem 6 violated: proof verification failed"
crates/amun-storage-kernel/tests/specification_compliance.rs:130:    fn theorem_terminal_empty_is_zero() {
crates/amun-storage-kernel/tests/specification_compliance.rs:134:            "Theorem 7 violated: terminal empty != ZERO"
crates/amun-storage-kernel/tests/specification_compliance.rs:139:    // THEOREM 8: Empty Tree Absence Proof
crates/amun-storage-kernel/tests/specification_compliance.rs:142:    fn theorem_empty_tree_absence_proof() {
crates/amun-storage-kernel/tests/specification_compliance.rs:149:            "Theorem 8 violated: empty tree absence proof"
crates/amun-storage-kernel/tests/specification_compliance.rs:25:        assert_eq!(root_a.0, root_b.0, "Theorem 1 violated: order independence");
crates/amun-storage-kernel/tests/specification_compliance.rs:32:    fn theorem_delete_reinsert_identity() {
crates/amun-storage-kernel/tests/specification_compliance.rs:45:            "Theorem 2 violated: delete-reinsert identity"
crates/amun-storage-kernel/tests/specification_compliance.rs:53:    fn theorem_empty_identity() {
crates/amun-storage-kernel/tests/specification_compliance.rs:62:            "Theorem 3 violated: empty identity"
crates/amun-storage-kernel/tests/specification_compliance.rs:67:    // THEOREM 4: Proof Depth Invariant
crates/amun-storage-kernel/tests/specification_compliance.rs:70:    fn theorem_proof_depth() {
crates/amun-storage-kernel/tests/specification_compliance.rs:79:            "Theorem 4 violated: proof depth != 256"
crates/amun-storage-kernel/tests/specification_compliance.rs:86:            "Theorem 4 violated: absence proof depth != 256"
crates/amun-storage-kernel/tests/specification_compliance.rs:94:    fn theorem_delete_nonexistent_noop() {
crates/amun-storage-kernel/tests/specification_compliance.rs:9:    fn theorem_order_independence_two_keys() {
crates/amun-survival-console/src/dashboard.rs:10:    invariant_registry: InvariantRegistry,
crates/amun-survival-console/src/dashboard.rs:27:        invariant_registry: InvariantRegistry,
crates/amun-survival-console/src/dashboard.rs:59:        let invariants_hold = health == InvariantHealth::AllInvariantsHold;
crates/amun-survival-console/src/dashboard.rs:6:use amun_invariants::registry::{InvariantHealth, InvariantRegistry};
crates/amun-tokenomics-ledger/src/lib.rs:119:    // Batch update — canonical way to apply block-level economics
crates/amun-tokenomics-ledger/tests/test_ledger.rs:46:fn test_ledger_root_deterministic() {
crates/amun-transaction/src/tests.rs:110:    assert!(r.expect("test invariant").validate_basic().is_err());
crates/amun-transaction/src/tests.rs:64:    assert!(r.expect("test invariant").validate_basic().is_ok());
crates/amun-transaction/src/tests.rs:79:    assert!(r.expect("test invariant").validate_basic().is_ok());
crates/amun-transaction/src/tests.rs:94:    assert!(r.expect("test invariant").validate_basic().is_ok());
crates/amun-transactions/src/lib.rs:104:    fn n23_tx_hash_deterministic() {
crates/amun-transition-proof/src/enhanced_proof.rs:77:pub struct EnhancedTransitionProof {
crates/amun-transition-proof/src/proof_builder.rs:203:    fn compute_enhanced_hash(proof: &EnhancedTransitionProof) -> [u8; 32] {
crates/amun-transition-proof/src/proof_builder.rs:22:    ) -> TransitionProof {
crates/amun-transition-proof/src/proof_builder.rs:39:        TransitionProof::new(
crates/amun-transition-proof/src/proof_builder.rs:64:    ) -> EnhancedTransitionProof {
crates/amun-transition-proof/src/proof_builder.rs:7:use crate::enhanced_proof::{EnhancedTransitionProof, LineageProof, MerkleProof, WitnessBundle};
crates/amun-transition-proof/src/proof_builder.rs:8:use crate::transition_proof::TransitionProof;
crates/amun-transition-proof/src/proof_builder.rs:99:        let mut proof = EnhancedTransitionProof {
crates/amun-transition-proof/src/transition_proof.rs:13:pub struct TransitionProof {
crates/amun-transition-proof/src/transition_proof.rs:28:impl TransitionProof {
crates/amun-truth-engine/src/engine.rs:231:    pub fn import_snapshot(&mut self, snapshot: &CanonicalSnapshot) -> Result<(), &'static str> {
crates/amun-validator-identity/src/validator_id.rs:31:    fn n105_validator_id_deterministic() {
crates/amun-verification-kernel/src/lib.rs:50:    FormalProof,
crates/amun-verified-pipeline/src/vote_pipeline.rs:23:pub struct CanonicalVote {
crates/amun-verified-pipeline/src/vote_pipeline.rs:60:    pub fn canonicalize(&self) -> Result<CanonicalVote, &'static str> {
crates/amun-vm-kernel/src/pending_buffer.rs:30:    InvariantViolation {
crates/amun-wallet-management/src/lib.rs:19:    fn key02_deterministic_seed_import() {
crates/amun_consensus_math/tests/replay_binary.rs:91:fn test_consistency_across_calls() {
crates/amun_consensus_math/tests/saturation.rs:33:fn test_floor_division_consistency() {
crates/amun_state_machine/src/certification/mod.rs:35:    pub fn canonical_bytes(&self) -> Vec<u8> {
crates/amun_state_machine/src/certification/mod.rs:89:    pub fn canonical_bytes(&self) -> Vec<u8> {
crates/amun_state_machine/src/event.rs:37:/// Constitutional event with deterministic serialization
crates/amun_state_machine/src/ordering.rs:12:pub struct CanonicalOrderKey {
crates/amun_state_machine/src/ordering.rs:19:impl CanonicalOrderKey {
crates/amun_state_machine/src/ordering.rs:3://! Canonical Ordering Constitution
crates/amun_state_machine/src/ordering.rs:40:pub struct CanonicalOrdering;
crates/amun_state_machine/src/ordering.rs:42:impl CanonicalOrdering {
crates/amun_state_machine/src/ordering.rs:53:    pub fn is_canonically_ordered(events: &[Event], block_height: u64) -> bool {
crates/amun_state_machine/src/ordering.rs:86:    fn test_canonical_ordering() {
crates/amun_state_machine/src/receipt/mod.rs:71:    pub fn canonical_bytes(&self) -> Vec<u8> {
crates/amun_state_machine/src/scheduler.rs:10:pub struct DeterministicScheduler {
crates/amun_state_machine/src/scheduler.rs:15:impl DeterministicScheduler {
crates/amun_state_machine/src/scheduler.rs:84:impl Default for DeterministicScheduler {
crates/amun_state_machine/src/scheduler.rs:97:    fn test_scheduler_deterministic() {
crates/amun_state_machine/src/state.rs:109:    fn test_state_hash_deterministic() {
crates/amun_state_machine/src/state.rs:26:    pub fn canonical_bytes(&self) -> Vec<u8> {
docs/CANONICAL_HASH.md:19:| Transcript | `AMUN_TRANSCRIPT_V1` | Replay transcript |
docs/CANONICAL_HASH.md:6:- Constitutional identities
docs/CANONICAL_HASH.md:8:- Replay certificates
docs/CANONICAL_SERIALIZATION.md:26:### Constitutional Limits
docs/CANONICAL_SERIALIZATION.md:3:## Constitutional Law
docs/CCS_Core_Specification_v1.0.md:156:## 5. The Central Theorem
docs/CCS_Core_Specification_v1.0.md:158:**Constitutional Authority Uniqueness Theorem:**
docs/CCS_Core_Specification_v1.0.md:71:## 3. Axioms
docs/CCS_Core_Specification_v1.0.md:73:### Axiom 1: Context Dominance
docs/CCS_Core_Specification_v1.0.md:79:### Axiom 2: Constitutional Determinism
docs/CCS_Core_Specification_v1.0.md:85:### Axiom 3: Authority Uniqueness
docs/CCS_Core_Specification_v1.0.md:91:### Axiom 4: Constitutional Recoverability
docs/CCS_Core_Specification_v1.1.md:136:## 5. Theorems (Proven Consequences of the Axioms)
docs/CCS_Core_Specification_v1.1.md:138:### Theorem 1: Authority Uniqueness
docs/CCS_Core_Specification_v1.1.md:143:Axiom 2 (Constitutional Determinism).
docs/CCS_Core_Specification_v1.1.md:2:## Core Specification v1.1 – Final Foundational Axioms
docs/CCS_Core_Specification_v1.1.md:5:**Status:** Theory Nucleus – Independent Foundational Axioms
docs/CCS_Core_Specification_v1.1.md:69:## 3. Axioms (Foundational Assumptions of the Theory)
docs/CCS_Core_Specification_v1.1.md:71:### Axiom 1: Context Dominance
docs/CCS_Core_Specification_v1.1.md:77:### Axiom 2: Constitutional Determinism (Foundational Axiom)
docs/CCS_Core_Specification_v1.1.md:86:### Axiom 3: Constitutional Recoverability
docs/CCS_RUNTIME_MAPPING.md:19:| SH1 | Finality rules | One canonical chain survives |
docs/CONSTITUTIONAL_AUDIT.md:22:| **Constitutional Comparability (CC)** | ✅ Partially implemented | Evidence ordering exists but not formalized as standalone module |
docs/CONSTITUTIONAL_AUDIT.md:25:| **Formal Verification** | ❌ Not implemented | TLA+ specification exists in docs but not machine-checked |
docs/CONSTITUTIONAL_AUDIT.md:31:1. **Formal Verification (TLA+)** – CCS axioms documented but not mechanically verified
docs/CONSTITUTIONAL_AUDIT.md:37:5. **Comparability Formalization** – Evidence ordering implicit; should be explicit
docs/DOCS_INDEX.md:308:Protocol Laws & Formal Specifications
docs/DOCS_INDEX.md:365:    v2.027: Formal constitutional state.
docs/DOCS_INDEX.md:429:    Appendix A: Formal definitions and proofs.
docs/N126_FINAL_BASELINE.md:34:9.  StateTransitionValidity   = cert.state_root != [0u8; 32]           PARTIAL
docs/PROJECT_INDEX.md:141:## Formal Models
docs/PROJECT_INDEX.md:145:| docs/tla/AmunConsensus.tla | Formal TLA+ model |
docs/PROTOCOL_HARDENING_ROADMAP.md:6:- [ ] Formal safety proofs for finality
docs/REPLAY_LAW.md:3:## Phase 81 — Deterministic Replay Constitution
docs/REPOSITORY_LAYOUT.md:19:| constitution/ | Formal constitutional specifications |
docs/REPOSITORY_LAYOUT.md:257:Formal constitutional artifacts.
docs/V0_3_COMPLETION.md:24:## Next Phase: v0.4 – Formal Constitutional Theory
docs/V0_3_COMPLETION.md:25:- Formal CCS model
docs/V2_027_FORMAL_CONSTITUTIONAL_STATE.md:12:- **Formal evidence transfer:** All 40 nodes converged on height=1.
docs/V2_027_FORMAL_CONSTITUTIONAL_STATE.md:15:## Constitutional Authority Levels
docs/V2_027_FORMAL_CONSTITUTIONAL_STATE.md:16:| Level | Evidence | Authority |
docs/V2_027_FORMAL_CONSTITUTIONAL_STATE.md:1:# V2-027: Formal Constitutional State Transfer
docs/V4_002_003_CCS_THEORY_NUCLEUS.md:118:- **V4-005:** Formal TLA+ specification of the CCS model.
docs/V4_002_003_CCS_THEORY_NUCLEUS.md:119:- **V4-006:** Mechanized proof of the Authority Uniqueness Theorem.
docs/V4_002_003_CCS_THEORY_NUCLEUS.md:47:## 3. Axioms of CCS
docs/V4_002_003_CCS_THEORY_NUCLEUS.md:49:### Axiom 1: Context Dominance
docs/V4_002_003_CCS_THEORY_NUCLEUS.md:57:### Axiom 2: Contextual Recoverability
docs/V4_002_003_CCS_THEORY_NUCLEUS.md:65:### Axiom 3: Authority Uniqueness
docs/V4_002_003_CCS_THEORY_NUCLEUS.md:73:### Axiom 4: Epoch Supremacy
docs/V5_CCS_COMPLETE_RESEARCH_PROGRAM.md:20:### Layer 2: Axioms (What rules govern?)
docs/V5_CCS_COMPLETE_RESEARCH_PROGRAM.md:91:1. Formal CCS specification (independent of AmunChain)
docs/V5_CCS_RESEARCH_PROGRAM.md:102:1. Formal CCS specification (independent of AmunChain)
docs/V6_001_GEOMETRY_OF_AUTHORITY.md:1:# CCS v0.6 – Geometry of Authority (Draft Axioms)
docs/V6_001_GEOMETRY_OF_AUTHORITY.md:4:**Status:** V6-001 – Foundational Axioms for Authority Ordering
docs/V6_001_GEOMETRY_OF_AUTHORITY.md:58:## 3. Axioms of Constitutional Space
docs/V6_001_GEOMETRY_OF_AUTHORITY.md:60:### Axiom 1: Partial Order
docs/V6_001_GEOMETRY_OF_AUTHORITY.md:67:### Axiom 2: Foundational Position
docs/V6_001_GEOMETRY_OF_AUTHORITY.md:72:### Axiom 3: Conservation of Reachability
docs/V6_001_GEOMETRY_OF_AUTHORITY.md:81:### Axiom 4: Monotonicity of Authority
docs/V6_002_PATHS_NOT_POINTS.md:117:Axiom 2. No separate "Acyclicity Theorem" is needed.
docs/V6_002_PATHS_NOT_POINTS.md:13:space with partial order `≤`. But the Conservation Axiom (`P₀ ≤ P`)
docs/V6_002_PATHS_NOT_POINTS.md:4:**Status:** Draft Axioms – From Position to Path
docs/V6_002_PATHS_NOT_POINTS.md:69:## 3. Axioms (Path-Based)
docs/V6_002_PATHS_NOT_POINTS.md:71:### Axiom 1: Foundational Root
docs/V6_002_PATHS_NOT_POINTS.md:77:### Axiom 2: Deterministic Paths
docs/V6_002_PATHS_NOT_POINTS.md:86:### Axiom 3: Path Conservation
docs/V6_002_PATHS_NOT_POINTS.md:92:### Axiom 4: Path Monotonicity
docs/V6_003_CANONICALIZATION.md:110:- V3-007A: Constitutional amendments create forks in `G`.
docs/V6_003_CANONICALIZATION.md:28:### Layer 1: Constitutional Possibility Graph (G)
docs/V6_003_CANONICALIZATION.md:37:### Layer 2: Constitutional Authority Chain (Λ)
docs/V6_003_CANONICALIZATION.md:4:**Status:** Final Axiom – The Core of CCS
docs/V6_003_CANONICALIZATION.md:54:- **Constitutional:** The selection follows the rules of the
docs/V6_003_CANONICALIZATION.md:63:**Constitutional Authority** is the canonical path `Λ` selected by
docs/V6_003_CANONICALIZATION.md:70:## 4. Axioms (Complete)
docs/V6_003_CANONICALIZATION.md:72:### Axiom 1: Foundational Root
docs/V6_003_CANONICALIZATION.md:75:### Axiom 2: Constitutional Determinism (Final Form)
docs/V6_003_CANONICALIZATION.md:81:### Axiom 3: Path Conservation
docs/V6_003_CANONICALIZATION.md:86:### Axiom 4: Constitutional Monotonicity
docs/V6_004_CONSTITUTIONAL_SELECTION_PRINCIPLE.md:122:1. Formalize `ConstitutionalContinuity` as a measurable quantity.
docs/V6_004_CONSTITUTIONAL_SELECTION_PRINCIPLE.md:29:Formally:
docs/V6_004_CONSTITUTIONAL_SELECTION_PRINCIPLE.md:40:## 2. The Key Theorem (Conjecture)
docs/V6_004_CONSTITUTIONAL_SELECTION_PRINCIPLE.md:42:**Constitutional Authority Theorem:**
docs/V6_004_CONSTITUTIONAL_SELECTION_PRINCIPLE.md:69:## 4. Constitutional Continuity (Formal Conjecture)
docs/V6_005_CONTINUITY_ORDERING.md:46:### Axiom C1: Foundational Minimum
docs/V6_005_CONTINUITY_ORDERING.md:51:### Axiom C2: Extension Monotonicity
docs/V6_005_CONTINUITY_ORDERING.md:58:### Axiom C3: Evidence Consistency
docs/V6_005_CONTINUITY_ORDERING.md:66:### Axiom C4: Epoch Dominance
docs/V6_005_CONTINUITY_ORDERING.md:73:### Axiom C5: Transitive Closure
docs/V6_005_CONTINUITY_ORDERING.md:79:## 3. The Key Theorem (Conjecture)
docs/V6_005_CONTINUITY_ORDERING.md:81:**Constitutional Authority Theorem (Order Version):**
docs/V6_006_CONSTITUTIONAL_CONVERGENCE.md:115:1. Formal proof that `(Paths(G), ≼)` is a join-semilattice.
docs/V6_006_CONSTITUTIONAL_CONVERGENCE.md:116:2. Formal proof that the supremum is unique and canonical.
docs/V6_006_CONSTITUTIONAL_CONVERGENCE.md:32:Formally:
docs/V6_006_CONSTITUTIONAL_CONVERGENCE.md:52:## 3. The Central Theorem (Conjecture)
docs/V6_006_CONSTITUTIONAL_CONVERGENCE.md:54:**Constitutional Convergence Theorem:**
docs/V6_007_CONSTITUTIONAL_PREFERENCE.md:30:Formally:
docs/V6_007_CONSTITUTIONAL_PREFERENCE.md:48:## 3. The Core Theorem (Conjecture)
docs/V6_007_CONSTITUTIONAL_PREFERENCE.md:50:**Constitutional Preference Theorem:**
docs/V6_007_CONSTITUTIONAL_PREFERENCE.md:65:### Axiom P1: Foundational Minimum
docs/V6_007_CONSTITUTIONAL_PREFERENCE.md:70:### Axiom P2: Epoch Priority
docs/V6_007_CONSTITUTIONAL_PREFERENCE.md:74:### Axiom P3: Evidence Priority
docs/V6_007_CONSTITUTIONAL_PREFERENCE.md:78:### Axiom P4: Lexicographic Consistency
docs/V6_008_CONSTITUTIONAL_RESOLUTION.md:63:## 3. Axioms of Resolution
docs/V6_008_CONSTITUTIONAL_RESOLUTION.md:65:### Axiom R1: Exclusivity
docs/V6_008_CONSTITUTIONAL_RESOLUTION.md:70:### Axiom R2: Stability
docs/V6_008_CONSTITUTIONAL_RESOLUTION.md:75:### Axiom R3: Completeness
docs/V6_008_CONSTITUTIONAL_RESOLUTION.md:83:## 4. The Central Theorem
docs/V6_008_CONSTITUTIONAL_RESOLUTION.md:85:**Constitutional Resolution Theorem:**
docs/V6_009_CONSTITUTIONAL_META_RESOLUTION.md:59:## 3. The Meta-Theorem
docs/V6_009_CONSTITUTIONAL_META_RESOLUTION.md:61:**Constitutional Meta-Resolution Theorem:**
docs/V6_010_CONSTITUTIONAL_CONSTRAINT_PRINCIPLE.md:38:Formally:
docs/V6_010_CONSTITUTIONAL_CONSTRAINT_PRINCIPLE.md:68:## 4. The Central Theorem
docs/V6_010_CONSTITUTIONAL_CONSTRAINT_PRINCIPLE.md:70:**Constitutional Finality Theorem:**
docs/V7_001_CONSTITUTIONAL_DERIVABILITY.md:100:`{ C' : P₀ ⊢_C C' }` whose existence is guaranteed by Axiom D5.
docs/V7_001_CONSTITUTIONAL_DERIVABILITY.md:104:derivability closure. Axiom D5 guarantees this exists.
docs/V7_001_CONSTITUTIONAL_DERIVABILITY.md:142:## 6. The Final Theorem
docs/V7_001_CONSTITUTIONAL_DERIVABILITY.md:144:**Constitutional Completeness Theorem:**
docs/V7_001_CONSTITUTIONAL_DERIVABILITY.md:50:## 2. Axioms of Constitutional Derivability
docs/V7_001_CONSTITUTIONAL_DERIVABILITY.md:52:### Axiom D1: Reflexivity
docs/V7_001_CONSTITUTIONAL_DERIVABILITY.md:57:### Axiom D2: Transitivity
docs/V7_001_CONSTITUTIONAL_DERIVABILITY.md:62:### Axiom D3: Foundational Origin
docs/V7_001_CONSTITUTIONAL_DERIVABILITY.md:68:### Axiom D4: Evidence-Backed
docs/V7_001_CONSTITUTIONAL_DERIVABILITY.md:74:### Axiom D5: Deterministic Closure
docs/V7_001_CONSTITUTIONAL_DERIVABILITY.md:88:Every legitimate context `C` satisfies `P₀ ⊢_C C`. This is Axiom D3.
docs/V7_002_DERIVABILITY_GEOMETRY.md:15:Axiom D5 (Deterministic Closure) stated that every context has a unique
docs/V7_002_DERIVABILITY_GEOMETRY.md:34:`∃! P₀` with in-degree 0. This is Axiom D3.
docs/V7_002_DERIVABILITY_GEOMETRY.md:55:## 2. The Central Theorem
docs/V7_002_DERIVABILITY_GEOMETRY.md:57:**Derivability Geometry Theorem:**
docs/V7_003_CONSTITUTIONAL_REDUCTION.md:35:## 2. Axioms of Constitutional Reduction
docs/V7_003_CONSTITUTIONAL_REDUCTION.md:37:### Axiom R1: Foundational Root
docs/V7_003_CONSTITUTIONAL_REDUCTION.md:43:### Axiom R2: Deterministic Reduction
docs/V7_003_CONSTITUTIONAL_REDUCTION.md:50:### Axiom R3: Unique Normal Form (Church-Rosser for CCS)
docs/V7_003_CONSTITUTIONAL_REDUCTION.md:56:### Axiom R4: Reachability
docs/V7_004_CONSTITUTIONAL_CONFLUENCE.md:15:relation of CCS. But Axiom R2 assumed that each context has exactly
docs/V7_004_CONSTITUTIONAL_CONFLUENCE.md:27:## 1. Axioms of Constitutional Confluence
docs/V7_004_CONSTITUTIONAL_CONFLUENCE.md:29:### Axiom CF1: Foundational Root
docs/V7_004_CONSTITUTIONAL_CONFLUENCE.md:34:### Axiom CF2: Reachability
docs/V7_004_CONSTITUTIONAL_CONFLUENCE.md:39:### Axiom CF3: Local Branching (Replaces R2)
docs/V7_004_CONSTITUTIONAL_CONFLUENCE.md:46:### Axiom CF4: Constitutional Confluence
docs/V7_004_CONSTITUTIONAL_CONFLUENCE.md:53:### Axiom CF5: Termination (Well-Foundedness)
docs/V7_004_CONSTITUTIONAL_CONFLUENCE.md:61:## 2. The Central Theorem
docs/V7_004_CONSTITUTIONAL_CONFLUENCE.md:63:**Constitutional Church-Rosser Theorem:**
docs/V7_005_CONSTITUTIONAL_PRUNING.md:43:## 2. Axioms of Constitutional Pruning
docs/V7_005_CONSTITUTIONAL_PRUNING.md:45:### Axiom P1: Foundational Root
docs/V7_005_CONSTITUTIONAL_PRUNING.md:50:### Axiom P2: Legitimate Expansion
docs/V7_005_CONSTITUTIONAL_PRUNING.md:55:### Axiom P3: Irreversible Pruning
docs/V7_005_CONSTITUTIONAL_PRUNING.md:60:### Axiom P4: Eventual Convergence
docs/V7_005_CONSTITUTIONAL_PRUNING.md:66:### Axiom P5: Pruning by Evidence
docs/V7_005_CONSTITUTIONAL_PRUNING.md:77:## 3. The Central Theorem
docs/V7_005_CONSTITUTIONAL_PRUNING.md:79:**Constitutional Pruning Theorem:**
docs/V7_006_CONSTITUTIONAL_EXCLUSION.md:39:## 2. Axioms of Constitutional Exclusion
docs/V7_006_CONSTITUTIONAL_EXCLUSION.md:41:### Axiom E1: Foundational Legitimacy
docs/V7_006_CONSTITUTIONAL_EXCLUSION.md:46:### Axiom E2: Exclusion is Permanent
docs/V7_006_CONSTITUTIONAL_EXCLUSION.md:52:### Axiom E3: Exclusion Propagates Forward
docs/V7_006_CONSTITUTIONAL_EXCLUSION.md:58:### Axiom E4: Legitimacy by Non-Exclusion
docs/V7_006_CONSTITUTIONAL_EXCLUSION.md:63:### Axiom E5: Eventual Convergence
docs/V7_006_CONSTITUTIONAL_EXCLUSION.md:71:## 3. The Central Theorem
docs/V7_006_CONSTITUTIONAL_EXCLUSION.md:73:**Constitutional Exclusion Theorem:**
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:123:## 4. The Central Theorem
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:125:**Constitutional Completeness Theorem:**
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:51:## 2. Axioms
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:53:### Axiom D1: Foundational Root
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:58:### Axiom D2: Derivability is Transitive
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:63:### Axiom E1: Exclusion is Irreversible
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:68:### Axiom E2: Exclusion Closes Subtrees
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:73:### Axiom E3: Self-Exclusion is Impossible
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:78:### Axiom E4: Derivability and Exclusion are Disjoint
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:32:Axioms:
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:40:Axioms:
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:50:**Axiom C1: Constitutional Decidability**
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:56:**Axiom C2: Canonical Completeness**
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:61:**Axiom C3: Canonical Uniqueness**
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:68:## 2. The Complete Set of Axioms
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:72:| Layer | Relation | Axioms |
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:80:## 3. The Central Theorem
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:82:**Constitutional Closure Theorem:**
docs/V7_009_WHY_CLOSURE.md:120:1. Formalize S1-S4 in TLA+ or Coq.
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:112:| Primitive | Derivability `⊢_C` | Axioms D1-D2 |
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:113:| Primitive | Exclusion `⇍_C` | Axioms E1-E4 |
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:114:| Primitive | Single History `SH1` | Axiom SH1 |
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:117:| Derived | Closure | Theorem |
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:39:## 2. The Principle as a Formal Axiom
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:41:### Axiom SH1: Single History
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:53:## 3. The Central Theorem
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:55:**Constitutional Closure Theorem (Final Version):**
docs/V8_001_INDEPENDENCE_OF_SH1.md:77:## 3. Verification of Axioms
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:124:| Derivability (⊢_C) | Axiom |
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:125:| Exclusion (⇍_C) | Axiom |
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:126:| Constitutional Comparability (CC) | Axiom |
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:127:| Single History (SH1) | Theorem |
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:128:| Closure (C1-C3) | Theorem |
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:46:## 2. The Comparability Axiom (CC)
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:48:**Axiom CC: Constitutional Comparability**
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:92:## 4. The Central Theorem
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:94:**Comparability Theorem:**
docs/V8_RESEARCH_PROPOSAL.md:36:   - Formalize (⊢_C, ⇍_C, SH1) in TLA+
docs/V8_RESEARCH_PROPOSAL.md:37:   - Model-check the Closure Theorem
docs/V8_RESEARCH_PROPOSAL.md:40:2. **V8-002A: Coq/Lean Formalization**
docs/architecture/CRATE_CLASSIFICATION.md:16:- amun-constitution-core - Constitutional axioms
docs/architecture/PHASE_49_COMPLETE.md:108:3. **Formal State Transition Semantics** — (State, Event) → State'
docs/architecture/PHASE_49_COMPLETE.md:110:5. **Byzantine Evidence System** — canonical proof generation
docs/architecture/PHASE_49_COMPLETE.md:128:The foundation is ready for Phase 50: Formal Replay Infrastructure.
docs/architecture/PHASE_49_COMPLETE.md:21:| 0.5 | Constitutional Semantics | amun-constitution, amun-invariants, interfaces | ✅ Pure |
docs/architecture/PHASE_49_COMPLETE.md:43:5. **Formal Ordering**: Round monotonicity + sender ordering guarantees
docs/audit/AUDIT_EVIDENCE_BUNDLE.md:32:| Forged TransitionProof | byz_001 | ✅ Rejected |
docs/audit/AUDIT_EVIDENCE_BUNDLE.md:3:## 1. Invariant-to-Test Traceability
docs/audit/AUDIT_EVIDENCE_BUNDLE.md:5:| Invariant | Description | Test ID | Crate | Status |
docs/audit/AUDIT_EVIDENCE_BUNDLE.md:80:| Invariant Engine | amun-invariant-engine | 5 | ✅ |
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
docs/consensus/I1_translation.md:12:## Semantic Invariants (from CSA-6)
docs/consensus/I2_extension.md:12:## Semantic Invariants (from CSA-6)
docs/consensus/I3_reframing.md:22:## Semantic Invariants (from CSA-6)
docs/consensus/I4_drift.md:21:## Semantic Invariants (from CSA-6)
docs/consensus/origin.md:3:## Semantic Invariants (from CSA-6)
docs/constitutional-mathematics/PHASE_111_FOUNDATIONS_V1.md:108:- Formal Semantics
docs/constitutional-mathematics/PHASE_111_FOUNDATIONS_V1.md:109:- Axioms
docs/constitutional-mathematics/PHASE_111_FOUNDATIONS_V1.md:11:Transition Constitutional Mathematics from conceptual framework to formal research framework.
docs/constitutional-mathematics/PHASE_111_FOUNDATIONS_V1.md:61:### 4. Axiom Set v1
docs/constitutional-mathematics/PHASE_111_FOUNDATIONS_V1.md:91:### 6. First Formal Result
docs/constitutional-mathematics/PHASE_111_FOUNDATIONS_V1.md:93:Recognition Existence Lemma
docs/constitutional-mathematics/PHASE_112_SIMULATION_SPECIFICATION.md:111:Recognition Stability Theorem
docs/constitutional-mathematics/PHASE_112_SIMULATION_SPECIFICATION.md:141:Formal Research Framework
docs/constitutional-mathematics/README.md:28:Formal Semantics       ✅
docs/constitutional-mathematics/README.md:30:Axioms                 ✅
docs/constitutional-mathematics/README.md:34:First Lemma            ✅
docs/constitutional-mathematics/README.md:3:Formal Science of Computable Legitimacy Dynamics Under Scoped Sovereignty Constraints
docs/constitutional/phase84_freeze.md:2:## Frozen Invariants for Sovereign State Replication
docs/constitutional/phase85_seal.md:30:5. Formal ConstitutionalStateMachine (allowed/forbidden transitions)
docs/federation/FEDERATION_ARCHITECTURE.md:53:## Invariant
docs/federation/TRUST_GRAPH.md:51:## Invariant
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:147:    serialized_bytes = CanonicalSerialization(ConstitutionalCommitment)
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:254:## 12. Consensus Invariants
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:258:**Invariant 1:** `constitutional_root == BLAKE3("AMUN_CONSTITUTIONAL_ROOT_V1" || identity_root || evidence_root || governance_root || economic_root)`
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:260:**Invariant 2:** `constitutional_commitment_root == BLAKE3("AMUN_CONSTITUTIONAL_COMMITMENT_V1" || CanonicalSerialization(ConstitutionalCommitment))`
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:262:**Invariant 3:** `state_root` must include `constitutional_commitment_root` as a leaf in its Merkle tree computation.
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:264:**Invariant 4:** The `AppHash` MUST commit to `state_root`. For CCA v1.0, this is defined as `AppHash = state_root`. Future versions may introduce additional roots (such as `execution_root` or `receipt_root`) into the `AppHash` computation, in which case the invariant will be updated to reflect the new hash composition while preserving the requirement that `state_root` remains a committed input.
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:266:**Invariant 5:** Given identical state transition inputs (same genesis, same transactions up to height N), all validators must compute identical values for all roots defined in this specification.
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:33:12. Consensus Invariants
docs/protocol/FREEZE_CERTIFICATE_v1.md:29:### Constitutional Invariants (FROZEN)
docs/protocol/replay_physics_v1.md:13:### 2.2 Ordering Invariant
docs/protocol/replay_physics_v1.md:24:execute_and_self_verify() SHALL: 1. Execute trace via DeterministicExecutor, 2. Apply same entries via ReplayState, 3. Compare final roots, 4. Return EquivalenceProof.
docs/protocol/replay_physics_v1.md:6:All ConstitutionalHash values are raw 32-byte blobs. No length prefix. No type tag. Fixed-width canonical encoding.
docs/reports/CCA_IMPL_5B_FINAL_REPORT.md:150:2. Deterministic Constitutional Commitments
