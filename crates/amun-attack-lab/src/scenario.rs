use crate::network::NetworkConditions;
use crate::strategy::ByzantineStrategy;

#[derive(Debug, Clone)]
pub enum ExpectedOutcome {
    MustSurvive,
    MayDieIfThresholdExceeded,
    MustDie,
}

#[derive(Debug, Clone)]
pub struct AttackScenario {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub total_nodes: usize,
    pub byzantine_count: usize,
    pub strategy: ByzantineStrategy,
    pub network_conditions: NetworkConditions,
    pub max_duration_seconds: u64,
    pub expected_outcome: ExpectedOutcome,
}
