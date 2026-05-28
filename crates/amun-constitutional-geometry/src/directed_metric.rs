/// A directed constitutional metric measures the cost of legitimate
/// transformation from civilization A to civilization B. This cost is
/// NOT symmetric: A -> B may be legitimate while B -> A is impossible.
#[derive(Debug, Clone)]
pub struct DirectedMetric {
    /// The source civilization
    pub source_hash: [u8; 32],
    /// The target civilization  
    pub target_hash: [u8; 32],
    /// Cost of transformation from source to target
    pub forward_cost: f64,
    /// Cost of transformation from target to source (may be infinite)
    pub reverse_cost: f64,
    /// Whether the forward transformation preserves all invariants
    pub forward_preserves_invariants: bool,
    /// Whether the reverse transformation preserves all invariants
    pub reverse_preserves_invariants: bool,
    /// Whether forward transformation is even possible
    pub forward_possible: bool,
    /// Whether reverse transformation is even possible
    pub reverse_possible: bool,
    /// Number of invariants that must be broken for forward transformation
    pub forward_invariant_breaks: u64,
    /// Number of invariants that must be broken for reverse transformation
    pub reverse_invariant_breaks: u64,
}

impl DirectedMetric {
    pub fn new(source: [u8; 32], target: [u8; 32]) -> Self {
        Self {
            source_hash: source,
            target_hash: target,
            forward_cost: 0.0,
            reverse_cost: 0.0,
            forward_preserves_invariants: true,
            reverse_preserves_invariants: true,
            forward_possible: true,
            reverse_possible: true,
            forward_invariant_breaks: 0,
            reverse_invariant_breaks: 0,
        }
    }

    /// Constitutional distance is directed. The distance A->B is not
    /// necessarily equal to the distance B->A. In fact, one direction
    /// may be impossible (cost = infinity).
    pub fn is_symmetric(&self) -> bool {
        (self.forward_cost - self.reverse_cost).abs() < 0.001
    }

    /// Check if transformation in a direction requires breaking
    /// absolute invariants (which makes it constitutionally impossible).
    pub fn is_forward_constitutionally_impossible(&self) -> bool {
        self.forward_invariant_breaks > 0 && !self.forward_preserves_invariants
    }

    pub fn is_reverse_constitutionally_impossible(&self) -> bool {
        self.reverse_invariant_breaks > 0 && !self.reverse_preserves_invariants
    }
}
