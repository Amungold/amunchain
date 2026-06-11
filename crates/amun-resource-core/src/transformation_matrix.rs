use serde::{Deserialize, Serialize};

/// Resource archetypes for the transformation matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceArchetype {
    Asset,
    Evidence,
    Claim,
    Certificate,
    ConstitutionalAsset,
}

/// Static transformation matrix enforcing Law T1.
pub struct TransformationMatrix;

impl TransformationMatrix {
    /// Returns true if the transformation from `src` to `tgt` is legal.
    pub fn is_legal(src: ResourceArchetype, tgt: ResourceArchetype) -> bool {
        matches!(
            (src, tgt),
            (ResourceArchetype::Asset, ResourceArchetype::Asset)
                | (
                    ResourceArchetype::Asset,
                    ResourceArchetype::ConstitutionalAsset
                )
                | (ResourceArchetype::Evidence, ResourceArchetype::Claim)
                | (
                    ResourceArchetype::ConstitutionalAsset,
                    ResourceArchetype::ConstitutionalAsset
                )
                | (
                    ResourceArchetype::ConstitutionalAsset,
                    ResourceArchetype::Claim
                )
        )
    }

    /// Certificates are terminal — no derivations from a Certificate are legal.
    pub fn is_terminal(archetype: ResourceArchetype) -> bool {
        matches!(archetype, ResourceArchetype::Certificate)
    }

    /// Returns a human-readable description of the matrix.
    pub fn describe() -> &'static str {
        "Asset -> Asset | ConstitutionalAsset\n\
         Evidence -> Claim\n\
         Claim -> (evaluated by VerdictEvaluator, not a resource derivation)\n\
         Certificate -> (terminal, no derivations permitted)\n\
         ConstitutionalAsset -> ConstitutionalAsset | Claim"
    }
}
