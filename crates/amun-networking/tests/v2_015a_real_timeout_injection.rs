#![allow(dead_code)]
// V2-015A: Real Timeout Injection - FIXED
// Root cause: Prevote/Precommit were not scheduled after timeout recovery.
// Fix: Schedule voting phases after proposal injection.

use amun_consensus::{
    pacemaker::{FixedMultiplier, Pacemaker, PacemakerConfig},
    proposal_manager::{LeaderSelector, RoundRobinSelector},
    round_state_machine::RoundStateMachine,
    state::ConsensusStep,
    types::{BlockProposal, Vote, VoteType},
    validator::{Validator, ValidatorSet},
};
use amun_networking::{envelope::Envelope, transport::MockTransport};
use rand::prelude::*;
use rand::rngs::StdRng;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
enum EventType {
    ProposalBroadcast,
    PrevoteBroadcast,
    PrecommitBroadcast,
    ProcessInbox,
    CheckTimeout,
    DeliverPending,
}

#[derive(Debug, Clone)]
struct ScheduledEvent {
    time: u64,
    node_id: String,
    event_type: EventType,
    id: u64,
}

impl Ord for ScheduledEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .time
            .cmp(&self.time)
            .then_with(|| other.id.cmp(&self.id))
    }
}
impl PartialOrd for ScheduledEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for ScheduledEvent {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.id == other.id
    }
}
impl Eq for ScheduledEvent {}

struct EventScheduler {
    queue: BinaryHeap<ScheduledEvent>,
    next_id: u64,
}
impl EventScheduler {
    fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            next_id: 0,
        }
    }
    fn schedule(&mut self, time: u64, node_id: String, event_type: EventType) {
        self.queue.push(ScheduledEvent {
            time,
            node_id,
            event_type,
            id: self.next_id,
        });
        self.next_id += 1;
    }
    fn next_event(&mut self) -> Option<ScheduledEvent> {
        self.queue.pop()
    }
}

struct DelayedEnvelope {
    delivery_time: u64,
    recipient: String,
    envelope: Envelope,
}

struct ConsensusNode {
    validator: Validator,
    transport: MockTransport,
    state_machine: RoundStateMachine,
    last_proposal: Option<BlockProposal>,
    prevotes_broadcast: bool,
    precommits_broadcast: bool,
    seen_proposals: HashSet<[u8; 32]>,
    round_start_time: u64,
    timeout_triggered: bool,
    commit_counted: bool,
}

struct TimeoutInjectionSim {
    nodes: HashMap<String, ConsensusNode>,
    validator_set: ValidatorSet,
    selector: RoundRobinSelector,
    rng: StdRng,
    scheduler: EventScheduler,
    current_time: u64,
    round: u64,
    loss_rate: f64,
    delay_ms: u64,
    jitter_ms: u64,
    proposal_retries: u32,
    commits: u64,
    pending_messages: Vec<DelayedEnvelope>,
    max_recovery_rounds: u64,
    rounds_advanced: u64,
    timeout_count: u64,
}

impl TimeoutInjectionSim {
    fn new(
        validator_set: ValidatorSet,
        seed: u64,
        loss_rate: f64,
        delay_ms: u64,
        jitter_ms: u64,
        proposal_retries: u32,
        max_recovery_rounds: u64,
    ) -> Self {
        Self {
            nodes: HashMap::new(),
            validator_set,
            selector: RoundRobinSelector,
            rng: StdRng::seed_from_u64(seed),
            scheduler: EventScheduler::new(),
            current_time: 0,
            round: 0,
            loss_rate,
            delay_ms,
            jitter_ms,
            proposal_retries,
            commits: 0,
            pending_messages: Vec::new(),
            max_recovery_rounds,
            rounds_advanced: 0,
            timeout_count: 0,
        }
    }

    fn add_node(&mut self, id: String, validator: Validator, pacemaker_config: PacemakerConfig) {
        let pacemaker = Pacemaker::new(pacemaker_config);
        let state_machine = RoundStateMachine::new(1, pacemaker, [0u8; 32]);
        self.nodes.insert(
            id,
            ConsensusNode {
                validator,
                transport: MockTransport::new(),
                state_machine,
                last_proposal: None,
                prevotes_broadcast: false,
                precommits_broadcast: false,
                seen_proposals: HashSet::new(),
                round_start_time: 0,
                timeout_triggered: false,
                commit_counted: false,
            },
        );
    }

