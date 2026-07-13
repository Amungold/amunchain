#[cfg(test)]
use crate::capacity::*;
use crate::liveness::*;
use crate::quorum_transition::*;

#[test]
fn test_protocol_capacities_compatible() {
    let c1 = ProtocolCapacities::constitutional();
    let c2 = ProtocolCapacities::constitutional();
    assert!(c1.verify_compatible(&c2).is_ok());
}

#[test]
fn test_quorum_transition_safe() {
    let params = QuorumTransitionParameters {
        old_set_size: 4,
        new_set_size: 4,
        overlap_size: 3,
    };
    assert!(params.verify_safety().is_ok());
}

#[test]
fn test_quorum_transition_unsafe() {
    let params = QuorumTransitionParameters {
        old_set_size: 7,
        new_set_size: 10,
        overlap_size: 1,
    };
    assert!(params.verify_safety().is_err());
}

#[test]
fn test_liveness_timeout() {
    let params = LogicalLivenessParameters::constitutional();
    assert!(params.should_timeout(0, 3, 0));
    assert!(!params.should_timeout(0, 2, 0));
}
