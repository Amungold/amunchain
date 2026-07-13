use crate::WriteAheadLog;

#[derive(Debug)]
pub struct RecoveryPoint {
    pub last_sequence: u64,
    pub last_state_root: Option<[u8; 32]>,
    pub events_replayed: u64,
    pub valid: bool,
}

pub fn recover_from_wal(wal: &WriteAheadLog) -> std::io::Result<RecoveryPoint> {
    let events = wal.iter_events()?;
    let last_seq = wal.sequence();
    let count = events.len() as u64;
    Ok(RecoveryPoint { last_sequence: last_seq, last_state_root: None, events_replayed: count, valid: count == wal.frame_count() })
}
