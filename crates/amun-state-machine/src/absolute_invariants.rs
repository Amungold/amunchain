/// Absolute invariants are laws that CANNOT be amended under any circumstances.
/// They form the immutable foundation of constitutional reality.
/// These are the "physical constants" of the civilization.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbsoluteInvariant {
    /// The empty root is eternal and identical across all protocol versions
    EmptyRootEternal,
    /// Domain separators are globally unique forever
    DomainSeparatorsImmutable,
    /// MAX_DEPTH=256 is a universal constant of this universe
    MaxDepthUniversal,
    /// Replay determinism must be preserved across ALL amendments
    ReplayDeterminismAbsolute,
    /// No amendment may create circular causality
    CausalityIrreflexiveAbsolute,
    /// Genesis is the unique origin of all civilizations
    GenesisUniqueOrigin,
    /// Every state transition must be cryptographically provable
    ProvableTransitionAbsolute,
    /// The constitutional identity of a civilization cannot be silently mutated
    IdentityMutationProvable,
    /// No two different nodes may produce the same NodeHash
    NodeHashUniquenessAbsolute,
    /// The meta-constitutional laws themselves cannot be amended
    MetaConstitutionalImmutability,
}

impl AbsoluteInvariant {
    /// These invariants exist at Level 0 - they precede all other laws.
    pub fn level(&self) -> u8 {
        0
    }

    /// All absolute invariants must hold simultaneously.
    pub fn all() -> Vec<AbsoluteInvariant> {
        vec![
            Self::EmptyRootEternal,
            Self::DomainSeparatorsImmutable,
            Self::MaxDepthUniversal,
            Self::ReplayDeterminismAbsolute,
            Self::CausalityIrreflexiveAbsolute,
            Self::GenesisUniqueOrigin,
            Self::ProvableTransitionAbsolute,
            Self::IdentityMutationProvable,
            Self::NodeHashUniquenessAbsolute,
            Self::MetaConstitutionalImmutability,
        ]
    }
}
