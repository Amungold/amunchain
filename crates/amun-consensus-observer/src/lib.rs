pub mod events;
pub mod observer;
pub mod observer_hub;
pub mod round_tracer;
pub mod rule_engine;

#[cfg(feature = "consensus-observer")]
pub mod integration;

pub mod diagnostic;

pub use events::{ConsensusEvent, ConsensusEventKind};
pub use observer::{ConsensusObserver, MetricsObserver, ReplayableObserver};
pub use observer_hub::ObserverHub;
pub use round_tracer::RoundTracer;
pub use rule_engine::{AuditViolation, RuleCategory, RuleEngine};

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_observer_hub_basic() {
        let hub = ObserverHub::new();
        let event = ConsensusEvent::new(
            hub.next_event_id(),
            None,
            hub.next_correlation_id(),
            hub.next_sequence(),
            1,
            ConsensusEventKind::RoundStarted {
                height: 1,
                round: 0,
                validator_id: 1,
                my_index: 0,
                proposer_index: 0,
                is_proposer: true,
            },
        );
        hub.emit(event);
        assert_eq!(hub.event_count(), 2);
    }

    #[test]
    fn test_round_tracer() {
        let tracer = Arc::new(RoundTracer::new());
        let hub = ObserverHub::new();
        let event = ConsensusEvent::new(
            hub.next_event_id(),
            None,
            hub.next_correlation_id(),
            hub.next_sequence(),
            1,
            ConsensusEventKind::RoundStarted {
                height: 1,
                round: 0,
                validator_id: 1,
                my_index: 0,
                proposer_index: 0,
                is_proposer: true,
            },
        );
        tracer.on_event(&event);
        assert_eq!(tracer.event_count(), 1);
    }

    #[test]
    fn test_observer_hub_multiple_observers() {
        let hub = ObserverHub::new();
        let tracer1 = Arc::new(RoundTracer::new());
        let tracer2 = Arc::new(RoundTracer::new());

        hub.register(tracer1);
        hub.register(tracer2);

        let event = ConsensusEvent::new(
            hub.next_event_id(),
            None,
            hub.next_correlation_id(),
            hub.next_sequence(),
            1,
            ConsensusEventKind::RoundStarted {
                height: 1,
                round: 0,
                validator_id: 1,
                my_index: 0,
                proposer_index: 0,
                is_proposer: true,
            },
        );

        hub.emit(event);
        assert_eq!(hub.event_count(), 2);
    }

    #[test]
    fn test_rule_engine_detects_no_proposal() {
        let engine = RuleEngine::new()
            .with_rule(rule_engine::RoundMustStart)
            .with_rule(rule_engine::ProposalMustExist);

        let hub = ObserverHub::new();
        let events = vec![ConsensusEvent::new(
            hub.next_event_id(),
            None,
            hub.next_correlation_id(),
            hub.next_sequence(),
            1,
            ConsensusEventKind::RoundStarted {
                height: 1,
                round: 0,
                validator_id: 1,
                my_index: 0,
                proposer_index: 0,
                is_proposer: true,
            },
        )];

        let violations = engine.check_all(&events);
        assert!(!violations.is_empty());
        assert_eq!(violations[0].rule_name, "ProposalMustExist");
    }

    #[test]
    fn test_safety_rules_detect_double_vote() {
        let engine = RuleEngine::new()
            .with_rule(rule_engine::NoDoubleVote)
            .with_rule(rule_engine::NoDoubleProposal);

        let hub = ObserverHub::new();
        let events = vec![
            ConsensusEvent::new(
                hub.next_event_id(),
                None,
                hub.next_correlation_id(),
                hub.next_sequence(),
                1,
                ConsensusEventKind::VoteSent {
                    height: 1,
                    from: 1,
                    to: 2,
                },
            ),
            ConsensusEvent::new(
                hub.next_event_id(),
                None,
                hub.next_correlation_id(),
                hub.next_sequence(),
                1,
                ConsensusEventKind::VoteSent {
                    height: 1,
                    from: 1,
                    to: 3,
                },
            ),
        ];

        let violations = engine.check_safety(&events);
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_audit_for_n102_3_scenario() {
        let hub = ObserverHub::new();
        let events = vec![
            ConsensusEvent::new(
                hub.next_event_id(),
                None,
                hub.next_correlation_id(),
                hub.next_sequence(),
                1,
                ConsensusEventKind::ValidatorRegistered { validator_id: 1 },
            ),
            ConsensusEvent::new(
                hub.next_event_id(),
                Some(1),
                hub.next_correlation_id(),
                hub.next_sequence(),
                1,
                ConsensusEventKind::LeaderSelected {
                    height: 1,
                    leader_id: 1,
                    leader_index: 0,
                },
            ),
            ConsensusEvent::new(
                hub.next_event_id(),
                Some(2),
                hub.next_correlation_id(),
                hub.next_sequence(),
                1,
                ConsensusEventKind::RoundStarted {
                    height: 1,
                    round: 0,
                    validator_id: 1,
                    my_index: 0,
                    proposer_index: 0,
                    is_proposer: true,
                },
            ),
            ConsensusEvent::new(
                hub.next_event_id(),
                Some(3),
                hub.next_correlation_id(),
                hub.next_sequence(),
                1,
                ConsensusEventKind::ProposalRejected {
                    height: 1,
                    reason: events::ProposalRejectionReason::Other("Block builder failed".into()),
                },
            ),
            ConsensusEvent::new(
                hub.next_event_id(),
                Some(3),
                hub.next_correlation_id(),
                hub.next_sequence(),
                1,
                ConsensusEventKind::LeaderTimeout {
                    height: 1,
                    expected_leader: 1,
                },
            ),
        ];

        let engine = RuleEngine::new()
            .with_rule(rule_engine::RoundMustStart)
            .with_rule(rule_engine::ProposalMustExist);

        let liveness_violations = engine.check_liveness(&events);
        assert!(!liveness_violations.is_empty());

        let has_proposal_violation = liveness_violations
            .iter()
            .any(|v| v.rule_name == "ProposalMustExist");
        assert!(
            has_proposal_violation,
            "Expected ProposalMustExist violation for N102.3"
        );

        let engine = RuleEngine::new()
            .with_rule(rule_engine::NoDoubleVote)
            .with_rule(rule_engine::NoDoubleProposal);
        let safety_violations = engine.check_safety(&events);
        assert!(safety_violations.is_empty());
    }
}

pub mod runtime_observer;
pub use runtime_observer::RuntimeObserver;
