# PCCV

## Formal Objects
588:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:163:        use amun_pccv::transition_proof_engine::TransitionProofEngine as PCCVEngine;
592:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:36:    /// → Invariants → Evidence → TransitionProof → PCCV → Archive
616:crates/amun-dual-verification/src/dual_verifier.rs:5:use amun_pccv::transition_proof_engine::TransitionProofEngine as PCCVEngine;
740:crates/amun-pccv/src/pccv_verifier.rs:20:    pub fn verify(proof: &EnhancedTransitionProof, _registry: &ResourceRegistry) -> PCCVResult {
751:crates/amun-pccv/src/transition_proof_engine.rs:82:    ) -> (EnhancedTransitionProof, PCCVResult) {

## Algorithms
352:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:163:        use amun_pccv::transition_proof_engine::TransitionProofEngine as PCCVEngine;
353:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:164:        let enhanced_proof = PCCVEngine::build_proof(
354:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:175:        let pccv_result = amun_pccv::pccv_verifier::PCCVVerifier::verify(&enhanced_proof, registry);
595:crates/amun-dual-verification/src/dual_verifier.rs:42:                let enhanced_proof = PCCVEngine::build_proof(
596:crates/amun-dual-verification/src/dual_verifier.rs:4:use amun_pccv::pccv_verifier::{PCCVResult, PCCVVerifier};
597:crates/amun-dual-verification/src/dual_verifier.rs:53:                let pccv_result = PCCVVerifier::verify(&enhanced_proof, registry);
598:crates/amun-dual-verification/src/dual_verifier.rs:5:use amun_pccv::transition_proof_engine::TransitionProofEngine as PCCVEngine;
889:crates/amun-pccv/src/lib.rs:104:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
890:crates/amun-pccv/src/lib.rs:105:        let result = PCCVVerifier::verify(&p, &reg);
891:crates/amun-pccv/src/lib.rs:156:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
892:crates/amun-pccv/src/lib.rs:157:        let result = PCCVVerifier::verify(&p, &reg);
893:crates/amun-pccv/src/lib.rs:208:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
894:crates/amun-pccv/src/lib.rs:209:        let result = PCCVVerifier::verify(&p, &reg);
895:crates/amun-pccv/src/lib.rs:266:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
896:crates/amun-pccv/src/lib.rs:267:        let result = PCCVVerifier::verify(&p, &reg);
898:crates/amun-pccv/src/lib.rs:48:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
899:crates/amun-pccv/src/lib.rs:50:        let result = PCCVVerifier::verify(&p, &reg);
900:crates/amun-pccv/src/pccv_verifier.rs:17:pub struct PCCVVerifier;
901:crates/amun-pccv/src/pccv_verifier.rs:19:impl PCCVVerifier {
902:crates/amun-pccv/src/pccv_verifier.rs:20:    pub fn verify(proof: &EnhancedTransitionProof, _registry: &ResourceRegistry) -> PCCVResult {
903:crates/amun-pccv/src/transition_proof_engine.rs:104:    use crate::PCCVVerifier;
909:crates/amun-pccv/src/transition_proof_engine.rs:165:        let result = PCCVVerifier::verify(&proof, &reg);
910:crates/amun-pccv/src/transition_proof_engine.rs:233:        proof.proof_hash = PCCVVerifier::compute_proof_hash(&proof);
911:crates/amun-pccv/src/transition_proof_engine.rs:234:        let result = PCCVVerifier::verify(&proof, &reg);
916:crates/amun-pccv/src/transition_proof_engine.rs:68:        proof.proof_hash = PCCVVerifier::compute_proof_hash(&proof);
917:crates/amun-pccv/src/transition_proof_engine.rs:6:use crate::pccv_verifier::{PCCVResult, PCCVVerifier};
920:crates/amun-pccv/src/transition_proof_engine.rs:94:        let result = PCCVVerifier::verify(&proof, registry);
931:crates/amun-pccv/tests/replay_equivalence.rs:100:    let result1 = amun_pccv::pccv_verifier::PCCVVerifier::verify(&proof1, &reg1);
932:crates/amun-pccv/tests/replay_equivalence.rs:101:    let result2 = amun_pccv::pccv_verifier::PCCVVerifier::verify(&proof2, &reg2);

## Tests
1311:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:269:                assert!(pccv_verified, "PCCV must pass for valid execution");
2434:crates/amun-pccv/src/lib.rs:106:        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("R1")));
2435:crates/amun-pccv/src/lib.rs:158:        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("R6")));
2436:crates/amun-pccv/src/lib.rs:210:        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("T1")));
2437:crates/amun-pccv/src/lib.rs:268:        assert!(matches!(result, PCCVResult::Verified { .. }));
2438:crates/amun-pccv/src/lib.rs:51:        assert!(matches!(result, PCCVResult::Verified { .. }));
2439:crates/amun-pccv/src/transition_proof_engine.rs:166:        assert!(matches!(result, PCCVResult::Verified { .. }));
2440:crates/amun-pccv/src/transition_proof_engine.rs:235:        assert!(matches!(result, PCCVResult::Failed { ref reason } if reason.contains("T1")));
2452:crates/amun-pccv/tests/replay_equivalence.rs:102:    assert!(matches!(result1, PCCVResult::Verified { .. }));
2453:crates/amun-pccv/tests/replay_equivalence.rs:103:    assert!(matches!(result2, PCCVResult::Verified { .. }));

