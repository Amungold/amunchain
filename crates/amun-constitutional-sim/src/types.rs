use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClaimAction {
    Govern,
    Trade,
    Treaty,
    Recognize,
    Tax,
    Defend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jurisdiction {
    pub regions: Vec<u64>,
}

impl Jurisdiction {
    pub fn overlap(&self, other: &Jurisdiction) -> f64 {
        if self.regions.is_empty() || other.regions.is_empty() {
            return 0.0;
        }
        let set_self: HashSet<u64> = self.regions.iter().copied().collect();
        let set_other: HashSet<u64> = other.regions.iter().copied().collect();
        
        let intersection_count = set_self.intersection(&set_other).count();
        let union_count = set_self.union(&set_other).count();
        
        if union_count == 0 { return 0.0; }
        intersection_count as f64 / union_count as f64
    }
}
