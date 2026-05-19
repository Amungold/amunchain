const ARENA_SIZE: usize = 65536;

pub struct DeterministicArena {
    memory: Box<[u8; ARENA_SIZE]>,
    pub offset: usize,
    pub allocations: u64,
}

impl DeterministicArena {
    pub fn new() -> Self {
        Self {
            memory: Box::new([0u8; ARENA_SIZE]),
            offset: 0,
            allocations: 0,
        }
    }

    pub fn allocate(&mut self, size: usize) -> Option<(&mut [u8], usize)> {
        if self.offset + size > ARENA_SIZE {
            return None;
        }
        let start = self.offset;
        self.offset += size;
        self.allocations += 1;
        Some((&mut self.memory[start..start + size], start))
    }

    pub fn get_slice(&self, offset: usize, len: usize) -> Option<&[u8]> {
        if offset + len > self.offset {
            return None;
        }
        Some(&self.memory[offset..offset + len])
    }

    pub fn used(&self) -> usize {
        self.offset
    }

    pub fn remaining(&self) -> usize {
        ARENA_SIZE - self.offset
    }

    pub fn reset(&mut self) {
        self.offset = 0;
        self.allocations = 0;
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.memory[..self.offset]
    }
}
