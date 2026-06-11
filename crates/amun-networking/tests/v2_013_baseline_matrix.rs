use crate::harness::event_scheduler::EventScheduler;
// V2-013: Baseline Matrix — Constitutional Harness

use harness::{ConsensusScenario, ScenarioConfig, ScenarioRunner};

mod harness;

struct BaselineScenario {
    delay_ms: u64,
    loss_rate: f64,
}

impl BaselineScenario {
    fn new(delay_ms: u64, loss_rate: f64) -> Self {
        Self {
            delay_ms,
            loss_rate,
        }
    }
}

impl ConsensusScenario for BaselineScenario {
    fn schedule_events(&self, _scheduler: &mut EventScheduler, _node_ids: &[String]) {
        // Baseline scenario: no scheduled events
    }
    fn config(&self) -> ScenarioConfig {
        ScenarioConfig {
            delay_ms: self.delay_ms,
            loss_rate: self.loss_rate,
            jitter_ms: 0,
            proposal_retries: 3,
            num_validators: 40,
            quorum_threshold: 27,
        }
    }
}

#[test]
fn test_baseline_matrix() {
    let conditions = [(0, 0.0), (0, 0.10), (10, 0.0), (10, 0.10)];
    let trials = 100;

    println!("\nV2-013 Baseline Matrix ({} trials each)", trials);
    println!("Delay | Loss | Min | Max | Avg  | Success Rate");
    println!("------+------+-----+-----+------+-------------");

    for (delay_ms, loss_rate) in conditions {
        let mut committed_counts = Vec::new();

        for trial in 0..trials {
            let seed = (delay_ms * 10000u64) + (loss_rate as u64 * 1000u64) + trial as u64;
            let scenario = BaselineScenario::new(delay_ms, loss_rate);
            let mut runner = ScenarioRunner::new(scenario.config(), seed);
            let result = runner.run(&scenario);
            committed_counts.push(result.commits);
        }

        committed_counts.sort();
        let min = committed_counts.first().unwrap();
        let max = committed_counts.last().unwrap();
        let avg = committed_counts.iter().map(|x| *x as u64).sum::<u64>() as f64 / trials as f64;
        let success_rate =
            committed_counts.iter().filter(|&&c| c >= 27).count() as f64 / trials as f64 * 100.0;

        println!(
            "{:>5}ms | {:>4}% | {:>3} | {:>3} | {:>4.1} | {:>11.1}%",
            delay_ms,
            (loss_rate * 100.0) as u32,
            min,
            max,
            avg,
            success_rate
        );
    }
}
