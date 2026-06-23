use amun_nft_governance::{GovernanceLedger, GovernanceRight};
use amun_nft_governance_execution::GovernanceExecution;

#[test]
fn n143_create_proposal_and_vote() {
    let mut gov = GovernanceLedger::new();
    let proposer = [10u8; 32];
    gov.set_rights(GovernanceRight {
        token_id: [1u8; 32],
        owner: proposer,
        can_propose: true,
        can_veto: false,
        voting_power: 50,
    });

    let mut exec = GovernanceExecution::new();
    let id = exec
        .propose(&gov, &proposer, "Test proposal".into(), 1, 100)
        .unwrap();
    assert!(exec.proposals.contains_key(&id));

    let voter = [20u8; 32];
    gov.set_rights(GovernanceRight {
        token_id: [2u8; 32],
        owner: voter,
        can_propose: false,
        can_veto: false,
        voting_power: 30,
    });
    assert!(exec.vote(&gov, &id, &voter, true));
    let (support, against) = exec.tally(&id);
    assert_eq!(support, 30);
    assert_eq!(against, 0);
}

#[test]
fn n143_execute_passing_proposal() {
    let mut gov = GovernanceLedger::new();
    let proposer = [10u8; 32];
    gov.set_rights(GovernanceRight {
        token_id: [1u8; 32],
        owner: proposer,
        can_propose: true,
        can_veto: false,
        voting_power: 10,
    });
    let voter = [20u8; 32];
    gov.set_rights(GovernanceRight {
        token_id: [2u8; 32],
        owner: voter,
        can_propose: false,
        can_veto: false,
        voting_power: 100,
    });

    let mut exec = GovernanceExecution::new();
    let id = exec
        .propose(&gov, &proposer, "Passing proposal".into(), 1, 100)
        .unwrap();
    exec.vote(&gov, &id, &voter, true);
    assert!(exec.execute(&id, 50));
}

#[test]
fn n143_proposal_fails_without_rights() {
    let gov = GovernanceLedger::new();
    let mut exec = GovernanceExecution::new();
    assert_eq!(
        exec.propose(&gov, &[99u8; 32], "No right".into(), 1, 100),
        None
    );
}

#[test]
fn n143_execution_root_deterministic() {
    let mut gov = GovernanceLedger::new();
    gov.set_rights(GovernanceRight {
        token_id: [1u8; 32],
        owner: [10u8; 32],
        can_propose: true,
        can_veto: false,
        voting_power: 10,
    });
    let mut e1 = GovernanceExecution::new();
    let mut e2 = GovernanceExecution::new();
    let id1 = e1.propose(&gov, &[10u8; 32], "A".into(), 1, 100).unwrap();
    let id2 = e2.propose(&gov, &[10u8; 32], "A".into(), 1, 100).unwrap();
    let voter = [20u8; 32];
    gov.set_rights(GovernanceRight {
        token_id: [2u8; 32],
        owner: voter,
        can_propose: false,
        can_veto: false,
        voting_power: 5,
    });
    e1.vote(&gov, &id1, &voter, true);
    e2.vote(&gov, &id2, &voter, true);
    assert_eq!(e1.compute_execution_root(), e2.compute_execution_root());
}
