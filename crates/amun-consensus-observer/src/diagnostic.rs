use crate::{
    rule_engine, ConsensusEvent, ConsensusEventKind, ObserverHub, RoundTracer, RuleEngine,
};
use std::sync::Arc;

pub struct N1023Diagnostic {
    pub hub: Arc<ObserverHub>,
    tracer: Arc<RoundTracer>,
    rule_engine: RuleEngine,
}

impl N1023Diagnostic {
    pub fn new() -> Self {
        let hub = Arc::new(ObserverHub::new());
        let tracer = Arc::new(RoundTracer::new());

        // Shared ownership: hub stores Arc<RoundTracer>, we keep our own Arc
        hub.register(tracer.clone());

        let rule_engine = RuleEngine::new()
            .with_rule(rule_engine::RoundMustStart)
            .with_rule(rule_engine::ProposalMustExist);

        Self {
            hub,
            tracer,
            rule_engine,
        }
    }

    pub fn leader_selected(&self, height: u64, leader_id: u64, leader_index: usize) {
        let event = ConsensusEvent::new(
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
        );
        self.hub.emit(event);
    }

    pub fn round_started(
        &self,
        height: u64,
        round: u32,
        validator_id: u64,
        my_index: usize,
        proposer_index: usize,
    ) {
        let event = ConsensusEvent::new(
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
        );
        self.hub.emit(event);
    }

    pub fn proposal_created(&self, height: u64, proposer_id: u64, block_hash: [u8; 32]) {
        let event = ConsensusEvent::new(
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
        );
        self.hub.emit(event);
    }

    pub fn proposal_received(&self, height: u64, from: u64, to: u64) {
        let event = ConsensusEvent::new(
            self.hub.next_event_id(),
            None,
            self.hub.next_correlation_id(),
            self.hub.next_sequence(),
            height,
            ConsensusEventKind::ProposalReceived { height, from, to },
        );
        self.hub.emit(event);
    }

    pub fn print_timeline(&self) {
        println!("\n================ ROUND 1 TIMELINE ================");
        let report = self.tracer.generate_report();
        if report.contains("NO EVENTS") {
            println!("NO EVENTS RECORDED - Round may not have started");
        } else {
            println!("{}", report);
        }

        let events = self.tracer.events();
        let consensus_events: Vec<ConsensusEvent> = events.into_iter().map(|e| e.event).collect();
        let violations = self.rule_engine.check_liveness(&consensus_events);

        if !violations.is_empty() {
            println!("\n--- LIVENESS VIOLATIONS ---");
            for v in &violations {
                println!("[{}] {}: {}", v.category_name(), v.rule_name, v.message);
            }
        }
        println!("=================================================\n");
    }
}
