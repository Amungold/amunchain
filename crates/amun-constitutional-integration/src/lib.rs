use amun_constitutional_proof::{
    ConstitutionalReport, ConstitutionalVerdict, EvidenceArchive,
    EvidenceRecord, EvidenceStatus, EvidenceType, ObligationId, ObligationKind,
    ObligationNamespace, ObligationRegistry, ObligationResult, ObligationResultStatus,
    ObligationSeverity, ProofObligation, PublicationPackage, ReportGenerator,
    VerdictEvaluator,
};
use amun_verification_kernel::VerificationCertificate;
use std::collections::HashMap;

pub struct ConstitutionalBridge;

impl ConstitutionalBridge {
    pub fn build_obligation_registry() -> ObligationRegistry {
        let mut reg = ObligationRegistry::new();

        // Primary obligations
        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Safety, 1),
            ObligationKind::Primary,
            "No conflicting finalized blocks at same height",
            "forall b1,b2 in Finalized : b1.height = b2.height implies b1 = b2",
            ObligationSeverity::Critical,
            "N45",
        )).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Safety, 2),
            ObligationKind::Primary,
            "Finality requires quorum certification",
            "forall b in Finalized : exists QC(b) and |signers(QC(b))| >= 2f+1",
            ObligationSeverity::Critical,
            "N41",
        )).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Safety, 3),
            ObligationKind::Primary,
            "Finalized block can never be reverted",
            "forall b : b in Finalized(t) implies b in Finalized(t+delta)",
            ObligationSeverity::Critical,
            "N44",
        )).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Replay, 1),
            ObligationKind::Primary,
            "Every finalized block has replay evidence",
            "forall b in Finalized : exists replay_certificate(b)",
            ObligationSeverity::Critical,
            "N42",
        )).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Replay, 2),
            ObligationKind::Primary,
            "Replay execution is deterministic",
            "forall b : Replay(b, t1) = Replay(b, t2)",
            ObligationSeverity::Critical,
            "N42",
        )).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Evidence, 1),
            ObligationKind::Primary,
            "Every replay certificate maps to evidence",
            "forall rc in ReplayCertificate : exists ev in EvidenceRoot : ev.replay = rc",
            ObligationSeverity::Critical,
            "N42",
        )).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Evidence, 2),
            ObligationKind::Primary,
            "Evidence roots are immutable",
            "forall ev : ev in EvidenceRoot implies hash(ev) = constant",
            ObligationSeverity::Critical,
            "N42",
        )).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Finality, 1),
            ObligationKind::Primary,
            "Every finalized block has finality certificate",
            "forall b in Finalized : exists finality_certificate(b)",
            ObligationSeverity::Critical,
            "N41",
        )).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Finality, 2),
            ObligationKind::Primary,
            "Finality certificates are cryptographically valid",
            "forall fc in FinalityCertificate : verify_signatures(fc) = true",
            ObligationSeverity::Critical,
            "N41",
        )).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Fault, 1),
            ObligationKind::Primary,
            "Double vote detected and evidenced",
            "exists v : voted(v,b1) and voted(v,b2) and b1.height=b2.height implies detected(v)",
            ObligationSeverity::Critical,
            "N44",
        )).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Recovery, 1),
            ObligationKind::Primary,
            "Recovery preserves state root equivalence",
            "forall s : state_root(Recover(s)) = state_root(Original(s))",
            ObligationSeverity::Critical,
            "N42",
        )).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Performance, 1),
            ObligationKind::Primary,
            "TPS characterization",
            "TPS in [min, max] and TPS_measured_over(duration >= 60s)",
            ObligationSeverity::Minor,
            "N46",
        )).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Performance, 2),
            ObligationKind::Primary,
            "Latency percentiles documented",
            "p50, p95, p99 latency documented",
            ObligationSeverity::Minor,
            "N46",
        )).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Performance, 3),
            ObligationKind::Primary,
            "Resource utilization bounded",
            "CPU, MEM, IO bounded and documented",
            ObligationSeverity::Minor,
            "N46",
        )).unwrap();

        // Derived obligations
        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Safety, 4),
            ObligationKind::Derived,
            "Replay results must match finalized state",
            "forall b in Finalized : state_root(Replay(b)) = state_root(b)",
            ObligationSeverity::Critical,
            "N42",
        ).with_dependency(ObligationId::new(ObligationNamespace::Replay, 2))
         .with_dependency(ObligationId::new(ObligationNamespace::Evidence, 1))
        ).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Replay, 3),
            ObligationKind::Derived,
            "Replay certificates are unique",
            "forall b : |{rc : rc.block = b}| = 1",
            ObligationSeverity::Major,
            "N42",
        ).with_dependency(ObligationId::new(ObligationNamespace::Replay, 2))
        ).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Replay, 4),
            ObligationKind::Derived,
            "Replay chain is continuous",
            "forall b1,b2 in ReplayChain : b1.height+1=b2.height implies linked(b1,b2)",
            ObligationSeverity::Critical,
            "N42",
        ).with_dependency(ObligationId::new(ObligationNamespace::Replay, 1))
        ).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Evidence, 3),
            ObligationKind::Derived,
            "Evidence chain has no gaps",
            "forall ev1,ev2 in EvidenceChain : linked(ev1, ev2)",
            ObligationSeverity::Critical,
            "N42",
        ).with_dependency(ObligationId::new(ObligationNamespace::Evidence, 2))
        ).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Evidence, 4),
            ObligationKind::Derived,
            "Evidence hash lineage is preserved",
            "forall ev1,ev2 : ev2.parent = ev1 implies hash(ev1) in ev2.parent_hash",
            ObligationSeverity::Major,
            "N42",
        ).with_dependency(ObligationId::new(ObligationNamespace::Evidence, 3))
        ).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Finality, 3),
            ObligationKind::Derived,
            "Conflicting finality certificates are impossible",
            "not exists fc1,fc2 : fc1.block = fc2.block and fc1 != fc2",
            ObligationSeverity::Critical,
            "N44",
        ).with_dependency(ObligationId::new(ObligationNamespace::Finality, 1))
         .with_dependency(ObligationId::new(ObligationNamespace::Safety, 1))
        ).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Cluster, 1),
            ObligationKind::Derived,
            "Finalized = Replay Verified = Evidence Certified",
            "|Finalized| = |ReplayVerified| = |EvidenceCertified|",
            ObligationSeverity::Critical,
            "N43",
        ).with_dependency(ObligationId::new(ObligationNamespace::Finality, 1))
         .with_dependency(ObligationId::new(ObligationNamespace::Replay, 1))
         .with_dependency(ObligationId::new(ObligationNamespace::Evidence, 1))
        ).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Cluster, 2),
            ObligationKind::Derived,
            "Detected Divergence = Resolved Divergence",
            "divergence_detected = divergence_resolved",
            ObligationSeverity::Critical,
            "N43",
        ).with_dependency(ObligationId::new(ObligationNamespace::Cluster, 1))
        ).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Cluster, 3),
            ObligationKind::Derived,
            "Cluster invariants hold at all scales",
            "forall n in {32,64,128,256} : invariants_hold(n)",
            ObligationSeverity::Critical,
            "N43",
        ).with_dependency(ObligationId::new(ObligationNamespace::Cluster, 1))
         .with_dependency(ObligationId::new(ObligationNamespace::Cluster, 2))
        ).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Fault, 2),
            ObligationKind::Derived,
            "Conflicting QC rejected by honest nodes",
            "exists qc1,qc2 : qc1.block=qc2.block and qc1!=qc2 implies system_rejects",
            ObligationSeverity::Critical,
            "N44",
        ).with_dependency(ObligationId::new(ObligationNamespace::Finality, 3))
        ).unwrap();

        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Recovery, 2),
            ObligationKind::Derived,
            "Snapshot restores full evidence chain",
            "|EvidenceChain(Recovered)| = |EvidenceChain(Original)|",
            ObligationSeverity::Major,
            "N42",
        ).with_dependency(ObligationId::new(ObligationNamespace::Recovery, 1))
        ).unwrap();

        reg.freeze().unwrap();
        reg
    }

    pub fn verification_certificate_to_evidence(
        cert: &VerificationCertificate,
        phase: &str,
        obligation_id: &ObligationId,
    ) -> EvidenceRecord {
        let mut hasher = blake3::Hasher::new();
        hasher.update(cert.certificate_hash.as_bytes());
        hasher.update(cert.certificate_id.as_bytes());
        hasher.update(obligation_id.to_string().as_bytes());
        let data_hash = hex::encode(hasher.finalize().as_bytes());

        EvidenceRecord::new(
            format!("EV-{}-{}-{}", phase, cert.certificate_id, obligation_id),
            EvidenceType::CertificateEvidence,
            cert.verifier.clone(),
            cert.issued_at,
            data_hash,
            phase.to_string(),
            vec![obligation_id.clone()],
        )
        .with_status(EvidenceStatus::Verified)
    }

    pub fn run_full_pipeline(
        phase_certificates: HashMap<String, VerificationCertificate>,
        timestamp: u64,
    ) -> Result<(
        ObligationRegistry,
        EvidenceArchive,
        Vec<ConstitutionalVerdict>,
        ConstitutionalReport,
        PublicationPackage,
    ), String> {
        let registry = Self::build_obligation_registry();
        let mut archive = EvidenceArchive::new();
        for (phase, cert) in &phase_certificates {
            if !cert.verify() {
                return Err(format!("Certificate {} verification failed", cert.certificate_id));
            }
            let obligations: Vec<ProofObligation> = registry.all_obligations()
                .filter(|o| o.phase == phase.as_str())
                .cloned()
                .collect();
            for obl in &obligations {
                let ev = Self::verification_certificate_to_evidence(cert, phase, &obl.id);
                archive.insert(ev).map_err(|e| format!("insert error: {}", e))?;
            }
        }

        let mut verdicts = Vec::new();
        for phase in &["N41", "N42", "N43", "N44", "N45", "N46"] {
            let obligations: Vec<ProofObligation> = registry.all_obligations()
                .filter(|o| o.phase == *phase)
                .cloned()
                .collect();
            let mut results = Vec::new();
            for obl in &obligations {
                let ev = archive.by_obligation(&obl.id);
                let status = if ev.is_empty() { ObligationResultStatus::Inconclusive } else { ObligationResultStatus::Satisfied };
                let refs: Vec<String> = ev.iter().map(|e| e.evidence_id.clone()).collect();
                results.push(if status == ObligationResultStatus::Satisfied {
                    ObligationResult::satisfied(obl.id.clone(), refs)
                } else {
                    ObligationResult::inconclusive(obl.id.clone(), refs)
                });
            }
            let verdict = VerdictEvaluator::evaluate(
                format!("N47-V-{}-001", phase),
                format!("{}-Validation", phase),
                "phase_validation".into(),
                phase.to_string(),
                &obligations,
                results,
                timestamp,
                "N47-Bridge".into(),
            );
            verdicts.push(verdict);
        }

        let report = ReportGenerator::generate_report(&registry, &archive, verdicts.clone(), timestamp);
        let pkg = PublicationPackage::new("N47-PKG-001".into(), report.clone(), verdicts.clone(), timestamp);
        Ok((registry, archive, verdicts, report, pkg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_verification_kernel::{ConstitutionalClaim, ClaimType, ClaimStatus, Evidence as VKEvidence, EvidenceType as VKEvidenceType};

    fn make_cert(phase: &str) -> VerificationCertificate {
        VerificationCertificate::issue(
            phase,
            vec![ConstitutionalClaim {
                claim_id: format!("{}-C1", phase), claim_type: ClaimType::Safety,
                description: format!("Claim {}", phase), phase: phase.to_string(),
                evidence_refs: vec!["ev".into()], status: ClaimStatus::Proven,
            }],
            vec![VKEvidence {
                evidence_id: "ev".into(), claim_id: format!("{}-C1", phase),
                evidence_type: VKEvidenceType::TestResult,
                description: "ev".into(), data_hash: "hash".into(),
                source: "test".into(), timestamp: 1000,
            }],
            "verifier", 1000,
        )
    }

    #[test]
    fn n47_7_build_obligation_registry() {
        let reg = ConstitutionalBridge::build_obligation_registry();
        assert_eq!(reg.total(), 25);
        assert!(reg.is_frozen());
    }

    #[test]
    fn n47_7_run_full_pipeline() {
        let mut certs = HashMap::new();
        for p in &["N41","N42","N43","N44","N45","N46"] { certs.insert(p.to_string(), make_cert(p)); }
        let r = ConstitutionalBridge::run_full_pipeline(certs, 5000);
        assert!(r.is_ok());
        let (_, _, v, _, _) = r.unwrap();
        assert_eq!(v.len(), 6);
    }
}