    fn schedule_delivery(&mut self, delivery_time: u64, recipient: String, envelope: Envelope) {
        self.pending_messages.push(DelayedEnvelope {
            delivery_time,
            recipient: recipient.clone(),
            envelope,
        });
        self.scheduler
            .schedule(delivery_time, recipient, EventType::DeliverPending);
    }

    fn deliver_ready_messages(&mut self, current_time: u64) {
        let mut remaining = Vec::new();
        for msg in self.pending_messages.drain(..) {
            if msg.delivery_time <= current_time {
                if let Some(node) = self.nodes.get_mut(&msg.recipient) {
                    node.transport.deliver(msg.envelope);
                }
            } else {
                remaining.push(msg);
            }
        }
        self.pending_messages = remaining;
    }

    fn broadcast_message(&mut self, sender_id: &str, envelope: Envelope, include_self: bool) {
        let jitter = if self.jitter_ms > 0 {
            self.rng.gen_range(0..=self.jitter_ms)
        } else {
            0
        };
        let delivery_time = self.current_time + self.delay_ms + jitter;
        let mut recipients = Vec::new();
        for other_id in self.nodes.keys() {
            if other_id == sender_id {
                if include_self {
                    recipients.push(other_id.clone());
                }
            } else {
                recipients.push(other_id.clone());
            }
        }
        for recipient in recipients {
            if self.rng.gen::<f64>() < self.loss_rate {
                continue;
            }
            self.schedule_delivery(delivery_time, recipient, envelope.clone());
        }
    }

