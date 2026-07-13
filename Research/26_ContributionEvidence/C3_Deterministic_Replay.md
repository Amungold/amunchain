# Deterministic Replay

## Formal Objects
75:crates/amun-constitutional-integration/src/lib.rs:167:            .with_dependency(ObligationId::new(ObligationNamespace::Replay, 2))
78:crates/amun-constitutional-integration/src/lib.rs:174:                ObligationId::new(ObligationNamespace::Replay, 3),
82:crates/amun-constitutional-integration/src/lib.rs:181:            .with_dependency(ObligationId::new(ObligationNamespace::Replay, 2)),
84:crates/amun-constitutional-integration/src/lib.rs:187:                ObligationId::new(ObligationNamespace::Replay, 4),
88:crates/amun-constitutional-integration/src/lib.rs:194:            .with_dependency(ObligationId::new(ObligationNamespace::Replay, 1)),
112:crates/amun-constitutional-integration/src/lib.rs:248:            .with_dependency(ObligationId::new(ObligationNamespace::Replay, 1))
154:crates/amun-constitutional-integration/src/lib.rs:48:            ObligationId::new(ObligationNamespace::Replay, 1),
159:crates/amun-constitutional-integration/src/lib.rs:58:            ObligationId::new(ObligationNamespace::Replay, 2),
233:crates/amun-constitutional-proof/src/lib.rs:1096:        let obl = ObligationId::new(ObligationNamespace::Replay, 1);
244:crates/amun-constitutional-proof/src/lib.rs:122:        let id = ObligationId::new(ObligationNamespace::Replay, 4);
245:crates/amun-constitutional-proof/src/lib.rs:123:        assert_eq!(id.namespace(), ObligationNamespace::Replay);
250:crates/amun-constitutional-proof/src/lib.rs:1310:        let obl = ObligationId::new(ObligationNamespace::Replay, 1);
273:crates/amun-constitutional-proof/src/lib.rs:1496:            ObligationId::new(ObligationNamespace::Replay, 1),
277:crates/amun-constitutional-proof/src/lib.rs:151:            ObligationNamespace::Replay,
327:crates/amun-constitutional-proof/src/lib.rs:232:            ObligationId::new(ObligationNamespace::Replay, 2),
330:crates/amun-constitutional-proof/src/lib.rs:239:        .with_dependency(ObligationId::new(ObligationNamespace::Replay, 1));
365:crates/amun-constitutional-proof/src/lib.rs:395:        let missing_dep = make_id(ObligationNamespace::Replay, 99);
412:crates/amun-constitutional-proof/src/lib.rs:546:        let id = ObligationId::new(ObligationNamespace::Replay, 1);
426:crates/amun-constitutional-proof/src/lib.rs:620:                ObligationId::new(ObligationNamespace::Replay, 1),
428:crates/amun-constitutional-proof/src/lib.rs:624:                ObligationId::new(ObligationNamespace::Replay, 2),
430:crates/amun-constitutional-proof/src/lib.rs:629:                ObligationId::new(ObligationNamespace::Replay, 3),
447:crates/amun-constitutional-proof/src/lib.rs:773:            ObligationId::new(ObligationNamespace::Replay, 1),
449:crates/amun-constitutional-proof/src/lib.rs:777:            ObligationId::new(ObligationNamespace::Replay, 2),
483:crates/amun-constitutional-proof/src/lib.rs:983:        let id = ObligationId::new(ObligationNamespace::Replay, 1);
851:crates/amun-state-machine/src/historical_invariants.rs:133:            HistoricalInvariant::MaxReplayDivergence {
857:crates/amun-state-machine/src/historical_invariants.rs:48:            HistoricalInvariant::MaxReplayDivergence {
871:crates/amun-state-machine/src/meta_amendment.rs:57:                        AbsoluteInvariant::ReplayDeterminismAbsolute => {
1074:docs/audit/SECURITY_INVARIANTS.md:32:| C4 | No conflicting finality | Theorem 5 (Replay-Backed Safety) |

## Algorithms
12:crates/amun-audit/tests/audit_layer06_replay.rs:135:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
14:crates/amun-audit/tests/audit_layer06_replay.rs:3:    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry};
15:crates/amun-audit/tests/audit_layer06_replay.rs:50:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
17:crates/amun-audit/tests/audit_layer06_replay.rs:91:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
18:crates/amun-audit/tests/audit_layer11_crash.rs:124:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
19:crates/amun-audit/tests/audit_layer11_crash.rs:3:    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry, WalIterator};
23:crates/amun-audit/tests/audit_layer15_temporal.rs:3:    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry};
24:crates/amun-audit/tests/audit_layer15_temporal.rs:50:        let result1 = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
25:crates/amun-audit/tests/audit_layer15_temporal.rs:51:        let result2 = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
100:crates/amun-byzantine-tests/tests/attack_suite.rs:56:    let replay = ReplayVerifier::replay(&proof, &program, &mut fresh_reg, &[]);
101:crates/amun-byzantine-tests/tests/attack_suite.rs:9:use amun_replay_verifier::replay_verifier::ReplayVerifier;
282:crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:115:/// Evidence sourced from ReplayVerifier or N109.7 architectural guarantee.
289:crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:19:    // N127A.3: From ReplayVerifier (N109.7 architectural guarantee)
291:crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:80:    pub fn from_replay(replay_deterministic: bool) -> ReplayEvidence {
360:crates/amun-constitutional-semantics/src/lib.rs:120:    #[test] fn test_replay_policy() { assert!(ReplayPolicy::CONSENSUS_AUTHORITATIVE.replay_required); assert!(!ReplayPolicy::EPHEMERAL.replay_required); }
363:crates/amun-constitutional-semantics/src/lib.rs:23:    pub fn verify_boundary(&self, boundary: &ReplayBoundary) -> bool { self.start_sequence >= boundary.finalized_sequence }
364:crates/amun-constitutional-semantics/src/lib.rs:29:    pub fn is_replay_safe(&self) -> bool { matches!(self, EventFinality::Finalized | EventFinality::ReplayCertified) }
367:crates/amun-constitutional-semantics/src/lib.rs:36:    pub fn with_replay_certification(mut self, at_sequence: u64) -> Self { self.finality = EventFinality::ReplayCertified; self.finalized_at_sequence = Some(at_sequence); self }
370:crates/amun-constitutional-semantics/src/lib.rs:58:    pub fn verify_normalization(&self, witnesses: &[(ReplayDomain, u64, [u8; 32])]) -> bool { Self::normalize(witnesses).normalization_root == self.normalization_root }
494:crates/amun-constitutional/src/replay_certificate.rs:170:    pub fn outcome(&self) -> crate::replay_outcome::ReplayOutcome {
609:crates/amun-evidence-finality/src/evidence_finality.rs:184:            let replay = ReplayVerifier::replay(&proof, program, &mut fresh_reg, &[]);
610:crates/amun-evidence-finality/src/evidence_finality.rs:7:use amun_replay_verifier::replay_verifier::{ReplayResult, ReplayVerifier};
648:crates/amun-experimental-framework/src/main.rs:214:            ReplayVerifier::replay(&proof, &program, &mut fresh, &[]);
650:crates/amun-experimental-framework/src/main.rs:311:            ReplayVerifier::replay(&proof, program, &mut fresh, &[]);
651:crates/amun-experimental-framework/src/main.rs:371:                    ReplayVerifier::replay(&transition_proof, &program, &mut fresh, &[]);
653:crates/amun-experimental-framework/src/main.rs:6:use amun_replay_verifier::replay_verifier::ReplayVerifier;
721:crates/amun-live-cluster/src/validator.rs:594:                        // N126.3: Replay determinism from ExecutionEngine
967:crates/amun-replay-cert/src/verifier.rs:5:pub fn verify_certificate(cert: &ReplayCertificate) -> Result<(), &'static str> {
968:crates/amun-replay-consensus/src/replay_backed_consensus.rs:14:/// Replay-Backed Consensus Engine.
975:crates/amun-replay-consensus/src/replay_backed_consensus.rs:56:            let replay = ReplayVerifier::replay(&proof, program, &mut fresh_reg, &[]);
976:crates/amun-replay-consensus/src/replay_backed_consensus.rs:5:use amun_replay_verifier::replay_verifier::{ReplayResult, ReplayVerifier};
979:crates/amun-replay-consensus/src/replay_backed_types.rs:97:    pub fn issue(block: &ReplayVerifiedBlock, qc: ReplayBackedQC) -> Self {
999:crates/amun-replay-semantics/src/lib.rs:20:impl ReplayBoundary { pub fn genesis(genesis_hash: [u8; 32]) -> Self { Self { finalized_sequence: 0, boundary_chain_hash: genesis_hash, boundary_state_root: genesis_hash, epoch: ReplayEpoch::new(genesis_hash, 0, 1), replay_version: 1 } } }
1000:crates/amun-replay-semantics/src/lib.rs:32:    pub fn new(domain: ReplayDomain, epoch: ReplayEpoch, transcript_root: [u8; 32], state_root: [u8; 32], receipt_root: [u8; 32], ordering_root: [u8; 32], boundary: ReplayBoundary, event_count: u64, replay_version: u32) -> Self {
1003:crates/amun-replay-semantics/src/lib.rs:48:    pub fn verify(&self, a: &ReplayCertificate, b: &ReplayCertificate) -> bool {
1004:crates/amun-replay-semantics/src/lib.rs:71:impl ReplayCheckpoint { pub fn verify(&self, boundary: &ReplayBoundary) -> bool { self.sequence >= boundary.finalized_sequence && self.certificate.verify() && self.certificate.boundary.boundary_chain_hash == boundary.boundary_chain_hash } }
1005:crates/amun-replay-semantics/src/lib.rs:75:    pub fn replay_determinism(a: &ReplayCertificate, b: &ReplayCertificate) -> Result<(), ReplayFailure> {
1006:crates/amun-replay-semantics/src/lib.rs:87:    #[test] fn test_cert_self_verifying() { assert!(ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1).verify()); }
1007:crates/amun-replay-semantics/src/lib.rs:88:    #[test] fn test_cert_tamper_detected() { let mut c = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); c.state_root = [0xFF;32]; assert!(!c.verify()); }
1008:crates/amun-replay-semantics/src/lib.rs:89:    #[test] fn test_equivalence_strict() { let c = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); assert!(ReplayEquivalence::Strict.verify(&c, &c)); }
1009:crates/amun-replay-semantics/src/lib.rs:90:    #[test] fn test_law_determinism_divergence() { let a = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); let b = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0xFF;32], [0x03;32], [0x04;32], tb(), 100, 1); assert!(laws::replay_determinism(&a, &b).is_err()); }
1013:crates/amun-replay-verifier/src/replay_verifier.rs:155:            ReplayVerifier::replay(&proof, &p, &mut r2, &[]),
1015:crates/amun-replay-verifier/src/replay_verifier.rs:186:            ReplayVerifier::replay(&proof, &p, &mut r2, &[]),
1017:crates/amun-replay-verifier/src/replay_verifier.rs:217:                ReplayVerifier::replay(&proof, &p, &mut f, &[]),
1018:crates/amun-replay-verifier/src/replay_verifier.rs:28:pub struct ReplayVerifier;
1019:crates/amun-replay-verifier/src/replay_verifier.rs:30:impl ReplayVerifier {
1074:crates/amun-snapshot-engine/src/lib.rs:31:pub use replay_continuity::{ContinuityResult, ReplayContinuityEngine};
1084:crates/amun-snapshot-engine/src/replay_continuity.rs:10:pub struct ReplayContinuityEngine;
1085:crates/amun-snapshot-engine/src/replay_continuity.rs:1:// Replay Continuity Engine
1086:crates/amun-snapshot-engine/src/replay_continuity.rs:27:impl ReplayContinuityEngine {
1230:crates/amun-storage-kernel/src/persistence/wal/iterator.rs:83:pub struct ReplayVerifier;
1231:crates/amun-storage-kernel/src/persistence/wal/iterator.rs:84:impl ReplayVerifier {
1233:crates/amun-storage-kernel/src/persistence/wal/mod.rs:5:pub use iterator::{ReplayVerifier, WalIterator};
1242:crates/amun-storage-kernel/tests/replay_equivalence.rs:136:        let result = ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap());
1244:crates/amun-storage-kernel/tests/replay_equivalence.rs:4:        persistence::wal::{ReplayVerifier, WalEntry},
1245:crates/amun-storage-kernel/tests/replay_equivalence.rs:54:            ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap()).unwrap();
1247:crates/amun-storage-kernel/tests/replay_equivalence.rs:94:        let result = ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap());
1252:crates/amun-testnet-sim/tests/adversarial_tests.rs:209:                                                    // Actually, ReplayVerifier::replay calls execute() which starts
1253:crates/amun-testnet-sim/tests/adversarial_tests.rs:212:                let replay = ReplayVerifier::replay(&transition_proof, &program, &mut fresh, &[]);
1255:crates/amun-testnet-sim/tests/adversarial_tests.rs:7:use amun_replay_verifier::replay_verifier::{ReplayResult, ReplayVerifier};
1267:crates/amun-transcript-semantics/src/lib.rs:102:    #[test] fn test_causal_chain() { let p = EventIdentity::new([0x01;32],[0x00;32],[0xAA;32],1,ReplayDomain::Consensus,[0xBB;32]); let c = EventIdentity::new([0x02;32],[0x01;32],[0xAA;32],2,ReplayDomain::Consensus,[0xBB;32]); assert!(c.verify_causal_chain(&p)); }
1269:crates/amun-transcript-semantics/src/lib.rs:104:    #[test] fn test_immutable_cert() { let c = ImmutableReplayCertificate::new(ReplayDomain::Consensus,[0xBB;32],[0x01;32],[0x02;32],[0x03;32],[0x04;32],100,1); assert!(c.verify()); let h1 = c.certificate_hash(); let h2 = c.certificate_hash(); assert_eq!(h1, h2); }
1273:crates/amun-transcript-semantics/src/lib.rs:64:    pub fn new(domain: ReplayDomain, epoch_id: [u8; 32], transcript_root: [u8; 32], state_root: [u8; 32], receipt_root: [u8; 32], ordering_root: [u8; 32], event_count: u64, replay_version: u32) -> Self {
1293:crates/amun-truth-engine/src/lib.rs:4:pub use engine::{ReplayError, TruthEngine};

## Tests
1155:crates/amun-constitutional-proof/src/lib.rs:123:        assert_eq!(id.namespace(), ObligationNamespace::Replay);
1318:crates/amun-constitutional-semantics/src/lib.rs:116:    #[test] fn test_finality_progression() { assert!(EventFinality::Tentative < EventFinality::Finalized); assert!(EventFinality::Finalized < EventFinality::ReplayCertified); }
1320:crates/amun-constitutional-semantics/src/lib.rs:118:    #[test] fn test_witness_normalization_deterministic() { let w = vec![(ReplayDomain::Consensus,2,[0x02;32]),(ReplayDomain::Consensus,1,[0x01;32])]; assert_eq!(WitnessNormalization::normalize(&w).normalization_root, WitnessNormalization::normalize(&w).normalization_root); }
1322:crates/amun-constitutional-semantics/src/lib.rs:120:    #[test] fn test_replay_policy() { assert!(ReplayPolicy::CONSENSUS_AUTHORITATIVE.replay_required); assert!(!ReplayPolicy::EPHEMERAL.replay_required); }
1459:crates/amun-constitutional/src/divergence_type.rs:81:        assert!(DivergenceType::ReplayError.is_error());
1463:crates/amun-constitutional/src/divergence_type.rs:88:        assert!(!DivergenceType::ReplayError.is_ambiguous());
1484:crates/amun-constitutional/src/replay_certificate.rs:213:        assert!(mc(1, 0, 99, ReplayOutcome::Admitted, None).verify().is_ok());
1491:crates/amun-constitutional/src/replay_outcome.rs:71:        assert!(ReplayOutcome::Admitted.is_admitted());
1492:crates/amun-constitutional/src/replay_outcome.rs:72:        assert!(!ReplayOutcome::Divergent.is_admitted());
1493:crates/amun-constitutional/src/replay_outcome.rs:73:        assert!(!ReplayOutcome::BoundaryViolation.is_admitted());
1494:crates/amun-constitutional/src/replay_outcome.rs:74:        assert!(!ReplayOutcome::ConstitutionalFailure.is_admitted());
1495:crates/amun-constitutional/src/replay_outcome.rs:79:        assert!(!ReplayOutcome::Admitted.is_failure());
1496:crates/amun-constitutional/src/replay_outcome.rs:80:        assert!(ReplayOutcome::Divergent.is_failure());
1497:crates/amun-constitutional/src/replay_outcome.rs:81:        assert!(ReplayOutcome::BoundaryViolation.is_failure());
1498:crates/amun-constitutional/src/replay_outcome.rs:82:        assert!(ReplayOutcome::ConstitutionalFailure.is_failure());
1734:crates/amun-failure/src/tests.rs:36:    assert!(!ConstitutionalFault::ReplayViolation.should_halt());
2251:crates/amun-nft-evidence/tests/n131_evidence_tests.rs:104:    assert_eq!(result, Err(CekError::Law4ReplayDetected));
2526:crates/amun-replay-consensus/src/replay_backed_consensus.rs:242:        assert!(ReplayBackedConsensus::form_consensus(&block, 5, sigs).is_err());
2734:crates/amun-replay-semantics/src/lib.rs:87:    #[test] fn test_cert_self_verifying() { assert!(ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1).verify()); }
2735:crates/amun-replay-semantics/src/lib.rs:88:    #[test] fn test_cert_tamper_detected() { let mut c = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); c.state_root = [0xFF;32]; assert!(!c.verify()); }
2736:crates/amun-replay-semantics/src/lib.rs:89:    #[test] fn test_equivalence_strict() { let c = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); assert!(ReplayEquivalence::Strict.verify(&c, &c)); }
2737:crates/amun-replay-semantics/src/lib.rs:90:    #[test] fn test_law_determinism_divergence() { let a = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); let b = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0xFF;32], [0x03;32], [0x04;32], tb(), 100, 1); assert!(laws::replay_determinism(&a, &b).is_err()); }
2971:crates/amun-testnet-sim/tests/adversarial_tests.rs:106:    assert!(ReplayBackedConsensus::form_consensus(
2972:crates/amun-testnet-sim/tests/adversarial_tests.rs:137:    assert!(ReplayBackedConsensus::form_consensus(
2977:crates/amun-testnet-sim/tests/adversarial_tests.rs:252:    assert!(ReplayBackedConsensus::form_consensus(
2980:crates/amun-testnet-sim/tests/adversarial_tests.rs:63:    assert!(ReplayBackedConsensus::form_consensus(
2981:crates/amun-testnet-sim/tests/adversarial_tests.rs:69:    assert!(ReplayBackedConsensus::form_consensus(
2982:crates/amun-testnet-sim/tests/adversarial_tests.rs:76:        assert!(ReplayBackedConsensus::form_consensus(
3030:crates/amun-transcript-semantics/src/lib.rs:102:    #[test] fn test_causal_chain() { let p = EventIdentity::new([0x01;32],[0x00;32],[0xAA;32],1,ReplayDomain::Consensus,[0xBB;32]); let c = EventIdentity::new([0x02;32],[0x01;32],[0xAA;32],2,ReplayDomain::Consensus,[0xBB;32]); assert!(c.verify_causal_chain(&p)); }
3032:crates/amun-transcript-semantics/src/lib.rs:104:    #[test] fn test_immutable_cert() { let c = ImmutableReplayCertificate::new(ReplayDomain::Consensus,[0xBB;32],[0x01;32],[0x02;32],[0x03;32],[0x04;32],100,1); assert!(c.verify()); let h1 = c.certificate_hash(); let h2 = c.certificate_hash(); assert_eq!(h1, h2); }

## Traceability
crates/amun-audit/tests/audit_layer06_replay.rs:135:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer06_replay.rs:3:    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry};
crates/amun-audit/tests/audit_layer06_replay.rs:50:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer06_replay.rs:91:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer11_crash.rs:124:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer11_crash.rs:3:    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry, WalIterator};
crates/amun-audit/tests/audit_layer15_temporal.rs:3:    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry};
crates/amun-audit/tests/audit_layer15_temporal.rs:50:        let result1 = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-audit/tests/audit_layer15_temporal.rs:51:        let result2 = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
crates/amun-byzantine-tests/tests/attack_suite.rs:56:    let replay = ReplayVerifier::replay(&proof, &program, &mut fresh_reg, &[]);
crates/amun-byzantine-tests/tests/attack_suite.rs:59:        amun_replay_verifier::replay_verifier::ReplayResult::Match { .. }
crates/amun-byzantine-tests/tests/attack_suite.rs:9:use amun_replay_verifier::replay_verifier::ReplayVerifier;
crates/amun-certificate-network/src/distribution.rs:18:    CertificateResponse { certificate: ReplayCertificate },
crates/amun-certificate-network/src/distribution.rs:49:    pub certificate: ReplayCertificate,
crates/amun-certificate-network/src/distribution.rs:56:        certificate: ReplayCertificate,
crates/amun-certificate-network/src/distribution.rs:7:    CertificateInclusionProof, ConstitutionalStateRuntime, ReplayCertificate,
crates/amun-certificate-network/src/gossip.rs:117:    pub fn get_certificate(&self, hash: &[u8; 32]) -> Option<&ReplayCertificate> {
crates/amun-certificate-network/src/gossip.rs:163:        ReplayCertificate,
crates/amun-certificate-network/src/gossip.rs:2:use amun_constitutional_state::{CertificateInclusionProof, ReplayCertificate};
crates/amun-certificate-network/src/gossip.rs:38:        certificates: Vec<ReplayCertificate>,
crates/amun-certificate-network/src/gossip.rs:77:    certificates: BTreeMap<[u8; 32], ReplayCertificate>,
crates/amun-certificate-network/src/gossip.rs:88:    pub fn store_certificate(&mut self, cert: ReplayCertificate) {
crates/amun-chain-checkpoint/src/lib.rs:136:    /// AmunChain proof layers (ReplayCertificate uses `AMUN_REPLAY_CERTIFICATE_V1`,
crates/amun-chain-checkpoint/src/lib.rs:47:/// of the AmunChain proof layers (ReplayCertificate, CertificateMerkleRoot, etc.).
crates/amun-constitutional-block/src/lib.rs:107:/// the block header, a ReplayCertificate, and an inclusion proof.
crates/amun-constitutional-block/src/lib.rs:118:    cert: &ReplayCertificate,
crates/amun-constitutional-block/src/lib.rs:39:use amun_constitutional_state::{ConstitutionalStateRuntime, ReplayCertificate};
crates/amun-constitutional-block/src/lib.rs:50:    cert: &ReplayCertificate,
crates/amun-constitutional-block/src/lib.rs:85:///   - ReplayCertificate::verify() for journal↔state proof
crates/amun-constitutional-block/src/lib.rs:86:pub fn verify_full_replay(
crates/amun-constitutional-block/src/lib.rs:88:    cert: &ReplayCertificate,
crates/amun-constitutional-block/tests/block_tests.rs:253:    use amun_constitutional_block::verify_full_replay;
crates/amun-constitutional-block/tests/block_tests.rs:277:    assert!(verify_full_replay(&block, &cert, rt.journal()).is_ok());
crates/amun-constitutional-block/tests/block_tests.rs:282:    use amun_constitutional_block::verify_full_replay;
crates/amun-constitutional-block/tests/block_tests.rs:309:    assert!(verify_full_replay(&block, &cert, &tampered).is_err());
crates/amun-constitutional-block/tests/block_tests.rs:314:    use amun_constitutional_block::verify_full_replay;
crates/amun-constitutional-block/tests/block_tests.rs:339:    assert!(verify_full_replay(&block, &cert, &rt.journal()[..1]).is_err());
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:115:/// Evidence sourced from ReplayVerifier or N109.7 architectural guarantee.
crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:19:    // N127A.3: From ReplayVerifier (N109.7 architectural guarantee)
crates/amun-constitutional-integration/src/lib.rs:71:            "forall rc in ReplayCertificate : exists ev in EvidenceRoot : ev.replay = rc",
crates/amun-constitutional-state/src/lib.rs:197:pub struct ReplayCertificate {
crates/amun-constitutional-state/src/lib.rs:223:    ) -> ReplayCertificate {
crates/amun-constitutional-state/src/lib.rs:224:        ReplayCertificate {
crates/amun-constitutional-state/src/lib.rs:234:impl ReplayCertificate {
crates/amun-constitutional-state/src/lib.rs:277:impl ReplayCertificate {
crates/amun-constitutional-state/src/lib.rs:357:    /// Build a Merkle root from multiple ReplayCertificates.
crates/amun-constitutional-state/src/lib.rs:359:    pub fn certificate_merkle_root(certificates: &[ReplayCertificate]) -> [u8; 32] {
crates/amun-constitutional-state/src/lib.rs:404:/// A Merkle proof that a specific ReplayCertificate is included
crates/amun-constitutional-state/src/lib.rs:418:        certificates: &[ReplayCertificate],
crates/amun-constitutional/src/architectural_invariants.rs:24:/// A ReplayCertificate attests replay ADMISSIBILITY, not
crates/amun-constitutional/src/execution_receipt.rs:15://!   The certificate_hash links to a ReplayCertificate that attests
crates/amun-constitutional/src/execution_receipt.rs:42:/// `certificate_hash` links to a ReplayCertificate. The certificate model
crates/amun-constitutional/src/hash_domains.rs:45:/// Domain: ReplayCertificate objects
crates/amun-constitutional/src/lib.rs:121:pub use replay_certificate::ReplayCertificate;
crates/amun-constitutional/src/replay_certificate.rs:130:impl ReplayCertificate {
crates/amun-constitutional/src/replay_certificate.rs:179:        parent: &ReplayCertificate,
crates/amun-constitutional/src/replay_certificate.rs:18:pub struct ReplayCertificate {
crates/amun-constitutional/src/replay_certificate.rs:1://! ReplayCertificate — cryptographically scoped admissibility envelope.
crates/amun-constitutional/src/replay_certificate.rs:205:    ) -> ReplayCertificate {
crates/amun-constitutional/src/replay_certificate.rs:206:        ReplayCertificate::new(
crates/amun-constitutional/src/replay_certificate.rs:247:        let c = ReplayCertificate::new(
crates/amun-constitutional/src/replay_certificate.rs:32:impl ConstitutionalIdentity for ReplayCertificate {
crates/amun-constitutional/src/replay_certificate.rs:47:impl ConstitutionalObject for ReplayCertificate {
crates/amun-constitutional/src/schema_registry.rs:69:    /// ReplayCertificate
crates/amun-evidence-finality/src/evidence_finality.rs:184:            let replay = ReplayVerifier::replay(&proof, program, &mut fresh_reg, &[]);
crates/amun-evidence-finality/src/evidence_finality.rs:186:            let verified = matches!(replay, ReplayResult::Match { .. });
crates/amun-evidence-finality/src/evidence_finality.rs:7:use amun_replay_verifier::replay_verifier::{ReplayResult, ReplayVerifier};
crates/amun-experimental-framework/src/main.rs:214:            ReplayVerifier::replay(&proof, &program, &mut fresh, &[]);
crates/amun-experimental-framework/src/main.rs:311:            ReplayVerifier::replay(&proof, program, &mut fresh, &[]);
crates/amun-experimental-framework/src/main.rs:371:                    ReplayVerifier::replay(&transition_proof, &program, &mut fresh, &[]);
crates/amun-experimental-framework/src/main.rs:6:use amun_replay_verifier::replay_verifier::ReplayVerifier;
crates/amun-replay-cert/src/certificate.rs:19:impl ReplayCertificate {
crates/amun-replay-cert/src/certificate.rs:6:pub struct ReplayCertificate {
crates/amun-replay-cert/src/verifier.rs:1:use crate::certificate::ReplayCertificate;
crates/amun-replay-cert/src/verifier.rs:5:pub fn verify_certificate(cert: &ReplayCertificate) -> Result<(), &'static str> {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:56:            let replay = ReplayVerifier::replay(&proof, program, &mut fresh_reg, &[]);
crates/amun-replay-consensus/src/replay_backed_consensus.rs:59:                ReplayResult::Match {
crates/amun-replay-consensus/src/replay_backed_consensus.rs:5:use amun_replay_verifier::replay_verifier::{ReplayResult, ReplayVerifier};
crates/amun-replay-engine/src/equivalence.rs:15:pub struct ConstitutionalReplayResult {
crates/amun-replay-engine/src/equivalence.rs:42:impl ConstitutionalReplayResult {
crates/amun-replay-engine/src/lib.rs:26:    AuthorityProof, CheckpointResult, ConstitutionalReplayResult, ContinuityProof,
crates/amun-replay-engine/src/lib.rs:61:    ) -> Result<ConstitutionalReplayResult, ReplayFailure> {
crates/amun-replay-engine/src/lib.rs:77:        Ok(ConstitutionalReplayResult {
crates/amun-replay-semantics/src/lib.rs:23:pub struct ReplayCertificate {
crates/amun-replay-semantics/src/lib.rs:30:impl ReplayCertificate {
crates/amun-replay-semantics/src/lib.rs:48:    pub fn verify(&self, a: &ReplayCertificate, b: &ReplayCertificate) -> bool {
crates/amun-replay-semantics/src/lib.rs:49:        match self { ReplayEquivalence::Strict => a.certificate_hash == b.certificate_hash, ReplayEquivalence::Semantic => ReplayCertificate::prove_equivalence(a, b), ReplayEquivalence::EpochBounded(ep) => a.epoch.epoch_id == ep.epoch_id && b.epoch.epoch_id == ep.epoch_id && ReplayCertificate::prove_equivalence(a, b) }
crates/amun-replay-semantics/src/lib.rs:67:pub struct ReplayWitness { pub start_sequence: u64, pub end_sequence: u64, pub pre_state_root: [u8; 32], pub post_state_root: [u8; 32], pub transcript_fragment_hash: [u8; 32], pub witness_data: Vec<u8> }
crates/amun-replay-semantics/src/lib.rs:70:pub struct ReplayCheckpoint { pub sequence: u64, pub state_root: [u8; 32], pub transcript_chain_hash: [u8; 32], pub certificate: ReplayCertificate }
crates/amun-replay-semantics/src/lib.rs:75:    pub fn replay_determinism(a: &ReplayCertificate, b: &ReplayCertificate) -> Result<(), ReplayFailure> {
crates/amun-replay-semantics/src/lib.rs:87:    #[test] fn test_cert_self_verifying() { assert!(ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1).verify()); }
crates/amun-replay-semantics/src/lib.rs:88:    #[test] fn test_cert_tamper_detected() { let mut c = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); c.state_root = [0xFF;32]; assert!(!c.verify()); }
crates/amun-replay-semantics/src/lib.rs:89:    #[test] fn test_equivalence_strict() { let c = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); assert!(ReplayEquivalence::Strict.verify(&c, &c)); }
crates/amun-replay-semantics/src/lib.rs:90:    #[test] fn test_law_determinism_divergence() { let a = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); let b = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0xFF;32], [0x03;32], [0x04;32], tb(), 100, 1); assert!(laws::replay_determinism(&a, &b).is_err()); }
crates/amun-replay-verifier/src/replay_verifier.rs:100:                    return ReplayResult::Divergence {
crates/amun-replay-verifier/src/replay_verifier.rs:106:                ReplayResult::Match {
crates/amun-replay-verifier/src/replay_verifier.rs:111:            Ok(PipelineResult::Rejected { evidence, .. }) => ReplayResult::Error {
crates/amun-replay-verifier/src/replay_verifier.rs:114:            Err(e) => ReplayResult::Error { reason: e },
crates/amun-replay-verifier/src/replay_verifier.rs:13:pub enum ReplayResult {
crates/amun-replay-verifier/src/replay_verifier.rs:155:            ReplayVerifier::replay(&proof, &p, &mut r2, &[]),
crates/amun-replay-verifier/src/replay_verifier.rs:156:            ReplayResult::Match { .. }
crates/amun-replay-verifier/src/replay_verifier.rs:186:            ReplayVerifier::replay(&proof, &p, &mut r2, &[]),
crates/amun-replay-verifier/src/replay_verifier.rs:187:            ReplayResult::Match { .. }
crates/amun-replay-verifier/src/replay_verifier.rs:217:                ReplayVerifier::replay(&proof, &p, &mut f, &[]),
crates/amun-replay-verifier/src/replay_verifier.rs:218:                ReplayResult::Match { .. }
crates/amun-replay-verifier/src/replay_verifier.rs:28:pub struct ReplayVerifier;
crates/amun-replay-verifier/src/replay_verifier.rs:30:impl ReplayVerifier {
crates/amun-replay-verifier/src/replay_verifier.rs:36:    ) -> ReplayResult {
crates/amun-replay-verifier/src/replay_verifier.rs:86:                    return ReplayResult::Divergence {
crates/amun-replay-verifier/src/replay_verifier.rs:93:                    return ReplayResult::Divergence {
crates/amun-replay/src/certificate.rs:115:    pub fn latest(&self) -> Option<&ReplayCertificate> {
crates/amun-replay/src/certificate.rs:126:    use crate::validation::{ReplayResult, ReplayValidator};
crates/amun-replay/src/certificate.rs:139:    fn valid_result() -> ReplayResult {
crates/amun-replay/src/certificate.rs:149:        let genesis = ReplayCertificate::genesis([0xAA; 32], 1000);
crates/amun-replay/src/certificate.rs:157:        let mut store = ReplayCertificateStore::new();
crates/amun-replay/src/certificate.rs:158:        let genesis = ReplayCertificate::genesis([0xAA; 32], 1000);
crates/amun-replay/src/certificate.rs:162:        let cert1 = ReplayCertificate::issue(&result, 3, prev_id, [0xAA; 32], 2000).unwrap();
crates/amun-replay/src/certificate.rs:165:        let cert2 = ReplayCertificate::issue(&result, 6, prev_id2, [0xAA; 32], 3000).unwrap();
crates/amun-replay/src/certificate.rs:172:        let mut store = ReplayCertificateStore::new();
crates/amun-replay/src/certificate.rs:173:        let genesis = ReplayCertificate::genesis([0xAA; 32], 1000);
crates/amun-replay/src/certificate.rs:176:        let mut cert = ReplayCertificate::issue(&result, 3, [0xFF; 32], [0xAA; 32], 2000).unwrap();
crates/amun-replay/src/certificate.rs:17:impl ReplayCertificate {
crates/amun-replay/src/certificate.rs:183:        let mut store = ReplayCertificateStore::new();
crates/amun-replay/src/certificate.rs:184:        let genesis = ReplayCertificate::genesis([0xAA; 32], 1000);
crates/amun-replay/src/certificate.rs:188:        let mut cert = ReplayCertificate::issue(&result, 3, prev_id, [0xAA; 32], 2000).unwrap();
crates/amun-replay/src/certificate.rs:195:        let mut store = ReplayCertificateStore::new();
crates/amun-replay/src/certificate.rs:197:            .store(ReplayCertificate::genesis([0xAA; 32], 1000))
crates/amun-replay/src/certificate.rs:19:        result: &ReplayResult,
crates/amun-replay/src/certificate.rs:1:use crate::validation::ReplayResult;
crates/amun-replay/src/certificate.rs:202:            .store(ReplayCertificate::issue(&r, 3, id1, [0xAA; 32], 2000).unwrap())
crates/amun-replay/src/certificate.rs:206:            .store(ReplayCertificate::issue(&r, 6, id2, [0xAA; 32], 3000).unwrap())
crates/amun-replay/src/certificate.rs:6:pub struct ReplayCertificate {
crates/amun-replay/src/certificate.rs:81:pub struct ReplayCertificateStore {
crates/amun-replay/src/certificate.rs:82:    certificates: HashMap<[u8; 32], ReplayCertificate>,
crates/amun-replay/src/certificate.rs:86:impl ReplayCertificateStore {
crates/amun-replay/src/certificate.rs:91:    pub fn store(&mut self, cert: ReplayCertificate) -> Result<(), &'static str> {
crates/amun-replay/src/lib.rs:36:pub use certificate::{ReplayCertificate, ReplayCertificateStore};
crates/amun-replay/src/lib.rs:38:pub use validation::{ReplayResult, ReplayValidator};
crates/amun-replay/src/store.rs:18:    pub fn insert(&mut self, cert: ReplayCertificate) {
crates/amun-replay/src/store.rs:1:use amun_constitutional_state::ReplayCertificate;
crates/amun-replay/src/store.rs:24:    pub fn get(&self, hash: &[u8; 32]) -> Option<&ReplayCertificate> {
crates/amun-replay/src/store.rs:4:/// Stores ReplayCertificates keyed by their certificate_hash.
crates/amun-replay/src/store.rs:50:/// A generic interface for retrieving ReplayCertificates by hash.
crates/amun-replay/src/store.rs:52:    fn get_certificate(&self, hash: &[u8; 32]) -> Option<ReplayCertificate>;
crates/amun-replay/src/store.rs:56:    fn get_certificate(&self, hash: &[u8; 32]) -> Option<ReplayCertificate> {
crates/amun-replay/src/store.rs:7:    certificates: BTreeMap<[u8; 32], ReplayCertificate>,
crates/amun-replay/src/validation.rs:11:impl ReplayResult {
crates/amun-replay/src/validation.rs:34:    pub fn validate(commits: &[StateCommit]) -> ReplayResult {
crates/amun-replay/src/validation.rs:36:            return ReplayResult::success([0u8; 32], 0);
crates/amun-replay/src/validation.rs:42:                return ReplayResult::failure(prev.new_root, curr.previous_root, i);
crates/amun-replay/src/validation.rs:48:        ReplayResult::success(last.new_root, commits.len())
crates/amun-replay/src/validation.rs:4:pub struct ReplayResult {
crates/amun-replay/src/validation.rs:51:    pub fn validate_log(log: &CommitLog) -> ReplayResult {
crates/amun-state-root/src/lib.rs:13:pub use replay::{ReplayCertificate, ReplayEquivalenceProof, ReplayTranscript};
crates/amun-state-root/src/replay.rs:42:pub struct ReplayCertificate {
crates/amun-state-root/src/replay.rs:47:impl CanonicalEncode for ReplayCertificate {
crates/amun-stateless-sync/src/lib.rs:105:    pub fn import_certificate(&mut self, cert: ReplayCertificate) {
crates/amun-stateless-sync/src/lib.rs:34:use amun_constitutional_state::{CertificateInclusionProof, ReplayCertificate};
crates/amun-stateless-sync/src/lib.rs:59:        certificates: Vec<ReplayCertificate>,
crates/amun-stateless-sync/src/lib.rs:80:    certificates: BTreeMap<[u8; 32], ReplayCertificate>,
crates/amun-storage-kernel/VALIDITY_HIERARCHY.md:33:| StateRootMismatch | ReplayVerifier divergence | Quarantine state, resync from trusted peer |
crates/amun-storage-kernel/src/persistence/wal/iterator.rs:83:pub struct ReplayVerifier;
crates/amun-storage-kernel/src/persistence/wal/iterator.rs:84:impl ReplayVerifier {
crates/amun-storage-kernel/src/persistence/wal/iterator.rs:85:    pub fn verify_full_replay(wal_path: &str) -> Result<([u8; 32], u64), String> {
crates/amun-storage-kernel/src/persistence/wal/mod.rs:5:pub use iterator::{ReplayVerifier, WalIterator};
crates/amun-storage-kernel/tests/replay_equivalence.rs:136:        let result = ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap());
crates/amun-storage-kernel/tests/replay_equivalence.rs:4:        persistence::wal::{ReplayVerifier, WalEntry},
crates/amun-storage-kernel/tests/replay_equivalence.rs:54:            ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap()).unwrap();
crates/amun-storage-kernel/tests/replay_equivalence.rs:94:        let result = ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap());
crates/amun-testnet-sim/tests/adversarial_tests.rs:209:                                                    // Actually, ReplayVerifier::replay calls execute() which starts
crates/amun-testnet-sim/tests/adversarial_tests.rs:212:                let replay = ReplayVerifier::replay(&transition_proof, &program, &mut fresh, &[]);
crates/amun-testnet-sim/tests/adversarial_tests.rs:213:                if !matches!(replay, ReplayResult::Match { .. }) {
crates/amun-testnet-sim/tests/adversarial_tests.rs:7:use amun_replay_verifier::replay_verifier::{ReplayResult, ReplayVerifier};
crates/amun-transcript-semantics/src/lib.rs:104:    #[test] fn test_immutable_cert() { let c = ImmutableReplayCertificate::new(ReplayDomain::Consensus,[0xBB;32],[0x01;32],[0x02;32],[0x03;32],[0x04;32],100,1); assert!(c.verify()); let h1 = c.certificate_hash(); let h2 = c.certificate_hash(); assert_eq!(h1, h2); }
crates/amun-transcript-semantics/src/lib.rs:105:    #[test] fn test_witness_hash() { let w = ReplayWitness::MerkleWitness { leaf_hash: [0x01;32], proof_hashes: vec![[0x02;32]], leaf_index: 0 }; assert_ne!(w.witness_hash(), [0;32]); }
crates/amun-transcript-semantics/src/lib.rs:59:pub struct ImmutableReplayCertificate { inner: CertifiedEnvelope }
crates/amun-transcript-semantics/src/lib.rs:62:impl ImmutableReplayCertificate {
crates/amun-transcript-semantics/src/lib.rs:78:pub enum ReplayWitness {
crates/amun-transcript-semantics/src/lib.rs:83:    CompositeWitness { witnesses: Vec<ReplayWitness>, composite_hash: [u8; 32] },
crates/amun-transcript-semantics/src/lib.rs:85:impl ReplayWitness {
crates/amun-transcript-semantics/src/lib.rs:88:        match self { ReplayWitness::MerkleWitness { leaf_hash, proof_hashes, leaf_index } => { h.update(b"MERKLE"); h.update(leaf_hash); h.update(leaf_index.to_le_bytes()); for ph in proof_hashes { h.update(ph); } } ReplayWitness::ExecutionWitness { block_hash, trace_hash, step_count } => { h.update(b"EXECUTION"); h.update(block_hash); h.update(trace_hash); h.update(step_count.to_le_bytes()); } ReplayWitness::TranscriptWitness { start_sequence, end_sequence, fragment_hash } => { h.update(b"TRANSCRIPT"); h.update(start_sequence.to_le_bytes()); h.update(end_sequence.to_le_bytes()); h.update(fragment_hash); } ReplayWitness::ReceiptWitness { receipt_hashes, chain_root } => { h.update(b"RECEIPT"); for rh in receipt_hashes { h.update(rh); } h.update(chain_root); } ReplayWitness::CompositeWitness { composite_hash, .. } => { h.update(b"COMPOSITE"); h.update(composite_hash); } }
crates/amun_state_machine/src/certification/mod.rs:61:pub struct ReplayCertificate {
crates/amun_state_machine/src/certification/mod.rs:70:impl ReplayCertificate {
crates/amun_state_machine/src/lib.rs:24:pub use certification::{ExecutionCertificate, ReplayCertificate, compute_execution_fingerprint};
docs/N126_FINAL_BASELINE.md:45:1. ReplayVerifier integration (ReplayDeterminism)
docs/audit/SECURITY_INVARIANTS.md:8:| R2 | Consumed resources cannot be used | VMKernel::verify | ReplayVerifier |
docs/constitutional/phase84_freeze.md:54:- ReplayVerifier checks every frame's state_root against reconstructed state

## Dependencies
2:crates/amun-audit/tests/audit_layer06_replay.rs:135:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
3:crates/amun-audit/tests/audit_layer06_replay.rs:25:    // CONST-REPLAY-001: Replay produces identical state
4:crates/amun-audit/tests/audit_layer06_replay.rs:3:    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry};
5:crates/amun-audit/tests/audit_layer06_replay.rs:50:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
6:crates/amun-audit/tests/audit_layer06_replay.rs:60:                    "CONST-REPLAY-001 VIOLATION: Replayed root diverges"
7:crates/amun-audit/tests/audit_layer06_replay.rs:63:            Err(e) => panic!("CONST-REPLAY-001: Replay failed: {}", e),
8:crates/amun-audit/tests/audit_layer06_replay.rs:67:    // CONST-REPLAY-002: Replay detects state root divergence
9:crates/amun-audit/tests/audit_layer06_replay.rs:91:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
10:crates/amun-audit/tests/audit_layer06_replay.rs:98:    // CONST-REPLAY-003: Replay detects epoch regression
11:crates/amun-audit/tests/audit_layer11_crash.rs:124:        let result = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
12:crates/amun-audit/tests/audit_layer11_crash.rs:3:    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry, WalIterator};
13:crates/amun-audit/tests/audit_layer15_temporal.rs:25:    // CONST-TEMP-001: Replaying same WAL twice produces identical root
14:crates/amun-audit/tests/audit_layer15_temporal.rs:3:    use amun_storage_kernel::persistence::wal::{ReplayVerifier, WalEntry};
15:crates/amun-audit/tests/audit_layer15_temporal.rs:50:        let result1 = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
16:crates/amun-audit/tests/audit_layer15_temporal.rs:51:        let result2 = ReplayVerifier::verify_full_replay(path.to_str().unwrap());
17:crates/amun-audit/tests/audit_layer15_temporal.rs:58:                    "CONST-TEMP-001 VIOLATION: Replay twice produces different roots"
18:crates/amun-audit/tests/audit_layer15_temporal.rs:62:            (Err(e), _) | (_, Err(e)) => panic!("CONST-TEMP-001: Replay failed: {}", e),
19:crates/amun-authority-registry/src/recovery.rs:22:        // 2. Replay only WAL entries after the snapshot height
20:crates/amun-authority-registry/src/wal.rs:31:    /// Replay all entries in order against a fresh GovernanceState.
21:crates/amun-authority-registry/src/wal.rs:40:    /// Replay and then finalize all approved proposals.
79:crates/amun-byzantine-tests/tests/attack_suite.rs:56:    let replay = ReplayVerifier::replay(&proof, &program, &mut fresh_reg, &[]);
80:crates/amun-byzantine-tests/tests/attack_suite.rs:59:        amun_replay_verifier::replay_verifier::ReplayResult::Match { .. }
85:crates/amun-byzantine-tests/tests/attack_suite.rs:9:use amun_replay_verifier::replay_verifier::ReplayVerifier;
86:crates/amun-canonical-collections/src/lib.rs:120:impl<K: Ord + CanonicalEncode, V: CanonicalEncode> ReplaySafe for CanonicalMap<K, V> {
87:crates/amun-canonical-collections/src/lib.rs:160:impl<T: CanonicalEncode> ReplaySafe for CanonicalDeque<T> {
88:crates/amun-canonical-collections/src/lib.rs:18:pub trait ReplaySafe: DeterministicCollection + CanonicalEncode {
89:crates/amun-canonical-collections/src/lib.rs:72:impl<T: Ord + CanonicalEncode> ReplaySafe for CanonicalSet<T> {
90:crates/amun-certificate-network/src/distribution.rs:18:    CertificateResponse { certificate: ReplayCertificate },
91:crates/amun-certificate-network/src/distribution.rs:49:    pub certificate: ReplayCertificate,
92:crates/amun-certificate-network/src/distribution.rs:56:        certificate: ReplayCertificate,
93:crates/amun-certificate-network/src/distribution.rs:7:    CertificateInclusionProof, ConstitutionalStateRuntime, ReplayCertificate,
94:crates/amun-certificate-network/src/gossip.rs:117:    pub fn get_certificate(&self, hash: &[u8; 32]) -> Option<&ReplayCertificate> {
95:crates/amun-certificate-network/src/gossip.rs:163:        ReplayCertificate,
96:crates/amun-certificate-network/src/gossip.rs:2:use amun_constitutional_state::{CertificateInclusionProof, ReplayCertificate};
97:crates/amun-certificate-network/src/gossip.rs:38:        certificates: Vec<ReplayCertificate>,
98:crates/amun-certificate-network/src/gossip.rs:77:    certificates: BTreeMap<[u8; 32], ReplayCertificate>,
99:crates/amun-certificate-network/src/gossip.rs:88:    pub fn store_certificate(&mut self, cert: ReplayCertificate) {
100:crates/amun-chain-checkpoint/src/lib.rs:136:    /// AmunChain proof layers (ReplayCertificate uses `AMUN_REPLAY_CERTIFICATE_V1`,
101:crates/amun-chain-checkpoint/src/lib.rs:47:/// of the AmunChain proof layers (ReplayCertificate, CertificateMerkleRoot, etc.).
103:crates/amun-chain-checkpoint/src/lib.rs:60:    /// Replay certificate merkle root at end_height.
105:crates/amun-codec/src/versioned.rs:79:                ConstitutionalFault::ReplayViolation,
106:crates/amun-compatibility/src/matrix.rs:23:            CompatibilityClass::ReplayCompatible
345:crates/amun-consensus-network/src/execution_commitment.rs:13://   - Replay of commitments across heights
478:crates/amun-consensus-network/src/slashing_ledger.rs:1:// N119 — Deterministic Slashing Ledger & Replay Protection
481:crates/amun-consensus-network/src/slashing_ledger.rs:69:        // N119.2: Replay protection
579:crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:1:// N121.2 — Deterministic Replay of Slashing State
581:crates/amun-consensus-network/tests/n121_2_deterministic_replay.rs:92:/// N121.2: Replay from empty always produces zero root
589:crates/amun-consensus/src/round_state_machine.rs:132:        //   - ReplayDeterminism (replay root == original root)
590:crates/amun-constitution-builder/src/federation.rs:51:            "Replay isolation preserved".into(),
591:crates/amun-constitution-builder/src/federation.rs:89:        lines.push(format!("Replay Boundary: {}", self.replay_boundary));
592:crates/amun-constitution-builder/src/treaty.rs:64:        lines.push("Replay Boundaries:".to_string());
593:crates/amun-constitution/src/freeze_validator.rs:49:            return Err(FreezeViolation::ReplayPreservationViolation {
594:crates/amun-constitution/src/freeze_validator.rs:8:    ReplayPreservationViolation { field: String },
595:crates/amun-constitution/src/replay.rs:16:impl ReplayContext {
596:crates/amun-constitution/src/replay.rs:53:impl CanonicalEncode for ReplayContext {
597:crates/amun-constitution/src/replay.rs:61:impl CanonicalDecode for ReplayContext {
598:crates/amun-constitution/src/replay.rs:6:pub struct ReplayContext {
618:crates/amun-constitutional-block/src/lib.rs:107:/// the block header, a ReplayCertificate, and an inclusion proof.
619:crates/amun-constitutional-block/src/lib.rs:118:    cert: &ReplayCertificate,
620:crates/amun-constitutional-block/src/lib.rs:39:use amun_constitutional_state::{ConstitutionalStateRuntime, ReplayCertificate};
621:crates/amun-constitutional-block/src/lib.rs:50:    cert: &ReplayCertificate,
622:crates/amun-constitutional-block/src/lib.rs:81:///   Block → Certificate → Journal → Replay → StateRoot → Accept/Reject
623:crates/amun-constitutional-block/src/lib.rs:85:///   - ReplayCertificate::verify() for journal↔state proof
624:crates/amun-constitutional-block/src/lib.rs:88:    cert: &ReplayCertificate,
628:crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:113:// N127A.3 — Replay Evidence Adapter
629:crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:115:/// Evidence sourced from ReplayVerifier or N109.7 architectural guarantee.
630:crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:117:pub struct ReplayEvidence {
643:crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:149:        replay: ReplayEvidence,
650:crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:190:        let replay = ReplayEvidence {
653:crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:19:    // N127A.3: From ReplayVerifier (N109.7 architectural guarantee)
661:crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:80:    pub fn from_replay(replay_deterministic: bool) -> ReplayEvidence {
662:crates/amun-constitutional-enforcement/src/constitutional_evidence.rs:81:        ReplayEvidence {
669:crates/amun-constitutional-enforcement/src/evidence_providers.rs:126:        let evidence = ReplayEvidenceProvider::from_qc_and_state(true, true, true);
670:crates/amun-constitutional-enforcement/src/evidence_providers.rs:132:        let evidence = ReplayEvidenceProvider::from_qc_and_state(false, true, true);
676:crates/amun-constitutional-enforcement/src/evidence_providers.rs:38:pub struct ReplayEvidenceProvider;
677:crates/amun-constitutional-enforcement/src/evidence_providers.rs:40:impl ReplayEvidenceProvider {
678:crates/amun-constitutional-enforcement/src/evidence_providers.rs:45:    ) -> ReplayEvidence {
679:crates/amun-constitutional-enforcement/src/evidence_providers.rs:46:        ReplayEvidence {
689:crates/amun-constitutional-enforcement/src/evidence_providers.rs:7:    ConstitutionalEvidence, ExecutionEvidence, GovernanceEvidence, QcEvidence, ReplayEvidence,
692:crates/amun-constitutional-enforcement/src/evidence_providers.rs:87:    let replay = ReplayEvidenceProvider::from_qc_and_state(
699:crates/amun-constitutional-enforcement/src/evidence_records.rs:106:    pub replay_evidence: ReplayEvidence,
705:crates/amun-constitutional-enforcement/src/evidence_records.rs:129:        replay_evidence: ReplayEvidence,
712:crates/amun-constitutional-enforcement/src/evidence_records.rs:206:        let ev = ReplayEvidence::new(root, root);
713:crates/amun-constitutional-enforcement/src/evidence_records.rs:212:        let ev = ReplayEvidence::new([0x42; 32], [0xFF; 32]);
718:crates/amun-constitutional-enforcement/src/evidence_records.rs:224:            ReplayEvidence::new([0x42; 32], [0x42; 32]),
723:crates/amun-constitutional-enforcement/src/evidence_records.rs:244:            ReplayEvidence::new([0x42; 32], [0x42; 32]),
728:crates/amun-constitutional-enforcement/src/evidence_records.rs:74:pub struct ReplayEvidence {
729:crates/amun-constitutional-enforcement/src/evidence_records.rs:85:impl ReplayEvidence {
732:crates/amun-constitutional-enforcement/src/lib.rs:110:                ConstitutionalLaw::ReplayDeterminism,
736:crates/amun-constitutional-enforcement/src/lib.rs:169:                &ConstitutionalLaw::ReplayDeterminism,
737:crates/amun-constitutional-enforcement/src/lib.rs:171:                "Replay divergence",
741:crates/amun-constitutional-enforcement/src/lib.rs:63:    ReplayDeterminism,
747:crates/amun-constitutional-enforcement/src/proof_engine.rs:170:        // Replay Determinism
748:crates/amun-constitutional-enforcement/src/proof_engine.rs:173:                law: ConstitutionalLaw::ReplayDeterminism,
749:crates/amun-constitutional-enforcement/src/proof_engine.rs:174:                description: "Replay produced different state root".into(),
757:crates/amun-constitutional-enforcement/src/state_transition.rs:158:                    .any(|v| v.law == ConstitutionalLaw::ReplayDeterminism));
758:crates/amun-constitutional-enforcement/src/state_transition.rs:54:        // N126.2: Replay must produce identical state root
759:crates/amun-constitutional-enforcement/src/state_transition.rs:57:                law: ConstitutionalLaw::ReplayDeterminism,
760:crates/amun-constitutional-enforcement/src/state_transition.rs:59:                    "Replay divergence: execution={:02x?} replay={:02x?}",
766:crates/amun-constitutional-geometry/src/curvature.rs:22:/// Replay curvature measures deformation in the causal geometry
767:crates/amun-constitutional-geometry/src/curvature.rs:25:pub struct ReplayCurvature {
768:crates/amun-constitutional-geometry/src/curvature.rs:31:impl ReplayCurvature {
771:crates/amun-constitutional-geometry/src/emergent_horizons.rs:31:    /// Replay curvature becomes infinite
772:crates/amun-constitutional-geometry/src/emergent_horizons.rs:32:    ReplaySingularity,
774:crates/amun-constitutional-geometry/src/flow_dynamics.rs:29:    ReplayForce { strength: f64 },
776:crates/amun-constitutional-geometry/src/flow_dynamics.rs:64:                ConstitutionalForce::ReplayForce { strength } => {
777:crates/amun-constitutional-geometry/src/flow_dynamics.rs:65:                    total[1] += strength; // Replay dimension
778:crates/amun-constitutional-geometry/src/horizons.rs:15:    /// Replay horizon: replay divergence has made reconciliation impossible
779:crates/amun-constitutional-geometry/src/horizons.rs:16:    ReplayHorizon,
780:crates/amun-constitutional-geometry/src/lib.rs:17:pub use curvature::{CausalCurvature, LegitimacyCurvature, ReplayCurvature};
781:crates/amun-constitutional-geometry/src/metrics.rs:90:        // Replay distance: can replay be preserved?
783:crates/amun-constitutional-geometry/src/stability.rs:49:    /// Replay determinism creates a stable attractor
784:crates/amun-constitutional-geometry/src/stability.rs:50:    ReplayAttractor,
785:crates/amun-constitutional-geometry/src/trajectories.rs:44:                        | super::horizons::HorizonType::ReplayHorizon
797:crates/amun-constitutional-integration/src/lib.rs:162:                "Replay results must match finalized state",
798:crates/amun-constitutional-integration/src/lib.rs:163:                "forall b in Finalized : state_root(Replay(b)) = state_root(b)",
799:crates/amun-constitutional-integration/src/lib.rs:167:            .with_dependency(ObligationId::new(ObligationNamespace::Replay, 2))
802:crates/amun-constitutional-integration/src/lib.rs:174:                ObligationId::new(ObligationNamespace::Replay, 3),
803:crates/amun-constitutional-integration/src/lib.rs:176:                "Replay certificates are unique",
805:crates/amun-constitutional-integration/src/lib.rs:181:            .with_dependency(ObligationId::new(ObligationNamespace::Replay, 2)),
807:crates/amun-constitutional-integration/src/lib.rs:187:                ObligationId::new(ObligationNamespace::Replay, 4),
808:crates/amun-constitutional-integration/src/lib.rs:189:                "Replay chain is continuous",
809:crates/amun-constitutional-integration/src/lib.rs:190:                "forall b1,b2 in ReplayChain : b1.height+1=b2.height implies linked(b1,b2)",
810:crates/amun-constitutional-integration/src/lib.rs:194:            .with_dependency(ObligationId::new(ObligationNamespace::Replay, 1)),
824:crates/amun-constitutional-integration/src/lib.rs:242:                "Finalized = Replay Verified = Evidence Certified",
825:crates/amun-constitutional-integration/src/lib.rs:243:                "|Finalized| = |ReplayVerified| = |EvidenceCertified|",
827:crates/amun-constitutional-integration/src/lib.rs:248:            .with_dependency(ObligationId::new(ObligationNamespace::Replay, 1))
857:crates/amun-constitutional-integration/src/lib.rs:48:            ObligationId::new(ObligationNamespace::Replay, 1),
860:crates/amun-constitutional-integration/src/lib.rs:58:            ObligationId::new(ObligationNamespace::Replay, 2),
861:crates/amun-constitutional-integration/src/lib.rs:60:            "Replay execution is deterministic",
862:crates/amun-constitutional-integration/src/lib.rs:61:            "forall b : Replay(b, t1) = Replay(b, t2)",
865:crates/amun-constitutional-integration/src/lib.rs:71:            "forall rc in ReplayCertificate : exists ev in EvidenceRoot : ev.replay = rc",
875:crates/amun-constitutional-kernel/src/lib.rs:14://! - Replay-safe receipts: every transition leaves a verifiable proof.
876:crates/amun-constitutional-kernel/tests/kernel_tests.rs:67:    // Replay must produce identical result
922:crates/amun-constitutional-proof/src/evidence_type.rs:9:    ReplayEvidence,
941:crates/amun-constitutional-proof/src/lib.rs:1096:        let obl = ObligationId::new(ObligationNamespace::Replay, 1);
942:crates/amun-constitutional-proof/src/lib.rs:1097:        let ev = make_evidence("EV-ARC-1", "N42", EvidenceType::ReplayEvidence, obl);
952:crates/amun-constitutional-proof/src/lib.rs:1141:                EvidenceType::ReplayEvidence,
971:crates/amun-constitutional-proof/src/lib.rs:122:        let id = ObligationId::new(ObligationNamespace::Replay, 4);
973:crates/amun-constitutional-proof/src/lib.rs:123:        assert_eq!(id.namespace(), ObligationNamespace::Replay);
976:crates/amun-constitutional-proof/src/lib.rs:1256:                EvidenceType::ReplayEvidence,
982:crates/amun-constitutional-proof/src/lib.rs:1298:                EvidenceType::ReplayEvidence,
983:crates/amun-constitutional-proof/src/lib.rs:1310:        let obl = ObligationId::new(ObligationNamespace::Replay, 1);
984:crates/amun-constitutional-proof/src/lib.rs:1313:            make_evidence("EV-ADM-1", "N42", EvidenceType::ReplayEvidence, obl.clone());
987:crates/amun-constitutional-proof/src/lib.rs:1318:            make_evidence("EV-ADM-2", "N42", EvidenceType::ReplayEvidence, obl.clone());
990:crates/amun-constitutional-proof/src/lib.rs:1323:            make_evidence("EV-ADM-3", "N42", EvidenceType::ReplayEvidence, obl.clone());
993:crates/amun-constitutional-proof/src/lib.rs:1327:        let mut rejected = make_evidence("EV-ADM-4", "N42", EvidenceType::ReplayEvidence, obl);
1012:crates/amun-constitutional-proof/src/lib.rs:1496:            ObligationId::new(ObligationNamespace::Replay, 1),
1014:crates/amun-constitutional-proof/src/lib.rs:151:            ObligationNamespace::Replay,
1031:crates/amun-constitutional-proof/src/lib.rs:202:            "|Finalized| = |ReplayVerified| = |EvidenceCertified|",
1034:crates/amun-constitutional-proof/src/lib.rs:232:            ObligationId::new(ObligationNamespace::Replay, 2),
1035:crates/amun-constitutional-proof/src/lib.rs:234:            "Replay determinism",
1036:crates/amun-constitutional-proof/src/lib.rs:235:            "forall b : Replay(b, t1) = Replay(b, t2)",
1037:crates/amun-constitutional-proof/src/lib.rs:239:        .with_dependency(ObligationId::new(ObligationNamespace::Replay, 1));
1042:crates/amun-constitutional-proof/src/lib.rs:395:        let missing_dep = make_id(ObligationNamespace::Replay, 99);
1053:crates/amun-constitutional-proof/src/lib.rs:546:        let id = ObligationId::new(ObligationNamespace::Replay, 1);
1055:crates/amun-constitutional-proof/src/lib.rs:620:                ObligationId::new(ObligationNamespace::Replay, 1),
1056:crates/amun-constitutional-proof/src/lib.rs:624:                ObligationId::new(ObligationNamespace::Replay, 2),
1057:crates/amun-constitutional-proof/src/lib.rs:629:                ObligationId::new(ObligationNamespace::Replay, 3),
1058:crates/amun-constitutional-proof/src/lib.rs:639:            VerdictResult::ConditionalPass(vec!["Replay-2 failed".into()]),
1063:crates/amun-constitutional-proof/src/lib.rs:773:            ObligationId::new(ObligationNamespace::Replay, 1),
1064:crates/amun-constitutional-proof/src/lib.rs:777:            ObligationId::new(ObligationNamespace::Replay, 2),
1068:crates/amun-constitutional-proof/src/lib.rs:983:        let id = ObligationId::new(ObligationNamespace::Replay, 1);
1070:crates/amun-constitutional-proof/src/lib.rs:986:            EvidenceType::ReplayEvidence,
1072:crates/amun-constitutional-proof/src/obligation_namespace.rs:10:    Replay,
1075:crates/amun-constitutional-proof/src/obligation_namespace.rs:29:            Self::Replay => "REPLAY",
1078:crates/amun-constitutional-proof/src/obligation_namespace.rs:47:            "REPLAY" => Ok(Self::Replay),
1097:crates/amun-constitutional-quarantine/src/rehabilitation.rs:15:    ReplayContinuityVerification,
1098:crates/amun-constitutional-quarantine/src/rehabilitation.rs:28:                RehabilitationStep::ReplayContinuityVerification,
1099:crates/amun-constitutional-quarantine/src/rehabilitation.rs:32:                RehabilitationStep::ReplayContinuityVerification,
1100:crates/amun-constitutional-quarantine/src/rehabilitation.rs:37:                RehabilitationStep::ReplayContinuityVerification,
1101:crates/amun-constitutional-quarantine/src/rehabilitation.rs:43:                RehabilitationStep::ReplayContinuityVerification,
1184:crates/amun-constitutional-semantics/src/lib.rs:113:    fn mk(seq: u64, hash: [u8; 32], parent: [u8; 32]) -> TranscriptEntry { TranscriptEntry::Consensus(ConsensusEvent { identity: EventIdentity::new(hash, parent, [0xAA; 32], seq, ReplayDomain::Consensus, [0xBB; 32]), round: seq, event_type: ConsensusEventType::Proposal, authority: EventAuthority::Authoritative }) }
1185:crates/amun-constitutional-semantics/src/lib.rs:116:    #[test] fn test_finality_progression() { assert!(EventFinality::Tentative < EventFinality::Finalized); assert!(EventFinality::Finalized < EventFinality::ReplayCertified); }
1187:crates/amun-constitutional-semantics/src/lib.rs:118:    #[test] fn test_witness_normalization_deterministic() { let w = vec![(ReplayDomain::Consensus,2,[0x02;32]),(ReplayDomain::Consensus,1,[0x01;32])]; assert_eq!(WitnessNormalization::normalize(&w).normalization_root, WitnessNormalization::normalize(&w).normalization_root); }
1188:crates/amun-constitutional-semantics/src/lib.rs:120:    #[test] fn test_replay_policy() { assert!(ReplayPolicy::CONSENSUS_AUTHORITATIVE.replay_required); assert!(!ReplayPolicy::EPHEMERAL.replay_required); }
1189:crates/amun-constitutional-semantics/src/lib.rs:23:    pub fn verify_boundary(&self, boundary: &ReplayBoundary) -> bool { self.start_sequence >= boundary.finalized_sequence }
1191:crates/amun-constitutional-semantics/src/lib.rs:27:#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)] pub enum EventFinality { Tentative, QuorumAccepted, Finalized, ReplayCertified }
1193:crates/amun-constitutional-semantics/src/lib.rs:29:    pub fn is_replay_safe(&self) -> bool { matches!(self, EventFinality::Finalized | EventFinality::ReplayCertified) }
1194:crates/amun-constitutional-semantics/src/lib.rs:30:    pub fn is_immutable(&self) -> bool { matches!(self, EventFinality::Finalized | EventFinality::ReplayCertified) }
1198:crates/amun-constitutional-semantics/src/lib.rs:36:    pub fn with_replay_certification(mut self, at_sequence: u64) -> Self { self.finality = EventFinality::ReplayCertified; self.finalized_at_sequence = Some(at_sequence); self }
1200:crates/amun-constitutional-semantics/src/lib.rs:48:#[derive(Debug, Clone, PartialEq, Eq)] pub struct NormalizedWitness { pub domain: ReplayDomain, pub sequence: u64, pub witness_hash: [u8; 32] }
1203:crates/amun-constitutional-semantics/src/lib.rs:51:    pub fn normalize(witnesses: &[(ReplayDomain, u64, [u8; 32])]) -> Self {
1205:crates/amun-constitutional-semantics/src/lib.rs:58:    pub fn verify_normalization(&self, witnesses: &[(ReplayDomain, u64, [u8; 32])]) -> bool { Self::normalize(witnesses).normalization_root == self.normalization_root }
1206:crates/amun-constitutional-semantics/src/lib.rs:61:// ─── Replay Policy ─────────────────────────────────────────
1207:crates/amun-constitutional-semantics/src/lib.rs:62:#[derive(Debug, Clone, PartialEq, Eq)] pub struct ReplayPolicy { pub authority: EventAuthority, pub replay_required: bool, pub divergence_is_violation: bool, pub contributes_to_causality: bool, pub can_be_checkpoint: bool, pub requires_certification: bool }
1208:crates/amun-constitutional-semantics/src/lib.rs:63:impl ReplayPolicy {
1209:crates/amun-constitutional-semantics/src/lib.rs:6:use amun_replay_semantics::{ReplayDomain, ReplayBoundary, ReplayFailure};
1210:crates/amun-constitutional-semantics/src/lib.rs:99:    pub fn transcript_continuity(events: &[TranscriptEntry]) -> Result<ContinuityResult, ReplayFailure> {
1211:crates/amun-constitutional-state/src/lib.rs:197:pub struct ReplayCertificate {
1212:crates/amun-constitutional-state/src/lib.rs:223:    ) -> ReplayCertificate {
1213:crates/amun-constitutional-state/src/lib.rs:224:        ReplayCertificate {
1214:crates/amun-constitutional-state/src/lib.rs:234:impl ReplayCertificate {
1215:crates/amun-constitutional-state/src/lib.rs:277:impl ReplayCertificate {
1216:crates/amun-constitutional-state/src/lib.rs:357:    /// Build a Merkle root from multiple ReplayCertificates.
1217:crates/amun-constitutional-state/src/lib.rs:359:    pub fn certificate_merkle_root(certificates: &[ReplayCertificate]) -> [u8; 32] {
1218:crates/amun-constitutional-state/src/lib.rs:404:/// A Merkle proof that a specific ReplayCertificate is included
1219:crates/amun-constitutional-state/src/lib.rs:418:        certificates: &[ReplayCertificate],
1220:crates/amun-constitutional-state/src/lib.rs:82:    /// Replay a journal to reconstruct the state.
1221:crates/amun-constitutional/src/architectural_invariants.rs:103:/// INVARIANT 7: Replay-Derived State Identity
1223:crates/amun-constitutional/src/architectural_invariants.rs:24:/// A ReplayCertificate attests replay ADMISSIBILITY, not
1279:crates/amun-constitutional/src/certificate_scope.rs:174:    fn make_scope(start: u64, end: u64, outcome: ReplayOutcome) -> CertificateScope {
1280:crates/amun-constitutional/src/certificate_scope.rs:188:        let s1 = make_scope(0, 99, ReplayOutcome::Admitted);
1281:crates/amun-constitutional/src/certificate_scope.rs:189:        let s2 = make_scope(0, 99, ReplayOutcome::Admitted);
1282:crates/amun-constitutional/src/certificate_scope.rs:196:        let narrow = make_scope(0, 49, ReplayOutcome::Admitted);
1283:crates/amun-constitutional/src/certificate_scope.rs:197:        let broad = make_scope(0, 99, ReplayOutcome::Admitted);
1284:crates/amun-constitutional/src/certificate_scope.rs:19:use crate::replay_outcome::ReplayOutcome;
1285:crates/amun-constitutional/src/certificate_scope.rs:203:        let narrow = make_scope(0, 49, ReplayOutcome::Admitted);
1286:crates/amun-constitutional/src/certificate_scope.rs:204:        let broad = make_scope(0, 99, ReplayOutcome::Divergent);
1287:crates/amun-constitutional/src/certificate_scope.rs:213:        let s1 = make_scope(0, 99, ReplayOutcome::Admitted);
1288:crates/amun-constitutional/src/certificate_scope.rs:214:        let mut s2 = make_scope(0, 99, ReplayOutcome::Admitted);
1289:crates/amun-constitutional/src/certificate_scope.rs:222:        let s1 = make_scope(0, 50, ReplayOutcome::Admitted);
1290:crates/amun-constitutional/src/certificate_scope.rs:223:        let s2 = make_scope(25, 75, ReplayOutcome::Divergent);
1291:crates/amun-constitutional/src/certificate_scope.rs:229:        let scope = make_scope(10, 20, ReplayOutcome::Admitted);
1292:crates/amun-constitutional/src/certificate_scope.rs:240:            make_scope(0, 99, ReplayOutcome::Admitted).span_length(),
1293:crates/amun-constitutional/src/certificate_scope.rs:244:            make_scope(10, 20, ReplayOutcome::Admitted).span_length(),
1294:crates/amun-constitutional/src/certificate_scope.rs:251:        let parent = make_scope(0, 99, ReplayOutcome::Admitted);
1295:crates/amun-constitutional/src/certificate_scope.rs:252:        let child = make_scope(0, 49, ReplayOutcome::Admitted);
1296:crates/amun-constitutional/src/certificate_scope.rs:258:        let parent = make_scope(0, 99, ReplayOutcome::Admitted);
1297:crates/amun-constitutional/src/certificate_scope.rs:259:        let mut child = make_scope(0, 49, ReplayOutcome::Admitted);
1298:crates/amun-constitutional/src/certificate_scope.rs:266:        let s1 = make_scope(0, 99, ReplayOutcome::Admitted);
1299:crates/amun-constitutional/src/certificate_scope.rs:267:        let s2 = make_scope(0, 99, ReplayOutcome::Admitted);
1300:crates/amun-constitutional/src/certificate_scope.rs:273:        let s1 = make_scope(0, 99, ReplayOutcome::Admitted);
1301:crates/amun-constitutional/src/certificate_scope.rs:274:        let s2 = make_scope(0, 49, ReplayOutcome::Admitted);
1302:crates/amun-constitutional/src/certificate_scope.rs:59:    /// Replay revision this certificate was issued under.
1303:crates/amun-constitutional/src/certificate_scope.rs:66:    pub outcome: ReplayOutcome,
1353:crates/amun-constitutional/src/divergence_point.rs:222:            DivergenceType::ReplayError,
1354:crates/amun-constitutional/src/divergence_type.rs:20:    ReplayError = 0x02,
1355:crates/amun-constitutional/src/divergence_type.rs:60:            DivergenceType::ReplayError | DivergenceType::BoundaryViolation
1356:crates/amun-constitutional/src/divergence_type.rs:81:        assert!(DivergenceType::ReplayError.is_error());
1357:crates/amun-constitutional/src/divergence_type.rs:88:        assert!(!DivergenceType::ReplayError.is_ambiguous());
1358:crates/amun-constitutional/src/execution_journal.rs:247:                "Replay revision mismatch",
1360:crates/amun-constitutional/src/execution_limits.rs:44:    pub replay: ReplayLimits,
1362:crates/amun-constitutional/src/execution_limits.rs:63:            replay: ReplayLimits {
1364:crates/amun-constitutional/src/execution_limits.rs:7:pub struct ReplayLimits {
1365:crates/amun-constitutional/src/execution_receipt.rs:15://!   The certificate_hash links to a ReplayCertificate that attests
1366:crates/amun-constitutional/src/execution_receipt.rs:180:        replay_outcome: ReplayOutcome,
1367:crates/amun-constitutional/src/execution_receipt.rs:236:            ReplayOutcome::Admitted,
1368:crates/amun-constitutional/src/execution_receipt.rs:26:use crate::replay_outcome::ReplayOutcome;
1369:crates/amun-constitutional/src/execution_receipt.rs:278:            ReplayOutcome::ConstitutionalFailure,
1370:crates/amun-constitutional/src/execution_receipt.rs:292:            ReplayOutcome::ConstitutionalFailure,
1371:crates/amun-constitutional/src/execution_receipt.rs:42:/// `certificate_hash` links to a ReplayCertificate. The certificate model
1373:crates/amun-constitutional/src/execution_receipt.rs:80:    pub replay_outcome: ReplayOutcome,
1376:crates/amun-constitutional/src/hash_domains.rs:39:/// Domain: ReplayOutcome objects
1377:crates/amun-constitutional/src/hash_domains.rs:45:/// Domain: ReplayCertificate objects
1378:crates/amun-constitutional/src/kernel_types.rs:14:pub struct ReplayFailure {
1379:crates/amun-constitutional/src/kernel_types.rs:16:    pub domain: ReplayDomain,
1380:crates/amun-constitutional/src/kernel_types.rs:21:pub struct ReplayPolicy {
1381:crates/amun-constitutional/src/kernel_types.rs:27:impl Default for ReplayPolicy {
1382:crates/amun-constitutional/src/kernel_types.rs:41:    pub domain: ReplayDomain,
1383:crates/amun-constitutional/src/kernel_types.rs:6:pub enum ReplayDomain {
1385:crates/amun-constitutional/src/lib.rs:121:pub use replay_certificate::ReplayCertificate;
1386:crates/amun-constitutional/src/lib.rs:122:pub use replay_outcome::ReplayOutcome;
1390:crates/amun-constitutional/src/replay_certificate.rs:130:impl ReplayCertificate {
1391:crates/amun-constitutional/src/replay_certificate.rs:139:        outcome: crate::replay_outcome::ReplayOutcome,
1392:crates/amun-constitutional/src/replay_certificate.rs:170:    pub fn outcome(&self) -> crate::replay_outcome::ReplayOutcome {
1393:crates/amun-constitutional/src/replay_certificate.rs:179:        parent: &ReplayCertificate,
1394:crates/amun-constitutional/src/replay_certificate.rs:18:pub struct ReplayCertificate {
1395:crates/amun-constitutional/src/replay_certificate.rs:197:    use crate::replay_outcome::ReplayOutcome;
1396:crates/amun-constitutional/src/replay_certificate.rs:1://! ReplayCertificate — cryptographically scoped admissibility envelope.
1397:crates/amun-constitutional/src/replay_certificate.rs:203:        outcome: ReplayOutcome,
1398:crates/amun-constitutional/src/replay_certificate.rs:205:    ) -> ReplayCertificate {
1399:crates/amun-constitutional/src/replay_certificate.rs:206:        ReplayCertificate::new(
1400:crates/amun-constitutional/src/replay_certificate.rs:213:        assert!(mc(1, 0, 99, ReplayOutcome::Admitted, None).verify().is_ok());
1401:crates/amun-constitutional/src/replay_certificate.rs:218:            mc(1, 0, 99, ReplayOutcome::Admitted, None).certificate_hash,
1402:crates/amun-constitutional/src/replay_certificate.rs:219:            mc(1, 0, 99, ReplayOutcome::Admitted, None).certificate_hash
1403:crates/amun-constitutional/src/replay_certificate.rs:225:            mc(1, 0, 49, ReplayOutcome::Admitted, None).certificate_hash,
1404:crates/amun-constitutional/src/replay_certificate.rs:226:            mc(1, 0, 99, ReplayOutcome::Admitted, None).certificate_hash
1405:crates/amun-constitutional/src/replay_certificate.rs:232:            mc(1, 0, 99, ReplayOutcome::Admitted, None).certificate_hash,
1406:crates/amun-constitutional/src/replay_certificate.rs:233:            mc(1, 0, 99, ReplayOutcome::Divergent, None).certificate_hash
1407:crates/amun-constitutional/src/replay_certificate.rs:238:        let p = mc(1, 0, 99, ReplayOutcome::Admitted, None);
1408:crates/amun-constitutional/src/replay_certificate.rs:239:        let c = mc(2, 0, 49, ReplayOutcome::Admitted, Some(p.certificate_hash));
1409:crates/amun-constitutional/src/replay_certificate.rs:246:        let p = mc(1, 0, 99, ReplayOutcome::Admitted, None);
1410:crates/amun-constitutional/src/replay_certificate.rs:247:        let c = ReplayCertificate::new(
1411:crates/amun-constitutional/src/replay_certificate.rs:255:            ReplayOutcome::Admitted,
1412:crates/amun-constitutional/src/replay_certificate.rs:265:        let mut c = mc(1, 100, 50, ReplayOutcome::Admitted, None);
1413:crates/amun-constitutional/src/replay_certificate.rs:271:        let mut c = mc(1, 0, 99, ReplayOutcome::Admitted, None);
1414:crates/amun-constitutional/src/replay_certificate.rs:32:impl ConstitutionalIdentity for ReplayCertificate {
1415:crates/amun-constitutional/src/replay_certificate.rs:47:impl ConstitutionalObject for ReplayCertificate {
1416:crates/amun-constitutional/src/replay_outcome.rs:15:pub enum ReplayOutcome {
1417:crates/amun-constitutional/src/replay_outcome.rs:1://! ReplayOutcome — constitutional admissibility result.
1418:crates/amun-constitutional/src/replay_outcome.rs:29:impl ReplayOutcome {
1419:crates/amun-constitutional/src/replay_outcome.rs:40:        matches!(self, ReplayOutcome::Admitted)
1420:crates/amun-constitutional/src/replay_outcome.rs:56:            ReplayOutcome::Admitted.outcome_hash(),
1421:crates/amun-constitutional/src/replay_outcome.rs:57:            ReplayOutcome::Admitted.outcome_hash()
1422:crates/amun-constitutional/src/replay_outcome.rs:64:            ReplayOutcome::Admitted.outcome_hash(),
1423:crates/amun-constitutional/src/replay_outcome.rs:65:            ReplayOutcome::Divergent.outcome_hash()
1424:crates/amun-constitutional/src/replay_outcome.rs:71:        assert!(ReplayOutcome::Admitted.is_admitted());
1425:crates/amun-constitutional/src/replay_outcome.rs:72:        assert!(!ReplayOutcome::Divergent.is_admitted());
1426:crates/amun-constitutional/src/replay_outcome.rs:73:        assert!(!ReplayOutcome::BoundaryViolation.is_admitted());
1427:crates/amun-constitutional/src/replay_outcome.rs:74:        assert!(!ReplayOutcome::ConstitutionalFailure.is_admitted());
1428:crates/amun-constitutional/src/replay_outcome.rs:79:        assert!(!ReplayOutcome::Admitted.is_failure());
1429:crates/amun-constitutional/src/replay_outcome.rs:80:        assert!(ReplayOutcome::Divergent.is_failure());
1430:crates/amun-constitutional/src/replay_outcome.rs:81:        assert!(ReplayOutcome::BoundaryViolation.is_failure());
1431:crates/amun-constitutional/src/replay_outcome.rs:82:        assert!(ReplayOutcome::ConstitutionalFailure.is_failure());
1433:crates/amun-constitutional/src/schema_registry.rs:69:    /// ReplayCertificate
1436:crates/amun-constitutional/src/snapshot.rs:11://!   Replay → StateAnchor → Snapshot
1437:crates/amun-constitutional/src/snapshot_scope.rs:79:    /// Replay revision active for this scope.
1438:crates/amun-constitutional/src/state_anchor.rs:13://! INVARIANT (Replay-Derived State Identity):
1439:crates/amun-constitutional/src/state_anchor_scope.rs:50:    /// Replay revision active for this scope.
1519:crates/amun-crypto-hardening/src/anti_replay.rs:11:impl AntiReplayGuard {
1520:crates/amun-crypto-hardening/src/anti_replay.rs:54:        let mut guard = AntiReplayGuard::new(100);
1521:crates/amun-crypto-hardening/src/anti_replay.rs:60:        let mut guard = AntiReplayGuard::new(100);
1522:crates/amun-crypto-hardening/src/anti_replay.rs:68:        let mut guard = AntiReplayGuard::new(10);
1523:crates/amun-crypto-hardening/src/anti_replay.rs:6:pub struct AntiReplayGuard {
1575:crates/amun-entropy-transcript/src/source.rs:10:    Replay,
1627:crates/amun-evidence-finality/src/evidence_finality.rs:184:            let replay = ReplayVerifier::replay(&proof, program, &mut fresh_reg, &[]);
1628:crates/amun-evidence-finality/src/evidence_finality.rs:186:            let verified = matches!(replay, ReplayResult::Match { .. });
1652:crates/amun-evidence-finality/src/evidence_finality.rs:7:use amun_replay_verifier::replay_verifier::{ReplayResult, ReplayVerifier};
1673:crates/amun-evidence-root/src/lib.rs:8:/// - Replay certificate (replay verification proof)
1694:crates/amun-evolution/src/certificate.rs:3:    ContinuityClass, GovernanceGuarantee, ProofGuarantee, ReplayGuarantee, SnapshotGuarantee,
1695:crates/amun-evolution/src/certificate.rs:67:    pub replay_guarantee: ReplayGuarantee,
1696:crates/amun-evolution/src/certificate.rs:91:        replay: ReplayGuarantee,
1697:crates/amun-evolution/src/executor.rs:28:    ReplayBreak,
1698:crates/amun-evolution/src/validator.rs:22:        proof.replay_guarantee.rank() >= ReplayGuarantee::Deterministic.rank()
1699:crates/amun-evolution/src/validator.rs:3:    EvolutionProof, GovernanceGuarantee, ReplayGuarantee, SnapshotGuarantee,
1703:crates/amun-experimental-framework/src/main.rs:214:            ReplayVerifier::replay(&proof, &program, &mut fresh, &[]);
1704:crates/amun-experimental-framework/src/main.rs:229:// ── Experiment 2: Replay vs Execution (all workloads) ───────
1705:crates/amun-experimental-framework/src/main.rs:231:    println!("\n=== Experiment 2: Replay vs Execution ===");
1707:crates/amun-experimental-framework/src/main.rs:311:            ReplayVerifier::replay(&proof, program, &mut fresh, &[]);
1710:crates/amun-experimental-framework/src/main.rs:371:                    ReplayVerifier::replay(&transition_proof, &program, &mut fresh, &[]);
1717:crates/amun-experimental-framework/src/main.rs:6:use amun_replay_verifier::replay_verifier::ReplayVerifier;
1736:crates/amun-explorer-api/src/services/constitutional_service.rs:68:                evidence_type: "ReplayEvidence".into(),
1752:crates/amun-failure/src/taxonomy.rs:17:    ReplayViolation = 0x3002,
1754:crates/amun-failure/src/taxonomy.rs:55:            | Self::ReplayViolation
1756:crates/amun-failure/src/tests.rs:36:    assert!(!ConstitutionalFault::ReplayViolation.should_halt());
1788:crates/amun-host/src/lib.rs:9:pub use replay::ReplayGuard;
1789:crates/amun-host/src/replay.rs:1:pub struct ReplayGuard {
1790:crates/amun-host/src/replay.rs:6:impl ReplayGuard {
1898:crates/amun-lineage/src/compatibility.rs:19:            CompatibilityClass::ReplayCompatible => 0x04,
1899:crates/amun-lineage/src/compatibility.rs:32:            CompatibilityClass::ReplayCompatible => 4,
1900:crates/amun-lineage/src/compatibility.rs:7:    ReplayCompatible,
1901:crates/amun-lineage/src/lib.rs:13:    ReplayGuarantee, SnapshotGuarantee,
1902:crates/amun-lineage/src/record.rs:14:impl ReplayGuarantee {
1903:crates/amun-lineage/src/record.rs:169:    pub replay_guarantee: ReplayGuarantee,
1904:crates/amun-lineage/src/record.rs:17:            ReplayGuarantee::Exact => 0x03,
1905:crates/amun-lineage/src/record.rs:181:        replay: ReplayGuarantee,
1906:crates/amun-lineage/src/record.rs:18:            ReplayGuarantee::Deterministic => 0x02,
1907:crates/amun-lineage/src/record.rs:19:            ReplayGuarantee::Partial => 0x01,
1908:crates/amun-lineage/src/record.rs:20:            ReplayGuarantee::Unsupported => 0x00,
1909:crates/amun-lineage/src/record.rs:7:pub enum ReplayGuarantee {
1910:crates/amun-live-cluster/src/validator.rs:14:        ConstitutionalEvidenceRecord, DoubleSpendEvidence, GovernanceEvidence, ReplayEvidence,
1914:crates/amun-live-cluster/src/validator.rs:594:                        // N126.3: Replay determinism from ExecutionEngine
1921:crates/amun-live-cluster/src/validator.rs:662:                        let rep_ev = ReplayEvidence::new(cert.state_root, history_root);
1925:crates/amun-live-cluster/src/validator.rs:747:                            // N110.4c.1: Replay protection
1945:crates/amun-network-constitution/src/anti_replay.rs:47:impl Default for AntiReplayGuard {
1946:crates/amun-network-constitution/src/anti_replay.rs:4:pub struct AntiReplayGuard {
1947:crates/amun-network-constitution/src/anti_replay.rs:9:impl AntiReplayGuard {
1948:crates/amun-network-constitution/src/lib.rs:4:pub use anti_replay::AntiReplayGuard;
1959:crates/amun-networking/src/quarantine.rs:32:    ReplayVerified,
1960:crates/amun-networking/src/risk.rs:11:    ReplayInstability { divergence_count: u64 },
1961:crates/amun-networking/src/risk.rs:61:            ConstitutionalRisk::ReplayInstability { .. } => {
1962:crates/amun-networking/src/sovereignty.rs:16:/// Multi-phase: Identity -> Physics -> Replay -> Lineage -> Temporal -> Sync
1963:crates/amun-networking/src/sovereignty.rs:36:    ReplayCompatibilityCheck,
1964:crates/amun-networking/src/sovereignty.rs:77:                HandshakePhase::ReplayCompatibilityCheck
2018:crates/amun-nft-evidence/src/lib.rs:24:    Law4ReplayDetected,
2026:crates/amun-nft-evidence/src/lib.rs:67:    /// Law 4: Replay Protection — check nonce or timestamp ordering
2027:crates/amun-nft-evidence/src/lib.rs:73:            return Err(CekError::Law4ReplayDetected);
2033:crates/amun-nft-evidence/tests/n131_evidence_tests.rs:104:    assert_eq!(result, Err(CekError::Law4ReplayDetected));
2186:crates/amun-node/src/bin/test_replay_determinism.rs:52:    // Phase 2: Replay on 4 independent stores
2187:crates/amun-node/src/bin/test_replay_determinism.rs:72:        println!("Replay {} root: {}", v, hex::encode(replay_root));
2188:crates/amun-node/src/bin/test_replay_determinism.rs:79:        println!("\nPASS: Replay determinism verified across all validators");
2189:crates/amun-node/src/bin/test_replay_determinism.rs:82:        println!("\nFAIL: Replay determinism violation");
2190:crates/amun-node/src/bin/test_replay_determinism.rs:85:                "Replay {} root: {} (match: {})",
2192:crates/amun-node/src/bin/test_replay_stress.rs:132:            .expect("Replay execution failed");
2193:crates/amun-node/src/bin/test_replay_stress.rs:135:                .expect("Replay advance failed");
2194:crates/amun-node/src/bin/test_replay_stress.rs:138:        println!("Replay {} final root: {}", v, hex::encode(replay_root));
2195:crates/amun-node/src/bin/test_replay_stress.rs:145:        println!("\nPASS: Replay stress test passed ({} blocks)", block_count);
2196:crates/amun-node/src/bin/test_replay_stress.rs:148:        println!("\nFAIL: Replay divergence detected");
2197:crates/amun-node/src/bin/test_replay_stress.rs:151:                "Replay {} root: {} (match: {})",
2200:crates/amun-node/src/bin/test_replay_stress.rs:89:    // Phase 2: Replay on 4 independent stores from scratch
2396:crates/amun-recovery/src/lib.rs:116:        let record = ReplayRecord {
2397:crates/amun-recovery/src/lib.rs:173:            let record = ReplayRecord {
2398:crates/amun-recovery/src/lib.rs:23:    replay: ReplayStore,
2399:crates/amun-recovery/src/lib.rs:32:            replay: ReplayStore::new(&format!("{}/replay.json", data_dir)),
2400:crates/amun-recovery/src/lib.rs:3:use amun_replay_store::{ReplayRecord, ReplayStore};
2401:crates/amun-recovery/src/lib.rs:41:        record: &ReplayRecord,
2402:crates/amun-recovery/src/lib.rs:69:            return Err("Replay chain verification failed".into());
2403:crates/amun-replay-cert/src/certificate.rs:19:impl ReplayCertificate {
2404:crates/amun-replay-cert/src/certificate.rs:6:pub struct ReplayCertificate {
2405:crates/amun-replay-cert/src/certifier.rs:14:impl ReplayCertifier {
2406:crates/amun-replay-cert/src/certifier.rs:18:            transcript: ReplayTranscript::new(),
2407:crates/amun-replay-cert/src/certifier.rs:3:use crate::transcript::ReplayTranscript;
2408:crates/amun-replay-cert/src/certifier.rs:7:pub struct ReplayCertifier {
2409:crates/amun-replay-cert/src/certifier.rs:9:    transcript: ReplayTranscript,
2410:crates/amun-replay-cert/src/lib.rs:10:pub use transcript::ReplayTranscript;
2411:crates/amun-replay-cert/src/lib.rs:12:impl Default for ReplayTranscript {
2412:crates/amun-replay-cert/src/lib.rs:7:pub use certifier::ReplayCertifier;
2413:crates/amun-replay-cert/src/transcript.rs:18:impl ReplayTranscript {
2414:crates/amun-replay-cert/src/transcript.rs:4:pub struct ReplayTranscript {
2415:crates/amun-replay-cert/src/verifier.rs:1:use crate::certificate::ReplayCertificate;
2416:crates/amun-replay-cert/src/verifier.rs:5:pub fn verify_certificate(cert: &ReplayCertificate) -> Result<(), &'static str> {
2417:crates/amun-replay-consensus/src/replay_backed_consensus.rs:116:        block: &ReplayVerifiedBlock,
2418:crates/amun-replay-consensus/src/replay_backed_consensus.rs:119:    ) -> Result<ReplayBackedFinalityCertificate, String> {
2419:crates/amun-replay-consensus/src/replay_backed_consensus.rs:11:    ReplayBackedFinalityCertificate, ReplayBackedQC, ReplayVerificationRecord, ReplayVerifiedBlock,
2420:crates/amun-replay-consensus/src/replay_backed_consensus.rs:124:        let mut qc = ReplayBackedQC::for_block(block, quorum_size);
2421:crates/amun-replay-consensus/src/replay_backed_consensus.rs:137:        Ok(ReplayBackedFinalityCertificate::issue(block, qc))
2423:crates/amun-replay-consensus/src/replay_backed_consensus.rs:14:/// Replay-Backed Consensus Engine.
2425:crates/amun-replay-consensus/src/replay_backed_consensus.rs:178:        let block = ReplayBackedConsensus::execute_and_replay(
2426:crates/amun-replay-consensus/src/replay_backed_consensus.rs:17:pub struct ReplayBackedConsensus;
2428:crates/amun-replay-consensus/src/replay_backed_consensus.rs:19:impl ReplayBackedConsensus {
2429:crates/amun-replay-consensus/src/replay_backed_consensus.rs:204:        let block = ReplayBackedConsensus::execute_and_replay(
2430:crates/amun-replay-consensus/src/replay_backed_consensus.rs:213:        let cert = ReplayBackedConsensus::form_consensus(&block, 5, sigs).unwrap();
2432:crates/amun-replay-consensus/src/replay_backed_consensus.rs:230:        let mut block = ReplayBackedConsensus::execute_and_replay(
2434:crates/amun-replay-consensus/src/replay_backed_consensus.rs:242:        assert!(ReplayBackedConsensus::form_consensus(&block, 5, sigs).is_err());
2436:crates/amun-replay-consensus/src/replay_backed_consensus.rs:258:        let block = ReplayBackedConsensus::execute_and_replay(
2437:crates/amun-replay-consensus/src/replay_backed_consensus.rs:267:        let cert1 = ReplayBackedConsensus::form_consensus(&block, 5, sigs.clone()).unwrap();
2438:crates/amun-replay-consensus/src/replay_backed_consensus.rs:268:        let cert2 = ReplayBackedConsensus::form_consensus(&block, 5, sigs).unwrap();
2440:crates/amun-replay-consensus/src/replay_backed_consensus.rs:27:    ) -> Result<ReplayVerifiedBlock, String> {
2441:crates/amun-replay-consensus/src/replay_backed_consensus.rs:285:        let block = ReplayBackedConsensus::execute_and_replay(
2442:crates/amun-replay-consensus/src/replay_backed_consensus.rs:54:            // Replay verification
2444:crates/amun-replay-consensus/src/replay_backed_consensus.rs:56:            let replay = ReplayVerifier::replay(&proof, program, &mut fresh_reg, &[]);
2445:crates/amun-replay-consensus/src/replay_backed_consensus.rs:59:                ReplayResult::Match {
2446:crates/amun-replay-consensus/src/replay_backed_consensus.rs:5:use amun_replay_verifier::replay_verifier::{ReplayResult, ReplayVerifier};
2447:crates/amun-replay-consensus/src/replay_backed_consensus.rs:62:                } => ReplayVerificationRecord {
2448:crates/amun-replay-consensus/src/replay_backed_consensus.rs:69:                _ => ReplayVerificationRecord {
2451:crates/amun-replay-consensus/src/replay_backed_consensus.rs:86:        let mut block = ReplayVerifiedBlock {
2452:crates/amun-replay-consensus/src/replay_backed_types.rs:14:impl ReplayVerificationRecord {
2454:crates/amun-replay-consensus/src/replay_backed_types.rs:23:pub struct ReplayVerifiedBlock {
2456:crates/amun-replay-consensus/src/replay_backed_types.rs:30:    pub replay_verifications: Vec<ReplayVerificationRecord>,
2457:crates/amun-replay-consensus/src/replay_backed_types.rs:34:impl ReplayVerifiedBlock {
2458:crates/amun-replay-consensus/src/replay_backed_types.rs:52:pub struct ReplayBackedQC {
2459:crates/amun-replay-consensus/src/replay_backed_types.rs:64:impl ReplayBackedQC {
2460:crates/amun-replay-consensus/src/replay_backed_types.rs:69:    pub fn for_block(block: &ReplayVerifiedBlock, threshold: usize) -> Self {
2461:crates/amun-replay-consensus/src/replay_backed_types.rs:6:pub struct ReplayVerificationRecord {
2462:crates/amun-replay-consensus/src/replay_backed_types.rs:86:pub struct ReplayBackedFinalityCertificate {
2463:crates/amun-replay-consensus/src/replay_backed_types.rs:92:    pub qc: ReplayBackedQC,
2464:crates/amun-replay-consensus/src/replay_backed_types.rs:96:impl ReplayBackedFinalityCertificate {
2465:crates/amun-replay-consensus/src/replay_backed_types.rs:97:    pub fn issue(block: &ReplayVerifiedBlock, qc: ReplayBackedQC) -> Self {
2497:crates/amun-replay-engine/src/deterministic.rs:3:use crate::errors::ReplayFailure;
2498:crates/amun-replay-engine/src/deterministic.rs:56:    ) -> Result<ConstitutionalStep, ReplayFailure> {
2499:crates/amun-replay-engine/src/deterministic.rs:58:            return Err(ReplayFailure::OrderingViolation {
2500:crates/amun-replay-engine/src/deterministic.rs:81:    ) -> Result<ExecutionTrace, ReplayFailure> {
2501:crates/amun-replay-engine/src/equivalence.rs:105:    use amun_constitutional::ReplayDomain;
2502:crates/amun-replay-engine/src/equivalence.rs:111:            domain: ReplayDomain::Canonical,
2503:crates/amun-replay-engine/src/equivalence.rs:117:        let state = ReplayState::new([0; 32]);
2504:crates/amun-replay-engine/src/equivalence.rs:126:        let state = ReplayState::new([0; 32]);
2505:crates/amun-replay-engine/src/equivalence.rs:15:pub struct ConstitutionalReplayResult {
2506:crates/amun-replay-engine/src/equivalence.rs:2:use crate::errors::ReplayFailure;
2507:crates/amun-replay-engine/src/equivalence.rs:3:use crate::state::ReplayState;
2508:crates/amun-replay-engine/src/equivalence.rs:42:impl ConstitutionalReplayResult {
2509:crates/amun-replay-engine/src/equivalence.rs:56:        expected_state: &ReplayState,
2510:crates/amun-replay-engine/src/equivalence.rs:57:    ) -> Result<EquivalenceProof, ReplayFailure> {
2511:crates/amun-replay-engine/src/equivalence.rs:68:        initial_state: &ReplayState,
2512:crates/amun-replay-engine/src/equivalence.rs:71:    ) -> Result<EquivalenceProof, ReplayFailure> {
2513:crates/amun-replay-engine/src/equivalence.rs:80:        // if we had applied the same entries to the ReplayState
2514:crates/amun-replay-engine/src/equivalence.rs:91:        // For self-verification: replay the entries through ReplayState
2515:crates/amun-replay-engine/src/errors.rs:8:pub enum ReplayFailure {
2527:crates/amun-replay-engine/src/lib.rs:102:pub struct ReplayCursor {
2528:crates/amun-replay-engine/src/lib.rs:108:impl ReplayCursor {
2529:crates/amun-replay-engine/src/lib.rs:125:    ) -> Result<(), ReplayFailure> {
2530:crates/amun-replay-engine/src/lib.rs:143:    use amun_constitutional::ReplayDomain;
2531:crates/amun-replay-engine/src/lib.rs:149:            domain: ReplayDomain::Canonical,
2532:crates/amun-replay-engine/src/lib.rs:155:        let mut session = ReplaySession::new([0; 32], 0);
2533:crates/amun-replay-engine/src/lib.rs:164:        let mut session = ReplaySession::new([0; 32], 0);
2534:crates/amun-replay-engine/src/lib.rs:26:    AuthorityProof, CheckpointResult, ConstitutionalReplayResult, ContinuityProof,
2535:crates/amun-replay-engine/src/lib.rs:29:use errors::ReplayFailure;
2536:crates/amun-replay-engine/src/lib.rs:30:use state::ReplayState;
2537:crates/amun-replay-engine/src/lib.rs:39:// NOTE: ReplayState and ReplayFailure are imported via `use` above.
2538:crates/amun-replay-engine/src/lib.rs:42:// STRUCT: ReplaySession
2539:crates/amun-replay-engine/src/lib.rs:45:pub struct ReplaySession {
2540:crates/amun-replay-engine/src/lib.rs:46:    pub state: ReplayState,
2541:crates/amun-replay-engine/src/lib.rs:47:    pub cursor: ReplayCursor,
2542:crates/amun-replay-engine/src/lib.rs:50:impl ReplaySession {
2543:crates/amun-replay-engine/src/lib.rs:53:            state: ReplayState::new(initial_state_root),
2544:crates/amun-replay-engine/src/lib.rs:54:            cursor: ReplayCursor::new(start_sequence),
2545:crates/amun-replay-engine/src/lib.rs:61:    ) -> Result<ConstitutionalReplayResult, ReplayFailure> {
2546:crates/amun-replay-engine/src/lib.rs:69:        self.state = ReplayState::new(proof.trace.final_state_hash());
2547:crates/amun-replay-engine/src/lib.rs:77:        Ok(ConstitutionalReplayResult {
2548:crates/amun-replay-engine/src/lib.rs:98:// STRUCT: ReplayCursor
2558:crates/amun-replay-engine/src/state.rs:103:            domain: ReplayDomain::Canonical,
2559:crates/amun-replay-engine/src/state.rs:108:            domain: ReplayDomain::Canonical,
2560:crates/amun-replay-engine/src/state.rs:18:use crate::errors::ReplayFailure;
2561:crates/amun-replay-engine/src/state.rs:21:pub struct ReplayState {
2562:crates/amun-replay-engine/src/state.rs:24:    pub divergences: Vec<ReplayFailure>,
2563:crates/amun-replay-engine/src/state.rs:27:impl ReplayState {
2564:crates/amun-replay-engine/src/state.rs:44:    pub fn apply_entry(&self, entry: &TranscriptEntry) -> Result<Self, ReplayFailure> {
2565:crates/amun-replay-engine/src/state.rs:63:    use amun_constitutional::ReplayDomain;
2566:crates/amun-replay-engine/src/state.rs:67:        let state = ReplayState::new([0xAA; 32]);
2567:crates/amun-replay-engine/src/state.rs:73:            domain: ReplayDomain::Canonical,
2568:crates/amun-replay-engine/src/state.rs:83:        let state = ReplayState::new([0xAA; 32]);
2569:crates/amun-replay-engine/src/state.rs:87:            domain: ReplayDomain::Canonical,
2570:crates/amun-replay-engine/src/state.rs:99:        let state = ReplayState::new([0xAA; 32]);
2571:crates/amun-replay-engine/src/version.rs:1:/// Replay protocol version - FROZEN for protocol v1.
2584:crates/amun-replay-optimization/src/lib.rs:17:pub struct ReplayCache {
2585:crates/amun-replay-optimization/src/lib.rs:24:impl ReplayCache {
2586:crates/amun-replay-optimization/src/lib.rs:93:impl Default for ReplayCache {
2587:crates/amun-replay-optimization/tests/n163_replay_tests.rs:22:    let mut cache = ReplayCache::new();
2588:crates/amun-replay-optimization/tests/n163_replay_tests.rs:47:    let mut cache = ReplayCache::new();
2589:crates/amun-replay-optimization/tests/n163_replay_tests.rs:5:    let mut cache = ReplayCache::new();
2590:crates/amun-replay-optimization/tests/n163_replay_tests.rs:64:    let mut cache1 = ReplayCache::new();
2591:crates/amun-replay-optimization/tests/n163_replay_tests.rs:65:    let mut cache2 = ReplayCache::new();
2592:crates/amun-replay-semantics/src/lib.rs:12:pub struct ReplayEpoch { pub epoch_id: [u8; 32], pub start_sequence: u64, pub end_sequence: Option<u64>, pub replay_version: u32 }
2593:crates/amun-replay-semantics/src/lib.rs:13:impl ReplayEpoch {
2594:crates/amun-replay-semantics/src/lib.rs:19:pub struct ReplayBoundary { pub finalized_sequence: u64, pub boundary_chain_hash: [u8; 32], pub boundary_state_root: [u8; 32], pub epoch: ReplayEpoch, pub replay_version: u32 }
2595:crates/amun-replay-semantics/src/lib.rs:1://! Replay Semantics — formal constitutional model for replay.
2596:crates/amun-replay-semantics/src/lib.rs:20:impl ReplayBoundary { pub fn genesis(genesis_hash: [u8; 32]) -> Self { Self { finalized_sequence: 0, boundary_chain_hash: genesis_hash, boundary_state_root: genesis_hash, epoch: ReplayEpoch::new(genesis_hash, 0, 1), replay_version: 1 } } }
2597:crates/amun-replay-semantics/src/lib.rs:23:pub struct ReplayCertificate {
2598:crates/amun-replay-semantics/src/lib.rs:24:    pub domain: ReplayDomain, pub epoch: ReplayEpoch,
2599:crates/amun-replay-semantics/src/lib.rs:27:    pub boundary: ReplayBoundary, pub event_count: u64, pub replay_version: u32,
2600:crates/amun-replay-semantics/src/lib.rs:2://! Layer 0.75 — Constitutional Replay Theory.
2601:crates/amun-replay-semantics/src/lib.rs:30:impl ReplayCertificate {
2602:crates/amun-replay-semantics/src/lib.rs:32:    pub fn new(domain: ReplayDomain, epoch: ReplayEpoch, transcript_root: [u8; 32], state_root: [u8; 32], receipt_root: [u8; 32], ordering_root: [u8; 32], boundary: ReplayBoundary, event_count: u64, replay_version: u32) -> Self {
2603:crates/amun-replay-semantics/src/lib.rs:46:#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum ReplayEquivalence { Strict, Semantic, EpochBounded(ReplayEpoch) }
2604:crates/amun-replay-semantics/src/lib.rs:47:impl ReplayEquivalence {
2605:crates/amun-replay-semantics/src/lib.rs:48:    pub fn verify(&self, a: &ReplayCertificate, b: &ReplayCertificate) -> bool {
2606:crates/amun-replay-semantics/src/lib.rs:49:        match self { ReplayEquivalence::Strict => a.certificate_hash == b.certificate_hash, ReplayEquivalence::Semantic => ReplayCertificate::prove_equivalence(a, b), ReplayEquivalence::EpochBounded(ep) => a.epoch.epoch_id == ep.epoch_id && b.epoch.epoch_id == ep.epoch_id && ReplayCertificate::prove_equivalence(a, b) }
2607:crates/amun-replay-semantics/src/lib.rs:54:pub enum ReplayFailure {
2608:crates/amun-replay-semantics/src/lib.rs:58:    EpochBoundaryViolation { expected_epoch: Box<ReplayEpoch>, actual_epoch: Box<ReplayEpoch> },
2609:crates/amun-replay-semantics/src/lib.rs:60:    BoundaryViolation { expected_boundary: Box<ReplayBoundary>, actual_boundary: Box<ReplayBoundary> },
2610:crates/amun-replay-semantics/src/lib.rs:61:    ReplayResourceExhaustion { limit: usize, attempted: usize },
2611:crates/amun-replay-semantics/src/lib.rs:64:#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum ReplayAuthority { SelfVerification, ValidatorQuorum { required_signatures: u64 }, ConstitutionalCourt, PublicVerification }
2612:crates/amun-replay-semantics/src/lib.rs:67:pub struct ReplayWitness { pub start_sequence: u64, pub end_sequence: u64, pub pre_state_root: [u8; 32], pub post_state_root: [u8; 32], pub transcript_fragment_hash: [u8; 32], pub witness_data: Vec<u8> }
2613:crates/amun-replay-semantics/src/lib.rs:70:pub struct ReplayCheckpoint { pub sequence: u64, pub state_root: [u8; 32], pub transcript_chain_hash: [u8; 32], pub certificate: ReplayCertificate }
2614:crates/amun-replay-semantics/src/lib.rs:71:impl ReplayCheckpoint { pub fn verify(&self, boundary: &ReplayBoundary) -> bool { self.sequence >= boundary.finalized_sequence && self.certificate.verify() && self.certificate.boundary.boundary_chain_hash == boundary.boundary_chain_hash } }
2615:crates/amun-replay-semantics/src/lib.rs:75:    pub fn replay_determinism(a: &ReplayCertificate, b: &ReplayCertificate) -> Result<(), ReplayFailure> {
2616:crates/amun-replay-semantics/src/lib.rs:76:        if a.transcript_root != b.transcript_root { return Err(ReplayFailure::TranscriptMismatch { expected_root: a.transcript_root, actual_root: b.transcript_root }); }
2617:crates/amun-replay-semantics/src/lib.rs:77:        if a.state_root != b.state_root { return Err(ReplayFailure::StateDivergence { expected_root: a.state_root, actual_root: b.state_root }); }
2618:crates/amun-replay-semantics/src/lib.rs:85:    fn te() -> ReplayEpoch { ReplayEpoch::new([0xBB; 32], 0, 1) }
2619:crates/amun-replay-semantics/src/lib.rs:86:    fn tb() -> ReplayBoundary { ReplayBoundary::genesis([0xAA; 32]) }
2620:crates/amun-replay-semantics/src/lib.rs:87:    #[test] fn test_cert_self_verifying() { assert!(ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1).verify()); }
2621:crates/amun-replay-semantics/src/lib.rs:88:    #[test] fn test_cert_tamper_detected() { let mut c = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); c.state_root = [0xFF;32]; assert!(!c.verify()); }
2622:crates/amun-replay-semantics/src/lib.rs:89:    #[test] fn test_equivalence_strict() { let c = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); assert!(ReplayEquivalence::Strict.verify(&c, &c)); }
2623:crates/amun-replay-semantics/src/lib.rs:90:    #[test] fn test_law_determinism_divergence() { let a = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0x02;32], [0x03;32], [0x04;32], tb(), 100, 1); let b = ReplayCertificate::new(ReplayDomain::Consensus, te(), [0x01;32], [0xFF;32], [0x03;32], [0x04;32], tb(), 100, 1); assert!(laws::replay_determinism(&a, &b).is_err()); }
2624:crates/amun-replay-semantics/src/lib.rs:9:#[derive(Debug, Clone, Copy, PartialEq, Eq)] pub enum ReplayDomain { Consensus, Execution, FullSystem, Governance, Transcript }
2625:crates/amun-replay-store/src/lib.rs:103:        let store = ReplayStore::new(path);
2626:crates/amun-replay-store/src/lib.rs:116:        let store = ReplayStore::new(path);
2627:crates/amun-replay-store/src/lib.rs:128:        let store = ReplayStore::new(path);
2628:crates/amun-replay-store/src/lib.rs:140:        let store = ReplayStore::new(path);
2629:crates/amun-replay-store/src/lib.rs:153:        let store = ReplayStore::new(path);
2630:crates/amun-replay-store/src/lib.rs:16:impl ReplayRecord {
2631:crates/amun-replay-store/src/lib.rs:31:pub struct ReplayStore {
2632:crates/amun-replay-store/src/lib.rs:35:impl ReplayStore {
2633:crates/amun-replay-store/src/lib.rs:43:    pub fn append(&self, record: &ReplayRecord) -> Result<(), String> {
2634:crates/amun-replay-store/src/lib.rs:54:    pub fn load_all(&self) -> Result<Vec<ReplayRecord>, String> {
2635:crates/amun-replay-store/src/lib.rs:63:    pub fn load_height(&self, height: u64) -> Result<Option<ReplayRecord>, String> {
2636:crates/amun-replay-store/src/lib.rs:89:    fn make_record(h: u64, before: &str, after: &str) -> ReplayRecord {
2637:crates/amun-replay-store/src/lib.rs:8:pub struct ReplayRecord {
2638:crates/amun-replay-store/src/lib.rs:90:        ReplayRecord {
2639:crates/amun-replay-verifier/src/replay_verifier.rs:100:                    return ReplayResult::Divergence {
2640:crates/amun-replay-verifier/src/replay_verifier.rs:106:                ReplayResult::Match {
2641:crates/amun-replay-verifier/src/replay_verifier.rs:111:            Ok(PipelineResult::Rejected { evidence, .. }) => ReplayResult::Error {
2642:crates/amun-replay-verifier/src/replay_verifier.rs:114:            Err(e) => ReplayResult::Error { reason: e },
2645:crates/amun-replay-verifier/src/replay_verifier.rs:13:pub enum ReplayResult {
2646:crates/amun-replay-verifier/src/replay_verifier.rs:155:            ReplayVerifier::replay(&proof, &p, &mut r2, &[]),
2647:crates/amun-replay-verifier/src/replay_verifier.rs:156:            ReplayResult::Match { .. }
2650:crates/amun-replay-verifier/src/replay_verifier.rs:186:            ReplayVerifier::replay(&proof, &p, &mut r2, &[]),
2651:crates/amun-replay-verifier/src/replay_verifier.rs:187:            ReplayResult::Match { .. }
2654:crates/amun-replay-verifier/src/replay_verifier.rs:217:                ReplayVerifier::replay(&proof, &p, &mut f, &[]),
2655:crates/amun-replay-verifier/src/replay_verifier.rs:218:                ReplayResult::Match { .. }
2656:crates/amun-replay-verifier/src/replay_verifier.rs:28:pub struct ReplayVerifier;
2657:crates/amun-replay-verifier/src/replay_verifier.rs:30:impl ReplayVerifier {
2661:crates/amun-replay-verifier/src/replay_verifier.rs:36:    ) -> ReplayResult {
2665:crates/amun-replay-verifier/src/replay_verifier.rs:86:                    return ReplayResult::Divergence {
2666:crates/amun-replay-verifier/src/replay_verifier.rs:93:                    return ReplayResult::Divergence {
2668:crates/amun-replay/src/certificate.rs:115:    pub fn latest(&self) -> Option<&ReplayCertificate> {
2669:crates/amun-replay/src/certificate.rs:126:    use crate::validation::{ReplayResult, ReplayValidator};
2670:crates/amun-replay/src/certificate.rs:139:    fn valid_result() -> ReplayResult {
2671:crates/amun-replay/src/certificate.rs:140:        ReplayValidator::validate(&[
2672:crates/amun-replay/src/certificate.rs:149:        let genesis = ReplayCertificate::genesis([0xAA; 32], 1000);
2673:crates/amun-replay/src/certificate.rs:157:        let mut store = ReplayCertificateStore::new();
2674:crates/amun-replay/src/certificate.rs:158:        let genesis = ReplayCertificate::genesis([0xAA; 32], 1000);
2675:crates/amun-replay/src/certificate.rs:162:        let cert1 = ReplayCertificate::issue(&result, 3, prev_id, [0xAA; 32], 2000).unwrap();
2676:crates/amun-replay/src/certificate.rs:165:        let cert2 = ReplayCertificate::issue(&result, 6, prev_id2, [0xAA; 32], 3000).unwrap();
2677:crates/amun-replay/src/certificate.rs:172:        let mut store = ReplayCertificateStore::new();
2678:crates/amun-replay/src/certificate.rs:173:        let genesis = ReplayCertificate::genesis([0xAA; 32], 1000);
2679:crates/amun-replay/src/certificate.rs:176:        let mut cert = ReplayCertificate::issue(&result, 3, [0xFF; 32], [0xAA; 32], 2000).unwrap();
2680:crates/amun-replay/src/certificate.rs:17:impl ReplayCertificate {
2681:crates/amun-replay/src/certificate.rs:183:        let mut store = ReplayCertificateStore::new();
2682:crates/amun-replay/src/certificate.rs:184:        let genesis = ReplayCertificate::genesis([0xAA; 32], 1000);
2683:crates/amun-replay/src/certificate.rs:188:        let mut cert = ReplayCertificate::issue(&result, 3, prev_id, [0xAA; 32], 2000).unwrap();
2684:crates/amun-replay/src/certificate.rs:195:        let mut store = ReplayCertificateStore::new();
2685:crates/amun-replay/src/certificate.rs:197:            .store(ReplayCertificate::genesis([0xAA; 32], 1000))
2686:crates/amun-replay/src/certificate.rs:19:        result: &ReplayResult,
2687:crates/amun-replay/src/certificate.rs:1:use crate::validation::ReplayResult;
2688:crates/amun-replay/src/certificate.rs:202:            .store(ReplayCertificate::issue(&r, 3, id1, [0xAA; 32], 2000).unwrap())
2689:crates/amun-replay/src/certificate.rs:206:            .store(ReplayCertificate::issue(&r, 6, id2, [0xAA; 32], 3000).unwrap())
2690:crates/amun-replay/src/certificate.rs:6:pub struct ReplayCertificate {
2691:crates/amun-replay/src/certificate.rs:81:pub struct ReplayCertificateStore {
2692:crates/amun-replay/src/certificate.rs:82:    certificates: HashMap<[u8; 32], ReplayCertificate>,
2693:crates/amun-replay/src/certificate.rs:86:impl ReplayCertificateStore {
2694:crates/amun-replay/src/certificate.rs:91:    pub fn store(&mut self, cert: ReplayCertificate) -> Result<(), &'static str> {
2695:crates/amun-replay/src/lib.rs:36:pub use certificate::{ReplayCertificate, ReplayCertificateStore};
2696:crates/amun-replay/src/lib.rs:38:pub use validation::{ReplayResult, ReplayValidator};
2697:crates/amun-replay/src/lib.rs:40:pub use store::{CertificateProvider, ReplayStore};
2698:crates/amun-replay/src/store.rs:10:impl ReplayStore {
2699:crates/amun-replay/src/store.rs:18:    pub fn insert(&mut self, cert: ReplayCertificate) {
2700:crates/amun-replay/src/store.rs:1:use amun_constitutional_state::ReplayCertificate;
2701:crates/amun-replay/src/store.rs:24:    pub fn get(&self, hash: &[u8; 32]) -> Option<&ReplayCertificate> {
2702:crates/amun-replay/src/store.rs:44:impl Default for ReplayStore {
2703:crates/amun-replay/src/store.rs:4:/// Stores ReplayCertificates keyed by their certificate_hash.
2704:crates/amun-replay/src/store.rs:50:/// A generic interface for retrieving ReplayCertificates by hash.
2705:crates/amun-replay/src/store.rs:52:    fn get_certificate(&self, hash: &[u8; 32]) -> Option<ReplayCertificate>;
2706:crates/amun-replay/src/store.rs:55:impl CertificateProvider for ReplayStore {
2707:crates/amun-replay/src/store.rs:56:    fn get_certificate(&self, hash: &[u8; 32]) -> Option<ReplayCertificate> {
2708:crates/amun-replay/src/store.rs:6:pub struct ReplayStore {
2709:crates/amun-replay/src/store.rs:73:        let mut store = ReplayStore::new();
2710:crates/amun-replay/src/store.rs:7:    certificates: BTreeMap<[u8; 32], ReplayCertificate>,
2711:crates/amun-replay/src/store.rs:83:        let store = ReplayStore::new();
2712:crates/amun-replay/src/store.rs:98:        let mut store = ReplayStore::new();
2713:crates/amun-replay/src/validation.rs:104:        let result = ReplayValidator::validate(&[]);
2714:crates/amun-replay/src/validation.rs:115:        let r1 = ReplayValidator::validate(&commits);
2715:crates/amun-replay/src/validation.rs:116:        let r2 = ReplayValidator::validate(&commits);
2716:crates/amun-replay/src/validation.rs:11:impl ReplayResult {
2717:crates/amun-replay/src/validation.rs:31:pub struct ReplayValidator;
2718:crates/amun-replay/src/validation.rs:33:impl ReplayValidator {
2719:crates/amun-replay/src/validation.rs:34:    pub fn validate(commits: &[StateCommit]) -> ReplayResult {
2720:crates/amun-replay/src/validation.rs:36:            return ReplayResult::success([0u8; 32], 0);
2721:crates/amun-replay/src/validation.rs:42:                return ReplayResult::failure(prev.new_root, curr.previous_root, i);
2722:crates/amun-replay/src/validation.rs:48:        ReplayResult::success(last.new_root, commits.len())
2723:crates/amun-replay/src/validation.rs:4:pub struct ReplayResult {
2724:crates/amun-replay/src/validation.rs:51:    pub fn validate_log(log: &CommitLog) -> ReplayResult {
2725:crates/amun-replay/src/validation.rs:78:        let result = ReplayValidator::validate(&commits);
2726:crates/amun-replay/src/validation.rs:89:        let result = ReplayValidator::validate(&commits);
2727:crates/amun-replay/src/validation.rs:97:        let result = ReplayValidator::validate(&commits);
2807:crates/amun-self-preservation/src/legitimacy_guards.rs:14:    ReplayDeterminism,
2829:crates/amun-snapshot-engine/src/compatibility.rs:123:            CompatibilityLevel::ReplayCompatible
2830:crates/amun-snapshot-engine/src/compatibility.rs:13:    ReplayCompatible,
2831:crates/amun-snapshot-engine/src/compatibility.rs:145:            CompatibilityLevel::FullyCompatible | CompatibilityLevel::ReplayCompatible
2832:crates/amun-snapshot-engine/src/compatibility.rs:155:                | CompatibilityLevel::ReplayCompatible
2833:crates/amun-snapshot-engine/src/compatibility.rs:166:                | CompatibilityLevel::ReplayCompatible
2834:crates/amun-snapshot-engine/src/lib.rs:31:pub use replay_continuity::{ContinuityResult, ReplayContinuityEngine};
2835:crates/amun-snapshot-engine/src/replay_continuity.rs:10:pub struct ReplayContinuityEngine;
2836:crates/amun-snapshot-engine/src/replay_continuity.rs:1:// Replay Continuity Engine
2837:crates/amun-snapshot-engine/src/replay_continuity.rs:27:impl ReplayContinuityEngine {
2838:crates/amun-snapshot-engine/src/replay_continuity.rs:41:        // Step 2: Replay WAL from snapshot checkpoint forward
2839:crates/amun-snapshot-engine/src/transition.rs:30:            CompatibilityLevel::ReplayCompatible | CompatibilityLevel::SnapshotCompatible => {
2862:crates/amun-state-machine/src/absolute_invariants.rs:13:    /// Replay determinism must be preserved across ALL amendments
2863:crates/amun-state-machine/src/absolute_invariants.rs:14:    ReplayDeterminismAbsolute,
2866:crates/amun-state-machine/src/absolute_invariants.rs:41:            Self::ReplayDeterminismAbsolute,
2868:crates/amun-state-machine/src/axioms.rs:12:    /// Replay determinism is preserved across all legal transitions
2869:crates/amun-state-machine/src/axioms.rs:13:    ReplayDeterminismPreserved,
2871:crates/amun-state-machine/src/axioms.rs:27:    HostileForkReplayImpossible,
2872:crates/amun-state-machine/src/axioms.rs:38:            Self::ReplayDeterminismPreserved => {
2873:crates/amun-state-machine/src/axioms.rs:39:                "Replay must remain deterministic across all transitions"
2875:crates/amun-state-machine/src/axioms.rs:47:            Self::HostileForkReplayImpossible => "Hostile forks cannot preserve replay",
2876:crates/amun-state-machine/src/delta_algebra.rs:11:    ReplayDelta {
2877:crates/amun-state-machine/src/delta_algebra.rs:43:            Self::ReplayDelta { .. } => 0x02,
2878:crates/amun-state-machine/src/delta_algebra.rs:57:            Self::ReplayDelta { .. } => 3,
2879:crates/amun-state-machine/src/delta_algebra.rs:77:            Self::ReplayDelta {
2880:crates/amun-state-machine/src/delta_laws.rs:33:                ConstitutionalDelta::ReplayDelta {
2881:crates/amun-state-machine/src/delta_laws.rs:37:                ConstitutionalDelta::ReplayDelta {
2888:crates/amun-state-machine/src/engine.rs:25:    pub replay_log: ConstitutionalReplayDAG,
2891:crates/amun-state-machine/src/engine.rs:38:            replay_log: ConstitutionalReplayDAG::new(),
2892:crates/amun-state-machine/src/engine.rs:4:use super::replay_log::ConstitutionalReplayDAG;
2893:crates/amun-state-machine/src/formal_entropy.rs:27:    /// Replay convergence absorbs entropy
2894:crates/amun-state-machine/src/formal_entropy.rs:28:    ReplayConvergence,
2897:crates/amun-state-machine/src/historical_invariants.rs:133:            HistoricalInvariant::MaxReplayDivergence {
2899:crates/amun-state-machine/src/historical_invariants.rs:14:    /// Replay divergence must not accumulate over lineage
2900:crates/amun-state-machine/src/historical_invariants.rs:15:    MaxReplayDivergence {
2905:crates/amun-state-machine/src/historical_invariants.rs:48:            HistoricalInvariant::MaxReplayDivergence {
2906:crates/amun-state-machine/src/historical_invariants.rs:54:                        "Replay divergence accumulation exceeded: {} > {}",
2911:crates/amun-state-machine/src/impossibility.rs:10:    HostileForkReplayPreservationImpossible,
2912:crates/amun-state-machine/src/invariants.rs:10:    /// Replay determinism must be preserved.
2913:crates/amun-state-machine/src/invariants.rs:11:    ReplayDeterminismPreserved,
2918:crates/amun-state-machine/src/lib.rs:50:pub use replay_log::{ConstitutionalReplayDAG, ReplayLogEntry};
2920:crates/amun-state-machine/src/merge_math.rs:10:    /// Replay histories are interleaved by epoch
2921:crates/amun-state-machine/src/merge_math.rs:11:    ReplayEpochInterleave,
2922:crates/amun-state-machine/src/merge_math.rs:34:            super::fork_merge::MergeType::Union => MergeResolution::ReplayUnion,
2923:crates/amun-state-machine/src/merge_math.rs:37:                    MergeResolution::ReplayLongestChainWins
2924:crates/amun-state-machine/src/merge_math.rs:39:                    MergeResolution::ReplayLongestChainWins
2925:crates/amun-state-machine/src/merge_math.rs:42:            super::fork_merge::MergeType::Federation => MergeResolution::ReplayEpochInterleave,
2926:crates/amun-state-machine/src/merge_math.rs:7:    ReplayUnion,
2927:crates/amun-state-machine/src/merge_math.rs:9:    ReplayLongestChainWins,
2929:crates/amun-state-machine/src/meta_amendment.rs:23:    ReplayGuarantees,
2931:crates/amun-state-machine/src/meta_amendment.rs:53:            MetaAmendmentScope::ProofSemantics | MetaAmendmentScope::ReplayGuarantees => {
2932:crates/amun-state-machine/src/meta_amendment.rs:57:                        AbsoluteInvariant::ReplayDeterminismAbsolute => {
2933:crates/amun-state-machine/src/meta_amendment.rs:58:                            if matches!(scope, MetaAmendmentScope::ReplayGuarantees) {
2934:crates/amun-state-machine/src/meta_amendment.rs:60:                                    "Cannot amend replay guarantees: ReplayDeterminismAbsolute"
2938:crates/amun-state-machine/src/preconditions.rs:15:    /// Replay continuity must be preserved.
2939:crates/amun-state-machine/src/preconditions.rs:16:    ReplayContinuityPreserved,
2940:crates/amun-state-machine/src/replay_log.rs:18:impl ReplayLogEntry {
2941:crates/amun-state-machine/src/replay_log.rs:46:/// Constitutional Replay DAG - branching constitutional history.
2942:crates/amun-state-machine/src/replay_log.rs:48:pub struct ConstitutionalReplayDAG {
2943:crates/amun-state-machine/src/replay_log.rs:49:    pub entries: HashMap<[u8; 32], ReplayLogEntry>,
2944:crates/amun-state-machine/src/replay_log.rs:54:impl Default for ConstitutionalReplayDAG {
2945:crates/amun-state-machine/src/replay_log.rs:60:impl ConstitutionalReplayDAG {
2946:crates/amun-state-machine/src/replay_log.rs:69:    pub fn append(&mut self, transition: &Transition, parents: Vec<[u8; 32]>) -> ReplayLogEntry {
2947:crates/amun-state-machine/src/replay_log.rs:70:        let entry = ReplayLogEntry::new(self.next_sequence, transition, parents.clone());
2948:crates/amun-state-machine/src/replay_log.rs:8:pub struct ReplayLogEntry {
2965:crates/amun-state-root/src/lib.rs:13:pub use replay::{ReplayCertificate, ReplayEquivalenceProof, ReplayTranscript};
2966:crates/amun-state-root/src/replay.rs:10:impl CanonicalEncode for ReplayEquivalenceProof {
2967:crates/amun-state-root/src/replay.rs:19:pub struct ReplayTranscript {
2968:crates/amun-state-root/src/replay.rs:29:impl CanonicalEncode for ReplayTranscript {
2969:crates/amun-state-root/src/replay.rs:42:pub struct ReplayCertificate {
2970:crates/amun-state-root/src/replay.rs:43:    pub transcript: ReplayTranscript,
2971:crates/amun-state-root/src/replay.rs:44:    pub proof: ReplayEquivalenceProof,
2972:crates/amun-state-root/src/replay.rs:47:impl CanonicalEncode for ReplayCertificate {
2973:crates/amun-state-root/src/replay.rs:4:pub struct ReplayEquivalenceProof {
2974:crates/amun-state-root/src/snapshot.rs:17:    /// Replay equivalence proof.
2975:crates/amun-state-root/src/snapshot.rs:18:    pub replay_equivalence_proof: ReplayEquivalenceProof,
2976:crates/amun-state-root/src/snapshot.rs:24:pub struct ReplayEquivalenceProof {
2977:crates/amun-state-root/src/snapshot.rs:32:impl ReplayEquivalenceProof {
2978:crates/amun-state-root/src/snapshot.rs:40:impl CanonicalEncode for ReplayEquivalenceProof {
2997:crates/amun-stateless-sync/src/lib.rs:105:    pub fn import_certificate(&mut self, cert: ReplayCertificate) {
2998:crates/amun-stateless-sync/src/lib.rs:34:use amun_constitutional_state::{CertificateInclusionProof, ReplayCertificate};
2999:crates/amun-stateless-sync/src/lib.rs:59:        certificates: Vec<ReplayCertificate>,
3000:crates/amun-stateless-sync/src/lib.rs:80:    certificates: BTreeMap<[u8; 32], ReplayCertificate>,
3001:crates/amun-stf/src/nonce.rs:39:                ConstitutionalFault::ReplayViolation,
3002:crates/amun-storage-kernel/ATOMIC_SNAPSHOT_CONSTITUTION.md:20:MUST match the state at the freeze point exactly. Replay of WAL from
3004:crates/amun-storage-kernel/CONSTITUTION.md:65:**Section 3.2: Replay Equivalence**
3005:crates/amun-storage-kernel/CONSTITUTION.md:66:Replaying a valid WAL from genesis SHALL produce the identical state root
3007:crates/amun-storage-kernel/SNAPSHOT_CONSTITUTION.md:141:**Section 8.1: Snapshot + WAL Replay**
3008:crates/amun-storage-kernel/VALIDITY_HIERARCHY.md:33:| StateRootMismatch | ReplayVerifier divergence | Quarantine state, resync from trusted peer |
3009:crates/amun-storage-kernel/VALIDITY_HIERARCHY.md:45:| ReplayDivergence | Replay produces different root | Investigate, rebuild from trusted source |
3012:crates/amun-storage-kernel/VALIDITY_HIERARCHY.md:82:| Local WAL intact | Replay locally |
3014:crates/amun-storage-kernel/src/persistence/wal/iterator.rs:83:pub struct ReplayVerifier;
3015:crates/amun-storage-kernel/src/persistence/wal/iterator.rs:84:impl ReplayVerifier {
3016:crates/amun-storage-kernel/src/persistence/wal/mod.rs:5:pub use iterator::{ReplayVerifier, WalIterator};
3024:crates/amun-storage-kernel/tests/replay_equivalence.rs:136:        let result = ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap());
3025:crates/amun-storage-kernel/tests/replay_equivalence.rs:4:        persistence::wal::{ReplayVerifier, WalEntry},
3026:crates/amun-storage-kernel/tests/replay_equivalence.rs:54:            ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap()).unwrap();
3027:crates/amun-storage-kernel/tests/replay_equivalence.rs:94:        let result = ReplayVerifier::verify_full_replay(wal_path.to_str().unwrap());
3033:crates/amun-testnet-sim/tests/adversarial_tests.rs:106:    assert!(ReplayBackedConsensus::form_consensus(
3034:crates/amun-testnet-sim/tests/adversarial_tests.rs:127:    let mut block = ReplayBackedConsensus::execute_and_replay(
3035:crates/amun-testnet-sim/tests/adversarial_tests.rs:137:    assert!(ReplayBackedConsensus::form_consensus(
3036:crates/amun-testnet-sim/tests/adversarial_tests.rs:206:                // Replay against a registry initialized to the proof's pre-state
3037:crates/amun-testnet-sim/tests/adversarial_tests.rs:209:                                                    // Actually, ReplayVerifier::replay calls execute() which starts
3038:crates/amun-testnet-sim/tests/adversarial_tests.rs:212:                let replay = ReplayVerifier::replay(&transition_proof, &program, &mut fresh, &[]);
3039:crates/amun-testnet-sim/tests/adversarial_tests.rs:213:                if !matches!(replay, ReplayResult::Match { .. }) {
3042:crates/amun-testnet-sim/tests/adversarial_tests.rs:242:    let mut block = ReplayBackedConsensus::execute_and_replay(
3043:crates/amun-testnet-sim/tests/adversarial_tests.rs:252:    assert!(ReplayBackedConsensus::form_consensus(
3045:crates/amun-testnet-sim/tests/adversarial_tests.rs:55:    let block = ReplayBackedConsensus::execute_and_replay(
3046:crates/amun-testnet-sim/tests/adversarial_tests.rs:63:    assert!(ReplayBackedConsensus::form_consensus(
3047:crates/amun-testnet-sim/tests/adversarial_tests.rs:69:    assert!(ReplayBackedConsensus::form_consensus(
3048:crates/amun-testnet-sim/tests/adversarial_tests.rs:6:use amun_replay_consensus::replay_backed_consensus::ReplayBackedConsensus;
3049:crates/amun-testnet-sim/tests/adversarial_tests.rs:76:        assert!(ReplayBackedConsensus::form_consensus(
3050:crates/amun-testnet-sim/tests/adversarial_tests.rs:7:use amun_replay_verifier::replay_verifier::{ReplayResult, ReplayVerifier};
3051:crates/amun-testnet-sim/tests/adversarial_tests.rs:98:    let block = ReplayBackedConsensus::execute_and_replay(
3053:crates/amun-transcript-semantics/src/lib.rs:102:    #[test] fn test_causal_chain() { let p = EventIdentity::new([0x01;32],[0x00;32],[0xAA;32],1,ReplayDomain::Consensus,[0xBB;32]); let c = EventIdentity::new([0x02;32],[0x01;32],[0xAA;32],2,ReplayDomain::Consensus,[0xBB;32]); assert!(c.verify_causal_chain(&p)); }
3054:crates/amun-transcript-semantics/src/lib.rs:104:    #[test] fn test_immutable_cert() { let c = ImmutableReplayCertificate::new(ReplayDomain::Consensus,[0xBB;32],[0x01;32],[0x02;32],[0x03;32],[0x04;32],100,1); assert!(c.verify()); let h1 = c.certificate_hash(); let h2 = c.certificate_hash(); assert_eq!(h1, h2); }
3055:crates/amun-transcript-semantics/src/lib.rs:105:    #[test] fn test_witness_hash() { let w = ReplayWitness::MerkleWitness { leaf_hash: [0x01;32], proof_hashes: vec![[0x02;32]], leaf_index: 0 }; assert_ne!(w.witness_hash(), [0;32]); }
3056:crates/amun-transcript-semantics/src/lib.rs:11:pub struct EventIdentity { pub event_hash: [u8; 32], pub causal_parent: [u8; 32], pub authority_root: [u8; 32], pub transcript_position: u64, pub domain: ReplayDomain, pub epoch_id: [u8; 32] }
3057:crates/amun-transcript-semantics/src/lib.rs:13:    pub fn new(event_hash: [u8; 32], causal_parent: [u8; 32], authority_root: [u8; 32], transcript_position: u64, domain: ReplayDomain, epoch_id: [u8; 32]) -> Self { Self { event_hash, causal_parent, authority_root, transcript_position, domain, epoch_id } }
3058:crates/amun-transcript-semantics/src/lib.rs:25:// ─── Replay Class ──────────────────────────────────────────
3059:crates/amun-transcript-semantics/src/lib.rs:27:pub enum ReplayClass { ReplayRequired, ReplayRecommended, ReplayDerived, ReplayExcluded, ReplayOptional }
3060:crates/amun-transcript-semantics/src/lib.rs:52:    pub fn domain(&self) -> ReplayDomain {
3061:crates/amun-transcript-semantics/src/lib.rs:53:        match self { TranscriptEntry::Consensus(_) => ReplayDomain::Consensus, TranscriptEntry::Execution(_) => ReplayDomain::Execution, TranscriptEntry::Governance(_) => ReplayDomain::Governance, TranscriptEntry::Certifying(_) => ReplayDomain::FullSystem }
3062:crates/amun-transcript-semantics/src/lib.rs:59:pub struct ImmutableReplayCertificate { inner: CertifiedEnvelope }
3063:crates/amun-transcript-semantics/src/lib.rs:61:struct CertifiedEnvelope { domain: ReplayDomain, epoch_id: [u8; 32], transcript_root: [u8; 32], state_root: [u8; 32], receipt_root: [u8; 32], ordering_root: [u8; 32], event_count: u64, replay_version: u32, certificate_hash: [u8; 32] }
3064:crates/amun-transcript-semantics/src/lib.rs:62:impl ImmutableReplayCertificate {
3065:crates/amun-transcript-semantics/src/lib.rs:64:    pub fn new(domain: ReplayDomain, epoch_id: [u8; 32], transcript_root: [u8; 32], state_root: [u8; 32], receipt_root: [u8; 32], ordering_root: [u8; 32], event_count: u64, replay_version: u32) -> Self {
3066:crates/amun-transcript-semantics/src/lib.rs:69:    pub fn domain(&self) -> ReplayDomain { self.inner.domain }
3067:crates/amun-transcript-semantics/src/lib.rs:6:use amun_replay_semantics::ReplayDomain;
3069:crates/amun-transcript-semantics/src/lib.rs:78:pub enum ReplayWitness {
3074:crates/amun-transcript-semantics/src/lib.rs:83:    CompositeWitness { witnesses: Vec<ReplayWitness>, composite_hash: [u8; 32] },
3075:crates/amun-transcript-semantics/src/lib.rs:85:impl ReplayWitness {
3076:crates/amun-transcript-semantics/src/lib.rs:88:        match self { ReplayWitness::MerkleWitness { leaf_hash, proof_hashes, leaf_index } => { h.update(b"MERKLE"); h.update(leaf_hash); h.update(leaf_index.to_le_bytes()); for ph in proof_hashes { h.update(ph); } } ReplayWitness::ExecutionWitness { block_hash, trace_hash, step_count } => { h.update(b"EXECUTION"); h.update(block_hash); h.update(trace_hash); h.update(step_count.to_le_bytes()); } ReplayWitness::TranscriptWitness { start_sequence, end_sequence, fragment_hash } => { h.update(b"TRANSCRIPT"); h.update(start_sequence.to_le_bytes()); h.update(end_sequence.to_le_bytes()); h.update(fragment_hash); } ReplayWitness::ReceiptWitness { receipt_hashes, chain_root } => { h.update(b"RECEIPT"); for rh in receipt_hashes { h.update(rh); } h.update(chain_root); } ReplayWitness::CompositeWitness { composite_hash, .. } => { h.update(b"COMPOSITE"); h.update(composite_hash); } }
3077:crates/amun-transcript-semantics/src/lib.rs:95:    #[derive(Debug, Clone, PartialEq, Eq)] pub enum TranscriptError { CausalChainBroken { parent_hash: [u8; 32], child_parent_hash: [u8; 32] }, AuthorityMismatch { expected: EventAuthority, actual: EventAuthority }, IncompleteReplay { expected: usize, actual: usize } }
3111:crates/amun-truth-engine/src/engine.rs:23:pub enum ReplayError {
3112:crates/amun-truth-engine/src/engine.rs:248:    pub fn compute_chain_root_until(&self, until: ChainPosition) -> Result<[u8; 32], ReplayError> {
3113:crates/amun-truth-engine/src/engine.rs:265:                        .ok_or(ReplayError::ChunkReadError {
3114:crates/amun-truth-engine/src/engine.rs:276:    pub fn compute_chain_root(&self, target_tx_count: u64) -> Result<[u8; 32], ReplayError> {
3115:crates/amun-truth-engine/src/engine.rs:291:                        .ok_or(ReplayError::ChunkReadError {
3116:crates/amun-truth-engine/src/lib.rs:4:pub use engine::{ReplayError, TruthEngine};
3165:crates/amun_state_machine/src/certification/mod.rs:61:pub struct ReplayCertificate {
3166:crates/amun_state_machine/src/certification/mod.rs:70:impl ReplayCertificate {
3167:crates/amun_state_machine/src/lib.rs:24:pub use certification::{ExecutionCertificate, ReplayCertificate, compute_execution_fingerprint};
3168:docs/AUDIT_LAYERS.md:69:# Layer 06 — Replay
3169:docs/CANONICAL_HASH.md:19:| Transcript | `AMUN_TRANSCRIPT_V1` | Replay transcript |
3170:docs/CANONICAL_HASH.md:8:- Replay certificates
3195:docs/CONSTITUTIONAL_MODEL.md:105:# Replayability
3196:docs/CONSTITUTIONAL_MODEL.md:109:Replay is constitutional infrastructure.
3198:docs/CRATE_ARCHITECTURE.md:195:## Layer 9 — Replay
3199:docs/CRATE_ARCHITECTURE.md:201:Replay equivalence and divergence detection.
3200:docs/CRATE_ARCHITECTURE.md:213:Replay semantic interpretation.
3201:docs/CRATE_ARCHITECTURE.md:246:-> Replay
3204:docs/DOCS_INDEX.md:23:    Replay protection through domain-separated signing payloads.
3205:docs/DOCS_INDEX.md:311:    Replay physics v1.
3206:docs/DOCS_INDEX.md:317:    Replay law.
3207:docs/DOCS_INDEX.md:319:    Replay model.
3208:docs/DOCS_INDEX.md:435:    Replay cost analysis.
3209:docs/DOCS_INDEX.md:457:    Replay checkpoint fixtures.
3210:docs/DOCS_INDEX.md:459:    Replay divergence fixtures.
3211:docs/DOCS_INDEX.md:461:    Replay equivalence fixtures.
3212:docs/DOCS_INDEX.md:463:    Replay genesis fixtures.
3214:docs/DOCS_INDEX.md:91:    Replay determinism law for storage.
3215:docs/N105_CRYPTOGRAPHIC_VALIDATOR_IDENTITY.md:53:Vote forgery resistance is achieved because without the private key, an attacker cannot produce a valid signature for a given voter_id. The registry ensures that only known validators can attempt to vote. Sybil resistance is enforced because the registry is populated from certificates signed by the authority. An attacker cannot inject fake validators. In a production setting, the authority would be the genesis trust anchor set. Replay protection is provided by the signing payload, which includes the height, block hash, and timestamp, binding each vote to a specific consensus round. The authority key is currently hardcoded to the seed 0x42, which is acceptable for test clusters. In production, this key must be distributed via genesis configuration and protected. Certificate integrity is verified at load time. A tampered certificate will cause a panic in test clusters or an error in production loading code, preventing the node from participating.
3223:docs/N126_FINAL_BASELINE.md:32:7.  ReplayDeterminism         = cert.state_root != [0u8; 32]           PARTIAL
3226:docs/N126_FINAL_BASELINE.md:45:1. ReplayVerifier integration (ReplayDeterminism)
3230:docs/PROJECT_INDEX.md:103:| docs/REPLAY_LAW.md | Replay legal semantics |
3231:docs/PROJECT_INDEX.md:127:| docs/protocol/replay_physics_v1.md | Replay physics |
3232:docs/PROJECT_INDEX.md:159:## Replayability
3233:docs/PROJECT_INDEX.md:55:| fixtures/ | Replay and snapshot fixtures |
3234:docs/PROJECT_INDEX.md:86:| constitution/consensus/REPLAY_DETERMINISM_LAW.md | Replay guarantees |
3235:docs/PROJECT_INDEX.md:97:| docs/REPLAY_MODEL.md | Replay semantics |
3236:docs/PROTOCOL_HARDENING_ROADMAP.md:18:## Phase 3: Replay Certification
3237:docs/PROTOCOL_HARDENING_ROADMAP.md:21:- [ ] Replay certificate generation
3238:docs/PROTOCOL_HARDENING_ROADMAP.md:22:- [ ] Replay attack proof system
3239:docs/REPLAY_LAW.md:109:Replay-equivalent iff:
3240:docs/REPLAY_LAW.md:142:ReplayHash = H(Transcript)
3244:docs/REPLAY_LAW.md:15:Replay guarantees:
3245:docs/REPLAY_LAW.md:166:Replay certificate proves:
3246:docs/REPLAY_LAW.md:188:Replay MUST fail if:
3247:docs/REPLAY_LAW.md:258:Replay security depends on:
3248:docs/REPLAY_LAW.md:269:Replay law applies to:
3249:docs/REPLAY_LAW.md:26:Replay(GenesisState, Transcript) = FinalState
3250:docs/REPLAY_LAW.md:280:Replay does NOT guarantee:
3251:docs/REPLAY_LAW.md:28:ReplayHash = OriginalReplayHash
3252:docs/REPLAY_LAW.md:290:Replay Model:
3253:docs/REPLAY_LAW.md:30:Replay MUST always produce identical output.
3254:docs/REPLAY_LAW.md:325:Replay becomes frozen after:
3255:docs/REPLAY_LAW.md:334:Replay is the constitutional heart of AmunChain.
3256:docs/REPLAY_LAW.md:36:Replay is defined as:
3257:docs/REPLAY_LAW.md:3:## Phase 81 — Deterministic Replay Constitution
3258:docs/REPLAY_LAW.md:40:Replay is NOT:
3259:docs/REPLAY_LAW.md:51:Replay input:
3260:docs/REPLAY_LAW.md:61:Replay output:
3261:docs/REPLAY_LAW.md:64:- Replay Transcript Hash
3262:docs/REPLAY_LAW.md:65:- Replay Certificate
3264:docs/REPLAY_MODEL.md:11:# Replay Guarantees
3265:docs/REPLAY_MODEL.md:1:# Replay Model
3266:docs/REPLAY_MODEL.md:28:Replay may not depend on:
3267:docs/REPLAY_MODEL.md:38:Replay divergence must always be detectable.
3268:docs/REPLAY_MODEL.md:48:# Replay DAG
3269:docs/REPLAY_MODEL.md:50:Replay execution is modeled as a DAG.
3270:docs/REPLAY_MODEL.md:5:Replay is a constitutional guarantee.
3271:docs/REPLAY_MODEL.md:60:# Replay Certificates
3272:docs/REPLAY_MODEL.md:62:Replay certificates prove:
3273:docs/REPLAY_MODEL.md:70:# Replay Philosophy
3274:docs/REPLAY_MODEL.md:72:Replay is constitutional historical reconstruction.
3275:docs/REPOSITORY_LAYOUT.md:206:## Replay Layer
3276:docs/REPOSITORY_LAYOUT.md:21:| fixtures/ | Replay and snapshot test fixtures |
3277:docs/REPOSITORY_LAYOUT.md:247:| REPLAY_MODEL.md | Replay semantics |
3278:docs/SECURITY_MODEL.md:39:## Replay Divergence
3322:docs/VALIDATOR_ORDERING.md:28:- Replay equivalence
3323:docs/architecture/CRATE_CLASSIFICATION.md:38:- amun-test-replay - Replay testing
3324:docs/architecture/PHASE_49_COMPLETE.md:100:| R.1 | System replayable from transcript | Replay Law |
3325:docs/architecture/PHASE_49_COMPLETE.md:107:2. **Replay Certification suite** — cross-node equivalence proofs
3327:docs/architecture/PHASE_49_COMPLETE.md:128:The foundation is ready for Phase 50: Formal Replay Infrastructure.
3328:docs/architecture/PHASE_49_COMPLETE.md:41:3. **Capability Firewall**: 6 separated capabilities (Executor, Verifier, Journal, Replay, Finalize, Authority)
3329:docs/audit/AUDIT_EVIDENCE_BUNDLE.md:19:| C1 | Replay before vote | w18_reject_if_replay_fails | amun-replay-consensus | ✅ |
3336:docs/audit/AUDIT_EVIDENCE_BUNDLE.md:85:| Replay | amun-replay-verifier | 3 | ✅ |
3346:docs/audit/SECURITY_INVARIANTS.md:29:| C1 | Replay before vote | ReplayBackedConsensus::form_consensus |
3348:docs/audit/SECURITY_INVARIANTS.md:32:| C4 | No conflicting finality | Theorem 5 (Replay-Backed Safety) |
3352:docs/audit/SECURITY_INVARIANTS.md:40:| K3 | Anti-replay protection | AntiReplayGuard::check_and_record |
3355:docs/audit/SECURITY_INVARIANTS.md:8:| R2 | Consumed resources cannot be used | VMKernel::verify | ReplayVerifier |
3358:docs/audit/THREAT_MODEL.md:20:| Consensus | Double voting | Replay-backed QC (C1) |
3359:docs/audit/THREAT_MODEL.md:22:| Network | Replay attack | AntiReplayGuard (K3) |
3360:docs/audit/TRACEABILITY_MATRIX.md:10:| N48.5-W14 Replay | amun-replay-verifier | 3 tests | ✅ |
3368:docs/consensus/constitution.md:111:### Section 6.1: Execution Replay
3371:docs/constitutional/phase84_freeze.md:24:- Levels: FullyCompatible > ReplayCompatible > SnapshotCompatible > ReadOnlyCompatible > Incompatible
3373:docs/constitutional/phase84_freeze.md:54:- ReplayVerifier checks every frame's state_root against reconstructed state
3374:docs/constitutional/phase85_seal.md:16:8. Graded guarantees: Replay/Snapshot/Proof/Governance/Continuity
3377:docs/federation/FEDERATION_ARCHITECTURE.md:29:| Replay Boundary Engine | Replay isolation |
3391:docs/protocol/FREEZE_CERTIFICATE_v1.md:22:- Genesis Replay Root: fixtures/replay/genesis/replay_root.bin
3393:docs/protocol/FREEZE_CERTIFICATE_v1.md:5:This certifies that the Amun Replay Protocol v1.0 has been
3397:docs/protocol/replay_physics_v1.md:14:sequence numbers MUST be strictly monotonic within a replay session. Any gap (expected_sequence != actual_sequence) SHALL produce ReplayFailure::OrderingViolation.
3398:docs/protocol/replay_physics_v1.md:24:execute_and_self_verify() SHALL: 1. Execute trace via DeterministicExecutor, 2. Apply same entries via ReplayState, 3. Compare final roots, 4. Return EquivalenceProof.
3399:docs/reports/CCA_IMPL_5B_FINAL_REPORT.md:154:3. Replayable Constitutional State
