#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateOperation { Put { key: [u8; 32], value: Vec<u8> }, Delete { key: [u8; 32] } }
#[derive(Debug, Clone)]
pub struct WriteSet { pub operations: Vec<StateOperation> }
impl WriteSet {
    pub fn new() -> Self { Self { operations: Vec::new() } }
    pub fn from_overlay(ops: &[(StateOperation, bool)]) -> Self {
        let mut ws = Self::new();
        for (op, _) in ops { ws.operations.push(op.clone()); }
        ws.operations.sort_by(|a, b| {
            let ka = match a { StateOperation::Put { key, .. }|StateOperation::Delete { key } => key };
            let kb = match b { StateOperation::Put { key, .. }|StateOperation::Delete { key } => key };
            ka.cmp(kb)
        });
        ws
    }
    pub fn is_empty(&self) -> bool { self.operations.is_empty() }
    pub fn iter(&self) -> impl Iterator<Item = &StateOperation> { self.operations.iter() }
}
