AmunChain — Unified Documentation Index
========================================

This file is the single entry point to every specification, audit
report, architecture description, protocol law, and milestone
document in the AmunChain repository.

Each entry is listed with its path relative to the repository root,
a one-line summary, and the phase or component it belongs to.

----------------------------------------------------------------
Core Protocol & Constitution
----------------------------------------------------------------
constitution/taxonomy.md
    Canonical taxonomy of all constitutional concepts.
constitution/CONSTITUTIONAL_TRUST_LAW.md
    Trust anchor derivation and verification rules.
constitution/CONSTITUTIONAL_SIGNING_LAW.md
    Signature format, domain separation, and signing ceremony.
constitution/SIGNATURE_FORMAT.md
    Binary layout of all signature types.
constitution/SIGNATURE_REPLAY_BINDING.md
    Replay protection through domain-separated signing payloads.
constitution/LEVEL0_CONSTITUTIONAL_FREEZE.md
    Rules for freezing constitutional state at Level 0.
constitution/CROSS_CIVILIZATION_COMPATIBILITY.md
    Inter-civilisation compatibility constraints.
constitution/MULTI_KEY_SOVEREIGNTY.md
    Multi-key sovereignty model for federated authorities.
constitution/GENESIS_SPECIFICATION.md
    Genesis block and founding identity specification.
constitution/storage_kernel.md
    Storage kernel constitution and invariants.

----------------------------------------------------------------
Consensus & Fork Choice
----------------------------------------------------------------
constitution/consensus/CONSENSUS_SAFETY_LAW.md
    Safety guarantees for the constitutional consensus protocol.
constitution/consensus/FORK_CHOICE_LAW.md
    Fork choice rule under constitutional constraints.
constitution/consensus/CONSENSUS_ERROR_MODEL.md
    Error classification for consensus faults.
constitution/consensus/NETWORK_ADVERSARY_LAW.md
    Adversary model for the networking layer.
constitution/consensus/REPLAY_DETERMINISM_LAW.md
    Deterministic replay requirements for consensus.

----------------------------------------------------------------
Federation & Authority
----------------------------------------------------------------
constitution/federation/FEDERATION_PROTOCOL.md
    Federation protocol specification.
constitution/federation/FEDERATION_HANDSHAKE.md
    Federation handshake and negotiation.
constitution/federation/FEDERATION_CERTIFICATE_FORMAT.md
    Certificate format for federation members.
constitution/federation/FEDERATION_REGISTRY.md
    Registry of active federations.
constitution/FEDERATION_PROTOCOL.md
    (Duplicate entry — canonical version above.)
docs/federation/FEDERATION_ARCHITECTURE.md
    High-level federation architecture.
docs/federation/TRUST_GRAPH.md
    Trust graph construction and maintenance.

----------------------------------------------------------------
Registry & Identity
----------------------------------------------------------------
constitution/registry/KEY_REGISTRY.md
    Key registry specification.
constitution/registry/SIGNATURE_REGISTRY.md
    Signature registry for validator identities.
genesis/FOUNDING_IDENTITY.md
    Founding identity and genesis authority derivation.

----------------------------------------------------------------
Storage & Snapshot
----------------------------------------------------------------
crates/amun-storage-kernel/CONSTITUTION.md
    Storage kernel constitution.
crates/amun-storage-kernel/SPECIFICATION.md
    Storage kernel formal specification.
crates/amun-storage-kernel/SNAPSHOT_CONSTITUTION.md
    Snapshot format and verification rules.
crates/amun-storage-kernel/ATOMIC_SNAPSHOT_CONSTITUTION.md
    Atomic snapshot guarantees.
crates/amun-storage-kernel/CANONICAL_TRAVERSAL_LAW.md
    Canonical traversal rules for storage.
crates/amun-storage-kernel/REPLAY_LAW.md
    Replay determinism law for storage.
crates/amun-storage-kernel/CONSTITUTIONAL_DEBT.md
    Known constitutional debt in the storage layer.
crates/amun-storage-kernel/VALIDITY_HIERARCHY.md
    Validity hierarchy for storage operations.

