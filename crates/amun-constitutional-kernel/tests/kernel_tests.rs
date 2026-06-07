use amun_constitutional_signing::ConstitutionalKeyPair;
use amun_constitutional_authority::ConstitutionalCertificate;
use amun_constitutional_governance::capability::Capability;
use amun_constitutional_governance::quorum::QuorumPolicy;
use amun_constitutional_governance::voting::{Proposal, Ballot, Tally};
use amun_constitutional_governance::amendment::AmendmentLifecycle;
use amun_constitutional_kernel::{
    ExecutionContext,
    ConstitutionalStateMachine,
    AmendmentActivator,
};
use std::collections::BTreeMap;

fn build_test_context() -> (ExecutionContext, Vec<Capability>, Vec<ConstitutionalCertificate>) {
    let key = ConstitutionalKeyPair::generate();
    let root_cert = ConstitutionalCertificate::new_root(
        key.verifying_key_hex(),
        "2026-01-01T00:00:00Z".into(),
        "2036-01-01T00:00:00Z".into(),
        "ConstitutionalRoot".into(),
        "2026-05-28T00:00:00Z".into(),
    );

    let capabilities = vec![
        Capability::new(
            "activate_amendment".into(),
            "constitutional".into(),
            key.verifying_key_hex(),
            "2026-01-01T00:00:00Z".into(),
            "2036-01-01T00:00:00Z".into(),
            serde_json::json!({}),
        ),
        Capability::new(
            "set_parameter".into(),
            "constitutional".into(),
            key.verifying_key_hex(),
            "2026-01-01T00:00:00Z".into(),
            "2036-01-01T00:00:00Z".into(),
            serde_json::json!({}),
        ),
    ];

    let ctx = ExecutionContext::new(
        vec![root_cert],
        "2026-06-01T00:00:00Z".into(),
    );

    (ctx, capabilities, vec![])
}

#[test]
fn test_state_machine_determinism() {
    let (ctx, capabilities, _) = build_test_context();
    let mut machine = ConstitutionalStateMachine::new();

    let updates: BTreeMap<String, String> = [
        ("key_a".into(), "value_a".into()),
    ].into();

    let (v1, receipt1) = machine.transition(
        &ctx, &capabilities, "set_parameter", "constitutional", updates.clone(),
    ).expect("transition should succeed");

    // Replay must produce identical result
    let mut machine2 = ConstitutionalStateMachine::new();
    let (v2, receipt2) = machine2.transition(
        &ctx, &capabilities, "set_parameter", "constitutional", updates,
    ).expect("transition should succeed");

    assert_eq!(v1, v2);
    assert_eq!(receipt1, receipt2);
    assert_eq!(machine.state, machine2.state);
}

#[test]
fn test_unauthorised_action_rejected() {
    let (ctx, _capabilities, _) = build_test_context();
    let mut machine = ConstitutionalStateMachine::new();

    let empty_caps: Vec<Capability> = vec![];
    let updates: BTreeMap<String, String> = [].into();

    let result = machine.transition(
        &ctx, &empty_caps, "set_parameter", "constitutional", updates,
    );
    assert!(result.is_err());
}

#[test]
fn test_amendment_activation() {
    let (ctx, capabilities, _) = build_test_context();
    let mut machine = ConstitutionalStateMachine::new();

    let mut amendment = AmendmentLifecycle::new(
        "Title".into(), "desc".into(),
        "2026-01-01T00:00:00Z".into(), "2030-01-01T00:00:00Z".into(),
    );
    amendment.propose();
    amendment.approve();

    let tally = Tally {
        proposal_id: amendment.proposal.proposal_id.clone(),
        total_participants: 10,
        approvals: 8,
        passed: true,
    };

    let version = AmendmentActivator::activate(
        &mut machine, &ctx, &capabilities, &mut amendment, &tally,
    ).expect("activation should succeed");

    assert!(version > 0);
    assert!(machine.state.fields.contains_key(&format!("amendment.{}", amendment.proposal.proposal_id)));
}

#[test]
fn test_proposal_ballot_integration() {
    // Demonstrate a full governance flow that uses Proposal, Ballot, QuorumPolicy, and Tally.
    let (ctx, capabilities, _) = build_test_context();
    let mut machine = ConstitutionalStateMachine::new();

    // Create a proposal with a two-thirds super-majority requirement
    let quorum = QuorumPolicy::super_majority_two_thirds();
    let proposal = Proposal::new(
        "constitutional_update".into(),
        "kernel transition".into(),
        "2026-01-01T00:00:00Z".into(),
        "2030-01-01T00:00:00Z".into(),
        quorum,
    );
    assert!(!proposal.proposal_id.is_empty());

    // Simulate votes from three eligible voters (two approvals -> 2/3 majority)
    let voter_a = "voter_a_key".to_string();
    let voter_b = "voter_b_key".to_string();
    let voter_c = "voter_c_key".to_string();
    let eligible = vec![voter_a.clone(), voter_b.clone(), voter_c.clone()];

    let ballots = vec![
        Ballot {
            proposal_id: proposal.proposal_id.clone(),
            voter_public_key_hex: voter_a,
            approval: true,
            timestamp: "2026-05-28T12:00:00Z".into(),
        },
        Ballot {
            proposal_id: proposal.proposal_id.clone(),
            voter_public_key_hex: voter_b,
            approval: true,
            timestamp: "2026-05-28T13:00:00Z".into(),
        },
        Ballot {
            proposal_id: proposal.proposal_id.clone(),
            voter_public_key_hex: voter_c,
            approval: false,
            timestamp: "2026-05-28T14:00:00Z".into(),
        },
    ];

    let tally = Tally::compute(&proposal, &ballots, &eligible);
    assert!(tally.passed);

    // Now use the amendment lifecycle and activate it through the kernel
    let mut amendment = AmendmentLifecycle::new(
        "Kernel Integration".into(),
        "use proposal and ballot types".into(),
        "2026-01-01T00:00:00Z".into(),
        "2030-01-01T00:00:00Z".into(),
    );
    amendment.propose();
    amendment.approve();

    // For the purpose of this test we reuse the independent tally that passed
    let activation_tally = Tally {
        proposal_id: amendment.proposal.proposal_id.clone(),
        total_participants: tally.total_participants,
        approvals: tally.approvals,
        passed: tally.passed,
    };

    let version = AmendmentActivator::activate(
        &mut machine, &ctx, &capabilities, &mut amendment, &activation_tally,
    ).expect("activation should succeed");

    assert!(version > 0);
    assert!(machine.state.fields.contains_key(&format!("amendment.{}", amendment.proposal.proposal_id)));
}
