use super::MAX_CANONICAL_ALLOCATION;

pub struct CanonicalReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> CanonicalReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }
    pub fn is_finished(&self) -> bool {
        self.pos == self.data.len()
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        if self.pos + 1 > self.data.len() {
            None
        } else {
            let v = self.data[self.pos];
            self.pos += 1;
            Some(v)
        }
    }

    pub fn read_bool(&mut self) -> Option<bool> {
        self.read_u8().map(|v| v != 0)
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        if self.pos + 2 > self.data.len() {
            None
        } else {
            let b = [self.data[self.pos], self.data[self.pos + 1]];
            self.pos += 2;
            Some(u16::from_le_bytes(b))
        }
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        if self.pos + 4 > self.data.len() {
            None
        } else {
            let b = [
                self.data[self.pos],
                self.data[self.pos + 1],
                self.data[self.pos + 2],
                self.data[self.pos + 3],
            ];
            self.pos += 4;
            Some(u32::from_le_bytes(b))
        }
    }

    pub fn read_u64(&mut self) -> Option<u64> {
        if self.pos + 8 > self.data.len() {
            None
        } else {
            let b = self.data[self.pos..self.pos + 8].try_into().ok()?;
            self.pos += 8;
            Some(u64::from_le_bytes(b))
        }
    }

    pub fn read_bytes(&mut self) -> Option<Vec<u8>> {
        let len = self.read_u64()? as usize;
        if (len as u64) > MAX_CANONICAL_ALLOCATION {
            return None;
        }
        if self.pos + len > self.data.len() {
            None
        } else {
            let v = self.data[self.pos..self.pos + len].to_vec();
            self.pos += len;
            Some(v)
        }
    }

    pub fn read_hash(&mut self) -> Option<[u8; 32]> {
        if self.pos + 32 > self.data.len() {
            return None;
        }
        let mut a = [0u8; 32];
        a.copy_from_slice(&self.data[self.pos..self.pos + 32]);
        self.pos += 32;
        Some(a)
    }
}