----------------------------------------------------------------
Execution & Runtime
----------------------------------------------------------------
constitution/PHASE3A_ATOMIC_RUNTIME.md
    Atomic runtime specification.
constitution/PHASE3B_PERSISTENT_STORAGE.md
    Persistent storage specification.
constitution/PHASE3_INDEX.md
    Phase 3 index and overview.
constitution/PHASE4_CONSENSUS_ENGINE.md
    Consensus engine specification.
constitution/PHASE4_INDEX.md
    Phase 4 index.
constitution/PHASE4_SUMMARY.md
    Phase 4 summary.
constitution/PHASE5_COMPLETE.md
    Phase 5 completion report.
constitution/PHASE6_INDEX.md
    Phase 6 index.
constitution/PHASE6_NETWORK_LAYER_AND_BYZANTINE_HARDENING.md
    Networking layer and Byzantine hardening.
constitution/PHASE6_SUMMARY.md
    Phase 6 summary.
constitution/RELEASE_SPECIFICATION.md
    Release specification template.

----------------------------------------------------------------
Constitutional Mathematics (Phases 111-116)
----------------------------------------------------------------
docs/constitutional-mathematics/README.md
    Overview of the constitutional mathematics program.
docs/constitutional-mathematics/PHASE_111_FOUNDATIONS_V1.md
    Phase 111: Mathematical foundations v1.
docs/constitutional-mathematics/PHASE_112_SIMULATION_SPECIFICATION.md
    Phase 112: Simulation specification.
PHASE116_FINAL_ANALYSIS.md
    Phase 116 final analysis report.

----------------------------------------------------------------
Audit & Validation
----------------------------------------------------------------
validation/BASELINE.md
    Audit baseline definition.
validation/GATES.md
    Quality gates for each validation phase.
validation/KNOWLEDGE_BASE.md
    Audit knowledge base.
validation/REPORT.md
    Master audit report.
docs/audit/SECURITY_INVARIANTS.md
    Security invariants checklist.
docs/audit/THREAT_MODEL.md
    Threat model for AmunChain.
docs/audit/TRACEABILITY_MATRIX.md
    Traceability matrix linking requirements to tests.
docs/audit/AUDIT_EVIDENCE_BUNDLE.md
    Evidence bundle specification for audits.
docs/CONSTITUTIONAL_AUDIT.md
    Constitutional audit methodology.

----------------------------------------------------------------
Validation Evidence (NV-01 through NV-08)
----------------------------------------------------------------
validation/evidence/NV-01/MANIFEST.md
validation/evidence/NV-01/RESULT.md
validation/evidence/NV-01B/MANIFEST.md
validation/evidence/NV-01B/RESULT.md
validation/evidence/NV-02A/RESULT.md
validation/evidence/NV-02B/RESULT.md
validation/evidence/NV-02C/RESULT.md
validation/evidence/NV-02D/RESULT.md
validation/evidence/NV-03A/RESULT.md
validation/evidence/NV-03B/RESULT.md
validation/evidence/NV-04B/RESULT.md
validation/evidence/NV-04D/RESULT.md
validation/evidence/ROOT_SNAPSHOT_FORMAT.md
    Root snapshot format specification.

----------------------------------------------------------------
Validation Reports (Certifications)
----------------------------------------------------------------
validation/reports/NV-01_AUDIT.md
validation/reports/NV-01_CERTIFICATION.md
validation/reports/NV-01B_AUDIT.md
validation/reports/NV-01B_CERTIFICATION.md
validation/reports/NV-02_CERTIFICATION.md
validation/reports/NV-03_CERTIFICATION.md
validation/reports/NV-04_CERTIFICATION.md
validation/reports/NV-05_CERTIFICATION.md
validation/reports/NV-05_5_CERTIFICATION.md
validation/reports/NV-06_CERTIFICATION.md
validation/reports/NV-07_CERTIFICATION.md
validation/reports/NV-08_CERTIFICATION.md
validation/reports/CERTIFICATION_TEMPLATE.md
    Certification report template.

----------------------------------------------------------------
Architecture & Design
----------------------------------------------------------------
docs/CRATE_ARCHITECTURE.md
    Overview of all crates and their roles.
