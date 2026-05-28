# Protocol Hardening Roadmap

## Phase 1: Consensus Safety (Current Priority)

- [ ] Complete 3-chain commit verification
- [ ] Formal safety proofs for finality
- [ ] Byzantine fault tolerance tests
- [ ] Locked QC monotonicity proofs
- [ ] Fork choice determinism verification

## Phase 2: Deterministic Execution

- [ ] Complete STF (State Transition Function)
- [ ] Eliminate all non-determinism sources
- [ ] Execution receipt canonicalization
- [ ] Cross-version replay compatibility

## Phase 3: Replay Certification

- [ ] Complete replay engine
- [ ] Replay certificate generation
- [ ] Replay attack proof system
- [ ] Deterministic transcript verification

## Phase 4: Validator Lifecycle

- [ ] Validator registration/deregistration
- [ ] Epoch transition mechanism
- [ ] Slashing implementation
- [ ] Stake delegation model

## Phase 5: Storage Proofs

- [ ] State root verification
- [ ] Snapshot generation and sync
- [ ] Light client proofs
- [ ] Merkle tree optimization

## Phase 6: Network Protocol

- [ ] P2P gossip implementation
- [ ] Block propagation
- [ ] Vote aggregation
- [ ] Peer reputation

## Governance Note

**ARCHITECTURE IS FROZEN.** No new governance layers, registries, or enforcement tools.
All development effort goes to PROTOCOL COMPLETION.
