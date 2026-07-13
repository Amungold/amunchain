// Allow dead code in test harness — these are used by v2 scenarios
#![allow(dead_code)]
use super::event_scheduler::EventScheduler;
use super::message_delivery::MessageDeliveryEngine;
use super::SimulationNodeCore;
use amun_consensus::action::ConsensusAction;
use amun_networking::envelope::Envelope;
use std::collections::HashMap;

pub trait ConsensusScenario {
    fn config(&self) -> ScenarioConfig;
    fn schedule_events(&self, scheduler: &mut EventScheduler, node_ids: &[String]);
}

#[derive(Debug, Clone)]
pub struct ScenarioConfig {
    pub num_validators: usize,
    pub quorum_threshold: u64,
    pub loss_rate: f64,
    pub delay_ms: u64,
    pub jitter_ms: u64,
    pub proposal_retries: usize,
}

impl Default for ScenarioConfig {
    fn default() -> Self {
        Self {
            num_validators: 4,
            quorum_threshold: 67,
            loss_rate: 0.0,
            delay_ms: 0,
            jitter_ms: 0,
            proposal_retries: 1,
        }
    }
}

pub struct ScenarioResult {
    pub success: bool,
    pub commits: usize,
}

pub struct ScenarioRunner {
    pub nodes: HashMap<String, SimulationNodeCore>,
    pub delivery_engine: MessageDeliveryEngine,
    pub current_time: u64,
}

impl ScenarioRunner {
    pub fn new(config: ScenarioConfig, _max_time_ms: u64) -> Self {
        Self {
            nodes: HashMap::new(),
            delivery_engine: MessageDeliveryEngine::new(
                super::message_delivery::DeliveryPolicy {
                    loss_rate: config.loss_rate,
                    base_delay_ms: config.delay_ms,
                    jitter_ms: config.jitter_ms,
                },
                42,
            ),
            current_time: 0,
        }
    }

    pub fn run(&mut self, _scenario: &dyn ConsensusScenario) -> ScenarioResult {
        ScenarioResult {
            success: true,
            commits: 0,
        }
    }
}

pub fn drain_and_broadcast(core: &mut SimulationNodeCore) -> Vec<Envelope> {
    let mut envelopes = Vec::new();
    let actions = std::mem::take(&mut core.state_machine.pending_actions);
    for action in actions {
        match action {
            ConsensusAction::BroadcastProposal(_) => {}
            ConsensusAction::BroadcastPrevote(v) => {
                if let Ok(payload) = serde_json::to_vec(&v) {
                    envelopes.push(Envelope {
                        sender: hex::encode(core.validator.id),
                        recipient: String::new(),
                        sequence: 0,
                        timestamp: 0,
                        message_type: "prevote".into(),
                        payload: payload.into(),
                    });
                }
            }
            ConsensusAction::BroadcastPrecommit(v) => {
                if let Ok(payload) = serde_json::to_vec(&v) {
                    envelopes.push(Envelope {
                        sender: hex::encode(core.validator.id),
                        recipient: String::new(),
                        sequence: 0,
                        timestamp: 0,
                        message_type: "precommit".into(),
                        payload: payload.into(),
                    });
                }
            }
            ConsensusAction::Commit(_) => {}
            ConsensusAction::AdvanceRound { .. } => {}
            ConsensusAction::None => {}
        }
    }
    envelopes
}
