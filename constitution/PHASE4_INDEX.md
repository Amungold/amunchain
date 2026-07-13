# Phase 4 — Document Index
## Document Control
| Field | Value |
|-------|-------|
| Document ID | AMUN-PHASE4-INDEX-001 |
| Revision | 1.0.0 |
| Status | Frozen |
| Author | Engineering Team |
| Date | 2026-05-17 |
## Index of Phase 4 Documents
PHASE4_CONSENSUS_ENGINE.md — Full technical specification with architecture, guarantees, test results, and amendment procedure. PHASE4_SUMMARY.md — High-level summary of what was built, design decisions, and security posture. PHASE4_INDEX.md — This file.
## Phase 4 Quick Reference
Crate name: amun-consensus. Crate version: 1.0.0. Source files: 12 modules plus 1 test file. Total tests: 16. Test pass rate: 100 percent. Maximum validators: 256. Byzantine fault tolerance: f less than n over 3. Quorum requirement: 2f plus 1. Signature scheme: BLS12-381 integration ready. Safety rules: 4 (equivocation, lock, quorum, duplicate detection). Liveness mechanism: Exponential backoff with 100 timeout cap. Error handling: ConstitutionalFault taxonomy with audit codes. no_std compatible: Yes. Unsafe code: None outside amun-unsafe boundary. Unwrap in production: None. Panic in production: None.
## Source Module Reference
engine.rs — Consensus orchestrator processes proposals casts votes builds QCs. round.rs — Round state machine Proposal Prepare PreCommit Commit Timeout. validator.rs — Validator registry 256 capacity quorum math vote recording. proposal.rs — Block proposal block epoch round proposer signature. vote.rs — Consensus vote phase round block_hash validator_index signature. qc.rs — Quorum Certificate aggregated verified signatures. safety.rs — Safety rules equivocation lock quorum duplicate signer checks. liveness.rs — Liveness rules timeout logic leader selection. signature_verifier.rs — BLS12-381 verification framework. vote_tracker.rs — Replay protection round validator dedup with pruning. tests.rs — 16 frozen tests across 7 categories.
## Build and Test Commands
cargo build -p amun-consensus, cargo test -p amun-consensus, cargo clippy -p amun-consensus, cargo fmt --check -p amun-consensus
## Amendment Log
2026-05-17, Revision 1.0.0, Initial freeze, Engineering Team.
End of index. Status: FROZEN. Revision: 1.0.0. Date: 2026-05-17.
