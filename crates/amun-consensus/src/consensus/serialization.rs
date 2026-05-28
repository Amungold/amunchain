pub struct CanonicalEncoder { buf: Vec<u8> }
impl CanonicalEncoder {
    pub fn new() -> Self { Self { buf: Vec::new() } }
    pub fn write_u64(&mut self, v: u64) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    pub fn write_u32(&mut self, v: u32) { self.buf.extend_from_slice(&v.to_be_bytes()); }
    pub fn write_u8(&mut self, v: u8) { self.buf.push(v); }
    pub fn write_bytes(&mut self, b: &[u8]) { self.write_u32(b.len() as u32); self.buf.extend_from_slice(b); }
    pub fn write_hash(&mut self, h: &[u8; 32]) { self.buf.extend_from_slice(h); }
    pub fn into_bytes(self) -> Vec<u8> { self.buf }
}
impl Default for CanonicalEncoder { fn default() -> Self { Self::new() } }

pub struct CanonicalDecoder<'a> { data: &'a [u8], pos: usize }
impl<'a> CanonicalDecoder<'a> {
    pub fn new(data: &'a [u8]) -> Self { Self { data, pos: 0 } }
    pub fn read_u64(&mut self) -> Option<u64> { if self.pos + 8 > self.data.len() { return None; } let bytes = &self.data[self.pos..self.pos+8]; self.pos += 8; Some(u64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]])) }
    pub fn read_u32(&mut self) -> Option<u32> { if self.pos + 4 > self.data.len() { return None; } let bytes = &self.data[self.pos..self.pos+4]; self.pos += 4; Some(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])) }
    pub fn read_u8(&mut self) -> Option<u8> { if self.pos >= self.data.len() { return None; } let v = self.data[self.pos]; self.pos += 1; Some(v) }
    pub fn read_bytes(&mut self) -> Option<Vec<u8>> { let len = self.read_u32()? as usize; if self.pos + len > self.data.len() { return None; } let bytes = self.data[self.pos..self.pos+len].to_vec(); self.pos += len; Some(bytes) }
    pub fn read_hash(&mut self) -> Option<[u8; 32]> { if self.pos + 32 > self.data.len() { return None; } let mut hash = [0u8; 32]; hash.copy_from_slice(&self.data[self.pos..self.pos+32]); self.pos += 32; Some(hash) }
    pub fn is_exhausted(&self) -> bool { self.pos >= self.data.len() }
}
