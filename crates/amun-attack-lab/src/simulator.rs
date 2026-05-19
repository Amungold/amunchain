use crate::scenario::AttackScenario;
use amun_deterministic_allocator::DeterministicMap;
use amun_entropy_transcript::DeterministicEntropy;

#[derive(Debug, Clone)]
pub struct WeightedValidator {
    pub id: usize,
    pub stake: u64,
    pub is_byzantine: bool,
    pub locked_round: u64,
    pub locked_value: Option<[u8; 32]>,
}

#[derive(Debug)]
pub struct SimulationState {
    pub validators: Vec<WeightedValidator>,
    pub finalized_blocks: Vec<FinalizedBlock>,
    pub equivocation_events: Vec<EquivocationEvent>,
    pub lock_violation_events: Vec<LockViolationEvent>,
    pub current_round: u64,
    pub total_stake: u64,
}

#[derive(Debug, Clone)]
pub struct FinalizedBlock {
    pub height: u64,
    pub round: u64,
    pub hash: [u8; 32],
}

#[derive(Debug, Clone)]
pub struct EquivocationEvent {
    pub validator_id: usize,
    pub round: u64,
}

#[derive(Debug, Clone)]
pub struct LockViolationEvent {
    pub validator_id: usize,
    pub round: u64,
}

#[derive(Debug)]
pub struct AttackSimulator {
    pub scenarios: Vec<AttackScenario>,
}

#[derive(Debug, Clone)]
pub enum SimulationResult {
    Survived { blocks_finalized: u64, duration_steps: u64 },
    Died { violation_description: String, at_step: u64 },
}

impl AttackSimulator {
    pub fn new() -> Self {
        Self {
            scenarios: Vec::new(),
        }
    }

    pub fn register_scenario(&mut self, scenario: AttackScenario) -> Result<(), &'static str> {
        if self.scenarios.len() >= 128 {
            return Err("scenario list full");
        }
        self.scenarios.push(scenario);
        Ok(())
    }

    pub fn scenario_count(&self) -> usize {
        self.scenarios.len()
    }

    pub fn simulate(
        &self,
        scenario: &AttackScenario,
        max_steps: u64,
        entropy_seed: [u8; 32],
    ) -> SimulationResult {
        let mut entropy = DeterministicEntropy::new(entropy_seed);
        
        let mut state = SimulationState {
            validators: (0..scenario.total_nodes)
                .map(|id| WeightedValidator {
                    id,
                    stake: if id < scenario.byzantine_count {
                        entropy.gen_range(1, 20)
                    } else {
                        entropy.gen_range(50, 200)
                    },
                    is_byzantine: id < scenario.byzantine_count,
                    locked_round: 0,
                    locked_value: None,
                })
                .collect(),
            finalized_blocks: Vec::new(),
            equivocation_events: Vec::new(),
            lock_violation_events: Vec::new(),
            current_round: 0,
            total_stake: 0,
        };

        state.total_stake = state.validators.iter().map(|v| v.stake).sum();

        for step in 0..max_steps {
            state.current_round += 1;

            for v in state.validators.iter().filter(|v| v.is_byzantine) {
                if entropy.gen_range(0, 100) < 20 {
                    state.equivocation_events.push(EquivocationEvent {
                        validator_id: v.id,
                        round: state.current_round,
                    });
                }
            }

            for v in state.validators.iter_mut() {
                if v.is_byzantine && entropy.gen_range(0, 100) < 10 {
                    state.lock_violation_events.push(LockViolationEvent {
                        validator_id: v.id,
                        round: state.current_round,
                    });
                }
            }

            if let Some(violation) = self.check_safety(&state) {
                return SimulationResult::Died {
                    violation_description: violation,
                    at_step: step,
                };
            }

            if state.finalized_blocks.len() >= 10 {
                return SimulationResult::Survived {
                    blocks_finalized: state.finalized_blocks.len() as u64,
                    duration_steps: step,
                };
            }

            if state.current_round % 3 == 0 && state.equivocation_events.len() < 3 {
                let hash = {
                    let mut h = [0u8; 32];
                    h[..8].copy_from_slice(&state.current_round.to_le_bytes());
                    h
                };
                state.finalized_blocks.push(FinalizedBlock {
                    height: state.finalized_blocks.len() as u64 + 1,
                    round: state.current_round,
                    hash,
                });
            }
        }

        SimulationResult::Survived {
            blocks_finalized: state.finalized_blocks.len() as u64,
            duration_steps: max_steps,
        }
    }

    fn check_safety(&self, state: &SimulationState) -> Option<String> {
        let mut height_map: DeterministicMap<u64, [u8; 32]> = DeterministicMap::new();
        for block in &state.finalized_blocks {
            if let Some(existing) = height_map.get(&block.height) {
                if *existing != block.hash {
                    return Some(format!(
                        "SAFETY VIOLATION: conflicting finality at height {}",
                        block.height
                    ));
                }
            }
            let _ = height_map.insert(block.height, block.hash);
        }
        None
    }
}
