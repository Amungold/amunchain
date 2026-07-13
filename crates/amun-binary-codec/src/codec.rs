const MAX_CANONICAL_PAYLOAD_SIZE: usize = 16 * 1024 * 1024;

#[derive(Debug)]
pub enum CodecError {
    PayloadTooLarge,
    BufferOverflow,
    TruncatedData,
}

pub struct CanonicalEncoder {
    buf: Vec<u8>,
}

impl CanonicalEncoder {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn write_u64(&mut self, v: u64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_u32(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_u8(&mut self, v: u8) {
        self.buf.push(v);
    }

    pub fn write_bytes(&mut self, data: &[u8]) -> Result<(), CodecError> {
        if data.len() > MAX_CANONICAL_PAYLOAD_SIZE {
            return Err(CodecError::PayloadTooLarge);
        }
        self.write_u32(data.len() as u32);
        self.buf.extend_from_slice(data);
        Ok(())
    }

    pub fn write_fixed_bytes(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }

    pub fn finish(self) -> Vec<u8> {
        self.buf
    }
}

pub struct CanonicalDecoder<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> CanonicalDecoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn read_u64(&mut self) -> Option<u64> {
        if self.pos + 8 > self.data.len() {
            return None;
        }
        let v = u64::from_le_bytes(self.data[self.pos..self.pos + 8].try_into().unwrap());
        self.pos += 8;
        Some(v)
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        if self.pos + 4 > self.data.len() {
            return None;
        }
        let v = u32::from_le_bytes(self.data[self.pos..self.pos + 4].try_into().unwrap());
        self.pos += 4;
        Some(v)
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        if self.pos >= self.data.len() {
            return None;
        }
        let v = self.data[self.pos];
        self.pos += 1;
        Some(v)
    }

    pub fn read_bytes(&mut self) -> Option<&[u8]> {
        let len = self.read_u32()? as usize;
        if len > MAX_CANONICAL_PAYLOAD_SIZE {
            return None;
        }
        if self.pos + len > self.data.len() {
            return None;
        }
        let slice = &self.data[self.pos..self.pos + len];
        self.pos += len;
        Some(slice)
    }

    pub fn read_fixed_bytes<const N: usize>(&mut self) -> Option<[u8; N]> {
        if self.pos + N > self.data.len() {
            return None;
        }
        let mut out = [0u8; N];
        out.copy_from_slice(&self.data[self.pos..self.pos + N]);
        self.pos += N;
        Some(out)
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }
}

impl Default for CanonicalEncoder {
    fn default() -> Self {
        Self::new()
    }
}
