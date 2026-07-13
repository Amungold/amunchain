use crate::write_set::WriteSet;
use amun_chain_position::ChainPosition;
use blake3::Hasher;

#[derive(Debug, Clone)]
pub struct ExecutionReceipt {
    pub position: ChainPosition,
    pub from_root: [u8; 32],
    pub to_root: [u8; 32],
    pub gas_used: u64,
    pub write_set: WriteSet,
    pub exit_code: u8,
}
impl ExecutionReceipt {
    pub fn success(
        pos: ChainPosition,
        from: [u8; 32],
        to: [u8; 32],
        gas: u64,
        ws: WriteSet,
    ) -> Self {
        Self {
            position: pos,
            from_root: from,
            to_root: to,
            gas_used: gas,
            write_set: ws,
            exit_code: 0,
        }
    }
    pub fn failed(pos: ChainPosition, from: [u8; 32], to: [u8; 32], gas: u64, code: u8) -> Self {
        Self {
            position: pos,
            from_root: from,
            to_root: to,
            gas_used: gas,
            write_set: WriteSet::new(),
            exit_code: code,
        }
    }
    pub fn hash(&self) -> [u8; 32] {
        let mut h = Hasher::new();
        h.update(b"AMUN_RECEIPT_V2");
        h.update(&self.position.hash());
        h.update(&self.from_root);
        h.update(&self.to_root);
        h.update(&self.gas_used.to_le_bytes());
        h.update(&[self.exit_code]);
        for op in self.write_set.iter() {
            match op {
                crate::write_set::StateOperation::Put { key, value } => {
                    h.update(key);
                    h.update(value);
                }
                crate::write_set::StateOperation::Delete { key } => {
                    h.update(key);
                }
            }
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(&h.finalize().as_bytes()[..32]);
        out
    }
}
