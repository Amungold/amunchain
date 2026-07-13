#![forbid(unsafe_code)]

use amun_consensus_math::Fixed;
use crate::event::{Event, EventType};
use crate::state::{ConstitutionalState, Account};
use crate::receipt::{ExecutionReceipt, ErrorCode};

const TRANSITION_VERSION: u32 = 1;

pub struct TransitionResult {
    pub success: bool,
    pub error_message: Option<String>,
    pub error_code: ErrorCode,
    pub receipt: Option<ExecutionReceipt>,
    pub state_unchanged: bool,
}

impl TransitionResult {
    pub fn success() -> Self {
        Self {
            success: true,
            error_message: None,
            error_code: ErrorCode::Success,
            receipt: None,
            state_unchanged: false,
        }
    }

    pub fn failure(error_code: ErrorCode, message: &str, state_unchanged: bool) -> Self {
        Self {
            success: false,
            error_message: Some(message.to_string()),
            error_code,
            receipt: None,
            state_unchanged,
        }
    }
}

// Fixed::from_raw now returns Fixed directly, not Option
// Amount validation is done via bounds checking
fn validate_amount(amount_raw: i64) -> Result<Fixed, ErrorCode> {
    if amount_raw < -10_000_000_000_000_000 || amount_raw > 10_000_000_000_000_000 {
        return Err(ErrorCode::InvalidAmount);
    }
    Ok(Fixed::from_raw(amount_raw))
}

pub struct TransitionEngine;

impl TransitionEngine {
    pub fn apply(state: &mut ConstitutionalState, event: &Event) -> TransitionResult {
        let pre_hash = state.hash();
        
        let result = match event.event_type {
            EventType::Transfer => Self::handle_transfer(state, event),
            EventType::Mint => Self::handle_mint(state, event),
            EventType::Burn => Self::handle_burn(state, event),
            EventType::Delegate => Self::handle_delegate(state, event),
            EventType::Undelegate => Self::handle_undelegate(state, event),
            EventType::Slash => Self::handle_slash(state, event),
            EventType::Reward => Self::handle_reward(state, event),
        };
        
        if result.success {
            if let Some(account) = state.get_account_mut(event.source) {
                account.nonce += 1;
            }
            state.height += 1;
            state.recompute_hash();
        }
        
        let receipt = ExecutionReceipt::new(
            pre_hash,
            state.hash(),
            event,
            result.success,
            result.error_code,
            1,
            TRANSITION_VERSION,
        );
        
        TransitionResult {
            success: result.success,
            error_message: result.error_message,
            error_code: result.error_code,
            receipt: Some(receipt),
            state_unchanged: !result.success,
        }
    }
    
    fn handle_transfer(state: &mut ConstitutionalState, event: &Event) -> TransitionResult {
        let amount = match validate_amount(event.amount) {
            Ok(v) => v,
            Err(e) => return TransitionResult::failure(e, "Invalid amount", true),
        };
        
        let source_balance = match state.get_account(event.source) {
            Some(acc) => acc.balance,
            None => return TransitionResult::failure(ErrorCode::AccountNotFound, "Source account not found", true),
        };
        
        if source_balance < amount {
            return TransitionResult::failure(ErrorCode::InsufficientBalance, "Insufficient balance", true);
        }
        
        if !state.accounts.contains_key(&event.target) {
            state.accounts.insert(event.target, Account::new(Fixed::ZERO));
        }
        
        if let Some(source) = state.get_account_mut(event.source) {
            source.balance = source.balance - amount;
        }
        if let Some(target) = state.get_account_mut(event.target) {
            target.balance = target.balance + amount;
        }
        
        TransitionResult::success()
    }
    
    fn handle_mint(state: &mut ConstitutionalState, event: &Event) -> TransitionResult {
        let amount = match validate_amount(event.amount) {
            Ok(v) => v,
            Err(e) => return TransitionResult::failure(e, "Invalid amount", true),
        };
        
        if !state.accounts.contains_key(&event.target) {
            state.accounts.insert(event.target, Account::new(Fixed::ZERO));
        }
        
        if let Some(target) = state.get_account_mut(event.target) {
            target.balance = target.balance + amount;
        }
        state.total_supply = state.total_supply + amount;
        
        TransitionResult::success()
    }
    
