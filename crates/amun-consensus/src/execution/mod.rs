#![forbid(unsafe_code)]

//! Block Execution Engine
//!
//! Executes blocks deterministically and produces execution results.

use sha2::{Sha256, Digest};
use amun_state_machine::{
    ConstitutionalState, TransitionEngine, ExecutionReceipts
};
use crate::block::{Block};
use crate::ordering::OrderingValidator;

/// Result of block execution
#[derive(Debug, Clone)]
pub struct BlockExecutionResult {
    pub pre_state_hash: [u8; 32],
    pub post_state_hash: [u8; 32],
    pub receipts: ExecutionReceipts,
    pub trace_root: [u8; 32],
    pub event_root: [u8; 32],
    pub snapshot_root: [u8; 32],
    pub success_count: u64,
    pub failure_count: u64,
}

impl BlockExecutionResult {
    pub fn new(
        pre_state_hash: [u8; 32],
        post_state_hash: [u8; 32],
        receipts: ExecutionReceipts,
        trace_root: [u8; 32],
        event_root: [u8; 32],
        snapshot_root: [u8; 32],
        success_count: u64,
        failure_count: u64,
    ) -> Self {
        Self {
            pre_state_hash,
            post_state_hash,
            receipts,
            trace_root,
            event_root,
            snapshot_root,
            success_count,
            failure_count,
        }
    }

    pub fn receipts_root(&self) -> [u8; 32] {
        self.receipts.accumulator_hash
    }
}

/// Block Execution Engine
/// Executes blocks deterministically
pub struct BlockExecutionEngine;

impl BlockExecutionEngine {
    /// Execute a block and produce execution result
    pub fn execute_block(
        state: &mut ConstitutionalState,
        block: &Block,
    ) -> Result<BlockExecutionResult, ExecutionError> {
        let pre_state_hash = state.hash();
        
        // Verify block ordering first
        if !OrderingValidator::validate(&block.body.events, block.header.height).is_valid() {
            return Err(ExecutionError::InvalidOrdering);
        }
        
        let mut receipts = ExecutionReceipts::new();
        let mut event_hashes = Vec::new();
        let mut success_count = 0;
        let mut failure_count = 0;
        
        // Execute each event in order
        for event in &block.body.events {
            let result = TransitionEngine::apply(state, event);
            
            if let Some(receipt) = result.receipt {
                receipts.push(receipt);
                if result.success {
                    success_count += 1;
                } else {
                    failure_count += 1;
                }
            }
            
            event_hashes.push(event.hash());
        }
        
        // Compute event root
        let event_root = Self::compute_event_root(&event_hashes);
        
        // Compute trace root
        let trace_root = Self::compute_trace_root(&receipts);
        
        // Compute snapshot root
        let snapshot_root = Self::compute_snapshot_root(state);
        
        let post_state_hash = state.hash();
        
        Ok(BlockExecutionResult::new(
            pre_state_hash,
            post_state_hash,
            receipts,
            trace_root,
            event_root,
            snapshot_root,
            success_count,
            failure_count,
        ))
    }
    
    /// Verify that a block execution result matches the block header
    pub fn verify_block(
        state: &mut ConstitutionalState,
        block: &Block,
        result: &BlockExecutionResult,
    ) -> bool {
        match Self::execute_block(state, block) {
            Ok(verified) => {
                verified.post_state_hash == result.post_state_hash &&
                verified.receipts_root() == result.receipts_root() &&
                verified.trace_root == result.trace_root &&
                verified.event_root == result.event_root
            }
            Err(_) => false,
        }
    }
    
    fn compute_event_root(event_hashes: &[[u8; 32]]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        for hash in event_hashes {
            hasher.update(hash);
        }
        hasher.finalize().into()
    }
    
    fn compute_trace_root(receipts: &ExecutionReceipts) -> [u8; 32] {
        let mut hasher = Sha256::new();
        for i in 0..receipts.len() {
            if let Some(receipt) = receipts.get(i) {
                hasher.update(receipt.hash());
            }
        }
        hasher.finalize().into()
    }
    
    fn compute_snapshot_root(state: &ConstitutionalState) -> [u8; 32] {
        state.hash()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionError {
    InvalidOrdering,
    InvalidEvent(String),
    StateError(String),
}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidOrdering => write!(f, "Invalid event ordering in block"),
            Self::InvalidEvent(msg) => write!(f, "Invalid event: {}", msg),
            Self::StateError(msg) => write!(f, "State error: {}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amun_state_machine::{Event, EventType};
    use amun_consensus_math::Fixed;
    use crate::block::{BlockHeader, BlockBody};
    
    #[test]
    fn test_execute_block() {
        let mut state = ConstitutionalState::new();
        state.add_account(1, Fixed::from_int(1000));
        state.add_account(2, Fixed::from_int(500));
        
        let events = vec![
            Event::new(EventType::Transfer, 1, 2, Fixed::from_int(100).raw(), 0),
        ];
        
        let mut body = BlockBody::new(events);
        body.sort_canonical(1);
        
        let header = BlockHeader::new(
            1,
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            100,
        );
        
        let block = Block::new(header, body);
        
        let result = BlockExecutionEngine::execute_block(&mut state, &block);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert_eq!(result.success_count, 1);
        assert_eq!(result.failure_count, 0);
        assert_ne!(result.post_state_hash, result.pre_state_hash);
    }
    
    #[test]
    fn test_execute_block_with_failure() {
        let mut state = ConstitutionalState::new();
        state.add_account(1, Fixed::from_int(100));
        
        let events = vec![
            Event::new(EventType::Transfer, 1, 2, Fixed::from_int(10000).raw(), 0),
        ];
        
        let mut body = BlockBody::new(events);
        body.sort_canonical(1);
        
        let header = BlockHeader::new(
            1,
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            100,
        );
        
        let block = Block::new(header, body);
        
        let result = BlockExecutionEngine::execute_block(&mut state, &block);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert_eq!(result.success_count, 0);
        assert_eq!(result.failure_count, 1);
        
        // State should be unchanged on failure
        assert_eq!(result.pre_state_hash, result.post_state_hash);
    }
    
    #[test]
    fn test_verify_block() {
        let mut state = ConstitutionalState::new();
        state.add_account(1, Fixed::from_int(1000));
        state.add_account(2, Fixed::from_int(500));
        
        let events = vec![
            Event::new(EventType::Transfer, 1, 2, Fixed::from_int(100).raw(), 0),
        ];
        
        let mut body = BlockBody::new(events);
        body.sort_canonical(1);
        
        let header = BlockHeader::new(
            1,
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            [0; 32],
            100,
        );
        
        let block = Block::new(header, body);
        
        let result = BlockExecutionEngine::execute_block(&mut state, &block).unwrap();
        
        let mut verify_state = ConstitutionalState::new();
        verify_state.add_account(1, Fixed::from_int(1000));
        verify_state.add_account(2, Fixed::from_int(500));
        
        assert!(BlockExecutionEngine::verify_block(&mut verify_state, &block, &result));
    }
}
