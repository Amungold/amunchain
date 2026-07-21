// ============================================================================
// ADR-021: Passive Integration Points
// These are thin wrappers that emit events WITHOUT changing consensus logic.
// ============================================================================

/// Call this when a round starts (in engine.rs or validator.rs)
#[macro_export]
macro_rules! observe_round_started {
    ($hub:expr, $height:expr, $round:expr, $validator_id:expr, $my_index:expr, $proposer_index:expr) => {
        #[cfg(feature = "consensus-observer")]
        if let Some(hub) = &$hub {
            use $crate::events::{ConsensusEvent, ConsensusEventKind};
            hub.emit(ConsensusEvent::new(
                hub.next_event_id(),
                None,
                hub.next_correlation_id(),
                hub.next_sequence(),
                $height,
                ConsensusEventKind::RoundStarted {
                    height: $height,
                    round: $round,
                    validator_id: $validator_id,
                    my_index: $my_index,
                    proposer_index: $proposer_index,
                    is_proposer: $my_index == $proposer_index,
                },
            ));
        }
    };
}

/// Call this when a leader is selected
#[macro_export]
macro_rules! observe_leader_selected {
    ($hub:expr, $height:expr, $leader_id:expr, $leader_index:expr) => {
        #[cfg(feature = "consensus-observer")]
        if let Some(hub) = &$hub {
            use $crate::events::{ConsensusEvent, ConsensusEventKind};
            hub.emit(ConsensusEvent::new(
                hub.next_event_id(),
                None,
                hub.next_correlation_id(),
                hub.next_sequence(),
                $height,
                ConsensusEventKind::LeaderSelected {
                    height: $height,
                    leader_id: $leader_id,
                    leader_index: $leader_index,
                },
            ));
        }
    };
}

/// Call this when a proposal is created (by the proposer)
#[macro_export]
macro_rules! observe_proposal_created {
    ($hub:expr, $height:expr, $proposer_id:expr, $block_hash:expr) => {
        #[cfg(feature = "consensus-observer")]
        if let Some(hub) = &$hub {
            use $crate::events::{ConsensusEvent, ConsensusEventKind};
            hub.emit(ConsensusEvent::new(
                hub.next_event_id(),
                None,
                hub.next_correlation_id(),
                hub.next_sequence(),
                $height,
                ConsensusEventKind::ProposalCreated {
                    height: $height,
                    proposer_id: $proposer_id,
                    block_hash: $block_hash,
                },
            ));
        }
    };
}

/// Call this when a proposal is received (by validators)
#[macro_export]
macro_rules! observe_proposal_received {
    ($hub:expr, $height:expr, $from:expr, $to:expr) => {
        #[cfg(feature = "consensus-observer")]
        if let Some(hub) = &$hub {
            use $crate::events::{ConsensusEvent, ConsensusEventKind};
            hub.emit(ConsensusEvent::new(
                hub.next_event_id(),
                None,
                hub.next_correlation_id(),
                hub.next_sequence(),
                $height,
                ConsensusEventKind::ProposalReceived {
                    height: $height,
                    from: $from,
                    to: $to,
                },
            ));
        }
    };
}

/// Call this when a vote is sent
#[macro_export]
macro_rules! observe_vote_sent {
    ($hub:expr, $height:expr, $from:expr, $to:expr) => {
        #[cfg(feature = "consensus-observer")]
        if let Some(hub) = &$hub {
            use $crate::events::{ConsensusEvent, ConsensusEventKind};
            hub.emit(ConsensusEvent::new(
                hub.next_event_id(),
                None,
                hub.next_correlation_id(),
                hub.next_sequence(),
                $height,
                ConsensusEventKind::VoteSent {
                    height: $height,
                    from: $from,
                    to: $to,
                },
            ));
        }
    };
}

/// Call this when a vote is received
#[macro_export]
macro_rules! observe_vote_received {
    ($hub:expr, $height:expr, $from:expr, $to:expr) => {
        #[cfg(feature = "consensus-observer")]
        if let Some(hub) = &$hub {
            use $crate::events::{ConsensusEvent, ConsensusEventKind};
            hub.emit(ConsensusEvent::new(
                hub.next_event_id(),
                None,
                hub.next_correlation_id(),
                hub.next_sequence(),
                $height,
                ConsensusEventKind::VoteReceived {
                    height: $height,
                    from: $from,
                    to: $to,
                    valid: true,
                },
            ));
        }
    };
}

/// Call this when a QC is formed
#[macro_export]
macro_rules! observe_qc_formed {
    ($hub:expr, $height:expr, $num_signatures:expr) => {
        #[cfg(feature = "consensus-observer")]
        if let Some(hub) = &$hub {
            use $crate::events::{ConsensusEvent, ConsensusEventKind};
            hub.emit(ConsensusEvent::new(
                hub.next_event_id(),
                None,
                hub.next_correlation_id(),
                hub.next_sequence(),
                $height,
                ConsensusEventKind::QuorumCertificateFormed {
                    height: $height,
                    num_signatures: $num_signatures,
                },
            ));
        }
    };
}

/// Call this when a block is finalized
#[macro_export]
macro_rules! observe_block_finalized {
    ($hub:expr, $height:expr, $block_hash:expr) => {
        #[cfg(feature = "consensus-observer")]
        if let Some(hub) = &$hub {
            use $crate::events::{ConsensusEvent, ConsensusEventKind};
            hub.emit(ConsensusEvent::new(
                hub.next_event_id(),
                None,
                hub.next_correlation_id(),
                hub.next_sequence(),
                $height,
                ConsensusEventKind::BlockFinalized {
                    height: $height,
                    block_hash: $block_hash,
                },
            ));
        }
    };
}

/// Prints a timeline report (call at end of test)
#[macro_export]
macro_rules! print_round_timeline {
    ($tracer:expr) => {
        #[cfg(feature = "consensus-observer")]
        {
            let report = $tracer.generate_report();
            if !report.contains("NO EVENTS") {
                println!("{}", report);
            }
        }
    };
}
