use crate::storage::wal::replay::WALReplayIterator;

pub struct ByzantineDefense;

impl ByzantineDefense {
    pub fn verify_chain(iter: &mut WALReplayIterator) -> bool {
        let mut expected_seq = 0;
        let mut expected_prev = [0u8; 32];
        while let Some(frame) = iter.next() {
            if frame.sequence != expected_seq || frame.prev_hash != expected_prev {
                return false;
            }
            expected_seq += 1;
            expected_prev = frame.entry_hash;
        }
        !iter.corruption_detected
    }
}
