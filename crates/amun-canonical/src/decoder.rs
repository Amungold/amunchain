use crate::error::CanonicalError;

pub struct CanonicalDecoder<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> CanonicalDecoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn read_u64(&mut self) -> Result<u64, CanonicalError> {
        if self.pos + 8 > self.data.len() {
            return Err(CanonicalError::TruncatedData);
        }
        let mut b = [0u8; 8];
        b.copy_from_slice(&self.data[self.pos..self.pos + 8]);
        self.pos += 8;
        Ok(u64::from_le_bytes(b))
    }

    pub fn read_u32(&mut self) -> Result<u32, CanonicalError> {
        if self.pos + 4 > self.data.len() {
            return Err(CanonicalError::TruncatedData);
        }
        let mut b = [0u8; 4];
        b.copy_from_slice(&self.data[self.pos..self.pos + 4]);
        self.pos += 4;
        Ok(u32::from_le_bytes(b))
    }

    pub fn read_bytes(&mut self) -> Result<&[u8], CanonicalError> {
        let len = self.read_u32()? as usize;
        if self.pos + len > self.data.len() {
            return Err(CanonicalError::TruncatedData);
        }
        let slice = &self.data[self.pos..self.pos + len];
        self.pos += len;
        Ok(slice)
    }

    pub fn read_u8(&mut self) -> Result<u8, CanonicalError> {
        if self.pos >= self.data.len() {
            return Err(CanonicalError::TruncatedData);
        }
        let v = self.data[self.pos];
        self.pos += 1;
        Ok(v)
    }
}
