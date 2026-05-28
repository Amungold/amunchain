//! Architectural Invariants — constitutional laws of the kernel.
//!
//! These invariants are ENFORCED BY DESIGN, not by runtime checks.
//! They represent the constitutional foundation of the artifact graph.
//!
//! VIOLATING ANY OF THESE INVARIANTS IS A CONSTITUTIONAL FAILURE.

/// INVARIANT 1: Context Locality
///
/// Artifact identity is context-scoped. Sequencing, ancestry,
/// replay locality, and admissibility are all local within a
/// single `context_hash`. There is NO global namespace for
/// artifact sequencing or identity.
///
/// CONSEQUENCE: Two artifacts in different contexts may share
/// sequence numbers, IDs, or other local identifiers without
/// conflict. Cross-context relationships are explicit
/// (via hash references), not implicit (via global ordering).
pub const INVARIANT_CONTEXT_LOCALITY: &str =
    "Artifact identity is context-scoped. No global namespace.";

/// INVARIANT 2: Certificate Non-Authority
///
/// A ReplayCertificate attests replay ADMISSIBILITY, not
/// execution finality. Certificate existence does NOT imply
/// that the state transition achieved consensus or finality.
///
/// CONSEQUENCE: Certificates are replay witnesses, not
/// consensus proofs. Finality is a consensus concern layered
/// ABOVE the constitutional kernel, never within it.
pub const INVARIANT_CERTIFICATE_NON_AUTHORITY: &str =
    "Certificate attests admissibility, not finality. Finality is above the kernel.";

/// INVARIANT 3: Hash-Domain Governance
///
/// Hash domains (in `hash_domains.rs`) are constitutional
/// constants. Adding, removing, or renaming a domain is a
/// constitutional governance action that invalidates ALL
/// historical artifacts hashed under the old domain set.
///
/// CONSEQUENCE: Domain changes require constitutional revision
/// and explicit migration. There is no "silent" domain change.
pub const INVARIANT_HASH_DOMAIN_GOVERNANCE: &str =
    "Hash domain changes are constitutional governance. No silent changes.";

/// INVARIANT 4: Independent Edge Verifiability
///
/// Every edge in the artifact graph MUST be independently
/// verifiable without requiring global state, runtime
/// environment, storage lookups, or network access.
///
/// CONSEQUENCE: Given two artifact hashes, the validity of
/// the edge between them can be determined from the artifacts
/// themselves. The graph is deterministically reconstructible
/// from its leaves.
pub const INVARIANT_INDEPENDENT_EDGE_VERIFIABILITY: &str =
    "Every artifact edge is independently verifiable. No global state required.";

/// INVARIANT 5: Receipt Terminality
///
/// ExecutionReceipts are TERMINAL artifacts. They witness
/// execution but NEVER participate in it. A receipt cannot
/// be an input to execution, only an output.
///
/// CONSEQUENCE: The receipt graph is a DAG rooted at execution
/// artifacts. There are NO cycles involving receipts.
pub const INVARIANT_RECEIPT_TERMINALITY: &str =
    "Receipts are terminal witnesses. They never participate in execution.";

/// INVARIANT 6: Artifact Immutability
///
/// Once created and hashed, a constitutional artifact is
/// IMMUTABLE. Its hash is its identity. Any change to an
/// artifact produces a NEW artifact with a NEW hash.
///
/// CONSEQUENCE: There is no "update in place" for
/// constitutional artifacts. Lineage is expressed through
/// hash chains, not mutation.
pub const INVARIANT_ARTIFACT_IMMUTABILITY: &str =
    "Constitutional artifacts are immutable. Identity = hash.";