docs/REPOSITORY_LAYOUT.md
    Repository directory structure.
docs/PROJECT_INDEX.md
    Project-wide index of all documents and phases.
docs/architecture/CRATE_CLASSIFICATION.md
    Crate classification by layer.
docs/architecture/DEPENDENCY_CONSTITUTION.md
    Dependency constitution and rules.
docs/architecture/DEPENDENCY_RULES.md
    Dependency rules enforcement.
docs/architecture/NAMING_CONVENTION.md
    Naming convention for crates and modules.
docs/architecture/PHASE_49_COMPLETE.md
    Phase 49 completion report.
docs/architecture/SECRETS_AUDIT.md
    Secrets audit methodology.
docs/architecture/TIMING_AUDIT.md
    Timing audit methodology.

----------------------------------------------------------------
Developer Guides & Operations
----------------------------------------------------------------
docs/DEVELOPER_GUIDE.md
    Developer setup and contribution guide.
docs/BUILD_REPRODUCIBILITY.md
    Build reproducibility guarantees.
docs/CANONICAL_HASH.md
    Canonical hashing specification.
docs/CANONICAL_SERIALIZATION.md
    Canonical serialization specification.
docs/SECURITY_MODEL.md
    Security model overview.
docs/miri-known-issues.md
    Known issues with Miri (undefined behavior detector).
CHANGELOG.md
    Release changelog.
SECURITY.md
    Security policy and responsible disclosure.
CODE_OF_CONDUCT.md
    Community code of conduct.
TRADEMARKS.md
    Trademark policy.
README.md
    Repository README.

----------------------------------------------------------------
Milestone Documents (N-series)
----------------------------------------------------------------
N1_N8_CORE_BLOCKCHAIN.md
    N1-N8: Core blockchain implementation.
N9_N11_NETWORK_DISTRIBUTION.md
    N9-N11: Network distribution layer.
N12_N13_CHECKPOINT_SYSTEM.md
    N12-N13: Checkpoint system.
N14_N16_BOOTSTRAP.md
    N14-N16: Bootstrap protocol.
N17_MULTI_NODE_NETWORK.md
    N17: Multi-node network deployment.
N18_N19_REJOIN.md
    N18-N19: Rejoin protocol.
N20_REAL_NETWORK_TRANSPORT.md
    N20: Real network transport (TCP).
N21_CONSTITUTIONAL_IDENTITY.md
    N21: Constitutional identity system.
N22_PUBLIC_TESTNET.md
    N22: Public testnet deployment.
N48_5A_CONSTITUTIONAL_PROGRAMS_SPECIFICATION.md
    N48.5A: Constitutional programs specification.
N48_5B_CONSTITUTIONAL_RESOURCE_MODEL.md
    N48.5B: Constitutional resource model.
N48_5C_CONTRACT_STATE_MODEL.md
    N48.5C: Contract state model.
N48_5C1_LINEAGE_INTEGRITY_IDENTITY_AND_TRANSFORMATION_LEGALITY.md
    N48.5C1: Lineage integrity, identity, and transformation legality.
N48_5D_CONSTITUTIONAL_VIRTUAL_MACHINE.md
    N48.5D: Constitutional virtual machine.
N48_5E_CONSTITUTIONAL_RUNTIME.md
    N48.5E: Constitutional runtime.
N48_5_SECTION7_EXPERIMENTAL_EVALUATION.md
    N48.5 Section 7: Experimental evaluation.
docs/N100_NODE_LIFECYCLE_SUMMARY.md
    N100: Node lifecycle summary.
docs/N105_CRYPTOGRAPHIC_VALIDATOR_IDENTITY.md
    N105: Cryptographic validator identity.
docs/N106_N107_CONSTITUTIONAL_AUTHORITY_AND_GOVERNANCE.md
    N106-N107: Constitutional authority and governance.
docs/N110_COMPLETE.md
    N110: Provable economic slashing pipeline (complete).

----------------------------------------------------------------
Research & Proposals
----------------------------------------------------------------
docs/CCS_Core_Specification_v1.0.md
    CCS core specification v1.0.
