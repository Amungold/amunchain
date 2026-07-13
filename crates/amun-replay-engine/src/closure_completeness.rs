//! ClosureCompleteness — graded epistemic states for constitutional derivation.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClosureCompleteness {
    Partial,
    Sufficient,
    Exhaustive,
    HistoricalOnly,
}

impl ClosureCompleteness {
    pub fn can_derive(&self) -> bool { matches!(self, ClosureCompleteness::Sufficient | ClosureCompleteness::Exhaustive) }
    pub fn should_continue_seeking(&self) -> bool { matches!(self, ClosureCompleteness::Partial) }
    pub fn propagation_complete(&self) -> bool { matches!(self, ClosureCompleteness::Sufficient | ClosureCompleteness::Exhaustive) }
    pub fn level(&self) -> u8 {
        match self {
            ClosureCompleteness::Partial => 0,
            ClosureCompleteness::HistoricalOnly => 1,
            ClosureCompleteness::Sufficient => 2,
            ClosureCompleteness::Exhaustive => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_partial() { assert!(!ClosureCompleteness::Partial.can_derive()); assert!(ClosureCompleteness::Partial.should_continue_seeking()); }
    #[test] fn test_sufficient() { assert!(ClosureCompleteness::Sufficient.can_derive()); assert!(ClosureCompleteness::Sufficient.propagation_complete()); }
    #[test] fn test_exhaustive_maximal() { assert!(ClosureCompleteness::Exhaustive.level() > ClosureCompleteness::Sufficient.level()); }
}
