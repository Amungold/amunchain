use crate::harness::event_scheduler::EventScheduler;
// V2-001: Happy Path — first Stable Scenario on Constitutional Harness

use harness::{ConsensusScenario, ScenarioConfig, ScenarioRunner};
mod harness;

struct HappyPathScenario;

impl ConsensusScenario for HappyPathScenario {
    fn schedule_events(&self, _scheduler: &mut EventScheduler, _node_ids: &[String]) {}
    fn config(&self) -> ScenarioConfig {
        ScenarioConfig {
            num_validators: 40, quorum_threshold: 27,
            loss_rate: 0.0, delay_ms: 1, jitter_ms: 0,
            proposal_retries: 1,
        }
    }
}

#[test]
fn test_happy_path_consensus() {
    let scenario = HappyPathScenario;
    let mut runner = ScenarioRunner::new(scenario.config(), 42);
    let result = runner.run(&scenario);
    assert!(result.success, "Happy path consensus failed: {} commits", result.commits);
}
