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
1092:crates/amun-constitutional-proof/src/report_generator.rs:3:use crate::{ConstitutionalVerdict, EvidenceArchive, ObligationRegistry, VerdictResult};
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
1309:crates/amun-constitutional/src/constitutional_witness.rs:1://! ConstitutionalWitness — a formally sufficient constitutional proof surface.
1596:crates/amun-evidence-engine/src/evidence_engine.rs:153:        archive.insert(ConstitutionalEvidence::InvariantViolation {
1603:crates/amun-evidence-engine/src/evidence_engine.rs:34:                ConstitutionalEvidence::InvariantViolation {
1700:crates/amun-execution-receipt/src/lib.rs:6://! ## Constitutional Invariants
1730:crates/amun-failure/src/tests.rs:32:    assert!(!ConstitutionalFault::InvalidStateTransition.should_halt());
1797:crates/amun-invariant-engine/src/invariant_engine.rs:135:            ConstitutionalEvidence::InvariantViolation { obligation_id, .. } => {
1807:crates/amun-invariant-engine/src/invariant_engine.rs:194:                ConstitutionalEvidence::InvariantViolation { state_root: sr, .. } => {
1809:crates/amun-invariant-engine/src/invariant_engine.rs:19:    ) -> (Vec<InvariantResult>, Vec<ConstitutionalEvidence>)
1810:crates/amun-invariant-engine/src/invariant_engine.rs:1:use amun_evidence_engine::evidence_types::ConstitutionalEvidence;
1813:crates/amun-invariant-engine/src/invariant_engine.rs:35:                evidence.push(ConstitutionalEvidence::InvariantViolation {
187:crates/amun-constitutional-proof/src/certification.rs:1:use crate::{ConstitutionalVerdict, EvidenceArchive, ObligationRegistry, VerdictResult};
1966:crates/amun-networking/tests/n18_node_rejoin.rs:73:// N18.5 — Constitutional Invariant REJOIN-001
25:crates/amun-consensus-types/src/errors.rs:19:            ConstitutionalFault::InvalidStateTransition,
34:crates/amun-constitutional-enforcement/src/lib.rs:112:                ConstitutionalLaw::StateTransitionValidity,
35:crates/amun-constitutional-enforcement/src/lib.rs:179:                &ConstitutionalLaw::StateTransitionValidity,
37:crates/amun-constitutional-enforcement/src/proof_engine.rs:194:                law: ConstitutionalLaw::StateTransitionValidity,
40:crates/amun-constitutional-enforcement/src/state_transition.rs:178:                    .any(|v| v.law == ConstitutionalLaw::StateTransitionValidity));
42:crates/amun-constitutional-enforcement/src/state_transition.rs:70:                law: ConstitutionalLaw::StateTransitionValidity,
46:crates/amun-constitutional-geometry/src/flow_dynamics.rs:61:                ConstitutionalForce::InvariantForce { strength, .. } => {
554:crates/amun-constitutional-proof/src/report_generator.rs:3:use crate::{ConstitutionalVerdict, EvidenceArchive, ObligationRegistry, VerdictResult};
560:crates/amun-constitutional-proof/src/verdict_evaluator.rs:3:    ConstitutionalVerdict, ObligationResult, ObligationResultStatus, ObligationSeverity,
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
882:crates/amun-constitutional-proof/src/certification.rs:1:use crate::{ConstitutionalVerdict, EvidenceArchive, ObligationRegistry, VerdictResult};
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
980:docs/V6_009_CONSTITUTIONAL_META_RESOLUTION.md:61:**Constitutional Meta-Resolution Theorem:**
982:docs/V6_010_CONSTITUTIONAL_CONSTRAINT_PRINCIPLE.md:70:**Constitutional Finality Theorem:**
986:docs/V7_001_CONSTITUTIONAL_DERIVABILITY.md:144:**Constitutional Completeness Theorem:**
987:docs/V7_001_CONSTITUTIONAL_DERIVABILITY.md:50:## 2. Axioms of Constitutional Derivability
998:docs/V7_003_CONSTITUTIONAL_REDUCTION.md:35:## 2. Axioms of Constitutional Reduction
9:crates/amun-codec/src/writer.rs:115:                ConstitutionalFault::InvalidStateTransition,
crates/amun-audit/src/lib.rs:2:// Invariant ownership map:
crates/amun-audit/tests/audit_layer02_geometry.rs:7:    fn geo001_proof_depth_invariant() {
crates/amun-audit/tests/audit_layer16_mutation.rs:40:    fn mut004_endian_invariant() {
crates/amun-bls/src/tests.rs:18:        assert!(verify(msg, &sig, &kp.public).expect("test invariant"));
crates/amun-bls/src/tests.rs:25:        assert!(!verify(msg, &zero_sig, &kp.public).expect("test invariant"));
crates/amun-bytecode/src/interpreter.rs:218:                OpCode::CheckInvariant { .. } => {
crates/amun-bytecode/src/opcodes.rs:14:    CheckInvariant { invariant_idx: u32 },
crates/amun-bytecode/src/opcodes.rs:40:            OpCode::CheckInvariant { .. } => "OP_CHECK_INVARIANT",
crates/amun-bytecode/src/opcodes.rs:63:            OpCode::CheckInvariant { .. } => 50,
crates/amun-bytecode/src/opcodes.rs:94:            OpCode::CheckInvariant { .. } | OpCode::EmitClaim { .. }
crates/amun-bytecode/src/program.rs:21:    pub fn new(level: u8, invariant_count: u32, entry_point: u32, code: Vec<OpCode>) -> Self {
crates/amun-codec/src/writer.rs:115:                ConstitutionalFault::InvalidStateTransition,
crates/amun-codec/src/writer.rs:128:                ConstitutionalFault::InvalidStateTransition,
crates/amun-consensus-execution/src/block_dag.rs:215:        // Invariant assertions
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
crates/amun-consensus-law/src/lib.rs:11:pub use validator::ValidatorObligations;
crates/amun-consensus-law/src/lib.rs:8:pub use invariants::ConsensusInvariants;
crates/amun-consensus-law/src/safety.rs:1:pub struct SafetyAxioms;
crates/amun-consensus-law/src/validator.rs:11:impl ValidatorObligations {
crates/amun-consensus-law/src/validator.rs:9:pub struct ValidatorObligations;
crates/amun-consensus-types/src/errors.rs:19:            ConstitutionalFault::InvalidStateTransition,
crates/amun-consensus/src/round_state_machine.rs:133:        //   - StateTransitionValidity (pre_state != post_state && valid)
crates/amun-constitution/src/fork_choice.rs:1:// Formal fork-choice function.
crates/amun-constitution/src/quorum_transition.rs:1:// Quorum transition safety theorem parameters.
crates/amun-constitution/src/refinement_chain.rs:10:    RefinementObligation {
crates/amun-constitution/src/refinement_chain.rs:15:    RefinementObligation {
crates/amun-constitution/src/refinement_chain.rs:20:    RefinementObligation {
crates/amun-constitution/src/refinement_chain.rs:3:pub struct RefinementObligation {
crates/amun-constitution/src/refinement_chain.rs:9:pub const REFINEMENT_OBLIGATIONS: &[RefinementObligation] = &[
crates/amun-constitutional-block/src/lib.rs:75:use amun_constitutional_state::StateTransitionRecord;
crates/amun-constitutional-block/src/lib.rs:89:    records: &[StateTransitionRecord],
crates/amun-constitutional-enforcement/src/lib.rs:112:                ConstitutionalLaw::StateTransitionValidity,
crates/amun-constitutional-enforcement/src/lib.rs:179:                &ConstitutionalLaw::StateTransitionValidity,
crates/amun-constitutional-enforcement/src/lib.rs:65:    StateTransitionValidity,
crates/amun-constitutional-enforcement/src/proof_engine.rs:194:                law: ConstitutionalLaw::StateTransitionValidity,
crates/amun-constitutional-enforcement/src/state_transition.rs:111:    let result = StateTransitionResult::new(
crates/amun-constitutional-enforcement/src/state_transition.rs:11:pub struct StateTransitionResult {
crates/amun-constitutional-enforcement/src/state_transition.rs:178:                    .any(|v| v.law == ConstitutionalLaw::StateTransitionValidity));
crates/amun-constitutional-enforcement/src/state_transition.rs:26:impl StateTransitionResult {
crates/amun-constitutional-enforcement/src/state_transition.rs:70:                law: ConstitutionalLaw::StateTransitionValidity,
crates/amun-constitutional-geometry/src/emergent_horizons.rs:25:    /// Invariant mass creates an inescapable region
crates/amun-constitutional-geometry/src/emergent_horizons.rs:26:    InvariantSingularity,
crates/amun-constitutional-geometry/src/flow_dynamics.rs:24:    InvariantForce {
crates/amun-constitutional-geometry/src/flow_dynamics.rs:61:                ConstitutionalForce::InvariantForce { strength, .. } => {
crates/amun-constitutional-geometry/src/metric_tensor.rs:49:    pub fn apply_gravitational_mass(&mut self, invariant_mass: f64, at_dimension: usize) {
crates/amun-constitutional-geometry/src/stability.rs:48:    InvariantAttractor,
crates/amun-constitutional-governance/src/lib.rs:15://! model formal and replay-verifiable.
crates/amun-constitutional-integration/src/lib.rs:102:            ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:107:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:108:            ObligationId::new(ObligationNamespace::Fault, 1),
crates/amun-constitutional-integration/src/lib.rs:109:            ObligationKind::Primary,
crates/amun-constitutional-integration/src/lib.rs:112:            ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:117:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:118:            ObligationId::new(ObligationNamespace::Recovery, 1),
crates/amun-constitutional-integration/src/lib.rs:119:            ObligationKind::Primary,
crates/amun-constitutional-integration/src/lib.rs:122:            ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:127:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:128:            ObligationId::new(ObligationNamespace::Performance, 1),
crates/amun-constitutional-integration/src/lib.rs:129:            ObligationKind::Primary,
crates/amun-constitutional-integration/src/lib.rs:132:            ObligationSeverity::Minor,
crates/amun-constitutional-integration/src/lib.rs:137:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:138:            ObligationId::new(ObligationNamespace::Performance, 2),
crates/amun-constitutional-integration/src/lib.rs:139:            ObligationKind::Primary,
crates/amun-constitutional-integration/src/lib.rs:13:    pub fn build_obligation_registry() -> ObligationRegistry {
crates/amun-constitutional-integration/src/lib.rs:142:            ObligationSeverity::Minor,
crates/amun-constitutional-integration/src/lib.rs:147:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:148:            ObligationId::new(ObligationNamespace::Performance, 3),
crates/amun-constitutional-integration/src/lib.rs:149:            ObligationKind::Primary,
crates/amun-constitutional-integration/src/lib.rs:14:        let mut reg = ObligationRegistry::new();
crates/amun-constitutional-integration/src/lib.rs:152:            ObligationSeverity::Minor,
crates/amun-constitutional-integration/src/lib.rs:159:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:160:                ObligationId::new(ObligationNamespace::Safety, 4),
crates/amun-constitutional-integration/src/lib.rs:161:                ObligationKind::Derived,
crates/amun-constitutional-integration/src/lib.rs:164:                ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:167:            .with_dependency(ObligationId::new(ObligationNamespace::Replay, 2))
crates/amun-constitutional-integration/src/lib.rs:168:            .with_dependency(ObligationId::new(ObligationNamespace::Evidence, 1)),
crates/amun-constitutional-integration/src/lib.rs:173:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:174:                ObligationId::new(ObligationNamespace::Replay, 3),
crates/amun-constitutional-integration/src/lib.rs:175:                ObligationKind::Derived,
crates/amun-constitutional-integration/src/lib.rs:178:                ObligationSeverity::Major,
crates/amun-constitutional-integration/src/lib.rs:17:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:181:            .with_dependency(ObligationId::new(ObligationNamespace::Replay, 2)),
crates/amun-constitutional-integration/src/lib.rs:186:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:187:                ObligationId::new(ObligationNamespace::Replay, 4),
crates/amun-constitutional-integration/src/lib.rs:188:                ObligationKind::Derived,
crates/amun-constitutional-integration/src/lib.rs:18:            ObligationId::new(ObligationNamespace::Safety, 1),
crates/amun-constitutional-integration/src/lib.rs:191:                ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:194:            .with_dependency(ObligationId::new(ObligationNamespace::Replay, 1)),
crates/amun-constitutional-integration/src/lib.rs:199:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:19:            ObligationKind::Primary,
crates/amun-constitutional-integration/src/lib.rs:200:                ObligationId::new(ObligationNamespace::Evidence, 3),
crates/amun-constitutional-integration/src/lib.rs:201:                ObligationKind::Derived,
crates/amun-constitutional-integration/src/lib.rs:204:                ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:207:            .with_dependency(ObligationId::new(ObligationNamespace::Evidence, 2)),
crates/amun-constitutional-integration/src/lib.rs:212:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:213:                ObligationId::new(ObligationNamespace::Evidence, 4),
crates/amun-constitutional-integration/src/lib.rs:214:                ObligationKind::Derived,
crates/amun-constitutional-integration/src/lib.rs:217:                ObligationSeverity::Major,
crates/amun-constitutional-integration/src/lib.rs:220:            .with_dependency(ObligationId::new(ObligationNamespace::Evidence, 3)),
crates/amun-constitutional-integration/src/lib.rs:225:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:226:                ObligationId::new(ObligationNamespace::Finality, 3),
crates/amun-constitutional-integration/src/lib.rs:227:                ObligationKind::Derived,
crates/amun-constitutional-integration/src/lib.rs:22:            ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:230:                ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:233:            .with_dependency(ObligationId::new(ObligationNamespace::Finality, 1))
crates/amun-constitutional-integration/src/lib.rs:234:            .with_dependency(ObligationId::new(ObligationNamespace::Safety, 1)),
crates/amun-constitutional-integration/src/lib.rs:239:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:240:                ObligationId::new(ObligationNamespace::Cluster, 1),
crates/amun-constitutional-integration/src/lib.rs:241:                ObligationKind::Derived,
crates/amun-constitutional-integration/src/lib.rs:244:                ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:247:            .with_dependency(ObligationId::new(ObligationNamespace::Finality, 1))
crates/amun-constitutional-integration/src/lib.rs:248:            .with_dependency(ObligationId::new(ObligationNamespace::Replay, 1))
crates/amun-constitutional-integration/src/lib.rs:249:            .with_dependency(ObligationId::new(ObligationNamespace::Evidence, 1)),
crates/amun-constitutional-integration/src/lib.rs:254:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:255:                ObligationId::new(ObligationNamespace::Cluster, 2),
crates/amun-constitutional-integration/src/lib.rs:256:                ObligationKind::Derived,
crates/amun-constitutional-integration/src/lib.rs:259:                ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:262:            .with_dependency(ObligationId::new(ObligationNamespace::Cluster, 1)),
crates/amun-constitutional-integration/src/lib.rs:267:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:268:                ObligationId::new(ObligationNamespace::Cluster, 3),
crates/amun-constitutional-integration/src/lib.rs:269:                ObligationKind::Derived,
crates/amun-constitutional-integration/src/lib.rs:272:                ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:275:            .with_dependency(ObligationId::new(ObligationNamespace::Cluster, 1))
crates/amun-constitutional-integration/src/lib.rs:276:            .with_dependency(ObligationId::new(ObligationNamespace::Cluster, 2)),
crates/amun-constitutional-integration/src/lib.rs:27:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:281:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:282:                ObligationId::new(ObligationNamespace::Fault, 2),
crates/amun-constitutional-integration/src/lib.rs:283:                ObligationKind::Derived,
crates/amun-constitutional-integration/src/lib.rs:286:                ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:289:            .with_dependency(ObligationId::new(ObligationNamespace::Finality, 3)),
crates/amun-constitutional-integration/src/lib.rs:28:            ObligationId::new(ObligationNamespace::Safety, 2),
crates/amun-constitutional-integration/src/lib.rs:294:            ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:295:                ObligationId::new(ObligationNamespace::Recovery, 2),
crates/amun-constitutional-integration/src/lib.rs:296:                ObligationKind::Derived,
crates/amun-constitutional-integration/src/lib.rs:299:                ObligationSeverity::Major,
crates/amun-constitutional-integration/src/lib.rs:29:            ObligationKind::Primary,
crates/amun-constitutional-integration/src/lib.rs:302:            .with_dependency(ObligationId::new(ObligationNamespace::Recovery, 1)),
crates/amun-constitutional-integration/src/lib.rs:313:        obligation_id: &ObligationId,
crates/amun-constitutional-integration/src/lib.rs:32:            ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:338:            ObligationRegistry,
crates/amun-constitutional-integration/src/lib.rs:346:        let registry = Self::build_obligation_registry();
crates/amun-constitutional-integration/src/lib.rs:355:            let obligations: Vec<ProofObligation> = registry
crates/amun-constitutional-integration/src/lib.rs:370:            let obligations: Vec<ProofObligation> = registry
crates/amun-constitutional-integration/src/lib.rs:379:                    ObligationResultStatus::Inconclusive
crates/amun-constitutional-integration/src/lib.rs:37:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:381:                    ObligationResultStatus::Satisfied
crates/amun-constitutional-integration/src/lib.rs:384:                results.push(if status == ObligationResultStatus::Satisfied {
crates/amun-constitutional-integration/src/lib.rs:385:                    ObligationResult::satisfied(obl.id.clone(), refs)
crates/amun-constitutional-integration/src/lib.rs:387:                    ObligationResult::inconclusive(obl.id.clone(), refs)
crates/amun-constitutional-integration/src/lib.rs:38:            ObligationId::new(ObligationNamespace::Safety, 3),
crates/amun-constitutional-integration/src/lib.rs:39:            ObligationKind::Primary,
crates/amun-constitutional-integration/src/lib.rs:3:    EvidenceType, ObligationId, ObligationKind, ObligationNamespace, ObligationRegistry,
crates/amun-constitutional-integration/src/lib.rs:42:            ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:449:    fn n47_7_build_obligation_registry() {
crates/amun-constitutional-integration/src/lib.rs:450:        let reg = ConstitutionalBridge::build_obligation_registry();
crates/amun-constitutional-integration/src/lib.rs:47:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:48:            ObligationId::new(ObligationNamespace::Replay, 1),
crates/amun-constitutional-integration/src/lib.rs:49:            ObligationKind::Primary,
crates/amun-constitutional-integration/src/lib.rs:4:    ObligationResult, ObligationResultStatus, ObligationSeverity, ProofObligation,
crates/amun-constitutional-integration/src/lib.rs:52:            ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:57:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:58:            ObligationId::new(ObligationNamespace::Replay, 2),
crates/amun-constitutional-integration/src/lib.rs:59:            ObligationKind::Primary,
crates/amun-constitutional-integration/src/lib.rs:62:            ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:67:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:68:            ObligationId::new(ObligationNamespace::Evidence, 1),
crates/amun-constitutional-integration/src/lib.rs:69:            ObligationKind::Primary,
crates/amun-constitutional-integration/src/lib.rs:72:            ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:77:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:78:            ObligationId::new(ObligationNamespace::Evidence, 2),
crates/amun-constitutional-integration/src/lib.rs:79:            ObligationKind::Primary,
crates/amun-constitutional-integration/src/lib.rs:82:            ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:87:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:88:            ObligationId::new(ObligationNamespace::Finality, 1),
crates/amun-constitutional-integration/src/lib.rs:89:            ObligationKind::Primary,
crates/amun-constitutional-integration/src/lib.rs:92:            ObligationSeverity::Critical,
crates/amun-constitutional-integration/src/lib.rs:97:        reg.register(ProofObligation::new(
crates/amun-constitutional-integration/src/lib.rs:98:            ObligationId::new(ObligationNamespace::Finality, 2),
crates/amun-constitutional-integration/src/lib.rs:99:            ObligationKind::Primary,
crates/amun-constitutional-proof/src/article_i_certificate.rs:1:use crate::{ObligationKind, ObligationRegistry, ObligationSeverity};
crates/amun-constitutional-proof/src/article_i_certificate.rs:32:    pub fn issue(registry: &ObligationRegistry, issued_at: u64) -> Option<Self> {
crates/amun-constitutional-proof/src/article_i_certificate.rs:53:                ObligationKind::Primary => primaries += 1,
crates/amun-constitutional-proof/src/article_i_certificate.rs:54:                ObligationKind::Derived => derived += 1,
crates/amun-constitutional-proof/src/article_i_certificate.rs:57:                ObligationSeverity::Critical => critical += 1,
crates/amun-constitutional-proof/src/article_i_certificate.rs:58:                ObligationSeverity::Major => major += 1,
crates/amun-constitutional-proof/src/article_i_certificate.rs:59:                ObligationSeverity::Minor => minor += 1,
crates/amun-constitutional-proof/src/article_i_certificate.rs:60:                ObligationSeverity::Advisory => advisory += 1,
crates/amun-constitutional-proof/src/certification.rs:100:                r.status == crate::ObligationResultStatus::Failed && r.failure_reason.is_some()
crates/amun-constitutional-proof/src/certification.rs:116:            .filter(|r| r.status == crate::ObligationResultStatus::Failed)
crates/amun-constitutional-proof/src/certification.rs:1:use crate::{ConstitutionalVerdict, EvidenceArchive, ObligationRegistry, VerdictResult};
crates/amun-constitutional-proof/src/certification.rs:36:        registry: &ObligationRegistry,
crates/amun-constitutional-proof/src/certification.rs:84:    fn gate_c1_all_obligations_registered(registry: &ObligationRegistry) -> GateResult {
crates/amun-constitutional-proof/src/constitutional_verdict.rs:111:                ObligationResultStatus::Satisfied => "SATISFIED",
crates/amun-constitutional-proof/src/constitutional_verdict.rs:112:                ObligationResultStatus::Failed => "FAILED",
crates/amun-constitutional-proof/src/constitutional_verdict.rs:113:                ObligationResultStatus::Inconclusive => "INCONCLUSIVE",
crates/amun-constitutional-proof/src/constitutional_verdict.rs:114:                ObligationResultStatus::Waived => "WAIVED",
crates/amun-constitutional-proof/src/constitutional_verdict.rs:115:                ObligationResultStatus::NotApplicable => "NOT_APPLICABLE",
crates/amun-constitutional-proof/src/constitutional_verdict.rs:127:    fn collect_evidence_refs(results: &[ObligationResult]) -> Vec<String> {
crates/amun-constitutional-proof/src/constitutional_verdict.rs:23:    pub results: Vec<ObligationResult>,
crates/amun-constitutional-proof/src/constitutional_verdict.rs:44:        results: Vec<ObligationResult>,
crates/amun-constitutional-proof/src/constitutional_verdict.rs:52:            .filter(|r| r.status == ObligationResultStatus::Satisfied)
crates/amun-constitutional-proof/src/constitutional_verdict.rs:5:use crate::{ObligationResult, ObligationResultStatus, VerdictResult};
crates/amun-constitutional-proof/src/constitutional_verdict.rs:7:/// A constitutional verdict issued after evaluating a set of proof obligations.
crates/amun-constitutional-proof/src/dependency_graph.rs:106:    fn find_cycle(&self) -> Option<ObligationId> {
crates/amun-constitutional-proof/src/dependency_graph.rs:121:        node: &ObligationId,
crates/amun-constitutional-proof/src/dependency_graph.rs:122:        white: &mut HashSet<ObligationId>,
crates/amun-constitutional-proof/src/dependency_graph.rs:123:        gray: &mut HashSet<ObligationId>,
crates/amun-constitutional-proof/src/dependency_graph.rs:124:        black: &mut HashSet<ObligationId>,
crates/amun-constitutional-proof/src/dependency_graph.rs:150:        node: &ObligationId,
crates/amun-constitutional-proof/src/dependency_graph.rs:151:        kinds: &HashMap<ObligationId, crate::ObligationKind>,
crates/amun-constitutional-proof/src/dependency_graph.rs:163:                .unwrap_or(crate::ObligationKind::Primary);
crates/amun-constitutional-proof/src/dependency_graph.rs:164:            if kind == crate::ObligationKind::Primary {
crates/amun-constitutional-proof/src/dependency_graph.rs:17:    pub fn add_node(&mut self, id: ObligationId) {
crates/amun-constitutional-proof/src/dependency_graph.rs:23:    pub fn add_edge(&mut self, from: ObligationId, to: ObligationId) {
crates/amun-constitutional-proof/src/dependency_graph.rs:34:    pub fn topological_sort(&self) -> Result<Vec<ObligationId>, RegistryError> {
crates/amun-constitutional-proof/src/dependency_graph.rs:35:        let mut in_degree: HashMap<ObligationId, usize> = HashMap::new();
crates/amun-constitutional-proof/src/dependency_graph.rs:3:use crate::{ObligationId, RegistryError};
crates/amun-constitutional-proof/src/dependency_graph.rs:45:        let mut queue: VecDeque<ObligationId> = in_degree
crates/amun-constitutional-proof/src/dependency_graph.rs:70:            return Err(RegistryError::CircularDependency(ObligationId::new(
crates/amun-constitutional-proof/src/dependency_graph.rs:71:                crate::ObligationNamespace::Safety,
crates/amun-constitutional-proof/src/dependency_graph.rs:7:    nodes: HashSet<ObligationId>,
crates/amun-constitutional-proof/src/dependency_graph.rs:81:        kinds: &HashMap<ObligationId, crate::ObligationKind>,
crates/amun-constitutional-proof/src/dependency_graph.rs:87:                .unwrap_or(crate::ObligationKind::Primary);
crates/amun-constitutional-proof/src/dependency_graph.rs:88:            if kind == crate::ObligationKind::Derived && !self.has_primary_ancestor(node, kinds) {
crates/amun-constitutional-proof/src/dependency_graph.rs:8:    edges: HashMap<ObligationId, Vec<ObligationId>>,
crates/amun-constitutional-proof/src/dependency_graph.rs:9:    reverse_edges: HashMap<ObligationId, Vec<ObligationId>>,
crates/amun-constitutional-proof/src/evidence_archive.rs:3:use crate::{EvidenceRecord, EvidenceStatus, EvidenceType, ObligationId};
crates/amun-constitutional-proof/src/evidence_archive.rs:74:    pub fn by_obligation(&self, obligation_id: &ObligationId) -> Vec<&EvidenceRecord> {
crates/amun-constitutional-proof/src/evidence_record.rs:17:    pub obligation_ids: Vec<ObligationId>,
crates/amun-constitutional-proof/src/evidence_record.rs:31:        obligation_ids: Vec<ObligationId>,
crates/amun-constitutional-proof/src/evidence_record.rs:3:use crate::{EvidenceLineage, EvidenceStatus, EvidenceType, ObligationId, Reproducibility};
crates/amun-constitutional-proof/src/evidence_type.rs:15:    FormalProofEvidence,
crates/amun-constitutional-proof/src/lib.rs:1003:        let id = ObligationId::new(ObligationNamespace::Cluster, 1);
crates/amun-constitutional-proof/src/lib.rs:101:        let id = ObligationId::new(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:1025:        let id = ObligationId::new(ObligationNamespace::Finality, 1);
crates/amun-constitutional-proof/src/lib.rs:1048:        let id = ObligationId::new(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:1080:        obl_id: ObligationId,
crates/amun-constitutional-proof/src/lib.rs:1096:        let obl = ObligationId::new(ObligationNamespace::Replay, 1);
crates/amun-constitutional-proof/src/lib.rs:109:        let id: ObligationId = serde_json::from_str(json).unwrap();
crates/amun-constitutional-proof/src/lib.rs:1107:        let obl = ObligationId::new(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:110:        assert_eq!(id.namespace(), ObligationNamespace::Safety);
crates/amun-constitutional-proof/src/lib.rs:1116:        let obl = ObligationId::new(ObligationNamespace::Finality, 1);
crates/amun-constitutional-proof/src/lib.rs:1136:        let obl = ObligationId::new(ObligationNamespace::Evidence, 1);
crates/amun-constitutional-proof/src/lib.rs:1151:        let obl = ObligationId::new(ObligationNamespace::Cluster, 1);
crates/amun-constitutional-proof/src/lib.rs:116:        let id: ObligationId = "SAFETY-001".parse().unwrap();
crates/amun-constitutional-proof/src/lib.rs:1173:        let obl = ObligationId::new(ObligationNamespace::Finality, 1);
crates/amun-constitutional-proof/src/lib.rs:117:        assert_eq!(id, ObligationId::new(ObligationNamespace::Safety, 1));
crates/amun-constitutional-proof/src/lib.rs:1207:        let obl = ObligationId::new(ObligationNamespace::Finality, 1);
crates/amun-constitutional-proof/src/lib.rs:122:        let id = ObligationId::new(ObligationNamespace::Replay, 4);
crates/amun-constitutional-proof/src/lib.rs:1239:    fn n47_3_s1_query_by_obligation() {
crates/amun-constitutional-proof/src/lib.rs:123:        assert_eq!(id.namespace(), ObligationNamespace::Replay);
crates/amun-constitutional-proof/src/lib.rs:1241:        let obl1 = ObligationId::new(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:1242:        let obl2 = ObligationId::new(ObligationNamespace::Safety, 2);
crates/amun-constitutional-proof/src/lib.rs:1276:        let obl = ObligationId::new(ObligationNamespace::Finality, 1);
crates/amun-constitutional-proof/src/lib.rs:129:        let result: Result<ObligationId, _> = "UNKNOWN-001".parse();
crates/amun-constitutional-proof/src/lib.rs:1310:        let obl = ObligationId::new(ObligationNamespace::Replay, 1);
crates/amun-constitutional-proof/src/lib.rs:1337:        let obl = ObligationId::new(ObligationNamespace::Finality, 1);
crates/amun-constitutional-proof/src/lib.rs:1364:        let mut reg = ObligationRegistry::new();
crates/amun-constitutional-proof/src/lib.rs:1365:        reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:1366:            ObligationId::new(ObligationNamespace::Safety, 1),
crates/amun-constitutional-proof/src/lib.rs:1367:            ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:1370:            ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:1377:        let obl = ObligationId::new(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:1398:            vec![ObligationResult::satisfied(
crates/amun-constitutional-proof/src/lib.rs:1399:                ObligationId::new(ObligationNamespace::Safety, 1),
crates/amun-constitutional-proof/src/lib.rs:139:        let result: Result<ObligationId, _> = "SAFETY".parse();
crates/amun-constitutional-proof/src/lib.rs:1429:        let mut reg = ObligationRegistry::new();
crates/amun-constitutional-proof/src/lib.rs:142:            Err(RegistryError::InvalidObligationIdFormat(s)) => assert_eq!(s, "SAFETY"),
crates/amun-constitutional-proof/src/lib.rs:1430:        reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:1431:            ObligationId::new(ObligationNamespace::Safety, 1),
crates/amun-constitutional-proof/src/lib.rs:1432:            ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:1435:            ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:143:            _ => panic!("expected InvalidObligationIdFormat error"),
crates/amun-constitutional-proof/src/lib.rs:1442:        let obl = ObligationId::new(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:1463:            vec![ObligationResult::satisfied(
crates/amun-constitutional-proof/src/lib.rs:1464:                ObligationId::new(ObligationNamespace::Safety, 1),
crates/amun-constitutional-proof/src/lib.rs:1494:        let mut reg = ObligationRegistry::new();
crates/amun-constitutional-proof/src/lib.rs:1495:        reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:1496:            ObligationId::new(ObligationNamespace::Replay, 1),
crates/amun-constitutional-proof/src/lib.rs:1497:            ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:1500:            ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:150:            ObligationNamespace::Safety,
crates/amun-constitutional-proof/src/lib.rs:151:            ObligationNamespace::Replay,
crates/amun-constitutional-proof/src/lib.rs:152:            ObligationNamespace::Evidence,
crates/amun-constitutional-proof/src/lib.rs:1530:        let mut reg = ObligationRegistry::new();
crates/amun-constitutional-proof/src/lib.rs:1532:            reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:1533:                ObligationId::new(ObligationNamespace::Safety, i),
crates/amun-constitutional-proof/src/lib.rs:1534:                ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:1535:                format!("Obligation {i}"),
crates/amun-constitutional-proof/src/lib.rs:1537:                ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:153:            ObligationNamespace::Finality,
crates/amun-constitutional-proof/src/lib.rs:154:            ObligationNamespace::Cluster,
crates/amun-constitutional-proof/src/lib.rs:1555:                        vec![ObligationId::new(ObligationNamespace::Safety, 1)],
crates/amun-constitutional-proof/src/lib.rs:155:            ObligationNamespace::Fault,
crates/amun-constitutional-proof/src/lib.rs:156:            ObligationNamespace::Recovery,
crates/amun-constitutional-proof/src/lib.rs:157:            ObligationNamespace::Performance,
crates/amun-constitutional-proof/src/lib.rs:160:            let parsed: ObligationNamespace = displayed.as_str().try_into().unwrap();
crates/amun-constitutional-proof/src/lib.rs:1640:    fn n47_6_certify_fail_missing_obligations() {
crates/amun-constitutional-proof/src/lib.rs:1641:        let mut reg = ObligationRegistry::new();
crates/amun-constitutional-proof/src/lib.rs:1643:            reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:1644:                ObligationId::new(ObligationNamespace::Safety, i),
crates/amun-constitutional-proof/src/lib.rs:1645:                ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:1646:                format!("Obligation {i}"),
crates/amun-constitutional-proof/src/lib.rs:1648:                ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:167:        let id = ObligationId::new(ObligationNamespace::Finality, 3);
crates/amun-constitutional-proof/src/lib.rs:169:        let id2: ObligationId = serde_json::from_str(&json).unwrap();
crates/amun-constitutional-proof/src/lib.rs:1735:        let mut reg = ObligationRegistry::new();
crates/amun-constitutional-proof/src/lib.rs:1737:            reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:1738:                ObligationId::new(ObligationNamespace::Safety, i),
crates/amun-constitutional-proof/src/lib.rs:1739:                ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:173:    // --- ProofObligation tests (S1) ---
crates/amun-constitutional-proof/src/lib.rs:1740:                format!("Obligation {i}"),
crates/amun-constitutional-proof/src/lib.rs:1742:                ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:176:    fn n47_1_s1_create_primary_obligation() {
crates/amun-constitutional-proof/src/lib.rs:177:        let id = ObligationId::new(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:178:        let obl = ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:180:            ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:183:            ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:187:        assert_eq!(obl.kind, ObligationKind::Primary);
crates/amun-constitutional-proof/src/lib.rs:188:        assert_eq!(obl.severity, ObligationSeverity::Critical);
crates/amun-constitutional-proof/src/lib.rs:191:        assert_eq!(obl.status, ObligationStatus::Active);
crates/amun-constitutional-proof/src/lib.rs:197:        let dep = ObligationId::new(ObligationNamespace::Finality, 1);
crates/amun-constitutional-proof/src/lib.rs:198:        let obl = ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:199:            ObligationId::new(ObligationNamespace::Cluster, 1),
crates/amun-constitutional-proof/src/lib.rs:200:            ObligationKind::Derived,
crates/amun-constitutional-proof/src/lib.rs:203:            ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:209:        assert_eq!(obl.kind, ObligationKind::Derived);
crates/amun-constitutional-proof/src/lib.rs:216:        let obl = ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:217:            ObligationId::new(ObligationNamespace::Performance, 1),
crates/amun-constitutional-proof/src/lib.rs:218:            ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:221:            ObligationSeverity::Minor,
crates/amun-constitutional-proof/src/lib.rs:224:        .with_status(ObligationStatus::Frozen);
crates/amun-constitutional-proof/src/lib.rs:226:        assert_eq!(obl.status, ObligationStatus::Frozen);
crates/amun-constitutional-proof/src/lib.rs:231:        let obl = ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:232:            ObligationId::new(ObligationNamespace::Replay, 2),
crates/amun-constitutional-proof/src/lib.rs:233:            ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:236:            ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:239:        .with_dependency(ObligationId::new(ObligationNamespace::Replay, 1));
crates/amun-constitutional-proof/src/lib.rs:242:        let obl2: ProofObligation = serde_json::from_str(&json).unwrap();
crates/amun-constitutional-proof/src/lib.rs:258:        let obl: ProofObligation = serde_json::from_str(json).unwrap();
crates/amun-constitutional-proof/src/lib.rs:265:    fn make_id(namespace: ObligationNamespace, seq: u32) -> ObligationId {
crates/amun-constitutional-proof/src/lib.rs:266:        ObligationId::new(namespace, seq)
crates/amun-constitutional-proof/src/lib.rs:272:        let a = make_id(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:273:        let b = make_id(ObligationNamespace::Safety, 2);
crates/amun-constitutional-proof/src/lib.rs:281:        let a = make_id(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:282:        let b = make_id(ObligationNamespace::Safety, 2);
crates/amun-constitutional-proof/src/lib.rs:283:        let c = make_id(ObligationNamespace::Safety, 3);
crates/amun-constitutional-proof/src/lib.rs:294:        let a = make_id(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:295:        let b = make_id(ObligationNamespace::Safety, 2);
crates/amun-constitutional-proof/src/lib.rs:296:        let c = make_id(ObligationNamespace::Safety, 3);
crates/amun-constitutional-proof/src/lib.rs:310:        g.add_node(make_id(ObligationNamespace::Safety, 1));
crates/amun-constitutional-proof/src/lib.rs:317:        let primary = make_id(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:318:        let derived = make_id(ObligationNamespace::Cluster, 1);
crates/amun-constitutional-proof/src/lib.rs:321:        kinds.insert(primary.clone(), ObligationKind::Primary);
crates/amun-constitutional-proof/src/lib.rs:322:        kinds.insert(derived.clone(), ObligationKind::Derived);
crates/amun-constitutional-proof/src/lib.rs:331:        let d1 = make_id(ObligationNamespace::Cluster, 1);
crates/amun-constitutional-proof/src/lib.rs:332:        let d2 = make_id(ObligationNamespace::Cluster, 2);
crates/amun-constitutional-proof/src/lib.rs:335:        kinds.insert(d1.clone(), ObligationKind::Derived);
crates/amun-constitutional-proof/src/lib.rs:336:        kinds.insert(d2.clone(), ObligationKind::Derived);
crates/amun-constitutional-proof/src/lib.rs:350:    // --- ObligationRegistry tests (S3) ---
crates/amun-constitutional-proof/src/lib.rs:352:    fn simple_obl(id: ObligationId) -> ProofObligation {
crates/amun-constitutional-proof/src/lib.rs:353:        ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:355:            ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:358:            ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:364:    fn n47_1_s3_register_obligation() {
crates/amun-constitutional-proof/src/lib.rs:365:        let mut reg = ObligationRegistry::new();
crates/amun-constitutional-proof/src/lib.rs:366:        let id = make_id(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:374:        let mut reg = ObligationRegistry::new();
crates/amun-constitutional-proof/src/lib.rs:375:        let id = make_id(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:383:        let mut reg = ObligationRegistry::new();
crates/amun-constitutional-proof/src/lib.rs:384:        let id = make_id(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:388:        let result = reg.register(simple_obl(make_id(ObligationNamespace::Safety, 2)));
crates/amun-constitutional-proof/src/lib.rs:394:        let mut reg = ObligationRegistry::new();
crates/amun-constitutional-proof/src/lib.rs:395:        let missing_dep = make_id(ObligationNamespace::Replay, 99);
crates/amun-constitutional-proof/src/lib.rs:396:        let obl = ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:397:            make_id(ObligationNamespace::Safety, 1),
crates/amun-constitutional-proof/src/lib.rs:398:            ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:401:            ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:414:        let mut reg = ObligationRegistry::new();
crates/amun-constitutional-proof/src/lib.rs:415:        let id1 = make_id(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:416:        let id2 = make_id(ObligationNamespace::Safety, 2);
crates/amun-constitutional-proof/src/lib.rs:417:        reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:419:            ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:422:            ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:426:        reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:428:            ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:431:            ObligationSeverity::Minor,
crates/amun-constitutional-proof/src/lib.rs:435:        assert_eq!(reg.by_severity(ObligationSeverity::Critical).len(), 1);
crates/amun-constitutional-proof/src/lib.rs:436:        assert_eq!(reg.by_severity(ObligationSeverity::Minor).len(), 1);
crates/amun-constitutional-proof/src/lib.rs:437:        assert_eq!(reg.by_severity(ObligationSeverity::Major).len(), 0);
crates/amun-constitutional-proof/src/lib.rs:442:        let mut reg = ObligationRegistry::new();
crates/amun-constitutional-proof/src/lib.rs:443:        reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:444:            make_id(ObligationNamespace::Finality, 1),
crates/amun-constitutional-proof/src/lib.rs:445:            ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:448:            ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:452:        reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:453:            make_id(ObligationNamespace::Finality, 2),
crates/amun-constitutional-proof/src/lib.rs:454:            ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:457:            ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:470:        let mut reg = ObligationRegistry::new();
crates/amun-constitutional-proof/src/lib.rs:472:            let id = ObligationId::new(ObligationNamespace::Safety, i);
crates/amun-constitutional-proof/src/lib.rs:473:            reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:475:                ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:476:                format!("Obligation {i}"),
crates/amun-constitutional-proof/src/lib.rs:478:                ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:497:        let mut reg = ObligationRegistry::new();
crates/amun-constitutional-proof/src/lib.rs:499:            let id = ObligationId::new(ObligationNamespace::Safety, i);
crates/amun-constitutional-proof/src/lib.rs:500:            reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:502:                ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:503:                format!("Obligation {i}"),
crates/amun-constitutional-proof/src/lib.rs:505:                ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:514:    fn n47_1_cert_reject_insufficient_obligations() {
crates/amun-constitutional-proof/src/lib.rs:515:        let mut reg = ObligationRegistry::new();
crates/amun-constitutional-proof/src/lib.rs:517:            let id = ObligationId::new(ObligationNamespace::Safety, i);
crates/amun-constitutional-proof/src/lib.rs:518:            reg.register(ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:520:                ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:521:                format!("Obligation {i}"),
crates/amun-constitutional-proof/src/lib.rs:523:                ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:536:        let id = ObligationId::new(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:537:        let result = ObligationResult::satisfied(id.clone(), vec!["EV-001".into()]);
crates/amun-constitutional-proof/src/lib.rs:539:        assert_eq!(result.status, ObligationResultStatus::Satisfied);
crates/amun-constitutional-proof/src/lib.rs:546:        let id = ObligationId::new(ObligationNamespace::Replay, 1);
crates/amun-constitutional-proof/src/lib.rs:548:        let result = ObligationResult::failed(id.clone(), reason.clone(), vec![]);
crates/amun-constitutional-proof/src/lib.rs:549:        assert_eq!(result.status, ObligationResultStatus::Failed);
crates/amun-constitutional-proof/src/lib.rs:555:        let id = ObligationId::new(ObligationNamespace::Cluster, 1);
crates/amun-constitutional-proof/src/lib.rs:556:        let result = ObligationResult::inconclusive(id.clone(), vec!["EV-032".into()]);
crates/amun-constitutional-proof/src/lib.rs:557:        assert_eq!(result.status, ObligationResultStatus::Inconclusive);
crates/amun-constitutional-proof/src/lib.rs:577:    fn n47_2_s0_serialize_obligation_result() {
crates/amun-constitutional-proof/src/lib.rs:578:        let id = ObligationId::new(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:579:        let result = ObligationResult::satisfied(id, vec!["EV-001".into()]);
crates/amun-constitutional-proof/src/lib.rs:581:        let parsed: ObligationResult = serde_json::from_str(&json).unwrap();
crates/amun-constitutional-proof/src/lib.rs:590:            ObligationResult::satisfied(
crates/amun-constitutional-proof/src/lib.rs:591:                ObligationId::new(ObligationNamespace::Safety, 1),
crates/amun-constitutional-proof/src/lib.rs:594:            ObligationResult::satisfied(
crates/amun-constitutional-proof/src/lib.rs:595:                ObligationId::new(ObligationNamespace::Safety, 2),
crates/amun-constitutional-proof/src/lib.rs:617:    fn n47_2_s1_count_satisfied_obligations() {
crates/amun-constitutional-proof/src/lib.rs:619:            ObligationResult::satisfied(
crates/amun-constitutional-proof/src/lib.rs:620:                ObligationId::new(ObligationNamespace::Replay, 1),
crates/amun-constitutional-proof/src/lib.rs:623:            ObligationResult::failed(
crates/amun-constitutional-proof/src/lib.rs:624:                ObligationId::new(ObligationNamespace::Replay, 2),
crates/amun-constitutional-proof/src/lib.rs:628:            ObligationResult::inconclusive(
crates/amun-constitutional-proof/src/lib.rs:629:                ObligationId::new(ObligationNamespace::Replay, 3),
crates/amun-constitutional-proof/src/lib.rs:651:            ObligationResult::satisfied(
crates/amun-constitutional-proof/src/lib.rs:652:                ObligationId::new(ObligationNamespace::Safety, 1),
crates/amun-constitutional-proof/src/lib.rs:655:            ObligationResult::satisfied(
crates/amun-constitutional-proof/src/lib.rs:656:                ObligationId::new(ObligationNamespace::Safety, 2),
crates/amun-constitutional-proof/src/lib.rs:677:        let results = vec![ObligationResult::satisfied(
crates/amun-constitutional-proof/src/lib.rs:678:            ObligationId::new(ObligationNamespace::Cluster, 1),
crates/amun-constitutional-proof/src/lib.rs:707:            ObligationResult::satisfied(
crates/amun-constitutional-proof/src/lib.rs:708:                ObligationId::new(ObligationNamespace::Finality, 1),
crates/amun-constitutional-proof/src/lib.rs:711:            ObligationResult::failed(
crates/amun-constitutional-proof/src/lib.rs:712:                ObligationId::new(ObligationNamespace::Finality, 2),
crates/amun-constitutional-proof/src/lib.rs:735:    fn make_obl(id: ObligationId, severity: ObligationSeverity) -> ProofObligation {
crates/amun-constitutional-proof/src/lib.rs:736:        ProofObligation::new(
crates/amun-constitutional-proof/src/lib.rs:738:            ObligationKind::Primary,
crates/amun-constitutional-proof/src/lib.rs:749:            ObligationId::new(ObligationNamespace::Safety, 1),
crates/amun-constitutional-proof/src/lib.rs:750:            ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:752:        let results = vec![ObligationResult::failed(
crates/amun-constitutional-proof/src/lib.rs:773:            ObligationId::new(ObligationNamespace::Replay, 1),
crates/amun-constitutional-proof/src/lib.rs:774:            ObligationSeverity::Major,
crates/amun-constitutional-proof/src/lib.rs:777:            ObligationId::new(ObligationNamespace::Replay, 2),
crates/amun-constitutional-proof/src/lib.rs:778:            ObligationSeverity::Major,
crates/amun-constitutional-proof/src/lib.rs:782:            ObligationResult::failed(obl1.id, FailureReason::new("M1", "m1"), vec![]),
crates/amun-constitutional-proof/src/lib.rs:783:            ObligationResult::failed(obl2.id, FailureReason::new("M2", "m2"), vec![]),
crates/amun-constitutional-proof/src/lib.rs:801:            ObligationId::new(ObligationNamespace::Evidence, 1),
crates/amun-constitutional-proof/src/lib.rs:802:            ObligationSeverity::Major,
crates/amun-constitutional-proof/src/lib.rs:804:        let results = vec![ObligationResult::failed(
crates/amun-constitutional-proof/src/lib.rs:828:            ObligationId::new(ObligationNamespace::Performance, 1),
crates/amun-constitutional-proof/src/lib.rs:829:            ObligationSeverity::Minor,
crates/amun-constitutional-proof/src/lib.rs:831:        let results = vec![ObligationResult::failed(
crates/amun-constitutional-proof/src/lib.rs:855:            ObligationId::new(ObligationNamespace::Performance, 2),
crates/amun-constitutional-proof/src/lib.rs:856:            ObligationSeverity::Advisory,
crates/amun-constitutional-proof/src/lib.rs:858:        let results = vec![ObligationResult::failed(
crates/amun-constitutional-proof/src/lib.rs:882:            ObligationId::new(ObligationNamespace::Finality, 1),
crates/amun-constitutional-proof/src/lib.rs:883:            ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:885:        let results = vec![ObligationResult::satisfied(
crates/amun-constitutional-proof/src/lib.rs:903:    fn n47_2_s2_count_obligations_correctly() {
crates/amun-constitutional-proof/src/lib.rs:905:            ObligationId::new(ObligationNamespace::Safety, 1),
crates/amun-constitutional-proof/src/lib.rs:906:            ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:909:            ObligationId::new(ObligationNamespace::Safety, 2),
crates/amun-constitutional-proof/src/lib.rs:910:            ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:913:            ObligationId::new(ObligationNamespace::Safety, 3),
crates/amun-constitutional-proof/src/lib.rs:914:            ObligationSeverity::Major,
crates/amun-constitutional-proof/src/lib.rs:918:            ObligationResult::satisfied(obl1.id, vec!["EV1".into()]),
crates/amun-constitutional-proof/src/lib.rs:919:            ObligationResult::satisfied(obl2.id, vec!["EV2".into()]),
crates/amun-constitutional-proof/src/lib.rs:920:            ObligationResult::failed(obl3.id, FailureReason::new("M", "maj"), vec![]),
crates/amun-constitutional-proof/src/lib.rs:940:            ObligationId::new(ObligationNamespace::Performance, 3),
crates/amun-constitutional-proof/src/lib.rs:941:            ObligationSeverity::Advisory,
crates/amun-constitutional-proof/src/lib.rs:943:        let mut result = ObligationResult::satisfied(obl.id.clone(), vec!["EV-W".into()]);
crates/amun-constitutional-proof/src/lib.rs:944:        result.status = ObligationResultStatus::Waived;
crates/amun-constitutional-proof/src/lib.rs:95:        let id = ObligationId::new(ObligationNamespace::Safety, 1);
crates/amun-constitutional-proof/src/lib.rs:961:            ObligationId::new(ObligationNamespace::Cluster, 4),
crates/amun-constitutional-proof/src/lib.rs:962:            ObligationSeverity::Critical,
crates/amun-constitutional-proof/src/lib.rs:964:        let mut result = ObligationResult::satisfied(obl.id.clone(), vec!["EV-NA".into()]);
crates/amun-constitutional-proof/src/lib.rs:965:        result.status = ObligationResultStatus::NotApplicable;
crates/amun-constitutional-proof/src/lib.rs:983:        let id = ObligationId::new(ObligationNamespace::Replay, 1);
crates/amun-constitutional-proof/src/obligation_id.rs:10:pub struct ObligationId {
crates/amun-constitutional-proof/src/obligation_id.rs:11:    pub namespace: ObligationNamespace,
crates/amun-constitutional-proof/src/obligation_id.rs:15:impl ObligationId {
crates/amun-constitutional-proof/src/obligation_id.rs:16:    pub fn new(namespace: ObligationNamespace, sequence: u32) -> Self {
crates/amun-constitutional-proof/src/obligation_id.rs:1:use crate::{ObligationNamespace, RegistryError};
crates/amun-constitutional-proof/src/obligation_id.rs:23:    pub fn namespace(&self) -> ObligationNamespace {
crates/amun-constitutional-proof/src/obligation_id.rs:27:    pub fn sequence(&self) -> u32 {
crates/amun-constitutional-proof/src/obligation_id.rs:32:impl std::fmt::Display for ObligationId {
crates/amun-constitutional-proof/src/obligation_id.rs:33:    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
crates/amun-constitutional-proof/src/obligation_id.rs:38:impl Serialize for ObligationId {
crates/amun-constitutional-proof/src/obligation_id.rs:39:    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
crates/amun-constitutional-proof/src/obligation_id.rs:44:impl<'de> Deserialize<'de> for ObligationId {
crates/amun-constitutional-proof/src/obligation_id.rs:45:    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
crates/amun-constitutional-proof/src/obligation_id.rs:51:impl FromStr for ObligationId {
crates/amun-constitutional-proof/src/obligation_id.rs:52:    type Err = RegistryError;
crates/amun-constitutional-proof/src/obligation_id.rs:54:    fn from_str(s: &str) -> Result<Self, Self::Err> {
crates/amun-constitutional-proof/src/obligation_id.rs:57:            return Err(RegistryError::InvalidObligationIdFormat(s.to_string()));
crates/amun-constitutional-proof/src/obligation_id.rs:59:        let namespace: ObligationNamespace = parts[0].try_into()?;
crates/amun-constitutional-proof/src/obligation_id.rs:62:            .map_err(|_| RegistryError::InvalidObligationIdFormat(s.to_string()))?;
crates/amun-constitutional-proof/src/obligation_kind.rs:5:pub enum ObligationKind {
crates/amun-constitutional-proof/src/obligation_namespace.rs:10:    Replay,
crates/amun-constitutional-proof/src/obligation_namespace.rs:12:    Evidence,
crates/amun-constitutional-proof/src/obligation_namespace.rs:14:    Finality,
crates/amun-constitutional-proof/src/obligation_namespace.rs:25:impl std::fmt::Display for ObligationNamespace {
crates/amun-constitutional-proof/src/obligation_namespace.rs:26:    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
crates/amun-constitutional-proof/src/obligation_namespace.rs:29:            Self::Replay => "REPLAY",
crates/amun-constitutional-proof/src/obligation_namespace.rs:30:            Self::Evidence => "EVIDENCE",
crates/amun-constitutional-proof/src/obligation_namespace.rs:31:            Self::Finality => "FINALITY",
crates/amun-constitutional-proof/src/obligation_namespace.rs:41:impl TryFrom<&str> for ObligationNamespace {
crates/amun-constitutional-proof/src/obligation_namespace.rs:42:    type Error = RegistryError;
crates/amun-constitutional-proof/src/obligation_namespace.rs:44:    fn try_from(s: &str) -> Result<Self, Self::Error> {
crates/amun-constitutional-proof/src/obligation_namespace.rs:47:            "REPLAY" => Ok(Self::Replay),
crates/amun-constitutional-proof/src/obligation_namespace.rs:48:            "EVIDENCE" => Ok(Self::Evidence),
crates/amun-constitutional-proof/src/obligation_namespace.rs:49:            "FINALITY" => Ok(Self::Finality),
crates/amun-constitutional-proof/src/obligation_namespace.rs:6:pub enum ObligationNamespace {
crates/amun-constitutional-proof/src/obligation_namespace.rs:9:    #[serde(rename = "replay")]
crates/amun-constitutional-proof/src/obligation_registry.rs:101:    pub fn total(&self) -> usize {
crates/amun-constitutional-proof/src/obligation_registry.rs:105:    pub fn is_frozen(&self) -> bool {
crates/amun-constitutional-proof/src/obligation_registry.rs:109:    fn collect_kinds(&self) -> std::collections::HashMap<ObligationId, ObligationKind> {
crates/amun-constitutional-proof/src/obligation_registry.rs:10:    obligations: BTreeMap<ObligationId, ProofObligation>,
crates/amun-constitutional-proof/src/obligation_registry.rs:116:    fn build_graph_from_registry(&self) -> DependencyGraph {
crates/amun-constitutional-proof/src/obligation_registry.rs:128:impl Default for ObligationRegistry {
crates/amun-constitutional-proof/src/obligation_registry.rs:129:    fn default() -> Self {
crates/amun-constitutional-proof/src/obligation_registry.rs:15:impl ObligationRegistry {
crates/amun-constitutional-proof/src/obligation_registry.rs:16:    pub fn new() -> Self {
crates/amun-constitutional-proof/src/obligation_registry.rs:24:    pub fn register(&mut self, obligation: ProofObligation) -> Result<(), RegistryError> {
crates/amun-constitutional-proof/src/obligation_registry.rs:4:    DependencyGraph, ObligationId, ObligationKind, ObligationSeverity, ObligationStatus,
crates/amun-constitutional-proof/src/obligation_registry.rs:50:        self.graph.validate_derived_terminate_in_primary(&kinds)?;
crates/amun-constitutional-proof/src/obligation_registry.rs:53:            self.graph = self.build_graph_from_registry();
crates/amun-constitutional-proof/src/obligation_registry.rs:5:    ProofObligation, RegistryError,
crates/amun-constitutional-proof/src/obligation_registry.rs:61:    pub fn freeze(&mut self) -> Result<(), RegistryError> {
crates/amun-constitutional-proof/src/obligation_registry.rs:66:            obl.status = ObligationStatus::Frozen;
crates/amun-constitutional-proof/src/obligation_registry.rs:72:    pub fn get(&self, id: &ObligationId) -> Option<&ProofObligation> {
crates/amun-constitutional-proof/src/obligation_registry.rs:76:    pub fn all_obligations(&self) -> impl Iterator<Item = &ProofObligation> {
crates/amun-constitutional-proof/src/obligation_registry.rs:80:    pub fn by_namespace(&self, ns: ObligationSeverity) -> Vec<&ProofObligation> {
crates/amun-constitutional-proof/src/obligation_registry.rs:87:    pub fn by_severity(&self, severity: ObligationSeverity) -> Vec<&ProofObligation> {
crates/amun-constitutional-proof/src/obligation_registry.rs:94:    pub fn by_phase(&self, phase: &str) -> Vec<&ProofObligation> {
crates/amun-constitutional-proof/src/obligation_registry.rs:9:pub struct ObligationRegistry {
crates/amun-constitutional-proof/src/obligation_result.rs:11:    pub status: ObligationResultStatus,
crates/amun-constitutional-proof/src/obligation_result.rs:20:impl ObligationResult {
crates/amun-constitutional-proof/src/obligation_result.rs:21:    pub fn satisfied(id: ObligationId, evidence_refs: Vec<String>) -> Self {
crates/amun-constitutional-proof/src/obligation_result.rs:24:            status: ObligationResultStatus::Satisfied,
crates/amun-constitutional-proof/src/obligation_result.rs:30:    pub fn failed(id: ObligationId, reason: FailureReason, evidence_refs: Vec<String>) -> Self {
crates/amun-constitutional-proof/src/obligation_result.rs:33:            status: ObligationResultStatus::Failed,
crates/amun-constitutional-proof/src/obligation_result.rs:39:    pub fn inconclusive(id: ObligationId, evidence_refs: Vec<String>) -> Self {
crates/amun-constitutional-proof/src/obligation_result.rs:3:use crate::{FailureReason, ObligationId, ObligationResultStatus};
crates/amun-constitutional-proof/src/obligation_result.rs:42:            status: ObligationResultStatus::Inconclusive,
crates/amun-constitutional-proof/src/obligation_result.rs:7:pub struct ObligationResult {
crates/amun-constitutional-proof/src/obligation_result.rs:9:    pub obligation_id: ObligationId,
crates/amun-constitutional-proof/src/obligation_result_status.rs:5:pub enum ObligationResultStatus {
crates/amun-constitutional-proof/src/obligation_severity.rs:5:pub enum ObligationSeverity {
crates/amun-constitutional-proof/src/obligation_status.rs:10:    Deprecated { superseded_by: ObligationId },
crates/amun-constitutional-proof/src/obligation_status.rs:1:use crate::ObligationId;
crates/amun-constitutional-proof/src/obligation_status.rs:6:pub enum ObligationStatus {
crates/amun-constitutional-proof/src/proof_obligation.rs:11:pub struct ProofObligation {
crates/amun-constitutional-proof/src/proof_obligation.rs:12:    pub id: ObligationId,
crates/amun-constitutional-proof/src/proof_obligation.rs:13:    pub kind: ObligationKind,
crates/amun-constitutional-proof/src/proof_obligation.rs:16:    pub severity: ObligationSeverity,
crates/amun-constitutional-proof/src/proof_obligation.rs:19:    pub depends_on: Vec<ObligationId>,
crates/amun-constitutional-proof/src/proof_obligation.rs:21:    pub status: ObligationStatus,
crates/amun-constitutional-proof/src/proof_obligation.rs:24:impl ProofObligation {
crates/amun-constitutional-proof/src/proof_obligation.rs:26:    pub fn new(
crates/amun-constitutional-proof/src/proof_obligation.rs:27:        id: ObligationId,
crates/amun-constitutional-proof/src/proof_obligation.rs:28:        kind: ObligationKind,
crates/amun-constitutional-proof/src/proof_obligation.rs:31:        severity: ObligationSeverity,
crates/amun-constitutional-proof/src/proof_obligation.rs:3:use crate::{ObligationId, ObligationKind, ObligationSeverity, ObligationStatus};
crates/amun-constitutional-proof/src/proof_obligation.rs:43:            status: ObligationStatus::Active,
crates/amun-constitutional-proof/src/proof_obligation.rs:47:    /// Builder method: add a dependency.
crates/amun-constitutional-proof/src/proof_obligation.rs:48:    pub fn with_dependency(mut self, dep: ObligationId) -> Self {
crates/amun-constitutional-proof/src/proof_obligation.rs:53:    /// Builder method: set the status.
crates/amun-constitutional-proof/src/proof_obligation.rs:54:    pub fn with_status(mut self, status: ObligationStatus) -> Self {
crates/amun-constitutional-proof/src/proof_obligation.rs:59:    /// Builder method: set the version.
crates/amun-constitutional-proof/src/proof_obligation.rs:60:    pub fn with_version(mut self, version: u32) -> Self {
crates/amun-constitutional-proof/src/registry_error.rs:14:    ObligationFrozen(ObligationId),
crates/amun-constitutional-proof/src/registry_error.rs:17:    CircularDependency(ObligationId),
crates/amun-constitutional-proof/src/registry_error.rs:1:use crate::ObligationId;
crates/amun-constitutional-proof/src/registry_error.rs:20:    MissingDependency(ObligationId, ObligationId),
crates/amun-constitutional-proof/src/registry_error.rs:23:    DerivedNotTerminatingInPrimary(ObligationId),
crates/amun-constitutional-proof/src/registry_error.rs:29:    InvalidObligationIdFormat(String),
crates/amun-constitutional-proof/src/registry_error.rs:4:/// Errors that can occur in the Obligation Registry.
crates/amun-constitutional-proof/src/registry_error.rs:8:    DuplicateId(ObligationId),
crates/amun-constitutional-proof/src/report_generator.rs:25:        registry: &ObligationRegistry,
crates/amun-constitutional-proof/src/report_generator.rs:3:use crate::{ConstitutionalVerdict, EvidenceArchive, ObligationRegistry, VerdictResult};
crates/amun-constitutional-proof/src/report_generator.rs:51:            "- **Total Obligations**: {}\n",
crates/amun-constitutional-proof/src/report_generator.rs:80:                "- **Obligations Checked**: {}\n",
crates/amun-constitutional-proof/src/report_generator.rs:84:                "- **Obligations Satisfied**: {}\n",
crates/amun-constitutional-proof/src/verdict_evaluator.rs:27:        obligations: &[ProofObligation],
crates/amun-constitutional-proof/src/verdict_evaluator.rs:28:        results: Vec<ObligationResult>,
crates/amun-constitutional-proof/src/verdict_evaluator.rs:32:        let overall = Self::compute_overall_result(obligations, &results);
crates/amun-constitutional-proof/src/verdict_evaluator.rs:3:    ConstitutionalVerdict, ObligationResult, ObligationResultStatus, ObligationSeverity,
crates/amun-constitutional-proof/src/verdict_evaluator.rs:48:        obligations: &[ProofObligation],
crates/amun-constitutional-proof/src/verdict_evaluator.rs:49:        results: &[ObligationResult],
crates/amun-constitutional-proof/src/verdict_evaluator.rs:4:    ProofObligation, VerdictResult,
crates/amun-constitutional-proof/src/verdict_evaluator.rs:57:            if result.status == ObligationResultStatus::Satisfied
crates/amun-constitutional-proof/src/verdict_evaluator.rs:58:                || result.status == ObligationResultStatus::Waived
crates/amun-constitutional-proof/src/verdict_evaluator.rs:59:                || result.status == ObligationResultStatus::NotApplicable
crates/amun-constitutional-proof/src/verdict_evaluator.rs:77:                Some(ObligationSeverity::Critical) => critical_failures.push(description),
crates/amun-constitutional-proof/src/verdict_evaluator.rs:78:                Some(ObligationSeverity::Major) => major_failures.push(description),
crates/amun-constitutional-proof/src/verdict_evaluator.rs:79:                Some(ObligationSeverity::Minor) => minor_failures.push(description),
crates/amun-constitutional-proof/src/verdict_evaluator.rs:80:                Some(ObligationSeverity::Advisory) => advisory_failures.push(description),
crates/amun-constitutional-runtime/src/block_validator.rs:2:use amun_invariant_engine::invariant_types::InvariantDeclaration;
crates/amun-constitutional-runtime/src/block_validator.rs:32:        invariants: &[InvariantDeclaration],
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:135:        // Phase 5: Invariant Evaluation
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:136:        let (_invariant_results, invariant_evidence) = InvariantEngine::evaluate(
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:36:    /// → Invariants → Evidence → TransitionProof → PCCV → Archive
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:41:        invariants: &[InvariantDeclaration],
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:4:use amun_invariant_engine::invariant_engine::InvariantEngine;
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:5:use amun_invariant_engine::invariant_types::InvariantDeclaration;
crates/amun-constitutional-state/src/lib.rs:15:pub struct StateTransitionRecord {
crates/amun-constitutional-state/src/lib.rs:236:    pub fn verify(&self, records: &[StateTransitionRecord]) -> bool {
crates/amun-constitutional-state/src/lib.rs:23:    journal: Vec<StateTransitionRecord>,
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
crates/amun-constitutional/src/constitutional_failure.rs:206:    pub fn with_invariant(mut self, id: u64, lineage: ConstitutionalHash) -> Self {
crates/amun-constitutional/src/constitutional_failure.rs:54:    pub invariant_lineage_root: Option<ConstitutionalHash>,
crates/amun-constitutional/src/constitutional_witness.rs:1://! ConstitutionalWitness — a formally sufficient constitutional proof surface.
crates/amun-constitutional/src/constitutional_witness.rs:6://! INVARIANT: Every artifact in the witness has a defined WitnessType.
crates/amun-constitutional/src/execution_limits.rs:25:pub struct InvariantLimits {
crates/amun-constitutional/src/execution_limits.rs:47:    pub invariant: InvariantLimits,
crates/amun-constitutional/src/execution_limits.rs:78:            invariant: InvariantLimits {
crates/amun-constitutional/src/state_anchor.rs:13://! INVARIANT (Replay-Derived State Identity):
crates/amun-constitutional/src/state_anchor_scope.rs:104:            AnchorScopeRelationship::StateTransition
crates/amun-constitutional/src/state_anchor_scope.rs:108:            AnchorScopeRelationship::StateTransition
crates/amun-constitutional/src/state_anchor_scope.rs:121:    /// StateTransition is accepted (constitutional evolution),
crates/amun-constitutional/src/state_anchor_scope.rs:131:            | AnchorScopeRelationship::StateTransition => Ok(rel),
crates/amun-constitutional/src/state_anchor_scope.rs:170:            AnchorScopeRelationship::StateTransition
crates/amun-constitutional/src/state_anchor_scope.rs:27:    StateTransition,
crates/amun-dual-verification/src/dual_verifier.rs:18:        invariants: &[InvariantDeclaration],
crates/amun-dual-verification/src/dual_verifier.rs:3:use amun_invariant_engine::invariant_types::InvariantDeclaration;
crates/amun-evidence-engine/src/evidence_engine.rs:153:        archive.insert(ConstitutionalEvidence::InvariantViolation {
crates/amun-evidence-engine/src/evidence_engine.rs:33:            VMEvidence::InvariantViolation { obligation_id } => {
crates/amun-evidence-engine/src/evidence_engine.rs:34:                ConstitutionalEvidence::InvariantViolation {
crates/amun-evidence-engine/src/evidence_types.rs:23:    /// Contract invariant failure — evaluated post-commit.
crates/amun-evidence-engine/src/evidence_types.rs:24:    InvariantViolation {
crates/amun-evidence-engine/src/evidence_types.rs:57:            Self::InvariantViolation {
crates/amun-evidence-engine/src/evidence_types.rs:77:            Self::InvariantViolation { .. } => "invariant_violation",
crates/amun-execution-receipt/src/lib.rs:6://! ## Constitutional Invariants
crates/amun-explorer-api/src/routes/constitutional.rs:12:    ConstitutionalService::list_obligations()
crates/amun-explorer-api/src/services/constitutional_service.rs:46:    pub fn list_obligations() -> ApiResult<Vec<String>> {
crates/amun-failure/src/taxonomy.rs:12:    InvalidStateTransition = 0x2001,
crates/amun-failure/src/taxonomy.rs:27:    // Contract & Invariant (0x6XXX)
crates/amun-failure/src/taxonomy.rs:51:            Self::InvalidStateTransition
crates/amun-failure/src/tests.rs:32:    assert!(!ConstitutionalFault::InvalidStateTransition.should_halt());
crates/amun-formal/src/invariants.rs:12:    pub fn stake_consistency(total_staked: u64, validator_stakes: &[u64]) -> bool {
crates/amun-formal/src/invariants.rs:16:    pub fn quorum_safety(votes: u64, total: u64, threshold_bps: u16) -> bool {
crates/amun-formal/src/invariants.rs:1:pub struct FormalInvariants;
crates/amun-formal/src/invariants.rs:24:    pub fn no_overflow_in_supply(a: u64, b: u64) -> bool {
crates/amun-formal/src/invariants.rs:3:impl FormalInvariants {
crates/amun-formal/src/invariants.rs:4:    pub fn supply_conservation(minted: u64, burned: u64, initial: u64, current: u64) -> bool {
crates/amun-formal/src/invariants.rs:8:    pub fn nonce_monotonicity(prev_nonce: u64, new_nonce: u64) -> bool {
crates/amun-formal/src/lib.rs:2:pub use invariants::FormalInvariants;
crates/amun-gas-engine/src/opcode_costs.rs:22:            OpCode::CheckInvariant { .. } => 50,
crates/amun-genesis/src/constitution.rs:12:    pub fn new(invariant_kernel_hash: [u8; 32], complexity_budget_json: String) -> Self {
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
crates/amun-lineage-law/src/compatibility.rs:7:pub struct CompatibilityTheorem {
crates/amun-networking/tests/n18_node_rejoin.rs:5:// N18.2 — Lifecycle Invariants
crates/amun-networking/tests/n18_node_rejoin.rs:73:// N18.5 — Constitutional Invariant REJOIN-001
crates/amun-nft-fuzz/src/lib.rs:156:        // Invariant: marketplace should not crash the registry
crates/amun-nft-fuzz/src/lib.rs:178:        // Invariant: royalty cannot exceed sale price
crates/amun-nft-fuzz/src/lib.rs:206:        // Invariant: after revoke, cannot propose or veto
crates/amun-nft-fuzz/src/lib.rs:247:        // Invariant: bridge root should be deterministic
crates/amun-nft-fuzz/src/lib.rs:86:        // Invariant: collection must remain active
crates/amun-nft-fuzz/src/lib.rs:93:        // Invariant: active count >= minted NFTs
crates/amun-replay-engine/src/constitutional_governance.rs:125:        // Invariants 1-25 are all constitutional law.
crates/amun-replay-engine/src/constitutional_governance.rs:136:    pub fn can_modify_invariant(&self, invariant_index: u8) -> bool {
crates/amun-replay-engine/src/constitutional_governance.rs:274:    fn test_immutable_invariants() {
crates/amun-replay-engine/src/constitutional_governance.rs:68:    InvariantViolation {
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
crates/amun-replay-semantics/src/lib.rs:1://! Replay Semantics — formal constitutional model for replay.
crates/amun-replay-verifier/src/replay_verifier.rs:35:        invariants: &[InvariantDeclaration],
crates/amun-replay-verifier/src/replay_verifier.rs:3:use amun_invariant_engine::invariant_types::InvariantDeclaration;
crates/amun-replay/src/commit_log.rs:59:            .expect("commit_log: invariant violated — empty after push")
crates/amun-sdk-layer/src/tests.rs:15:    fn test_sandbox_simulation() { let mut sandbox = Sandbox::new(); let a0 = sandbox.create_account(1_000_000).data.expect("test invariant"); let a1 = sandbox.create_account(500_000).data.expect("test invariant"); let result = sandbox.simulate_transfer(a0, a1, 100_000); assert!(result.success); }
crates/amun-sdk-layer/src/tests.rs:7:    fn test_token_api_transfer() { let mut token = TokenApi::create_account(1_000_000).data.expect("test invariant"); let result = TokenApi::transfer(&mut token, 100_000); assert!(result.success); }
crates/amun-self-preservation/src/action_principle.rs:47:pub struct LeastInvariantViolation;
crates/amun-self-preservation/src/action_principle.rs:49:impl LeastInvariantViolation {
crates/amun-self-preservation/src/consistency.rs:24:        invariant: super::legitimacy_guards::GuardedInvariant,
crates/amun-self-preservation/src/legitimacy_guards.rs:13:pub enum GuardedInvariant {
crates/amun-self-preservation/src/legitimacy_guards.rs:27:    InvariantBroken {
crates/amun-self-preservation/src/legitimacy_guards.rs:28:        invariant: GuardedInvariant,
crates/amun-self-preservation/src/legitimacy_guards.rs:38:    pub fn new(invariant: GuardedInvariant, max_violations: u64) -> Self {
crates/amun-self-preservation/src/legitimacy_guards.rs:63:        Some(GuardViolation::InvariantBroken {
crates/amun-self-preservation/src/legitimacy_guards.rs:6:    pub protects: GuardedInvariant,
crates/amun-self-preservation/src/legitimacy_guards.rs:71:    pub fn can_guard(invariant: &GuardedInvariant) -> bool {
crates/amun-self-preservation/src/legitimacy_guards.rs:72:        !matches!(invariant, GuardedInvariant::MetaAmendmentBounds)
crates/amun-self-preservation/src/lib.rs:15:pub use action_principle::{ConstitutionalAction, LeastInvariantViolation};
crates/amun-smt/src/node.rs:3://! # Invariants (enforced by tree, verified by validator)
crates/amun-smt/src/tree.rs:3://! # Core Invariant
crates/amun-smt/src/validator.rs:117:            // Invariant: maximal skip (canonical minimality)
crates/amun-smt/src/validator.rs:46:            // Invariant: no empty children
crates/amun-smt/src/validator.rs:51:            // Invariant: skip_len within bounds
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
crates/amun-state-machine/src/axioms.rs:18:    /// Constitutional freeze boundaries are immutable unless explicitly amended
crates/amun-state-machine/src/axioms.rs:1:/// Constitutional axioms - the foundational mathematical truths
crates/amun-state-machine/src/axioms.rs:21:    EmptyRootInvariant,
crates/amun-state-machine/src/axioms.rs:30:impl ConstitutionalAxiom {
crates/amun-state-machine/src/axioms.rs:44:            Self::EmptyRootInvariant => "Empty root is invariant across protocol versions",
crates/amun-state-machine/src/axioms.rs:54:pub enum AxiomVerification {
crates/amun-state-machine/src/axioms.rs:57:        axiom: ConstitutionalAxiom,
crates/amun-state-machine/src/axioms.rs:5:pub enum ConstitutionalAxiom {
crates/amun-state-machine/src/axioms.rs:62:/// The ConstitutionalAxiomEngine verifies that all axioms hold for a given state.
crates/amun-state-machine/src/axioms.rs:63:pub struct ConstitutionalAxiomEngine;
crates/amun-state-machine/src/axioms.rs:65:impl ConstitutionalAxiomEngine {
crates/amun-state-machine/src/delta_algebra.rs:3:/// Formal constitutional delta types.
crates/amun-state-machine/src/derivation.rs:1:use super::axioms::ConstitutionalAxiom;
crates/amun-state-machine/src/derivation.rs:35:    Axiom(ConstitutionalAxiom),
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
crates/amun-state-machine/src/formal_entropy.rs:16:pub struct EntropySink {
crates/amun-state-machine/src/formal_entropy.rs:22:pub enum EntropySinkType {
crates/amun-state-machine/src/formal_entropy.rs:29:    /// Constitutional court ruling absorbs entropy
crates/amun-state-machine/src/formal_entropy.rs:35:pub struct EntropyConservationLaws;
crates/amun-state-machine/src/formal_entropy.rs:4:pub struct FormalEntropy {
crates/amun-state-machine/src/formal_entropy.rs:58:pub struct EntropyCollapseThreshold {
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
crates/amun-state-machine/src/lib.rs:42:pub use historical_invariants::{HistoricalInvariant, HistoricalInvariantEngine};
crates/amun-state-machine/src/lib.rs:44:pub use invariants::ConstitutionalInvariant;
crates/amun-state-machine/src/meta_amendment.rs:1:use super::absolute_invariants::AbsoluteInvariant;
crates/amun-state-machine/src/meta_amendment.rs:47:        absolute_invariants: &[AbsoluteInvariant],
crates/amun-state-machine/src/meta_amendment.rs:54:                // Must verify that no absolute invariant is touched
crates/amun-state-machine/src/meta_amendment.rs:57:                        AbsoluteInvariant::ReplayDeterminismAbsolute => {
crates/amun-state-machine/src/meta_amendment.rs:65:                        AbsoluteInvariant::ProvableTransitionAbsolute => {
crates/amun-state-machine/src/meta_amendment.rs:9:    pub absolute_invariants_untouchable: Vec<AbsoluteInvariant>,
crates/amun-state-machine/src/postconditions.rs:15:    /// Constitutional invariants must still hold.
crates/amun-state-machine/src/postconditions.rs:16:    InvariantsPreserved,
crates/amun-state-machine/src/verifier.rs:104:                ConstitutionalInvariant::TransitionHistoryAcyclic => {
crates/amun-state-machine/src/verifier.rs:109:                ConstitutionalInvariant::LineageIntact
crates/amun-state-machine/src/verifier.rs:1:use super::invariants::ConstitutionalInvariant;
crates/amun-state-machine/src/verifier.rs:89:    pub fn verify_invariants(
crates/amun-state-machine/src/verifier.rs:92:        invariants: &[ConstitutionalInvariant],
crates/amun-state-machine/src/verifier.rs:97:                ConstitutionalInvariant::NoImpossibleState => {
crates/amun-stf/src/lib.rs:13:pub use transition::StateTransition;
crates/amun-stf/src/tests.rs:14:    stf.apply_set(key, val).expect("test invariant");
crates/amun-stf/src/tests.rs:15:    let new_root = stf.commit().expect("test invariant");
crates/amun-stf/src/tests.rs:27:    stf.apply_set(key, val).expect("test invariant");
crates/amun-stf/src/tests.rs:43:        stf2.apply_set(k, v).expect("test invariant");
crates/amun-stf/src/tests.rs:45:    let r1 = stf1.commit().expect("test invariant");
crates/amun-stf/src/tests.rs:46:    let r2 = stf2.commit().expect("test invariant");
crates/amun-stf/src/transition.rs:10:impl StateTransition {
crates/amun-stf/src/transition.rs:8:pub struct StateTransition;
crates/amun-storage-kernel/CONSTITUTION.md:108:Ratified upon verification of all 10 Constitutional Theorems.
crates/amun-storage-kernel/CONSTITUTION.md:85:**Section 5.1: Uniqueness Invariant**
crates/amun-storage-kernel/tests/specification_compliance.rs:110:    // THEOREM 6: Proof Verification Roundtrip
crates/amun-storage-kernel/tests/specification_compliance.rs:113:    fn theorem_proof_roundtrip() {
crates/amun-storage-kernel/tests/specification_compliance.rs:122:            "Theorem 6 violated: proof verification failed"
crates/amun-storage-kernel/tests/specification_compliance.rs:139:    // THEOREM 8: Empty Tree Absence Proof
crates/amun-storage-kernel/tests/specification_compliance.rs:142:    fn theorem_empty_tree_absence_proof() {
crates/amun-storage-kernel/tests/specification_compliance.rs:149:            "Theorem 8 violated: empty tree absence proof"
crates/amun-storage-kernel/tests/specification_compliance.rs:67:    // THEOREM 4: Proof Depth Invariant
crates/amun-storage-kernel/tests/specification_compliance.rs:70:    fn theorem_proof_depth() {
crates/amun-storage-kernel/tests/specification_compliance.rs:79:            "Theorem 4 violated: proof depth != 256"
crates/amun-storage-kernel/tests/specification_compliance.rs:86:            "Theorem 4 violated: absence proof depth != 256"
crates/amun-survival-console/src/dashboard.rs:10:    invariant_registry: InvariantRegistry,
crates/amun-survival-console/src/dashboard.rs:27:        invariant_registry: InvariantRegistry,
crates/amun-survival-console/src/dashboard.rs:59:        let invariants_hold = health == InvariantHealth::AllInvariantsHold;
crates/amun-survival-console/src/dashboard.rs:6:use amun_invariants::registry::{InvariantHealth, InvariantRegistry};
crates/amun-transaction/src/tests.rs:110:    assert!(r.expect("test invariant").validate_basic().is_err());
crates/amun-transaction/src/tests.rs:64:    assert!(r.expect("test invariant").validate_basic().is_ok());
crates/amun-transaction/src/tests.rs:79:    assert!(r.expect("test invariant").validate_basic().is_ok());
crates/amun-transaction/src/tests.rs:94:    assert!(r.expect("test invariant").validate_basic().is_ok());
crates/amun-verification-kernel/src/lib.rs:50:    FormalProof,
crates/amun-vm-kernel/src/pending_buffer.rs:30:    InvariantViolation {
docs/CCS_Core_Specification_v1.0.md:158:**Constitutional Authority Uniqueness Theorem:**
docs/CCS_Core_Specification_v1.0.md:79:### Axiom 2: Constitutional Determinism
docs/CCS_Core_Specification_v1.0.md:91:### Axiom 4: Constitutional Recoverability
docs/CCS_Core_Specification_v1.1.md:143:Axiom 2 (Constitutional Determinism).
docs/CCS_Core_Specification_v1.1.md:77:### Axiom 2: Constitutional Determinism (Foundational Axiom)
docs/CCS_Core_Specification_v1.1.md:86:### Axiom 3: Constitutional Recoverability
docs/CONSTITUTIONAL_AUDIT.md:22:| **Constitutional Comparability (CC)** | ✅ Partially implemented | Evidence ordering exists but not formalized as standalone module |
docs/CONSTITUTIONAL_AUDIT.md:25:| **Formal Verification** | ❌ Not implemented | TLA+ specification exists in docs but not machine-checked |
docs/CONSTITUTIONAL_AUDIT.md:31:1. **Formal Verification (TLA+)** – CCS axioms documented but not mechanically verified
docs/CONSTITUTIONAL_AUDIT.md:37:5. **Comparability Formalization** – Evidence ordering implicit; should be explicit
docs/DOCS_INDEX.md:365:    v2.027: Formal constitutional state.
docs/DOCS_INDEX.md:429:    Appendix A: Formal definitions and proofs.
docs/N126_FINAL_BASELINE.md:34:9.  StateTransitionValidity   = cert.state_root != [0u8; 32]           PARTIAL
docs/PROTOCOL_HARDENING_ROADMAP.md:6:- [ ] Formal safety proofs for finality
docs/REPOSITORY_LAYOUT.md:19:| constitution/ | Formal constitutional specifications |
docs/REPOSITORY_LAYOUT.md:257:Formal constitutional artifacts.
docs/V0_3_COMPLETION.md:24:## Next Phase: v0.4 – Formal Constitutional Theory
docs/V2_027_FORMAL_CONSTITUTIONAL_STATE.md:12:- **Formal evidence transfer:** All 40 nodes converged on height=1.
docs/V2_027_FORMAL_CONSTITUTIONAL_STATE.md:15:## Constitutional Authority Levels
docs/V2_027_FORMAL_CONSTITUTIONAL_STATE.md:16:| Level | Evidence | Authority |
docs/V2_027_FORMAL_CONSTITUTIONAL_STATE.md:1:# V2-027: Formal Constitutional State Transfer
docs/V4_002_003_CCS_THEORY_NUCLEUS.md:119:- **V4-006:** Mechanized proof of the Authority Uniqueness Theorem.
docs/V6_001_GEOMETRY_OF_AUTHORITY.md:58:## 3. Axioms of Constitutional Space
docs/V6_003_CANONICALIZATION.md:75:### Axiom 2: Constitutional Determinism (Final Form)
docs/V6_003_CANONICALIZATION.md:86:### Axiom 4: Constitutional Monotonicity
docs/V6_004_CONSTITUTIONAL_SELECTION_PRINCIPLE.md:122:1. Formalize `ConstitutionalContinuity` as a measurable quantity.
docs/V6_004_CONSTITUTIONAL_SELECTION_PRINCIPLE.md:29:Formally:
docs/V6_004_CONSTITUTIONAL_SELECTION_PRINCIPLE.md:40:## 2. The Key Theorem (Conjecture)
docs/V6_004_CONSTITUTIONAL_SELECTION_PRINCIPLE.md:42:**Constitutional Authority Theorem:**
docs/V6_004_CONSTITUTIONAL_SELECTION_PRINCIPLE.md:69:## 4. Constitutional Continuity (Formal Conjecture)
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
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:125:**Constitutional Completeness Theorem:**
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:32:Axioms:
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:40:Axioms:
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:50:**Axiom C1: Constitutional Decidability**
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:56:**Axiom C2: Canonical Completeness**
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:61:**Axiom C3: Canonical Uniqueness**
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:68:## 2. The Complete Set of Axioms
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:72:| Layer | Relation | Axioms |
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:80:## 3. The Central Theorem
docs/V7_008_CONSTITUTIONAL_CLOSURE.md:82:**Constitutional Closure Theorem:**
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:55:**Constitutional Closure Theorem (Final Version):**
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:124:| Derivability (⊢_C) | Axiom |
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:125:| Exclusion (⇍_C) | Axiom |
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:126:| Constitutional Comparability (CC) | Axiom |
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:127:| Single History (SH1) | Theorem |
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:128:| Closure (C1-C3) | Theorem |
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:46:## 2. The Comparability Axiom (CC)
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:48:**Axiom CC: Constitutional Comparability**
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:92:## 4. The Central Theorem
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md:94:**Comparability Theorem:**
docs/architecture/CRATE_CLASSIFICATION.md:16:- amun-constitution-core - Constitutional axioms
docs/architecture/PHASE_49_COMPLETE.md:21:| 0.5 | Constitutional Semantics | amun-constitution, amun-invariants, interfaces | ✅ Pure |
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
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:254:## 12. Consensus Invariants
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:258:**Invariant 1:** `constitutional_root == BLAKE3("AMUN_CONSTITUTIONAL_ROOT_V1" || identity_root || evidence_root || governance_root || economic_root)`
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:260:**Invariant 2:** `constitutional_commitment_root == BLAKE3("AMUN_CONSTITUTIONAL_COMMITMENT_V1" || CanonicalSerialization(ConstitutionalCommitment))`
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:262:**Invariant 3:** `state_root` must include `constitutional_commitment_root` as a leaf in its Merkle tree computation.
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:264:**Invariant 4:** The `AppHash` MUST commit to `state_root`. For CCA v1.0, this is defined as `AppHash = state_root`. Future versions may introduce additional roots (such as `execution_root` or `receipt_root`) into the `AppHash` computation, in which case the invariant will be updated to reflect the new hash composition while preserving the requirement that `state_root` remains a committed input.
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:266:**Invariant 5:** Given identical state transition inputs (same genesis, same transactions up to height N), all validators must compute identical values for all roots defined in this specification.
docs/protocol/CCA_v1.0_Constitutional_Consensus_Architecture.md:33:12. Consensus Invariants
docs/protocol/FREEZE_CERTIFICATE_v1.md:29:### Constitutional Invariants (FROZEN)
docs/protocol/replay_physics_v1.md:13:### 2.2 Ordering Invariant
crates/amun-lineage-law/src/compatibility.rs:48:        theorem.theorem_hash = theorem.compute_hash();
crates/amun-lineage-law/src/compatibility.rs:65:        self.compute_hash() == self.theorem_hash
crates/amun-state-machine/src/axioms.rs:66:    pub fn verify_all() -> Vec<AxiomVerification> {
crates/amun-lineage-law/src/compatibility.rs:85:    pub fn determine(theorem: &CompatibilityTheorem) -> Self {
crates/amun-lineage-law/src/compatibility.rs:90:            && theorem.is_replay_compatible
crates/amun-snapshot-engine/src/replay_continuity.rs:28:    /// THEOREM 11: Verify that restoring a snapshot and replaying the
crates/amun-snapshot-engine/src/replay_continuity.rs:2:// THEOREM 11: state -> snapshot -> restore -> WAL replay -> final_root
crates/amun-state-machine/src/axioms.rs:31:    pub fn description(&self) -> &'static str {
crates/amun-state-machine/src/formal_entropy.rs:39:    pub fn first_law(entropy: &FormalEntropy) -> bool {
crates/amun-state-machine/src/formal_entropy.rs:45:    pub fn second_law(entropy: &FormalEntropy) -> bool {
crates/amun-state-machine/src/formal_entropy.rs:50:    pub fn third_law(entropy: &FormalEntropy) -> bool {
crates/amun-state-machine/src/formal_entropy.rs:65:    fn default() -> Self {
crates/amun-storage-kernel/tests/specification_compliance.rs:130:    fn theorem_terminal_empty_is_zero() {
crates/amun-storage-kernel/tests/specification_compliance.rs:32:    fn theorem_delete_reinsert_identity() {
crates/amun-storage-kernel/tests/specification_compliance.rs:53:    fn theorem_empty_identity() {
crates/amun-storage-kernel/tests/specification_compliance.rs:94:    fn theorem_delete_nonexistent_noop() {
crates/amun-storage-kernel/tests/specification_compliance.rs:9:    fn theorem_order_independence_two_keys() {
## Formal Objects
crates/amun-consensus-law/src/lib.rs:9:pub use safety::SafetyAxioms;
crates/amun-consensus-law/src/safety.rs:3:impl SafetyAxioms {
crates/amun-lineage-law/src/compatibility.rs:18:impl CompatibilityTheorem {
crates/amun-lineage-law/src/compatibility.rs:4:/// A CompatibilityTheorem proves that two protocol versions can
crates/amun-lineage-law/src/lib.rs:9:pub use compatibility::{CompatibilityTheorem, CompatibilityVerdict};
crates/amun-state-machine/src/axioms.rs:12:    /// Replay determinism is preserved across all legal transitions
crates/amun-state-machine/src/axioms.rs:13:    ReplayDeterminismPreserved,
crates/amun-state-machine/src/axioms.rs:14:    /// No transition can decrease the epoch
crates/amun-state-machine/src/axioms.rs:16:    /// No transition can decrease generation within the same epoch
crates/amun-state-machine/src/axioms.rs:26:    /// Hostile forks cannot preserve replay
crates/amun-state-machine/src/axioms.rs:27:    HostileForkReplayImpossible,
crates/amun-state-machine/src/axioms.rs:36:                "Every transition preserves or explicitly changes identity"
crates/amun-state-machine/src/axioms.rs:38:            Self::ReplayDeterminismPreserved => {
crates/amun-state-machine/src/axioms.rs:39:                "Replay must remain deterministic across all transitions"
crates/amun-state-machine/src/axioms.rs:47:            Self::HostileForkReplayImpossible => "Hostile forks cannot preserve replay",
crates/amun-state-machine/src/axioms.rs:52:/// Axiom verification result.
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
crates/amun-state-machine/src/derivation.rs:29:    pub theorem: Theorem,
crates/amun-state-machine/src/derivation.rs:37:    TheoremApplication(usize),
crates/amun-state-machine/src/formal_entropy.rs:27:    /// Replay convergence absorbs entropy
crates/amun-state-machine/src/formal_entropy.rs:28:    ReplayConvergence,
crates/amun-state-machine/src/formal_entropy.rs:37:impl EntropyConservationLaws {
crates/amun-state-machine/src/formal_entropy.rs:64:impl Default for EntropyCollapseThreshold {
crates/amun-state-machine/src/lib.rs:40:    EntropyCollapseThreshold, EntropyConservationLaws, EntropySink, EntropySinkType, FormalEntropy,
crates/amun-storage-kernel/CANONICAL_TRAVERSAL_LAW.md:2:## Formal specification of deterministic state traversal
crates/amun-storage-kernel/VALIDITY_HIERARCHY.md:2:## Formal taxonomy of state validity and failure modes
crates/amun-storage-kernel/tests/specification_compliance.rs:105:            "Theorem 5 violated: delete nonexistent changed root"
crates/amun-storage-kernel/tests/specification_compliance.rs:134:            "Theorem 7 violated: terminal empty != ZERO"
crates/amun-storage-kernel/tests/specification_compliance.rs:25:        assert_eq!(root_a.0, root_b.0, "Theorem 1 violated: order independence");
crates/amun-storage-kernel/tests/specification_compliance.rs:45:            "Theorem 2 violated: delete-reinsert identity"
crates/amun-storage-kernel/tests/specification_compliance.rs:62:            "Theorem 3 violated: empty identity"
docs/CCS_Core_Specification_v1.0.md:156:## 5. The Central Theorem
docs/CCS_Core_Specification_v1.0.md:71:## 3. Axioms
docs/CCS_Core_Specification_v1.0.md:73:### Axiom 1: Context Dominance
docs/CCS_Core_Specification_v1.0.md:85:### Axiom 3: Authority Uniqueness
docs/CCS_Core_Specification_v1.1.md:136:## 5. Theorems (Proven Consequences of the Axioms)
docs/CCS_Core_Specification_v1.1.md:138:### Theorem 1: Authority Uniqueness
docs/CCS_Core_Specification_v1.1.md:2:## Core Specification v1.1 – Final Foundational Axioms
docs/CCS_Core_Specification_v1.1.md:5:**Status:** Theory Nucleus – Independent Foundational Axioms
docs/CCS_Core_Specification_v1.1.md:69:## 3. Axioms (Foundational Assumptions of the Theory)
docs/CCS_Core_Specification_v1.1.md:71:### Axiom 1: Context Dominance
docs/DOCS_INDEX.md:308:Protocol Laws & Formal Specifications
docs/PROJECT_INDEX.md:141:## Formal Models
docs/PROJECT_INDEX.md:145:| docs/tla/AmunConsensus.tla | Formal TLA+ model |
docs/V0_3_COMPLETION.md:25:- Formal CCS model
docs/V4_002_003_CCS_THEORY_NUCLEUS.md:118:- **V4-005:** Formal TLA+ specification of the CCS model.
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
docs/V6_003_CANONICALIZATION.md:4:**Status:** Final Axiom – The Core of CCS
docs/V6_003_CANONICALIZATION.md:70:## 4. Axioms (Complete)
docs/V6_003_CANONICALIZATION.md:72:### Axiom 1: Foundational Root
docs/V6_003_CANONICALIZATION.md:81:### Axiom 3: Path Conservation
docs/V6_005_CONTINUITY_ORDERING.md:46:### Axiom C1: Foundational Minimum
docs/V6_005_CONTINUITY_ORDERING.md:51:### Axiom C2: Extension Monotonicity
docs/V6_005_CONTINUITY_ORDERING.md:58:### Axiom C3: Evidence Consistency
docs/V6_005_CONTINUITY_ORDERING.md:66:### Axiom C4: Epoch Dominance
docs/V6_005_CONTINUITY_ORDERING.md:73:### Axiom C5: Transitive Closure
docs/V6_005_CONTINUITY_ORDERING.md:79:## 3. The Key Theorem (Conjecture)
docs/V7_002_DERIVABILITY_GEOMETRY.md:15:Axiom D5 (Deterministic Closure) stated that every context has a unique
docs/V7_002_DERIVABILITY_GEOMETRY.md:34:`∃! P₀` with in-degree 0. This is Axiom D3.
docs/V7_002_DERIVABILITY_GEOMETRY.md:55:## 2. The Central Theorem
docs/V7_002_DERIVABILITY_GEOMETRY.md:57:**Derivability Geometry Theorem:**
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:123:## 4. The Central Theorem
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:51:## 2. Axioms
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:53:### Axiom D1: Foundational Root
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:58:### Axiom D2: Derivability is Transitive
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:63:### Axiom E1: Exclusion is Irreversible
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:68:### Axiom E2: Exclusion Closes Subtrees
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:73:### Axiom E3: Self-Exclusion is Impossible
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md:78:### Axiom E4: Derivability and Exclusion are Disjoint
docs/V7_009_WHY_CLOSURE.md:120:1. Formalize S1-S4 in TLA+ or Coq.
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:112:| Primitive | Derivability `⊢_C` | Axioms D1-D2 |
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:113:| Primitive | Exclusion `⇍_C` | Axioms E1-E4 |
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:114:| Primitive | Single History `SH1` | Axiom SH1 |
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:117:| Derived | Closure | Theorem |
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:39:## 2. The Principle as a Formal Axiom
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:41:### Axiom SH1: Single History
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md:53:## 3. The Central Theorem
docs/V8_001_INDEPENDENCE_OF_SH1.md:77:## 3. Verification of Axioms
docs/V8_RESEARCH_PROPOSAL.md:36:   - Formalize (⊢_C, ⇍_C, SH1) in TLA+
docs/V8_RESEARCH_PROPOSAL.md:37:   - Model-check the Closure Theorem
docs/V8_RESEARCH_PROPOSAL.md:40:2. **V8-002A: Coq/Lean Formalization**
docs/architecture/PHASE_49_COMPLETE.md:108:3. **Formal State Transition Semantics** — (State, Event) → State'
docs/architecture/PHASE_49_COMPLETE.md:128:The foundation is ready for Phase 50: Formal Replay Infrastructure.
docs/architecture/PHASE_49_COMPLETE.md:43:5. **Formal Ordering**: Round monotonicity + sender ordering guarantees
# Invariant
# ProofObligation
# FormalStatement
