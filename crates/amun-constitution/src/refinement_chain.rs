// Refinement chain from CCF to machine code.

pub struct RefinementObligation {
    pub specification: &'static str,
    pub implementation: &'static str,
    pub proof_sketch: &'static str,
}

pub const REFINEMENT_OBLIGATIONS: &[RefinementObligation] = &[
    RefinementObligation {
        specification: "CCF State Transition Equations",
        implementation: "Operational Semantics Rules",
        proof_sketch: "Each CCF equation maps to one or more operational rules.",
    },
    RefinementObligation {
        specification: "Operational Semantics Rules",
        implementation: "StateMachine trait",
        proof_sketch: "Each rule maps to a method on the trait.",
    },
    RefinementObligation {
        specification: "Canonical Encoding (Cursor)",
        implementation: "Canonical Encoding (Buffer)",
        proof_sketch: "Refinement theorem: cursor == encode.",
    },
];