/// All invariants for verification and documentation.
pub const ALL_INVARIANTS: &[&str] = &[
    INVARIANT_REPLAY_DERIVED_STATE_IDENTITY,
    INVARIANT_RESTORATION_LINEAGE_VALIDITY,
    INVARIANT_RESTORATION_CONTINUATION,
    INVARIANT_CONSTITUTIONAL_CAUSAL_VALIDITY,
    INVARIANT_PROOF_SUFFICIENCY,
    INVARIANT_RUNTIME_EXECUTION_INDEPENDENCE,
    INVARIANT_DOMAIN_SEPARATION,
    INVARIANT_CAUSAL_ACYCLICITY,
    INVARIANT_RUNTIME_TRUTH_ISOLATION,
    INVARIANT_CANONICAL_SERIALIZATION,
    INVARIANT_NETWORK_PHENOMENON_AUTHORITY,
    INVARIANT_CONTEXT_LOCALITY,
    INVARIANT_CERTIFICATE_NON_AUTHORITY,
    INVARIANT_HASH_DOMAIN_GOVERNANCE,
    INVARIANT_INDEPENDENT_EDGE_VERIFIABILITY,
    INVARIANT_RECEIPT_TERMINALITY,
    INVARIANT_ARTIFACT_IMMUTABILITY,
];

/// INVARIANT 7: Replay-Derived State Identity
///
/// Constitutional state identity is derived exclusively from
/// deterministic replay lineage and attested transcript scope,
/// never from mutable storage state or runtime execution environment.
///
/// CONSEQUENCE: State roots are attestations of replay outcomes,
/// not pointers to storage locations. The same state root means
/// the same replay lineage produced it — not the same database
/// contents.
pub const INVARIANT_REPLAY_DERIVED_STATE_IDENTITY: &str =
    "State identity is replay-derived, not storage-derived.";

/// INVARIANT 8: Restoration Lineage Validity
///
/// Snapshot restoration validity is derived from replay lineage,
/// anchor lineage, certificate scope, and constitutional revisions —
/// never from database integrity, disk integrity, or serialization
/// correctness alone.
///
/// CONSEQUENCE: A snapshot is valid if and only if its constitutional
/// lineage is intact. Storage-level integrity is necessary but not
/// sufficient for constitutional validity.
pub const INVARIANT_RESTORATION_LINEAGE_VALIDITY: &str =
    "Restoration validity is lineage-derived, not storage-derived.";

/// INVARIANT 9: Restoration Continuation
///
/// Restoration does not create a new replay lineage. It continues
/// the existing lineage from a constitutionally admissible snapshot.
/// The restored execution is a CONTINUATION of the original replay,
/// not a new execution branch.
///
/// CONSEQUENCE: After restoration, the replay journal continues
/// from the snapshot's transcript position. The lineage chain
/// (context → boundary → evidence → commitment → receipt) remains
/// unbroken across the restoration boundary.
pub const INVARIANT_RESTORATION_CONTINUATION: &str =
    "Restoration continues the existing lineage. It is not a new branch.";

/// INVARIANT 10: Constitutional Causal Validity
///
/// Constitutional validity is causally derived, not temporally derived.
/// An artifact's temporal position (when it was created) does not imply
/// constitutional dependency. Dependency must be explicit and verifiable.
///
/// CONSEQUENCE: Two artifacts created at the same time may have no
/// causal relationship. An artifact created later may not depend on
/// an artifact created earlier unless there is an explicit causal edge.
pub const INVARIANT_CONSTITUTIONAL_CAUSAL_VALIDITY: &str =
    "Constitutional validity is causally derived, not temporally derived.";

/// INVARIANT 11: Constitutional Proof Sufficiency
///
/// Constitutional admissibility requires sufficient witnesses, not complete
/// historical knowledge. A proof is valid if its witness set is causally
/// closed and contains all hard dependencies. Artifacts outside the witness
/// set do not affect validity.
///
/// CONSEQUENCE: Two witnesses that are causally equivalent (same closure,
/// same hard dependencies) are constitutionally equivalent, regardless of
/// what other artifacts exist in the full lineage.
pub const INVARIANT_PROOF_SUFFICIENCY: &str =
    "Admissibility requires sufficient witnesses, not complete history.";

