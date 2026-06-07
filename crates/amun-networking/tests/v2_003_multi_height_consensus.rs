#![allow(dead_code)]
// V2-003: Multi-Height Consensus — protocol-owned votes
use harness::{ConsensusScenario, ScenarioConfig, ScenarioRunner, EventScheduler, EventType};
mod harness;

struct MultiHeightScenario { heights: u64 }

impl ConsensusScenario for MultiHeightScenario {
    fn config(&self) -> ScenarioConfig {
        ScenarioConfig { num_validators: 40, quorum_threshold: 27, loss_rate: 0.0, delay_ms: 1, jitter_ms: 0, proposal_retries: 1 }
    }
    fn schedule_events(&self, s: &mut EventScheduler, nids: &[String]) {
        for h in 0..self.heights {
            let base = h * 60;
            for nid in nids { s.schedule(base, nid.clone(), EventType::ProposalBroadcast); }
            for nid in nids { s.schedule(base + 10, nid.clone(), EventType::ProcessInbox); }
            for nid in nids { s.schedule(base + 25, nid.clone(), EventType::ProcessInbox); }
            for nid in nids { s.schedule(base + 40, nid.clone(), EventType::ProcessInbox); }
        }
        s.schedule(self.heights * 60 + 10, nids[0].clone(), EventType::CheckTimeout);
    }
}

#[test]
fn test_multi_height_consensus() {
    for &heights in &[1, 2, 3] {
        let scenario = MultiHeightScenario { heights };
        let mut runner = ScenarioRunner::new(scenario.config(), heights * 200);
        let result = runner.run(&scenario);
        assert!(result.success, "Multi-height failed at {} heights: {} commits", heights, result.commits);
    }
}
