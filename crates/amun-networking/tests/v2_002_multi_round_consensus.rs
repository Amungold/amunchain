#![allow(dead_code)]
// V2-002: Multi-Round Consensus — protocol-owned votes
use harness::{ConsensusScenario, EventScheduler, EventType, ScenarioConfig, ScenarioRunner};
mod harness;

struct MultiRoundScenario {
    rounds: u64,
}

impl ConsensusScenario for MultiRoundScenario {
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
    fn schedule_events(&self, s: &mut EventScheduler, nids: &[String]) {
        for round in 0..self.rounds {
            let base = round * 50;
            for nid in nids {
                s.schedule(base, nid.clone(), EventType::ProposalBroadcast);
            }
            for nid in nids {
                s.schedule(base + 10, nid.clone(), EventType::ProcessInbox);
            }
            // Protocol emits prevote/precommit via actions, just need inbox to process
            for nid in nids {
                s.schedule(base + 25, nid.clone(), EventType::ProcessInbox);
            }
            for nid in nids {
                s.schedule(base + 40, nid.clone(), EventType::ProcessInbox);
            }
        }
        s.schedule(
            self.rounds * 50 + 10,
            nids[0].clone(),
            EventType::CheckTimeout,
        );
    }
}

#[test]
fn test_multi_round_consensus() {
    for &rounds in &[1, 2, 3, 5] {
        let scenario = MultiRoundScenario { rounds };
        let mut runner = ScenarioRunner::new(scenario.config(), rounds * 100);
        let result = runner.run(&scenario);
        assert!(
            result.success,
            "Multi-round failed at {} rounds: {} commits",
            rounds, result.commits
        );
    }
}
