pub struct CanonicalWriter {
    buf: Vec<u8>,
}

impl Default for CanonicalWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl CanonicalWriter {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            buf: Vec::with_capacity(cap),
        }
    }
    pub fn write_u8(&mut self, v: u8) {
        self.buf.push(v);
    }
    pub fn write_bool(&mut self, v: bool) {
        self.buf.push(if v { 1 } else { 0 });
    }
    pub fn write_u16(&mut self, v: u16) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }
    pub fn write_u32(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }
    pub fn write_u64(&mut self, v: u64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        let len = bytes.len() as u64;
        if len > super::MAX_CANONICAL_ALLOCATION {
            panic!("CanonicalWriter: allocation exceeds constitutional limit");
        }
        self.write_u64(len);
        self.buf.extend_from_slice(bytes);
    }
    pub fn write_hash(&mut self, hash: &[u8; 32]) {
        self.buf.extend_from_slice(hash);
    }
    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.buf
    }
    pub fn len(&self) -> usize {
        self.buf.len()
    }
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
}