/// INVARIANT 12: Runtime Execution Independence
///
/// Runtime execution order must not affect constitutional validity.
/// The constitutional kernel judges artifacts based on their content,
/// not on the order they were produced, the worker that produced them,
/// or the scheduling decisions that led to their production.
///
/// CONSEQUENCE: Two identical artifact sets produced by different
/// execution schedules MUST have identical constitutional validity.
pub const INVARIANT_RUNTIME_EXECUTION_INDEPENDENCE: &str =
    "Runtime execution order must not affect constitutional validity.";

/// INVARIANT 13: Constitutional-Operational Domain Separation
///
/// No constitutional object may share a hash domain with an operational
/// object. Constitutional hashing and operational hashing are separate
/// namespaces. This prevents runtime/constitutional contamination,
/// accidental replay equivalence, and operational proof leakage.
pub const INVARIANT_DOMAIN_SEPARATION: &str =
    "Constitutional and operational hash domains are separate namespaces.";

/// INVARIANT 14: Causal Acyclicity
///
/// The constitutional causal graph MUST be acyclic. No artifact may
/// transitively depend on itself. Cycles violate witness minimality,
/// prevent termination of closure verification, and create contradictory
/// admissibility roots.
///
/// CONSEQUENCE: Any cycle in the causal graph is a constitutional
/// violation. All causal edges MUST form a Directed Acyclic Graph (DAG).
pub const INVARIANT_CAUSAL_ACYCLICITY: &str =
    "The constitutional causal graph must be acyclic (DAG).";

/// INVARIANT 15: Runtime Truth Isolation
///
/// Runtime execution side effects MUST NOT alter constitutional truth.
/// The runtime executes and produces artifacts; the constitutional kernel
/// judges those artifacts. No runtime capability, scheduling decision,
/// worker state, or execution timing may influence constitutional validity,
/// witness identity, or proof semantics.
///
/// CONSEQUENCE: Two runtimes with different internal states, schedules,
/// or worker configurations MUST produce constitutionally equivalent
/// results when given the same constitutional inputs.
pub const INVARIANT_RUNTIME_TRUTH_ISOLATION: &str =
    "Runtime side effects cannot alter constitutional truth.";

/// INVARIANT 16: Canonical Serialization Determinism
///
/// Canonical serialization must produce identical bytes for identical
/// constitutional semantics. The byte representation of a constitutional
/// object is deterministic: same fields, same order, same endianness,
/// same encoding rules across all platforms and runtimes.
///
/// CONSEQUENCE: Two constitutionally identical objects serialized on
/// different platforms MUST produce identical byte sequences.
pub const INVARIANT_CANONICAL_SERIALIZATION: &str =
    "Canonical serialization produces identical bytes for identical semantics.";

/// INVARIANT 17: Network Phenomenon Authority Prevention
///
/// No network phenomenon may acquire implicit constitutional authority.
/// Network topology, message ordering, propagation speed, worker count,
/// or any other distributed runtime characteristic MUST NOT become
/// a source of constitutional truth, admissibility, or validity.
///
/// CONSEQUENCE: Constitutional truth must be derivable identically
/// regardless of network conditions, worker availability, or
/// communication patterns. The network is a transport medium, not
/// a truth authority.
pub const INVARIANT_NETWORK_PHENOMENON_AUTHORITY: &str =
    "No network phenomenon may acquire implicit constitutional authority.";

/// INVARIANT 19: Reconciliation Non-Authority
///
/// Frontier reconciliation must never create constitutional authority.
/// Epistemic convergence is not a source of constitutional legitimacy.
/// When workers merge or compare derivational frontiers, the result
/// is an expanded proof surface, not a truth hierarchy.
///
/// CONSEQUENCE: The most complete frontier is not the most authoritative.
/// The most propagated surface is not the most valid.
/// Constitutional derivability alone defines semantic legitimacy.
pub const INVARIANT_RECONCILIATION_NON_AUTHORITY: &str =
    "Frontier reconciliation must never create constitutional authority.";

