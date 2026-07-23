# Storage/WAL Audit

**Status:** Complete  
**Baseline:** Commitment Layer V1  
**Scope:** `amun-chain-store`, `amun-wal`

## Findings

| Area | Mechanism | Status |
|------|-----------|--------|
| Crash Recovery | ChainStore::open() + load_tip() | ✅ |
| Recovery Tests | n70_recover_after_restart, n71_recover | ✅ |
| WAL Module | amun-wal crate exists | ✅ |
| Atomic Framing | offset + length prefix | ✅ |
| Flush on Append | `f.flush()` + `idx.flush()` in store.rs | ✅ |
| Snapshot Integrity | verify_snapshot() with manifest | ✅ |
| Duplicate Detection | Height-based dedup | ✅ |
| Corruption Detection | StoreError::Corrupted | ⚠️ No per-record checksum |
| Explicit fsync | ⚠️ | Not identified in ChainStore during this audit |

## Recommendations

| ID | Item | Priority |
|----|------|----------|
| STOR-1 | Per-record checksum in ChainStore | P2 |
| STOR-2 | Verify fsync behavior in WAL layer | P2 |
| STOR-3 | Snapshot fuzz testing | P3 |
