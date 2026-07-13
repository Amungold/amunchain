// Panic-free canonical writer. ALL methods return WriteResult.
// ShadowBufferWriter provides transactional writes with rollback.

use amun_failure::{module_ids, operation_ids, AmunResult, ConstitutionalFault, FailureContext};

pub type WriteResult = AmunResult<()>;

pub trait CanonicalWriter {
    fn write_bytes(&mut self, bytes: &[u8]) -> WriteResult;
    fn position(&self) -> usize;
}

pub struct BufferWriter<'a> {
    buf: &'a mut [u8],
    pos: usize,
    snapshot: usize,
}

impl<'a> BufferWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self {
            buf,
            pos: 0,
            snapshot: 0,
        }
    }

    pub fn begin(&mut self) {
        self.snapshot = self.pos;
    }
    pub fn commit(&mut self) {}
    pub fn rollback(&mut self) {
        self.pos = self.snapshot;
    }
    pub fn into_written(self) -> &'a mut [u8] {
        &mut self.buf[..self.pos]
    }
}

impl<'a> CanonicalWriter for BufferWriter<'a> {
    fn write_bytes(&mut self, bytes: &[u8]) -> WriteResult {
        let end = self.pos.checked_add(bytes.len()).ok_or_else(|| {
            FailureContext::new(
                ConstitutionalFault::ArithmeticOverflow,
                module_ids::AMUN_CODEC,
                operation_ids::BUFFER_WRITE,
            )
        })?;
        if end > self.buf.len() {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                module_ids::AMUN_CODEC,
                operation_ids::BUFFER_WRITE,
            ));
        }
        self.buf[self.pos..end].copy_from_slice(bytes);
        self.pos = end;
        Ok(())
    }
    fn position(&self) -> usize {
        self.pos
    }
}

pub struct HasherWriter<'a> {
    hasher: &'a mut blake3::Hasher,
    pos: usize,
}

impl<'a> HasherWriter<'a> {
    pub fn new(hasher: &'a mut blake3::Hasher) -> Self {
        Self { hasher, pos: 0 }
    }
}

impl<'a> CanonicalWriter for HasherWriter<'a> {
    fn write_bytes(&mut self, bytes: &[u8]) -> WriteResult {
        self.hasher.update(bytes);
        self.pos = self.pos.checked_add(bytes.len()).ok_or_else(|| {
            FailureContext::new(
                ConstitutionalFault::ArithmeticOverflow,
                module_ids::AMUN_CODEC,
                operation_ids::HASHER_WRITE,
            )
        })?;
        Ok(())
    }
    fn position(&self) -> usize {
        self.pos
    }
}

pub struct ShadowBufferWriter<'a> {
    main: &'a mut [u8],
    main_pos: usize,
    shadow: [u8; 4096],
    shadow_pos: usize,
    in_transaction: bool,
}

impl<'a> ShadowBufferWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self {
            main: buf,
            main_pos: 0,
            shadow: [0u8; 4096],
            shadow_pos: 0,
            in_transaction: false,
        }
    }

    pub fn begin_transaction(&mut self) -> WriteResult {
        if self.in_transaction {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidStateTransition,
                module_ids::AMUN_CODEC,
                operation_ids::TRANSACTION_BEGIN,
            ));
        }
        self.in_transaction = true;
        self.shadow_pos = 0;
        Ok(())
    }

    pub fn commit_transaction(&mut self) -> WriteResult {
        if !self.in_transaction {
            return Err(FailureContext::new(
                ConstitutionalFault::InvalidStateTransition,
                module_ids::AMUN_CODEC,
                operation_ids::TRANSACTION_COMMIT,
            ));
        }
        let end = self.main_pos + self.shadow_pos;
        if end > self.main.len() {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                module_ids::AMUN_CODEC,
                operation_ids::TRANSACTION_COMMIT,
            ));
        }
        self.main[self.main_pos..end].copy_from_slice(&self.shadow[..self.shadow_pos]);
        self.main_pos = end;
        self.in_transaction = false;
        self.shadow[..self.shadow_pos].fill(0);
        self.shadow_pos = 0;
        Ok(())
    }

    pub fn rollback_transaction(&mut self) {
        self.shadow[..self.shadow_pos].fill(0);
        self.shadow_pos = 0;
        self.in_transaction = false;
    }

    pub fn into_written(self) -> &'a mut [u8] {
        &mut self.main[..self.main_pos]
    }
}

impl<'a> CanonicalWriter for ShadowBufferWriter<'a> {
    fn write_bytes(&mut self, bytes: &[u8]) -> WriteResult {
        if self.in_transaction {
            let end = self.shadow_pos + bytes.len();
            if end > self.shadow.len() {
                return Err(FailureContext::new(
                    ConstitutionalFault::BufferTooSmall,
                    module_ids::AMUN_CODEC,
                    operation_ids::SHADOW_WRITE,
                ));
            }
            self.shadow[self.shadow_pos..end].copy_from_slice(bytes);
            self.shadow_pos = end;
        } else {
            let end = self.main_pos + bytes.len();
            if end > self.main.len() {
                return Err(FailureContext::new(
                    ConstitutionalFault::BufferTooSmall,
                    module_ids::AMUN_CODEC,
                    operation_ids::BUFFER_WRITE,
                ));
            }
            self.main[self.main_pos..end].copy_from_slice(bytes);
            self.main_pos = end;
        }
        Ok(())
    }
    fn position(&self) -> usize {
        if self.in_transaction {
            self.main_pos + self.shadow_pos
        } else {
            self.main_pos
        }
    }
}
