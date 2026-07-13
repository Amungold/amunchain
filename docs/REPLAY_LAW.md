# REPLAY LAW — CONSTITUTIONAL REPLAY SPECIFICATION

## Phase 81 — Deterministic Replay Constitution

Project: AmunChain
Status: DRAFT CONSTITUTIONAL SPECIFICATION
Date: 2026-05-23

================================================================

1. PURPOSE

This document defines the constitutional replay law of AmunChain.

Replay guarantees:
- deterministic execution
- transcript reproducibility
- replay certification
- consensus reproducibility
- architecture-independent execution

================================================================

2. FUNDAMENTAL REPLAY PRINCIPLE

Replay(GenesisState, Transcript) = FinalState

ReplayHash = OriginalReplayHash

Replay MUST always produce identical output.

================================================================

3. CONSTITUTIONAL REPLAY DEFINITION

Replay is defined as:

Sequential deterministic re-execution of a canonical transition transcript.

Replay is NOT:
- speculative
- parallel
- scheduler-dependent
- runtime-dependent
- architecture-dependent

================================================================

4. REPLAY INPUT MODEL

Replay input:
- Genesis State
- Canonical Transcript
- Constitutional Rules
- Deterministic State Algebra

================================================================

5. REPLAY OUTPUT MODEL

Replay output:
- Final State
- Final State Root
- Replay Transcript Hash
- Replay Certificate

================================================================

6. TRANSCRIPT LAW

Canonical transcript structure:

Transcript = Vec<VerifiedTransitionWitness>

Transcript order is constitutionally frozen.

================================================================

7. WITNESS STRUCTURE

Each witness contains:
- pre_state_hash
- post_state_hash
- transition_hash
- input_hash
- output_hash
- gas_used

================================================================

8. WITNESS CONTINUITY LAW

post_hash(i) == pre_hash(i+1)

Violation invalidates replay.

================================================================

9. SEQUENTIAL REPLAY LAW

S(n+1) = T(n)(S(n))

Transitions are NOT commutative.

================================================================

10. REPLAY EQUIVALENCE

Replay-equivalent iff:
- transition ordering identical
- canonical bytes identical
- witness transcript identical
- transcript hash identical
- resulting state root identical

================================================================

11. REPLAY DETERMINISM LAW

same transcript
=> same final state
=> same state root
=> same replay hash

================================================================

12. FORBIDDEN REPLAY BEHAVIOR

Forbidden:
- nondeterministic replay
- unordered replay
- parallel replay
- scheduler replay
- wall-clock replay
- hardware replay
- architecture-dependent replay

================================================================

13. REPLAY HASH LAW

ReplayHash = H(Transcript)

H = domain-separated canonical hash

Recommended:
- blake3

================================================================

14. TRANSCRIPT HASH LAW

TranscriptHash =
    H(
        Witness1 ||
        Witness2 ||
        Witness3
    )

Transcript ordering is frozen.

================================================================

15. CONSTITUTIONAL REPLAY CERTIFICATE

Replay certificate proves:
- transcript validity
- witness continuity
- deterministic replay
- final state equivalence

================================================================

16. REPLAY CERTIFICATE COMPONENTS

Certificate SHOULD contain:
- genesis_hash
- final_state_hash
- transcript_hash
- transition_count
- gas_total
- replay_version

================================================================

17. REPLAY FAILURE CONDITIONS

Replay MUST fail if:
- witness continuity breaks
- transition invalid
- canonical decode fails
- state hash mismatch
- transcript mismatch
- version mismatch
- trailing bytes detected

================================================================

18. EXHAUSTION LAW

decode(bytes)
succeeds iff
decoder.is_exhausted() == true

================================================================

19. VERSIONING LAW

Root objects MUST begin with:
CANONICAL_VERSION

Frozen value:
CANONICAL_VERSION = 2

================================================================

20. DETERMINISTIC ORDERING LAW

Required:
- BTreeMap
- BTreeSet
- sorted vectors

Forbidden:
- HashMap
- HashSet
- randomized iteration

================================================================

21. REPLAY SAFETY INVARIANTS

same transcript => same replay result

same order => same replay hash

continuous witnesses => valid replay chain

same canonical bytes => same replay hash

================================================================

22. GAS DETERMINISM LAW

Forbidden:
- timing gas
- hardware gas
- scheduler gas
- memory-layout gas

Allowed:
- deterministic algebraic gas

================================================================

23. REPLAY SECURITY MODEL

Replay security depends on:
- canonical serialization
- sealed hashing
- deterministic ordering
- versioned framing
- transcript continuity

================================================================

24. REPLAY SCOPE

Replay law applies to:
- state transitions
- witness generation
- replay certification
- consensus reproduction
- state reconstruction

================================================================

25. REPLAY NON-GOALS

Replay does NOT guarantee:
- network timing equivalence
- P2P equivalence
- mempool equivalence
- hardware performance equivalence

================================================================

26. RECOMMENDED FUTURE MODEL

Replay Model:
    transcript-equivalent

Execution:
    sequential deterministic

Proof:
    transition-based

State Tree:
    Sparse Merkle Tree

Identity:
    address/account model

================================================================

27. WHAT MUST NOT BE ADDED YET

DO NOT add:
- contracts
- parallel execution
- networking
- storage engine
- RPC
- wallets
- mempool

================================================================

28. CONSTITUTIONAL REPLAY STATUS

Status:
DRAFT — PRE-FREEZE

Replay becomes frozen after:
- witness model freeze
- Merkle proof freeze
- invariant freeze

================================================================

29. FINAL CONSTITUTIONAL REPLAY STATEMENT

Replay is the constitutional heart of AmunChain.

Consensus correctness is reducible to:

deterministic replay equivalence

Everything else is infrastructure.

================================================================

END OF REPLAY LAW DOCUMENT