/// INVARIANT 20: Canonical Non-Exclusivity
///
/// Canonical equivalence representation must never imply derivational
/// exclusivity or semantic privilege. A canonical form is a transport
/// convenience, not a truth hierarchy. Multiple non-canonical surfaces
/// that share the same admissibility outcome remain equally lawful.
///
/// CONSEQUENCE: Canonicalization is compression, not authority.
/// The canonical fingerprint identifies the admissibility class, not
/// the privileged derivation path.
pub const INVARIANT_CANONICAL_NON_EXCLUSIVITY: &str =
    "Canonical equivalence representation must never imply derivational exclusivity.";

/// INVARIANT 21: Operational Hostility Containment
///
/// Operational hostility must be containable without semantic escalation.
/// Runtime suspicion does not imply constitutional invalidity.
/// A worker that behaves adversarially may be quarantined operationally,
/// but this quarantine must not become a constitutional judgment.
///
/// CONSEQUENCE: Containment is operational isolation, not constitutional
/// condemnation. The constitutional kernel judges artifacts, not behaviors.
/// Hostile runtime behavior is an operational concern, not a truth concern.
pub const INVARIANT_OPERATIONAL_HOSTILITY_CONTAINMENT: &str =
    "Operational hostility must be containable without semantic escalation.";

/// INVARIANT 22: Proof Routing Non-Influence
///
/// Proof routing must never influence constitutional admissibility.
/// Shortest path is not best proof. Fastest propagation is not strongest
/// admissibility. Network topology, relay density, and propagation speed
/// are operational concerns that MUST NOT affect constitutional judgment.
///
/// CONSEQUENCE: A proof that arrives through a slow, indirect route is
/// constitutionally identical to one that arrives through the fastest path.
/// Routing optimizes delivery, not truth.
pub const INVARIANT_ROUTING_NON_INFLUENCE: &str =
    "Proof routing must never influence constitutional admissibility.";

/// INVARIANT 23: Adaptive Routing Non-Preference
///
/// Routing adaptivity must never create topological semantic preference.
/// No adaptive optimization may silently elevate topological position
/// into constitutional influence. Relay density, path popularity, and
/// propagation speed are operational metrics — NOT truth indicators.
///
/// CONSEQUENCE: A proof that traverses unpopular relays is constitutionally
/// identical to one that traverses high-density hubs. The routing fabric
/// adapts to optimize delivery, not to legitimize topology.
pub const INVARIANT_ADAPTIVE_NON_PREFERENCE: &str =
    "Routing adaptivity must never create topological semantic preference.";

/// INVARIANT 24: Cryptographic Inspectability
///
/// Cryptographic encapsulation must never replace constitutional
/// derivability with opaque proof authority. ZK proofs may attest
/// to derivability, but the constitutional kernel remains the sole
/// source of truth. Compressed or zero-knowledge proofs must remain
/// constitutionally inspectable, derivationally attributable, and
/// semantically decomposable.
///
/// CONSEQUENCE: A ZK proof that passes verification does not make
/// the underlying derivation "more true" than a non-ZK proof.
/// Cryptographic convenience ≠ constitutional privilege.
pub const INVARIANT_CRYPTOGRAPHIC_INSPECTABILITY: &str =
    "Cryptographic encapsulation must never replace constitutional derivability.";

/// INVARIANT 25: Governance Non-Manufacture of Truth
///
/// Governance ratification must never manufacture constitutional truth.
/// Constitutional evolution must remain derivationally constrained across
/// time. An amendment is valid only if it preserves all constitutional
/// invariants — not because enough entities approved it.
///
/// CONSEQUENCE: Popularity, majority coordination, historical precedent,
/// and temporal succession are operational governance phenomena. They
/// do NOT constitute constitutional legitimacy. Only invariant-preserving
/// derivability makes an amendment constitutionally lawful.
pub const INVARIANT_GOVERNANCE_NON_MANUFACTURE: &str =
    "Governance ratification must never manufacture constitutional truth.";

/// INVARIANT 26: Economic Non-Purchase of Legitimacy
///
/// Economic influence must never purchase constitutional legitimacy.
/// Payment for execution, routing, or storage is an operational
/// transaction — NOT a constitutional transaction. An artifact's
/// admissibility does not increase with the resources spent to
/// produce or transport it.
///
/// CONSEQUENCE: Expensive proofs are not stronger proofs.
/// Wealthy relays are not authoritative relays.
/// Constitutional truth is not for sale.
pub const INVARIANT_ECONOMIC_NON_PURCHASE: &str =
    "Economic influence must never purchase constitutional legitimacy.";

