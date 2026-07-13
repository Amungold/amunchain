use amun_constitutional_geometry::directed_metric::DirectedMetric;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct ConstitutionalAction {
    pub path: Vec<[u8; 32]>,
    pub total_action: f64,
    pub invariants_violated: u64,
    pub absolute_invariants_broken: u64,
    pub is_legitimate: bool,
}

impl Default for ConstitutionalAction {
    fn default() -> Self {
        Self::new()
    }
}

impl ConstitutionalAction {
    pub fn new() -> Self {
        Self {
            path: Vec::new(),
            total_action: 0.0,
            invariants_violated: 0,
            absolute_invariants_broken: 0,
            is_legitimate: true,
        }
    }

    pub fn add_step(&mut self, state_hash: [u8; 32], metric: &DirectedMetric) {
        self.path.push(state_hash);
        self.total_action += metric.forward_cost;
        self.invariants_violated += metric.forward_invariant_breaks;

        if metric.is_forward_constitutionally_impossible() {
            self.absolute_invariants_broken += 1;
            self.is_legitimate = false;
        }
    }

    pub fn is_optimal(&self, alternative: &ConstitutionalAction) -> bool {
        self.total_action <= alternative.total_action
            && self.invariants_violated <= alternative.invariants_violated
    }
}

pub struct LeastInvariantViolation;

impl LeastInvariantViolation {
    pub fn select_optimal(paths: &[ConstitutionalAction]) -> Option<ConstitutionalAction> {
        paths
            .iter()
            .filter(|p| p.is_legitimate)
            .min_by(|a, b| {
                a.total_action
                    .partial_cmp(&b.total_action)
                    .unwrap_or(Ordering::Equal)
                    .then_with(|| a.invariants_violated.cmp(&b.invariants_violated))
            })
            .cloned()
    }

    pub fn verify_principle(
        path: &ConstitutionalAction,
        alternatives: &[ConstitutionalAction],
    ) -> bool {
        if let Some(optimal) = Self::select_optimal(alternatives) {
            path.total_action <= optimal.total_action
        } else {
            path.is_legitimate
        }
    }
}
