// ============================================================================
// ADR-021: Runtime Observer Interface
// Thin interface that ConsensusRuntime depends on (not N1023Diagnostic directly)
// ============================================================================

use crate::{ConsensusEvent, ConsensusEventKind, ObserverHub, RoundTracer};
use std::sync::Arc;

/// Generic observer that ConsensusRuntime uses.
/// N1023Diagnostic is just one implementation of this pattern.
pub struct RuntimeObserver {
    hub: Arc<ObserverHub>,
}

impl RuntimeObserver {
    pub fn new(hub: Arc<ObserverHub>) -> Self {
        Self { hub }
    }

    pub fn leader_selected(&self, height: u64, leader_id: u64, leader_index: usize) {
        self.hub.emit(ConsensusEvent::new(
            self.hub.next_event_id(),
            None,
            self.hub.next_correlation_id(),
            self.hub.next_sequence(),
            height,
            ConsensusEventKind::LeaderSelected {
                height,
                leader_id,
                leader_index,
            },
        ));
    }

    pub fn round_started(
        &self,
        height: u64,
        round: u32,
        validator_id: u64,
        my_index: usize,
        proposer_index: usize,
    ) {
        self.hub.emit(ConsensusEvent::new(
            self.hub.next_event_id(),
            None,
            self.hub.next_correlation_id(),
            self.hub.next_sequence(),
            height,
            ConsensusEventKind::RoundStarted {
                height,
                round,
                validator_id,
                my_index,
                proposer_index,
                is_proposer: my_index == proposer_index,
            },
        ));
    }

    pub fn proposal_created(&self, height: u64, proposer_id: u64, block_hash: [u8; 32]) {
        self.hub.emit(ConsensusEvent::new(
            self.hub.next_event_id(),
            None,
            self.hub.next_correlation_id(),
            self.hub.next_sequence(),
            height,
            ConsensusEventKind::ProposalCreated {
                height,
                proposer_id,
                block_hash,
            },
        ));
    }

    pub fn proposal_received(&self, height: u64, from: u64, to: u64) {
        self.hub.emit(ConsensusEvent::new(
            self.hub.next_event_id(),
            None,
            self.hub.next_correlation_id(),
            self.hub.next_sequence(),
            height,
            ConsensusEventKind::ProposalReceived { height, from, to },
        ));
    }
}
