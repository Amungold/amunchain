#[cfg(test)] mod tests { use crate::*;
    #[test] fn test_proposal_new() { let p = Proposal::new(1, 100); assert_eq!(p.status, ProposalStatus::Deposit); }
    #[test] fn test_proposal_passing() { let mut p = Proposal::new(1, 100); p.yes = 700; p.no = 200; assert!(p.is_passing(1000)); }
    #[test] fn test_proposal_failing() { let mut p = Proposal::new(1, 100); p.yes = 200; p.no = 700; assert!(!p.is_passing(1000)); }
}
