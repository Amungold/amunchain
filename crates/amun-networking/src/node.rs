use crate::envelope::Envelope;
use crate::transport::MockTransport;
use amun_consensus::action::ConsensusAction;
use amun_consensus::pacemaker::{FixedMultiplier, Pacemaker, PacemakerConfig};
use amun_consensus::round_state_machine::RoundStateMachine;
use amun_consensus::types::{BlockProposal, Vote};
use amun_consensus::validator::ValidatorSet;

/// A constitutional network node that bridges consensus engine with network transport.
///
/// Integrates:
/// - amun-consensus (RoundStateMachine) — BFT consensus engine
/// - amun-networking (MockTransport) — network message delivery
///
/// Flow:
///   Leader calls propose() → emits BroadcastProposal to network
///   All nodes receive proposal → accept_proposal() → emit BroadcastPrevote
///   Votes collected → QC formed → emit BroadcastPrecommit
///   Precommits collected → QC formed → emit Commit → block finalized
/// Node lifecycle states governing consensus participation.
/// Only Active nodes may propose or vote.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeLifecycle {
    Bootstrapping,
    CatchingUp,
    Verifying,
    Active,
}
pub struct NetworkNode {
    pub id: [u8; 32],
    pub transport: MockTransport,
    pub consensus: RoundStateMachine,
    pub committed_blocks: Vec<[u8; 32]>,
    pub lifecycle: NodeLifecycle,
    pub current_height: u64,
    sequence: u64,
    /// Optional cryptographic key pair for signing messages.
    pub keypair: Option<crate::crypto_identity::PeerKeyPair>,
}

impl NetworkNode {
    pub fn new(id: [u8; 32]) -> Self {
        let pacemaker = Pacemaker::new(PacemakerConfig {
            base_propose_timeout_ms: 1000,
            base_prevote_timeout_ms: 500,
            base_precommit_timeout_ms: 500,
            max_timeout_rounds: 10,
            timeout_multiplier: FixedMultiplier {
                numerator: 2,
                denominator: 1,
            },
        });
        Self {
            id,
            transport: MockTransport::new(),
            consensus: RoundStateMachine::new(0, pacemaker, id),
            committed_blocks: Vec::new(),
            keypair: None,
            lifecycle: NodeLifecycle::Active,
            current_height: 0,
            sequence: 0,
        }
    }

    /// Create a node that must bootstrap before becoming active.
    pub fn new_bootstrapping(id: [u8; 32]) -> Self {
        let mut node = Self::new(id);
        node.lifecycle = NodeLifecycle::Bootstrapping;
        node
    }

    /// Transition to Active after successful bootstrap.
    pub fn activate(&mut self) {
        self.lifecycle = NodeLifecycle::Active;
    }

    /// Begin the bootstrap phase. Node requests sync data from peers.
    pub fn begin_bootstrap(&mut self) {
        self.lifecycle = NodeLifecycle::Bootstrapping;
    }

    /// Begin catching up. Node downloads checkpoints and blocks.
    pub fn begin_catchup(&mut self) {
        self.lifecycle = NodeLifecycle::CatchingUp;
    }

    /// Begin verification. Node verifies downloaded state against trusted root.
    pub fn begin_verification(&mut self) {
        self.lifecycle = NodeLifecycle::Verifying;
    }

    /// Import a checkpoint height from a trusted source.
    /// Used during catch-up to sync the node's height with the network.
    pub fn import_checkpoint_height(&mut self, height: u64) {
        self.current_height = height;
        self.consensus.state.height = height;
        self.consensus.last_committed_height = height.saturating_sub(1);
    }

    /// Get this node's PeerId if a keypair is set.
    pub fn peer_id(&self) -> Option<crate::peer_identity::PeerId> {
        self.keypair.as_ref().map(|kp| kp.peer_id())
    }

    /// Check if the node is allowed to participate in consensus.
    pub fn is_active(&self) -> bool {
        self.lifecycle == NodeLifecycle::Active
    }

    /// Leader proposes a new block. Emits BroadcastProposal to the network.
    pub fn propose(&mut self) {
        if self.lifecycle != NodeLifecycle::Active {
            return;
        }
        let block_hash = [self.current_height as u8; 32];
        self.consensus.propose(block_hash, self.id);
    }

    /// Drain pending consensus actions and convert to network envelopes.
    /// Uses std::mem::take to safely drain without mutation during iteration.
    pub fn drain_actions(&mut self) -> Vec<Envelope> {
        let mut envelopes = Vec::new();
        let actions = std::mem::take(&mut self.consensus.pending_actions);
        for action in actions {
            match action {
                ConsensusAction::BroadcastProposal(proposal) => {
                    if let Ok(payload) = serde_json::to_vec(&proposal) {
                        envelopes.push(self.make_envelope("proposal", payload));
                    }
                    // Leader also processes its own proposal locally
                    self.consensus.accept_proposal(proposal.block_hash);
                }
                ConsensusAction::BroadcastPrevote(vote) => {
                    if let Ok(payload) = serde_json::to_vec(&vote) {
                        envelopes.push(self.make_envelope("vote", payload));
                    }
                }
                ConsensusAction::BroadcastPrecommit(vote) => {
                    if let Ok(payload) = serde_json::to_vec(&vote) {
                        envelopes.push(self.make_envelope("vote", payload));
                    }
                }
                ConsensusAction::Commit(_) => {
                    // finalize_commit() handles state transition
                }
                ConsensusAction::AdvanceRound { .. } => {}
                ConsensusAction::None => {}
            }
        }
        envelopes
    }

    /// Process incoming envelopes from the transport.
    /// Proposals advance to Prevote step. Votes feed into the collector.
    /// Commit is checked once after all messages are processed.
    pub fn process_incoming(&mut self, validator_set: &ValidatorSet) {
        while let Some(envelope) = self.transport.next_incoming() {
            if envelope.message_type == "proposal" {
                if let Ok(proposal) = serde_json::from_slice::<BlockProposal>(&envelope.payload) {
                    if proposal.height == self.current_height {
                        self.consensus.accept_proposal(proposal.block_hash);
                    }
                }
            } else if envelope.message_type == "vote" {
                if let Ok(vote) = serde_json::from_slice::<Vote>(&envelope.payload) {
                    if vote.height == self.current_height {
                        self.consensus.process_vote(vote, validator_set);
                    }
                }
            }
        }
        // Commit finalization happens once, after all votes processed
        if self.consensus.is_committed() {
            if let Ok(qc) = self.consensus.finalize_commit() {
                self.committed_blocks.push(qc.block_hash);
                self.current_height += 1;
            }
        }
    }

    /// Send all pending actions to the transport outbox.
    pub fn flush_outgoing(&mut self) {
        let actions = self.drain_actions();
        for envelope in actions {
            self.transport.send(envelope);
        }
    }

    /// Drain the transport outbox for network delivery.
    pub fn drain_outbox(&mut self) -> Vec<Envelope> {
        let mut out = Vec::new();
        while let Some(env) = self.transport.next_outgoing() {
            out.push(env);
        }
        out
    }

    fn make_envelope(&mut self, msg_type: &str, payload: Vec<u8>) -> Envelope {
        self.sequence += 1;
        Envelope {
            sender: hex::encode(self.id),
            recipient: String::new(),
            sequence: self.sequence,
            timestamp: 0,
            message_type: msg_type.into(),
            payload: payload.into(),
        }
    }
}
