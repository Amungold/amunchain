#![forbid(unsafe_code)]

use sha2::{Sha256, Digest};
use crate::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ErrorCode {
    Success = 0,
    InvalidNonce = 1,
    InsufficientBalance = 2,
    InvalidAmount = 3,
    AccountNotFound = 4,
    NotDelegated = 5,
    OverflowSaturation = 6,
    InvalidEventType = 7,
}

impl ErrorCode {
    pub fn to_u16(self) -> u16 {
        self as u16
    }
    
    pub fn from_u16(code: u16) -> Option<Self> {
        match code {
            0 => Some(Self::Success),
            1 => Some(Self::InvalidNonce),
            2 => Some(Self::InsufficientBalance),
            3 => Some(Self::InvalidAmount),
            4 => Some(Self::AccountNotFound),
            5 => Some(Self::NotDelegated),
            6 => Some(Self::OverflowSaturation),
            7 => Some(Self::InvalidEventType),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionReceipt {
    pub pre_state_hash: [u8; 32],
    pub post_state_hash: [u8; 32],
    pub event_hash: [u8; 32],
    pub success: bool,
    pub error_code: u16,
    pub gas_used: u64,
    pub transition_version: u32,
}

impl ExecutionReceipt {
    pub fn new(
        pre_state_hash: [u8; 32],
        post_state_hash: [u8; 32],
        event: &Event,
        success: bool,
        error_code: ErrorCode,
        gas_used: u64,
        transition_version: u32,
    ) -> Self {
        Self {
            pre_state_hash,
            post_state_hash,
            event_hash: event.hash(),
            success,
            error_code: error_code.to_u16(),
            gas_used,
            transition_version,
        }
    }
    
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(32 + 32 + 32 + 1 + 2 + 8 + 4);
        bytes.extend_from_slice(&self.pre_state_hash);
        bytes.extend_from_slice(&self.post_state_hash);
        bytes.extend_from_slice(&self.event_hash);
        bytes.push(self.success as u8);
        bytes.extend_from_slice(&self.error_code.to_be_bytes());
        bytes.extend_from_slice(&self.gas_used.to_be_bytes());
        bytes.extend_from_slice(&self.transition_version.to_be_bytes());
        bytes
    }
    
    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.canonical_bytes());
        hasher.finalize().into()
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionReceipts {
    pub receipts: Vec<ExecutionReceipt>,
    pub accumulator_hash: [u8; 32],  // Linear hash chain, not Merkle root
}

impl ExecutionReceipts {
    pub fn new() -> Self {
        Self {
            receipts: Vec::new(),
            accumulator_hash: [0; 32],
        }
    }
    
    pub fn push(&mut self, receipt: ExecutionReceipt) {
        self.receipts.push(receipt);
        self.recompute_accumulator();
    }
    
    pub fn recompute_accumulator(&mut self) {
        let mut hasher = Sha256::new();
        for receipt in &self.receipts {
            hasher.update(receipt.hash());
        }
        self.accumulator_hash = hasher.finalize().into();
    }
    
    pub fn len(&self) -> usize {
        self.receipts.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.receipts.is_empty()
    }
    
    pub fn get(&self, index: usize) -> Option<&ExecutionReceipt> {
        self.receipts.get(index)
    }
}

impl Default for ExecutionReceipts {
    fn default() -> Self {
        Self::new()
    }
}
