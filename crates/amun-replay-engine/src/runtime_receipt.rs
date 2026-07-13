//! RuntimeReceipt — operational receipt using OperationalHasher.
//!
//! This receipt records runtime execution metadata. It uses
//! OperationalHasher, NOT ConstitutionalHasher, because operational
//! metadata must not be constitutionally meaningful.

use amun_constitutional::kernel_types::ConstitutionalHash;
use crate::operational_hasher::OperationalHasher;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeReceipt {
    pub receipt_id: u64,
    pub task_id: u64,
    pub worker_id: u64,
    pub start_position: u64,
    pub end_position: u64,
    pub execution_success: bool,
    pub produced_artifact_hash: ConstitutionalHash,
    pub receipt_hash: [u8; 32],
}

impl RuntimeReceipt {
    pub fn new(
        receipt_id: u64, task_id: u64, worker_id: u64,
        start_position: u64, end_position: u64,
        execution_success: bool, produced_artifact_hash: ConstitutionalHash,
    ) -> Self {
        let mut r = Self {
            receipt_id, task_id, worker_id,
            start_position, end_position,
            execution_success, produced_artifact_hash,
            receipt_hash: [0; 32],
        };
        r.receipt_hash = r.compute_hash();
        r
    }

    fn compute_hash(&self) -> [u8; 32] {
        let mut h = OperationalHasher::new(b"RUNTIME_RECEIPT");
        h.update_u64(self.receipt_id)
            .update_u64(self.task_id)
            .update_u64(self.worker_id)
            .update_u64(self.start_position)
            .update_u64(self.end_position)
            .update_u8(self.execution_success as u8)
            .update_bytes(&self.produced_artifact_hash);
        h.finalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_receipt_hash_deterministic() { let r1 = RuntimeReceipt::new(1,10,100,0,99,true,[0xAA;32]); let r2 = RuntimeReceipt::new(1,10,100,0,99,true,[0xAA;32]); assert_eq!(r1.receipt_hash, r2.receipt_hash); }
    #[test] fn test_operational_separation() { let r = RuntimeReceipt::new(1,10,100,0,99,true,[0xAA;32]); assert_ne!(r.receipt_hash, [0u8;32]); }
}
