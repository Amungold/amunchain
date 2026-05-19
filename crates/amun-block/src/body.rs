use crate::limits::{BlockLimits, CONSTITUTIONAL_MAX_TX_COUNT};
use amun_codec::{CanonicalDecode, CanonicalEncode, CanonicalWriter, WriteResult};
use amun_failure::{AmunResult, ConstitutionalFault, FailureContext};
use amun_kernel_types::PublicHash32;
use heapless::Vec;

// BlockBody stores transaction HASHES only (PublicHash32).
// Tx bytes live in mempool/state layer.
// Max stack size: 500 * 32 bytes = 16 KB.
// The tx_root in BlockHeader cryptographically binds the block to its transactions.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockBody {
    pub tx_hashes: Vec<PublicHash32, CONSTITUTIONAL_MAX_TX_COUNT>,
}

impl BlockBody {
    pub fn new() -> Self {
        Self {
            tx_hashes: Vec::new(),
        }
    }

    pub fn add_tx_hash(
        &mut self,
        tx_hash: PublicHash32,
        limits: &BlockLimits,
    ) -> Result<(), &'static str> {
        if self.tx_hashes.len() >= limits.max_transactions as usize {
            return Err("Block transaction limit exceeded");
        }
        self.tx_hashes.push(tx_hash).map_err(|_| "Push failed")?;
        Ok(())
    }

    pub fn tx_count(&self) -> usize {
        self.tx_hashes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.tx_hashes.is_empty()
    }

    pub(crate) fn compute_encoded_size(&self) -> Result<usize, FailureContext> {
        let count = self.tx_hashes.len();
        let hash_size = PublicHash32::MAX_ENCODED_SIZE;
        let body_size = count.checked_mul(hash_size).ok_or_else(|| {
            FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x0007, 0x0110)
        })?;
        let total = 4usize.checked_add(body_size).ok_or_else(|| {
            FailureContext::new(ConstitutionalFault::ArithmeticOverflow, 0x0007, 0x0111)
        })?;
        Ok(total)
    }
}

impl CanonicalEncode for BlockBody {
    const MAX_ENCODED_SIZE: usize =
        CONSTITUTIONAL_MAX_TX_COUNT * PublicHash32::MAX_ENCODED_SIZE + 4;
    fn encode_to_writer(&self, writer: &mut impl CanonicalWriter) -> WriteResult {
        (self.tx_hashes.len() as u32).encode_to_writer(writer)?;
        for hash in &self.tx_hashes {
            hash.encode_to_writer(writer)?;
        }
        Ok(())
    }
}

impl CanonicalDecode for BlockBody {
    fn decode(input: &[u8]) -> AmunResult<(Self, usize)> {
        if input.len() < 4 {
            return Err(FailureContext::new(
                ConstitutionalFault::BufferTooSmall,
                0x0007,
                0x0108,
            ));
        }
        let (tx_count, len1) = u32::decode(input)?;
        let tx_count = tx_count as usize;
        if tx_count > CONSTITUTIONAL_MAX_TX_COUNT {
            return Err(FailureContext::new(
                ConstitutionalFault::CapacityExceeded,
                0x0007,
                0x0104,
            ));
        }
        let mut pos = len1;
        let mut tx_hashes = Vec::new();
        for _ in 0..tx_count {
            if pos + PublicHash32::MAX_ENCODED_SIZE > input.len() {
                return Err(FailureContext::new(
                    ConstitutionalFault::BufferTooSmall,
                    0x0007,
                    0x0105,
                ));
            }
            let (hash, len2) = PublicHash32::decode(&input[pos..])?;
            pos += len2;
            tx_hashes.push(hash).map_err(|_| {
                FailureContext::new(ConstitutionalFault::CapacityExceeded, 0x0007, 0x0103)
            })?;
        }
        Ok((Self { tx_hashes }, pos))
    }
}

impl Default for BlockBody {
    fn default() -> Self {
        Self::new()
    }
}
