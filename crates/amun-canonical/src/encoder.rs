use crate::schema::SchemaVersion;
use crate::error::CanonicalError;

pub struct CanonicalEncoder {
    buf: heapless::Vec<u8, 16384>,
    schema: SchemaVersion,
}

impl CanonicalEncoder {
    pub fn new(schema: SchemaVersion) -> Self {
        let mut s = Self {
            buf: heapless::Vec::new(),
            schema,
        };
        let _ = s.buf.extend_from_slice(schema.as_tag());
        s
    }

    pub fn write_u64(&mut self, v: u64) -> Result<(), CanonicalError> {
        self.buf
            .extend_from_slice(&v.to_le_bytes())
            .map_err(|_| CanonicalError::BufferOverflow)
    }

    pub fn write_u32(&mut self, v: u32) -> Result<(), CanonicalError> {
        self.buf
            .extend_from_slice(&v.to_le_bytes())
            .map_err(|_| CanonicalError::BufferOverflow)
    }

    pub fn write_u16(&mut self, v: u16) -> Result<(), CanonicalError> {
        self.buf
            .extend_from_slice(&v.to_le_bytes())
            .map_err(|_| CanonicalError::BufferOverflow)
    }

    pub fn write_u8(&mut self, v: u8) -> Result<(), CanonicalError> {
        self.buf.push(v).map_err(|_| CanonicalError::BufferOverflow)
    }

    pub fn write_bytes(&mut self, b: &[u8]) -> Result<(), CanonicalError> {
        if b.len() > u32::MAX as usize {
            return Err(CanonicalError::InvalidLength);
        }
        self.write_u32(b.len() as u32)?;
        self.buf
            .extend_from_slice(b)
            .map_err(|_| CanonicalError::BufferOverflow)
    }

    pub fn finish(&self) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.schema.as_tag());
        hasher.update(&self.buf);
        let h = hasher.finalize();
        let mut out = [0u8; 32];
        out.copy_from_slice(&h.as_bytes()[..32]);
        out
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf
    }
}
