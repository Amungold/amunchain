pub struct TlvWriter { buf: Vec<u8> }
impl TlvWriter { pub fn new() -> Self { Self { buf: Vec::new() } } pub fn write_u8(&mut self, tag: u8, val: u8) { self.buf.push(tag); self.buf.extend_from_slice(&1u32.to_be_bytes()); self.buf.push(val); } pub fn into_bytes(self) -> Vec<u8> { self.buf } }
impl Default for TlvWriter { fn default() -> Self { Self::new() } }
