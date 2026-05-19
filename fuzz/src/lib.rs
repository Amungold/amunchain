#[cfg(fuzzing)]
pub mod snapshot_fuzz {
    use amun_snapshot_constitution::CanonicalSnapshot;
    use amun_chain_position::ChainPosition;
    use amun_snapshot_constitution::SnapshotExecutionContext;

    pub fn fuzz_snapshot_import(data: &[u8]) {
        let mut entries = Vec::new();
        let mut remaining = data;
        while remaining.len() >= 36 {
            let mut key = [0u8; 32];
            key.copy_from_slice(&remaining[..32]);
            let value_len = u32::from_le_bytes(remaining[32..36].try_into().unwrap_or([0;4])) as usize;
            remaining = &remaining[36..];
            let value_len = value_len.min(remaining.len());
            let value = remaining[..value_len].to_vec();
            remaining = &remaining[value_len..];
            entries.push((key, value));
        }

        let ctx = SnapshotExecutionContext {
            genesis_root: [0u8; 32],
            current_position: ChainPosition::genesis(),
            current_epoch: 0,
            epoch_seal_hash: None,
            execution_version: 1,
            sealed_epochs: Vec::new(),
        };

        let _ = CanonicalSnapshot::new(ChainPosition::genesis(), [0u8; 32], entries, ctx);
    }
}

#[cfg(fuzzing)]
pub mod wal_fuzz {
    use amun_wal::WriteAheadLog;
    use amun_protocol_event::ProtocolEvent;
    use amun_chain_position::ChainPosition;

    pub fn fuzz_wal_roundtrip(data: &[u8]) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("fuzz.wal");

        let mut wal = WriteAheadLog::create(path.clone()).unwrap();
        
        let mut offset = 0;
        while offset + 33 <= data.len() {
            let mut root = [0u8; 32];
            root.copy_from_slice(&data[offset..offset+32]);
            let event = ProtocolEvent::ExecuteTransaction {
                position: ChainPosition::new(0, offset as u64),
                payload: data[offset..offset+32].to_vec(),
                expected_root: root,
            };
            let _ = wal.append_event(&event);
            offset += 33;
        }

        let _ = wal.iter_events();
    }
}
