use amun_constitution_builder::digest::ArtifactDigest;
use amun_constitutional_signing::ConstitutionalKeyPair;
use amun_constitutional_governance::capability::Capability;
use amun_constitutional_governance::delegation::{self, DelegateCertificate};
use amun_constitutional_governance::quorum::QuorumPolicy;
use amun_constitutional_governance::voting::{Proposal, Ballot, Tally};
use amun_constitutional_governance::amendment::AmendmentLifecycle;

#[test]
fn capability_has_stable_id() {
    let key = ConstitutionalKeyPair::generate();
    let cap1 = Capability::new(
        "vote".into(), "governance".into(),
        key.verifying_key_hex(),
        "2026-01-01T00:00:00Z".into(), "2030-01-01T00:00:00Z".into(),
        serde_json::json!({"max_delegations": 3}),
    );
    let cap2 = Capability::new(
        "vote".into(), "governance".into(),
        key.verifying_key_hex(),
        "2026-01-01T00:00:00Z".into(), "2030-01-01T00:00:00Z".into(),
        serde_json::json!({"max_delegations": 3}),
    );
    assert_eq!(cap1.capability_id, cap2.capability_id);
    assert_eq!(cap1.capability_id, cap1.digest_hex());
}

#[test]
fn valid_delegation_chain_accepted() {
    let root_key = ConstitutionalKeyPair::generate();
    let delegate1_key = ConstitutionalKeyPair::generate();
    let delegate2_key = ConstitutionalKeyPair::generate();

    let cap1 = Capability::new(
        "vote".into(), "governance".into(),
        delegate1_key.verifying_key_hex(),
        "2026-01-01T00:00:00Z".into(), "2030-01-01T00:00:00Z".into(),
        serde_json::json!({}),
    );
    let cert1 = DelegateCertificate::sign(cap1, &root_key);

    let cap2 = Capability::new(
        "vote".into(), "governance".into(),
        delegate2_key.verifying_key_hex(),
        "2027-01-01T00:00:00Z".into(), "2029-12-31T23:59:59Z".into(),
        serde_json::json!({}),
    );
    let cert2 = DelegateCertificate::sign(cap2, &delegate1_key);

    let chain = vec![cert1, cert2];
    assert!(delegation::verify_delegation_chain(&chain, &root_key.verifying_key_hex()).is_ok());
}

#[test]
fn delegation_chain_rejects_unsigned() {
    let root_key = ConstitutionalKeyPair::generate();
    let fake_key = ConstitutionalKeyPair::generate();

    let cap = Capability::new(
        "vote".into(), "governance".into(),
        "some_other_key".into(),
        "2026-01-01T00:00:00Z".into(), "2030-01-01T00:00:00Z".into(),
        serde_json::json!({}),
    );
    let cert = DelegateCertificate::sign(cap, &fake_key);
    let chain = vec![cert];
    assert!(delegation::verify_delegation_chain(&chain, &root_key.verifying_key_hex()).is_err());
}

#[test]
fn simple_majority_passes() {
    let policy = QuorumPolicy::simple_majority();
    assert!(policy.is_satisfied(10, 6));
    assert!(!policy.is_satisfied(10, 4));
}

#[test]
fn super_majority_two_thirds() {
    let policy = QuorumPolicy::super_majority_two_thirds();
    assert!(policy.is_satisfied(9, 6));
    assert!(!policy.is_satisfied(9, 5));
}

#[test]
fn min_participants_not_met() {
    let policy = QuorumPolicy {
        min_participants: 5,
        approval_numerator: 1,
        approval_denominator: 2,
    };
    assert!(!policy.is_satisfied(3, 3));
}

#[test]
fn tally_is_deterministic() {
    let proposal = Proposal::new(
        "Test".into(), "desc".into(),
        "2026-01-01T00:00:00Z".into(), "2030-01-01T00:00:00Z".into(),
        QuorumPolicy::simple_majority(),
    );

    let voter_a = "key_a".to_string();
    let voter_b = "key_b".to_string();
    let eligible = vec![voter_a.clone(), voter_b.clone()];

    // Two eligible voters, both cast a ballot, both vote against.
    let ballots = vec![
        Ballot { proposal_id: proposal.proposal_id.clone(), voter_public_key_hex: voter_a.clone(), approval: false, timestamp: "t1".into() },
        Ballot { proposal_id: proposal.proposal_id.clone(), voter_public_key_hex: voter_b.clone(), approval: false, timestamp: "t2".into() },
    ];

    let tally = Tally::compute(&proposal, &ballots, &eligible);
    assert_eq!(tally.total_participants, 2);
    assert_eq!(tally.approvals, 0);
    assert!(!tally.passed);

    // Determinism check – same inputs yield identical output.
    let tally2 = Tally::compute(&proposal, &ballots, &eligible);
    assert_eq!(tally, tally2);
}

#[test]
fn amendment_lifecycle_progression() {
    let mut am = AmendmentLifecycle::new(
        "Title".into(), "description".into(),
        "2026-01-01T00:00:00Z".into(), "2030-01-01T00:00:00Z".into(),
    );
    am.propose();
    am.approve();
    am.activate();
}
