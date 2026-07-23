# Storage/WAL Audit

**Status:** Complete  
**Baseline:** Commitment Layer V1

## Findings

| Area | Mechanism | Status |
|------|-----------|--------|
| Crash Recovery | ChainStore::open() + load_tip() | ✅ |
| Recovery Tests | n70_recover_after_restart, n71_recover | ✅ |
| WAL Module | amun-wal crate exists | ✅ |
| Atomic Framing | offset + length prefix | ✅ |
| Snapshot Integrity | verify_snapshot() with manifest | ✅ |
| Duplicate Detection | Height-based dedup | ✅ |
| Corruption Detection | StoreError::Corrupted | ⚠️ No per-record checksum |
| fsync | OS-level only | ⚠️ |

## Recommendations

| ID | Item | Priority |
|----|------|----------|
| STOR-1 | Per-record checksum in ChainStore | P2 |
| STOR-2 | Explicit fsync after each append | P2 |
| STOR-3 | Snapshot fuzz testing | P3 |
