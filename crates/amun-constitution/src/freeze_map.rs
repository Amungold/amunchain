/// Constitutional Freeze Map
/// Defines what CAN and CANNOT be amended, and under what conditions.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutabilityClass {
    /// Immutable - cannot be changed under any circumstances
    Immutable,
    /// Amendment requires constitutional supermajority + replay proof
    ConstitutionalAmendment,
    /// Amendment requires protocol upgrade + migration
    ProtocolUpgrade,
    /// Amendment requires governance vote
    GovernanceVote,
    /// Mutable by validators within defined parameters
    ParameterChange,
    /// Mutable by individual nodes (local configuration)
    NodeLocal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuorumClass {
    /// Requires 90%+ of all validators
    SuperMajority,
    /// Requires 67%+ of all validators
    Majority,
    /// Requires 51%+ of participating validators
    SimpleMajority,
    /// No quorum needed (node-local)
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreezeBoundary {
    pub field_name: &'static str,
    pub module: &'static str,
    pub mutability: MutabilityClass,
    pub required_quorum: QuorumClass,
    pub requires_replay_preservation: bool,
    pub requires_snapshot_compatibility: bool,
    pub description: &'static str,
}

pub struct ConstitutionalFreezeMap;

impl ConstitutionalFreezeMap {
    pub fn frozen_boundaries() -> Vec<FreezeBoundary> {
        vec![
            FreezeBoundary {
                field_name: "MAX_DEPTH",
                module: "amun-storage-kernel/src/smt/tree.rs",
                mutability: MutabilityClass::Immutable,
                required_quorum: QuorumClass::SuperMajority,
                requires_replay_preservation: true,
                requires_snapshot_compatibility: true,
                description: "Tree depth - changing this breaks all proofs, replays, and snapshots",
            },
            FreezeBoundary {
                field_name: "PROOF_VERSION_V1",
                module: "amun-storage-kernel/src/smt/proof.rs",
                mutability: MutabilityClass::ConstitutionalAmendment,
                required_quorum: QuorumClass::SuperMajority,
                requires_replay_preservation: true,
                requires_snapshot_compatibility: true,
                description: "Proof version - requires new proof system and migration",
            },
            FreezeBoundary {
                field_name: "PROTOCOL_DOMAIN_*",
                module: "amun-canonical-codec/src/constants.rs",
                mutability: MutabilityClass::Immutable,
                required_quorum: QuorumClass::SuperMajority,
                requires_replay_preservation: true,
                requires_snapshot_compatibility: true,
                description: "Domain separators - changing these forks the civilization",
            },
            FreezeBoundary {
                field_name: "CANONICAL_EMPTY_ROOT_V1",
                module: "amun-storage-kernel/src/smt/constants.rs",
                mutability: MutabilityClass::Immutable,
                required_quorum: QuorumClass::SuperMajority,
                requires_replay_preservation: true,
                requires_snapshot_compatibility: true,
                description: "Empty root - changing this creates an incompatible universe",
            },
            FreezeBoundary {
                field_name: "MAX_CHUNK_SIZE",
                module: "amun-snapshot-engine/src/snapshot.rs",
                mutability: MutabilityClass::ProtocolUpgrade,
                required_quorum: QuorumClass::Majority,
                requires_replay_preservation: false,
                requires_snapshot_compatibility: true,
                description: "Chunk size - affects snapshot format compatibility",
            },
            FreezeBoundary {
                field_name: "Constitution text",
                module: "docs/constitutional/",
                mutability: MutabilityClass::ConstitutionalAmendment,
                required_quorum: QuorumClass::SuperMajority,
                requires_replay_preservation: false,
                requires_snapshot_compatibility: false,
                description: "Constitutional documents - governance amendment",
            },
        ]
    }
}
