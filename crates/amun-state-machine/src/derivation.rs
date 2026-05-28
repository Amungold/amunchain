use super::axioms::ConstitutionalAxiom;

/// A theorem derived from constitutional axioms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theorem {
    pub name: String,
    pub statement: String,
    pub depends_on: Vec<ConstitutionalAxiom>,
    pub derived_at: u64,
    pub theorem_hash: [u8; 32],
}

/// An inference rule for deriving new theorems.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InferenceRule {
    /// If A implies B and B implies C, then A implies C
    Transitivity,
    /// If a property holds for all transitions, it holds for each individual transition
    UniversalInstantiation,
    /// If a merger preserves property P for both parents, the child also has P
    MergePreservation,
    /// If no counterexample exists, the theorem holds
    NoCounterexample,
}

/// A derivation is a proof tree from axioms to a theorem.
#[derive(Debug, Clone)]
pub struct Derivation {
    pub theorem: Theorem,
    pub proof_steps: Vec<DerivationStep>,
}

#[derive(Debug, Clone)]
pub enum DerivationStep {
    Axiom(ConstitutionalAxiom),
    Inference(InferenceRule, Vec<usize>),
    TheoremApplication(usize),
}

/// Consistency proof: proves that axioms do not contradict each other.
#[derive(Debug, Clone)]
pub struct ConsistencyProof {
    pub axioms_checked: Vec<ConstitutionalAxiom>,
    pub is_consistent: bool,
    pub proof_hash: [u8; 32],
}