docs/CCS_Core_Specification_v1.1.md
    CCS core specification v1.1.
docs/CCS_RUNTIME_MAPPING.md
    CCS runtime mapping.
docs/CONSTITUTIONAL_MODEL.md
    Constitutional model overview.
docs/V5_CCS_RESEARCH_PROGRAM.md
    V5: CCS research program.
docs/V5_CCS_COMPLETE_RESEARCH_PROGRAM.md
    V5: CCS complete research program.
docs/V8_RESEARCH_PROPOSAL.md
    V8: Research proposal.
docs/V4_002_003_CCS_THEORY_NUCLEUS.md
    V4.002-003: CCS theory nucleus.

----------------------------------------------------------------
Protocol Laws & Formal Specifications
----------------------------------------------------------------
docs/protocol/replay_physics_v1.md
    Replay physics v1.
docs/protocol/lineage_law_v1.md
    Lineage law v1.
docs/protocol/FREEZE_CERTIFICATE_v1.md
    Freeze certificate v1.
docs/REPLAY_LAW.md
    Replay law.
docs/REPLAY_MODEL.md
    Replay model.
docs/consensus/constitution.md
    Consensus constitution.
docs/consensus/origin.md
    Consensus origin document.
docs/constitutional_layers.md
    Constitutional layers overview.
docs/constitutional/topology.md
    Constitutional topology.
docs/constitutional/phase84_freeze.md
    Phase 84: Freeze protocol.
docs/constitutional/phase85_seal.md
    Phase 85: Seal protocol.

----------------------------------------------------------------
Phase 80a (Freeze & Numeric Constitution)
----------------------------------------------------------------
phase_80a/docs/freeze_report.md
    Freeze operation report.
phase_80a/docs/hashing_constitution.md
    Hashing constitution.
phase_80a/docs/numeric_constitution.md
    Numeric constitution.
phase_80a/docs/overflow_constitution.md
    Overflow constitution.
phase_80a/docs/rounding_constitution.md
    Rounding constitution.
phase_80a/docs/transcendental_constitution.md
    Transcendental constitution.

----------------------------------------------------------------
Versioned Milestone Documents
----------------------------------------------------------------
docs/V0_3_COMPLETION.md
    v0.3 completion report.
docs/V2_003_DISCOVERY.md
    v2.003 discovery document.
docs/V2_016G_PARTITION_SAFETY_RECOVERY.md
    v2.016G: Partition safety and recovery.
docs/V2_016N_40_40_RECOVERY.md
    v2.016N: 40-40 recovery protocol.
docs/V2_023_PARTITION_FINAL.md
    v2.023: Partition final analysis.
docs/V2_026_ADVERSARIAL_RECONCILIATION.md
    v2.026: Adversarial reconciliation.
docs/V2_027_FORMAL_CONSTITUTIONAL_STATE.md
    v2.027: Formal constitutional state.
docs/V3_004_SIGNATURE_VERIFICATION.md
    v3.004: Signature verification.
docs/V3_005D_QC_CRYPTOGRAPHIC_PROOF.md
    v3.005D: QC cryptographic proof.
docs/V3_006_BYZANTINE_CONSTITUTIONAL_ATTACKS.md
    v3.006: Byzantine constitutional attacks.
docs/V3_006D_FOREIGN_AUTHORITY.md
    v3.006D: Foreign authority.
docs/V6_001_GEOMETRY_OF_AUTHORITY.md
    V6.001: Geometry of authority.
docs/V6_002_PATHS_NOT_POINTS.md
    V6.002: Paths not points.
docs/V6_003_CANONICALIZATION.md
    V6.003: Canonicalization.
docs/V6_004_CONSTITUTIONAL_SELECTION_PRINCIPLE.md
    V6.004: Constitutional selection principle.
docs/V6_005_CONTINUITY_ORDERING.md
    V6.005: Continuity ordering.
docs/V6_006_CONSTITUTIONAL_CONVERGENCE.md
    V6.006: Constitutional convergence.
