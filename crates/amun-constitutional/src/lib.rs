#![allow(clippy::result_large_err, clippy::too_many_arguments)]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(not(feature = "std"))]
extern crate alloc;

// Foundation
pub mod architectural_invariants;
pub mod constitutional_failure;
pub mod constitutional_hasher;
pub mod constitutional_object;
pub mod execution_limits;
pub mod hash_domain_registry;
pub mod hash_domains;
pub mod kernel_types;
pub mod prelude;
pub mod schema_registry;

// Execution object model
pub mod execution_boundary;
pub mod execution_context;
pub mod execution_journal;
pub mod transition_commitment;
pub mod transition_evidence;

// Receipt + Certificate + State Anchor + Snapshot
pub mod certificate_scope;
pub mod execution_receipt;
pub mod replay_certificate;
pub mod replay_outcome;
pub mod snapshot;
pub mod snapshot_scope;
pub mod state_anchor;
pub mod state_anchor_scope;

// Restoration + Continuation
pub mod continuation_chain;
pub mod restoration_point;

// Divergence semantics
pub mod divergence_point;
pub mod divergence_resolution;
pub mod divergence_type;

// Causality semantics
pub mod causal_edge;
pub mod causality_chain;
pub mod causality_type;

// Witness semantics
pub mod constitutional_witness;
pub mod witness_type;

// Proof graph correctness
pub mod canonical_witness;
pub mod cycle_detection;

// Serialization + Migration (PHASE 65)
pub mod canonical_serialize;
pub mod revision_migration;

// Artifact graph integrity
pub mod artifact_graph;

// Re-exports
pub use architectural_invariants::*;
pub use artifact_graph::{ArtifactEdge, ArtifactEdgeType, ArtifactGraph, EdgeVerification};
pub use canonical_serialize::{CanonicalDecode, CanonicalEncode};
pub use canonical_witness::{canonical_order, is_canonical, normalize};
pub use causal_edge::CausalEdge;
pub use causality_chain::CausalityChain;
pub use causality_type::CausalityType;
pub use certificate_scope::{CertificateScope, ScopeRelationship};
pub use constitutional_failure::{failure_domain, failure_type, severity, ConstitutionalFailure};
pub use constitutional_hasher::ConstitutionalHasher;
pub use constitutional_object::{ConstitutionalIdentity, ConstitutionalObject};
pub use constitutional_witness::{ConstitutionalWitness, WitnessEntry};
pub use continuation_chain::ContinuationChain;
pub use cycle_detection::{detect_cycles, CycleDetectionResult};
pub use divergence_point::DivergencePoint;
pub use divergence_resolution::{DivergenceResolution, ResolutionType};
pub use divergence_type::DivergenceType;
pub use execution_boundary::ExecutionBoundary;
pub use execution_context::ExecutionContext;
pub use execution_journal::{ExecutionJournal, JournalEntry};
pub use execution_limits::*;
pub use execution_receipt::ExecutionReceipt;
pub use hash_domain_registry::*;
pub use hash_domains::*;
pub use kernel_types::*;
pub use replay_certificate::ReplayCertificate;
pub use replay_outcome::ReplayOutcome;
pub use restoration_point::RestorationPoint;
pub use revision_migration::{MigrationPath, RevisionCompatibility, UpgradeProof};
pub use schema_registry::*;
pub use snapshot::ConstitutionalSnapshot;
pub use snapshot_scope::{RestorationOutcome, SnapshotScope, SnapshotScopeRelationship};
pub use state_anchor::ConstitutionalStateAnchor;
pub use state_anchor_scope::{AnchorScopeRelationship, StateAnchorScope};
pub use transition_commitment::TransitionCommitment;
pub use transition_evidence::TransitionEvidence;
pub use witness_type::WitnessType;
