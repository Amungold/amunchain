use crate::storage::wal::writer::WALWriter;
use crate::storage::wal::codec::WALOp;

pub fn create_checkpoint(writer: &WALWriter, state_root: [u8; 32], version: u64) -> std::io::Result<[u8; 32]> {
    writer.append(WALOp::Checkpoint { state_root, version })
}
