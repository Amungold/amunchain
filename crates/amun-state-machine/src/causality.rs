use std::collections::HashMap;

/// Causal height: number of causal ancestors.
pub type CausalHeight = u64;

/// Lamport-style logical clock for constitutional causality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LamportTime(pub u64);

/// Vector clock for partial ordering in DAG.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorClock {
    pub components: HashMap<[u8; 32], u64>,
}

impl Default for VectorClock {
    fn default() -> Self {
        Self::new()
    }
}

impl VectorClock {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn increment(&mut self, civilization_id: &[u8; 32]) {
        *self.components.entry(*civilization_id).or_insert(0) += 1;
    }

    /// a happens-before b if a < b in all components
    pub fn happens_before(&self, other: &VectorClock) -> bool {
        if self.components.is_empty() {
            return true;
        }
        self.components
            .iter()
            .all(|(k, v)| other.components.get(k).is_some_and(|ov| v <= ov))
    }

    /// Merge two vector clocks (for merge operations)
    pub fn merge(&self, other: &VectorClock) -> VectorClock {
        let mut merged = self.clone();
        for (k, v) in &other.components {
            let entry = merged.components.entry(*k).or_insert(0);
            *entry = (*entry).max(*v);
        }
        merged
    }
}

/// Causality metadata attached to each transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CausalityMetadata {
    pub lamport_time: LamportTime,
    pub vector_clock: VectorClock,
    pub causal_height: CausalHeight,
    pub parent_transitions: Vec<[u8; 32]>,
}

impl CausalityMetadata {
    pub fn new(
        parents: Vec<[u8; 32]>,
        parent_clocks: Vec<&VectorClock>,
        civilization_id: [u8; 32],
    ) -> Self {
        let max_lamport = parent_clocks
            .iter()
            .map(|c| c.components.values().sum::<u64>())
            .max()
            .unwrap_or(0);
        let mut vc = VectorClock::new();
        for pc in &parent_clocks {
            vc = vc.merge(pc);
        }
        vc.increment(&civilization_id);
        let height = parent_clocks
            .iter()
            .map(|c| c.components.values().sum::<u64>())
            .max()
            .unwrap_or(0)
            + 1;
        Self {
            lamport_time: LamportTime(max_lamport + 1),
            vector_clock: vc,
            causal_height: height,
            parent_transitions: parents,
        }
    }
}