## Traceability
crates/amun-byzantine-tests/tests/attack_suite.rs:14:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-byzantine-tests/tests/attack_suite.rs:263:    let proof = TransitionProof::new(
crates/amun-consensus-integration/src/consensus_types.rs:12:    pub transitions: Vec<TransitionProof>,
crates/amun-consensus-integration/src/consensus_types.rs:2:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-constitutional-runtime/src/block_validator.rs:22:/// Validates that every transaction in a block passes PCCV.
crates/amun-constitutional-runtime/src/block_validator.rs:23:/// If any transaction fails PCCV, the entire block is invalid.
crates/amun-constitutional-runtime/src/block_validator.rs:28:    /// The block is valid iff ALL transactions pass PCCV.
crates/amun-constitutional-runtime/src/certificate_chain.rs:103:    use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-constitutional-runtime/src/certificate_chain.rs:126:        let transitions = vec![TransitionProof::new(
crates/amun-constitutional-runtime/src/finality_certificate.rs:109:    pub fn compute_evidence_root(transitions: &[TransitionProof]) -> [u8; 32] {
crates/amun-constitutional-runtime/src/finality_certificate.rs:117:    pub fn compute_pccv_root(transitions: &[TransitionProof]) -> [u8; 32] {
crates/amun-constitutional-runtime/src/finality_certificate.rs:122:                hasher.update(b"AMUN_PCCV_ROOT_V1");
crates/amun-constitutional-runtime/src/finality_certificate.rs:173:        let transitions = vec![TransitionProof::new(
crates/amun-constitutional-runtime/src/finality_certificate.rs:17:    pub transitions: Vec<TransitionProof>,
crates/amun-constitutional-runtime/src/finality_certificate.rs:1:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-constitutional-runtime/src/finality_certificate.rs:25:        transitions: Vec<TransitionProof>,
crates/amun-constitutional-runtime/src/history_root.rs:118:        let transitions = vec![TransitionProof::new(
crates/amun-constitutional-runtime/src/history_root.rs:95:    use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:104:            let proof = TransitionProof::new(
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:10:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:149:        // Phase 6: Build TransitionProof via ProofBuilder
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:161:        // ── N50: Integrated PCCV verification ─────────────────
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:163:        use amun_pccv::transition_proof_engine::TransitionProofEngine as PCCVEngine;
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:164:        let enhanced_proof = PCCVEngine::build_proof(
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:175:        let pccv_result = amun_pccv::pccv_verifier::PCCVVerifier::verify(&enhanced_proof, registry);
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:178:            amun_pccv::pccv_verifier::PCCVResult::Verified { .. }
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:17:    /// Transaction committed successfully with PCCV verification.
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:182:            // PCCV failed — this is a constitutional violation
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:184:                amun_pccv::pccv_verifier::PCCVResult::Failed { reason } => reason.clone(),
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:185:                _ => "Unknown PCCV failure".into(),
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:188:                reason: format!("PCCV verification failed: {}", reason),
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:195:            let proof = TransitionProof::new(
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:20:        transition_proof: TransitionProof,
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:269:                assert!(pccv_verified, "PCCV must pass for valid execution");
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:26:        transition_proof: TransitionProof,
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:271:            _ => panic!("Expected Committed with PCCV"),
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:278:        // constitutional violations via PCCV integration.
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:280:        // so PCCV passes trivially. Full illegal execution testing
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:30:/// The Constitutional Runtime Pipeline with integrated PCCV.
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:36:    /// → Invariants → Evidence → TransitionProof → PCCV → Archive
crates/amun-constitutional-runtime/src/runtime_pipeline.rs:76:            let proof = TransitionProof::new(
crates/amun-dual-verification/src/dual_verifier.rs:41:                // Build enhanced proof for PCCV
crates/amun-dual-verification/src/dual_verifier.rs:42:                let enhanced_proof = PCCVEngine::build_proof(
crates/amun-dual-verification/src/dual_verifier.rs:4:use amun_pccv::pccv_verifier::{PCCVResult, PCCVVerifier};
crates/amun-dual-verification/src/dual_verifier.rs:53:                let pccv_result = PCCVVerifier::verify(&enhanced_proof, registry);
crates/amun-dual-verification/src/dual_verifier.rs:54:                Ok(pccv_verified && matches!(pccv_result, PCCVResult::Verified { .. }))
crates/amun-dual-verification/src/dual_verifier.rs:5:use amun_pccv::transition_proof_engine::TransitionProofEngine as PCCVEngine;
crates/amun-light-client/src/constitutional_client.rs:109:    use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-light-client/src/constitutional_client.rs:132:        let transitions = vec![TransitionProof::new(
crates/amun-light-client/tests/light_client_tests.rs:32:    let transitions = vec![TransitionProof::new(
crates/amun-light-client/tests/light_client_tests.rs:9:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-pccv/src/lib.rs:104:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
crates/amun-pccv/src/lib.rs:105:        let result = PCCVVerifier::verify(&p, &reg);
crates/amun-pccv/src/lib.rs:106:        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("R1")));
crates/amun-pccv/src/lib.rs:122:        let proof = EnhancedTransitionProof {
crates/amun-pccv/src/lib.rs:156:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
crates/amun-pccv/src/lib.rs:157:        let result = PCCVVerifier::verify(&p, &reg);
crates/amun-pccv/src/lib.rs:158:        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("R6")));
crates/amun-pccv/src/lib.rs:174:        let proof = EnhancedTransitionProof {
crates/amun-pccv/src/lib.rs:208:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
crates/amun-pccv/src/lib.rs:209:        let result = PCCVVerifier::verify(&p, &reg);
crates/amun-pccv/src/lib.rs:210:        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("T1")));
crates/amun-pccv/src/lib.rs:228:        let proof = EnhancedTransitionProof {
crates/amun-pccv/src/lib.rs:266:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
crates/amun-pccv/src/lib.rs:267:        let result = PCCVVerifier::verify(&p, &reg);
crates/amun-pccv/src/lib.rs:268:        assert!(matches!(result, PCCVResult::Verified { .. }));
crates/amun-pccv/src/lib.rs:28:        let proof = EnhancedTransitionProof {
crates/amun-pccv/src/lib.rs:48:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
crates/amun-pccv/src/lib.rs:50:        let result = PCCVVerifier::verify(&p, &reg);
crates/amun-pccv/src/lib.rs:51:        assert!(matches!(result, PCCVResult::Verified { .. }));
crates/amun-pccv/src/lib.rs:67:        let proof = EnhancedTransitionProof {
crates/amun-pccv/src/pccv_verifier.rs:100:                return PCCVResult::Failed {
crates/amun-pccv/src/pccv_verifier.rs:108:                return PCCVResult::Failed {
crates/amun-pccv/src/pccv_verifier.rs:115:            return PCCVResult::Failed {
crates/amun-pccv/src/pccv_verifier.rs:137:            return PCCVResult::Failed {
crates/amun-pccv/src/pccv_verifier.rs:142:        PCCVResult::Verified {
crates/amun-pccv/src/pccv_verifier.rs:148:    pub fn compute_proof_hash(proof: &EnhancedTransitionProof) -> [u8; 32] {
crates/amun-pccv/src/pccv_verifier.rs:17:pub struct PCCVVerifier;
crates/amun-pccv/src/pccv_verifier.rs:19:impl PCCVVerifier {
crates/amun-pccv/src/pccv_verifier.rs:1:use crate::enhanced_proof::EnhancedTransitionProof;
crates/amun-pccv/src/pccv_verifier.rs:20:    pub fn verify(proof: &EnhancedTransitionProof, _registry: &ResourceRegistry) -> PCCVResult {
crates/amun-pccv/src/pccv_verifier.rs:22:            return PCCVResult::Failed {
crates/amun-pccv/src/pccv_verifier.rs:29:                return PCCVResult::Failed {
crates/amun-pccv/src/pccv_verifier.rs:38:                return PCCVResult::Failed {
crates/amun-pccv/src/pccv_verifier.rs:50:                return PCCVResult::Failed {
crates/amun-pccv/src/pccv_verifier.rs:59:                    return PCCVResult::Failed {
crates/amun-pccv/src/pccv_verifier.rs:78:                    return PCCVResult::Failed {
crates/amun-pccv/src/pccv_verifier.rs:7:pub enum PCCVResult {
crates/amun-pccv/src/pccv_verifier.rs:86:                    return PCCVResult::Failed {
crates/amun-pccv/src/transition_proof_engine.rs:101:    use crate::enhanced_proof::EnhancedTransitionProof;
crates/amun-pccv/src/transition_proof_engine.rs:103:    use crate::PCCVResult;
crates/amun-pccv/src/transition_proof_engine.rs:104:    use crate::PCCVVerifier;
crates/amun-pccv/src/transition_proof_engine.rs:105:    use crate::TransitionProofEngine;
crates/amun-pccv/src/transition_proof_engine.rs:11:impl TransitionProofEngine {
crates/amun-pccv/src/transition_proof_engine.rs:154:        let proof = TransitionProofEngine::build_proof(
crates/amun-pccv/src/transition_proof_engine.rs:165:        let result = PCCVVerifier::verify(&proof, &reg);
crates/amun-pccv/src/transition_proof_engine.rs:166:        assert!(matches!(result, PCCVResult::Verified { .. }));
crates/amun-pccv/src/transition_proof_engine.rs:218:        let mut proof = EnhancedTransitionProof {
crates/amun-pccv/src/transition_proof_engine.rs:22:    ) -> EnhancedTransitionProof {
crates/amun-pccv/src/transition_proof_engine.rs:233:        proof.proof_hash = PCCVVerifier::compute_proof_hash(&proof);
crates/amun-pccv/src/transition_proof_engine.rs:234:        let result = PCCVVerifier::verify(&proof, &reg);
crates/amun-pccv/src/transition_proof_engine.rs:235:        assert!(matches!(result, PCCVResult::Failed { ref reason } if reason.contains("T1")));
crates/amun-pccv/src/transition_proof_engine.rs:253:        let proof1 = TransitionProofEngine::build_proof(
crates/amun-pccv/src/transition_proof_engine.rs:264:        let proof2 = TransitionProofEngine::build_proof(
crates/amun-pccv/src/transition_proof_engine.rs:52:        let mut proof = EnhancedTransitionProof {
crates/amun-pccv/src/transition_proof_engine.rs:5:use crate::enhanced_proof::EnhancedTransitionProof;
crates/amun-pccv/src/transition_proof_engine.rs:68:        proof.proof_hash = PCCVVerifier::compute_proof_hash(&proof);
crates/amun-pccv/src/transition_proof_engine.rs:6:use crate::pccv_verifier::{PCCVResult, PCCVVerifier};
crates/amun-pccv/src/transition_proof_engine.rs:82:    ) -> (EnhancedTransitionProof, PCCVResult) {
crates/amun-pccv/src/transition_proof_engine.rs:94:        let result = PCCVVerifier::verify(&proof, registry);
crates/amun-pccv/src/transition_proof_engine.rs:9:pub struct TransitionProofEngine;
crates/amun-pccv/tests/replay_equivalence.rs:100:    let result1 = amun_pccv::pccv_verifier::PCCVVerifier::verify(&proof1, &reg1);
crates/amun-pccv/tests/replay_equivalence.rs:101:    let result2 = amun_pccv::pccv_verifier::PCCVVerifier::verify(&proof2, &reg2);
crates/amun-pccv/tests/replay_equivalence.rs:102:    assert!(matches!(result1, PCCVResult::Verified { .. }));
crates/amun-pccv/tests/replay_equivalence.rs:103:    assert!(matches!(result2, PCCVResult::Verified { .. }));
crates/amun-pccv/tests/replay_equivalence.rs:144:            TransitionProofEngine::build_proof(
crates/amun-pccv/tests/replay_equivalence.rs:1:use amun_pccv::pccv_verifier::PCCVResult;
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
crates/amun-replay-consensus/src/replay_backed_consensus.rs:140:    fn compute_proof_root(transitions: &[TransitionProof]) -> [u8; 32] {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:7:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-replay-consensus/src/replay_backed_types.rs:1:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-replay-consensus/src/replay_backed_types.rs:29:    pub transitions: Vec<TransitionProof>,
crates/amun-replay-verifier/src/replay_verifier.rs:32:        proof: &TransitionProof,
crates/amun-replay-verifier/src/replay_verifier.rs:9:use amun_transition_proof::transition_proof::TransitionProof;
crates/amun-resource-core/src/resource_registry.rs:211:    /// enabling Merkle proof construction for PCCV.
crates/amun-state-machine/src/lib.rs:55:    Transition, TransitionAlgebra, TransitionId, TransitionProof, TransitionType,
crates/amun-state-machine/src/transitions.rs:202:pub struct TransitionProof {
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
docs/audit/AUDIT_EVIDENCE_BUNDLE.md:32:| Forged TransitionProof | byz_001 | ✅ Rejected |
docs/audit/AUDIT_EVIDENCE_BUNDLE.md:88:| PCCV | amun-pccv | 11 | ✅ |
docs/audit/SECURITY_INVARIANTS.md:10:| R4 | Certificates are terminal | TransformationMatrix::is_terminal | PCCVVerifier |
docs/audit/SECURITY_INVARIANTS.md:7:| R1 | No duplicate active resource IDs | ResourceRegistry::register_genesis | PCCVVerifier |
docs/audit/THREAT_MODEL.md:14:| Execution | Forged TransitionProof | Proof hash verification (Theorem 1) |
docs/audit/TRACEABILITY_MATRIX.md:14:| N48.5-W20-21A PCCV | amun-pccv | 11 tests | ✅ |

## Dependencies
1106:crates/amun-constitutional-runtime/src/block_validator.rs:22:/// Validates that every transaction in a block passes PCCV.
1107:crates/amun-constitutional-runtime/src/block_validator.rs:23:/// If any transaction fails PCCV, the entire block is invalid.
1108:crates/amun-constitutional-runtime/src/block_validator.rs:28:    /// The block is valid iff ALL transactions pass PCCV.
1126:crates/amun-constitutional-runtime/src/finality_certificate.rs:122:                hasher.update(b"AMUN_PCCV_ROOT_V1");
1150:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:161:        // ── N50: Integrated PCCV verification ─────────────────
1151:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:163:        use amun_pccv::transition_proof_engine::TransitionProofEngine as PCCVEngine;
1152:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:164:        let enhanced_proof = PCCVEngine::build_proof(
1153:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:175:        let pccv_result = amun_pccv::pccv_verifier::PCCVVerifier::verify(&enhanced_proof, registry);
1154:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:178:            amun_pccv::pccv_verifier::PCCVResult::Verified { .. }
1155:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:17:    /// Transaction committed successfully with PCCV verification.
1156:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:182:            // PCCV failed — this is a constitutional violation
1157:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:184:                amun_pccv::pccv_verifier::PCCVResult::Failed { reason } => reason.clone(),
1158:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:185:                _ => "Unknown PCCV failure".into(),
1160:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:188:                reason: format!("PCCV verification failed: {}", reason),
1166:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:269:                assert!(pccv_verified, "PCCV must pass for valid execution");
1168:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:271:            _ => panic!("Expected Committed with PCCV"),
1169:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:278:        // constitutional violations via PCCV integration.
1170:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:280:        // so PCCV passes trivially. Full illegal execution testing
1172:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:30:/// The Constitutional Runtime Pipeline with integrated PCCV.
1175:crates/amun-constitutional-runtime/src/runtime_pipeline.rs:36:    /// → Invariants → Evidence → TransitionProof → PCCV → Archive
1568:crates/amun-dual-verification/src/dual_verifier.rs:41:                // Build enhanced proof for PCCV
1569:crates/amun-dual-verification/src/dual_verifier.rs:42:                let enhanced_proof = PCCVEngine::build_proof(
1570:crates/amun-dual-verification/src/dual_verifier.rs:4:use amun_pccv::pccv_verifier::{PCCVResult, PCCVVerifier};
1571:crates/amun-dual-verification/src/dual_verifier.rs:53:                let pccv_result = PCCVVerifier::verify(&enhanced_proof, registry);
1572:crates/amun-dual-verification/src/dual_verifier.rs:54:                Ok(pccv_verified && matches!(pccv_result, PCCVResult::Verified { .. }))
1573:crates/amun-dual-verification/src/dual_verifier.rs:5:use amun_pccv::transition_proof_engine::TransitionProofEngine as PCCVEngine;
2219:crates/amun-pccv/src/lib.rs:104:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
2220:crates/amun-pccv/src/lib.rs:105:        let result = PCCVVerifier::verify(&p, &reg);
2221:crates/amun-pccv/src/lib.rs:106:        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("R1")));
2228:crates/amun-pccv/src/lib.rs:156:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
2229:crates/amun-pccv/src/lib.rs:157:        let result = PCCVVerifier::verify(&p, &reg);
2230:crates/amun-pccv/src/lib.rs:158:        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("R6")));
2240:crates/amun-pccv/src/lib.rs:208:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
2241:crates/amun-pccv/src/lib.rs:209:        let result = PCCVVerifier::verify(&p, &reg);
2242:crates/amun-pccv/src/lib.rs:210:        assert!(matches!(result, PCCVResult::Failed { reason } if reason.contains("T1")));
2250:crates/amun-pccv/src/lib.rs:266:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
2251:crates/amun-pccv/src/lib.rs:267:        let result = PCCVVerifier::verify(&p, &reg);
2252:crates/amun-pccv/src/lib.rs:268:        assert!(matches!(result, PCCVResult::Verified { .. }));
2255:crates/amun-pccv/src/lib.rs:48:        p.proof_hash = PCCVVerifier::compute_proof_hash(&p);
2257:crates/amun-pccv/src/lib.rs:50:        let result = PCCVVerifier::verify(&p, &reg);
2258:crates/amun-pccv/src/lib.rs:51:        assert!(matches!(result, PCCVResult::Verified { .. }));
2265:crates/amun-pccv/src/pccv_verifier.rs:100:                return PCCVResult::Failed {
2266:crates/amun-pccv/src/pccv_verifier.rs:108:                return PCCVResult::Failed {
2267:crates/amun-pccv/src/pccv_verifier.rs:115:            return PCCVResult::Failed {
2268:crates/amun-pccv/src/pccv_verifier.rs:137:            return PCCVResult::Failed {
2269:crates/amun-pccv/src/pccv_verifier.rs:142:        PCCVResult::Verified {
2271:crates/amun-pccv/src/pccv_verifier.rs:17:pub struct PCCVVerifier;
2272:crates/amun-pccv/src/pccv_verifier.rs:19:impl PCCVVerifier {
2274:crates/amun-pccv/src/pccv_verifier.rs:20:    pub fn verify(proof: &EnhancedTransitionProof, _registry: &ResourceRegistry) -> PCCVResult {
2275:crates/amun-pccv/src/pccv_verifier.rs:22:            return PCCVResult::Failed {
2276:crates/amun-pccv/src/pccv_verifier.rs:29:                return PCCVResult::Failed {
2277:crates/amun-pccv/src/pccv_verifier.rs:38:                return PCCVResult::Failed {
2279:crates/amun-pccv/src/pccv_verifier.rs:50:                return PCCVResult::Failed {
2280:crates/amun-pccv/src/pccv_verifier.rs:59:                    return PCCVResult::Failed {
2281:crates/amun-pccv/src/pccv_verifier.rs:78:                    return PCCVResult::Failed {
2282:crates/amun-pccv/src/pccv_verifier.rs:7:pub enum PCCVResult {
2283:crates/amun-pccv/src/pccv_verifier.rs:86:                    return PCCVResult::Failed {
2286:crates/amun-pccv/src/transition_proof_engine.rs:103:    use crate::PCCVResult;
2287:crates/amun-pccv/src/transition_proof_engine.rs:104:    use crate::PCCVVerifier;
2297:crates/amun-pccv/src/transition_proof_engine.rs:165:        let result = PCCVVerifier::verify(&proof, &reg);
2298:crates/amun-pccv/src/transition_proof_engine.rs:166:        assert!(matches!(result, PCCVResult::Verified { .. }));
2308:crates/amun-pccv/src/transition_proof_engine.rs:233:        proof.proof_hash = PCCVVerifier::compute_proof_hash(&proof);
2309:crates/amun-pccv/src/transition_proof_engine.rs:234:        let result = PCCVVerifier::verify(&proof, &reg);
2310:crates/amun-pccv/src/transition_proof_engine.rs:235:        assert!(matches!(result, PCCVResult::Failed { ref reason } if reason.contains("T1")));
2321:crates/amun-pccv/src/transition_proof_engine.rs:68:        proof.proof_hash = PCCVVerifier::compute_proof_hash(&proof);
2322:crates/amun-pccv/src/transition_proof_engine.rs:6:use crate::pccv_verifier::{PCCVResult, PCCVVerifier};
2325:crates/amun-pccv/src/transition_proof_engine.rs:82:    ) -> (EnhancedTransitionProof, PCCVResult) {
2326:crates/amun-pccv/src/transition_proof_engine.rs:94:        let result = PCCVVerifier::verify(&proof, registry);
2353:crates/amun-pccv/tests/replay_equivalence.rs:100:    let result1 = amun_pccv::pccv_verifier::PCCVVerifier::verify(&proof1, &reg1);
2354:crates/amun-pccv/tests/replay_equivalence.rs:101:    let result2 = amun_pccv::pccv_verifier::PCCVVerifier::verify(&proof2, &reg2);
2355:crates/amun-pccv/tests/replay_equivalence.rs:102:    assert!(matches!(result1, PCCVResult::Verified { .. }));
2356:crates/amun-pccv/tests/replay_equivalence.rs:103:    assert!(matches!(result2, PCCVResult::Verified { .. }));
2364:crates/amun-pccv/tests/replay_equivalence.rs:1:use amun_pccv::pccv_verifier::PCCVResult;
2731:crates/amun-resource-core/src/resource_registry.rs:211:    /// enabling Merkle proof construction for PCCV.
3338:docs/audit/AUDIT_EVIDENCE_BUNDLE.md:88:| PCCV | amun-pccv | 11 | ✅ |
3339:docs/audit/SECURITY_INVARIANTS.md:10:| R4 | Certificates are terminal | TransformationMatrix::is_terminal | PCCVVerifier |
3354:docs/audit/SECURITY_INVARIANTS.md:7:| R1 | No duplicate active resource IDs | ResourceRegistry::register_genesis | PCCVVerifier |
3362:docs/audit/TRACEABILITY_MATRIX.md:14:| N48.5-W20-21A PCCV | amun-pccv | 11 tests | ✅ |