    fn handle_burn(state: &mut ConstitutionalState, event: &Event) -> TransitionResult {
        let amount = match validate_amount(event.amount) {
            Ok(v) => v,
            Err(e) => return TransitionResult::failure(e, "Invalid amount", true),
        };
        
        let source_balance = match state.get_account(event.source) {
            Some(acc) => acc.balance,
            None => return TransitionResult::failure(ErrorCode::AccountNotFound, "Source account not found", true),
        };
        
        if source_balance < amount {
            return TransitionResult::failure(ErrorCode::InsufficientBalance, "Insufficient balance", true);
        }
        
        if let Some(source) = state.get_account_mut(event.source) {
            source.balance = source.balance - amount;
        }
        state.total_supply = state.total_supply - amount;
        
        TransitionResult::success()
    }
    
    fn handle_delegate(state: &mut ConstitutionalState, event: &Event) -> TransitionResult {
        let amount = match validate_amount(event.amount) {
            Ok(v) => v,
            Err(e) => return TransitionResult::failure(e, "Invalid amount", true),
        };
        
        let source_balance = match state.get_account(event.source) {
            Some(acc) => acc.balance,
            None => return TransitionResult::failure(ErrorCode::AccountNotFound, "Source account not found", true),
        };
        
        if source_balance < amount {
            return TransitionResult::failure(ErrorCode::InsufficientBalance, "Insufficient balance", true);
        }
        
        if let Some(source) = state.get_account_mut(event.source) {
            source.balance = source.balance - amount;
            source.delegated_to = Some(event.target);
            source.delegation_amount = amount;
        }
        
        if let Some(target) = state.get_account_mut(event.target) {
            target.balance = target.balance + amount;
        }
        
        TransitionResult::success()
    }
    
    fn handle_undelegate(state: &mut ConstitutionalState, event: &Event) -> TransitionResult {
        let source = match state.get_account(event.source) {
            Some(acc) => acc,
            None => return TransitionResult::failure(ErrorCode::AccountNotFound, "Source account not found", true),
        };
        
        let amount = source.delegation_amount;
        let target = match source.delegated_to {
            Some(t) => t,
            None => return TransitionResult::failure(ErrorCode::NotDelegated, "Not delegated", true),
        };
        
        if let Some(source_acc) = state.get_account_mut(event.source) {
            source_acc.balance = source_acc.balance + amount;
            source_acc.delegated_to = None;
            source_acc.delegation_amount = Fixed::ZERO;
        }
        
        if let Some(target_acc) = state.get_account_mut(target) {
            target_acc.balance = target_acc.balance - amount;
        }
        
        TransitionResult::success()
    }
    
    fn handle_slash(state: &mut ConstitutionalState, event: &Event) -> TransitionResult {
        let amount = match validate_amount(event.amount) {
            Ok(v) => v,
            Err(e) => return TransitionResult::failure(e, "Invalid amount", true),
        };
        
        let source_balance = match state.get_account(event.source) {
            Some(acc) => acc.balance,
            None => return TransitionResult::failure(ErrorCode::AccountNotFound, "Source account not found", true),
        };
        
        let slashed = source_balance * amount / Fixed::ONE;
        
        if let Some(source) = state.get_account_mut(event.source) {
            source.balance = source.balance - slashed;
        }
        state.total_supply = state.total_supply - slashed;
        
        TransitionResult::success()
    }
    
    fn handle_reward(state: &mut ConstitutionalState, event: &Event) -> TransitionResult {
        let amount = match validate_amount(event.amount) {
            Ok(v) => v,
            Err(e) => return TransitionResult::failure(e, "Invalid amount", true),
        };
        
        if !state.accounts.contains_key(&event.target) {
            state.accounts.insert(event.target, Account::new(Fixed::ZERO));
        }
        
        if let Some(target) = state.get_account_mut(event.target) {
            target.balance = target.balance + amount;
        }
        state.total_supply = state.total_supply + amount;
        
        TransitionResult::success()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ConstitutionalState;
    
    #[test]
    fn test_failed_transition_state_unchanged() {
        let mut state = ConstitutionalState::new();
        state.add_account(1, Fixed::from_int(100));
        let pre_hash = state.hash();
        
        let event = Event::new(EventType::Transfer, 1, 2, Fixed::from_int(10000).raw(), 0);
        let result = TransitionEngine::apply(&mut state, &event);
        
        assert!(!result.success);
        assert!(result.state_unchanged);
        assert_eq!(state.hash(), pre_hash);
    }
    
    #[test]
    fn test_invalid_amount_rejected() {
        let mut state = ConstitutionalState::new();
        state.add_account(1, Fixed::from_int(100));
        
        let event = Event::new(EventType::Transfer, 1, 2, 10_000_000_000_000_001, 0);
        let result = TransitionEngine::apply(&mut state, &event);
        
        assert!(!result.success);
        assert_eq!(result.error_code, ErrorCode::InvalidAmount);
    }
}
