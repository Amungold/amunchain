# AmunChain Audit Evidence Bundle

## 1. Invariant-to-Test Traceability

| Invariant | Description | Test ID | Crate | Status |
|-----------|-------------|---------|-------|--------|
| R1 | No duplicate active resource IDs | n49_detect_r1_duplicate_id | amun-pccv | ✅ |
| R2 | Consumed resources unusable | w1_reject_duplicate_genesis | amun-resource-core | ✅ |
| R3 | Child requires consumed parent | w21_lineage_proof_follows_chain | amun-pccv | ✅ |
| R4 | Certificates are terminal | n49_detect_t1_illegal_transformation | amun-pccv | ✅ |
| R5 | Cross-contract uniqueness | w11_reject_double_consumption | amun-cross-contract | ✅ |
| R6 | Version monotonicity | n49_detect_r6_version_regression | amun-pccv | ✅ |
| X1 | Transfer proof single-use | w11_double_consumption_produces_x1_evidence | amun-cross-contract | ✅ |
| E1 | Deterministic execution | w14_replay_equality | amun-replay-verifier | ✅ |
| E2 | Atomic commit | w3_atomic_commit_all_or_nothing | amun-vm-kernel | ✅ |
| E3 | Gas exhaustion evidence | w7_execute_out_of_gas_produces_evidence | amun-gas-engine | ✅ |
| E4 | Handle safety | w9_handle_safety_detects_use_after_consumption | amun-handle-resolver | ✅ |
| E5 | No handle leaks | w9_no_leaks_when_all_resources_reachable | amun-handle-resolver | ✅ |
| C1 | Replay before vote | w18_reject_if_replay_fails | amun-replay-consensus | ✅ |
| C2 | Quorum threshold | w17_reject_insufficient_quorum | amun-consensus-integration | ✅ |
| C3 | Five-root binding | w19_five_roots_bound_in_certificate | amun-evidence-finality | ✅ |
| C4 | No conflicting finality | n60_network_partition_no_double_finality | amun-testnet-sim | ✅ |
| K1 | Secret keys never serialized | key11_secret_types_not_serializable | amun-wallet-management | ✅ |
| K2 | Signature verification | n58_tampered_signature_rejected | amun-crypto-hardening | ✅ |
| K3 | Anti-replay protection | n58_detect_replay | amun-crypto-hardening | ✅ |
| K4 | Key rotation chain integrity | n58_rotation_chain | amun-crypto-hardening | ✅ |

## 2. Byzantine Attack Coverage

| Attack | Test ID | Result |
|--------|---------|--------|
| Forged TransitionProof | byz_001 | ✅ Rejected |
| Double transfer (X1) | byz_002 | ✅ Rejected |
| Lineage cycle injection | byz_003 | ✅ Rejected |
| Version regression | byz_004 | ✅ Rejected |
| Parent hash forgery | byz_005 | ✅ Rejected |
| Illegal transformation | byz_006 | ✅ Rejected |
| Deep lineage bomb (2K) | byz_007 | ✅ Survived |
| Wide fanout flood (10K) | byz_008 | ✅ Survived |
| Proof tampering | byz_009 | ✅ Rejected |
| Proof replay attack | byz_010 | ✅ Rejected |
| Network partition | n60_network_partition | ✅ No double finality |
| Malicious validator | n60_malicious_validator | ✅ Invalid QC rejected |
| Tampered proof (consensus) | n60_tampered_proof | ✅ Rejected |
| Byzantine conflicting blocks | n60_byzantine_conflicting | ✅ Detected |
| Crash + rejoin | n60_crash_recovery | ✅ State preserved |

## 3. Security Assumptions Register

| ID | Assumption | Justification |
|----|-----------|---------------|
| A1 | Blake3 collision-resistant | Widely audited cryptographic hash |
| A2 | Ed25519 existential unforgeability | NIST/CFRG standard, dalek implementation |
| A3 | Honest validators > 2/3 | BFT consensus requirement |
| A4 | Partial synchrony (GST exists) | Standard distributed systems model |
| A5 | Execution determinism | VM has no non-deterministic inputs |
| A6 | No quantum adversary | Ed25519/Blake3 not post-quantum |

## 4. Known Limitations

| ID | Limitation | Mitigation |
|----|-----------|------------|
| L1 | Benchmarks are single-machine microbenchmarks | Network benchmarks planned for N61 |
| L2 | No formal verification of VM against resource laws | Model checking deferred to future work |
| L3 | No BLS threshold signature implementation | Ed25519 used; BLS planned post-N60 |
| L4 | State sync requires trusted history root | Social consensus/checkpointing for root |
| L5 | No persistent validator storage implementation | In-memory simulation; persistence planned |
| L6 | Deep lineage (>2K) has O(depth) construction cost | Ancestor cache; bounded at 2^16 max |

## 5. Test Coverage Summary

| Layer | Crate | Tests | Status |
|-------|-------|-------|--------|
| Resource Model | amun-resource-core | 9 unit + 7 stress | ✅ |
| VM Kernel | amun-vm-kernel | 3 | ✅ |
| Evidence | amun-evidence-engine | 5 | ✅ |
| Transition Proof | amun-transition-proof | 5 | ✅ |
| Bytecode | amun-bytecode | 5 | ✅ |
| Gas Engine | amun-gas-engine | 14 | ✅ |
| Invariant Engine | amun-invariant-engine | 5 | ✅ |
| Handle Resolver | amun-handle-resolver | 4 | ✅ |
| Cross-Contract | amun-cross-contract | 4 | ✅ |
| Proof Archive | amun-proof-archive | 6 | ✅ |
| Runtime | amun-constitutional-runtime | 19 | ✅ |
| Replay | amun-replay-verifier | 3 | ✅ |
| Consensus | amun-replay-consensus | 5 | ✅ |
| Finality | amun-evidence-finality | 3 | ✅ |
| PCCV | amun-pccv | 11 | ✅ |
| Light Client | amun-light-client | 7 | ✅ |
| State Sync | amun-state-sync | 9 | ✅ |
| Networking | amun-validator-networking | 7 | ✅ |
| Crypto | amun-crypto-hardening | 9 | ✅ |
| Byzantine | amun-byzantine-tests | 10 | ✅ |
| Adversarial | amun-testnet-sim | 7 | ✅ |
| Foundation | amun-constitutional-proof | 71 | ✅ |
| Constitution | amun-constitutional | 157 | ✅ |
| **Total** | | **1** | ✅ |

## 6. Reproducibility

- Rust toolchain: nightly-2026-06-01
- Build: `cargo build --workspace --release`
- Tests: `cargo test --workspace`
- Clippy: `cargo clippy --workspace --all-targets -- -D warnings`
- OS: Ubuntu 24.04
- CPU: AMD EPYC 9634
