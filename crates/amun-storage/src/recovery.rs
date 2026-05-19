use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use heapless::Vec;
use crate::wal::{WriteAheadLog, WalPayload};
use crate::snapshot::StateSnapshot;
use crate::store::PersistentStore;

pub struct RecoveryEngine;

impl RecoveryEngine {
    pub fn recover(snapshot: &StateSnapshot, wal: &WriteAheadLog) -> AmunResult<PersistentStore> {
        if !wal.verify_chain_integrity() {
            return Err(FailureContext::new(ConstitutionalFault::JournalHashMismatch, 0x000B, 0x0050));
        }
        let mut store = PersistentStore::new();
        for (k, v) in &snapshot.entries {
            store.apply_replay(&WalPayload::Set {
                key: { let mut kc: Vec<u8, 32> = Vec::new(); kc.extend_from_slice(k).map_err(|_| FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000B, 0x0040))?; kc },
                value: { let mut vc: Vec<u8, 64> = Vec::new(); vc.extend_from_slice(v).map_err(|_| FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x000B, 0x0041))?; vc },
            })?;
        }
        let committed = wal.committed_records();
        for record in committed {
            store.apply_replay(&record.payload)?;
        }
        if let Some(last_record) = committed.last() {
            if let WalPayload::Commit { state_root, .. } = &last_record.payload {
                let computed_root = Ok(store.state_root)?;
                if computed_root != *state_root {
                    return Err(FailureContext::new(ConstitutionalFault::JournalHashMismatch, 0x000B, 0x0042));
                }
            }
        }
        Ok(store)
    }
}