/// INVARIANT 27: Resource-Semantic Neutrality
///
/// Resource concentration must never create semantic concentration.
/// Capital asymmetry, stake gravity, liquidity accumulation, or
/// any form of economic centralization MUST NOT translate into
/// derivational privilege, admissibility preference, or truth authority.
///
/// CONSEQUENCE: A proof produced with minimal resources is constitutionally
/// identical to one produced with vast resources. Economics sustains
/// the runtime, not the truth.
pub const INVARIANT_RESOURCE_SEMANTIC_NEUTRALITY: &str =
    "Resource concentration must never create semantic concentration.";

/// INVARIANT 28: Identity Non-Privilege
///
/// Identity persistence must never create constitutional privilege.
/// Reputation, longevity, institutional trust, and historical presence
/// are operational phenomena — NOT sources of constitutional authority.
/// An artifact's admissibility does not depend on who produced it,
/// for how long they have existed, or how trusted they are.
///
/// CONSEQUENCE: A proof from a new participant is constitutionally
/// identical to one from an ancient institution. Identity is an
/// operational attribute, not a semantic privilege.
pub const INVARIANT_IDENTITY_NON_PRIVILEGE: &str =
    "Identity persistence must never create constitutional privilege.";

/// INVARIANT 29: Federation Non-Supremacy
///
/// Interoperability must never imply constitutional supremacy.
/// Federation enables derivational exchange between constitutions
/// without establishing any constitution as the canonical authority.
/// No constitution may override another's invariants through
/// federation mechanisms.
///
/// CONSEQUENCE: A larger federation is not a more authoritative one.
/// Bridging is transport, not governance. Interoperability is
/// coexistence, not assimilation.
pub const INVARIANT_FEDERATION_NON_SUPREMACY: &str =
    "Interoperability must never imply constitutional supremacy.";

/// INVARIANT 30: Translation Sovereignty Preservation
///
/// Constitutional translation must preserve sovereignty attribution.
/// When a derivation is translated between constitutions, the source
/// constitution's authority over its own semantics must remain intact.
/// Translation is interpretation, not semantic replacement.
///
/// CONSEQUENCE: A translated proof carries its origin sovereignty.
/// The target constitution may interpret it according to its own
/// invariants, but may not claim authority over the source.
pub const INVARIANT_TRANSLATION_SOVEREIGNTY: &str =
    "Translation must preserve sovereignty attribution.";

/// INVARIANT 31: Temporal Non-Precedence
///
/// Temporal precedence must never create constitutional precedence.
/// Earlier existence does not imply semantic superiority. Historical
/// persistence is an operational observation, not a source of
/// constitutional authority. The past is auditable, not sovereign.
///
/// CONSEQUENCE: An old revision is not a more authoritative one.
/// A long-lived federation is not a more legitimate one.
/// Historical inertia must never become constitutional gravity.
pub const INVARIANT_TEMPORAL_NON_PRECEDENCE: &str =
    "Temporal precedence must never create constitutional precedence.";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_invariants_documented() {
        // Verify that all invariants are non-empty (documented)
        for invariant in ALL_INVARIANTS {
            assert!(!invariant.is_empty(), "Invariant must be documented");
        }
    }

    #[test]
    fn test_no_duplicate_invariants() {
        // Verify no duplicate invariant strings
        for (i, left) in ALL_INVARIANTS.iter().enumerate() {
            for right in ALL_INVARIANTS.iter().skip(i + 1) {
                assert_ne!(left, right, "Duplicate invariant found");
            }
        }
    }

    #[test]
    fn test_invariant_count() {
        // If you add an invariant, update this test
        // and add it to ALL_INVARIANTS above
        assert_eq!(ALL_INVARIANTS.len(), 17);
    }
}