    fn process_event(&mut self, event: ScheduledEvent) {
        self.current_time = event.time;
        let node_id = event.node_id.clone();

        match event.event_type {
            EventType::ProposalBroadcast => {
                let node = self.nodes.get(&node_id).unwrap();
                let current_round = node.state_machine.state.round;
                let leader_id = self
                    .selector
                    .leader(1, current_round, &self.validator_set)
                    .unwrap();
                if node.validator.id == leader_id {
                    let proposal = BlockProposal {
                        height: 1,
                        block_hash: [0xAA; 32],
                        proposer: leader_id,
                        round: current_round,
                        timestamp: 1000,
                    };
                    let envelope = Envelope {
                        sender: node_id.clone(),
                        recipient: String::new(),
                        sequence: 1,
                        timestamp: 1000,
                        message_type: "proposal".into(),
                        payload: serde_json::to_vec(&proposal).unwrap(),
                    };
                    let _ = node;
                    for _ in 0..self.proposal_retries {
                        self.broadcast_message(&node_id, envelope.clone(), true);
                    }
                }
            }
            EventType::PrevoteBroadcast => {
                let node = self.nodes.get_mut(&node_id).unwrap();
                if !node.prevotes_broadcast {
                    if let Some(prop) = node.last_proposal.as_ref() {
                        let vote = Vote {
                            height: 1,
                            block_hash: prop.block_hash,
                            voter: node.validator.id,
                            round: node.state_machine.state.round,
                            vote_type: VoteType::Prevote,
                            timestamp: 1001,
                        };
                        let envelope = Envelope {
                            sender: node_id.clone(),
                            recipient: String::new(),
                            sequence: 2,
                            timestamp: 1001,
                            message_type: "prevote".into(),
                            payload: serde_json::to_vec(&vote).unwrap(),
                        };
                        let _ = node;
                        self.broadcast_message(&node_id, envelope, true);
                        if let Some(n) = self.nodes.get_mut(&node_id) {
                            n.prevotes_broadcast = true;
                        }
                    }
                }
            }
            EventType::PrecommitBroadcast => {
                let node = self.nodes.get_mut(&node_id).unwrap();
                if !node.precommits_broadcast
                    && node.state_machine.state.step == ConsensusStep::Precommit
                {
                    if let Some(prop) = node.last_proposal.as_ref() {
                        let vote = Vote {
                            height: 1,
                            block_hash: prop.block_hash,
                            voter: node.validator.id,
                            round: node.state_machine.state.round,
                            vote_type: VoteType::Precommit,
                            timestamp: 1002,
                        };
                        let envelope = Envelope {
                            sender: node_id.clone(),
                            recipient: String::new(),
                            sequence: 3,
                            timestamp: 1002,
                            message_type: "precommit".into(),
                            payload: serde_json::to_vec(&vote).unwrap(),
                        };
                        let _ = node;
                        self.broadcast_message(&node_id, envelope, true);
                        if let Some(n) = self.nodes.get_mut(&node_id) {
                            n.precommits_broadcast = true;
                        }
                    }
                }
            }
            EventType::DeliverPending => {
                self.deliver_ready_messages(self.current_time);
            }
            EventType::ProcessInbox => {
                let node = self.nodes.get_mut(&node_id).unwrap();
                while let Some(env) = node.transport.next_incoming() {
                    match env.message_type.as_str() {
                        "proposal" => {
                            if let Ok(p) = serde_json::from_slice::<BlockProposal>(&env.payload) {
                                if p.round == node.state_machine.state.round {
                                    if node.seen_proposals.contains(&p.block_hash) {
                                        continue;
                                    }
                                    node.seen_proposals.insert(p.block_hash);
                                    node.state_machine.state.see_valid(p.block_hash);
                                    node.state_machine.state.advance_step();
                                    node.last_proposal = Some(p);
                                }
                            }
                        }
                        "prevote" => {
                            if let Ok(v) = serde_json::from_slice::<Vote>(&env.payload) {
                                if v.round == node.state_machine.state.round {
                                    node.state_machine.process_vote(v, &self.validator_set);
                                }
                            }
                        }
                        "precommit" => {
                            if let Ok(v) = serde_json::from_slice::<Vote>(&env.payload) {
                                if v.round == node.state_machine.state.round {
                                    node.state_machine.process_vote(v, &self.validator_set);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                let _ = node.state_machine.finalize_commit();
                if node.state_machine.last_committed_height == 1 && !node.commit_counted {
                    self.commits += 1;
                    node.commit_counted = true;
                }
            }
            EventType::CheckTimeout => {
                let node = self.nodes.get_mut(&node_id).unwrap();
                if node.state_machine.last_committed_height < 1 {
                    if let Some(timeout) = node.state_machine.current_timeout_ms() {
                        if self.current_time >= node.round_start_time + timeout {
                            let _ = node.state_machine.advance_round();
                            self.rounds_advanced += 1;
                            self.timeout_count += 1;
                            node.round_start_time = self.current_time;
                            node.prevotes_broadcast = false;
                            node.precommits_broadcast = false;
                        }
                    }
                }
            }
        }
    }

    fn run_timeout_recovery_cycle(&mut self, inject_after_rounds: u64) {
        let node_ids: Vec<String> = self.nodes.keys().cloned().collect();
        for t in 0..500u64 {
            for nid in &node_ids {
                self.scheduler
                    .schedule(t, nid.clone(), EventType::CheckTimeout);
            }
        }
        for t in (10..500u64).step_by(10) {
            for nid in &node_ids {
                self.scheduler
                    .schedule(t, nid.clone(), EventType::ProcessInbox);
            }
        }
        let inject_time = inject_after_rounds * 100;
        for nid in &node_ids {
            self.scheduler
                .schedule(inject_time, nid.clone(), EventType::ProposalBroadcast);
        }
        // Fix: Schedule voting phases after proposal delivery window
        for nid in &node_ids {
            self.scheduler
                .schedule(inject_time + 20, nid.clone(), EventType::PrevoteBroadcast);
            self.scheduler
                .schedule(inject_time + 40, nid.clone(), EventType::PrecommitBroadcast);
        }
        while let Some(event) = self.scheduler.next_event() {
            self.process_event(event.clone());
        }
    }
}

#[test]
fn test_timeout_injection_recovery() {
    let timeout_ms = 100u64;
    let inject_after_rounds = 3;
    let mut validators = Vec::new();
    for i in 0u8..40 {
        let mut id = [0u8; 32];
        id[0] = i;
        validators.push(Validator {
            id,
            voting_power: 1,
        });
    }
    let validator_set = ValidatorSet::new(validators).unwrap();
    let pacemaker_config = PacemakerConfig {
        base_propose_timeout_ms: timeout_ms,
        base_prevote_timeout_ms: timeout_ms / 2,
        base_precommit_timeout_ms: timeout_ms / 2,
        timeout_multiplier: FixedMultiplier::new(3, 2),
        max_timeout_rounds: 10,
    };
    let mut sim = TimeoutInjectionSim::new(validator_set, 42, 0.0, 10, 0, 3, 3);
    for i in 0u8..40 {
        let mut id = [0u8; 32];
        id[0] = i;
        sim.add_node(
            format!("{:02x}", i),
            Validator {
                id,
                voting_power: 1,
            },
            pacemaker_config.clone(),
        );
    }
    sim.run_timeout_recovery_cycle(inject_after_rounds);
    let commits = sim.commits;
    eprintln!(
        "=== Timeout Injection Test: {} commits after injecting proposal at round {} ===",
        commits, inject_after_rounds
    );
    assert!(
        commits >= 27,
        "Pacemaker failed to maintain liveness after {} timeouts",
        sim.timeout_count
    );
}
