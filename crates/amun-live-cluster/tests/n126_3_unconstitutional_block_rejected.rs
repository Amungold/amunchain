// N126.3 — Unconstitutional block must be rejected at commit time
use amun_constitutional_enforcement::{
    ConstitutionalEnforcementKernel, ConstitutionalLaw, ConstitutionalVerdict,
};

#[test]
fn n126_3_state_root_mismatch_rejected() {
    let mut kernel = ConstitutionalEnforcementKernel::new();
    // StateRootIntegrity fails → block must be rejected
    let verdict = kernel.review_block(
        100, false, // state_root_valid = FALSE → violation
        true, true, true, true, true, true, true, true, true,
    );
    match verdict {
        ConstitutionalVerdict::Unconstitutional { violations } => {
            assert!(
                violations
                    .iter()
                    .any(|v| v.law == ConstitutionalLaw::StateRootIntegrity),
                "Must detect StateRootIntegrity violation"
            );
        }
        _ => panic!("Expected Unconstitutional verdict"),
    }
}

#[test]
fn n126_3_finality_missing_supermajority_rejected() {
    let mut kernel = ConstitutionalEnforcementKernel::new();
    // All true except finality_supermajority
    let verdict = kernel.review_block(
        100, true, true, true, true, true, true, true, false, true, true,
    );
    match verdict {
        ConstitutionalVerdict::Unconstitutional { violations } => {
            assert!(
                violations
                    .iter()
                    .any(|v| v.law == ConstitutionalLaw::FinalitySupermajority),
                "Must detect FinalitySupermajority violation"
            );
        }
        _ => panic!("Expected Unconstitutional verdict"),
    }
}

#[test]
fn n126_3_constitutional_block_accepted() {
    let mut kernel = ConstitutionalEnforcementKernel::new();
    let verdict = kernel.review_block(
        100, true, true, true, true, true, true, true, true, true, true,
    );
    assert_eq!(verdict, ConstitutionalVerdict::Constitutional);
}
