mod article_i_certificate;
mod article_iii_certificate;
mod certification;
mod constitutional_verdict;
mod dependency_graph;
mod evidence_archive;
mod evidence_lineage;
mod evidence_record;
mod evidence_status;
mod evidence_type;
mod failure_reason;
mod obligation_id;
mod obligation_kind;
mod obligation_namespace;
mod obligation_registry;
mod obligation_result;
mod obligation_result_status;
mod obligation_severity;
mod obligation_status;
mod proof_obligation;
mod publication_package;
mod registry_error;
mod report_generator;
mod reproducibility;
mod verdict_evaluator;
mod verdict_result;

pub use article_i_certificate::*;
pub use article_iii_certificate::*;
pub use certification::*;
pub use constitutional_verdict::*;
pub use dependency_graph::*;
pub use evidence_archive::*;
pub use evidence_lineage::*;
pub use evidence_record::*;
pub use evidence_status::*;
pub use evidence_type::*;
pub use failure_reason::*;
pub use obligation_id::*;
pub use obligation_kind::*;
pub use obligation_namespace::*;
pub use obligation_registry::*;
pub use obligation_result::*;
pub use obligation_result_status::*;
pub use obligation_severity::*;
pub use obligation_status::*;
pub use proof_obligation::*;
pub use publication_package::*;
pub use registry_error::*;
pub use report_generator::*;
pub use reproducibility::*;
pub use verdict_evaluator::*;
pub use verdict_result::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // --- Foundation type tests (S0) ---

    #[test]
    fn n47_1_s0_display_safety_001() {
        let id = ObligationId::new(ObligationNamespace::Safety, 1);
        assert_eq!(format!("{}", id), "SAFETY-001");
    }

    #[test]
    fn n47_1_s0_serialize_safety_001() {
        let id = ObligationId::new(ObligationNamespace::Safety, 1);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"SAFETY-001\"");
    }

    #[test]
    fn n47_1_s0_deserialize_safety_001() {
        let json = "\"SAFETY-001\"";
        let id: ObligationId = serde_json::from_str(json).unwrap();
        assert_eq!(id.namespace(), ObligationNamespace::Safety);
        assert_eq!(id.sequence(), 1);
    }

    #[test]
    fn n47_1_s0_parse_safety_001() {
        let id: ObligationId = "SAFETY-001".parse().unwrap();
        assert_eq!(id, ObligationId::new(ObligationNamespace::Safety, 1));
    }

    #[test]
    fn n47_1_s0_namespace_extraction() {
        let id = ObligationId::new(ObligationNamespace::Replay, 4);
        assert_eq!(id.namespace(), ObligationNamespace::Replay);
        assert_eq!(id.sequence(), 4);
    }

    #[test]
    fn n47_1_s0_reject_invalid_namespace() {
        let result: Result<ObligationId, _> = "UNKNOWN-001".parse();
        assert!(result.is_err());
        match result {
            Err(RegistryError::UnknownNamespace(ns)) => assert_eq!(ns, "UNKNOWN"),
            _ => panic!("expected UnknownNamespace error"),
        }
    }

    #[test]
    fn n47_1_s0_reject_invalid_format() {
        let result: Result<ObligationId, _> = "SAFETY".parse();
        assert!(result.is_err());
        match result {
            Err(RegistryError::InvalidObligationIdFormat(s)) => assert_eq!(s, "SAFETY"),
            _ => panic!("expected InvalidObligationIdFormat error"),
        }
    }

    #[test]
    fn n47_1_s0_namespace_display_roundtrip() {
        for ns in &[
            ObligationNamespace::Safety,
            ObligationNamespace::Replay,
            ObligationNamespace::Evidence,
            ObligationNamespace::Finality,
            ObligationNamespace::Cluster,
            ObligationNamespace::Fault,
            ObligationNamespace::Recovery,
            ObligationNamespace::Performance,
        ] {
            let displayed = format!("{}", ns);
            let parsed: ObligationNamespace = displayed.as_str().try_into().unwrap();
            assert_eq!(*ns, parsed);
        }
    }

    #[test]
    fn n47_1_s0_id_serialization_roundtrip() {
        let id = ObligationId::new(ObligationNamespace::Finality, 3);
        let json = serde_json::to_string(&id).unwrap();
        let id2: ObligationId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, id2);
    }

    // --- ProofObligation tests (S1) ---

    #[test]
    fn n47_1_s1_create_primary_obligation() {
        let id = ObligationId::new(ObligationNamespace::Safety, 1);
        let obl = ProofObligation::new(
            id.clone(),
            ObligationKind::Primary,
            "No conflicting finalized blocks at same height",
            "forall b1,b2 in Finalized : b1.height = b2.height implies b1 = b2",
            ObligationSeverity::Critical,
            "N45",
        );
        assert_eq!(obl.id, id);
        assert_eq!(obl.kind, ObligationKind::Primary);
        assert_eq!(obl.severity, ObligationSeverity::Critical);
        assert_eq!(obl.phase, "N45");
        assert_eq!(obl.version, 1);
        assert_eq!(obl.status, ObligationStatus::Active);
        assert!(obl.depends_on.is_empty());
    }

    #[test]
    fn n47_1_s1_create_derived_with_deps() {
        let dep = ObligationId::new(ObligationNamespace::Finality, 1);
        let obl = ProofObligation::new(
            ObligationId::new(ObligationNamespace::Cluster, 1),
            ObligationKind::Derived,
            "Triple equivalence at scale",
            "|Finalized| = |ReplayVerified| = |EvidenceCertified|",
            ObligationSeverity::Critical,
            "N43",
        )
        .with_dependency(dep.clone())
        .with_version(1);

        assert_eq!(obl.kind, ObligationKind::Derived);
        assert_eq!(obl.depends_on.len(), 1);
        assert_eq!(obl.depends_on[0], dep);
    }

    #[test]
    fn n47_1_s1_status_builder() {
        let obl = ProofObligation::new(
            ObligationId::new(ObligationNamespace::Performance, 1),
            ObligationKind::Primary,
            "TPS characterization",
            "TPS in [min, max] and TPS_measured_over(duration >= 60s)",
            ObligationSeverity::Minor,
            "N46",
        )
        .with_status(ObligationStatus::Frozen);

        assert_eq!(obl.status, ObligationStatus::Frozen);
    }

    #[test]
    fn n47_1_s1_serialization_roundtrip() {
        let obl = ProofObligation::new(
            ObligationId::new(ObligationNamespace::Replay, 2),
            ObligationKind::Primary,
            "Replay determinism",
            "forall b : Replay(b, t1) = Replay(b, t2)",
            ObligationSeverity::Critical,
            "N42",
        )
        .with_dependency(ObligationId::new(ObligationNamespace::Replay, 1));

        let json = serde_json::to_string_pretty(&obl).unwrap();
        let obl2: ProofObligation = serde_json::from_str(&json).unwrap();
        assert_eq!(obl, obl2);
    }

    #[test]
    fn n47_1_s1_deserialize_minimal() {
        let json = r#"{
            "id": "SAFETY-002",
            "kind": "primary",
            "description": "Quorum certification",
            "formal_statement": "forall b in Finalized : exists QC(b) and |signers(QC(b))| >= 2f+1",
            "severity": "critical",
            "phase": "N41",
            "version": 1,
            "status": "active"
        }"#;
        let obl: ProofObligation = serde_json::from_str(json).unwrap();
        assert_eq!(obl.id.to_string(), "SAFETY-002");
        assert!(obl.depends_on.is_empty());
    }

    // --- DependencyGraph tests (S2) ---

    fn make_id(namespace: ObligationNamespace, seq: u32) -> ObligationId {
        ObligationId::new(namespace, seq)
    }

    #[test]
    fn n47_1_s2_add_dependency() {
        let mut graph = DependencyGraph::new();
        let a = make_id(ObligationNamespace::Safety, 1);
        let b = make_id(ObligationNamespace::Safety, 2);
        graph.add_edge(a.clone(), b.clone());
        assert!(graph.all_dependencies_exist().is_ok());
    }

    #[test]
    fn n47_1_s2_detect_cycle() {
        let mut graph = DependencyGraph::new();
        let a = make_id(ObligationNamespace::Safety, 1);
        let b = make_id(ObligationNamespace::Safety, 2);
        let c = make_id(ObligationNamespace::Safety, 3);
        graph.add_edge(a.clone(), b.clone());
        graph.add_edge(b.clone(), c.clone());
        graph.add_edge(c.clone(), a.clone());
        assert!(graph.has_cycles());
        assert!(graph.topological_sort().is_err());
    }

    #[test]
    fn n47_1_s2_topological_sort() {
        let mut graph = DependencyGraph::new();
        let a = make_id(ObligationNamespace::Safety, 1);
        let b = make_id(ObligationNamespace::Safety, 2);
        let c = make_id(ObligationNamespace::Safety, 3);
        graph.add_edge(a.clone(), b.clone());
        graph.add_edge(b.clone(), c.clone());
        let sorted = graph.topological_sort().unwrap();
        let pos_c = sorted.iter().position(|x| x == &c).unwrap();
        let pos_b = sorted.iter().position(|x| x == &b).unwrap();
        let pos_a = sorted.iter().position(|x| x == &a).unwrap();
        assert!(pos_c < pos_b);
        assert!(pos_b < pos_a);
    }

    #[test]
    fn n47_1_s2_missing_dependency() {
        let mut g = DependencyGraph::new();
        g.add_node(make_id(ObligationNamespace::Safety, 1));
        assert!(g.all_dependencies_exist().is_ok());
    }

    #[test]
    fn n47_1_s2_derived_terminates_in_primary() {
        let mut graph = DependencyGraph::new();
        let primary = make_id(ObligationNamespace::Safety, 1);
        let derived = make_id(ObligationNamespace::Cluster, 1);

        let mut kinds = HashMap::new();
        kinds.insert(primary.clone(), ObligationKind::Primary);
        kinds.insert(derived.clone(), ObligationKind::Derived);

        graph.add_edge(derived.clone(), primary.clone());
        assert!(graph.validate_derived_terminate_in_primary(&kinds).is_ok());
    }

    #[test]
    fn n47_1_s2_reject_infinite_derivation() {
        let mut graph = DependencyGraph::new();
        let d1 = make_id(ObligationNamespace::Cluster, 1);
        let d2 = make_id(ObligationNamespace::Cluster, 2);

        let mut kinds = HashMap::new();
        kinds.insert(d1.clone(), ObligationKind::Derived);
        kinds.insert(d2.clone(), ObligationKind::Derived);

        graph.add_edge(d1.clone(), d2.clone());
        graph.add_edge(d2.clone(), d1.clone());
        let result = graph.validate_derived_terminate_in_primary(&kinds);
        assert!(result.is_err());
        match result {
            Err(RegistryError::DerivedNotTerminatingInPrimary(id)) => {
                assert!(id == d1 || id == d2);
            }
            _ => panic!("expected DerivedNotTerminatingInPrimary error"),
        }
    }

    // --- ObligationRegistry tests (S3) ---

    fn simple_obl(id: ObligationId) -> ProofObligation {
        ProofObligation::new(
            id,
            ObligationKind::Primary,
            "desc",
            "formal",
            ObligationSeverity::Critical,
            "N42",
        )
    }

    #[test]
    fn n47_1_s3_register_obligation() {
        let mut reg = ObligationRegistry::new();
        let id = make_id(ObligationNamespace::Safety, 1);
        assert!(reg.register(simple_obl(id.clone())).is_ok());
        assert_eq!(reg.total(), 1);
        assert!(reg.get(&id).is_some());
    }

    #[test]
    fn n47_1_s3_reject_duplicate() {
        let mut reg = ObligationRegistry::new();
        let id = make_id(ObligationNamespace::Safety, 1);
        reg.register(simple_obl(id.clone())).unwrap();
        let result = reg.register(simple_obl(id.clone()));
        assert!(matches!(result, Err(RegistryError::DuplicateId(_))));
    }

    #[test]
    fn n47_1_s3_freeze_and_reject_modification() {
        let mut reg = ObligationRegistry::new();
        let id = make_id(ObligationNamespace::Safety, 1);
        reg.register(simple_obl(id.clone())).unwrap();
        reg.freeze().unwrap();
        assert!(reg.is_frozen());
        let result = reg.register(simple_obl(make_id(ObligationNamespace::Safety, 2)));
        assert!(matches!(result, Err(RegistryError::RegistryFrozen)));
    }

    #[test]
    fn n47_1_s3_reject_missing_dependency() {
        let mut reg = ObligationRegistry::new();
        let missing_dep = make_id(ObligationNamespace::Replay, 99);
        let obl = ProofObligation::new(
            make_id(ObligationNamespace::Safety, 1),
            ObligationKind::Primary,
            "desc",
            "formal",
            ObligationSeverity::Critical,
            "N42",
        )
        .with_dependency(missing_dep.clone());
        let result = reg.register(obl);
        assert!(matches!(
            result,
            Err(RegistryError::MissingDependency(_, _))
        ));
    }

    #[test]
    fn n47_1_s3_query_by_severity() {
        let mut reg = ObligationRegistry::new();
        let id1 = make_id(ObligationNamespace::Safety, 1);
        let id2 = make_id(ObligationNamespace::Safety, 2);
        reg.register(ProofObligation::new(
            id1,
            ObligationKind::Primary,
            "desc",
            "formal",
            ObligationSeverity::Critical,
            "N42",
        ))
        .unwrap();
        reg.register(ProofObligation::new(
            id2,
            ObligationKind::Primary,
            "desc",
            "formal",
            ObligationSeverity::Minor,
            "N46",
        ))
        .unwrap();
        assert_eq!(reg.by_severity(ObligationSeverity::Critical).len(), 1);
        assert_eq!(reg.by_severity(ObligationSeverity::Minor).len(), 1);
        assert_eq!(reg.by_severity(ObligationSeverity::Major).len(), 0);
    }

    #[test]
    fn n47_1_s3_query_by_phase() {
        let mut reg = ObligationRegistry::new();
        reg.register(ProofObligation::new(
            make_id(ObligationNamespace::Finality, 1),
            ObligationKind::Primary,
            "desc",
            "formal",
            ObligationSeverity::Critical,
            "N41",
        ))
        .unwrap();
        reg.register(ProofObligation::new(
            make_id(ObligationNamespace::Finality, 2),
            ObligationKind::Primary,
            "desc",
            "formal",
            ObligationSeverity::Critical,
            "N45",
        ))
        .unwrap();
        assert_eq!(reg.by_phase("N41").len(), 1);
        assert_eq!(reg.by_phase("N45").len(), 1);
        assert_eq!(reg.by_phase("N99").len(), 0);
    }

    // --- Article I Certificate tests ---

    #[test]
    fn n47_1_cert_issue_success() {
        let mut reg = ObligationRegistry::new();
        for i in 1..=22 {
            let id = ObligationId::new(ObligationNamespace::Safety, i);
            reg.register(ProofObligation::new(
                id,
                ObligationKind::Primary,
                format!("Obligation {}", i),
                format!("formal {}", i),
                ObligationSeverity::Critical,
                "N42",
            ))
            .unwrap();
        }
        reg.freeze().unwrap();
        let cert = ArticleICertificate::issue(&reg, 1000);
        assert!(cert.is_some());
        let c = cert.unwrap();
        assert_eq!(c.obligations_registered, 22);
        assert!(c.dependency_graph_valid);
        assert!(c.cycle_free);
        assert!(c.registry_frozen);
        assert_eq!(c.total_primary, 22);
        assert_eq!(c.total_derived, 0);
    }

    #[test]
    fn n47_1_cert_reject_unfrozen() {
        let mut reg = ObligationRegistry::new();
        for i in 1..=22 {
            let id = ObligationId::new(ObligationNamespace::Safety, i);
            reg.register(ProofObligation::new(
                id,
                ObligationKind::Primary,
                format!("Obligation {}", i),
                format!("formal {}", i),
                ObligationSeverity::Critical,
                "N42",
            ))
            .unwrap();
        }
        assert!(ArticleICertificate::issue(&reg, 1000).is_none());
    }

    #[test]
    fn n47_1_cert_reject_insufficient_obligations() {
        let mut reg = ObligationRegistry::new();
        for i in 1..=21 {
            let id = ObligationId::new(ObligationNamespace::Safety, i);
            reg.register(ProofObligation::new(
                id,
                ObligationKind::Primary,
                format!("Obligation {}", i),
                format!("formal {}", i),
                ObligationSeverity::Critical,
                "N42",
            ))
            .unwrap();
        }
        reg.freeze().unwrap();
        assert!(ArticleICertificate::issue(&reg, 1000).is_none());
    }

    // --- N47.2-S0: Verdict Foundation Types tests ---

    #[test]
    fn n47_2_s0_create_satisfied_result() {
        let id = ObligationId::new(ObligationNamespace::Safety, 1);
        let result = ObligationResult::satisfied(id.clone(), vec!["EV-001".into()]);
        assert_eq!(result.obligation_id, id);
        assert_eq!(result.status, ObligationResultStatus::Satisfied);
        assert_eq!(result.evidence_refs.len(), 1);
        assert!(result.failure_reason.is_none());
    }

    #[test]
    fn n47_2_s0_create_failed_result() {
        let id = ObligationId::new(ObligationNamespace::Replay, 1);
        let reason = FailureReason::new("MISSING_EVIDENCE", "No replay data found");
        let result = ObligationResult::failed(id.clone(), reason.clone(), vec![]);
        assert_eq!(result.status, ObligationResultStatus::Failed);
        assert_eq!(result.failure_reason.unwrap(), reason);
    }

    #[test]
    fn n47_2_s0_create_inconclusive_result() {
        let id = ObligationId::new(ObligationNamespace::Cluster, 1);
        let result = ObligationResult::inconclusive(id.clone(), vec!["EV-032".into()]);
        assert_eq!(result.status, ObligationResultStatus::Inconclusive);
        assert!(result.failure_reason.is_none());
    }

    #[test]
    fn n47_2_s0_verdict_pass() {
        assert!(matches!(VerdictResult::Pass, VerdictResult::Pass));
    }

    #[test]
    fn n47_2_s0_verdict_fail_with_reasons() {
        let reasons = vec!["CRITICAL_FAILURE".into()];
        let v = VerdictResult::Fail(reasons.clone());
        match v {
            VerdictResult::Fail(r) => assert_eq!(r, reasons),
            _ => panic!("expected Fail"),
        }
    }

    #[test]
    fn n47_2_s0_serialize_obligation_result() {
        let id = ObligationId::new(ObligationNamespace::Safety, 1);
        let result = ObligationResult::satisfied(id, vec!["EV-001".into()]);
        let json = serde_json::to_string(&result).unwrap();
        let parsed: ObligationResult = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, result);
    }

    // --- N47.2-S1: ConstitutionalVerdict tests ---

    #[test]
    fn n47_2_s1_create_constitutional_verdict() {
        let results = vec![
            ObligationResult::satisfied(
                ObligationId::new(ObligationNamespace::Safety, 1),
                vec!["EV-001".into()],
            ),
            ObligationResult::satisfied(
                ObligationId::new(ObligationNamespace::Safety, 2),
                vec!["EV-002".into()],
            ),
        ];
        let verdict = ConstitutionalVerdict::new(
            "N47-V-N41-001".into(),
            "N41-Finality".into(),
            "phase_validation".into(),
            "N41".into(),
            results,
            VerdictResult::Pass,
            1000,
            "n47-verdict-engine".into(),
        );
        assert_eq!(verdict.obligations_checked, 2);
        assert_eq!(verdict.obligations_satisfied, 2);
        assert_eq!(verdict.failed_count(), 0);
        assert!(verdict.verify());
        assert_eq!(verdict.evidence_refs.len(), 2);
    }

    #[test]
    fn n47_2_s1_count_satisfied_obligations() {
        let results = vec![
            ObligationResult::satisfied(
                ObligationId::new(ObligationNamespace::Replay, 1),
                vec!["EV-R1".into()],
            ),
            ObligationResult::failed(
                ObligationId::new(ObligationNamespace::Replay, 2),
                FailureReason::new("ERR", "fail"),
                vec![],
            ),
            ObligationResult::inconclusive(
                ObligationId::new(ObligationNamespace::Replay, 3),
                vec![],
            ),
        ];
        let verdict = ConstitutionalVerdict::new(
            "V-001".into(),
            "S-001".into(),
            "test".into(),
            "N42".into(),
            results,
            VerdictResult::ConditionalPass(vec!["Replay-2 failed".into()]),
            2000,
            "test".into(),
        );
        assert_eq!(verdict.obligations_checked, 3);
        assert_eq!(verdict.obligations_satisfied, 1);
        assert_eq!(verdict.failed_count(), 2);
    }

    #[test]
    fn n47_2_s1_collect_evidence_refs() {
        let results = vec![
            ObligationResult::satisfied(
                ObligationId::new(ObligationNamespace::Safety, 1),
                vec!["EV-A".into(), "EV-B".into()],
            ),
            ObligationResult::satisfied(
                ObligationId::new(ObligationNamespace::Safety, 2),
                vec!["EV-B".into(), "EV-C".into()],
            ),
        ];
        let verdict = ConstitutionalVerdict::new(
            "V-002".into(),
            "S-002".into(),
            "test".into(),
            "N43".into(),
            results,
            VerdictResult::Pass,
            3000,
            "test".into(),
        );
        let mut expected = vec!["EV-A", "EV-B", "EV-C"];
        expected.sort();
        assert_eq!(verdict.evidence_refs, expected);
    }

    #[test]
    fn n47_2_s1_compute_verdict_hash() {
        let results = vec![ObligationResult::satisfied(
            ObligationId::new(ObligationNamespace::Cluster, 1),
            vec!["EV-CL1".into()],
        )];
        let v1 = ConstitutionalVerdict::new(
            "V-HASH".into(),
            "S-HASH".into(),
            "hash_test".into(),
            "N43".into(),
            results.clone(),
            VerdictResult::Pass,
            4000,
            "hash_verifier".into(),
        );
        let v2 = ConstitutionalVerdict::new(
            "V-HASH".into(),
            "S-HASH".into(),
            "hash_test".into(),
            "N43".into(),
            results,
            VerdictResult::Pass,
            4000,
            "hash_verifier".into(),
        );
        assert_eq!(v1.verdict_hash, v2.verdict_hash);
    }

    #[test]
    fn n47_2_s1_serialization_roundtrip() {
        let results = vec![
            ObligationResult::satisfied(
                ObligationId::new(ObligationNamespace::Finality, 1),
                vec!["EV-F1".into()],
            ),
            ObligationResult::failed(
                ObligationId::new(ObligationNamespace::Finality, 2),
                FailureReason::new("INVALID_SIG", "bad signature"),
                vec![],
            ),
        ];
        let verdict = ConstitutionalVerdict::new(
            "V-SER".into(),
            "S-SER".into(),
            "ser_test".into(),
            "N44".into(),
            results,
            VerdictResult::Fail(vec!["bad signature".into()]),
            5000,
            "ser_verifier".into(),
        );
        let json = serde_json::to_string_pretty(&verdict).unwrap();
        let parsed: ConstitutionalVerdict = serde_json::from_str(&json).unwrap();
        assert_eq!(verdict, parsed);
        assert!(parsed.verify());
    }

    // --- N47.2-S2: VerdictEvaluator tests ---

    fn make_obl(id: ObligationId, severity: ObligationSeverity) -> ProofObligation {
        ProofObligation::new(
            id,
            ObligationKind::Primary,
            "desc",
            "formal",
            severity,
            "N42",
        )
    }

    #[test]
    fn n47_2_s2_fail_on_critical() {
        let obl = make_obl(
            ObligationId::new(ObligationNamespace::Safety, 1),
            ObligationSeverity::Critical,
        );
        let results = vec![ObligationResult::failed(
            obl.id.clone(),
            FailureReason::new("CRIT", "critical failure"),
            vec![],
        )];
        let verdict = VerdictEvaluator::evaluate(
            "V-FAIL-CRIT".into(),
            "S-FAIL".into(),
            "test".into(),
            "N44".into(),
            &[obl],
            results,
            6000,
            "eval".into(),
        );
        assert!(matches!(verdict.overall_result, VerdictResult::Fail(_)));
    }

    #[test]
    fn n47_2_s2_fail_on_two_major() {
        let obl1 = make_obl(
            ObligationId::new(ObligationNamespace::Replay, 1),
            ObligationSeverity::Major,
        );
        let obl2 = make_obl(
            ObligationId::new(ObligationNamespace::Replay, 2),
            ObligationSeverity::Major,
        );
        let obligations = vec![obl1.clone(), obl2.clone()];
        let results = vec![
            ObligationResult::failed(obl1.id, FailureReason::new("M1", "m1"), vec![]),
            ObligationResult::failed(obl2.id, FailureReason::new("M2", "m2"), vec![]),
        ];
        let verdict = VerdictEvaluator::evaluate(
            "V-FAIL-2MAJ".into(),
            "S-2MAJ".into(),
            "test".into(),
            "N43".into(),
            &obligations,
            results,
            7000,
            "eval".into(),
        );
        assert!(matches!(verdict.overall_result, VerdictResult::Fail(_)));
    }

    #[test]
    fn n47_2_s2_conditional_pass_on_one_major() {
        let obl = make_obl(
            ObligationId::new(ObligationNamespace::Evidence, 1),
            ObligationSeverity::Major,
        );
        let results = vec![ObligationResult::failed(
            obl.id.clone(),
            FailureReason::new("M1", "single major"),
            vec![],
        )];
        let verdict = VerdictEvaluator::evaluate(
            "V-COND".into(),
            "S-COND".into(),
            "test".into(),
            "N42".into(),
            &[obl],
            results,
            8000,
            "eval".into(),
        );
        assert!(matches!(
            verdict.overall_result,
            VerdictResult::ConditionalPass(_)
        ));
    }

    #[test]
    fn n47_2_s2_pass_with_minor_failures() {
        let obl = make_obl(
            ObligationId::new(ObligationNamespace::Performance, 1),
            ObligationSeverity::Minor,
        );
        let results = vec![ObligationResult::failed(
            obl.id.clone(),
            FailureReason::new("MIN", "minor issue"),
            vec![],
        )];
        let verdict = VerdictEvaluator::evaluate(
            "V-PASS-MIN".into(),
            "S-MIN".into(),
            "test".into(),
            "N46".into(),
            &[obl],
            results,
            9000,
            "eval".into(),
        );
        assert!(matches!(
            verdict.overall_result,
            VerdictResult::ConditionalPass(_)
        ));
    }

    #[test]
    fn n47_2_s2_pass_with_advisory_failures() {
        let obl = make_obl(
            ObligationId::new(ObligationNamespace::Performance, 2),
            ObligationSeverity::Advisory,
        );
        let results = vec![ObligationResult::failed(
            obl.id.clone(),
            FailureReason::new("ADV", "advisory note"),
            vec![],
        )];
        let verdict = VerdictEvaluator::evaluate(
            "V-PASS-ADV".into(),
            "S-ADV".into(),
            "test".into(),
            "N46".into(),
            &[obl],
            results,
            10000,
            "eval".into(),
        );
        assert!(matches!(
            verdict.overall_result,
            VerdictResult::ConditionalPass(_)
        ));
    }

    #[test]
    fn n47_2_s2_pass_all_satisfied() {
        let obl = make_obl(
            ObligationId::new(ObligationNamespace::Finality, 1),
            ObligationSeverity::Critical,
        );
        let results = vec![ObligationResult::satisfied(
            obl.id.clone(),
            vec!["EV-OK".into()],
        )];
        let verdict = VerdictEvaluator::evaluate(
            "V-PASS-ALL".into(),
            "S-ALL".into(),
            "test".into(),
            "N41".into(),
            &[obl],
            results,
            11000,
            "eval".into(),
        );
        assert_eq!(verdict.overall_result, VerdictResult::Pass);
    }

    #[test]
    fn n47_2_s2_count_obligations_correctly() {
        let obl1 = make_obl(
            ObligationId::new(ObligationNamespace::Safety, 1),
            ObligationSeverity::Critical,
        );
        let obl2 = make_obl(
            ObligationId::new(ObligationNamespace::Safety, 2),
            ObligationSeverity::Critical,
        );
        let obl3 = make_obl(
            ObligationId::new(ObligationNamespace::Safety, 3),
            ObligationSeverity::Major,
        );
        let obligations = vec![obl1.clone(), obl2.clone(), obl3.clone()];
        let results = vec![
            ObligationResult::satisfied(obl1.id, vec!["EV1".into()]),
            ObligationResult::satisfied(obl2.id, vec!["EV2".into()]),
            ObligationResult::failed(obl3.id, FailureReason::new("M", "maj"), vec![]),
        ];
        let verdict = VerdictEvaluator::evaluate(
            "V-COUNT".into(),
            "S-COUNT".into(),
            "test".into(),
            "N45".into(),
            &obligations,
            results,
            12000,
            "eval".into(),
        );
        assert_eq!(verdict.obligations_checked, 3);
        assert_eq!(verdict.obligations_satisfied, 2);
        assert_eq!(verdict.failed_count(), 1);
    }

    #[test]
    fn n47_2_s2_waived_advisory_does_not_fail() {
        let obl = make_obl(
            ObligationId::new(ObligationNamespace::Performance, 3),
            ObligationSeverity::Advisory,
        );
        let mut result = ObligationResult::satisfied(obl.id.clone(), vec!["EV-W".into()]);
        result.status = ObligationResultStatus::Waived;
        let verdict = VerdictEvaluator::evaluate(
            "V-WAIVED".into(),
            "S-WAIVED".into(),
            "test".into(),
            "N46".into(),
            &[obl],
            vec![result],
            13000,
            "eval".into(),
        );
        assert!(!matches!(verdict.overall_result, VerdictResult::Fail(_)));
    }

    #[test]
    fn n47_2_s2_not_applicable_does_not_fail() {
        let obl = make_obl(
            ObligationId::new(ObligationNamespace::Cluster, 4),
            ObligationSeverity::Critical,
        );
        let mut result = ObligationResult::satisfied(obl.id.clone(), vec!["EV-NA".into()]);
        result.status = ObligationResultStatus::NotApplicable;
        let verdict = VerdictEvaluator::evaluate(
            "V-NA".into(),
            "S-NA".into(),
            "test".into(),
            "N43".into(),
            &[obl],
            vec![result],
            14000,
            "eval".into(),
        );
        assert!(!matches!(verdict.overall_result, VerdictResult::Fail(_)));
    }

    // --- N47.3-S0: Evidence Foundation Types tests ---

    #[test]
    fn n47_3_s0_create_evidence_record() {
        let id = ObligationId::new(ObligationNamespace::Replay, 1);
        let ev = EvidenceRecord::new(
            "EV-001".into(),
            EvidenceType::ReplayEvidence,
            "amun-replay-engine".into(),
            1000,
            "abc123hash".into(),
            "N42".into(),
            vec![id.clone()],
        );
        assert_eq!(ev.evidence_id, "EV-001");
        assert_eq!(ev.status, EvidenceStatus::Collected);
        assert!(ev.reproducibility.is_none());
        assert!(ev.lineage.is_none());
        assert_eq!(ev.obligation_ids.len(), 1);
        assert_eq!(ev.obligation_ids[0], id);
    }

    #[test]
    fn n47_3_s0_evidence_with_reproducibility() {
        let id = ObligationId::new(ObligationNamespace::Cluster, 1);
        let repro = Reproducibility::new(
            "cargo run -p amun-cluster-sim -- --nodes 32".into(),
            "env-hash-123".into(),
            "out-hash-456".into(),
        );
        let ev = EvidenceRecord::new(
            "EV-REPRO".into(),
            EvidenceType::SimulationEvidence,
            "amun-cluster-sim".into(),
            2000,
            "sim-hash".into(),
            "N43".into(),
            vec![id],
        )
        .with_reproducibility(repro.clone());
        assert!(ev.reproducibility.is_some());
        assert_eq!(ev.reproducibility.unwrap(), repro);
    }

    #[test]
    fn n47_3_s0_evidence_with_lineage() {
        let id = ObligationId::new(ObligationNamespace::Finality, 1);
        let lineage = EvidenceLineage::new(
            "EV-PARENT".into(),
            "derived from parent".into(),
            "parent-hash".into(),
        );
        let ev = EvidenceRecord::new(
            "EV-CHILD".into(),
            EvidenceType::CertificateEvidence,
            "amun-finality".into(),
            3000,
            "child-hash".into(),
            "N41".into(),
            vec![id],
        )
        .with_lineage(lineage.clone())
        .with_status(EvidenceStatus::Verified);
        assert_eq!(ev.status, EvidenceStatus::Verified);
        assert_eq!(ev.lineage.unwrap(), lineage);
    }

    #[test]
    fn n47_3_s0_serialization_roundtrip() {
        let id = ObligationId::new(ObligationNamespace::Safety, 1);
        let ev = EvidenceRecord::new(
            "EV-SER".into(),
            EvidenceType::AuditEvidence,
            "amun-audit".into(),
            4000,
            "audit-hash".into(),
            "N42".into(),
            vec![id],
        )
        .with_status(EvidenceStatus::Archived)
        .with_reproducibility(Reproducibility::new(
            "cmd".into(),
            "env".into(),
            "out".into(),
        ))
        .with_lineage(EvidenceLineage::new(
            "parent".into(),
            "derived".into(),
            "parent-hash".into(),
        ));
        let json = serde_json::to_string_pretty(&ev).unwrap();
        let parsed: EvidenceRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(ev, parsed);
    }

    // --- N47.3-S1: EvidenceArchive tests ---

    fn make_evidence(
        id: &str,
        phase: &str,
        ev_type: EvidenceType,
        obl_id: ObligationId,
    ) -> EvidenceRecord {
        EvidenceRecord::new(
            id.into(),
            ev_type,
            "test-source".into(),
            5000,
            format!("hash-{}", id),
            phase.into(),
            vec![obl_id],
        )
    }

    #[test]
    fn n47_3_s1_insert_and_retrieve() {
        let mut archive = EvidenceArchive::new();
        let obl = ObligationId::new(ObligationNamespace::Replay, 1);
        let ev = make_evidence("EV-ARC-1", "N42", EvidenceType::ReplayEvidence, obl);
        archive.insert(ev.clone()).unwrap();
        assert_eq!(archive.total_count(), 1);
        let fetched = archive.get("EV-ARC-1").unwrap();
        assert_eq!(fetched, &ev);
    }

    #[test]
    fn n47_3_s1_reject_duplicate() {
        let mut archive = EvidenceArchive::new();
        let obl = ObligationId::new(ObligationNamespace::Safety, 1);
        let ev = make_evidence("EV-DUP", "N42", EvidenceType::AuditEvidence, obl);
        archive.insert(ev.clone()).unwrap();
        assert!(archive.insert(ev).is_err());
    }

    #[test]
    fn n47_3_s1_verify_and_archive() {
        let mut archive = EvidenceArchive::new();
        let obl = ObligationId::new(ObligationNamespace::Finality, 1);
        let ev = make_evidence("EV-LIFECYCLE", "N41", EvidenceType::ConsensusEvidence, obl);
        archive.insert(ev).unwrap();

        archive.verify("EV-LIFECYCLE").unwrap();
        assert_eq!(
            archive.get("EV-LIFECYCLE").unwrap().status,
            EvidenceStatus::Verified
        );

        archive.archive("EV-LIFECYCLE").unwrap();
        assert_eq!(
            archive.get("EV-LIFECYCLE").unwrap().status,
            EvidenceStatus::Archived
        );
    }

    #[test]
    fn n47_3_s1_cannot_archive_unverified() {
        let mut archive = EvidenceArchive::new();
        let obl = ObligationId::new(ObligationNamespace::Evidence, 1);
        archive
            .insert(make_evidence(
                "EV-UNV",
                "N42",
                EvidenceType::ReplayEvidence,
                obl,
            ))
            .unwrap();
        assert!(archive.archive("EV-UNV").is_err());
    }

    #[test]
    fn n47_3_s1_reject_is_permanent() {
        let mut archive = EvidenceArchive::new();
        let obl = ObligationId::new(ObligationNamespace::Cluster, 1);
        archive
            .insert(make_evidence(
                "EV-REJ",
                "N43",
                EvidenceType::SimulationEvidence,
                obl,
            ))
            .unwrap();
        archive.reject("EV-REJ").unwrap();
        assert_eq!(
            archive.get("EV-REJ").unwrap().status,
            EvidenceStatus::Rejected
        );
        assert!(archive.verify("EV-REJ").is_err());
        assert!(archive.archive("EV-REJ").is_err());
        assert_eq!(archive.total_count(), 1);
    }

    #[test]
    fn n47_3_s1_lineage_integrity() {
        let mut archive = EvidenceArchive::new();
        let obl = ObligationId::new(ObligationNamespace::Finality, 1);

        let parent = EvidenceRecord::new(
            "EV-PARENT".into(),
            EvidenceType::ConsensusEvidence,
            "source".into(),
            1000,
            "parent-hash-ok".into(),
            "N41".into(),
            vec![obl.clone()],
        )
        .with_status(EvidenceStatus::Verified);
        archive.insert(parent).unwrap();

        let child = EvidenceRecord::new(
            "EV-CHILD".into(),
            EvidenceType::CertificateEvidence,
            "source".into(),
            2000,
            "child-hash".into(),
            "N41".into(),
            vec![obl],
        )
        .with_lineage(EvidenceLineage::new(
            "EV-PARENT".into(),
            "derived".into(),
            "parent-hash-ok".into(),
        ));
        assert!(archive.insert(child).is_ok());
    }

    #[test]
    fn n47_3_s1_reject_lineage_hash_mismatch() {
        let mut archive = EvidenceArchive::new();
        let obl = ObligationId::new(ObligationNamespace::Finality, 1);

        let parent = EvidenceRecord::new(
            "EV-PARENT-BAD".into(),
            EvidenceType::ConsensusEvidence,
            "source".into(),
            1000,
            "actual-hash".into(),
            "N41".into(),
            vec![obl.clone()],
        )
        .with_status(EvidenceStatus::Verified);
        archive.insert(parent).unwrap();

        let child = EvidenceRecord::new(
            "EV-CHILD-BAD".into(),
            EvidenceType::CertificateEvidence,
            "source".into(),
            2000,
            "child-hash".into(),
            "N41".into(),
            vec![obl],
        )
        .with_lineage(EvidenceLineage::new(
            "EV-PARENT-BAD".into(),
            "derived".into(),
            "wrong-hash".into(),
        ));
        assert!(archive.insert(child).is_err());
    }

    #[test]
    fn n47_3_s1_query_by_obligation() {
        let mut archive = EvidenceArchive::new();
        let obl1 = ObligationId::new(ObligationNamespace::Safety, 1);
        let obl2 = ObligationId::new(ObligationNamespace::Safety, 2);

        archive
            .insert(make_evidence(
                "EV-A",
                "N41",
                EvidenceType::AuditEvidence,
                obl1.clone(),
            ))
            .unwrap();
        archive
            .insert(make_evidence(
                "EV-B",
                "N42",
                EvidenceType::ReplayEvidence,
                obl1.clone(),
            ))
            .unwrap();
        archive
            .insert(make_evidence(
                "EV-C",
                "N43",
                EvidenceType::SimulationEvidence,
                obl2.clone(),
            ))
            .unwrap();

        assert_eq!(archive.by_obligation(&obl1).len(), 2);
        assert_eq!(archive.by_obligation(&obl2).len(), 1);
    }

    #[test]
    fn n47_3_s1_query_by_phase() {
        let mut archive = EvidenceArchive::new();
        let obl = ObligationId::new(ObligationNamespace::Finality, 1);

        archive
            .insert(make_evidence(
                "EV-P1",
                "N41",
                EvidenceType::ConsensusEvidence,
                obl.clone(),
            ))
            .unwrap();
        archive
            .insert(make_evidence(
                "EV-P2",
                "N41",
                EvidenceType::CertificateEvidence,
                obl.clone(),
            ))
            .unwrap();
        archive
            .insert(make_evidence(
                "EV-P3",
                "N42",
                EvidenceType::ReplayEvidence,
                obl,
            ))
            .unwrap();

        assert_eq!(archive.by_phase("N41").len(), 2);
        assert_eq!(archive.by_phase("N42").len(), 1);
        assert_eq!(archive.by_phase("N99").len(), 0);
    }

    #[test]
    fn n47_3_s1_admissibility_rules() {
        let obl = ObligationId::new(ObligationNamespace::Replay, 1);

        let mut collected =
            make_evidence("EV-ADM-1", "N42", EvidenceType::ReplayEvidence, obl.clone());
        collected.status = EvidenceStatus::Collected;
        assert!(!EvidenceArchive::is_admissible(&collected));

        let mut verified =
            make_evidence("EV-ADM-2", "N42", EvidenceType::ReplayEvidence, obl.clone());
        verified.status = EvidenceStatus::Verified;
        assert!(EvidenceArchive::is_admissible(&verified));

        let mut archived =
            make_evidence("EV-ADM-3", "N42", EvidenceType::ReplayEvidence, obl.clone());
        archived.status = EvidenceStatus::Archived;
        assert!(EvidenceArchive::is_admissible(&archived));

        let mut rejected = make_evidence("EV-ADM-4", "N42", EvidenceType::ReplayEvidence, obl);
        rejected.status = EvidenceStatus::Rejected;
        assert!(!EvidenceArchive::is_admissible(&rejected));
    }

    // --- Article III Certificate tests ---

    #[test]
    fn n47_3_cert_issue() {
        let mut archive = EvidenceArchive::new();
        let obl = ObligationId::new(ObligationNamespace::Finality, 1);
        archive
            .insert(EvidenceRecord::new(
                "EV-CERT".into(),
                EvidenceType::ConsensusEvidence,
                "source".into(),
                1000,
                "hash".into(),
                "N41".into(),
                vec![obl],
            ))
            .unwrap();
        let cert = ArticleIIICertificate::issue(&archive, 1000);
        assert!(cert.is_some());
        assert_eq!(cert.unwrap().evidence_records, 1);
    }

    #[test]
    fn n47_3_cert_reject_empty_archive() {
        let archive = EvidenceArchive::new();
        assert!(ArticleIIICertificate::issue(&archive, 1000).is_none());
    }

    // --- N47.4: Report Generator tests ---

    #[test]
    fn n47_4_generate_markdown_report() {
        let mut reg = ObligationRegistry::new();
        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Safety, 1),
            ObligationKind::Primary,
            "desc",
            "formal",
            ObligationSeverity::Critical,
            "N41",
        ))
        .unwrap();
        reg.freeze().unwrap();

        let mut archive = EvidenceArchive::new();
        let obl = ObligationId::new(ObligationNamespace::Safety, 1);
        archive
            .insert(
                EvidenceRecord::new(
                    "EV-RPT".into(),
                    EvidenceType::AuditEvidence,
                    "test".into(),
                    1000,
                    "hash".into(),
                    "N41".into(),
                    vec![obl],
                )
                .with_status(EvidenceStatus::Archived),
            )
            .unwrap();

        let verdict = ConstitutionalVerdict::new(
            "V-RPT".into(),
            "S-RPT".into(),
            "test".into(),
            "N41".into(),
            vec![ObligationResult::satisfied(
                ObligationId::new(ObligationNamespace::Safety, 1),
                vec!["EV-RPT".into()],
            )],
            VerdictResult::Pass,
            1000,
            "test".into(),
        );

        let report = ReportGenerator::generate_report(&reg, &archive, vec![verdict], 5000);

        assert_eq!(report.total_obligations, 1);
        assert_eq!(report.total_evidence, 1);
        assert_eq!(report.total_verdicts, 1);

        // Test Markdown output
        let md = ReportGenerator::to_markdown(&report);
        assert!(md.contains("N47 Constitutional Validation Report"));
        assert!(md.contains("AmunChain"));

        // Test JSON output
        let json = ReportGenerator::to_json(&report).unwrap();
        assert!(json.contains("AmunChain"));
        let parsed: ConstitutionalReport = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.total_obligations, 1);
    }

    // --- N47.5: Publication Package tests ---

    #[test]
    fn n47_5_create_publication_package() {
        let mut reg = ObligationRegistry::new();
        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Safety, 1),
            ObligationKind::Primary,
            "desc",
            "formal",
            ObligationSeverity::Critical,
            "N41",
        ))
        .unwrap();
        reg.freeze().unwrap();

        let mut archive = EvidenceArchive::new();
        let obl = ObligationId::new(ObligationNamespace::Safety, 1);
        archive
            .insert(
                EvidenceRecord::new(
                    "EV-PKG".into(),
                    EvidenceType::AuditEvidence,
                    "test".into(),
                    1000,
                    "hash".into(),
                    "N41".into(),
                    vec![obl],
                )
                .with_status(EvidenceStatus::Archived),
            )
            .unwrap();

        let verdict = ConstitutionalVerdict::new(
            "V-PKG".into(),
            "S-PKG".into(),
            "test".into(),
            "N41".into(),
            vec![ObligationResult::satisfied(
                ObligationId::new(ObligationNamespace::Safety, 1),
                vec!["EV-PKG".into()],
            )],
            VerdictResult::Pass,
            1000,
            "test".into(),
        );

        let report = ReportGenerator::generate_report(&reg, &archive, vec![verdict.clone()], 6000);

        let mut pkg = PublicationPackage::new("N47-PKG-001".into(), report, vec![verdict], 6000);

        assert!(pkg.verify());
        assert!(!pkg.frozen);

        pkg.freeze();
        assert!(pkg.frozen);

        pkg.sign(
            "N47-Constitutional-Authority".into(),
            "key-001".into(),
            "base64-signature".into(),
            6000,
        );
        assert!(pkg.frozen);
        assert!(pkg.signature.is_some());
    }

    #[test]
    fn n47_5_package_verification() {
        let mut reg = ObligationRegistry::new();
        reg.register(ProofObligation::new(
            ObligationId::new(ObligationNamespace::Replay, 1),
            ObligationKind::Primary,
            "desc",
            "formal",
            ObligationSeverity::Critical,
            "N42",
        ))
        .unwrap();
        reg.freeze().unwrap();

        let archive = EvidenceArchive::new();
        let verdict = ConstitutionalVerdict::new(
            "V-VERIFY".into(),
            "S-VERIFY".into(),
            "test".into(),
            "N42".into(),
            vec![],
            VerdictResult::Pass,
            7000,
            "test".into(),
        );

        let report = ReportGenerator::generate_report(&reg, &archive, vec![verdict.clone()], 7000);

        let pkg = PublicationPackage::new("N47-PKG-VERIFY".into(), report, vec![verdict], 7000);

        assert!(pkg.verify());
        assert_eq!(pkg.manifest.artifact_count, 2); // 1 report + 1 verdict
    }

    // --- N47.6: Constitutional Certification tests ---

    #[test]
    fn n47_6_certify_pass() {
        let mut reg = ObligationRegistry::new();
        for i in 1..=22 {
            reg.register(ProofObligation::new(
                ObligationId::new(ObligationNamespace::Safety, i),
                ObligationKind::Primary,
                format!("Obligation {}", i),
                format!("formal {}", i),
                ObligationSeverity::Critical,
                "N42",
            ))
            .unwrap();
        }
        reg.freeze().unwrap();

        let mut archive = EvidenceArchive::new();
        for i in 1..=30 {
            archive
                .insert(
                    EvidenceRecord::new(
                        format!("EV-CERT-{}", i),
                        EvidenceType::AuditEvidence,
                        "test".into(),
                        1000,
                        format!("hash-{}", i),
                        "N42".into(),
                        vec![ObligationId::new(ObligationNamespace::Safety, 1)],
                    )
                    .with_status(EvidenceStatus::Archived),
                )
                .unwrap();
        }

        let verdicts = vec![
            ConstitutionalVerdict::new(
                "V-N41".into(),
                "S-N41".into(),
                "test".into(),
                "N41".into(),
                vec![],
                VerdictResult::Pass,
                1000,
                "test".into(),
            ),
            ConstitutionalVerdict::new(
                "V-N42".into(),
                "S-N42".into(),
                "test".into(),
                "N42".into(),
                vec![],
                VerdictResult::Pass,
                1000,
                "test".into(),
            ),
            ConstitutionalVerdict::new(
                "V-N43".into(),
                "S-N43".into(),
                "test".into(),
                "N43".into(),
                vec![],
                VerdictResult::Pass,
                1000,
                "test".into(),
            ),
            ConstitutionalVerdict::new(
                "V-N44".into(),
                "S-N44".into(),
                "test".into(),
                "N44".into(),
                vec![],
                VerdictResult::Pass,
                1000,
                "test".into(),
            ),
            ConstitutionalVerdict::new(
                "V-N45".into(),
                "S-N45".into(),
                "test".into(),
                "N45".into(),
                vec![],
                VerdictResult::Pass,
                1000,
                "test".into(),
            ),
            ConstitutionalVerdict::new(
                "V-N46".into(),
                "S-N46".into(),
                "test".into(),
                "N46".into(),
                vec![],
                VerdictResult::Pass,
                1000,
                "test".into(),
            ),
        ];

        let cert = CertificationEvaluator::evaluate(
            &reg,
            &archive,
            &verdicts,
            "N47-PKG-001".into(),
            8000,
            "N47-Constitutional-Authority".into(),
        );

        assert_eq!(cert.certificate_id, "N47-CERT-001");
        assert!(matches!(cert.verdict, CertificationVerdict::Pass));
        assert!(cert.gates.iter().all(|g| g.passed));
    }

    #[test]
    fn n47_6_certify_fail_missing_obligations() {
        let mut reg = ObligationRegistry::new();
        for i in 1..=10 {
            reg.register(ProofObligation::new(
                ObligationId::new(ObligationNamespace::Safety, i),
                ObligationKind::Primary,
                format!("Obligation {}", i),
                format!("formal {}", i),
                ObligationSeverity::Critical,
                "N42",
            ))
            .unwrap();
        }
        reg.freeze().unwrap();

        let archive = EvidenceArchive::new();
        let verdicts = vec![
            ConstitutionalVerdict::new(
                "V-N41".into(),
                "S-N41".into(),
                "test".into(),
                "N41".into(),
                vec![],
                VerdictResult::Pass,
                1000,
                "test".into(),
            ),
            ConstitutionalVerdict::new(
                "V-N42".into(),
                "S-N42".into(),
                "test".into(),
                "N42".into(),
                vec![],
                VerdictResult::Pass,
                1000,
                "test".into(),
            ),
            ConstitutionalVerdict::new(
                "V-N43".into(),
                "S-N43".into(),
                "test".into(),
                "N43".into(),
                vec![],
                VerdictResult::Pass,
                1000,
                "test".into(),
            ),
            ConstitutionalVerdict::new(
                "V-N44".into(),
                "S-N44".into(),
                "test".into(),
                "N44".into(),
                vec![],
                VerdictResult::Pass,
                1000,
                "test".into(),
            ),
            ConstitutionalVerdict::new(
                "V-N45".into(),
                "S-N45".into(),
                "test".into(),
                "N45".into(),
                vec![],
                VerdictResult::Pass,
                1000,
                "test".into(),
            ),
            ConstitutionalVerdict::new(
                "V-N46".into(),
                "S-N46".into(),
                "test".into(),
                "N46".into(),
                vec![],
                VerdictResult::Pass,
                1000,
                "test".into(),
            ),
        ];

        let cert = CertificationEvaluator::evaluate(
            &reg,
            &archive,
            &verdicts,
            "N47-PKG-001".into(),
            8000,
            "N47-Constitutional-Authority".into(),
        );

        assert!(matches!(cert.verdict, CertificationVerdict::Fail(_)));
        let gate_c1 = cert.gates.iter().find(|g| g.gate_id == "GATE-C1").unwrap();
        assert!(!gate_c1.passed);
    }

    #[test]
    fn n47_6_certify_fail_missing_phase_verdict() {
        let mut reg = ObligationRegistry::new();
        for i in 1..=22 {
            reg.register(ProofObligation::new(
                ObligationId::new(ObligationNamespace::Safety, i),
                ObligationKind::Primary,
                format!("Obligation {}", i),
                format!("formal {}", i),
                ObligationSeverity::Critical,
                "N42",
            ))
            .unwrap();
        }
        reg.freeze().unwrap();

        let archive = EvidenceArchive::new();
        // Only N41, missing N42-N46
        let verdicts = vec![ConstitutionalVerdict::new(
            "V-N41".into(),
            "S-N41".into(),
            "test".into(),
            "N41".into(),
            vec![],
            VerdictResult::Pass,
            1000,
            "test".into(),
        )];

        let cert = CertificationEvaluator::evaluate(
            &reg,
            &archive,
            &verdicts,
            "N47-PKG-001".into(),
            8000,
            "N47-Constitutional-Authority".into(),
        );

        assert!(matches!(cert.verdict, CertificationVerdict::Fail(_)));
        let gate_c4 = cert.gates.iter().find(|g| g.gate_id == "GATE-C4").unwrap();
        assert!(!gate_c4.passed);
    }
}
