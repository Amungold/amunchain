#![allow(dead_code)]
// V2-004: Long-Run Stability — diagnostic version

use harness::{ConsensusScenario, EventScheduler, EventType, ScenarioConfig, ScenarioRunner};
mod harness;

struct LongRunScenario {
    heights: u64,
}

impl ConsensusScenario for LongRunScenario {
    fn config(&self) -> ScenarioConfig {
        ScenarioConfig {
            num_validators: 40,
            quorum_threshold: 27,
            loss_rate: 0.0,
            delay_ms: 1,
            jitter_ms: 0,
            proposal_retries: 1,
        }
    }
    fn schedule_events(&self, scheduler: &mut EventScheduler, node_ids: &[String]) {
        for h in 0..self.heights {
            let base = h * 60;
            for nid in node_ids {
                scheduler.schedule(base, nid.clone(), EventType::ProposalBroadcast);
            }
            for nid in node_ids {
                scheduler.schedule(base + 10, nid.clone(), EventType::ProcessInbox);
            }
            for nid in node_ids {
                scheduler.schedule(base + 25, nid.clone(), EventType::ProcessInbox);
            }
            for nid in node_ids {
                scheduler.schedule(base + 40, nid.clone(), EventType::ProcessInbox);
            }
        }
        scheduler.schedule(
            self.heights * 60 + 10,
            node_ids[0].clone(),
            EventType::CheckTimeout,
        );
    }
}

#[test]
fn test_long_run_stability() {
    let heights = 50;
    let scenario = LongRunScenario { heights };
    let mut runner = ScenarioRunner::new(scenario.config(), heights * 1000);
    let result = runner.run(&scenario);
    let min = scenario.config().quorum_threshold;
    eprintln!(
        "V2-004: {} heights -> {} commits (min={})",
        heights, result.commits, min
    );
    assert!(result.success);
}
