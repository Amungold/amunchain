# AmunChain Security Invariants Catalog

## Resource Invariants (R1–R6 + X1)

| ID | Invariant | Enforcement | Verification |
|----|-----------|-------------|--------------|
| R1 | No duplicate active resource IDs | ResourceRegistry::register_genesis | PCCVVerifier |
| R2 | Consumed resources cannot be used | VMKernel::verify | ReplayVerifier |
| R3 | Child requires consumed parent | ResourceRegistry::consume_and_derive | WitnessBuilder |
| R4 | Certificates are terminal | TransformationMatrix::is_terminal | PCCVVerifier |
| R5 | Cross-contract uniqueness | TransferProofRegistry::consume | StatelessVerifier |
| R6 | Version monotonicity | ResourceRegistry::consume_and_derive | WitnessBuilder |
| X1 | Transfer proof single-use | TransferProofRegistry | N47 Verdict Engine |

## Execution Invariants

| ID | Invariant | Enforcement |
|----|-----------|-------------|
| E1 | Deterministic execution | VM has no non-deterministic inputs |
| E2 | Atomic commit | All operations commit or none |
| E3 | Gas exhaustion produces evidence | GasEngine::execute_with_gas |
| E4 | Handle safety | HandleResolver::validate_handle_safety |
| E5 | No handle leaks | HandleResolver::detect_leaks |

## Consensus Invariants

| ID | Invariant | Enforcement |
|----|-----------|-------------|
| C1 | Replay before vote | ReplayBackedConsensus::form_consensus |
| C2 | Quorum threshold | QC::is_valid |
| C3 | Five-root binding | ConstitutionalFinalityCertificate::verify |
| C4 | No conflicting finality | Theorem 5 (Replay-Backed Safety) |

## Cryptographic Invariants

| ID | Invariant | Enforcement |
|----|-----------|-------------|
| K1 | Secret keys never serialized | ValidatorKeypair: !Serialize |
| K2 | Signatures verified before acceptance | ValidatorPublicKey::verify |
| K3 | Anti-replay protection | AntiReplayGuard::check_and_record |
| K4 | Key rotation chain integrity | KeyRotationChain::verify_chain |