docs/V6_007_CONSTITUTIONAL_PREFERENCE.md
    V6.007: Constitutional preference.
docs/V6_008_CONSTITUTIONAL_RESOLUTION.md
    V6.008: Constitutional resolution.
docs/V6_009_CONSTITUTIONAL_META_RESOLUTION.md
    V6.009: Constitutional meta-resolution.
docs/V6_010_CONSTITUTIONAL_CONSTRAINT_PRINCIPLE.md
    V6.010: Constitutional constraint principle.
docs/V7_001_CONSTITUTIONAL_DERIVABILITY.md
    V7.001: Constitutional derivability.
docs/V7_002_DERIVABILITY_GEOMETRY.md
    V7.002: Derivability geometry.
docs/V7_003_CONSTITUTIONAL_REDUCTION.md
    V7.003: Constitutional reduction.
docs/V7_004_CONSTITUTIONAL_CONFLUENCE.md
    V7.004: Constitutional confluence.
docs/V7_005_CONSTITUTIONAL_PRUNING.md
    V7.005: Constitutional pruning.
docs/V7_006_CONSTITUTIONAL_EXCLUSION.md
    V7.006: Constitutional exclusion.
docs/V7_007_DERIVABILITY_AND_EXCLUSION.md
    V7.007: Derivability and exclusion.
docs/V7_008_CONSTITUTIONAL_CLOSURE.md
    V7.008: Constitutional closure.
docs/V7_009_WHY_CLOSURE.md
    V7.009: Why closure.
docs/V7_010_SINGLE_HISTORY_PRINCIPLE.md
    V7.010: Single history principle.
docs/V8_001_INDEPENDENCE_OF_SH1.md
    V8.001: Independence of SH1.
docs/V8_002_CONSTITUTIONAL_COMPARABILITY.md
    V8.002: Constitutional comparability.

----------------------------------------------------------------
Flagship Paper & Appendices
----------------------------------------------------------------
AMUNCHAIN_FLAGSHIP_PAPER_DRAFT.md
    Flagship paper draft.
AMUNCHAIN_CONSTITUTIONAL_KERNEL.md
    Constitutional kernel overview.
AMUNCHAIN_STATE_V01.md
    State specification v0.1.
APPENDIX_A_FORMAL_DEFINITIONS_AND_PROOFS.md
    Appendix A: Formal definitions and proofs.
SECTION7_FINAL.md
    Section 7 final.
SECTION8_FINAL.md
    Section 8 final.
REPLAY_COST_ANALYSIS.md
    Replay cost analysis.

----------------------------------------------------------------
Status & Roadmap
----------------------------------------------------------------
STATUS.md
    Current project status.
docs/PROTOCOL_HARDENING_ROADMAP.md
    Protocol hardening roadmap.
docs/VALIDATOR_ORDERING.md
    Validator ordering specification.
docs/PHASE_84_UNIFICATION.md
    Phase 84 unification document.
RELEASE_BASELINES/v0.3.1-hardening-final.md
    v0.3.1 hardening final baseline.

----------------------------------------------------------------
Test & Benchmark Artifacts
----------------------------------------------------------------
artifacts/n102.3/results.md
    N102.3 benchmark results.
fixtures/replay/checkpoint/README.md
    Replay checkpoint fixtures.
fixtures/replay/divergence/README.md
    Replay divergence fixtures.
fixtures/replay/equivalence/README.md
    Replay equivalence fixtures.
fixtures/replay/genesis/README.md
    Replay genesis fixtures.

----------------------------------------------------------------
Genesis & Compatibility
----------------------------------------------------------------
genesis/GENESIS_MANIFEST.md
    Genesis manifest.
genesis/GENESIS_COMPATIBILITY_LAW.md
    Genesis compatibility law.

----------------------------------------------------------------
RFCs
----------------------------------------------------------------
docs/rfcs/README.md
    RFC index and process.

----------------------------------------------------------------
Proof & Evidence Templates
----------------------------------------------------------------
validation/evidence/MANIFEST_TEMPLATE.md
    Manifest template for validation evidence.
validation/reports/CERTIFICATION_TEMPLATE.md
    Certification report template.
